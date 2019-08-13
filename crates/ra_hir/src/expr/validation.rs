use rustc_hash::FxHashSet;
use std::sync::Arc;

use ra_syntax::ast::{AstNode, StructLit};

use super::{Expr, ExprId, StructLitField};
use crate::{
    adt::AdtDef,
    code_model::Enum,
    diagnostics::{DiagnosticSink, MissingFields, MissingOkInTailExpr},
    expr::AstPtr,
    name,
    path::{PathKind, PathSegment},
    ty::{InferenceResult, Ty, TypeCtor},
    Function, HasSource, HirDatabase, ModuleDef, Name, Path, PerNs, Resolution,
};
use ra_syntax::ast;

pub(crate) struct ExprValidator<'a, 'b: 'a> {
    func: Function,
    infer: Arc<InferenceResult>,
    sink: &'a mut DiagnosticSink<'b>,
}

impl<'a, 'b> ExprValidator<'a, 'b> {
    pub(crate) fn new(
        func: Function,
        infer: Arc<InferenceResult>,
        sink: &'a mut DiagnosticSink<'b>,
    ) -> ExprValidator<'a, 'b> {
        ExprValidator { func, infer, sink }
    }

    pub(crate) fn validate_body(&mut self, db: &impl HirDatabase) {
        let body = self.func.body(db);

        for e in body.exprs() {
            if let (id, Expr::StructLit { path, fields, spread }) = e {
                self.validate_struct_literal(id, path, fields, *spread, db);
            }
        }

        let body_expr = &body[body.body_expr()];
        if let Expr::Block { statements: _, tail: Some(t) } = body_expr {
            self.validate_results_in_tail_expr(*t, db);
        }
    }

    fn validate_struct_literal(
        &mut self,
        id: ExprId,
        _path: &Option<Path>,
        fields: &[StructLitField],
        spread: Option<ExprId>,
        db: &impl HirDatabase,
    ) {
        if spread.is_some() {
            return;
        }

        let struct_def = match self.infer[id].as_adt() {
            Some((AdtDef::Struct(s), _)) => s,
            _ => return,
        };

        let lit_fields: FxHashSet<_> = fields.iter().map(|f| &f.name).collect();
        let missed_fields: Vec<Name> = struct_def
            .fields(db)
            .iter()
            .filter_map(|f| {
                let name = f.name(db);
                if lit_fields.contains(&name) {
                    None
                } else {
                    Some(name)
                }
            })
            .collect();
        if missed_fields.is_empty() {
            return;
        }
        let source_map = self.func.body_source_map(db);
        let file_id = self.func.source(db).file_id;
        let parse = db.parse(file_id.original_file(db));
        let source_file = parse.tree();
        if let Some(field_list_node) = source_map
            .expr_syntax(id)
            .map(|ptr| ptr.to_node(source_file.syntax()))
            .and_then(StructLit::cast)
            .and_then(|lit| lit.named_field_list())
        {
            let field_list_ptr = AstPtr::new(&field_list_node);
            self.sink.push(MissingFields {
                file: file_id,
                field_list: field_list_ptr,
                missed_fields,
            })
        }
    }

    fn validate_results_in_tail_expr(&mut self, id: ExprId, db: &impl HirDatabase) {
        let mismatch = match self.infer.type_mismatch_for_expr(id) {
            Some(m) => m,
            None => return,
        };

        let std_result_path = Path {
            kind: PathKind::Abs,
            segments: vec![
                PathSegment { name: name::STD, args_and_bindings: None },
                PathSegment { name: name::RESULT_MOD, args_and_bindings: None },
                PathSegment { name: name::RESULT_TYPE, args_and_bindings: None },
            ],
        };

        let resolver = self.func.resolver(db);
        let std_result_enum =
            match resolver.resolve_path_segments(db, &std_result_path).into_fully_resolved() {
                PerNs { types: Some(Resolution::Def(ModuleDef::Enum(e))), .. } => e,
                _ => return,
            };

        let std_result_type = std_result_enum.ty(db);

        fn enum_from_type(ty: &Ty) -> Option<Enum> {
            match ty {
                Ty::Apply(t) => match t.ctor {
                    TypeCtor::Adt(AdtDef::Enum(e)) => Some(e),
                    _ => None,
                },
                _ => None,
            }
        }

        if enum_from_type(&mismatch.expected) != enum_from_type(&std_result_type) {
            return;
        }

        let ret = match &mismatch.expected {
            Ty::Apply(t) => t,
            _ => return,
        };

        let params = &ret.parameters;
        if params.len() == 2 && &params[0] == &mismatch.actual {
            let source_map = self.func.body_source_map(db);
            let file_id = self.func.source(db).file_id;
            let parse = db.parse(file_id.original_file(db));
            let source_file = parse.tree();
            let expr_syntax = source_map.expr_syntax(id);
            if expr_syntax.is_none() {
                return;
            }
            let expr_syntax = expr_syntax.unwrap();
            let node = expr_syntax.to_node(source_file.syntax());
            let ast = ast::Expr::cast(node);
            if ast.is_none() {
                return;
            }
            let ast = ast.unwrap();

            self.sink.push(MissingOkInTailExpr { file: file_id, expr: AstPtr::new(&ast) });
        }
    }
}
