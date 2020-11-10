use hir::HirDisplay;
use syntax::{ast, AstNode, TextRange, TextSize};
use test_utils::mark;

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: infer_function_return_type
//
// Adds the return type to a function or closure inferred from its tail expression if it doesn't have a return
// type specified. This assists is useable in a functions or closures tail expression or return type position.
//
// ```
// fn foo() { 4<|>2i32 }
// ```
// ->
// ```
// fn foo() -> i32 { 42i32 }
// ```
pub(crate) fn infer_function_return_type(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    let (tail_expr, builder_edit_pos, wrap_expr) = extract_tail(ctx)?;
    let module = ctx.sema.scope(tail_expr.syntax()).module()?;
    let ty = ctx.sema.type_of_expr(&tail_expr)?;
    if ty.is_unit() {
        return None;
    }
    let ty = ty.display_source_code(ctx.db(), module.into()).ok()?;

    acc.add(
        AssistId("infer_function_return_type", AssistKind::RefactorRewrite),
        "Add this function's return type",
        tail_expr.syntax().text_range(),
        |builder| {
            match builder_edit_pos {
                InsertOrReplace::Insert(insert_pos) => {
                    builder.insert(insert_pos, &format!("-> {} ", ty))
                }
                InsertOrReplace::Replace(text_range) => {
                    builder.replace(text_range, &format!("-> {}", ty))
                }
            }
            if wrap_expr {
                mark::hit!(wrap_closure_non_block_expr);
                // `|x| x` becomes `|x| -> T x` which is invalid, so wrap it in a block
                builder.replace(tail_expr.syntax().text_range(), &format!("{{{}}}", tail_expr));
            }
        },
    )
}

enum InsertOrReplace {
    Insert(TextSize),
    Replace(TextRange),
}

/// Check the potentially already specified return type and reject it or turn it into a builder command
/// if allowed.
fn ret_ty_to_action(ret_ty: Option<ast::RetType>, insert_pos: TextSize) -> Option<InsertOrReplace> {
    match ret_ty {
        Some(ret_ty) => match ret_ty.ty() {
            Some(ast::Type::InferType(_)) | None => {
                mark::hit!(existing_infer_ret_type);
                mark::hit!(existing_infer_ret_type_closure);
                Some(InsertOrReplace::Replace(ret_ty.syntax().text_range()))
            }
            _ => {
                mark::hit!(existing_ret_type);
                mark::hit!(existing_ret_type_closure);
                None
            }
        },
        None => Some(InsertOrReplace::Insert(insert_pos + TextSize::from(1))),
    }
}

fn extract_tail(ctx: &AssistContext) -> Option<(ast::Expr, InsertOrReplace, bool)> {
    let (tail_expr, return_type_range, action, wrap_expr) =
        if let Some(closure) = ctx.find_node_at_offset::<ast::ClosureExpr>() {
            let rpipe_pos = closure.param_list()?.syntax().last_token()?.text_range().end();
            let action = ret_ty_to_action(closure.ret_type(), rpipe_pos)?;

            let body = closure.body()?;
            let body_start = body.syntax().first_token()?.text_range().start();
            let (tail_expr, wrap_expr) = match body {
                ast::Expr::BlockExpr(block) => (block.expr()?, false),
                body => (body, true),
            };

            let ret_range = TextRange::new(rpipe_pos, body_start);
            (tail_expr, ret_range, action, wrap_expr)
        } else {
            let func = ctx.find_node_at_offset::<ast::Fn>()?;
            let rparen_pos = func.param_list()?.r_paren_token()?.text_range().end();
            let action = ret_ty_to_action(func.ret_type(), rparen_pos)?;

            let body = func.body()?;
            let tail_expr = body.expr()?;

            let ret_range_end = body.l_curly_token()?.text_range().start();
            let ret_range = TextRange::new(rparen_pos, ret_range_end);
            (tail_expr, ret_range, action, false)
        };
    let frange = ctx.frange.range;
    if return_type_range.contains_range(frange) {
        mark::hit!(cursor_in_ret_position);
        mark::hit!(cursor_in_ret_position_closure);
    } else if tail_expr.syntax().text_range().contains_range(frange) {
        mark::hit!(cursor_on_tail);
        mark::hit!(cursor_on_tail_closure);
    } else {
        return None;
    }
    Some((tail_expr, action, wrap_expr))
}

