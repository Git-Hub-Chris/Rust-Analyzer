//! Various extension methods to ast Nodes, which are hard to code-generate.
//! Extensions for various expressions live in a sibling `expr_extensions` module.

use itertools::Itertools;

use crate::{
    ast::{
        self, child_opt, child_token_opt, children, AstElement, AstNode, AstToken, AttrInput,
        NameOwner, SyntaxNode,
    },
    SmolStr, SyntaxElement,
    SyntaxKind::*,
    SyntaxToken, T,
};
use ra_parser::SyntaxKind;

impl ast::Name {
    pub fn text(&self) -> &SmolStr {
        text_of_first_token(self.syntax())
    }
}

impl ast::NameRef {
    pub fn text(&self) -> &SmolStr {
        text_of_first_token(self.syntax())
    }

    pub fn as_tuple_field(&self) -> Option<usize> {
        self.syntax().children_with_tokens().find_map(|c| {
            if c.kind() == SyntaxKind::INT_NUMBER {
                c.as_token().and_then(|tok| tok.text().as_str().parse().ok())
            } else {
                None
            }
        })
    }
}

fn text_of_first_token(node: &SyntaxNode) -> &SmolStr {
    node.green().children().next().and_then(|it| it.into_token()).unwrap().text()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttrKind {
    Inner,
    Outer,
}

impl ast::Attr {
    pub fn as_simple_atom(&self) -> Option<SmolStr> {
        match self.input() {
            None => self.simple_name(),
            Some(_) => None,
        }
    }

    pub fn as_simple_call(&self) -> Option<(SmolStr, ast::TokenTree)> {
        match self.input() {
            Some(AttrInput::TokenTree(tt)) => Some((self.simple_name()?, tt)),
            _ => None,
        }
    }

    pub fn as_simple_key_value(&self) -> Option<(SmolStr, SmolStr)> {
        match self.input() {
            Some(AttrInput::Literal(lit)) => {
                let key = self.simple_name()?;
                // FIXME: escape? raw string?
                let value = lit.syntax().first_token()?.text().trim_matches('"').into();
                Some((key, value))
            }
            _ => None,
        }
    }

    pub fn simple_name(&self) -> Option<SmolStr> {
        let path = self.path()?;
        match (path.segment(), path.qualifier()) {
            (Some(segment), None) => Some(segment.syntax().first_token()?.text().clone()),
            _ => None,
        }
    }

    pub fn kind(&self) -> AttrKind {
        let first_token = self.syntax().first_token();
        let first_token_kind = first_token.as_ref().map(SyntaxToken::kind);
        let second_token_kind =
            first_token.and_then(|token| token.next_token()).as_ref().map(SyntaxToken::kind);

        match (first_token_kind, second_token_kind) {
            (Some(SyntaxKind::POUND), Some(SyntaxKind::EXCL)) => AttrKind::Inner,
            _ => AttrKind::Outer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegmentKind {
    Name(ast::NameRef),
    Type { type_ref: Option<ast::TypeRef>, trait_ref: Option<ast::PathType> },
    SelfKw,
    SuperKw,
    CrateKw,
}

impl ast::PathSegment {
    pub fn parent_path(&self) -> ast::Path {
        self.syntax()
            .parent()
            .and_then(ast::Path::cast)
            .expect("segments are always nested in paths")
    }

    pub fn kind(&self) -> Option<PathSegmentKind> {
        let res = if let Some(name_ref) = self.name_ref() {
            PathSegmentKind::Name(name_ref)
        } else {
            match self.syntax().first_child_or_token()?.kind() {
                T![self] => PathSegmentKind::SelfKw,
                T![super] => PathSegmentKind::SuperKw,
                T![crate] => PathSegmentKind::CrateKw,
                T![<] => {
                    // <T> or <T as Trait>
                    // T is any TypeRef, Trait has to be a PathType
                    let mut type_refs =
                        self.syntax().children().filter(|node| ast::TypeRef::can_cast(node.kind()));
                    let type_ref = type_refs.next().and_then(ast::TypeRef::cast);
                    let trait_ref = type_refs.next().and_then(ast::PathType::cast);
                    PathSegmentKind::Type { type_ref, trait_ref }
                }
                _ => return None,
            }
        };
        Some(res)
    }
}

impl ast::Path {
    pub fn parent_path(&self) -> Option<ast::Path> {
        self.syntax().parent().and_then(ast::Path::cast)
    }
}

impl ast::Module {
    pub fn has_semi(&self) -> bool {
        match self.syntax().last_child_or_token() {
            None => false,
            Some(node) => node.kind() == T![;],
        }
    }
}

impl ast::UseTreeList {
    pub fn parent_use_tree(&self) -> ast::UseTree {
        self.syntax()
            .parent()
            .and_then(ast::UseTree::cast)
            .expect("UseTreeLists are always nested in UseTrees")
    }
}

impl ast::ImplDef {
    pub fn target_type(&self) -> Option<ast::TypeRef> {
        match self.target() {
            (Some(t), None) | (_, Some(t)) => Some(t),
            _ => None,
        }
    }

    pub fn target_trait(&self) -> Option<ast::TypeRef> {
        match self.target() {
            (Some(t), Some(_)) => Some(t),
            _ => None,
        }
    }

    fn target(&self) -> (Option<ast::TypeRef>, Option<ast::TypeRef>) {
        let mut types = children(self);
        let first = types.next();
        let second = types.next();
        (first, second)
    }

    pub fn is_negative(&self) -> bool {
        self.syntax().children_with_tokens().any(|t| t.kind() == T![!])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructKind {
    Record(ast::RecordFieldDefList),
    Tuple(ast::TupleFieldDefList),
    Unit,
}

impl StructKind {
    fn from_node<N: AstNode>(node: &N) -> StructKind {
        if let Some(nfdl) = child_opt::<_, ast::RecordFieldDefList>(node) {
            StructKind::Record(nfdl)
        } else if let Some(pfl) = child_opt::<_, ast::TupleFieldDefList>(node) {
            StructKind::Tuple(pfl)
        } else {
            StructKind::Unit
        }
    }
}

impl ast::StructDef {
    pub fn kind(&self) -> StructKind {
        StructKind::from_node(self)
    }
}

impl ast::EnumVariant {
    pub fn parent_enum(&self) -> ast::EnumDef {
        self.syntax()
            .parent()
            .and_then(|it| it.parent())
            .and_then(ast::EnumDef::cast)
            .expect("EnumVariants are always nested in Enums")
    }
    pub fn kind(&self) -> StructKind {
        StructKind::from_node(self)
    }
}

impl ast::FnDef {
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        self.syntax()
            .last_child_or_token()
            .and_then(|it| it.into_token())
            .filter(|it| it.kind() == T![;])
    }

    pub fn is_async(&self) -> bool {
        self.syntax().children_with_tokens().any(|it| it.kind() == T![async])
    }
}

impl ast::LetStmt {
    pub fn has_semi(&self) -> bool {
        match self.syntax().last_child_or_token() {
            None => false,
            Some(node) => node.kind() == T![;],
        }
    }

    pub fn eq_token(&self) -> Option<SyntaxToken> {
        self.syntax().children_with_tokens().find(|t| t.kind() == EQ).and_then(|it| it.into_token())
    }
}

impl ast::ExprStmt {
    pub fn has_semi(&self) -> bool {
        match self.syntax().last_child_or_token() {
            None => false,
            Some(node) => node.kind() == T![;],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Name(ast::NameRef),
    Index(SyntaxToken),
}

impl ast::FieldExpr {
    pub fn index_token(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            // FIXME: Accepting floats here to reject them in validation later
            .find(|c| c.kind() == SyntaxKind::INT_NUMBER || c.kind() == SyntaxKind::FLOAT_NUMBER)
            .as_ref()
            .and_then(SyntaxElement::as_token)
            .cloned()
    }

    pub fn field_access(&self) -> Option<FieldKind> {
        if let Some(nr) = self.name_ref() {
            Some(FieldKind::Name(nr))
        } else if let Some(tok) = self.index_token() {
            Some(FieldKind::Index(tok))
        } else {
            None
        }
    }
}

impl ast::RefPat {
    pub fn is_mut(&self) -> bool {
        self.syntax().children_with_tokens().any(|n| n.kind() == T![mut])
    }
}

impl ast::BindPat {
    pub fn is_mutable(&self) -> bool {
        self.syntax().children_with_tokens().any(|n| n.kind() == T![mut])
    }

    pub fn is_ref(&self) -> bool {
        self.syntax().children_with_tokens().any(|n| n.kind() == T![ref])
    }
}

pub struct SlicePatComponents {
    pub prefix: Vec<ast::Pat>,
    pub slice: Option<ast::Pat>,
    pub suffix: Vec<ast::Pat>,
}

impl ast::SlicePat {
    pub fn components(&self) -> SlicePatComponents {
        let mut args = self.args().peekable();
        let prefix = args
            .peeking_take_while(|p| match p {
                ast::Pat::DotDotPat(_) => false,
                ast::Pat::BindPat(bp) => match bp.pat() {
                    Some(ast::Pat::DotDotPat(_)) => false,
                    _ => true,
                },
                ast::Pat::RefPat(rp) => match rp.pat() {
                    Some(ast::Pat::DotDotPat(_)) => false,
                    Some(ast::Pat::BindPat(bp)) => match bp.pat() {
                        Some(ast::Pat::DotDotPat(_)) => false,
                        _ => true,
                    },
                    _ => true,
                },
                _ => true,
            })
            .collect();
        let slice = args.next();
        let suffix = args.collect();

        SlicePatComponents { prefix, slice, suffix }
    }
}

impl ast::PointerType {
    pub fn is_mut(&self) -> bool {
        self.syntax().children_with_tokens().any(|n| n.kind() == T![mut])
    }
}

impl ast::ReferenceType {
    pub fn is_mut(&self) -> bool {
        self.syntax().children_with_tokens().any(|n| n.kind() == T![mut])
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelfParamKind {
    /// self
    Owned,
    /// &self
    Ref,
    /// &mut self
    MutRef,
}

impl ast::SelfParam {
    pub fn kind(&self) -> SelfParamKind {
        if self.amp().is_some() {
            if self.amp_mut_kw().is_some() {
                SelfParamKind::MutRef
            } else {
                SelfParamKind::Ref
            }
        } else {
            SelfParamKind::Owned
        }
    }

    /// the "mut" in "mut self", not the one in "&mut self"
    pub fn mut_kw(&self) -> Option<ast::MutKw> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .take_while(|it| it.kind() != T![&])
            .find_map(ast::MutKw::cast)
    }

    /// the "mut" in "&mut self", not the one in "mut self"
    pub fn amp_mut_kw(&self) -> Option<ast::MutKw> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .skip_while(|it| it.kind() != T![&])
            .find_map(ast::MutKw::cast)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeBoundKind {
    /// Trait
    PathType(ast::PathType),
    /// for<'a> ...
    ForType(ast::ForType),
    /// 'a
    Lifetime(ast::Lifetime),
}

impl ast::TypeBound {
    pub fn kind(&self) -> TypeBoundKind {
        if let Some(path_type) = children(self).next() {
            TypeBoundKind::PathType(path_type)
        } else if let Some(for_type) = children(self).next() {
            TypeBoundKind::ForType(for_type)
        } else if let Some(lifetime) = self.lifetime() {
            TypeBoundKind::Lifetime(lifetime)
        } else {
            unreachable!()
        }
    }

    pub fn has_question_mark(&self) -> bool {
        self.question().is_some()
    }

    pub fn const_question(&self) -> Option<ast::Question> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .take_while(|it| it.kind() != T![const])
            .find_map(ast::Question::cast)
    }

    pub fn question(&self) -> Option<ast::Question> {
        if self.const_kw().is_some() {
            self.syntax()
                .children_with_tokens()
                .filter_map(|it| it.into_token())
                .skip_while(|it| it.kind() != T![const])
                .find_map(ast::Question::cast)
        } else {
            child_token_opt(self)
        }
    }
}

impl ast::TraitDef {
    pub fn is_auto(&self) -> bool {
        self.syntax().children_with_tokens().any(|t| t.kind() == T![auto])
    }
}

pub enum VisibilityKind {
    In(ast::Path),
    PubCrate,
    PubSuper,
    PubSelf,
    Pub,
}

impl ast::Visibility {
    pub fn kind(&self) -> VisibilityKind {
        if let Some(path) = children(self).next() {
            VisibilityKind::In(path)
        } else if self.is_pub_crate() {
            VisibilityKind::PubCrate
        } else if self.is_pub_super() {
            VisibilityKind::PubSuper
        } else if self.is_pub_self() {
            VisibilityKind::PubSuper
        } else {
            VisibilityKind::Pub
        }
    }

    fn is_pub_crate(&self) -> bool {
        self.syntax().children_with_tokens().any(|it| it.kind() == T![crate])
    }

    fn is_pub_super(&self) -> bool {
        self.syntax().children_with_tokens().any(|it| it.kind() == T![super])
    }

    fn is_pub_self(&self) -> bool {
        self.syntax().children_with_tokens().any(|it| it.kind() == T![self])
    }
}

impl ast::MacroCall {
    pub fn is_macro_rules(&self) -> Option<ast::Name> {
        let name_ref = self.path()?.segment()?.name_ref()?;
        if name_ref.text() == "macro_rules" {
            self.name()
        } else {
            None
        }
    }
}

impl ast::LifetimeParam {
    pub fn lifetime_bounds(&self) -> impl Iterator<Item = ast::Lifetime> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .skip_while(|x| x.kind() != T![:])
            .filter_map(ast::Lifetime::cast)
    }
}

impl ast::RangePat {
    pub fn start(&self) -> Option<ast::Pat> {
        self.syntax()
            .children_with_tokens()
            .take_while(|it| !ast::RangeSeparator::can_cast_element(it.kind()))
            .filter_map(|it| it.into_node())
            .find_map(ast::Pat::cast)
    }

    pub fn end(&self) -> Option<ast::Pat> {
        self.syntax()
            .children_with_tokens()
            .skip_while(|it| !ast::RangeSeparator::can_cast_element(it.kind()))
            .filter_map(|it| it.into_node())
            .find_map(ast::Pat::cast)
    }
}

impl ast::TokenTree {
    pub fn left_delimiter(&self) -> Option<ast::LeftDelimiter> {
        self.syntax().first_child_or_token().and_then(ast::LeftDelimiter::cast_element)
    }

    pub fn right_delimiter(&self) -> Option<ast::RightDelimiter> {
        self.syntax().last_child_or_token().and_then(ast::RightDelimiter::cast_element)
    }
}
