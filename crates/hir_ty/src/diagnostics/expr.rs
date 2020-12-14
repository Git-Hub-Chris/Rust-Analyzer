//! FIXME: write short doc here

use std::sync::Arc;

use hir_def::{expr::Statement, path::path, resolver::HasResolver, AdtId, DefWithBodyId};
use hir_expand::diagnostics::DiagnosticSink;
use rustc_hash::FxHashSet;
use syntax::{ast, AstPtr};

use crate::{
    db::HirDatabase,
    diagnostics::{
        match_check::{is_useful, MatchCheckCtx, Matrix, PatStack, Usefulness},
        AddReferenceToInitializer, MismatchedArgCount, MissingFields, MissingMatchArms,
        MissingOkInTailExpr, MissingPatFields, RemoveThisSemicolon,
    },
    utils::variant_data,
    ApplicationTy, InferenceResult, Ty, TyLoweringContext, TypeCtor,
};

pub(crate) use hir_def::{
    body::{Body, BodySourceMap},
    expr::{Expr, ExprId, MatchArm, Pat, PatId},
    type_ref::Mutability,
    LocalFieldId, VariantId,
};

pub(super) struct ExprValidator<'a, 'b: 'a> {
    owner: DefWithBodyId,
    infer: Arc<InferenceResult>,
    sink: &'a mut DiagnosticSink<'b>,
}

impl<'a, 'b> ExprValidator<'a, 'b> {
    pub(super) fn new(
        owner: DefWithBodyId,
        infer: Arc<InferenceResult>,
        sink: &'a mut DiagnosticSink<'b>,
    ) -> ExprValidator<'a, 'b> {
        ExprValidator { owner, infer, sink }
    }

    pub(super) fn validate_body(&mut self, db: &dyn HirDatabase) {
        let body = db.body(self.owner.into());

        for (id, expr) in body.exprs.iter() {
            if let Some((variant_def, missed_fields, true)) =
                record_literal_missing_fields(db, &self.infer, id, expr)
            {
                self.create_record_literal_missing_fields_diagnostic(
                    id,
                    db,
                    variant_def,
                    missed_fields,
                );
            }

            match expr {
                Expr::Block { statements, .. } => {
                    self.validate_block(db, statements);
                }
                Expr::Match { expr, arms } => {
                    self.validate_match(id, *expr, arms, db, self.infer.clone());
                }
                Expr::Call { .. } | Expr::MethodCall { .. } => {
                    self.validate_call(db, id, expr);
                }
                _ => {}
            }
        }
        for (id, pat) in body.pats.iter() {
            if let Some((variant_def, missed_fields, true)) =
                record_pattern_missing_fields(db, &self.infer, id, pat)
            {
                self.create_record_pattern_missing_fields_diagnostic(
                    id,
                    db,
                    variant_def,
                    missed_fields,
                );
            }
        }
        let body_expr = &body[body.body_expr];
        if let Expr::Block { statements, tail, .. } = body_expr {
            if let Some(t) = tail {
                self.validate_results_in_tail_expr(body.body_expr, *t, db);
            } else if let Some(Statement::Expr(id)) = statements.last() {
                self.validate_missing_tail_expr(body.body_expr, *id, db);
            }
        }
    }

    fn create_record_literal_missing_fields_diagnostic(
        &mut self,
        id: ExprId,
        db: &dyn HirDatabase,
        variant_def: VariantId,
        missed_fields: Vec<LocalFieldId>,
    ) {
        // XXX: only look at source_map if we do have missing fields
        let (_, source_map) = db.body_with_source_map(self.owner.into());

        if let Ok(source_ptr) = source_map.expr_syntax(id) {
            let root = source_ptr.file_syntax(db.upcast());
            if let ast::Expr::RecordExpr(record_expr) = &source_ptr.value.to_node(&root) {
                if let Some(_) = record_expr.record_expr_field_list() {
                    let variant_data = variant_data(db.upcast(), variant_def);
                    let missed_fields = missed_fields
                        .into_iter()
                        .map(|idx| variant_data.fields()[idx].name.clone())
                        .collect();
                    self.sink.push(MissingFields {
                        file: source_ptr.file_id,
                        field_list_parent: AstPtr::new(&record_expr),
                        field_list_parent_path: record_expr.path().map(|path| AstPtr::new(&path)),
                        missed_fields,
                    })
                }
            }
        }
    }

