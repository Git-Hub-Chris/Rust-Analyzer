use ra_parser::{TreeSink, ParseError};
use ra_syntax::{
    AstNode, SyntaxNode, TextRange, SyntaxKind, SmolStr, SyntaxTreeBuilder, TreeArc, SyntaxElement,
    ast, SyntaxKind::*, TextUnit
};

use crate::subtree_source::{SubtreeTokenSource, Querier};
use crate::ExpandError;

/// Maps `tt::TokenId` to the relative range of the original token.
#[derive(Default)]
pub struct TokenMap {
    /// Maps `tt::TokenId` to the *relative* source range.
    tokens: Vec<TextRange>,
}

/// Convert the syntax tree (what user has written) to a `TokenTree` (what macro
/// will consume).
pub fn ast_to_token_tree(ast: &ast::TokenTree) -> Option<(tt::Subtree, TokenMap)> {
    let mut token_map = TokenMap::default();
    let node = ast.syntax();
    let tt = convert_tt(&mut token_map, node.range().start(), node)?;
    Some((tt, token_map))
}

/// Convert the syntax node to a `TokenTree` (what macro
/// will consume).
pub fn syntax_node_to_token_tree(node: &SyntaxNode) -> Option<(tt::Subtree, TokenMap)> {
    let mut token_map = TokenMap::default();
    let tt = convert_tt(&mut token_map, node.range().start(), node)?;
    Some((tt, token_map))
}

// The following items are what `rustc` macro can be parsed into :
// link: https://github.com/rust-lang/rust/blob/9ebf47851a357faa4cd97f4b1dc7835f6376e639/src/libsyntax/ext/expand.rs#L141
// * Expr(P<ast::Expr>)                     -> token_tree_to_expr
// * Pat(P<ast::Pat>)                       -> token_tree_to_pat
// * Ty(P<ast::Ty>)                         -> token_tree_to_ty
// * Stmts(SmallVec<[ast::Stmt; 1]>)        -> token_tree_to_stmts
// * Items(SmallVec<[P<ast::Item>; 1]>)     -> token_tree_to_items
//
// * TraitItems(SmallVec<[ast::TraitItem; 1]>)
// * ImplItems(SmallVec<[ast::ImplItem; 1]>)
// * ForeignItems(SmallVec<[ast::ForeignItem; 1]>
//
//

/// Parses the token tree (result of macro expansion) to an expression
pub fn token_tree_to_expr(tt: &tt::Subtree) -> Result<TreeArc<ast::Expr>, ExpandError> {
    let token_source = SubtreeTokenSource::new(tt);
    let mut tree_sink = TtTreeSink::new(token_source.querier());
    ra_parser::parse_expr(&token_source, &mut tree_sink);
    let syntax = tree_sink.inner.finish();
    ast::Expr::cast(&syntax)
        .map(|m| m.to_owned())
        .ok_or_else(|| crate::ExpandError::ConversionError)
}

/// Parses the token tree (result of macro expansion) to a Pattern
pub fn token_tree_to_pat(tt: &tt::Subtree) -> Result<TreeArc<ast::Pat>, ExpandError> {
    let token_source = SubtreeTokenSource::new(tt);
    let mut tree_sink = TtTreeSink::new(token_source.querier());
    ra_parser::parse_pat(&token_source, &mut tree_sink);
    let syntax = tree_sink.inner.finish();
    ast::Pat::cast(&syntax).map(|m| m.to_owned()).ok_or_else(|| ExpandError::ConversionError)
}

/// Parses the token tree (result of macro expansion) to a Type
pub fn token_tree_to_ty(tt: &tt::Subtree) -> Result<TreeArc<ast::TypeRef>, ExpandError> {
    let token_source = SubtreeTokenSource::new(tt);
    let mut tree_sink = TtTreeSink::new(token_source.querier());
    ra_parser::parse_ty(&token_source, &mut tree_sink);
    let syntax = tree_sink.inner.finish();
    ast::TypeRef::cast(&syntax).map(|m| m.to_owned()).ok_or_else(|| ExpandError::ConversionError)
}

