use std::iter::once;

use hir::{
    Adt, AsAssocItem, AssocItemContainer, FieldSource, HasSource, HirDisplay, Module, ModuleDef,
    ModuleSource, Semantics, Substs,
};
use itertools::Itertools;
use ra_db::SourceDatabase;
use ra_ide_db::{
    defs::{classify_name, classify_name_ref, Definition},
    RootDatabase,
};
use ra_syntax::{
    ast::{self, DocCommentsOwner},
    match_ast, AstNode,
    SyntaxKind::*,
    SyntaxToken, TokenAtOffset,
};

use crate::{
    display::{macro_label, rust_code_markup, rust_code_markup_with_doc, ShortLabel, ToNav},
    runnables::runnable,
    FileId, FilePosition, NavigationTarget, RangeInfo, Runnable,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverConfig {
    pub run: bool,
    pub debug: bool,
    pub goto_type_def: bool,
    pub implementations: bool,
}

impl Default for HoverConfig {
    fn default() -> Self {
        // A client should explicitly specify if it supports hover actions!
        Self { run: false, debug: false, goto_type_def: false, implementations: false }
    }
}

impl HoverConfig {
    pub fn runnable(&self) -> bool {
        self.run || self.debug
    }

    pub fn any(&self) -> bool {
        self.implementations || self.goto_type_def || self.runnable()
    }

    pub fn none(&self) -> bool {
        !self.any()
    }
}

#[derive(Debug)]
pub struct HoverGotoTypeData {
    pub hint: String,
    pub link: NavigationTarget,
}

impl HoverGotoTypeData {
    fn new(hint: String, link: NavigationTarget) -> Self {
        Self { hint, link }
    }
}

#[derive(Debug)]
pub enum HoverAction {
    Runnable(Runnable),
    GoToType(Vec<HoverGotoTypeData>),
    Implementaion(FilePosition),
}

/// Contains the results when hovering over an item
#[derive(Debug, Default)]
pub struct HoverResult {
    results: Vec<String>,
    actions: Vec<HoverAction>,
}

impl HoverResult {
    pub fn new() -> HoverResult {
        Self::default()
    }

    pub fn extend(&mut self, item: Option<String>) {
        self.results.extend(item);
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }

    pub fn first(&self) -> Option<&str> {
        self.results.first().map(String::as_str)
    }

    pub fn results(&self) -> &[String] {
        &self.results
    }

    pub fn actions(&self) -> &[HoverAction] {
        &self.actions
    }

    pub fn append_action(&mut self, action: HoverAction) {
        self.actions.push(action);
    }

    /// Returns the results converted into markup
    /// for displaying in a UI
    ///
    /// Does not process actions!
    pub fn to_markup(&self) -> String {
        self.results.join("\n\n---\n")
    }
}

fn hover_text(
    docs: Option<String>,
    desc: Option<String>,
    mod_path: Option<String>,
) -> Option<String> {
    if let Some(desc) = desc {
        Some(rust_code_markup_with_doc(&desc, docs.as_deref(), mod_path.as_deref()))
    } else {
        docs
    }
}

fn definition_owner_name(db: &RootDatabase, def: &Definition) -> Option<String> {
    match def {
        Definition::Field(f) => Some(f.parent_def(db).name(db)),
        Definition::Local(l) => l.parent(db).name(db),
        Definition::ModuleDef(md) => match md {
            ModuleDef::Function(f) => match f.as_assoc_item(db)?.container(db) {
                AssocItemContainer::Trait(t) => Some(t.name(db)),
                AssocItemContainer::ImplDef(i) => i.target_ty(db).as_adt().map(|adt| adt.name(db)),
            },
            ModuleDef::EnumVariant(e) => Some(e.parent_enum(db).name(db)),
            _ => None,
        },
        Definition::SelfType(i) => i.target_ty(db).as_adt().map(|adt| adt.name(db)),
        _ => None,
    }
    .map(|name| name.to_string())
}

fn determine_mod_path(db: &RootDatabase, module: Module, name: Option<String>) -> String {
    once(db.crate_graph()[module.krate().into()].display_name.as_ref().map(ToString::to_string))
        .chain(
            module
                .path_to_root(db)
                .into_iter()
                .rev()
                .map(|it| it.name(db).map(|name| name.to_string())),
        )
        .chain(once(name))
        .flatten()
        .join("::")
}

fn adt_mod_path(db: &RootDatabase, adt: &Adt) -> String {
    determine_mod_path(db, adt.module(db), Some(adt.name(db).to_string()))
}

fn definition_mod_path(db: &RootDatabase, def: &Definition) -> Option<String> {
    def.module(db).map(|module| determine_mod_path(db, module, definition_owner_name(db, def)))
}

fn hover_text_from_name_kind(db: &RootDatabase, def: Definition) -> Option<String> {
    let mod_path = definition_mod_path(db, &def);
    return match def {
        Definition::Macro(it) => {
            let src = it.source(db);
            hover_text(src.value.doc_comment_text(), Some(macro_label(&src.value)), mod_path)
        }
        Definition::Field(it) => {
            let src = it.source(db);
            match src.value {
                FieldSource::Named(it) => {
                    hover_text(it.doc_comment_text(), it.short_label(), mod_path)
                }
                _ => None,
            }
        }
        Definition::ModuleDef(it) => match it {
            ModuleDef::Module(it) => match it.definition_source(db).value {
                ModuleSource::Module(it) => {
                    hover_text(it.doc_comment_text(), it.short_label(), mod_path)
                }
                _ => None,
            },
            ModuleDef::Function(it) => from_def_source(db, it, mod_path),
            ModuleDef::Adt(Adt::Struct(it)) => from_def_source(db, it, mod_path),
            ModuleDef::Adt(Adt::Union(it)) => from_def_source(db, it, mod_path),
            ModuleDef::Adt(Adt::Enum(it)) => from_def_source(db, it, mod_path),
            ModuleDef::EnumVariant(it) => from_def_source(db, it, mod_path),
            ModuleDef::Const(it) => from_def_source(db, it, mod_path),
            ModuleDef::Static(it) => from_def_source(db, it, mod_path),
            ModuleDef::Trait(it) => from_def_source(db, it, mod_path),
            ModuleDef::TypeAlias(it) => from_def_source(db, it, mod_path),
            ModuleDef::BuiltinType(it) => Some(it.to_string()),
        },
        Definition::Local(it) => Some(rust_code_markup(&it.ty(db).display(db))),
        Definition::TypeParam(_) | Definition::SelfType(_) => {
            // FIXME: Hover for generic param
            None
        }
    };

    fn from_def_source<A, D>(db: &RootDatabase, def: D, mod_path: Option<String>) -> Option<String>
    where
        D: HasSource<Ast = A>,
        A: ast::DocCommentsOwner + ast::NameOwner + ShortLabel,
    {
        let src = def.source(db);
        hover_text(src.value.doc_comment_text(), src.value.short_label(), mod_path)
    }
}

fn goto_type_action(db: &RootDatabase, def: Definition) -> Option<HoverAction> {
    fn add_subst_targets(
        db: &RootDatabase,
        substs: Option<&Substs>,
        acc: &mut Vec<HoverGotoTypeData>,
    ) {
        if let Some(substs) = substs {
            for ty in substs.iter() {
                if let Some((adt_id, s)) = ty.as_adt() {
                    let adt = Adt::from(adt_id);
                    let mod_path = adt_mod_path(db, &adt);
                    acc.push(HoverGotoTypeData::new(mod_path, adt.to_nav(db)));
                    add_subst_targets(db, Some(s), acc);
                }
            }
        }
    }

    fn to_impl_action(nav_target: NavigationTarget) -> HoverAction {
        HoverAction::Implementaion(FilePosition {
            file_id: nav_target.file_id(),
            offset: nav_target.range().start(),
        })
    }

    match def {
        Definition::Local(it) => {
            let ty = it.ty(db);
            let adt: Adt = ty.autoderef(db).find_map(|ty| ty.as_adt())?;
            let mod_path = adt_mod_path(db, &adt);
            let mut targets = vec![HoverGotoTypeData::new(mod_path, adt.to_nav(db))];
            add_subst_targets(db, ty.substs().as_ref(), &mut targets);

            Some(HoverAction::GoToType(targets))
        }
        Definition::ModuleDef(it) => match it {
            ModuleDef::Adt(Adt::Struct(it)) => Some(to_impl_action(it.to_nav(db))),
            ModuleDef::Adt(Adt::Union(it)) => Some(to_impl_action(it.to_nav(db))),
            ModuleDef::Adt(Adt::Enum(it)) => Some(to_impl_action(it.to_nav(db))),
            ModuleDef::Trait(it) => Some(to_impl_action(it.to_nav(db))),
            _ => None,
        },
        _ => None,
    }

    /*
    FilePosition
    let nav_info = match world.analysis().goto_implementation(position)? {
        None => return Ok(None),
        Some(it) => it,
    };
    */
}

fn runnable_action(
    sema: &Semantics<RootDatabase>,
    def: Definition,
    file_id: FileId,
) -> Option<HoverAction> {
    match def {
        Definition::ModuleDef(it) => match it {
            ModuleDef::Module(it) => match it.definition_source(sema.db).value {
                ModuleSource::Module(it) => runnable(&sema, it.syntax().clone(), file_id)
                    .map(|it| HoverAction::Runnable(it)),
                _ => None,
            },
            ModuleDef::Function(it) => {
                runnable(&sema, it.source(sema.db).value.syntax().clone(), file_id)
                    .map(|it| HoverAction::Runnable(it))
            }
            _ => None,
        },
        _ => None,
    }
}

// Feature: Hover
//
// Shows additional information, like type of an expression or documentation for definition when "focusing" code.
// Focusing is usually hovering with a mouse, but can also be triggered with a shortcut.
pub(crate) fn hover(db: &RootDatabase, position: FilePosition) -> Option<RangeInfo<HoverResult>> {
    let sema = Semantics::new(db);
    let file = sema.parse(position.file_id).syntax().clone();
    let token = pick_best(file.token_at_offset(position.offset))?;
    let token = sema.descend_into_macros(token);

    let mut res = HoverResult::new();

    if let Some((node, name_kind)) = match_ast! {
        match (token.parent()) {
            ast::NameRef(name_ref) => {
                classify_name_ref(&sema, &name_ref).map(|d| (name_ref.syntax().clone(), d.definition()))
            },
            ast::Name(name) => {
                classify_name(&sema, &name).map(|d| (name.syntax().clone(), d.definition()))
            },
            _ => None,
        }
    } {
        let range = sema.original_range(&node).range;
        res.extend(hover_text_from_name_kind(db, name_kind));

        if !res.is_empty() {
            if let Some(action) = runnable_action(&sema, name_kind, position.file_id) {
                res.append_action(action);
            }

            if let Some(action) = goto_type_action(db, name_kind) {
                res.append_action(action);
            }

            return Some(RangeInfo::new(range, res));
        }
    }

fn pick_best(tokens: TokenAtOffset<SyntaxToken>) -> Option<SyntaxToken> {
    return tokens.max_by_key(priority);
    fn priority(n: &SyntaxToken) -> usize {
        match n.kind() {
            IDENT | INT_NUMBER => 3,
            L_PAREN | R_PAREN => 2,
            kind if kind.is_trivia() => 0,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use ra_db::FileLoader;
    use ra_syntax::TextRange;

    use crate::mock_analysis::{analysis_and_position, single_file_with_position};

    fn trim_markup(s: &str) -> &str {
        s.trim_start_matches("```rust\n").trim_end_matches("\n```")
    }

    fn trim_markup_opt(s: Option<&str>) -> Option<&str> {
        s.map(trim_markup)
    }

    fn check_hover_result(fixture: &str, expected: &[&str]) -> String {
        let (analysis, position) = analysis_and_position(fixture);
        let hover = analysis.hover(position).unwrap().unwrap();
        let mut results = Vec::from(hover.info.results());
        results.sort();

        for (markup, expected) in
            results.iter().zip(expected.iter().chain(std::iter::repeat(&"<missing>")))
        {
            assert_eq!(trim_markup(&markup), *expected);
        }

        assert_eq!(hover.info.len(), expected.len());

        let content = analysis.db.file_text(position.file_id);
        content[hover.range].to_string()
    }

    fn check_hover_no_result(fixture: &str) {
        let (analysis, position) = analysis_and_position(fixture);
        assert!(analysis.hover(position).unwrap().is_none());
    }

    #[test]
    fn hover_shows_type_of_an_expression() {
        let (analysis, position) = single_file_with_position(
            "
            pub fn foo() -> u32 { 1 }

            fn main() {
                let foo_test = foo()<|>;
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(hover.range, TextRange::new(95.into(), 100.into()));
        assert_eq!(trim_markup_opt(hover.info.first()), Some("u32"));
    }

    #[test]
    fn hover_shows_long_type_of_an_expression() {
        check_hover_result(
            r#"
            //- /main.rs
            struct Scan<A, B, C> {
                a: A,
                b: B,
                c: C,
            }

            struct FakeIter<I> {
                inner: I,
            }

            struct OtherStruct<T> {
                i: T,
            }

            enum FakeOption<T> {
                Some(T),
                None,
            }

            fn scan<A, B, C>(a: A, b: B, c: C) -> FakeIter<Scan<OtherStruct<A>, B, C>> {
                FakeIter { inner: Scan { a, b, c } }
            }

            fn main() {
                let num: i32 = 55;
                let closure = |memo: &mut u32, value: &u32, _another: &mut u32| -> FakeOption<u32> {
                    FakeOption::Some(*memo + value)
                };
                let number = 5u32;
                let mut iter<|> = scan(OtherStruct { i: num }, closure, number);
            }
            "#,
            &["FakeIter<Scan<OtherStruct<OtherStruct<i32>>, |&mut u32, &u32, &mut u32| -> FakeOption<u32>, u32>>"],
        );
    }

    #[test]
    fn hover_shows_fn_signature() {
        // Single file with result
        check_hover_result(
            r#"
            //- /main.rs
            pub fn foo() -> u32 { 1 }

            fn main() {
                let foo_test = fo<|>o();
            }
        "#,
            &["pub fn foo() -> u32"],
        );

        // Multiple candidates but results are ambiguous.
        check_hover_result(
            r#"
            //- /a.rs
            pub fn foo() -> u32 { 1 }

            //- /b.rs
            pub fn foo() -> &str { "" }

            //- /c.rs
            pub fn foo(a: u32, b: u32) {}

            //- /main.rs
            mod a;
            mod b;
            mod c;

            fn main() {
                let foo_test = fo<|>o();
            }
        "#,
            &["{unknown}"],
        );
    }

    #[test]
    fn hover_shows_fn_signature_with_type_params() {
        check_hover_result(
            r#"
            //- /main.rs
            pub fn foo<'a, T: AsRef<str>>(b: &'a T) -> &'a str { }

            fn main() {
                let foo_test = fo<|>o();
            }
        "#,
            &["pub fn foo<'a, T: AsRef<str>>(b: &'a T) -> &'a str"],
        );
    }

    #[test]
    fn hover_shows_fn_signature_on_fn_name() {
        check_hover_result(
            r#"
            //- /main.rs
            pub fn foo<|>(a: u32, b: u32) -> u32 {}

            fn main() {
            }
        "#,
            &["pub fn foo(a: u32, b: u32) -> u32"],
        );
    }

    #[test]
    fn hover_shows_struct_field_info() {
        // Hovering over the field when instantiating
        check_hover_result(
            r#"
            //- /main.rs
            struct Foo {
                field_a: u32,
            }

            fn main() {
                let foo = Foo {
                    field_a<|>: 0,
                };
            }
        "#,
            &["Foo\n```\n\n```rust\nfield_a: u32"],
        );

        // Hovering over the field in the definition
        check_hover_result(
            r#"
            //- /main.rs
            struct Foo {
                field_a<|>: u32,
            }

            fn main() {
                let foo = Foo {
                    field_a: 0,
                };
            }
        "#,
            &["Foo\n```\n\n```rust\nfield_a: u32"],
        );
    }

    #[test]
    fn hover_const_static() {
        check_hover_result(
            r#"
            //- /main.rs
            const foo<|>: u32 = 0;
        "#,
            &["const foo: u32"],
        );

        check_hover_result(
            r#"
            //- /main.rs
            static foo<|>: u32 = 0;
        "#,
            &["static foo: u32"],
        );
    }

    #[test]
    fn hover_default_generic_types() {
        check_hover_result(
            r#"
//- /main.rs
struct Test<K, T = u8> {
    k: K,
    t: T,
}

fn main() {
    let zz<|> = Test { t: 23, k: 33 };
}"#,
            &["Test<i32, u8>"],
        );
    }

    #[test]
    fn hover_some() {
        let (analysis, position) = single_file_with_position(
            "
            enum Option<T> { Some(T) }
            use Option::Some;

            fn main() {
                So<|>me(12);
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("Option\n```\n\n```rust\nSome"));

        let (analysis, position) = single_file_with_position(
            "
            enum Option<T> { Some(T) }
            use Option::Some;

            fn main() {
                let b<|>ar = Some(12);
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("Option<i32>"));
    }

    #[test]
    fn hover_enum_variant() {
        check_hover_result(
            r#"
            //- /main.rs
            enum Option<T> {
                /// The None variant
                Non<|>e
            }
        "#,
            &["
Option
```

```rust
None
```
___

The None variant
            "
            .trim()],
        );

        check_hover_result(
            r#"
            //- /main.rs
            enum Option<T> {
                /// The Some variant
                Some(T)
            }
            fn main() {
                let s = Option::Som<|>e(12);
            }
        "#,
            &["
Option
```

```rust
Some
```
___

The Some variant
            "
            .trim()],
        );
    }

    #[test]
    fn hover_for_local_variable() {
        let (analysis, position) = single_file_with_position("fn func(foo: i32) { fo<|>o; }");
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("i32"));
    }

    #[test]
    fn hover_for_local_variable_pat() {
        let (analysis, position) = single_file_with_position("fn func(fo<|>o: i32) {}");
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("i32"));
    }

    #[test]
    fn hover_local_var_edge() {
        let (analysis, position) = single_file_with_position(
            "
fn func(foo: i32) { if true { <|>foo; }; }
",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("i32"));
    }

    #[test]
    fn hover_for_param_edge() {
        let (analysis, position) = single_file_with_position("fn func(<|>foo: i32) {}");
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("i32"));
    }

    #[test]
    fn test_hover_infer_associated_method_result() {
        let (analysis, position) = single_file_with_position(
            "
            struct Thing { x: u32 }

            impl Thing {
                fn new() -> Thing {
                    Thing { x: 0 }
                }
            }

            fn main() {
                let foo_<|>test = Thing::new();
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("Thing"));
    }

    #[test]
    fn test_hover_infer_associated_method_exact() {
        let (analysis, position) = single_file_with_position(
            "
            mod wrapper {
                struct Thing { x: u32 }

                impl Thing {
                    fn new() -> Thing {
                        Thing { x: 0 }
                    }
                }
            }

            fn main() {
                let foo_test = wrapper::Thing::new<|>();
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(
            trim_markup_opt(hover.info.first()),
            Some("wrapper::Thing\n```\n\n```rust\nfn new() -> Thing")
        );
    }

    #[test]
    fn test_hover_infer_associated_const_in_pattern() {
        let (analysis, position) = single_file_with_position(
            "
            struct X;
            impl X {
                const C: u32 = 1;
            }

            fn main() {
                match 1 {
                    X::C<|> => {},
                    2 => {},
                    _ => {}
                };
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("const C: u32"));
    }

    #[test]
    fn test_hover_self() {
        let (analysis, position) = single_file_with_position(
            "
            struct Thing { x: u32 }
            impl Thing {
                fn new() -> Self {
                    Self<|> { x: 0 }
                }
            }
        ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("Thing"));

        /* FIXME: revive these tests
                let (analysis, position) = single_file_with_position(
                    "
                    struct Thing { x: u32 }
                    impl Thing {
                        fn new() -> Self<|> {
                            Self { x: 0 }
                        }
                    }
                    ",
                );

                let hover = analysis.hover(position).unwrap().unwrap();
                assert_eq!(trim_markup_opt(hover.info.first()), Some("Thing"));

                let (analysis, position) = single_file_with_position(
                    "
                    enum Thing { A }
                    impl Thing {
                        pub fn new() -> Self<|> {
                            Thing::A
                        }
                    }
                    ",
                );
                let hover = analysis.hover(position).unwrap().unwrap();
                assert_eq!(trim_markup_opt(hover.info.first()), Some("enum Thing"));

                let (analysis, position) = single_file_with_position(
                    "
                    enum Thing { A }
                    impl Thing {
                        pub fn thing(a: Self<|>) {
                        }
                    }
                    ",
                );
                let hover = analysis.hover(position).unwrap().unwrap();
                assert_eq!(trim_markup_opt(hover.info.first()), Some("enum Thing"));
        */
    }

    #[test]
    fn test_hover_shadowing_pat() {
        let (analysis, position) = single_file_with_position(
            "
            fn x() {}

            fn y() {
                let x = 0i32;
                x<|>;
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("i32"));
    }

    #[test]
    fn test_hover_macro_invocation() {
        let (analysis, position) = single_file_with_position(
            "
            macro_rules! foo {
                () => {}
            }

            fn f() {
                fo<|>o!();
            }
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("macro_rules! foo"));
    }

    #[test]
    fn test_hover_tuple_field() {
        let (analysis, position) = single_file_with_position(
            "
            struct TS(String, i32<|>);
            ",
        );
        let hover = analysis.hover(position).unwrap().unwrap();
        assert_eq!(trim_markup_opt(hover.info.first()), Some("i32"));
    }

    #[test]
    fn test_hover_through_macro() {
        let hover_on = check_hover_result(
            "
            //- /lib.rs
            macro_rules! id {
                ($($tt:tt)*) => { $($tt)* }
            }
            fn foo() {}
            id! {
                fn bar() {
                    fo<|>o();
                }
            }
            ",
            &["fn foo()"],
        );

        assert_eq!(hover_on, "foo")
    }

    #[test]
    fn test_hover_through_expr_in_macro() {
        let hover_on = check_hover_result(
            "
            //- /lib.rs
            macro_rules! id {
                ($($tt:tt)*) => { $($tt)* }
            }
            fn foo(bar:u32) {
                let a = id!(ba<|>r);
            }
            ",
            &["u32"],
        );

        assert_eq!(hover_on, "bar")
    }

    #[test]
    fn test_hover_through_expr_in_macro_recursive() {
        let hover_on = check_hover_result(
            "
            //- /lib.rs
            macro_rules! id_deep {
                ($($tt:tt)*) => { $($tt)* }
            }
            macro_rules! id {
                ($($tt:tt)*) => { id_deep!($($tt)*) }
            }
            fn foo(bar:u32) {
                let a = id!(ba<|>r);
            }
            ",
            &["u32"],
        );

        assert_eq!(hover_on, "bar")
    }

    #[test]
    fn test_hover_through_func_in_macro_recursive() {
        let hover_on = check_hover_result(
            "
            //- /lib.rs
            macro_rules! id_deep {
                ($($tt:tt)*) => { $($tt)* }
            }
            macro_rules! id {
                ($($tt:tt)*) => { id_deep!($($tt)*) }
            }
            fn bar() -> u32 {
                0
            }
            fn foo() {
                let a = id!([0u32, bar(<|>)] );
            }
            ",
            &["u32"],
        );

        assert_eq!(hover_on, "bar()")
    }

    #[test]
    fn test_hover_through_literal_string_in_macro() {
        let hover_on = check_hover_result(
            r#"
            //- /lib.rs
            macro_rules! arr {
                ($($tt:tt)*) => { [$($tt)*)] }
            }
            fn foo() {
                let mastered_for_itunes = "";
                let _ = arr!("Tr<|>acks", &mastered_for_itunes);
            }
            "#,
            &["&str"],
        );

        assert_eq!(hover_on, "\"Tracks\"");
    }

    #[test]
    fn test_hover_through_assert_macro() {
        let hover_on = check_hover_result(
            r#"
            //- /lib.rs
            #[rustc_builtin_macro]
            macro_rules! assert {}

            fn bar() -> bool { true }
            fn foo() {
                assert!(ba<|>r());
            }
            "#,
            &["fn bar() -> bool"],
        );

        assert_eq!(hover_on, "bar");
    }

    #[test]
    fn test_hover_through_literal_string_in_builtin_macro() {
        check_hover_no_result(
            r#"
            //- /lib.rs
            #[rustc_builtin_macro]
            macro_rules! format {}

            fn foo() {
                format!("hel<|>lo {}", 0);
            }
            "#,
        );
    }

    #[test]
    fn test_hover_non_ascii_space_doc() {
        check_hover_result(
            "
            //- /lib.rs
            ///　<- `\u{3000}` here
            fn foo() {
            }

            fn bar() {
                fo<|>o();
            }
            ",
            &["fn foo()\n```\n___\n\n<- `\u{3000}` here"],
        );
    }

    #[test]
    fn test_hover_function_show_qualifiers() {
        check_hover_result(
            "
            //- /lib.rs
            async fn foo<|>() {}
            ",
            &["async fn foo()"],
        );
        check_hover_result(
            "
            //- /lib.rs
            pub const unsafe fn foo<|>() {}
            ",
            &["pub const unsafe fn foo()"],
        );
        check_hover_result(
            r#"
            //- /lib.rs
            pub(crate) async unsafe extern "C" fn foo<|>() {}
            "#,
            &[r#"pub(crate) async unsafe extern "C" fn foo()"#],
        );
    }

    #[test]
    fn test_hover_trait_show_qualifiers() {
        check_hover_result(
            "
            //- /lib.rs
            unsafe trait foo<|>() {}
            ",
            &["unsafe trait foo"],
        );
    }

    #[test]
    fn test_hover_mod_with_same_name_as_function() {
        check_hover_result(
            "
            //- /lib.rs
            use self::m<|>y::Bar;

            mod my {
                pub struct Bar;
            }

            fn my() {}
            ",
            &["mod my"],
        );
    }
}
