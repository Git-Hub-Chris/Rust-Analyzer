use std::iter;

use ast::edit::IndentLevel;
use ide_db::base_db::AnchoredPathBuf;
use itertools::Itertools;
use stdx::format_to;
use syntax::{
    ast::{self, edit::AstNodeEdit, HasName},
    AstNode, TextRange,
};

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: move_module_to_file
//
// Moves inline module's contents to a separate file.
//
// ```
// mod $0foo {
//     fn t() {}
// }
// ```
// ->
// ```
// mod foo;
// ```
pub(crate) fn move_module_to_file(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    let module_ast = ctx.find_node_at_offset::<ast::Module>()?;
    let module_items = module_ast.item_list()?;

    let l_curly_offset = module_items.syntax().text_range().start();
    if l_curly_offset <= ctx.offset() {
        cov_mark::hit!(available_before_curly);
        return None;
    }
    let target = TextRange::new(module_ast.syntax().text_range().start(), l_curly_offset);

    let module_name = module_ast.name()?;

    // get to the outermost module syntax so we can grab the module of file we are in
    let outermost_mod_decl =
        iter::successors(Some(module_ast.clone()), |module| module.parent()).last()?;
    let module_def = ctx.sema.to_def(&outermost_mod_decl)?;
    let parent_module = module_def.parent(ctx.db())?;

    acc.add(
        AssistId("move_module_to_file", AssistKind::RefactorExtract),
        "Extract module to file",
        target,
        |builder| {
            let path = {
                let mut buf = String::from("./");
                match parent_module.name(ctx.db()) {
                    Some(name) if !parent_module.is_mod_rs(ctx.db()) => {
                        format_to!(buf, "{}/", name)
                    }
                    _ => (),
                }
                let segments = iter::successors(Some(module_ast.clone()), |module| module.parent())
                    .filter_map(|it| it.name())
                    .collect::<Vec<_>>();
                format_to!(buf, "{}", segments.into_iter().rev().format("/"));
                format_to!(buf, ".rs");
                buf
            };
            let contents = {
                let items = module_items.dedent(IndentLevel(1)).to_string();
                let mut items =
                    items.trim_start_matches('{').trim_end_matches('}').trim().to_string();
                if !items.is_empty() {
                    items.push('\n');
                }
                items
            };

            let buf = format!("mod {};", module_name);

            let replacement_start = match module_ast.mod_token() {
                Some(mod_token) => mod_token.text_range(),
                None => module_ast.syntax().text_range(),
            }
            .start();

            builder.replace(
                TextRange::new(replacement_start, module_ast.syntax().text_range().end()),
                buf,
            );

            let dst = AnchoredPathBuf { anchor: ctx.frange.file_id, path };
            builder.create_file(dst, contents);
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::tests::{check_assist, check_assist_not_applicable};

    use super::*;

    #[test]
    fn extract_from_root() {
        check_assist(
            move_module_to_file,
            r#"
mod $0tests {
    #[test] fn t() {}
}
"#,
            r#"
//- /main.rs
mod tests;
//- /tests.rs
#[test] fn t() {}
"#,
        );
    }

    #[test]
    fn extract_from_submodule() {
        check_assist(
            move_module_to_file,
            r#"
//- /main.rs
mod submod;
//- /submod.rs
$0mod inner {
    fn f() {}
}
fn g() {}
"#,
            r#"
//- /submod.rs
mod inner;
fn g() {}
//- /submod/inner.rs
fn f() {}
"#,
        );
    }

    #[test]
    fn extract_from_mod_rs() {
        check_assist(
            move_module_to_file,
            r#"
//- /main.rs
mod submodule;
//- /submodule/mod.rs
mod inner$0 {
    fn f() {}
}
fn g() {}
"#,
            r#"
//- /submodule/mod.rs
mod inner;
fn g() {}
//- /submodule/inner.rs
fn f() {}
"#,
        );
    }

    #[test]
    fn extract_public() {
        check_assist(
            move_module_to_file,
            r#"
pub mod $0tests {
    #[test] fn t() {}
}
"#,
            r#"
//- /main.rs
pub mod tests;
//- /tests.rs
#[test] fn t() {}
"#,
        );
    }

    #[test]
    fn extract_public_crate() {
        check_assist(
            move_module_to_file,
            r#"
pub(crate) mod $0tests {
    #[test] fn t() {}
}
"#,
            r#"
//- /main.rs
pub(crate) mod tests;
//- /tests.rs
#[test] fn t() {}
"#,
        );
    }

    #[test]
    fn available_before_curly() {
        cov_mark::check!(available_before_curly);
        check_assist_not_applicable(move_module_to_file, r#"mod m { $0 }"#);
    }

    #[test]
    fn keep_outer_comments_and_attributes() {
        check_assist(
            move_module_to_file,
            r#"
/// doc comment
#[attribute]
mod $0tests {
    #[test] fn t() {}
}
"#,
            r#"
//- /main.rs
/// doc comment
#[attribute]
mod tests;
//- /tests.rs
#[test] fn t() {}
"#,
        );
    }

    #[test]
    fn extract_nested() {
        check_assist(
            move_module_to_file,
            r#"
//- /lib.rs
mod foo;
//- /foo.rs
mod bar {
    mod baz {
        mod qux$0 {}
    }
}
"#,
            r#"
//- /foo.rs
mod bar {
    mod baz {
        mod qux;
    }
}
//- /foo/bar/baz/qux.rs
"#,
        );
    }
}