    fn create_record_pattern_missing_fields_diagnostic(
        &mut self,
        id: PatId,
        db: &dyn HirDatabase,
        variant_def: VariantId,
        missed_fields: Vec<LocalFieldId>,
    ) {
        // XXX: only look at source_map if we do have missing fields
        let (_, source_map) = db.body_with_source_map(self.owner.into());

        if let Ok(source_ptr) = source_map.pat_syntax(id) {
            if let Some(expr) = source_ptr.value.as_ref().left() {
                let root = source_ptr.file_syntax(db.upcast());
                if let ast::Pat::RecordPat(record_pat) = expr.to_node(&root) {
                    if let Some(_) = record_pat.record_pat_field_list() {
                        let variant_data = variant_data(db.upcast(), variant_def);
                        let missed_fields = missed_fields
                            .into_iter()
                            .map(|idx| variant_data.fields()[idx].name.clone())
                            .collect();
                        self.sink.push(MissingPatFields {
                            file: source_ptr.file_id,
                            field_list_parent: AstPtr::new(&record_pat),
                            field_list_parent_path: record_pat
                                .path()
                                .map(|path| AstPtr::new(&path)),
                            missed_fields,
                        })
                    }
                }
            }
        }
    }

    fn validate_block(&mut self, db: &dyn HirDatabase, statements: &[Statement]) {
        let (_, source_map) = db.body_with_source_map(self.owner.into());

        let resolver = self.owner.resolver(db.upcast());
        let context = TyLoweringContext::new(db, &resolver);

        let infer = &self.infer;
        let sink = &mut self.sink;

        statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::Let { type_ref: Some(type_ref), initializer: Some(expr_id), .. } => {
                    let param = Ty::from_hir(&context, type_ref);
                    let (expr_without_ref, mutability) =
                        check_missing_refs(infer, *expr_id, &param)?;

                    Some((source_map.expr_syntax(expr_without_ref).ok()?, mutability))
                }
                _ => None,
            })
            .for_each(|(source_ptr, mutability)| {
                sink.push(AddReferenceToInitializer {
                    file: source_ptr.file_id,
                    arg_expr: source_ptr.value,
                    mutability,
                });
            });
    }

    fn validate_call(&mut self, db: &dyn HirDatabase, call_id: ExprId, expr: &Expr) -> Option<()> {
        // Check that the number of arguments matches the number of parameters.

        // FIXME: Due to shortcomings in the current type system implementation, only emit this
        // diagnostic if there are no type mismatches in the containing function.
        // if self.infer.type_mismatches.iter().next().is_some() {
        //     return Some(());
        // }

        let infer = &self.infer;
        let sink = &mut self.sink;

        let is_method_call = matches!(expr, Expr::MethodCall { .. });
        let (sig, args) = match expr {
            Expr::Call { callee, args } => {
                let callee = &infer.type_of_expr[*callee];
                let sig = callee.callable_sig(db)?;
                (sig, args.clone())
            }
            Expr::MethodCall { receiver, args, .. } => {
                let mut args = args.clone();
                args.insert(0, *receiver);

                // FIXME: note that we erase information about substs here. This
                // is not right, but, luckily, doesn't matter as we care only
                // about the number of params
                let callee = infer.method_resolution(call_id)?;
                let sig = db.callable_item_signature(callee.into()).value;

                (sig, args)
            }
            _ => return None,
        };

        if sig.is_varargs {
            return None;
        }

        let params = sig.params();

        let mut param_count = params.len();
        let mut arg_count = args.len();

        let (_, source_map) = db.body_with_source_map(self.owner.into());

        if arg_count != param_count {
            if let Ok(source_ptr) = source_map.expr_syntax(call_id) {
                if is_method_call {
                    param_count -= 1;
                    arg_count -= 1;
                }
                sink.push(MismatchedArgCount {
                    file: source_ptr.file_id,
                    call_expr: source_ptr.value,
                    expected: param_count,
                    found: arg_count,
                });
            }
        }

        let mut args_with_params = args.iter().zip(params.iter());

        if is_method_call {
            args_with_params.next();
        }

        args_with_params
            .filter_map(|(arg, param)| {
                let (arg, mutability) = check_missing_refs(infer, *arg, param)?;

                Some((source_map.expr_syntax(arg).ok()?, mutability))
            })
            .for_each(|(source_ptr, mutability)| {
                sink.push(AddReferenceToInitializer {
                    file: source_ptr.file_id,
                    arg_expr: source_ptr.value,
                    mutability,
                });
            });

        None
    }

    fn validate_match(
        &mut self,
        id: ExprId,
        match_expr: ExprId,
        arms: &[MatchArm],
        db: &dyn HirDatabase,
        infer: Arc<InferenceResult>,
    ) {
        let (body, source_map): (Arc<Body>, Arc<BodySourceMap>) =
            db.body_with_source_map(self.owner.into());

        let match_expr_ty = match infer.type_of_expr.get(match_expr) {
            // If we can't resolve the type of the match expression
            // we cannot perform exhaustiveness checks.
            None | Some(Ty::Unknown) => return,
            Some(ty) => ty,
        };

        let cx = MatchCheckCtx { match_expr, body, infer: infer.clone(), db };
        let pats = arms.iter().map(|arm| arm.pat);

        let mut seen = Matrix::empty();
        for pat in pats {
            if let Some(pat_ty) = infer.type_of_pat.get(pat) {
                // We only include patterns whose type matches the type
                // of the match expression. If we had a InvalidMatchArmPattern
                // diagnostic or similar we could raise that in an else
                // block here.
                //
                // When comparing the types, we also have to consider that rustc
                // will automatically de-reference the match expression type if
                // necessary.
                //
                // FIXME we should use the type checker for this.
                if pat_ty == match_expr_ty
                    || match_expr_ty
                        .as_reference()
                        .map(|(match_expr_ty, _)| match_expr_ty == pat_ty)
                        .unwrap_or(false)
                {
                    // If we had a NotUsefulMatchArm diagnostic, we could
                    // check the usefulness of each pattern as we added it
                    // to the matrix here.
                    let v = PatStack::from_pattern(pat);
                    seen.push(&cx, v);
                    continue;
                }
            }

            // If we can't resolve the type of a pattern, or the pattern type doesn't
            // fit the match expression, we skip this diagnostic. Skipping the entire
            // diagnostic rather than just not including this match arm is preferred
            // to avoid the chance of false positives.
            return;
        }

        match is_useful(&cx, &seen, &PatStack::from_wild()) {
            Ok(Usefulness::Useful) => (),
            // if a wildcard pattern is not useful, then all patterns are covered
            Ok(Usefulness::NotUseful) => return,
            // this path is for unimplemented checks, so we err on the side of not
            // reporting any errors
            _ => return,
        }

        if let Ok(source_ptr) = source_map.expr_syntax(id) {
            let root = source_ptr.file_syntax(db.upcast());
            if let ast::Expr::MatchExpr(match_expr) = &source_ptr.value.to_node(&root) {
                if let (Some(match_expr), Some(arms)) =
                    (match_expr.expr(), match_expr.match_arm_list())
                {
                    self.sink.push(MissingMatchArms {
                        file: source_ptr.file_id,
                        match_expr: AstPtr::new(&match_expr),
                        arms: AstPtr::new(&arms),
                    })
                }
            }
        }
    }

    fn validate_results_in_tail_expr(&mut self, body_id: ExprId, id: ExprId, db: &dyn HirDatabase) {
        // the mismatch will be on the whole block currently
        let mismatch = match self.infer.type_mismatch_for_expr(body_id) {
            Some(m) => m,
            None => return,
        };

        let core_result_path = path![core::result::Result];

        let resolver = self.owner.resolver(db.upcast());
        let core_result_enum = match resolver.resolve_known_enum(db.upcast(), &core_result_path) {
            Some(it) => it,
            _ => return,
        };

        let core_result_ctor = TypeCtor::Adt(AdtId::EnumId(core_result_enum));
        let params = match &mismatch.expected {
            Ty::Apply(ApplicationTy { ctor, parameters }) if ctor == &core_result_ctor => {
                parameters
            }
            _ => return,
        };

        if params.len() == 2 && params[0] == mismatch.actual {
            let (_, source_map) = db.body_with_source_map(self.owner.into());

            if let Ok(source_ptr) = source_map.expr_syntax(id) {
                self.sink
                    .push(MissingOkInTailExpr { file: source_ptr.file_id, expr: source_ptr.value });
            }
        }
    }

    fn validate_missing_tail_expr(
        &mut self,
        body_id: ExprId,
        possible_tail_id: ExprId,
        db: &dyn HirDatabase,
    ) {
        let mismatch = match self.infer.type_mismatch_for_expr(body_id) {
            Some(m) => m,
            None => return,
        };

        let possible_tail_ty = match self.infer.type_of_expr.get(possible_tail_id) {
            Some(ty) => ty,
            None => return,
        };

        if mismatch.actual != Ty::unit() || mismatch.expected != *possible_tail_ty {
            return;
        }

        let (_, source_map) = db.body_with_source_map(self.owner.into());

        if let Ok(source_ptr) = source_map.expr_syntax(possible_tail_id) {
            self.sink
                .push(RemoveThisSemicolon { file: source_ptr.file_id, expr: source_ptr.value });
        }
    }
}

