use ra_syntax::TextRange;
use test_utils::assert_eq_dbg;

use ra_analysis::{
    mock_analysis::{analysis_and_position, single_file, single_file_with_position, MockAnalysis},
    AnalysisChange, CrateGraph, FileId, FnSignatureInfo,
};

fn get_signature(text: &str) -> (FnSignatureInfo, Option<usize>) {
    let (analysis, position) = single_file_with_position(text);
    analysis.resolve_callable(position).unwrap().unwrap()
}

#[test]
fn approximate_resolve_works_in_items() {
    let (analysis, pos) = analysis_and_position(
        "
        //- /lib.rs
        struct Foo;
        enum E { X(Foo<|>) }
    ",
    );

    let symbols = analysis.approximately_resolve_symbol(pos).unwrap().unwrap();
    assert_eq_dbg(
        r#"([23; 26), [(FileId(1), FileSymbol { name: "Foo", node_range: [0; 11), kind: STRUCT_DEF })])"#,
        &symbols,
    );
}

#[test]
fn test_resolve_module() {
    let (analysis, pos) = analysis_and_position(
        "
        //- /lib.rs
        mod <|>foo;
        //- /foo.rs
        // empty
    ",
    );

    let symbols = analysis.approximately_resolve_symbol(pos).unwrap().unwrap();
    assert_eq_dbg(
        r#"([4; 7), [(FileId(2), FileSymbol { name: "foo", node_range: [0; 0), kind: MODULE })])"#,
        &symbols,
    );

    let (analysis, pos) = analysis_and_position(
        "
        //- /lib.rs
        mod <|>foo;
        //- /foo/mod.rs
        // empty
    ",
    );

    let symbols = analysis.approximately_resolve_symbol(pos).unwrap().unwrap();
    assert_eq_dbg(
        r#"([4; 7), [(FileId(2), FileSymbol { name: "foo", node_range: [0; 0), kind: MODULE })])"#,
        &symbols,
    );
}

#[test]
fn test_unresolved_module_diagnostic() {
    let (analysis, file_id) = single_file("mod foo;");
    let diagnostics = analysis.diagnostics(file_id).unwrap();
    assert_eq_dbg(
        r#"[Diagnostic {
            message: "unresolved module",
            range: [4; 7),
            fix: Some(SourceChange {
                label: "create module",
                source_file_edits: [],
                file_system_edits: [CreateFile { anchor: FileId(1), path: "../foo.rs" }],
                cursor_position: None }) }]"#,
        &diagnostics,
    );
}

