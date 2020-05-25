mod support;

use std::{collections::HashMap, path::PathBuf, time::Instant};

use lsp_types::{
    notification::DidOpenTextDocument,
    request::{
        CodeActionRequest, Completion, Formatting, GotoDefinition, GotoTypeDefinition, HoverRequest,
    },
    CodeActionContext, CodeActionParams, CompletionParams, DidOpenTextDocumentParams,
    DocumentFormattingParams, FormattingOptions, GotoDefinitionParams, HoverParams,
    PartialResultParams, Position, Range, TextDocumentItem, TextDocumentPositionParams,
    WorkDoneProgressParams,
};
use rust_analyzer::lsp_ext::{OnEnter, Runnables, RunnablesParams};
use serde_json::json;
use tempfile::TempDir;
use test_utils::skip_slow_tests;

use crate::support::{project, Project};

const PROFILE: &str = "";
// const PROFILE: &'static str = "*@3>100";

#[test]
fn completes_items_from_standard_library() {
    if skip_slow_tests() {
        return;
    }

    let project_start = Instant::now();
    let server = Project::with_fixture(
        r#"
//- Cargo.toml
[package]
name = "foo"
version = "0.0.0"

//- src/lib.rs
use std::collections::Spam;
"#,
    )
    .with_sysroot(true)
    .server();
    server.wait_until_workspace_is_loaded();
    eprintln!("loading took    {:?}", project_start.elapsed());
    let completion_start = Instant::now();
    let res = server.send_request::<Completion>(CompletionParams {
        text_document_position: TextDocumentPositionParams::new(
            server.doc_id("src/lib.rs"),
            Position::new(0, 23),
        ),
        context: None,
        partial_result_params: PartialResultParams::default(),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    assert!(format!("{}", res).contains("HashMap"));
    eprintln!("completion took {:?}", completion_start.elapsed());
}

#[test]
fn test_runnables_no_project() {
    if skip_slow_tests() {
        return;
    }

    let server = project(
        r"
//- lib.rs
#[test]
fn foo() {
}
",
    );
    server.wait_until_workspace_is_loaded();
    server.request::<Runnables>(
        RunnablesParams { text_document: server.doc_id("lib.rs"), position: None },
        json!([
          {
            "args": [ "test" ],
            "extraArgs": [ "foo", "--nocapture" ],
            "bin": "cargo",
            "env": { "RUST_BACKTRACE": "short" },
            "cwd": null,
            "label": "test foo",
            "range": {
              "end": { "character": 1, "line": 2 },
              "start": { "character": 0, "line": 0 }
            }
          },
          {
            "args": ["check", "--workspace"],
            "extraArgs": [],
            "bin": "cargo",
            "env": {},
            "cwd": null,
            "label": "cargo check --workspace",
            "range": {
              "end": { "character": 0, "line": 0 },
              "start": { "character": 0, "line": 0 }
            }
          }
        ]),
    );
}

#[test]
fn test_runnables_project() {
    if skip_slow_tests() {
        return;
    }

    let code = r#"
//- foo/Cargo.toml
[package]
name = "foo"
version = "0.0.0"

//- foo/src/lib.rs
pub fn foo() {}

//- foo/tests/spam.rs
#[test]
fn test_eggs() {}

//- bar/Cargo.toml
[package]
name = "bar"
version = "0.0.0"

//- bar/src/main.rs
fn main() {}
"#;

    let server = Project::with_fixture(code).root("foo").root("bar").server();

    server.wait_until_workspace_is_loaded();
    server.request::<Runnables>(
        RunnablesParams { text_document: server.doc_id("foo/tests/spam.rs"), position: None },
        json!([
            {
              "args": [ "test", "--package", "foo", "--test", "spam" ],
              "extraArgs": [ "test_eggs", "--exact", "--nocapture" ],
              "bin": "cargo",
              "env": { "RUST_BACKTRACE": "short" },
              "label": "test test_eggs",
              "range": {
                "end": { "character": 17, "line": 1 },
                "start": { "character": 0, "line": 0 }
              },
              "cwd": server.path().join("foo")
            },
            {
              "args": [ "check", "--package", "foo" ],
              "extraArgs": [],
              "bin": "cargo",
              "env": {},
              "label": "cargo check -p foo",
              "range": {
                "end": { "character": 0, "line": 0 },
                "start": { "character": 0, "line": 0 }
              },
              "cwd": server.path().join("foo")
            },
            {
              "args": [ "test", "--package", "foo" ],
              "extraArgs": [],
              "bin": "cargo",
              "env": {},
              "label": "cargo test -p foo",
              "range": {
                "end": { "character": 0, "line": 0 },
                "start": { "character": 0, "line": 0 }
              },
              "cwd": server.path().join("foo")
            }
        ]),
    );
}

#[test]
fn test_format_document() {
    if skip_slow_tests() {
        return;
    }

    let server = project(
        r#"
//- Cargo.toml
[package]
name = "foo"
version = "0.0.0"

//- src/lib.rs
mod bar;

fn main() {
}

pub use std::collections::HashMap;
"#,
    );
    server.wait_until_workspace_is_loaded();

    server.request::<Formatting>(
        DocumentFormattingParams {
            text_document: server.doc_id("src/lib.rs"),
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: false,
                insert_final_newline: None,
                trim_final_newlines: None,
                trim_trailing_whitespace: None,
                properties: HashMap::new(),
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
        json!([
            {
                "newText": r#"mod bar;

fn main() {}

pub use std::collections::HashMap;
"#,
                "range": {
                    "end": {
                        "character": 0,
                        "line": 7
                    },
                    "start": {
                        "character": 0,
                        "line": 0
                    }
                }
            }
        ]),
    );
}

#[test]
fn test_format_document_2018() {
    if skip_slow_tests() {
        return;
    }

    let server = project(
        r#"
//- Cargo.toml
[package]
name = "foo"
version = "0.0.0"
edition = "2018"

//- src/lib.rs
mod bar;

async fn test() {
}

fn main() {
}

pub use std::collections::HashMap;
"#,
    );
    server.wait_until_workspace_is_loaded();

    server.request::<Formatting>(
        DocumentFormattingParams {
            text_document: server.doc_id("src/lib.rs"),
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: false,
                properties: HashMap::new(),
                insert_final_newline: None,
                trim_final_newlines: None,
                trim_trailing_whitespace: None,
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
        json!([
            {
                "newText": r#"mod bar;

async fn test() {}

fn main() {}

pub use std::collections::HashMap;
"#,
                "range": {
                    "end": {
                        "character": 0,
                        "line": 10
                    },
                    "start": {
                        "character": 0,
                        "line": 0
                    }
                }
            }
        ]),
    );
}

#[test]
fn test_missing_module_code_action() {
    if skip_slow_tests() {
        return;
    }

    let server = project(
        r#"
//- Cargo.toml
[package]
name = "foo"
version = "0.0.0"

//- src/lib.rs
mod bar;

fn main() {}
"#,
    );
    server.wait_until_workspace_is_loaded();
    let empty_context = || CodeActionContext { diagnostics: Vec::new(), only: None };
    server.request::<CodeActionRequest>(
        CodeActionParams {
            text_document: server.doc_id("src/lib.rs"),
            range: Range::new(Position::new(0, 4), Position::new(0, 7)),
            context: empty_context(),
            partial_result_params: PartialResultParams::default(),
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
        json!([{
            "edit": {
              "documentChanges": [
                {
                  "kind": "create",
                  "uri": "file:///[..]/src/bar.rs"
                }
              ]
            },
            "title": "Create module"
        }]),
    );

    server.request::<CodeActionRequest>(
        CodeActionParams {
            text_document: server.doc_id("src/lib.rs"),
            range: Range::new(Position::new(2, 4), Position::new(2, 7)),
            context: empty_context(),
            partial_result_params: PartialResultParams::default(),
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
        json!([]),
    );
}

#[test]
fn test_missing_module_code_action_in_json_project() {
    if skip_slow_tests() {
        return;
    }

    let tmp_dir = TempDir::new().unwrap();

    let path = tmp_dir.path();

    let project = json!({
        "roots": [path],
        "crates": [ {
            "root_module": path.join("src/lib.rs"),
            "deps": [],
            "edition": "2015",
            "atom_cfgs": [],
            "key_value_cfgs": {}
        } ]
    });

    let code = format!(
        r#"
//- rust-project.json
{PROJECT}

//- src/lib.rs
mod bar;

fn main() {{}}
"#,
        PROJECT = project.to_string(),
    );

    let server = Project::with_fixture(&code).tmp_dir(tmp_dir).server();

    server.wait_until_workspace_is_loaded();
    let empty_context = || CodeActionContext { diagnostics: Vec::new(), only: None };
    server.request::<CodeActionRequest>(
        CodeActionParams {
            text_document: server.doc_id("src/lib.rs"),
            range: Range::new(Position::new(0, 4), Position::new(0, 7)),
            context: empty_context(),
            partial_result_params: PartialResultParams::default(),
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
        json!([{
            "edit": {
              "documentChanges": [
                {
                  "kind": "create",
                  "uri": "file://[..]/src/bar.rs"
                }
              ]
            },
            "title": "Create module"
        }]),
    );

    server.request::<CodeActionRequest>(
        CodeActionParams {
            text_document: server.doc_id("src/lib.rs"),
            range: Range::new(Position::new(2, 4), Position::new(2, 7)),
            context: empty_context(),
            partial_result_params: PartialResultParams::default(),
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
        json!([]),
    );
}

#[test]
fn diagnostics_dont_block_typing() {
    if skip_slow_tests() {
        return;
    }

    let librs: String = (0..10).map(|i| format!("mod m{};", i)).collect();
    let libs: String = (0..10).map(|i| format!("//- src/m{}.rs\nfn foo() {{}}\n\n", i)).collect();
    let server = Project::with_fixture(&format!(
        r#"
//- Cargo.toml
[package]
name = "foo"
version = "0.0.0"

//- src/lib.rs
{}

{}

fn main() {{}}
"#,
        librs, libs
    ))
    .with_sysroot(true)
    .server();

    server.wait_until_workspace_is_loaded();
    for i in 0..10 {
        server.notification::<DidOpenTextDocument>(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: server.doc_id(&format!("src/m{}.rs", i)).uri,
                language_id: "rust".to_string(),
                version: 0,
                text: "/// Docs\nfn foo() {}".to_string(),
            },
        });
    }
    let start = std::time::Instant::now();
    server.request::<OnEnter>(
        TextDocumentPositionParams {
            text_document: server.doc_id("src/m0.rs"),
            position: Position { line: 0, character: 5 },
        },
        json!([{
            "insertTextFormat": 2,
            "newText": "\n/// $0",
            "range": {
            "end": { "character": 5, "line": 0 },
            "start": { "character": 5, "line": 0 }
            }
        }]),
    );
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 2000, "typing enter took {:?}", elapsed);
}

#[test]
fn preserves_dos_line_endings() {
    if skip_slow_tests() {
        return;
    }

    let server = Project::with_fixture(
        &"
//- Cargo.toml
[package]
name = \"foo\"
version = \"0.0.0\"

//- src/main.rs
/// Some Docs\r\nfn main() {}
",
    )
    .server();

    server.request::<OnEnter>(
        TextDocumentPositionParams {
            text_document: server.doc_id("src/main.rs"),
            position: Position { line: 0, character: 8 },
        },
        json!([{
            "insertTextFormat": 2,
            "newText": "\r\n/// $0",
            "range": {
            "end": { "line": 0, "character": 8 },
            "start": { "line": 0, "character": 8 }
            }
        }]),
    );
}

#[test]
fn out_dirs_check() {
    if skip_slow_tests() {
        return;
    }

    let server = Project::with_fixture(
        r###"
//- Cargo.toml
[package]
name = "foo"
version = "0.0.0"

//- build.rs
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        r#"pub fn message() -> &'static str { "Hello, World!" }"#,
    )
    .unwrap();
    println!("cargo:rustc-cfg=atom_cfg");
    println!("cargo:rustc-cfg=featlike=\"set\"");
    println!("cargo:rerun-if-changed=build.rs");
}
//- src/main.rs
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

#[cfg(atom_cfg)]
struct A;
#[cfg(bad_atom_cfg)]
struct A;
#[cfg(featlike = "set")]
struct B;
#[cfg(featlike = "not_set")]
struct B;

fn main() {
    let va = A;
    let vb = B;
    message();
}

fn main() { message(); }
"###,
    )
    .with_config(|config| {
        config.cargo.load_out_dirs_from_check = true;
    })
    .server();
    server.wait_until_workspace_is_loaded();
    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("src/main.rs"),
            Position::new(14, 8),
        ),
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    });
    assert!(format!("{}", res).contains("hello.rs"));
    server.request::<GotoTypeDefinition>(
        GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams::new(
                server.doc_id("src/main.rs"),
                Position::new(12, 9),
            ),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
        json!([{
            "originSelectionRange": {
                "end": {
                    "character": 10,
                    "line": 12
                },
                "start": {
                    "character": 8,
                    "line": 12
                }
            },
            "targetRange": {
                "end": {
                    "character": 9,
                    "line": 3
                },
                "start": {
                    "character": 0,
                    "line": 2
                }
            },
            "targetSelectionRange": {
                "end": {
                    "character": 8,
                    "line": 3
                },
                "start": {
                    "character": 7,
                    "line": 3
                }
            },
            "targetUri": "file:///[..]src/main.rs"
        }]),
    );
    server.request::<GotoTypeDefinition>(
        GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams::new(
                server.doc_id("src/main.rs"),
                Position::new(13, 9),
            ),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
        json!([{
            "originSelectionRange": {
                "end": {
                    "character": 10,
                    "line": 13
                },
                "start": {
                    "character": 8,
                    "line":13
                }
            },
            "targetRange": {
                "end": {
                    "character": 9,
                    "line": 7
                },
                "start": {
                    "character": 0,
                    "line":6
                }
            },
            "targetSelectionRange": {
                "end": {
                    "character": 8,
                    "line": 7
                },
                "start": {
                    "character": 7,
                    "line": 7
                }
            },
            "targetUri": "file:///[..]src/main.rs"
        }]),
    );
}