fn check_missing_refs(
    infer: &InferenceResult,
    arg: ExprId,
    param: &Ty,
) -> Option<(ExprId, Mutability)> {
    let arg_ty = infer.type_of_expr.get(arg)?;

    match (arg_ty.as_reference(), param.as_reference()) {
        (
            None,
            Some((Ty::Apply(ApplicationTy { ctor: TypeCtor::Slice, parameters }), mutability)),
        ) => {
            if arg_ty.substs()?.as_single() != parameters.as_single() {
                return None;
            }

            Some((arg, mutability))
        }
        (None, Some((referenced_ty, mutability))) if referenced_ty == arg_ty => {
            Some((arg, mutability))
        }
        _ => None,
    }
}

pub fn record_literal_missing_fields(
    db: &dyn HirDatabase,
    infer: &InferenceResult,
    id: ExprId,
    expr: &Expr,
) -> Option<(VariantId, Vec<LocalFieldId>, /*exhaustive*/ bool)> {
    let (fields, exhausitve) = match expr {
        Expr::RecordLit { path: _, fields, spread } => (fields, spread.is_none()),
        _ => return None,
    };

    let variant_def = infer.variant_resolution_for_expr(id)?;
    if let VariantId::UnionId(_) = variant_def {
        return None;
    }

    let variant_data = variant_data(db.upcast(), variant_def);

    let specified_fields: FxHashSet<_> = fields.iter().map(|f| &f.name).collect();
    let missed_fields: Vec<LocalFieldId> = variant_data
        .fields()
        .iter()
        .filter_map(|(f, d)| if specified_fields.contains(&d.name) { None } else { Some(f) })
        .collect();
    if missed_fields.is_empty() {
        return None;
    }
    Some((variant_def, missed_fields, exhausitve))
}

