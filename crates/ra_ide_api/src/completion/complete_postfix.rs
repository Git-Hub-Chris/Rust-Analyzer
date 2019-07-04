use crate::{
    completion::{
        completion_item::{Completions, Builder, CompletionKind},
        completion_context::CompletionContext,
    },
    CompletionItem,
};
use ra_syntax::{ast::AstNode, TextRange};
use ra_text_edit::TextEditBuilder;

fn postfix_snippet(ctx: &CompletionContext, label: &str, detail: &str, snippet: &str) -> Builder {
    let edit = {
        let receiver_range = ctx.dot_receiver.expect("no receiver available").syntax().range();
        let delete_range = TextRange::from_to(receiver_range.start(), ctx.source_range().end());
        let mut builder = TextEditBuilder::default();
        builder.replace(delete_range, snippet.to_string());
        builder.finish()
    };
    CompletionItem::new(CompletionKind::Postfix, ctx.source_range(), label)
        .detail(detail)
        .snippet_edit(edit)
}

pub(super) fn complete_postfix(acc: &mut Completions, ctx: &CompletionContext) {
    if let Some(dot_receiver) = ctx.dot_receiver {
        let receiver_text = dot_receiver.syntax().text().to_string();
        postfix_snippet(ctx, "not", "!expr", &format!("!{}", receiver_text)).add_to(acc);
        postfix_snippet(ctx, "ref", "&expr", &format!("&{}", receiver_text)).add_to(acc);
        postfix_snippet(ctx, "refm", "&mut expr", &format!("&mut {}", receiver_text)).add_to(acc);
        postfix_snippet(ctx, "if", "if expr {}", &format!("if {} {{$0}}", receiver_text))
            .add_to(acc);
        postfix_snippet(
            ctx,
            "match",
            "match expr {}",
            &format!("match {} {{\n    ${{1:_}} => {{$0\\}},\n}}", receiver_text),
        )
        .add_to(acc);
        postfix_snippet(
            ctx,
            "while",
            "while expr {}",
            &format!("while {} {{\n$0\n}}", receiver_text),
        )
        .add_to(acc);
        postfix_snippet(ctx, "dbg", "dbg!(expr)", &format!("dbg!({})", receiver_text)).add_to(acc);
        postfix_snippet(ctx, "box", "Box::new(expr)", &format!("Box::new({})", receiver_text))
            .add_to(acc);
    }
}

#[cfg(test)]
mod tests {
    use crate::completion::{CompletionKind, check_completion};

    fn check_snippet_completion(test_name: &str, code: &str) {
        check_completion(test_name, code, CompletionKind::Postfix);
    }

    #[test]
    fn postfix_completion_works_for_trivial_path_expression() {
        check_snippet_completion(
            "postfix_completion_works_for_trivial_path_expression",
            r#"
            fn main() {
                let bar = "a";
                bar.<|>
            }
            "#,
        );
    }
}