#[test]
fn test_unresolved_module_diagnostic_no_diag_for_inline_mode() {
    let (analysis, file_id) = single_file("mod foo {}");
    let diagnostics = analysis.diagnostics(file_id).unwrap();
    assert_eq_dbg(r#"[]"#, &diagnostics);
}

#[test]
fn test_resolve_parent_module() {
    let (analysis, pos) = analysis_and_position(
        "
        //- /lib.rs
        mod foo;
        //- /foo.rs
        <|>// empty
    ",
    );
    let symbols = analysis.parent_module(pos).unwrap();
    assert_eq_dbg(
        r#"[(FileId(1), FileSymbol { name: "foo", node_range: [4; 7), kind: MODULE })]"#,
        &symbols,
    );
}

#[test]
fn test_resolve_parent_module_for_inline() {
    let (analysis, pos) = analysis_and_position(
        "
        //- /lib.rs
        mod foo {
            mod bar {
                mod baz { <|> }
            }
        }
    ",
    );
    let symbols = analysis.parent_module(pos).unwrap();
    assert_eq_dbg(
        r#"[(FileId(1), FileSymbol { name: "bar", node_range: [18; 21), kind: MODULE })]"#,
        &symbols,
    );
}

#[test]
fn test_resolve_crate_root() {
    let mock = MockAnalysis::with_files(
        "
        //- /lib.rs
        mod foo;
        //- /foo.rs
        // emtpy <|>
    ",
    );
    let root_file = mock.id_of("/lib.rs");
    let mod_file = mock.id_of("/foo.rs");
    let mut host = mock.analysis_host();
    assert!(host.analysis().crate_for(mod_file).unwrap().is_empty());

    let mut crate_graph = CrateGraph::default();
    let crate_id = crate_graph.add_crate_root(root_file);
    let mut change = AnalysisChange::new();
    change.set_crate_graph(crate_graph);
    host.apply_change(change);

    assert_eq!(host.analysis().crate_for(mod_file).unwrap(), vec![crate_id]);
}

#[test]
fn test_fn_signature_two_args_first() {
    let (desc, param) = get_signature(
        r#"fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo(<|>3, ); }"#,
    );

    assert_eq!(desc.name, "foo".to_string());
    assert_eq!(desc.params, vec!("x".to_string(), "y".to_string()));
    assert_eq!(desc.ret_type, Some("-> u32".into()));
    assert_eq!(param, Some(0));
}

#[test]
fn test_fn_signature_two_args_second() {
    let (desc, param) = get_signature(
        r#"fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo(3, <|>); }"#,
    );

    assert_eq!(desc.name, "foo".to_string());
    assert_eq!(desc.params, vec!("x".to_string(), "y".to_string()));
    assert_eq!(desc.ret_type, Some("-> u32".into()));
    assert_eq!(param, Some(1));
}

#[test]
fn test_fn_signature_for_impl() {
    let (desc, param) = get_signature(
        r#"struct F; impl F { pub fn new() { F{}} }
fn bar() {let _ : F = F::new(<|>);}"#,
    );

    assert_eq!(desc.name, "new".to_string());
    assert_eq!(desc.params, Vec::<String>::new());
    assert_eq!(desc.ret_type, None);
    assert_eq!(param, None);
}

#[test]
fn test_fn_signature_for_method_self() {
    let (desc, param) = get_signature(
        r#"struct F;
impl F {
    pub fn new() -> F{
        F{}
    }

    pub fn do_it(&self) {}
}

fn bar() {
    let f : F = F::new();
    f.do_it(<|>);
}"#,
    );

    assert_eq!(desc.name, "do_it".to_string());
    assert_eq!(desc.params, vec!["&self".to_string()]);
    assert_eq!(desc.ret_type, None);
    assert_eq!(param, None);
}

#[test]
fn test_fn_signature_for_method_with_arg() {
    let (desc, param) = get_signature(
        r#"struct F;
impl F {
    pub fn new() -> F{
        F{}
    }

    pub fn do_it(&self, x: i32) {}
}

fn bar() {
    let f : F = F::new();
    f.do_it(<|>);
}"#,
    );

    assert_eq!(desc.name, "do_it".to_string());
    assert_eq!(desc.params, vec!["&self".to_string(), "x".to_string()]);
    assert_eq!(desc.ret_type, None);
    assert_eq!(param, Some(1));
}

#[test]
fn test_fn_signature_with_docs_simple() {
    let (desc, param) = get_signature(
        r#"
// test
fn foo(j: u32) -> u32 {
    j
}

fn bar() {
    let _ = foo(<|>);
}
"#,
    );

    assert_eq!(desc.name, "foo".to_string());
    assert_eq!(desc.params, vec!["j".to_string()]);
    assert_eq!(desc.ret_type, Some("-> u32".to_string()));
    assert_eq!(param, Some(0));
    assert_eq!(desc.label, "fn foo(j: u32) -> u32".to_string());
    assert_eq!(desc.doc, Some("test".into()));
}

#[test]
fn test_fn_signature_with_docs() {
    let (desc, param) = get_signature(
        r#"
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn do() {
    add_one(<|>
}"#,
    );

    assert_eq!(desc.name, "add_one".to_string());
    assert_eq!(desc.params, vec!["x".to_string()]);
    assert_eq!(desc.ret_type, Some("-> i32".to_string()));
    assert_eq!(param, Some(0));
    assert_eq!(desc.label, "pub fn add_one(x: i32) -> i32".to_string());
    assert_eq!(
        desc.doc,
        Some(
            r#"Adds one to the number given.

# Examples

```rust
let five = 5;

assert_eq!(6, my_crate::add_one(5));
```"#
                .into()
        )
    );
}

#[test]
fn test_fn_signature_with_docs_impl() {
    let (desc, param) = get_signature(
        r#"
struct addr;
impl addr {
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, my_crate::add_one(5));
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
}

pub fn do_it() {
    addr {};
    addr::add_one(<|>);
}"#,
    );

    assert_eq!(desc.name, "add_one".to_string());
    assert_eq!(desc.params, vec!["x".to_string()]);
    assert_eq!(desc.ret_type, Some("-> i32".to_string()));
    assert_eq!(param, Some(0));
    assert_eq!(desc.label, "pub fn add_one(x: i32) -> i32".to_string());
    assert_eq!(
        desc.doc,
        Some(
            r#"Adds one to the number given.

# Examples

```rust
let five = 5;

assert_eq!(6, my_crate::add_one(5));
```"#
                .into()
        )
    );
}

#[test]
fn test_fn_signature_with_docs_from_actix() {
    let (desc, param) = get_signature(
        r#"
pub trait WriteHandler<E>
where
    Self: Actor,
    Self::Context: ActorContext,
{
    /// Method is called when writer emits error.
    ///
    /// If this method returns `ErrorAction::Continue` writer processing
    /// continues otherwise stream processing stops.
    fn error(&mut self, err: E, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    /// Method is called when writer finishes.
    ///
    /// By default this method stops actor's `Context`.
    fn finished(&mut self, ctx: &mut Self::Context) {
        ctx.stop()
    }
}

pub fn foo() {
    WriteHandler r;
    r.finished(<|>);
}

"#,
    );

    assert_eq!(desc.name, "finished".to_string());
    assert_eq!(
        desc.params,
        vec!["&mut self".to_string(), "ctx".to_string()]
    );
    assert_eq!(desc.ret_type, None);
    assert_eq!(param, Some(1));
    assert_eq!(
        desc.doc,
        Some(
            r#"Method is called when writer finishes.

By default this method stops actor's `Context`."#
                .into()
        )
    );
}

fn get_all_refs(text: &str) -> Vec<(FileId, TextRange)> {
    let (analysis, position) = single_file_with_position(text);
    analysis.find_all_refs(position).unwrap()
}

#[test]
fn test_find_all_refs_for_local() {
    let code = r#"
    fn main() {
        let mut i = 1;
        let j = 1;
        i = i<|> + j;

        {
            i = 0;
        }

        i = 5;
    }"#;

    let refs = get_all_refs(code);
    assert_eq!(refs.len(), 5);
}

#[test]
fn test_find_all_refs_for_param_inside() {
    let code = r#"
    fn foo(i : u32) -> u32 {
        i<|>
    }"#;

    let refs = get_all_refs(code);
    assert_eq!(refs.len(), 2);
}

#[test]
fn test_find_all_refs_for_fn_param() {
    let code = r#"
    fn foo(i<|> : u32) -> u32 {
        i
    }"#;

    let refs = get_all_refs(code);
    assert_eq!(refs.len(), 2);
}

#[test]
fn test_complete_crate_path() {
    let (analysis, position) = analysis_and_position(
        "
        //- /lib.rs
        mod foo;
        struct Spam;
        //- /foo.rs
        use crate::Sp<|>
    ",
    );
    let completions = analysis.completions(position).unwrap().unwrap();
    assert_eq_dbg(
        r#"[CompletionItem { label: "Spam", lookup: None, snippet: None },
            CompletionItem { label: "foo", lookup: None, snippet: None }]"#,
        &completions,
    );
}

#[test]
fn test_complete_crate_path_with_braces() {
    let (analysis, position) = analysis_and_position(
        "
        //- /lib.rs
        mod foo;
        struct Spam;
        //- /foo.rs
        use crate::{Sp<|>};
    ",
    );
    let completions = analysis.completions(position).unwrap().unwrap();
    assert_eq_dbg(
        r#"[CompletionItem { label: "Spam", lookup: None, snippet: None },
            CompletionItem { label: "foo", lookup: None, snippet: None }]"#,
        &completions,
    );
}

#[test]
fn test_complete_crate_path_in_nested_tree() {
    let (analysis, position) = analysis_and_position(
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
    ",
    );
    let completions = analysis.completions(position).unwrap().unwrap();
    assert_eq_dbg(
        r#"[CompletionItem { label: "Spam", lookup: None, snippet: None }]"#,
        &completions,
    );
}
