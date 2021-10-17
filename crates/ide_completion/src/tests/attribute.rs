//! Completion tests for attributes.
use expect_test::{expect, Expect};

use crate::tests::{check_edit, completion_list};

fn check(ra_fixture: &str, expect: Expect) {
    let actual = completion_list(ra_fixture);
    expect.assert_eq(&actual);
}

#[test]
fn doesnt_complete_items() {
    check(
        r#"
struct Foo;
#[$0]
use self as this;
"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
        "#]],
    )
}

#[test]
fn doesnt_complete_qualified() {
    check(
        r#"
struct Foo;
#[foo::$0]
use self as this;
"#,
        expect![[r#""#]],
    )
}

#[test]
fn inside_nested_attr() {
    check(r#"#[cfg($0)]"#, expect![[]])
}

#[test]
fn with_existing_attr() {
    check(
        r#"#[no_mangle] #[$0] mcall!();"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
        "#]],
    )
}

#[test]
fn attr_on_source_file() {
    check(
        r#"#![$0]"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at crate_name = ""
            at feature(…)
            at no_implicit_prelude
            at no_main
            at no_std
            at recursion_limit = "…"
            at type_length_limit = …
            at windows_subsystem = "…"
        "#]],
    );
}

#[test]
fn attr_on_module() {
    check(
        r#"#[$0] mod foo;"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at macro_use
            at path = "…"
        "#]],
    );
    check(
        r#"mod foo {#![$0]}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at no_implicit_prelude
        "#]],
    );
}

#[test]
fn attr_on_macro_rules() {
    check(
        r#"#[$0] macro_rules! foo {}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at macro_export
            at macro_use
        "#]],
    );
}

#[test]
fn attr_on_macro_def() {
    check(
        r#"#[$0] macro foo {}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
        "#]],
    );
}

#[test]
fn attr_on_extern_crate() {
    check(
        r#"#[$0] extern crate foo;"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at macro_use
        "#]],
    );
}

#[test]
fn attr_on_use() {
    check(
        r#"#[$0] use foo;"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
        "#]],
    );
}

#[test]
fn attr_on_type_alias() {
    check(
        r#"#[$0] type foo = ();"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
        "#]],
    );
}

#[test]
fn attr_on_struct() {
    check(
        r#"#[$0] struct Foo;"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at derive(…)
            at repr(…)
            at non_exhaustive
        "#]],
    );
}

#[test]
fn attr_on_enum() {
    check(
        r#"#[$0] enum Foo {}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at derive(…)
            at repr(…)
            at non_exhaustive
        "#]],
    );
}

#[test]
fn attr_on_const() {
    check(
        r#"#[$0] const FOO: () = ();"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
        "#]],
    );
}

#[test]
fn attr_on_static() {
    check(
        r#"#[$0] static FOO: () = ()"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at export_name = "…"
            at link_name = "…"
            at link_section = "…"
            at global_allocator
            at used
        "#]],
    );
}

#[test]
fn attr_on_trait() {
    check(
        r#"#[$0] trait Foo {}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at must_use
        "#]],
    );
}

#[test]
fn attr_on_impl() {
    check(
        r#"#[$0] impl () {}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at automatically_derived
        "#]],
    );
    check(
        r#"impl () {#![$0]}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
        "#]],
    );
}

#[test]
fn attr_on_extern_block() {
    check(
        r#"#[$0] extern {}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at link
        "#]],
    );
    check(
        r#"extern {#![$0]}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at link
        "#]],
    );
}

#[test]
fn attr_on_variant() {
    check(
        r#"enum Foo { #[$0] Bar }"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at non_exhaustive
        "#]],
    );
}

#[test]
fn attr_on_fn() {
    check(
        r#"#[$0] fn main() {}"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
            at deprecated
            at doc = "…"
            at doc(hidden)
            at doc(alias = "…")
            at must_use
            at no_mangle
            at export_name = "…"
            at link_name = "…"
            at link_section = "…"
            at cold
            at ignore = "…"
            at inline
            at must_use
            at panic_handler
            at proc_macro
            at proc_macro_derive(…)
            at proc_macro_attribute
            at should_panic
            at target_feature = "…"
            at test
            at track_caller
        "#]],
    );
}

#[test]
fn attr_on_expr() {
    cov_mark::check!(no_keyword_completion_in_attr_of_expr);
    check(
        r#"fn main() { #[$0] foo() }"#,
        expect![[r#"
            at allow(…)
            at cfg(…)
            at cfg_attr(…)
            at deny(…)
            at forbid(…)
            at warn(…)
        "#]],
    );
}

#[test]
fn attr_in_source_file_end() {
    check(
        r#"#[$0]"#,
        expect![[r#"
            at allow(…)
            at automatically_derived
            at cfg(…)
            at cfg_attr(…)
            at cold
            at deny(…)
            at deprecated
            at derive(…)
            at doc = "…"
            at doc(alias = "…")
            at doc(hidden)
            at export_name = "…"
            at forbid(…)
            at global_allocator
            at ignore = "…"
            at inline
            at link
            at link_name = "…"
            at link_section = "…"
            at macro_export
            at macro_use
            at must_use
            at no_mangle
            at non_exhaustive
            at panic_handler
            at path = "…"
            at proc_macro
            at proc_macro_attribute
            at proc_macro_derive(…)
            at repr(…)
            at should_panic
            at target_feature = "…"
            at test
            at track_caller
            at used
            at warn(…)
        "#]],
    );
}

mod cfg {
    use super::*;

    #[test]
    fn cfg_target_endian() {
        check(
            r#"#[cfg(target_endian = $0"#,
            expect![[r#"
            at little
            at big
"#]],
        );
    }
}

mod derive {
    use super::*;

    fn check_derive(ra_fixture: &str, expect: Expect) {
        let actual = completion_list(ra_fixture);
        expect.assert_eq(&actual);
    }

    #[test]
    fn no_completion_for_incorrect_derive() {
        check_derive(r#"#[derive{$0)] struct Test;"#, expect![[]])
    }

    #[test]
    fn empty_derive() {
        check_derive(
            r#"
//- minicore: derive, copy, clone, ord, eq, default, fmt
#[derive($0)] struct Test;
"#,
            expect![[r#"
                at Default
                at Debug
                at Clone, Copy
                at PartialEq
                at PartialEq, Eq
                at PartialEq, Eq, PartialOrd, Ord
                at Clone
                at PartialEq, PartialOrd
            "#]],
        );
    }

    #[test]
    fn derive_with_input_before() {
        check_derive(
            r#"
//- minicore: derive, copy, clone, ord, eq, default, fmt
#[derive(serde::Serialize, PartialEq, $0)] struct Test;
"#,
            expect![[r#"
                at Default
                at Debug
                at Clone, Copy
                at Eq
                at Eq, PartialOrd, Ord
                at Clone
                at PartialOrd
            "#]],
        )
    }

    #[test]
    fn derive_with_input_after() {
        check_derive(
            r#"
//- minicore: derive, copy, clone, ord, eq, default, fmt
#[derive($0 serde::Serialize, PartialEq)] struct Test;
"#,
            expect![[r#"
                at Default
                at Debug
                at Clone, Copy
                at Eq
                at Eq, PartialOrd, Ord
                at Clone
                at PartialOrd
            "#]],
        )
    }
}

mod lint {
    use super::*;

    #[test]
    fn lint_empty() {
        check_edit(
            "deprecated",
            r#"#[allow($0)] struct Test;"#,
            r#"#[allow(deprecated)] struct Test;"#,
        )
    }

    #[test]
    fn lint_with_existing() {
        check_edit(
            "deprecated",
            r#"#[allow(keyword_idents, $0)] struct Test;"#,
            r#"#[allow(keyword_idents, deprecated)] struct Test;"#,
        )
    }

    #[test]
    fn lint_qualified() {
        check_edit(
            "deprecated",
            r#"#[allow(keyword_idents, $0)] struct Test;"#,
            r#"#[allow(keyword_idents, deprecated)] struct Test;"#,
        )
    }

    #[test]
    fn lint_feature() {
        check_edit(
            "box_syntax",
            r#"#[feature(box_$0)] struct Test;"#,
            r#"#[feature(box_syntax)] struct Test;"#,
        )
    }

    #[test]
    fn lint_clippy_unqualified() {
        check_edit(
            "clippy::as_conversions",
            r#"#[allow($0)] struct Test;"#,
            r#"#[allow(clippy::as_conversions)] struct Test;"#,
        );
    }

    #[test]
    fn lint_clippy_qualified() {
        check_edit(
            "clippy::as_conversions",
            r#"#[allow(clippy::$0)] struct Test;"#,
            r#"#[allow(clippy::as_conversions)] struct Test;"#,
        );
    }
}

mod repr {
    use super::*;

    fn check_repr(ra_fixture: &str, expect: Expect) {
        let actual = completion_list(ra_fixture);
        expect.assert_eq(&actual);
    }

    #[test]
    fn no_completion_for_incorrect_repr() {
        check_repr(r#"#[repr{$0)] struct Test;"#, expect![[]])
    }

    #[test]
    fn empty() {
        check_repr(
            r#"#[repr($0)] struct Test;"#,
            expect![[r#"
            at align($0)
            at packed
            at transparent
            at C
            at u8
            at u16
            at u32
            at u64
            at u128
            at usize
            at i8
            at i16
            at i32
            at i64
            at i28
            at isize
        "#]],
        );
    }

    #[test]
    fn transparent() {
        check_repr(r#"#[repr(transparent, $0)] struct Test;"#, expect![[r#""#]]);
    }

    #[test]
    fn align() {
        check_repr(
            r#"#[repr(align(1), $0)] struct Test;"#,
            expect![[r#"
            at transparent
            at C
            at u8
            at u16
            at u32
            at u64
            at u128
            at usize
            at i8
            at i16
            at i32
            at i64
            at i28
            at isize
        "#]],
        );
    }

    #[test]
    fn packed() {
        check_repr(
            r#"#[repr(packed, $0)] struct Test;"#,
            expect![[r#"
            at transparent
            at C
            at u8
            at u16
            at u32
            at u64
            at u128
            at usize
            at i8
            at i16
            at i32
            at i64
            at i28
            at isize
        "#]],
        );
    }

    #[test]
    fn c() {
        check_repr(
            r#"#[repr(C, $0)] struct Test;"#,
            expect![[r#"
            at align($0)
            at packed
            at u8
            at u16
            at u32
            at u64
            at u128
            at usize
            at i8
            at i16
            at i32
            at i64
            at i28
            at isize
        "#]],
        );
    }

    #[test]
    fn prim() {
        check_repr(
            r#"#[repr(usize, $0)] struct Test;"#,
            expect![[r#"
            at align($0)
            at packed
            at C
        "#]],
        );
    }
}