/// Parses the token tree (result of macro expansion) as a sequence of stmts
pub fn token_tree_to_macro_stmts(
    tt: &tt::Subtree,
) -> Result<TreeArc<ast::MacroStmts>, ExpandError> {
    let token_source = SubtreeTokenSource::new(tt);
    let mut tree_sink = TtTreeSink::new(token_source.querier());
    ra_parser::parse_macro_stmts(&token_source, &mut tree_sink);
    let syntax = tree_sink.inner.finish();
    ast::MacroStmts::cast(&syntax).map(|m| m.to_owned()).ok_or_else(|| ExpandError::ConversionError)
}

/// Parses the token tree (result of macro expansion) as a sequence of items
pub fn token_tree_to_macro_items(
    tt: &tt::Subtree,
) -> Result<TreeArc<ast::MacroItems>, ExpandError> {
    let token_source = SubtreeTokenSource::new(tt);
    let mut tree_sink = TtTreeSink::new(token_source.querier());
    ra_parser::parse_macro_items(&token_source, &mut tree_sink);
    let syntax = tree_sink.inner.finish();
    ast::MacroItems::cast(&syntax).map(|m| m.to_owned()).ok_or_else(|| ExpandError::ConversionError)
}

/// Parses the token tree (result of macro expansion) as a sequence of items
pub fn token_tree_to_ast_item_list(tt: &tt::Subtree) -> TreeArc<ast::SourceFile> {
    let token_source = SubtreeTokenSource::new(tt);
    let mut tree_sink = TtTreeSink::new(token_source.querier());
    ra_parser::parse(&token_source, &mut tree_sink);
    let syntax = tree_sink.inner.finish();
    ast::SourceFile::cast(&syntax).unwrap().to_owned()
}

impl TokenMap {
    pub fn relative_range_of(&self, tt: tt::TokenId) -> Option<TextRange> {
        let idx = tt.0 as usize;
        self.tokens.get(idx).map(|&it| it)
    }

    fn alloc(&mut self, relative_range: TextRange) -> tt::TokenId {
        let id = self.tokens.len();
        self.tokens.push(relative_range);
        tt::TokenId(id as u32)
    }
}

fn convert_tt(
    token_map: &mut TokenMap,
    global_offset: TextUnit,
    tt: &SyntaxNode,
) -> Option<tt::Subtree> {
    // This tree is empty
    if tt.first_child_or_token().is_none() {
        return Some(tt::Subtree { token_trees: vec![], delimiter: tt::Delimiter::None });
    }

    let first_child = tt.first_child_or_token()?;
    let last_child = tt.last_child_or_token()?;
    let (delimiter, skip_first) = match (first_child.kind(), last_child.kind()) {
        (L_PAREN, R_PAREN) => (tt::Delimiter::Parenthesis, true),
        (L_CURLY, R_CURLY) => (tt::Delimiter::Brace, true),
        (L_BRACK, R_BRACK) => (tt::Delimiter::Bracket, true),
        _ => (tt::Delimiter::None, false),
    };

    let mut token_trees = Vec::new();
    let mut child_iter = tt.children_with_tokens().skip(skip_first as usize).peekable();

    while let Some(child) = child_iter.next() {
        if (skip_first && (child == first_child || child == last_child)) || child.kind().is_trivia()
        {
            continue;
        }
        match child {
            SyntaxElement::Token(token) => {
                if token.kind().is_punct() {
                    assert!(token.text().len() == 1, "Input ast::token punct must be single char.");
                    let char = token.text().chars().next().unwrap();

                    let spacing = match child_iter.peek() {
                        Some(SyntaxElement::Token(token)) => {
                            if token.kind().is_punct() {
                                tt::Spacing::Joint
                            } else {
                                tt::Spacing::Alone
                            }
                        }
                        _ => tt::Spacing::Alone,
                    };

                    token_trees.push(tt::Leaf::from(tt::Punct { char, spacing }).into());
                } else {
                    let child: tt::TokenTree = if token.kind() == SyntaxKind::TRUE_KW
                        || token.kind() == SyntaxKind::FALSE_KW
                    {
                        tt::Leaf::from(tt::Literal { text: token.text().clone() }).into()
                    } else if token.kind().is_keyword()
                        || token.kind() == IDENT
                        || token.kind() == LIFETIME
                    {
                        let relative_range = token.range() - global_offset;
                        let id = token_map.alloc(relative_range);
                        let text = token.text().clone();
                        tt::Leaf::from(tt::Ident { text, id }).into()
                    } else if token.kind().is_literal() {
                        tt::Leaf::from(tt::Literal { text: token.text().clone() }).into()
                    } else {
                        return None;
                    };
                    token_trees.push(child);
                }
            }
            SyntaxElement::Node(node) => {
                let child = convert_tt(token_map, global_offset, node)?.into();
                token_trees.push(child);
            }
        };
    }

    let res = tt::Subtree { delimiter, token_trees };
    Some(res)
}

