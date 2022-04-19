use hir::HasSource;
use ide_db::assists::{AssistId, AssistKind};
use syntax::{
    ast::{self, edit::IndentLevel},
    AstNode, TextRange, TextSize,
};

use crate::assist_context::{AssistContext, Assists};

pub(crate) fn generate_enum_variant(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    let path_expr: ast::PathExpr = ctx.find_node_at_offset()?;
    let path = path_expr.path()?;

    if ctx.sema.resolve_path(&path).is_some() {
        // No need to generate anything if the path resolves.
        return None;
    }

    let name_ref = path.segment()?.name_ref()?;
    if let Some(hir::PathResolution::Def(hir::ModuleDef::Adt(hir::Adt::Enum(e)))) =
        ctx.sema.resolve_path(&path.qualifier()?)
    {
        let target = path.syntax().text_range();
        return add_variant_to_accumulator(acc, ctx, target, e, &name_ref);
    }

    None
}

fn add_variant_to_accumulator(
    acc: &mut Assists,
    ctx: &AssistContext,
    target: TextRange,
    adt: hir::Enum,
    name_ref: &ast::NameRef,
) -> Option<()> {
    let adt_ast = get_enum_ast(ctx, adt)?;

    let enum_indent_level = IndentLevel::from_node(&adt_ast.syntax());

    let offset = adt_ast.variant_list()?.syntax().text_range().end() - TextSize::of('}');

    let prefix = if adt_ast.variant_list()?.variants().next().is_none() {
        format!("\n{}", IndentLevel(1))
    } else {
        format!("{}", IndentLevel(1))
    };
    let text = format!("{}{},\n{}", prefix, name_ref, enum_indent_level);

    acc.add(
        AssistId("generate_variant", AssistKind::Generate),
        "Generate variant",
        target,
        |builder| {
            builder.insert(offset, text);
        },
    )
}

fn get_enum_ast(ctx: &AssistContext, adt: hir::Enum) -> Option<ast::Enum> {
    let range = adt.source(ctx.db())?.syntax().original_file_range(ctx.db());
    let file = ctx.sema.parse(range.file_id);
    let adt_ast: ast::Enum =
        ctx.sema.find_node_at_offset_with_macros(file.syntax(), range.range.start())?;

    Some(adt_ast)
}

#[cfg(test)]
mod tests {
    use crate::tests::{check_assist, check_assist_not_applicable};

    use super::*;

    #[test]
    fn generate_basic_enum_variant_in_empty_enum() {
        check_assist(
            generate_enum_variant,
            r"
enum Foo {}

fn main() {
    Foo::Bar$0
}
",
            r"
enum Foo {
    Bar,
}

fn main() {
    Foo::Bar
}
",
        )
    }

    #[test]
    fn generate_basic_enum_variant_in_non_empty_enum() {
        check_assist(
            generate_enum_variant,
            r"
enum Foo {
    Bar,
}

fn main() {
    Foo::Baz$0
}
",
            r"
enum Foo {
    Bar,
    Baz,
}

fn main() {
    Foo::Baz
}
",
        )
    }

    #[test]
    fn not_applicable_for_existing_variant() {
        check_assist_not_applicable(
            generate_enum_variant,
            r"
enum Foo {
    Bar,
}

fn main() {
    Foo::Bar$0
}
",
        )
    }

    #[test]
    fn indentation_level_is_correct() {
        check_assist(
            generate_enum_variant,
            r"
mod m {
    enum Foo {
        Bar,
    }
}

fn main() {
    m::Foo::Baz$0
}
",
            r"
mod m {
    enum Foo {
        Bar,
        Baz,
    }
}

fn main() {
    m::Foo::Baz
}
",
        )
    }
}
