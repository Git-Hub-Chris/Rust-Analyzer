use std::sync::Arc;
use std::fmt::Write;

use ra_db::{SyntaxDatabase, salsa::Database};
use ra_syntax::ast::{self, AstNode};

use crate::{
    source_binder,
    mock::MockDatabase,
};

// These tests compare the inference results for all expressions in a file
// against snapshots of the expected results using insta. Run the tests with
// INSTA_UPDATE=1 to update the snapshots.

#[test]
fn infer_basics() {
    check_inference(
        "infer_basics",
        r#"
fn test(a: u32, b: isize, c: !, d: &str) {
    a;
    b;
    c;
    d;
    1usize;
    1isize;
    "test";
    1.0f32;
}"#,
    );
}

#[test]
fn infer_let() {
    check_inference(
        "infer_let",
        r#"
fn test() {
    let a = 1isize;
    let b: usize = 1;
    let c = b;
}
}"#,
    );
}

#[test]
fn infer_paths() {
    check_inference(
        "infer_paths",
        r#"
fn a() -> u32 { 1 }

mod b {
    fn c() -> u32 { 1 }
}

fn test() {
    a();
    b::c();
}
}"#,
    );
}

#[test]
fn infer_struct() {
    check_inference(
        "infer_struct",
        r#"
struct A {
    b: B,
    c: C,
}
struct B;
struct C(usize);

fn test() {
    let c = C(1);
    B;
    let a: A = A { b: B, c: C(1) };
    a.b;
    a.c;
}
"#,
    );
}

#[test]
fn infer_enum() {
    check_inference(
        "infer_enum",
        r#"
enum E {
  V1 { field: u32 },
  V2
}
fn test() {
  E::V1 { field: 1 };
  E::V2;
}"#,
    );
}

#[test]
fn infer_refs() {
    check_inference(
        "infer_refs",
        r#"
fn test(a: &u32, b: &mut u32, c: *const u32, d: *mut u32) {
    a;
    *a;
    &a;
    &mut a;
    b;
    *b;
    &b;
    c;
    *c;
    d;
    *d;
}
"#,
    );
}

#[test]
fn infer_literals() {
    check_inference(
        "infer_literals",
        r##"
fn test() {
    5i32;
    "hello";
    b"bytes";
    'c';
    b'b';
    3.14;
    5000;
    false;
    true;
    r#"
        //! doc
        // non-doc
        mod foo {}
        "#;
    br#"yolo"#;
}
"##,
    );
}

#[test]
fn infer_unary_op() {
    check_inference(
        "infer_unary_op",
        r#"
enum SomeType {}

fn test(x: SomeType) {
    let b = false;
    let c = !b;
    let a = 100;
    let d: i128 = -a;
    let e = -100;
    let f = !!!true;
    -3.14;
    -x;
    !x;
    -"hello";
}
"#,
    );
}

#[test]
fn infer_backwards() {
    check_inference(
        "infer_backwards",
        r#"
fn takes_u32(x: u32) {}

struct S { i32_field: i32 }

fn test() -> &mut &f64 {
    let a = unknown_function();
    takes_u32(a);
    let b = unknown_function();
    S { i32_field: b };
    let c = unknown_function();
    &mut &c
}
"#,
    );
}

#[test]
fn infer_self() {
    check_inference(
        "infer_self",
        r#"
struct S;

impl S {
    fn test(&self) {
        self;
    }
    fn test2(self: &Self) {
        self;
    }
}
"#,
    );
}

#[test]
fn infer_binary_op() {
    check_inference(
        "infer_binary_op",
        r#"
fn f(x: bool) -> i32 {
    0i32
}

fn test() -> bool {
    let x = a && b;
    let y = true || false;
    let z = x == y;
    let minus_forty: isize = -40isize;
    let h = minus_forty <= CONST_2;
    let c = f(z || y) + 5;
    let d = b;
    let g = minus_forty ^= i;
    let ten: usize = 10;
    let ten_is_eleven = ten == some_num;

    ten < 3
}
"#,
    );
}