#[test]
fn resolve_proc_macro() {
    if skip_slow_tests() {
        return;
    }
    let server = Project::with_fixture(
        r###"
//- foo/Cargo.toml
[package]
name = "foo"
version = "0.0.0"
edition = "2018"
[dependencies]
bar = {path = "../bar"}

//- foo/src/main.rs
use bar::Bar;
trait Bar {
  fn bar();
}
#[derive(Bar)]
struct Foo {}
fn main() {
  Foo::bar();
}

//- bar/Cargo.toml
[package]
name = "bar"
version = "0.0.0"
edition = "2018"

[lib]
proc-macro = true

//- bar/src/lib.rs
extern crate proc_macro;
use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
macro_rules! t {
    ($n:literal) => {
        TokenTree::from(Ident::new($n, Span::call_site()))
    };
    ({}) => {
        TokenTree::from(Group::new(Delimiter::Brace, TokenStream::new()))
    };
    (()) => {
        TokenTree::from(Group::new(Delimiter::Parenthesis, TokenStream::new()))
    };
}
#[proc_macro_derive(Bar)]
pub fn foo(_input: TokenStream) -> TokenStream {
    // We hard code the output here for preventing to use any deps
    let mut res = TokenStream::new();

    // impl Bar for Foo { fn bar() {} }
    let mut tokens = vec![t!("impl"), t!("Bar"), t!("for"), t!("Foo")];
    let mut fn_stream = TokenStream::new();
    fn_stream.extend(vec![t!("fn"), t!("bar"), t!(()), t!({})]);
    tokens.push(Group::new(Delimiter::Brace, fn_stream).into());
    res.extend(tokens);
    res
}

"###,
    )
    .with_config(|config| {
        let macro_srv_path = PathBuf::from(env!("CARGO_BIN_EXE_rust-analyzer"));

        config.cargo.load_out_dirs_from_check = true;
        config.proc_macro_srv = Some((macro_srv_path, vec!["proc-macro".into()]));
    })
    .root("foo")
    .root("bar")
    .server();
    server.wait_until_workspace_is_loaded();
    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("foo/src/main.rs"),
            Position::new(7, 9),
        ),
        work_done_progress_params: Default::default(),
    });

    let contents = res.get("contents").expect("missing key: contents");
    assert_eq!(true, contents.is_array(), "expected multiple marked strings");

    let element = contents.get(0).expect("missing content element: 0");
    assert_eq!(Some(Some("fn bar()")), element.get("value").map(|v| v.as_str()));

    let element = contents.get(1).expect("missing content element: 1");
    assert_eq!(Some(Some("foo::Bar")), element.get("value").map(|v| v.as_str()));
}
