use ra_syntax::{AstNode, ast};

use crate::{
    assists::{AssistCtx, Assist},
    formatting::extract_trivial_expression,
};

pub fn replace_if_let_with_match(ctx: AssistCtx) -> Option<Assist> {
    let if_expr: &ast::IfExpr = ctx.node_at_offset()?;
    let cond = if_expr.condition()?;
    let pat = cond.pat()?;
    let expr = cond.expr()?;
    let then_block = if_expr.then_branch()?;
    let else_block = if_expr.else_branch()?;

    ctx.build("replace with match", |edit| {
        let match_expr = build_match_expr(expr, pat, then_block, else_block);
        edit.replace_node_and_indent(if_expr.syntax(), match_expr);
        edit.set_cursor(if_expr.syntax().range().start())
    })
}

fn build_match_expr(
    expr: &ast::Expr,
    pat1: &ast::Pat,
    arm1: &ast::Block,
    arm2: &ast::Block,
) -> String {
    let mut buf = String::new();
    buf.push_str(&format!("match {} {{\n", expr.syntax().text()));
    buf.push_str(&format!("    {} => {}\n", pat1.syntax().text(), format_arm(arm1)));
    buf.push_str(&format!("    _ => {}\n", format_arm(arm2)));
    buf.push_str("}");
    buf
}

fn format_arm(block: &ast::Block) -> String {
    match extract_trivial_expression(block) {
        None => block.syntax().text().to_string(),
        Some(e) => format!("{},", e.syntax().text()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assists::check_assist;

    #[test]
    fn test_replace_if_let_with_match_unwraps_simple_expressions() {
        check_assist(
            replace_if_let_with_match,
            "
impl VariantData {
    pub fn is_struct(&self) -> bool {
        if <|>let VariantData::Struct(..) = *self {
            true
        } else {
            false
        }
    }
}           ",
            "
impl VariantData {
    pub fn is_struct(&self) -> bool {
        <|>match *self {
            VariantData::Struct(..) => true,
            _ => false,
        }
    }
}           ",
        )
    }
}