#[test]
fn infer_field_autoderef() {
    check_inference(
        "infer_field_autoderef",
        r#"
struct A {
    b: B,
}
struct B;

fn test1(a: A) {
    let a1 = a;
    a1.b;
    let a2 = &a;
    a2.b;
    let a3 = &mut a;
    a3.b;
    let a4 = &&&&&&&a;
    a4.b;
    let a5 = &mut &&mut &&mut a;
    a5.b;
}

fn test2(a1: *const A, a2: *mut A) {
    a1.b;
    a2.b;
}
"#,
    );
}

#[test]
fn bug_484() {
    check_inference(
        "bug_484",
        r#"
fn test() {
   let x = if true {};
}
"#,
    );
}

#[test]
fn infer_inherent_method() {
    check_inference(
        "infer_inherent_method",
        r#"
struct A;

impl A {
    fn foo(self, x: u32) -> i32 {}
}

mod b {
    impl super::A {
        fn bar(&self, x: u64) -> i64 {}
    }
}

fn test(a: A) {
    a.foo(1);
    (&a).bar(1);
    a.bar(1);
}
"#,
    );
}

#[test]
fn infer_tuple() {
    check_inference(
        "infer_tuple",
        r#"
fn test(x: &str, y: isize) {
    let a: (u32, &str) = (1, "a");
    let b = (a, x);
    let c = (y, x);
    let d = (c, x);
    let e = (1, "e");
    let f = (e, "d");
}
"#,
    );
}

#[test]
fn infer_array() {
    check_inference(
        "infer_array",
        r#"
fn test(x: &str, y: isize) {
    let a = [x];
    let b = [a, a];
    let c = [b, b];

    let d = [y, 1, 2, 3];
    let d = [1, y, 2, 3];
    let e = [y];
    let f = [d, d];
    let g = [e, e];

    let h = [1, 2];
    let i = ["a", "b"];

    let b = [a, ["b"]];
    let x: [u8; 0] = [];
}
"#,
    );
}

#[test]
fn infer_pattern() {
    check_inference(
        "infer_pattern",
        r#"
fn test(x: &i32) {
    let y = x;
    let &z = x;
    let a = z;
    let (c, d) = (1, "hello");

    for (e, f) in some_iter {
        let g = e;
    }

    if let [val] = opt {
        let h = val;
    }

    let lambda = |a: u64, b, c: i32| { a + b; c };

    let ref ref_to_x = x;
    let mut mut_x = x;
    let ref mut mut_ref_to_x = x;
    let k = mut_ref_to_x;
}
"#,
    );
}

#[test]
fn infer_adt_pattern() {
    check_inference(
        "infer_adt_pattern",
        r#"
enum E {
    A { x: usize },
    B
}

struct S(u32, E);

fn test() {
    let e = E::A { x: 3 };

    let S(y, z) = foo;
    let E::A { x: new_var } = e;

    match e {
        E::A { x } => x,
        E::B => 1,
    };

    let ref d @ E::A { .. } = e;
    d;
}
"#,
    );
}

#[test]
fn infer_struct_generics() {
    check_inference(
        "infer_struct_generics",
        r#"
struct A<T> {
    x: T,
}

fn test(a1: A<u32>, i: i32) {
    a1.x;
    let a2 = A { x: i };
    a2.x;
    let a3 = A::<i128> { x: 1 };
    a3.x;
}
"#,
    );
}

#[test]
fn infer_generics_in_patterns() {
    check_inference(
        "infer_generics_in_patterns",
        r#"
struct A<T> {
    x: T,
}

enum Option<T> {
    Some(T),
    None,
}

fn test(a1: A<u32>, o: Option<u64>) {
    let A { x: x2 } = a1;
    let A::<i64> { x: x3 } = A { x: 1 };
    match o {
        Option::Some(t) => t,
        _ => 1,
    };
}
"#,
    );
}

#[test]
fn infer_function_generics() {
    check_inference(
        "infer_function_generics",
        r#"
fn id<T>(t: T) -> T { t }

fn test() {
    id(1u32);
    id::<i128>(1);
    let x: u64 = id(1);
}
"#,
    );
}

