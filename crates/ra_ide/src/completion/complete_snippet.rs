//! FIXME: write short doc here

use crate::completion::{
    completion_item::Builder, CompletionContext, CompletionItem, CompletionItemKind,
    CompletionKind, Completions,
};

fn snippet(ctx: &CompletionContext, label: &str, snippet: &str) -> Builder {
    CompletionItem::new(CompletionKind::Snippet, ctx.source_range(), label)
        .insert_snippet(snippet)
        .kind(CompletionItemKind::Snippet)
}

pub(super) fn complete_expr_snippet(acc: &mut Completions, ctx: &CompletionContext) {
    if !(ctx.is_trivial_path && ctx.function_syntax.is_some()) {
        return;
    }

    snippet(ctx, "pd", "eprintln!(\"$0 = {:?}\", $0);").add_to(acc);
    snippet(ctx, "ppd", "eprintln!(\"$0 = {:#?}\", $0);").add_to(acc);
}

pub(super) fn complete_item_snippet(acc: &mut Completions, ctx: &CompletionContext) {
    if !ctx.is_new_item {
        return;
    }
    snippet(
        ctx,
        "Test function",
        "\
#[test]
fn ${1:feature}() {
    $0
}",
    )
    .lookup_by("tfn")
    .add_to(acc);

    snippet(ctx, "macro_rules", "macro_rules! $1 {\n\t($2) => {\n\t\t$0\n\t};\n}").add_to(acc);
    snippet(ctx, "pub(crate)", "pub(crate) $0").add_to(acc);
    snippet(ctx, "derive", "#[derive($0)]").add_to(acc);
}

#[cfg(test)]
mod tests {
    use crate::completion::{test_utils::do_completion, CompletionItem, CompletionKind};
    use insta::assert_debug_snapshot;

    fn do_snippet_completion(code: &str) -> Vec<CompletionItem> {
        do_completion(code, CompletionKind::Snippet)
    }

    #[test]
    fn completes_snippets_in_expressions() {
        assert_debug_snapshot!(
                    do_snippet_completion(r"fn foo(x: i32) { <|> }"),
        @r###"
        [
            CompletionItem {
                label: "pd",
                source_range: [17; 17),
                delete: [17; 17),
                insert: "eprintln!(\"$0 = {:?}\", $0);",
                kind: Snippet,
            },
            CompletionItem {
                label: "ppd",
                source_range: [17; 17),
                delete: [17; 17),
                insert: "eprintln!(\"$0 = {:#?}\", $0);",
                kind: Snippet,
            },
        ]
        "###
                );
    }

    #[test]
    fn should_not_complete_snippets_in_path() {
        assert_debug_snapshot!(
                    do_snippet_completion(r"fn foo(x: i32) { ::foo<|> }"),
        @"[]"
                );
        assert_debug_snapshot!(
                    do_snippet_completion(r"fn foo(x: i32) { ::<|> }"),
        @"[]"
                );
    }

    #[test]
    fn completes_snippets_in_items() {
        assert_debug_snapshot!(
            do_snippet_completion(
                r"
                #[cfg(test)]
                mod tests {
                    <|>
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Test function",
                source_range: [78; 78),
                delete: [78; 78),
                insert: "#[test]\nfn ${1:feature}() {\n    $0\n}",
                kind: Snippet,
                lookup: "tfn",
            },
            CompletionItem {
                label: "macro_rules",
                source_range: [78; 78),
                delete: [78; 78),
                insert: "macro_rules! $1 {\n\t($2) => {\n\t\t$0\n\t};\n}",
                kind: Snippet,
            },
            CompletionItem {
                label: "pub(crate)",
                source_range: [78; 78),
                delete: [78; 78),
                insert: "pub(crate) $0",
                kind: Snippet,
            },
        ]
        "###
        );
    }
}
