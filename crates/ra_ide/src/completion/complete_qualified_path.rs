//! Completion of paths, i.e. `some::prefix::<|>`.

use hir::{Adt, HasVisibility, PathResolution, ScopeDef};
use ra_syntax::AstNode;
use test_utils::tested_by;

use crate::completion::{CompletionContext, Completions};

pub(super) fn complete_qualified_path(acc: &mut Completions, ctx: &CompletionContext) {
    let path = match &ctx.path_prefix {
        Some(path) => path.clone(),
        _ => return,
    };
    let scope = ctx.scope();
    let def = match scope.resolve_hir_path(&path) {
        Some(PathResolution::Def(def)) => def,
        _ => return,
    };
    let context_module = scope.module();
    match def {
        hir::ModuleDef::Module(module) => {
            let module_scope = module.scope(ctx.db, context_module);
            for (name, def) in module_scope {
                if ctx.use_item_syntax.is_some() {
                    if let ScopeDef::Unknown = def {
                        if let Some(name_ref) = ctx.name_ref_syntax.as_ref() {
                            if name_ref.syntax().text() == name.to_string().as_str() {
                                // for `use self::foo<|>`, don't suggest `foo` as a completion
                                tested_by!(dont_complete_current_use);
                                continue;
                            }
                        }
                    }
                }

                acc.add_resolution(ctx, name.to_string(), &def);
            }
        }
        hir::ModuleDef::Adt(_) | hir::ModuleDef::TypeAlias(_) => {
            if let hir::ModuleDef::Adt(Adt::Enum(e)) = def {
                for variant in e.variants(ctx.db) {
                    acc.add_enum_variant(ctx, variant, None);
                }
            }
            let ty = match def {
                hir::ModuleDef::Adt(adt) => adt.ty(ctx.db),
                hir::ModuleDef::TypeAlias(a) => a.ty(ctx.db),
                _ => unreachable!(),
            };
            if let Some(krate) = ctx.krate {
                let traits_in_scope = scope.traits_in_scope();
                ty.iterate_path_candidates(ctx.db, krate, &traits_in_scope, None, |_ty, item| {
                    if context_module.map_or(false, |m| !item.is_visible_from(ctx.db, m)) {
                        return None;
                    }
                    match item {
                        hir::AssocItem::Function(func) => {
                            acc.add_function(ctx, func, None);
                        }
                        hir::AssocItem::Const(ct) => acc.add_const(ctx, ct),
                        hir::AssocItem::TypeAlias(ty) => acc.add_type_alias(ctx, ty),
                    }
                    None::<()>
                });
            }
        }
        hir::ModuleDef::Trait(t) => {
            // Handles `Trait::assoc` as well as `<Ty as Trait>::assoc`.
            for item in t.items(ctx.db) {
                if context_module.map_or(false, |m| !item.is_visible_from(ctx.db, m)) {
                    continue;
                }
                match item {
                    hir::AssocItem::Function(func) => {
                        acc.add_function(ctx, func, None);
                    }
                    hir::AssocItem::Const(ct) => acc.add_const(ctx, ct),
                    hir::AssocItem::TypeAlias(ty) => acc.add_type_alias(ctx, ty),
                }
            }
        }
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use test_utils::covers;

    use crate::completion::{test_utils::do_completion, CompletionItem, CompletionKind};
    use insta::assert_debug_snapshot;

    fn do_reference_completion(code: &str) -> Vec<CompletionItem> {
        do_completion(code, CompletionKind::Reference)
    }

    #[test]
    fn dont_complete_current_use() {
        covers!(dont_complete_current_use);
        let completions = do_completion(r"use self::foo<|>;", CompletionKind::Reference);
        assert!(completions.is_empty());
    }

    #[test]
    fn dont_complete_current_use_in_braces_with_glob() {
        let completions = do_completion(
            r"
            mod foo { pub struct S; }
            use self::{foo::*, bar<|>};
            ",
            CompletionKind::Reference,
        );
        assert_eq!(completions.len(), 2);
    }

    #[test]
    fn dont_complete_primitive_in_use() {
        let completions = do_completion(r"use self::<|>;", CompletionKind::BuiltinType);
        assert!(completions.is_empty());
    }

    #[test]
    fn dont_complete_primitive_in_module_scope() {
        let completions = do_completion(r"fn foo() { self::<|> }", CompletionKind::BuiltinType);
        assert!(completions.is_empty());
    }

    #[test]
    fn completes_primitives() {
        let completions =
            do_completion(r"fn main() { let _: <|> = 92; }", CompletionKind::BuiltinType);
        assert_eq!(completions.len(), 17);
    }

    #[test]
    fn completes_mod_with_docs() {
        assert_debug_snapshot!(
            do_reference_completion(
                r"
                use self::my<|>;

                /// Some simple
                /// docs describing `mod my`.
                mod my {
                    struct Bar;
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "my",
                source_range: 27..29,
                delete: 27..29,
                insert: "my",
                kind: Module,
                documentation: Documentation(
                    "Some simple\ndocs describing `mod my`.",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn path_visibility() {
        assert_debug_snapshot!(
            do_reference_completion(
                r"
                use self::my::<|>;

                mod my {
                    struct Bar;
                    pub struct Foo;
                    pub use Bar as PublicBar;
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Foo",
                source_range: 31..31,
                delete: 31..31,
                insert: "Foo",
                kind: Struct,
            },
            CompletionItem {
                label: "PublicBar",
                source_range: 31..31,
                delete: 31..31,
                insert: "PublicBar",
                kind: Struct,
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_use_item_starting_with_self() {
        assert_debug_snapshot!(
            do_reference_completion(
                r"
                use self::m::<|>;

                mod m {
                    pub struct Bar;
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Bar",
                source_range: 30..30,
                delete: 30..30,
                insert: "Bar",
                kind: Struct,
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_use_item_starting_with_crate() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                mod foo;
                struct Spam;
                //- /foo.rs
                use crate::Sp<|>
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Spam",
                source_range: 11..13,
                delete: 11..13,
                insert: "Spam",
                kind: Struct,
            },
            CompletionItem {
                label: "foo",
                source_range: 11..13,
                delete: 11..13,
                insert: "foo",
                kind: Module,
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_nested_use_tree() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                mod foo;
                struct Spam;
                //- /foo.rs
                use crate::{Sp<|>};
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Spam",
                source_range: 12..14,
                delete: 12..14,
                insert: "Spam",
                kind: Struct,
            },
            CompletionItem {
                label: "foo",
                source_range: 12..14,
                delete: 12..14,
                insert: "foo",
                kind: Module,
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_deeply_nested_use_tree() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                mod foo;
                pub mod bar {
                    pub mod baz {
                        pub struct Spam;
                    }
                }
                //- /foo.rs
                use crate::{bar::{baz::Sp<|>}};
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Spam",
                source_range: 23..25,
                delete: 23..25,
                insert: "Spam",
                kind: Struct,
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_enum_variant() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                /// An enum
                enum E {
                    /// Foo Variant
                    Foo,
                    /// Bar Variant with i32
                    Bar(i32)
                }
                fn foo() { let _ = E::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Bar(…)",
                source_range: 116..116,
                delete: 116..116,
                insert: "Bar($0)",
                kind: EnumVariant,
                lookup: "Bar",
                detail: "(i32)",
                documentation: Documentation(
                    "Bar Variant with i32",
                ),
                trigger_call_info: true,
            },
            CompletionItem {
                label: "Foo",
                source_range: 116..116,
                delete: 116..116,
                insert: "Foo",
                kind: EnumVariant,
                detail: "()",
                documentation: Documentation(
                    "Foo Variant",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_enum_variant_with_details() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                struct S { field: u32 }
                /// An enum
                enum E {
                    /// Foo Variant (empty)
                    Foo,
                    /// Bar Variant with i32 and u32
                    Bar(i32, u32),
                    ///
                    S(S),
                }
                fn foo() { let _ = E::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "Bar(…)",
                source_range: 180..180,
                delete: 180..180,
                insert: "Bar($0)",
                kind: EnumVariant,
                lookup: "Bar",
                detail: "(i32, u32)",
                documentation: Documentation(
                    "Bar Variant with i32 and u32",
                ),
                trigger_call_info: true,
            },
            CompletionItem {
                label: "Foo",
                source_range: 180..180,
                delete: 180..180,
                insert: "Foo",
                kind: EnumVariant,
                detail: "()",
                documentation: Documentation(
                    "Foo Variant (empty)",
                ),
            },
            CompletionItem {
                label: "S(…)",
                source_range: 180..180,
                delete: 180..180,
                insert: "S($0)",
                kind: EnumVariant,
                lookup: "S",
                detail: "(S)",
                documentation: Documentation(
                    "",
                ),
                trigger_call_info: true,
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_struct_associated_method() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                /// A Struct
                struct S;

                impl S {
                    /// An associated method
                    fn m() { }
                }

                fn foo() { let _ = S::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "m()",
                source_range: 100..100,
                delete: 100..100,
                insert: "m()$0",
                kind: Function,
                lookup: "m",
                detail: "fn m()",
                documentation: Documentation(
                    "An associated method",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_struct_associated_method_with_self() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                /// A Struct
                struct S;

                impl S {
                    /// An associated method
                    fn m(&self) { }
                }

                fn foo() { let _ = S::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "m()",
                source_range: 105..105,
                delete: 105..105,
                insert: "m()$0",
                kind: Method,
                lookup: "m",
                detail: "fn m(&self)",
                documentation: Documentation(
                    "An associated method",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_struct_associated_const() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                /// A Struct
                struct S;

                impl S {
                    /// An associated const
                    const C: i32 = 42;
                }

                fn foo() { let _ = S::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "C",
                source_range: 107..107,
                delete: 107..107,
                insert: "C",
                kind: Const,
                detail: "const C: i32 = 42;",
                documentation: Documentation(
                    "An associated const",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_struct_associated_type() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                /// A Struct
                struct S;

                impl S {
                    /// An associated type
                    type T = i32;
                }

                fn foo() { let _ = S::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "T",
                source_range: 101..101,
                delete: 101..101,
                insert: "T",
                kind: TypeAlias,
                detail: "type T = i32;",
                documentation: Documentation(
                    "An associated type",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_trait_associated_type() {
        assert_debug_snapshot!(
            do_reference_completion(
                r"
                mod foo {
                    pub struct S;
                }
                trait Tr {
                    type Assoc;
                }
                impl Tr for foo::S {}

                fn f() {
                    foo::S::<|>
                }
                ",
            ),
            @r###"
        [
            CompletionItem {
                label: "Assoc",
                source_range: 248..248,
                delete: 248..248,
                insert: "Assoc",
                kind: TypeAlias,
                detail: "type Assoc;",
            },
        ]
        "###
        );
    }

    #[test]
    fn associated_item_visibility() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                struct S;

                mod m {
                    impl super::S {
                        pub(super) fn public_method() { }
                        fn private_method() { }
                        pub(super) type PublicType = u32;
                        type PrivateType = u32;
                        pub(super) const PUBLIC_CONST: u32 = 1;
                        const PRIVATE_CONST: u32 = 1;
                    }
                }

                fn foo() { let _ = S::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "PUBLIC_CONST",
                source_range: 302..302,
                delete: 302..302,
                insert: "PUBLIC_CONST",
                kind: Const,
                detail: "pub(super) const PUBLIC_CONST: u32 = 1;",
            },
            CompletionItem {
                label: "PublicType",
                source_range: 302..302,
                delete: 302..302,
                insert: "PublicType",
                kind: TypeAlias,
                detail: "pub(super) type PublicType = u32;",
            },
            CompletionItem {
                label: "public_method()",
                source_range: 302..302,
                delete: 302..302,
                insert: "public_method()$0",
                kind: Function,
                lookup: "public_method",
                detail: "pub(super) fn public_method()",
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_enum_associated_method() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                /// An enum
                enum S {};

                impl S {
                    /// An associated method
                    fn m() { }
                }

                fn foo() { let _ = S::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "m()",
                source_range: 100..100,
                delete: 100..100,
                insert: "m()$0",
                kind: Function,
                lookup: "m",
                detail: "fn m()",
                documentation: Documentation(
                    "An associated method",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_union_associated_method() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                /// A union
                union U {};

                impl U {
                    /// An associated method
                    fn m() { }
                }

                fn foo() { let _ = U::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "m()",
                source_range: 101..101,
                delete: 101..101,
                insert: "m()$0",
                kind: Function,
                lookup: "m",
                detail: "fn m()",
                documentation: Documentation(
                    "An associated method",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_use_paths_across_crates() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /main.rs
                use foo::<|>;

                //- /foo/lib.rs
                pub mod bar {
                    pub struct S;
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "bar",
                source_range: 9..9,
                delete: 9..9,
                insert: "bar",
                kind: Module,
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_trait_associated_method_1() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                trait Trait {
                  /// A trait method
                  fn m();
                }

                fn foo() { let _ = Trait::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "m()",
                source_range: 73..73,
                delete: 73..73,
                insert: "m()$0",
                kind: Function,
                lookup: "m",
                detail: "fn m()",
                documentation: Documentation(
                    "A trait method",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_trait_associated_method_2() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                trait Trait {
                  /// A trait method
                  fn m();
                }

                struct S;
                impl Trait for S {}

                fn foo() { let _ = S::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "m()",
                source_range: 99..99,
                delete: 99..99,
                insert: "m()$0",
                kind: Function,
                lookup: "m",
                detail: "fn m()",
                documentation: Documentation(
                    "A trait method",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_trait_associated_method_3() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                //- /lib.rs
                trait Trait {
                  /// A trait method
                  fn m();
                }

                struct S;
                impl Trait for S {}

                fn foo() { let _ = <S as Trait>::<|> }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "m()",
                source_range: 110..110,
                delete: 110..110,
                insert: "m()$0",
                kind: Function,
                lookup: "m",
                detail: "fn m()",
                documentation: Documentation(
                    "A trait method",
                ),
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_type_alias() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                struct S;
                impl S { fn foo() {} }
                type T = S;
                impl T { fn bar() {} }

                fn main() {
                    T::<|>;
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "bar()",
                source_range: 185..185,
                delete: 185..185,
                insert: "bar()$0",
                kind: Function,
                lookup: "bar",
                detail: "fn bar()",
            },
            CompletionItem {
                label: "foo()",
                source_range: 185..185,
                delete: 185..185,
                insert: "foo()$0",
                kind: Function,
                lookup: "foo",
                detail: "fn foo()",
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_qualified_macros() {
        assert_debug_snapshot!(
            do_reference_completion(
                "
                #[macro_export]
                macro_rules! foo {
                    () => {}
                }

                fn main() {
                    let _ = crate::<|>
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "foo!(…)",
                source_range: 179..179,
                delete: 179..179,
                insert: "foo!($0)",
                kind: Macro,
                detail: "#[macro_export]\nmacro_rules! foo",
            },
            CompletionItem {
                label: "main()",
                source_range: 179..179,
                delete: 179..179,
                insert: "main()$0",
                kind: Function,
                lookup: "main",
                detail: "fn main()",
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_reexported_items_under_correct_name() {
        assert_debug_snapshot!(
            do_reference_completion(
                r"
                fn foo() {
                    self::m::<|>
                }

                mod m {
                    pub use super::p::wrong_fn as right_fn;
                    pub use super::p::WRONG_CONST as RIGHT_CONST;
                    pub use super::p::WrongType as RightType;
                }
                mod p {
                    fn wrong_fn() {}
                    const WRONG_CONST: u32 = 1;
                    struct WrongType {};
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "RIGHT_CONST",
                source_range: 57..57,
                delete: 57..57,
                insert: "RIGHT_CONST",
                kind: Const,
            },
            CompletionItem {
                label: "RightType",
                source_range: 57..57,
                delete: 57..57,
                insert: "RightType",
                kind: Struct,
            },
            CompletionItem {
                label: "right_fn()",
                source_range: 57..57,
                delete: 57..57,
                insert: "right_fn()$0",
                kind: Function,
                lookup: "right_fn",
                detail: "fn wrong_fn()",
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_in_simple_macro_call() {
        let completions = do_reference_completion(
            r#"
                macro_rules! m { ($e:expr) => { $e } }
                fn main() { m!(self::f<|>); }
                fn foo() {}
            "#,
        );
        assert_debug_snapshot!(completions, @r###"
        [
            CompletionItem {
                label: "foo()",
                source_range: 93..94,
                delete: 93..94,
                insert: "foo()$0",
                kind: Function,
                lookup: "foo",
                detail: "fn foo()",
            },
            CompletionItem {
                label: "main()",
                source_range: 93..94,
                delete: 93..94,
                insert: "main()$0",
                kind: Function,
                lookup: "main",
                detail: "fn main()",
            },
        ]
        "###);
    }

    #[test]
    fn function_mod_share_name() {
        assert_debug_snapshot!(
        do_reference_completion(
                r"
                fn foo() {
                    self::m::<|>
                }

                mod m {
                    pub mod z {}
                    pub fn z() {}
                }
                ",
        ),
            @r###"
        [
            CompletionItem {
                label: "z",
                source_range: 57..57,
                delete: 57..57,
                insert: "z",
                kind: Module,
            },
            CompletionItem {
                label: "z()",
                source_range: 57..57,
                delete: 57..57,
                insert: "z()$0",
                kind: Function,
                lookup: "z",
                detail: "pub fn z()",
            },
        ]
        "###
        );
    }

    #[test]
    fn completes_hashmap_new() {
        assert_debug_snapshot!(
            do_reference_completion(
                r"
                struct RandomState;
                struct HashMap<K, V, S = RandomState> {}

                impl<K, V> HashMap<K, V, RandomState> {
                    pub fn new() -> HashMap<K, V, RandomState> { }
                }
                fn foo() {
                    HashMap::<|>
                }
                "
            ),
            @r###"
        [
            CompletionItem {
                label: "new()",
                source_range: 292..292,
                delete: 292..292,
                insert: "new()$0",
                kind: Function,
                lookup: "new",
                detail: "pub fn new() -> HashMap<K, V, RandomState>",
            },
        ]
        "###
        );
    }
}