#[test]
fn infer_generic_chain() {
    check_inference(
        "infer_generic_chain",
        r#"
struct A<T> {
    x: T,
}
impl<T2> A<T2> {
    fn x(self) -> T2 {
        self.x
    }
}
fn id<T>(t: T) -> T { t }

fn test() -> i128 {
     let x = 1;
     let y = id(x);
     let a = A { x: id(y) };
     let z = id(a.x);
     let b = A { x: z };
     b.x()
}
"#,
    );
}

#[test]
fn no_panic_on_field_of_enum() {
    check_inference(
        "no_panic_on_field_of_enum",
        r#"
enum X {}

fn test(x: X) {
    x.some_field;
}
"#,
    );
}

#[test]
fn bug_585() {
    check_inference(
        "bug_585",
        r#"
fn test() {
    X {};
    match x {
        A::B {} => (),
        A::Y() => (),
    }
}
"#,
    );
}

fn infer(content: &str) -> String {
    let (db, _, file_id) = MockDatabase::with_single_file(content);
    let source_file = db.source_file(file_id);
    let mut acc = String::new();
    for fn_def in source_file.syntax().descendants().filter_map(ast::FnDef::cast) {
        let func = source_binder::function_from_source(&db, file_id, fn_def).unwrap();
        let inference_result = func.infer(&db);
        let body_syntax_mapping = func.body_syntax_mapping(&db);
        let mut types = Vec::new();
        for (pat, ty) in inference_result.type_of_pat.iter() {
            let syntax_ptr = match body_syntax_mapping.pat_syntax(pat) {
                Some(sp) => sp,
                None => continue,
            };
            types.push((syntax_ptr, ty));
        }
        for (expr, ty) in inference_result.type_of_expr.iter() {
            let syntax_ptr = match body_syntax_mapping.expr_syntax(expr) {
                Some(sp) => sp,
                None => continue,
            };
            types.push((syntax_ptr, ty));
        }
        // sort ranges for consistency
        types.sort_by_key(|(ptr, _)| (ptr.range().start(), ptr.range().end()));
        for (syntax_ptr, ty) in &types {
            let node = syntax_ptr.to_node(&source_file);
            write!(
                acc,
                "{} '{}': {}\n",
                syntax_ptr.range(),
                ellipsize(node.text().to_string().replace("\n", " "), 15),
                ty
            )
            .unwrap();
        }
    }
    acc
}

fn check_inference(name: &str, content: &str) {
    let result = infer(content);

    insta::assert_snapshot_matches!(&name, &result);
}

fn ellipsize(mut text: String, max_len: usize) -> String {
    if text.len() <= max_len {
        return text;
    }
    let ellipsis = "...";
    let e_len = ellipsis.len();
    let mut prefix_len = (max_len - e_len) / 2;
    while !text.is_char_boundary(prefix_len) {
        prefix_len += 1;
    }
    let mut suffix_len = max_len - e_len - prefix_len;
    while !text.is_char_boundary(text.len() - suffix_len) {
        suffix_len += 1;
    }
    text.replace_range(prefix_len..text.len() - suffix_len, ellipsis);
    text
}

#[test]
fn typing_whitespace_inside_a_function_should_not_invalidate_types() {
    let (mut db, pos) = MockDatabase::with_position(
        "
        //- /lib.rs
        fn foo() -> i32 {
            <|>1 + 1
        }
    ",
    );
    let func = source_binder::function_from_position(&db, pos).unwrap();
    {
        let events = db.log_executed(|| {
            func.infer(&db);
        });
        assert!(format!("{:?}", events).contains("infer"))
    }

    let new_text = "
        fn foo() -> i32 {
            1
            +
            1
        }
    "
    .to_string();

    db.query_mut(ra_db::FileTextQuery).set(pos.file_id, Arc::new(new_text));

    {
        let events = db.log_executed(|| {
            func.infer(&db);
        });
        assert!(!format!("{:?}", events).contains("infer"), "{:#?}", events)
    }
}
