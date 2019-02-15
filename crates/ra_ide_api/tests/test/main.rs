use insta::assert_debug_snapshot_matches;
use ra_ide_api::{
    mock_analysis::{single_file, single_file_with_position, MockAnalysis},
    AnalysisChange, CrateGraph, Edition::Edition2018, FileId, Query, NavigationTarget
};
use ra_syntax::{TextRange, SmolStr};

#[test]
fn test_unresolved_module_diagnostic() {
    let (analysis, file_id) = single_file("mod foo;");
    let diagnostics = analysis.diagnostics(file_id).unwrap();
    assert_debug_snapshot_matches!("unresolved_module_diagnostic", &diagnostics);
}

// FIXME: move this test to hir
#[test]
fn test_unresolved_module_diagnostic_no_diag_for_inline_mode() {
    let (analysis, file_id) = single_file("mod foo {}");
    let diagnostics = analysis.diagnostics(file_id).unwrap();
    assert!(diagnostics.is_empty());
}

#[test]
fn test_resolve_crate_root() {
    let mock = MockAnalysis::with_files(
        "
        //- /bar.rs
        mod foo;
        //- /foo.rs
        // empty <|>
    ",
    );
    let root_file = mock.id_of("/bar.rs");
    let mod_file = mock.id_of("/foo.rs");
    let mut host = mock.analysis_host();
    assert!(host.analysis().crate_for(mod_file).unwrap().is_empty());

    let mut crate_graph = CrateGraph::default();
    let crate_id = crate_graph.add_crate_root(root_file, Edition2018);
    let mut change = AnalysisChange::new();
    change.set_crate_graph(crate_graph);
    host.apply_change(change);

    assert_eq!(host.analysis().crate_for(mod_file).unwrap(), vec![crate_id]);
}

fn get_refs(text: &str, include_declaration: bool) -> Vec<(FileId, TextRange)> {
    let (analysis, position) = single_file_with_position(text);
    analysis.find_all_refs(position, include_declaration).unwrap()
}

fn get_all_refs(text: &str) -> Vec<(FileId, TextRange)> {
    get_refs(text, true)
}

fn get_symbols_matching(text: &str, query: &str) -> Vec<NavigationTarget> {
    let (analysis, _) = single_file(text);
    analysis.symbol_search(Query::new(query.into())).unwrap()
}

#[test]
fn test_find_refs_without_declaration_for_local() {
    let code = r#"
    fn main() {
        let mut i = 1;
        let j = 1;
        i = i + j;

        {
            i<|> = 0;
        }

        i = 5;
    }"#;

    let refs = get_refs(code, false);
    assert_eq!(refs.len(), 4);
}

#[test]
fn test_find_refs_without_declaration_for_fn_param() {
    let code = r#"
    fn foo(v<|>al: u32) -> u32 {
        let _ = val;
        let _ = val;
    }
    "#;

    let refs = get_refs(code, false);
    assert_eq!(refs.len(), 2);
}

#[test]
fn test_find_refs_without_declaration_for_fn_local() {
    let code = r#"
    fn foo(val: u32) -> u32 {
        let _ = val<|>;
        let _ = val;
    }
    "#;

    let refs = get_refs(code, false);
    assert_eq!(refs.len(), 2);
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
fn test_world_symbols_with_no_container() {
    let code = r#"
    enum FooInner { }
    "#;

    let mut symbols = get_symbols_matching(code, "FooInner");

    let s = symbols.pop().unwrap();

    assert_eq!(s.name(), "FooInner");
    assert!(s.container_name().is_none());
}

#[test]
fn test_world_symbols_include_container_name() {
    let code = r#"
fn foo() {
    enum FooInner { }
}
    "#;

    let mut symbols = get_symbols_matching(code, "FooInner");

    let s = symbols.pop().unwrap();

    assert_eq!(s.name(), "FooInner");
    assert_eq!(s.container_name(), Some(&SmolStr::new("foo")));

    let code = r#"
mod foo {
    struct FooInner;
}
    "#;

    let mut symbols = get_symbols_matching(code, "FooInner");

    let s = symbols.pop().unwrap();

    assert_eq!(s.name(), "FooInner");
    assert_eq!(s.container_name(), Some(&SmolStr::new("foo")));
}

#[test]
#[ignore]
fn world_symbols_include_stuff_from_macros() {
    let (analysis, _) = single_file(
        "
salsa::query_group! {
pub trait HirDatabase: SyntaxDatabase {}
}
    ",
    );

    let mut symbols = analysis.symbol_search(Query::new("Hir".into())).unwrap();
    let s = symbols.pop().unwrap();
    assert_eq!(s.name(), "HirDatabase");
    assert_eq!(s.full_range(), TextRange::from_to(33.into(), 44.into()));
}