#[cfg(test)]
mod tests {
    use crate::tests::{check_assist, check_assist_not_applicable};

    use super::*;

    #[test]
    fn infer_return_type_specified_inferred() {
        mark::check!(existing_infer_ret_type);
        check_assist(
            infer_function_return_type,
            r#"fn foo() -> <|>_ {
    45
}"#,
            r#"fn foo() -> i32 {
    45
}"#,
        );
    }

    #[test]
    fn infer_return_type_specified_inferred_closure() {
        mark::check!(existing_infer_ret_type_closure);
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    || -> _ {<|>45};
}"#,
            r#"fn foo() {
    || -> i32 {45};
}"#,
        );
    }

    #[test]
    fn infer_return_type_cursor_at_return_type_pos() {
        mark::check!(cursor_in_ret_position);
        check_assist(
            infer_function_return_type,
            r#"fn foo() <|>{
    45
}"#,
            r#"fn foo() -> i32 {
    45
}"#,
        );
    }

    #[test]
    fn infer_return_type_cursor_at_return_type_pos_closure() {
        mark::check!(cursor_in_ret_position_closure);
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    || <|>45
}"#,
            r#"fn foo() {
    || -> i32 {45}
}"#,
        );
    }

    #[test]
    fn infer_return_type() {
        mark::check!(cursor_on_tail);
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    45<|>
}"#,
            r#"fn foo() -> i32 {
    45
}"#,
        );
    }

    #[test]
    fn infer_return_type_nested() {
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    if true {
        3<|>
    } else {
        5
    }
}"#,
            r#"fn foo() -> i32 {
    if true {
        3
    } else {
        5
    }
}"#,
        );
    }

    #[test]
    fn not_applicable_ret_type_specified() {
        mark::check!(existing_ret_type);
        check_assist_not_applicable(
            infer_function_return_type,
            r#"fn foo() -> i32 {
    ( 45<|> + 32 ) * 123
}"#,
        );
    }

    #[test]
    fn not_applicable_non_tail_expr() {
        check_assist_not_applicable(
            infer_function_return_type,
            r#"fn foo() {
    let x = <|>3;
    ( 45 + 32 ) * 123
}"#,
        );
    }

    #[test]
    fn not_applicable_unit_return_type() {
        check_assist_not_applicable(
            infer_function_return_type,
            r#"fn foo() {
    (<|>)
}"#,
        );
    }

    #[test]
    fn infer_return_type_closure_block() {
        mark::check!(cursor_on_tail_closure);
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    |x: i32| {
        x<|>
    };
}"#,
            r#"fn foo() {
    |x: i32| -> i32 {
        x
    };
}"#,
        );
    }

    #[test]
    fn infer_return_type_closure() {
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    |x: i32| { x<|> };
}"#,
            r#"fn foo() {
    |x: i32| -> i32 { x };
}"#,
        );
    }

    #[test]
    fn infer_return_type_closure_wrap() {
        mark::check!(wrap_closure_non_block_expr);
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    |x: i32| x<|>;
}"#,
            r#"fn foo() {
    |x: i32| -> i32 {x};
}"#,
        );
    }

    #[test]
    fn infer_return_type_nested_closure() {
        check_assist(
            infer_function_return_type,
            r#"fn foo() {
    || {
        if true {
            3<|>
        } else {
            5
        }
    }
}"#,
            r#"fn foo() {
    || -> i32 {
        if true {
            3
        } else {
            5
        }
    }
}"#,
        );
    }

    #[test]
    fn not_applicable_ret_type_specified_closure() {
        mark::check!(existing_ret_type_closure);
        check_assist_not_applicable(
            infer_function_return_type,
            r#"fn foo() {
    || -> i32 { 3<|> }
}"#,
        );
    }

    #[test]
    fn not_applicable_non_tail_expr_closure() {
        check_assist_not_applicable(
            infer_function_return_type,
            r#"fn foo() {
    || -> i32 {
        let x = 3<|>;
        6
    }
}"#,
        );
    }
}