struct TtTreeSink<'a, Q: Querier> {
    buf: String,
    src_querier: &'a Q,
    text_pos: TextUnit,
    token_pos: usize,
    inner: SyntaxTreeBuilder,
}

impl<'a, Q: Querier> TtTreeSink<'a, Q> {
    fn new(src_querier: &'a Q) -> Self {
        TtTreeSink {
            buf: String::new(),
            src_querier,
            text_pos: 0.into(),
            token_pos: 0,
            inner: SyntaxTreeBuilder::default(),
        }
    }
}

fn is_delimiter(kind: SyntaxKind) -> bool {
    use SyntaxKind::*;

    match kind {
        L_PAREN | L_BRACK | L_CURLY | R_PAREN | R_BRACK | R_CURLY => true,
        _ => false,
    }
}

impl<'a, Q: Querier> TreeSink for TtTreeSink<'a, Q> {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        if kind == L_DOLLAR || kind == R_DOLLAR {
            self.token_pos += n_tokens as usize;
            return;
        }

        for _ in 0..n_tokens {
            self.buf += &self.src_querier.token(self.token_pos).1;
            self.token_pos += 1;
        }
        self.text_pos += TextUnit::of_str(&self.buf);
        let text = SmolStr::new(self.buf.as_str());
        self.buf.clear();
        self.inner.token(kind, text);

        // Add a white space between tokens, only if both are not delimiters
        if !is_delimiter(kind) {
            let (last_kind, _, last_joint_to_next) = self.src_querier.token(self.token_pos - 1);
            if !last_joint_to_next && last_kind.is_punct() {
                let (cur_kind, _, _) = self.src_querier.token(self.token_pos);
                if !is_delimiter(cur_kind) {
                    if cur_kind.is_punct() {
                        self.inner.token(WHITESPACE, " ".into());
                    }
                }
            }
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.inner.start_node(kind);
    }

    fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    fn error(&mut self, error: ParseError) {
        self.inner.error(error, self.text_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{expand, create_rules};

    #[test]
    fn convert_tt_token_source() {
        let rules = create_rules(
            r#"
            macro_rules! literals {
                ($i:ident) => {
                    {
                        let a = 'c';
                        let c = 1000;
                        let f = 12E+99_f64;
                        let s = "rust1";
                    }
                }
            }
            "#,
        );
        let expansion = expand(&rules, "literals!(foo)");
        let tt_src = SubtreeTokenSource::new(&expansion);

        let query = tt_src.querier();

        // [${]
        // [let] [a] [=] ['c'] [;]
        assert_eq!(query.token(2 + 3).1, "'c'");
        assert_eq!(query.token(2 + 3).0, CHAR);
        // [let] [c] [=] [1000] [;]
        assert_eq!(query.token(2 + 5 + 3).1, "1000");
        assert_eq!(query.token(2 + 5 + 3).0, INT_NUMBER);
        // [let] [f] [=] [12E+99_f64] [;]
        assert_eq!(query.token(2 + 10 + 3).1, "12E+99_f64");
        assert_eq!(query.token(2 + 10 + 3).0, FLOAT_NUMBER);

        // [let] [s] [=] ["rust1"] [;]
        assert_eq!(query.token(2 + 15 + 3).1, "\"rust1\"");
        assert_eq!(query.token(2 + 15 + 3).0, STRING);
    }
}