pub fn record_pattern_missing_fields(
    db: &dyn HirDatabase,
    infer: &InferenceResult,
    id: PatId,
    pat: &Pat,
) -> Option<(VariantId, Vec<LocalFieldId>, /*exhaustive*/ bool)> {
    let (fields, exhaustive) = match pat {
        Pat::Record { path: _, args, ellipsis } => (args, !ellipsis),
        _ => return None,
    };

    let variant_def = infer.variant_resolution_for_pat(id)?;
    if let VariantId::UnionId(_) = variant_def {
        return None;
    }

    let variant_data = variant_data(db.upcast(), variant_def);

    let specified_fields: FxHashSet<_> = fields.iter().map(|f| &f.name).collect();
    let missed_fields: Vec<LocalFieldId> = variant_data
        .fields()
        .iter()
        .filter_map(|(f, d)| if specified_fields.contains(&d.name) { None } else { Some(f) })
        .collect();
    if missed_fields.is_empty() {
        return None;
    }
    Some((variant_def, missed_fields, exhaustive))
}

#[cfg(test)]
mod tests {
    use crate::diagnostics::tests::check_diagnostics;

    #[test]
    fn simple_free_fn_zero() {
        check_diagnostics(
            r#"
fn zero() {}
fn f() { zero(1); }
       //^^^^^^^ Expected 0 arguments, found 1
"#,
        );

        check_diagnostics(
            r#"
fn zero() {}
fn f() { zero(); }
"#,
        );
    }

    #[test]
    fn simple_free_fn_one() {
        check_diagnostics(
            r#"
fn one(arg: u8) {}
fn f() { one(); }
       //^^^^^ Expected 1 argument, found 0
"#,
        );

        check_diagnostics(
            r#"
fn one(arg: u8) {}
fn f() { one(1); }
"#,
        );
    }

