//! rust-analyzer extensions to the LSP.

use std::{collections::HashMap, path::PathBuf};

use lsp_types::request::Request;
use lsp_types::{Position, Range, TextDocumentIdentifier};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

pub enum AnalyzerStatus {}

impl Request for AnalyzerStatus {
    type Params = ();
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/analyzerStatus";
}

pub enum CollectGarbage {}

impl Request for CollectGarbage {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "rust-analyzer/collectGarbage";
}

pub enum SyntaxTree {}

impl Request for SyntaxTree {
    type Params = SyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/syntaxTree";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreeParams {
    pub text_document: TextDocumentIdentifier,
    pub range: Option<Range>,
}

pub enum ExpandMacro {}

impl Request for ExpandMacro {
    type Params = ExpandMacroParams;
    type Result = Option<ExpandedMacro>;
    const METHOD: &'static str = "rust-analyzer/expandMacro";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpandMacroParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedMacro {
    pub name: String,
    pub expansion: String,
}

pub enum MatchingBrace {}

impl Request for MatchingBrace {
    type Params = MatchingBraceParams;
    type Result = Vec<Position>;
    const METHOD: &'static str = "experimental/matchingBrace";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchingBraceParams {
    pub text_document: TextDocumentIdentifier,
    pub positions: Vec<Position>,
}

pub enum ParentModule {}

impl Request for ParentModule {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = Option<lsp_types::GotoDefinitionResponse>;
    const METHOD: &'static str = "experimental/parentModule";
}

pub enum JoinLines {}

impl Request for JoinLines {
    type Params = JoinLinesParams;
    type Result = Vec<lsp_types::TextEdit>;
    const METHOD: &'static str = "experimental/joinLines";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinLinesParams {
    pub text_document: TextDocumentIdentifier,
    pub ranges: Vec<Range>,
}

pub enum OnEnter {}

impl Request for OnEnter {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = Option<Vec<SnippetTextEdit>>;
    const METHOD: &'static str = "experimental/onEnter";
}

pub enum Runnables {}

impl Request for Runnables {
    type Params = RunnablesParams;
    type Result = Vec<Runnable>;
    const METHOD: &'static str = "rust-analyzer/runnables";
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunnablesParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Option<Position>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Runnable {
    pub range: Range,
    pub label: String,
    pub bin: String,
    pub args: Vec<String>,
    pub extra_args: Vec<String>,
    pub env: FxHashMap<String, String>,
    pub cwd: Option<PathBuf>,
}

pub enum InlayHints {}

impl Request for InlayHints {
    type Params = InlayHintsParams;
    type Result = Vec<InlayHint>;
    const METHOD: &'static str = "rust-analyzer/inlayHints";
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsParams {
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum InlayKind {
    TypeHint,
    ParameterHint,
    ChainingHint,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InlayHint {
    pub range: Range,
    pub kind: InlayKind,
    pub label: String,
}

pub enum Ssr {}

impl Request for Ssr {
    type Params = SsrParams;
    type Result = lsp_types::WorkspaceEdit;
    const METHOD: &'static str = "experimental/ssr";
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SsrParams {
    pub query: String,
    pub parse_only: bool,
}

pub enum CodeActionRequest {}

impl Request for CodeActionRequest {
    type Params = lsp_types::CodeActionParams;
    type Result = Option<Vec<CodeAction>>;
    const METHOD: &'static str = "textDocument/codeAction";
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct CodeAction {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<lsp_types::Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<SnippetWorkspaceEdit>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetWorkspaceEdit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<HashMap<lsp_types::Url, Vec<lsp_types::TextEdit>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_changes: Option<Vec<SnippetDocumentChangeOperation>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum SnippetDocumentChangeOperation {
    Op(lsp_types::ResourceOp),
    Edit(SnippetTextDocumentEdit),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextDocumentEdit {
    pub text_document: lsp_types::VersionedTextDocumentIdentifier,
    pub edits: Vec<SnippetTextEdit>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextEdit {
    pub range: Range,
    pub new_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<lsp_types::InsertTextFormat>,
}

pub enum HoverRequest {}

impl Request for HoverRequest {
    type Params = lsp_types::HoverParams;
    type Result = Option<Hover>;
    const METHOD: &'static str = "textDocument/hover";
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Hover {
    pub contents: lsp_types::HoverContents,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<CommandLinkGroup>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLinkGroup {
    pub title: Option<String>,
    pub commands: Vec<CommandLink>,
}

// LSP v3.15 Command does not have a `tooltip` field, vscode supports one.
#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLink {
    pub title: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
}