    #[test]
    fn method_as_fn() {
        check_diagnostics(
            r#"
struct S;
impl S { fn method(&self) {} }

fn f() {
    S::method();
} //^^^^^^^^^^^ Expected 1 argument, found 0
"#,
        );

        check_diagnostics(
            r#"
struct S;
impl S { fn method(&self) {} }

fn f() {
    S::method(&S);
    S.method();
}
"#,
        );
    }

    #[test]
    fn method_with_arg() {
        check_diagnostics(
            r#"
struct S;
impl S { fn method(&self, arg: u8) {} }

            fn f() {
                S.method();
            } //^^^^^^^^^^ Expected 1 argument, found 0
            "#,
        );

        check_diagnostics(
            r#"
struct S;
impl S { fn method(&self, arg: u8) {} }

fn f() {
    S::method(&S, 0);
    S.method(1);
}
"#,
        );
    }

    #[test]
    fn tuple_struct() {
        check_diagnostics(
            r#"
struct Tup(u8, u16);
fn f() {
    Tup(0);
} //^^^^^^ Expected 2 arguments, found 1
"#,
        )
    }

    #[test]
    fn enum_variant() {
        check_diagnostics(
            r#"
enum En { Variant(u8, u16), }
fn f() {
    En::Variant(0);
} //^^^^^^^^^^^^^^ Expected 2 arguments, found 1
"#,
        )
    }

    #[test]
    fn enum_variant_type_macro() {
        check_diagnostics(
            r#"
macro_rules! Type {
    () => { u32 };
}
enum Foo {
    Bar(Type![])
}
impl Foo {
    fn new() {
        Foo::Bar(0);
        Foo::Bar(0, 1);
      //^^^^^^^^^^^^^^ Expected 1 argument, found 2
        Foo::Bar();
      //^^^^^^^^^^ Expected 1 argument, found 0
    }
}
        "#,
        );
    }

    #[test]
    fn varargs() {
        check_diagnostics(
            r#"
extern "C" {
    fn fixed(fixed: u8);
    fn varargs(fixed: u8, ...);
    fn varargs2(...);
}

fn f() {
    unsafe {
        fixed(0);
        fixed(0, 1);
      //^^^^^^^^^^^ Expected 1 argument, found 2
        varargs(0);
        varargs(0, 1);
        varargs2();
        varargs2(0);
        varargs2(0, 1);
    }
}
        "#,
        )
    }

    #[test]
    fn arg_count_lambda() {
        check_diagnostics(
            r#"
fn main() {
    let f = |()| ();
    f();
  //^^^ Expected 1 argument, found 0
    f(());
    f((), ());
  //^^^^^^^^^ Expected 1 argument, found 2
}
"#,
        )
    }

    #[test]
    fn add_reference_to_int() {
        check_diagnostics(
            r#"
fn main() {
    test(123);
       //^^^ Consider borrowing this argument
}

fn test(arg: &i32) {}
            "#,
        )
    }

    #[test]
    fn add_mutable_reference_to_int() {
        check_diagnostics(
            r#"
fn main() {
    test(123);
       //^^^ Consider borrowing this argument
}

fn test(arg: &mut i32) {}
            "#,
        )
    }

    #[test]
    fn add_reference_to_array() {
        check_diagnostics(
            r#"
fn main() {
    test([1, 2, 3]);
       //^^^^^^^^^ Consider borrowing this argument
}

fn test(arg: &[i32]) {}
            "#,
        )
    }

    #[test]
    fn add_reference_to_method_call() {
        check_diagnostics(
            r#"
fn main() {
    Test.call_by_ref(123);
                   //^^^ Consider borrowing this argument
}

struct Test;

impl Test {
    fn call_by_ref(&self, arg: &i32) {}
}
            "#,
        )
    }

    #[test]
    fn add_reference_to_let_stmt() {
        check_diagnostics(
            r#"
fn main() {
    let test: &i32 = 123;
                   //^^^ Consider borrowing this initializer
}
            "#,
        )
    }

    #[test]
    fn add_mutable_reference_to_let_stmt() {
        check_diagnostics(
            r#"
fn main() {
    let test: &mut i32 = 123;
                       //^^^ Consider borrowing this initializer
}
            "#,
        )
    }
}
