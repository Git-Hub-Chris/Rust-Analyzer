//! Generated file, do not edit by hand, see `xtask/src/codegen`

#[allow(unused_imports)]
use crate::{
    ast::{self, AstChildElements, AstChildTokens, AstChildren, AstElement, AstNode, AstToken},
    NodeOrToken, SyntaxElement,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Semi(SyntaxToken);
impl std::fmt::Display for Semi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Semi {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SEMI => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Semi {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SEMI => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comma(SyntaxToken);
impl std::fmt::Display for Comma {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Comma {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            COMMA => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Comma {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            COMMA => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LParen(SyntaxToken);
impl std::fmt::Display for LParen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for LParen {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            L_PAREN => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for LParen {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            L_PAREN => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RParen(SyntaxToken);
impl std::fmt::Display for RParen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RParen {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            R_PAREN => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RParen {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            R_PAREN => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LCurly(SyntaxToken);
impl std::fmt::Display for LCurly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for LCurly {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            L_CURLY => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for LCurly {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            L_CURLY => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RCurly(SyntaxToken);
impl std::fmt::Display for RCurly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RCurly {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            R_CURLY => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RCurly {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            R_CURLY => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LBrack(SyntaxToken);
impl std::fmt::Display for LBrack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for LBrack {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            L_BRACK => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for LBrack {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            L_BRACK => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RBrack(SyntaxToken);
impl std::fmt::Display for RBrack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RBrack {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            R_BRACK => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RBrack {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            R_BRACK => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LAngle(SyntaxToken);
impl std::fmt::Display for LAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for LAngle {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            L_ANGLE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for LAngle {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            L_ANGLE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RAngle(SyntaxToken);
impl std::fmt::Display for RAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RAngle {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            R_ANGLE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RAngle {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            R_ANGLE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct At(SyntaxToken);
impl std::fmt::Display for At {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for At {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for At {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pound(SyntaxToken);
impl std::fmt::Display for Pound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Pound {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            POUND => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Pound {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            POUND => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tilde(SyntaxToken);
impl std::fmt::Display for Tilde {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Tilde {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TILDE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Tilde {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TILDE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Question(SyntaxToken);
impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Question {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            QUESTION => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Question {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            QUESTION => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dollar(SyntaxToken);
impl std::fmt::Display for Dollar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Dollar {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DOLLAR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Dollar {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DOLLAR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Amp(SyntaxToken);
impl std::fmt::Display for Amp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Amp {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AMP => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Amp {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AMP => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipe(SyntaxToken);
impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Pipe {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PIPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Pipe {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PIPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Plus(SyntaxToken);
impl std::fmt::Display for Plus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Plus {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PLUS => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Plus {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PLUS => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Star(SyntaxToken);
impl std::fmt::Display for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Star {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STAR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Star {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            STAR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slash(SyntaxToken);
impl std::fmt::Display for Slash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Slash {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SLASH => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Slash {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SLASH => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Caret(SyntaxToken);
impl std::fmt::Display for Caret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Caret {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CARET => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Caret {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CARET => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Percent(SyntaxToken);
impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Percent {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PERCENT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Percent {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PERCENT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Underscore(SyntaxToken);
impl std::fmt::Display for Underscore {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Underscore {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            UNDERSCORE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Underscore {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            UNDERSCORE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dot(SyntaxToken);
impl std::fmt::Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Dot {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DOT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Dot {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DOT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dotdot(SyntaxToken);
impl std::fmt::Display for Dotdot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Dotdot {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DOTDOT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Dotdot {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DOTDOT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dotdotdot(SyntaxToken);
impl std::fmt::Display for Dotdotdot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Dotdotdot {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DOTDOTDOT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Dotdotdot {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DOTDOTDOT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dotdoteq(SyntaxToken);
impl std::fmt::Display for Dotdoteq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Dotdoteq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DOTDOTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Dotdoteq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DOTDOTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Colon(SyntaxToken);
impl std::fmt::Display for Colon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Colon {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            COLON => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Colon {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            COLON => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coloncolon(SyntaxToken);
impl std::fmt::Display for Coloncolon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Coloncolon {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            COLONCOLON => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Coloncolon {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            COLONCOLON => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eq(SyntaxToken);
impl std::fmt::Display for Eq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Eq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Eq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eqeq(SyntaxToken);
impl std::fmt::Display for Eqeq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Eqeq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EQEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Eqeq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EQEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FatArrow(SyntaxToken);
impl std::fmt::Display for FatArrow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for FatArrow {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FAT_ARROW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for FatArrow {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FAT_ARROW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Excl(SyntaxToken);
impl std::fmt::Display for Excl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Excl {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXCL => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Excl {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EXCL => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Neq(SyntaxToken);
impl std::fmt::Display for Neq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Neq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            NEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Neq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            NEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Minus(SyntaxToken);
impl std::fmt::Display for Minus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Minus {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MINUS => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Minus {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MINUS => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThinArrow(SyntaxToken);
impl std::fmt::Display for ThinArrow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ThinArrow {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            THIN_ARROW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ThinArrow {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            THIN_ARROW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lteq(SyntaxToken);
impl std::fmt::Display for Lteq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Lteq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Lteq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gteq(SyntaxToken);
impl std::fmt::Display for Gteq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Gteq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Gteq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            GTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pluseq(SyntaxToken);
impl std::fmt::Display for Pluseq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Pluseq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PLUSEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Pluseq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PLUSEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Minuseq(SyntaxToken);
impl std::fmt::Display for Minuseq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Minuseq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MINUSEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Minuseq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MINUSEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipeeq(SyntaxToken);
impl std::fmt::Display for Pipeeq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Pipeeq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PIPEEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Pipeeq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PIPEEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ampeq(SyntaxToken);
impl std::fmt::Display for Ampeq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Ampeq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AMPEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Ampeq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AMPEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Careteq(SyntaxToken);
impl std::fmt::Display for Careteq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Careteq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CARETEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Careteq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CARETEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slasheq(SyntaxToken);
impl std::fmt::Display for Slasheq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Slasheq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SLASHEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Slasheq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SLASHEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stareq(SyntaxToken);
impl std::fmt::Display for Stareq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Stareq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STAREQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Stareq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            STAREQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Percenteq(SyntaxToken);
impl std::fmt::Display for Percenteq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Percenteq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PERCENTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Percenteq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PERCENTEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ampamp(SyntaxToken);
impl std::fmt::Display for Ampamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Ampamp {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AMPAMP => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Ampamp {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AMPAMP => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipepipe(SyntaxToken);
impl std::fmt::Display for Pipepipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Pipepipe {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PIPEPIPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Pipepipe {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PIPEPIPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shl(SyntaxToken);
impl std::fmt::Display for Shl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Shl {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SHL => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Shl {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SHL => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shr(SyntaxToken);
impl std::fmt::Display for Shr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Shr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SHR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Shr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SHR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shleq(SyntaxToken);
impl std::fmt::Display for Shleq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Shleq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SHLEQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Shleq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SHLEQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shreq(SyntaxToken);
impl std::fmt::Display for Shreq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Shreq {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SHREQ => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Shreq {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SHREQ => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsKw(SyntaxToken);
impl std::fmt::Display for AsKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for AsKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AS_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for AsKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AS_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncKw(SyntaxToken);
impl std::fmt::Display for AsyncKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for AsyncKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ASYNC_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for AsyncKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ASYNC_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitKw(SyntaxToken);
impl std::fmt::Display for AwaitKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for AwaitKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AWAIT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for AwaitKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AWAIT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoxKw(SyntaxToken);
impl std::fmt::Display for BoxKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for BoxKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BOX_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for BoxKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BOX_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakKw(SyntaxToken);
impl std::fmt::Display for BreakKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for BreakKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BREAK_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for BreakKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BREAK_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstKw(SyntaxToken);
impl std::fmt::Display for ConstKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ConstKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONST_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ConstKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONST_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueKw(SyntaxToken);
impl std::fmt::Display for ContinueKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ContinueKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONTINUE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ContinueKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONTINUE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateKw(SyntaxToken);
impl std::fmt::Display for CrateKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for CrateKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CRATE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for CrateKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CRATE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynKw(SyntaxToken);
impl std::fmt::Display for DynKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for DynKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DYN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for DynKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DYN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElseKw(SyntaxToken);
impl std::fmt::Display for ElseKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ElseKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ELSE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ElseKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ELSE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumKw(SyntaxToken);
impl std::fmt::Display for EnumKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for EnumKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for EnumKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternKw(SyntaxToken);
impl std::fmt::Display for ExternKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ExternKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXTERN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ExternKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EXTERN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FalseKw(SyntaxToken);
impl std::fmt::Display for FalseKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for FalseKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FALSE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for FalseKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FALSE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnKw(SyntaxToken);
impl std::fmt::Display for FnKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for FnKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for FnKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForKw(SyntaxToken);
impl std::fmt::Display for ForKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ForKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FOR_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ForKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FOR_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfKw(SyntaxToken);
impl std::fmt::Display for IfKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for IfKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IF_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for IfKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            IF_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplKw(SyntaxToken);
impl std::fmt::Display for ImplKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ImplKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IMPL_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ImplKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            IMPL_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InKw(SyntaxToken);
impl std::fmt::Display for InKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for InKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for InKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            IN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetKw(SyntaxToken);
impl std::fmt::Display for LetKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for LetKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LET_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for LetKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LET_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoopKw(SyntaxToken);
impl std::fmt::Display for LoopKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for LoopKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LOOP_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for LoopKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LOOP_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroKw(SyntaxToken);
impl std::fmt::Display for MacroKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for MacroKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for MacroKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchKw(SyntaxToken);
impl std::fmt::Display for MatchKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for MatchKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for MatchKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModKw(SyntaxToken);
impl std::fmt::Display for ModKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ModKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MOD_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ModKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MOD_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveKw(SyntaxToken);
impl std::fmt::Display for MoveKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for MoveKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MOVE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for MoveKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MOVE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutKw(SyntaxToken);
impl std::fmt::Display for MutKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for MutKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MUT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for MutKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MUT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubKw(SyntaxToken);
impl std::fmt::Display for PubKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for PubKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PUB_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for PubKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PUB_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefKw(SyntaxToken);
impl std::fmt::Display for RefKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RefKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            REF_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RefKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            REF_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnKw(SyntaxToken);
impl std::fmt::Display for ReturnKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ReturnKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RETURN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ReturnKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RETURN_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelfKw(SyntaxToken);
impl std::fmt::Display for SelfKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for SelfKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SELF_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for SelfKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SELF_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticKw(SyntaxToken);
impl std::fmt::Display for StaticKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for StaticKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STATIC_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for StaticKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            STATIC_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructKw(SyntaxToken);
impl std::fmt::Display for StructKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for StructKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STRUCT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for StructKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            STRUCT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuperKw(SyntaxToken);
impl std::fmt::Display for SuperKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for SuperKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SUPER_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for SuperKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SUPER_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitKw(SyntaxToken);
impl std::fmt::Display for TraitKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for TraitKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRAIT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for TraitKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TRAIT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrueKw(SyntaxToken);
impl std::fmt::Display for TrueKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for TrueKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRUE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for TrueKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TRUE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryKw(SyntaxToken);
impl std::fmt::Display for TryKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for TryKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRY_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for TryKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TRY_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeKw(SyntaxToken);
impl std::fmt::Display for TypeKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for TypeKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for TypeKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnsafeKw(SyntaxToken);
impl std::fmt::Display for UnsafeKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for UnsafeKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            UNSAFE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for UnsafeKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            UNSAFE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseKw(SyntaxToken);
impl std::fmt::Display for UseKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for UseKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            USE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for UseKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            USE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhereKw(SyntaxToken);
impl std::fmt::Display for WhereKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for WhereKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WHERE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for WhereKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            WHERE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileKw(SyntaxToken);
impl std::fmt::Display for WhileKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for WhileKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WHILE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for WhileKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            WHILE_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutoKw(SyntaxToken);
impl std::fmt::Display for AutoKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for AutoKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AUTO_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for AutoKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AUTO_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefaultKw(SyntaxToken);
impl std::fmt::Display for DefaultKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for DefaultKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DEFAULT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for DefaultKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DEFAULT_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExistentialKw(SyntaxToken);
impl std::fmt::Display for ExistentialKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ExistentialKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXISTENTIAL_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ExistentialKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EXISTENTIAL_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnionKw(SyntaxToken);
impl std::fmt::Display for UnionKw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for UnionKw {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            UNION_KW => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for UnionKw {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            UNION_KW => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntNumber(SyntaxToken);
impl std::fmt::Display for IntNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for IntNumber {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            INT_NUMBER => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for IntNumber {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            INT_NUMBER => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FloatNumber(SyntaxToken);
impl std::fmt::Display for FloatNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for FloatNumber {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FLOAT_NUMBER => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for FloatNumber {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FLOAT_NUMBER => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Char(SyntaxToken);
impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Char {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CHAR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Char {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CHAR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Byte(SyntaxToken);
impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Byte {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BYTE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Byte {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BYTE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String(SyntaxToken);
impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for String {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STRING => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for String {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            STRING => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawString(SyntaxToken);
impl std::fmt::Display for RawString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RawString {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RAW_STRING => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RawString {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RAW_STRING => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ByteString(SyntaxToken);
impl std::fmt::Display for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for ByteString {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BYTE_STRING => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for ByteString {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BYTE_STRING => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawByteString(SyntaxToken);
impl std::fmt::Display for RawByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RawByteString {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RAW_BYTE_STRING => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RawByteString {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RAW_BYTE_STRING => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error(SyntaxToken);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Error {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ERROR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Error {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ERROR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(SyntaxToken);
impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Ident {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IDENT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Ident {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            IDENT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Whitespace(SyntaxToken);
impl std::fmt::Display for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WHITESPACE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Whitespace {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            WHITESPACE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime(SyntaxToken);
impl std::fmt::Display for Lifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Lifetime {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LIFETIME => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Lifetime {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LIFETIME => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment(SyntaxToken);
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Comment {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            COMMENT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Comment {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            COMMENT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shebang(SyntaxToken);
impl std::fmt::Display for Shebang {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for Shebang {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SHEBANG => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for Shebang {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SHEBANG => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LDollar(SyntaxToken);
impl std::fmt::Display for LDollar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for LDollar {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            L_DOLLAR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for LDollar {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            L_DOLLAR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RDollar(SyntaxToken);
impl std::fmt::Display for RDollar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstToken for RDollar {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            R_DOLLAR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxToken) -> Result<Self, SyntaxToken> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self(syntax))
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
    fn into_syntax(self) -> SyntaxToken {
        self.0
    }
}
impl AstElement for RDollar {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            R_DOLLAR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self(syntax.into_token().unwrap()))
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Token(&self.0)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Token(self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SOURCE_FILE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for SourceFile {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SOURCE_FILE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::ModuleItemOwner for SourceFile {}
impl ast::FnDefOwner for SourceFile {}
impl SourceFile {
    pub fn modules(&self) -> AstChildren<Module> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for FnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for FnDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FN_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for FnDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FN_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for FnDef {}
impl ast::NameOwner for FnDef {}
impl ast::TypeParamsOwner for FnDef {}
impl ast::DocCommentsOwner for FnDef {}
impl ast::AttrsOwner for FnDef {}
impl FnDef {
    pub fn param_list(&self) -> Option<ParamList> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn ret_type(&self) -> Option<RetType> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn body(&self) -> Option<BlockExpr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RetType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RetType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RET_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RetType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RET_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RetType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for StructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for StructDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STRUCT_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for StructDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            STRUCT_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for StructDef {}
impl ast::NameOwner for StructDef {}
impl ast::TypeParamsOwner for StructDef {}
impl ast::AttrsOwner for StructDef {}
impl ast::DocCommentsOwner for StructDef {}
impl StructDef {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnionDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for UnionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for UnionDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            UNION_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for UnionDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            UNION_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for UnionDef {}
impl ast::NameOwner for UnionDef {}
impl ast::TypeParamsOwner for UnionDef {}
impl ast::AttrsOwner for UnionDef {}
impl ast::DocCommentsOwner for UnionDef {}
impl UnionDef {
    pub fn record_field_def_list(&self) -> Option<RecordFieldDefList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldDefList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordFieldDefList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordFieldDefList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_DEF_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordFieldDefList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_DEF_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RecordFieldDefList {
    pub fn fields(&self) -> AstChildren<RecordFieldDef> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordFieldDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordFieldDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordFieldDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for RecordFieldDef {}
impl ast::NameOwner for RecordFieldDef {}
impl ast::AttrsOwner for RecordFieldDef {}
impl ast::DocCommentsOwner for RecordFieldDef {}
impl ast::TypeAscriptionOwner for RecordFieldDef {}
impl RecordFieldDef {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleFieldDefList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TupleFieldDefList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TupleFieldDefList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_FIELD_DEF_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TupleFieldDefList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_FIELD_DEF_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TupleFieldDefList {
    pub fn fields(&self) -> AstChildren<TupleFieldDef> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleFieldDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TupleFieldDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TupleFieldDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_FIELD_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TupleFieldDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_FIELD_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for TupleFieldDef {}
impl ast::AttrsOwner for TupleFieldDef {}
impl TupleFieldDef {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for EnumDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for EnumDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for EnumDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for EnumDef {}
impl ast::NameOwner for EnumDef {}
impl ast::TypeParamsOwner for EnumDef {}
impl ast::AttrsOwner for EnumDef {}
impl ast::DocCommentsOwner for EnumDef {}
impl EnumDef {
    pub fn variant_list(&self) -> Option<EnumVariantList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumVariantList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for EnumVariantList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for EnumVariantList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_VARIANT_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for EnumVariantList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_VARIANT_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl EnumVariantList {
    pub fn variants(&self) -> AstChildren<EnumVariant> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumVariant {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for EnumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for EnumVariant {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_VARIANT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for EnumVariant {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_VARIANT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::NameOwner for EnumVariant {}
impl ast::DocCommentsOwner for EnumVariant {}
impl ast::AttrsOwner for EnumVariant {}
impl EnumVariant {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TraitDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TraitDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRAIT_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TraitDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TRAIT_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for TraitDef {}
impl ast::NameOwner for TraitDef {}
impl ast::AttrsOwner for TraitDef {}
impl ast::DocCommentsOwner for TraitDef {}
impl ast::TypeParamsOwner for TraitDef {}
impl ast::TypeBoundsOwner for TraitDef {}
impl TraitDef {
    pub fn item_list(&self) -> Option<ItemList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Module {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MODULE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Module {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MODULE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for Module {}
impl ast::NameOwner for Module {}
impl ast::AttrsOwner for Module {}
impl ast::DocCommentsOwner for Module {}
impl Module {
    pub fn item_list(&self) -> Option<ItemList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ItemList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ITEM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ItemList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ITEM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::FnDefOwner for ItemList {}
impl ast::ModuleItemOwner for ItemList {}
impl ItemList {
    pub fn impl_items(&self) -> AstChildren<ImplItem> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ConstDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ConstDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONST_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ConstDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONST_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for ConstDef {}
impl ast::NameOwner for ConstDef {}
impl ast::TypeParamsOwner for ConstDef {}
impl ast::AttrsOwner for ConstDef {}
impl ast::DocCommentsOwner for ConstDef {}
impl ast::TypeAscriptionOwner for ConstDef {}
impl ConstDef {
    pub fn body(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for StaticDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for StaticDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STATIC_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for StaticDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            STATIC_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for StaticDef {}
impl ast::NameOwner for StaticDef {}
impl ast::TypeParamsOwner for StaticDef {}
impl ast::AttrsOwner for StaticDef {}
impl ast::DocCommentsOwner for StaticDef {}
impl ast::TypeAscriptionOwner for StaticDef {}
impl StaticDef {
    pub fn body(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAliasDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TypeAliasDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TypeAliasDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_ALIAS_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TypeAliasDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_ALIAS_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::VisibilityOwner for TypeAliasDef {}
impl ast::NameOwner for TypeAliasDef {}
impl ast::TypeParamsOwner for TypeAliasDef {}
impl ast::AttrsOwner for TypeAliasDef {}
impl ast::DocCommentsOwner for TypeAliasDef {}
impl ast::TypeBoundsOwner for TypeAliasDef {}
impl TypeAliasDef {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplDef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ImplDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ImplDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IMPL_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ImplDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            IMPL_DEF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::TypeParamsOwner for ImplDef {}
impl ast::AttrsOwner for ImplDef {}
impl ImplDef {
    pub fn item_list(&self) -> Option<ItemList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ParenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ParenType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ParenType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ParenType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TupleType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TupleType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TupleType {
    pub fn fields(&self) -> AstChildren<TypeRef> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NeverType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for NeverType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for NeverType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            NEVER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for NeverType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            NEVER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl NeverType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PathType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PathType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PATH_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PathType {
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointerType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PointerType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            POINTER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PointerType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            POINTER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PointerType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ArrayType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ArrayType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ArrayType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SliceType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for SliceType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SLICE_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for SliceType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SLICE_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl SliceType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferenceType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ReferenceType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            REFERENCE_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ReferenceType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            REFERENCE_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ReferenceType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceholderType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PlaceholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PlaceholderType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PLACEHOLDER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PlaceholderType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PLACEHOLDER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PlaceholderType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnPointerType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for FnPointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for FnPointerType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FN_POINTER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for FnPointerType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FN_POINTER_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl FnPointerType {
    pub fn param_list(&self) -> Option<ParamList> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn ret_type(&self) -> Option<RetType> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ForType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ForType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FOR_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ForType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FOR_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ForType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplTraitType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ImplTraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ImplTraitType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IMPL_TRAIT_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ImplTraitType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            IMPL_TRAIT_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::TypeBoundsOwner for ImplTraitType {}
impl ImplTraitType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynTraitType {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for DynTraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for DynTraitType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DYN_TRAIT_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for DynTraitType {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DYN_TRAIT_TYPE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::TypeBoundsOwner for DynTraitType {}
impl DynTraitType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TupleExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TupleExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TupleExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TupleExpr {
    pub fn exprs(&self) -> AstChildren<Expr> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ArrayExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ArrayExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ArrayExpr {
    pub fn exprs(&self) -> AstChildren<Expr> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ParenExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ParenExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PathExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PathExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PathExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PATH_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PathExpr {
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for LambdaExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for LambdaExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LAMBDA_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for LambdaExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LAMBDA_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl LambdaExpr {
    pub fn param_list(&self) -> Option<ParamList> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn ret_type(&self) -> Option<RetType> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn body(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for IfExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IF_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for IfExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            IF_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl IfExpr {
    pub fn condition(&self) -> Option<Condition> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoopExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for LoopExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for LoopExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LOOP_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for LoopExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LOOP_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::LoopBodyOwner for LoopExpr {}
impl LoopExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryBlockExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TryBlockExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TryBlockExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRY_BLOCK_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TryBlockExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TRY_BLOCK_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TryBlockExpr {
    pub fn body(&self) -> Option<BlockExpr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ForExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ForExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FOR_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ForExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FOR_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::LoopBodyOwner for ForExpr {}
impl ForExpr {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn iterable(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for WhileExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for WhileExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WHILE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for WhileExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            WHILE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::LoopBodyOwner for WhileExpr {}
impl WhileExpr {
    pub fn condition(&self) -> Option<Condition> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ContinueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ContinueExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONTINUE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ContinueExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONTINUE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ContinueExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for BreakExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for BreakExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BREAK_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for BreakExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BREAK_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl BreakExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Label {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LABEL => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Label {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LABEL => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl Label {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for BlockExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for BlockExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BLOCK_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for BlockExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BLOCK_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl BlockExpr {
    pub fn block(&self) -> Option<Block> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ReturnExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ReturnExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RETURN_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ReturnExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RETURN_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ReturnExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for CallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CALL_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for CallExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CALL_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::ArgListOwner for CallExpr {}
impl CallExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MethodCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MethodCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            METHOD_CALL_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MethodCallExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            METHOD_CALL_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::ArgListOwner for MethodCallExpr {}
impl MethodCallExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn name_ref(&self) -> Option<NameRef> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn type_arg_list(&self) -> Option<TypeArgList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for IndexExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            INDEX_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for IndexExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            INDEX_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl IndexExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for FieldExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for FieldExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FIELD_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for FieldExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            FIELD_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl FieldExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn name_ref(&self) -> Option<NameRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for AwaitExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for AwaitExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AWAIT_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for AwaitExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            AWAIT_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl AwaitExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRY_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TryExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TRY_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TryExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for CastExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for CastExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CAST_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for CastExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CAST_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl CastExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RefExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RefExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            REF_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RefExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            REF_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RefExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PrefixExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PREFIX_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PrefixExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PREFIX_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PrefixExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoxExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for BoxExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for BoxExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BOX_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for BoxExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BOX_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl BoxExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangeExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RangeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RangeExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RANGE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RangeExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RANGE_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RangeExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for BinExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for BinExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BIN_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for BinExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BIN_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl BinExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Literal {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl Literal {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchExpr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MatchExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MatchExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MatchExpr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_EXPR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl MatchExpr {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn match_arm_list(&self) -> Option<MatchArmList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArmList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MatchArmList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MatchArmList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_ARM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MatchArmList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_ARM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::AttrsOwner for MatchArmList {}
impl MatchArmList {
    pub fn arms(&self) -> AstChildren<MatchArm> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MatchArm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MatchArm {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_ARM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MatchArm {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_ARM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::AttrsOwner for MatchArm {}
impl MatchArm {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn guard(&self) -> Option<MatchGuard> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchGuard {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MatchGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MatchGuard {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_GUARD => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MatchGuard {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MATCH_GUARD => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl MatchGuard {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordLit {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordLit {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_LIT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordLit {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_LIT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RecordLit {
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn record_field_list(&self) -> Option<RecordFieldList> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordFieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordFieldList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordFieldList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RecordFieldList {
    pub fn fields(&self) -> AstChildren<RecordField> {
        AstChildren::new(&self.syntax)
    }
    pub fn spread(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordField {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordField {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordField {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RecordField {
    pub fn name_ref(&self) -> Option<NameRef> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for OrPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for OrPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            OR_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for OrPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            OR_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl OrPat {
    pub fn pats(&self) -> AstChildren<Pat> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ParenPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ParenPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ParenPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ParenPat {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RefPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RefPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            REF_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RefPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            REF_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RefPat {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoxPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for BoxPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for BoxPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BOX_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for BoxPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BOX_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl BoxPat {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for BindPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for BindPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BIND_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for BindPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BIND_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::NameOwner for BindPat {}
impl BindPat {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceholderPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PlaceholderPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PlaceholderPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PLACEHOLDER_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PlaceholderPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PLACEHOLDER_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PlaceholderPat {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotDotPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for DotDotPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for DotDotPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DOT_DOT_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for DotDotPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            DOT_DOT_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl DotDotPat {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PathPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PathPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PathPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PATH_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PathPat {
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlicePat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for SlicePat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for SlicePat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SLICE_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for SlicePat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SLICE_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl SlicePat {
    pub fn args(&self) -> AstChildren<Pat> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangePat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RangePat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RangePat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RANGE_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RangePat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RANGE_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RangePat {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for LiteralPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for LiteralPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for LiteralPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl LiteralPat {
    pub fn literal(&self) -> Option<Literal> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RecordPat {
    pub fn record_field_pat_list(&self) -> Option<RecordFieldPatList> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldPatList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordFieldPatList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordFieldPatList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_PAT_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordFieldPatList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_PAT_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl RecordFieldPatList {
    pub fn record_field_pats(&self) -> AstChildren<RecordFieldPat> {
        AstChildren::new(&self.syntax)
    }
    pub fn bind_pats(&self) -> AstChildren<BindPat> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for RecordFieldPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for RecordFieldPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for RecordFieldPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::NameOwner for RecordFieldPat {}
impl RecordFieldPat {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleStructPat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TupleStructPat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TupleStructPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_STRUCT_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TupleStructPat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_STRUCT_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TupleStructPat {
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn args(&self) -> AstChildren<Pat> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePat {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TuplePat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TuplePat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TuplePat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_PAT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TuplePat {
    pub fn args(&self) -> AstChildren<Pat> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Visibility {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Visibility {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            VISIBILITY => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Visibility {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            VISIBILITY => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl Visibility {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            NAME => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Name {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            NAME => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl Name {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for NameRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for NameRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            NAME_REF => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for NameRef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            NAME_REF => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl NameRef {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroCall {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MacroCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MacroCall {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_CALL => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MacroCall {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_CALL => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::NameOwner for MacroCall {}
impl ast::AttrsOwner for MacroCall {}
impl ast::DocCommentsOwner for MacroCall {}
impl MacroCall {
    pub fn token_tree(&self) -> Option<TokenTree> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Attr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ATTR => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Attr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ATTR => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl Attr {
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn input(&self) -> Option<AttrInput> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenTree {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TokenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TokenTree {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TOKEN_TREE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TokenTree {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TOKEN_TREE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TokenTree {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParamList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TypeParamList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TypeParamList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_PARAM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TypeParamList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_PARAM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TypeParamList {
    pub fn type_params(&self) -> AstChildren<TypeParam> {
        AstChildren::new(&self.syntax)
    }
    pub fn lifetime_params(&self) -> AstChildren<LifetimeParam> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParam {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TypeParam {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TypeParam {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::NameOwner for TypeParam {}
impl ast::AttrsOwner for TypeParam {}
impl ast::TypeBoundsOwner for TypeParam {}
impl TypeParam {
    pub fn default_type(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstParam {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ConstParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ConstParam {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONST_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ConstParam {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONST_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::NameOwner for ConstParam {}
impl ast::AttrsOwner for ConstParam {}
impl ast::TypeAscriptionOwner for ConstParam {}
impl ConstParam {
    pub fn default_val(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LifetimeParam {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for LifetimeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for LifetimeParam {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LIFETIME_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for LifetimeParam {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LIFETIME_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::AttrsOwner for LifetimeParam {}
impl LifetimeParam {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeBound {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TypeBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TypeBound {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_BOUND => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TypeBound {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_BOUND => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TypeBound {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeBoundList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TypeBoundList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TypeBoundList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_BOUND_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TypeBoundList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_BOUND_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TypeBoundList {
    pub fn bounds(&self) -> AstChildren<TypeBound> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WherePred {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for WherePred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for WherePred {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WHERE_PRED => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for WherePred {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            WHERE_PRED => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::TypeBoundsOwner for WherePred {}
impl WherePred {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhereClause {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for WhereClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for WhereClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WHERE_CLAUSE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for WhereClause {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            WHERE_CLAUSE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl WhereClause {
    pub fn predicates(&self) -> AstChildren<WherePred> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ExprStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXPR_STMT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ExprStmt {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EXPR_STMT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ExprStmt {
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStmt {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for LetStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for LetStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LET_STMT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for LetStmt {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LET_STMT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::TypeAscriptionOwner for LetStmt {}
impl LetStmt {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn initializer(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Condition {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Condition {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONDITION => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Condition {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONDITION => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl Condition {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Block {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BLOCK => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Block {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BLOCK => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::AttrsOwner for Block {}
impl ast::ModuleItemOwner for Block {}
impl Block {
    pub fn statements(&self) -> AstChildren<Stmt> {
        AstChildren::new(&self.syntax)
    }
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ParamList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ParamList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PARAM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ParamList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PARAM_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ParamList {
    pub fn self_param(&self) -> Option<SelfParam> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn params(&self) -> AstChildren<Param> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelfParam {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for SelfParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for SelfParam {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SELF_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for SelfParam {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            SELF_PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::TypeAscriptionOwner for SelfParam {}
impl ast::AttrsOwner for SelfParam {}
impl SelfParam {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Param {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Param {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PARAM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::TypeAscriptionOwner for Param {}
impl ast::AttrsOwner for Param {}
impl Param {
    pub fn pat(&self) -> Option<Pat> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseItem {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for UseItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for UseItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            USE_ITEM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for UseItem {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            USE_ITEM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::AttrsOwner for UseItem {}
impl ast::VisibilityOwner for UseItem {}
impl UseItem {
    pub fn use_tree(&self) -> Option<UseTree> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseTree {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for UseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for UseTree {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            USE_TREE => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for UseTree {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            USE_TREE => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl UseTree {
    pub fn path(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn use_tree_list(&self) -> Option<UseTreeList> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn alias(&self) -> Option<Alias> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alias {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Alias {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ALIAS => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Alias {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ALIAS => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::NameOwner for Alias {}
impl Alias {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseTreeList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for UseTreeList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for UseTreeList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            USE_TREE_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for UseTreeList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            USE_TREE_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl UseTreeList {
    pub fn use_trees(&self) -> AstChildren<UseTree> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternCrateItem {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ExternCrateItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ExternCrateItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXTERN_CRATE_ITEM => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ExternCrateItem {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EXTERN_CRATE_ITEM => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::AttrsOwner for ExternCrateItem {}
impl ast::VisibilityOwner for ExternCrateItem {}
impl ExternCrateItem {
    pub fn name_ref(&self) -> Option<NameRef> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn alias(&self) -> Option<Alias> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ArgList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ARG_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ArgList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ARG_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ArgList {
    pub fn args(&self) -> AstChildren<Expr> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for Path {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for Path {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PATH => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl Path {
    pub fn segment(&self) -> Option<PathSegment> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn qualifier(&self) -> Option<Path> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for PathSegment {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_SEGMENT => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for PathSegment {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            PATH_SEGMENT => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl PathSegment {
    pub fn name_ref(&self) -> Option<NameRef> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn type_arg_list(&self) -> Option<TypeArgList> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn param_list(&self) -> Option<ParamList> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn ret_type(&self) -> Option<RetType> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn path_type(&self) -> Option<PathType> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArgList {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TypeArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TypeArgList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_ARG_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TypeArgList {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_ARG_LIST => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TypeArgList {
    pub fn type_args(&self) -> AstChildren<TypeArg> {
        AstChildren::new(&self.syntax)
    }
    pub fn lifetime_args(&self) -> AstChildren<LifetimeArg> {
        AstChildren::new(&self.syntax)
    }
    pub fn assoc_type_args(&self) -> AstChildren<AssocTypeArg> {
        AstChildren::new(&self.syntax)
    }
    pub fn const_arg(&self) -> AstChildren<ConstArg> {
        AstChildren::new(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArg {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for TypeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for TypeArg {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for TypeArg {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl TypeArg {
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssocTypeArg {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for AssocTypeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for AssocTypeArg {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ASSOC_TYPE_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for AssocTypeArg {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ASSOC_TYPE_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl AssocTypeArg {
    pub fn name_ref(&self) -> Option<NameRef> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn type_ref(&self) -> Option<TypeRef> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LifetimeArg {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for LifetimeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for LifetimeArg {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LIFETIME_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for LifetimeArg {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LIFETIME_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl LifetimeArg {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstArg {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for ConstArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for ConstArg {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONST_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for ConstArg {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONST_ARG => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ConstArg {
    pub fn literal(&self) -> Option<Literal> {
        AstChildren::new(&self.syntax).next()
    }
    pub fn block_expr(&self) -> Option<BlockExpr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroItems {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MacroItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MacroItems {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_ITEMS => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MacroItems {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_ITEMS => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl ast::ModuleItemOwner for MacroItems {}
impl ast::FnDefOwner for MacroItems {}
impl MacroItems {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroStmts {
    pub(crate) syntax: SyntaxNode,
}
impl std::fmt::Display for MacroStmts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl AstNode for MacroStmts {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_STMTS => true,
            _ => false,
        }
    }
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        if Self::can_cast(syntax.kind()) {
            Ok(Self { syntax })
        } else {
            Err(syntax)
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl AstElement for MacroStmts {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            MACRO_STMTS => true,
            _ => false,
        }
    }
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        if Self::can_cast_element(syntax.kind()) {
            Ok(Self { syntax: syntax.into_node().unwrap() })
        } else {
            Err(syntax)
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        NodeOrToken::Node(&self.syntax)
    }
    fn into_syntax_element(self) -> SyntaxElement {
        NodeOrToken::Node(self.syntax)
    }
}
impl MacroStmts {
    pub fn statements(&self) -> AstChildren<Stmt> {
        AstChildren::new(&self.syntax)
    }
    pub fn expr(&self) -> Option<Expr> {
        AstChildren::new(&self.syntax).next()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NominalDef {
    StructDef(StructDef),
    EnumDef(EnumDef),
    UnionDef(UnionDef),
}
impl From<StructDef> for NominalDef {
    fn from(node: StructDef) -> NominalDef {
        NominalDef::StructDef(node)
    }
}
impl From<EnumDef> for NominalDef {
    fn from(node: EnumDef) -> NominalDef {
        NominalDef::EnumDef(node)
    }
}
impl From<UnionDef> for NominalDef {
    fn from(node: UnionDef) -> NominalDef {
        NominalDef::UnionDef(node)
    }
}
impl std::fmt::Display for NominalDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NominalDef::StructDef(it) => std::fmt::Display::fmt(it, f),
            NominalDef::EnumDef(it) => std::fmt::Display::fmt(it, f),
            NominalDef::UnionDef(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for NominalDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_DEF | STRUCT_DEF | UNION_DEF => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            STRUCT_DEF => StructDef::cast_or_return(syntax).map(|x| NominalDef::StructDef(x)),
            ENUM_DEF => EnumDef::cast_or_return(syntax).map(|x| NominalDef::EnumDef(x)),
            UNION_DEF => UnionDef::cast_or_return(syntax).map(|x| NominalDef::UnionDef(x)),
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            NominalDef::StructDef(it) => it.syntax(),
            NominalDef::EnumDef(it) => it.syntax(),
            NominalDef::UnionDef(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            NominalDef::StructDef(it) => it.into_syntax(),
            NominalDef::EnumDef(it) => it.into_syntax(),
            NominalDef::UnionDef(it) => it.into_syntax(),
        }
    }
}
impl AstElement for NominalDef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ENUM_DEF | STRUCT_DEF | UNION_DEF => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            STRUCT_DEF => {
                StructDef::cast_or_return_element(syntax).map(|x| NominalDef::StructDef(x))
            }
            ENUM_DEF => EnumDef::cast_or_return_element(syntax).map(|x| NominalDef::EnumDef(x)),
            UNION_DEF => UnionDef::cast_or_return_element(syntax).map(|x| NominalDef::UnionDef(x)),
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            NominalDef::StructDef(it) => it.syntax_element(),
            NominalDef::EnumDef(it) => it.syntax_element(),
            NominalDef::UnionDef(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            NominalDef::StructDef(it) => it.into_syntax_element(),
            NominalDef::EnumDef(it) => it.into_syntax_element(),
            NominalDef::UnionDef(it) => it.into_syntax_element(),
        }
    }
}
impl ast::NameOwner for NominalDef {}
impl ast::TypeParamsOwner for NominalDef {}
impl ast::AttrsOwner for NominalDef {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeRef {
    ParenType(ParenType),
    TupleType(TupleType),
    NeverType(NeverType),
    PathType(PathType),
    PointerType(PointerType),
    ArrayType(ArrayType),
    SliceType(SliceType),
    ReferenceType(ReferenceType),
    PlaceholderType(PlaceholderType),
    FnPointerType(FnPointerType),
    ForType(ForType),
    ImplTraitType(ImplTraitType),
    DynTraitType(DynTraitType),
}
impl From<ParenType> for TypeRef {
    fn from(node: ParenType) -> TypeRef {
        TypeRef::ParenType(node)
    }
}
impl From<TupleType> for TypeRef {
    fn from(node: TupleType) -> TypeRef {
        TypeRef::TupleType(node)
    }
}
impl From<NeverType> for TypeRef {
    fn from(node: NeverType) -> TypeRef {
        TypeRef::NeverType(node)
    }
}
impl From<PathType> for TypeRef {
    fn from(node: PathType) -> TypeRef {
        TypeRef::PathType(node)
    }
}
impl From<PointerType> for TypeRef {
    fn from(node: PointerType) -> TypeRef {
        TypeRef::PointerType(node)
    }
}
impl From<ArrayType> for TypeRef {
    fn from(node: ArrayType) -> TypeRef {
        TypeRef::ArrayType(node)
    }
}
impl From<SliceType> for TypeRef {
    fn from(node: SliceType) -> TypeRef {
        TypeRef::SliceType(node)
    }
}
impl From<ReferenceType> for TypeRef {
    fn from(node: ReferenceType) -> TypeRef {
        TypeRef::ReferenceType(node)
    }
}
impl From<PlaceholderType> for TypeRef {
    fn from(node: PlaceholderType) -> TypeRef {
        TypeRef::PlaceholderType(node)
    }
}
impl From<FnPointerType> for TypeRef {
    fn from(node: FnPointerType) -> TypeRef {
        TypeRef::FnPointerType(node)
    }
}
impl From<ForType> for TypeRef {
    fn from(node: ForType) -> TypeRef {
        TypeRef::ForType(node)
    }
}
impl From<ImplTraitType> for TypeRef {
    fn from(node: ImplTraitType) -> TypeRef {
        TypeRef::ImplTraitType(node)
    }
}
impl From<DynTraitType> for TypeRef {
    fn from(node: DynTraitType) -> TypeRef {
        TypeRef::DynTraitType(node)
    }
}
impl std::fmt::Display for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeRef::ParenType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::TupleType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::NeverType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::PathType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::PointerType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::ArrayType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::SliceType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::ReferenceType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::PlaceholderType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::FnPointerType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::ForType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::ImplTraitType(it) => std::fmt::Display::fmt(it, f),
            TypeRef::DynTraitType(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for TypeRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_TYPE | DYN_TRAIT_TYPE | FN_POINTER_TYPE | FOR_TYPE | IMPL_TRAIT_TYPE
            | NEVER_TYPE | PAREN_TYPE | PATH_TYPE | PLACEHOLDER_TYPE | POINTER_TYPE
            | REFERENCE_TYPE | SLICE_TYPE | TUPLE_TYPE => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            PAREN_TYPE => ParenType::cast_or_return(syntax).map(|x| TypeRef::ParenType(x)),
            TUPLE_TYPE => TupleType::cast_or_return(syntax).map(|x| TypeRef::TupleType(x)),
            NEVER_TYPE => NeverType::cast_or_return(syntax).map(|x| TypeRef::NeverType(x)),
            PATH_TYPE => PathType::cast_or_return(syntax).map(|x| TypeRef::PathType(x)),
            POINTER_TYPE => PointerType::cast_or_return(syntax).map(|x| TypeRef::PointerType(x)),
            ARRAY_TYPE => ArrayType::cast_or_return(syntax).map(|x| TypeRef::ArrayType(x)),
            SLICE_TYPE => SliceType::cast_or_return(syntax).map(|x| TypeRef::SliceType(x)),
            REFERENCE_TYPE => {
                ReferenceType::cast_or_return(syntax).map(|x| TypeRef::ReferenceType(x))
            }
            PLACEHOLDER_TYPE => {
                PlaceholderType::cast_or_return(syntax).map(|x| TypeRef::PlaceholderType(x))
            }
            FN_POINTER_TYPE => {
                FnPointerType::cast_or_return(syntax).map(|x| TypeRef::FnPointerType(x))
            }
            FOR_TYPE => ForType::cast_or_return(syntax).map(|x| TypeRef::ForType(x)),
            IMPL_TRAIT_TYPE => {
                ImplTraitType::cast_or_return(syntax).map(|x| TypeRef::ImplTraitType(x))
            }
            DYN_TRAIT_TYPE => {
                DynTraitType::cast_or_return(syntax).map(|x| TypeRef::DynTraitType(x))
            }
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TypeRef::ParenType(it) => it.syntax(),
            TypeRef::TupleType(it) => it.syntax(),
            TypeRef::NeverType(it) => it.syntax(),
            TypeRef::PathType(it) => it.syntax(),
            TypeRef::PointerType(it) => it.syntax(),
            TypeRef::ArrayType(it) => it.syntax(),
            TypeRef::SliceType(it) => it.syntax(),
            TypeRef::ReferenceType(it) => it.syntax(),
            TypeRef::PlaceholderType(it) => it.syntax(),
            TypeRef::FnPointerType(it) => it.syntax(),
            TypeRef::ForType(it) => it.syntax(),
            TypeRef::ImplTraitType(it) => it.syntax(),
            TypeRef::DynTraitType(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            TypeRef::ParenType(it) => it.into_syntax(),
            TypeRef::TupleType(it) => it.into_syntax(),
            TypeRef::NeverType(it) => it.into_syntax(),
            TypeRef::PathType(it) => it.into_syntax(),
            TypeRef::PointerType(it) => it.into_syntax(),
            TypeRef::ArrayType(it) => it.into_syntax(),
            TypeRef::SliceType(it) => it.into_syntax(),
            TypeRef::ReferenceType(it) => it.into_syntax(),
            TypeRef::PlaceholderType(it) => it.into_syntax(),
            TypeRef::FnPointerType(it) => it.into_syntax(),
            TypeRef::ForType(it) => it.into_syntax(),
            TypeRef::ImplTraitType(it) => it.into_syntax(),
            TypeRef::DynTraitType(it) => it.into_syntax(),
        }
    }
}
impl AstElement for TypeRef {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_TYPE | DYN_TRAIT_TYPE | FN_POINTER_TYPE | FOR_TYPE | IMPL_TRAIT_TYPE
            | NEVER_TYPE | PAREN_TYPE | PATH_TYPE | PLACEHOLDER_TYPE | POINTER_TYPE
            | REFERENCE_TYPE | SLICE_TYPE | TUPLE_TYPE => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            PAREN_TYPE => ParenType::cast_or_return_element(syntax).map(|x| TypeRef::ParenType(x)),
            TUPLE_TYPE => TupleType::cast_or_return_element(syntax).map(|x| TypeRef::TupleType(x)),
            NEVER_TYPE => NeverType::cast_or_return_element(syntax).map(|x| TypeRef::NeverType(x)),
            PATH_TYPE => PathType::cast_or_return_element(syntax).map(|x| TypeRef::PathType(x)),
            POINTER_TYPE => {
                PointerType::cast_or_return_element(syntax).map(|x| TypeRef::PointerType(x))
            }
            ARRAY_TYPE => ArrayType::cast_or_return_element(syntax).map(|x| TypeRef::ArrayType(x)),
            SLICE_TYPE => SliceType::cast_or_return_element(syntax).map(|x| TypeRef::SliceType(x)),
            REFERENCE_TYPE => {
                ReferenceType::cast_or_return_element(syntax).map(|x| TypeRef::ReferenceType(x))
            }
            PLACEHOLDER_TYPE => {
                PlaceholderType::cast_or_return_element(syntax).map(|x| TypeRef::PlaceholderType(x))
            }
            FN_POINTER_TYPE => {
                FnPointerType::cast_or_return_element(syntax).map(|x| TypeRef::FnPointerType(x))
            }
            FOR_TYPE => ForType::cast_or_return_element(syntax).map(|x| TypeRef::ForType(x)),
            IMPL_TRAIT_TYPE => {
                ImplTraitType::cast_or_return_element(syntax).map(|x| TypeRef::ImplTraitType(x))
            }
            DYN_TRAIT_TYPE => {
                DynTraitType::cast_or_return_element(syntax).map(|x| TypeRef::DynTraitType(x))
            }
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            TypeRef::ParenType(it) => it.syntax_element(),
            TypeRef::TupleType(it) => it.syntax_element(),
            TypeRef::NeverType(it) => it.syntax_element(),
            TypeRef::PathType(it) => it.syntax_element(),
            TypeRef::PointerType(it) => it.syntax_element(),
            TypeRef::ArrayType(it) => it.syntax_element(),
            TypeRef::SliceType(it) => it.syntax_element(),
            TypeRef::ReferenceType(it) => it.syntax_element(),
            TypeRef::PlaceholderType(it) => it.syntax_element(),
            TypeRef::FnPointerType(it) => it.syntax_element(),
            TypeRef::ForType(it) => it.syntax_element(),
            TypeRef::ImplTraitType(it) => it.syntax_element(),
            TypeRef::DynTraitType(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            TypeRef::ParenType(it) => it.into_syntax_element(),
            TypeRef::TupleType(it) => it.into_syntax_element(),
            TypeRef::NeverType(it) => it.into_syntax_element(),
            TypeRef::PathType(it) => it.into_syntax_element(),
            TypeRef::PointerType(it) => it.into_syntax_element(),
            TypeRef::ArrayType(it) => it.into_syntax_element(),
            TypeRef::SliceType(it) => it.into_syntax_element(),
            TypeRef::ReferenceType(it) => it.into_syntax_element(),
            TypeRef::PlaceholderType(it) => it.into_syntax_element(),
            TypeRef::FnPointerType(it) => it.into_syntax_element(),
            TypeRef::ForType(it) => it.into_syntax_element(),
            TypeRef::ImplTraitType(it) => it.into_syntax_element(),
            TypeRef::DynTraitType(it) => it.into_syntax_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleItem {
    StructDef(StructDef),
    UnionDef(UnionDef),
    EnumDef(EnumDef),
    FnDef(FnDef),
    TraitDef(TraitDef),
    TypeAliasDef(TypeAliasDef),
    ImplDef(ImplDef),
    UseItem(UseItem),
    ExternCrateItem(ExternCrateItem),
    ConstDef(ConstDef),
    StaticDef(StaticDef),
    Module(Module),
    MacroCall(MacroCall),
}
impl From<StructDef> for ModuleItem {
    fn from(node: StructDef) -> ModuleItem {
        ModuleItem::StructDef(node)
    }
}
impl From<UnionDef> for ModuleItem {
    fn from(node: UnionDef) -> ModuleItem {
        ModuleItem::UnionDef(node)
    }
}
impl From<EnumDef> for ModuleItem {
    fn from(node: EnumDef) -> ModuleItem {
        ModuleItem::EnumDef(node)
    }
}
impl From<FnDef> for ModuleItem {
    fn from(node: FnDef) -> ModuleItem {
        ModuleItem::FnDef(node)
    }
}
impl From<TraitDef> for ModuleItem {
    fn from(node: TraitDef) -> ModuleItem {
        ModuleItem::TraitDef(node)
    }
}
impl From<TypeAliasDef> for ModuleItem {
    fn from(node: TypeAliasDef) -> ModuleItem {
        ModuleItem::TypeAliasDef(node)
    }
}
impl From<ImplDef> for ModuleItem {
    fn from(node: ImplDef) -> ModuleItem {
        ModuleItem::ImplDef(node)
    }
}
impl From<UseItem> for ModuleItem {
    fn from(node: UseItem) -> ModuleItem {
        ModuleItem::UseItem(node)
    }
}
impl From<ExternCrateItem> for ModuleItem {
    fn from(node: ExternCrateItem) -> ModuleItem {
        ModuleItem::ExternCrateItem(node)
    }
}
impl From<ConstDef> for ModuleItem {
    fn from(node: ConstDef) -> ModuleItem {
        ModuleItem::ConstDef(node)
    }
}
impl From<StaticDef> for ModuleItem {
    fn from(node: StaticDef) -> ModuleItem {
        ModuleItem::StaticDef(node)
    }
}
impl From<Module> for ModuleItem {
    fn from(node: Module) -> ModuleItem {
        ModuleItem::Module(node)
    }
}
impl From<MacroCall> for ModuleItem {
    fn from(node: MacroCall) -> ModuleItem {
        ModuleItem::MacroCall(node)
    }
}
impl std::fmt::Display for ModuleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModuleItem::StructDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::UnionDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::EnumDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::FnDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::TraitDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::TypeAliasDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::ImplDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::UseItem(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::ExternCrateItem(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::ConstDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::StaticDef(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::Module(it) => std::fmt::Display::fmt(it, f),
            ModuleItem::MacroCall(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for ModuleItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONST_DEF | ENUM_DEF | EXTERN_CRATE_ITEM | FN_DEF | IMPL_DEF | MACRO_CALL | MODULE
            | STATIC_DEF | STRUCT_DEF | TRAIT_DEF | TYPE_ALIAS_DEF | UNION_DEF | USE_ITEM => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            STRUCT_DEF => StructDef::cast_or_return(syntax).map(|x| ModuleItem::StructDef(x)),
            UNION_DEF => UnionDef::cast_or_return(syntax).map(|x| ModuleItem::UnionDef(x)),
            ENUM_DEF => EnumDef::cast_or_return(syntax).map(|x| ModuleItem::EnumDef(x)),
            FN_DEF => FnDef::cast_or_return(syntax).map(|x| ModuleItem::FnDef(x)),
            TRAIT_DEF => TraitDef::cast_or_return(syntax).map(|x| ModuleItem::TraitDef(x)),
            TYPE_ALIAS_DEF => {
                TypeAliasDef::cast_or_return(syntax).map(|x| ModuleItem::TypeAliasDef(x))
            }
            IMPL_DEF => ImplDef::cast_or_return(syntax).map(|x| ModuleItem::ImplDef(x)),
            USE_ITEM => UseItem::cast_or_return(syntax).map(|x| ModuleItem::UseItem(x)),
            EXTERN_CRATE_ITEM => {
                ExternCrateItem::cast_or_return(syntax).map(|x| ModuleItem::ExternCrateItem(x))
            }
            CONST_DEF => ConstDef::cast_or_return(syntax).map(|x| ModuleItem::ConstDef(x)),
            STATIC_DEF => StaticDef::cast_or_return(syntax).map(|x| ModuleItem::StaticDef(x)),
            MODULE => Module::cast_or_return(syntax).map(|x| ModuleItem::Module(x)),
            MACRO_CALL => MacroCall::cast_or_return(syntax).map(|x| ModuleItem::MacroCall(x)),
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ModuleItem::StructDef(it) => it.syntax(),
            ModuleItem::UnionDef(it) => it.syntax(),
            ModuleItem::EnumDef(it) => it.syntax(),
            ModuleItem::FnDef(it) => it.syntax(),
            ModuleItem::TraitDef(it) => it.syntax(),
            ModuleItem::TypeAliasDef(it) => it.syntax(),
            ModuleItem::ImplDef(it) => it.syntax(),
            ModuleItem::UseItem(it) => it.syntax(),
            ModuleItem::ExternCrateItem(it) => it.syntax(),
            ModuleItem::ConstDef(it) => it.syntax(),
            ModuleItem::StaticDef(it) => it.syntax(),
            ModuleItem::Module(it) => it.syntax(),
            ModuleItem::MacroCall(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            ModuleItem::StructDef(it) => it.into_syntax(),
            ModuleItem::UnionDef(it) => it.into_syntax(),
            ModuleItem::EnumDef(it) => it.into_syntax(),
            ModuleItem::FnDef(it) => it.into_syntax(),
            ModuleItem::TraitDef(it) => it.into_syntax(),
            ModuleItem::TypeAliasDef(it) => it.into_syntax(),
            ModuleItem::ImplDef(it) => it.into_syntax(),
            ModuleItem::UseItem(it) => it.into_syntax(),
            ModuleItem::ExternCrateItem(it) => it.into_syntax(),
            ModuleItem::ConstDef(it) => it.into_syntax(),
            ModuleItem::StaticDef(it) => it.into_syntax(),
            ModuleItem::Module(it) => it.into_syntax(),
            ModuleItem::MacroCall(it) => it.into_syntax(),
        }
    }
}
impl AstElement for ModuleItem {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONST_DEF | ENUM_DEF | EXTERN_CRATE_ITEM | FN_DEF | IMPL_DEF | MACRO_CALL | MODULE
            | STATIC_DEF | STRUCT_DEF | TRAIT_DEF | TYPE_ALIAS_DEF | UNION_DEF | USE_ITEM => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            STRUCT_DEF => {
                StructDef::cast_or_return_element(syntax).map(|x| ModuleItem::StructDef(x))
            }
            UNION_DEF => UnionDef::cast_or_return_element(syntax).map(|x| ModuleItem::UnionDef(x)),
            ENUM_DEF => EnumDef::cast_or_return_element(syntax).map(|x| ModuleItem::EnumDef(x)),
            FN_DEF => FnDef::cast_or_return_element(syntax).map(|x| ModuleItem::FnDef(x)),
            TRAIT_DEF => TraitDef::cast_or_return_element(syntax).map(|x| ModuleItem::TraitDef(x)),
            TYPE_ALIAS_DEF => {
                TypeAliasDef::cast_or_return_element(syntax).map(|x| ModuleItem::TypeAliasDef(x))
            }
            IMPL_DEF => ImplDef::cast_or_return_element(syntax).map(|x| ModuleItem::ImplDef(x)),
            USE_ITEM => UseItem::cast_or_return_element(syntax).map(|x| ModuleItem::UseItem(x)),
            EXTERN_CRATE_ITEM => ExternCrateItem::cast_or_return_element(syntax)
                .map(|x| ModuleItem::ExternCrateItem(x)),
            CONST_DEF => ConstDef::cast_or_return_element(syntax).map(|x| ModuleItem::ConstDef(x)),
            STATIC_DEF => {
                StaticDef::cast_or_return_element(syntax).map(|x| ModuleItem::StaticDef(x))
            }
            MODULE => Module::cast_or_return_element(syntax).map(|x| ModuleItem::Module(x)),
            MACRO_CALL => {
                MacroCall::cast_or_return_element(syntax).map(|x| ModuleItem::MacroCall(x))
            }
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            ModuleItem::StructDef(it) => it.syntax_element(),
            ModuleItem::UnionDef(it) => it.syntax_element(),
            ModuleItem::EnumDef(it) => it.syntax_element(),
            ModuleItem::FnDef(it) => it.syntax_element(),
            ModuleItem::TraitDef(it) => it.syntax_element(),
            ModuleItem::TypeAliasDef(it) => it.syntax_element(),
            ModuleItem::ImplDef(it) => it.syntax_element(),
            ModuleItem::UseItem(it) => it.syntax_element(),
            ModuleItem::ExternCrateItem(it) => it.syntax_element(),
            ModuleItem::ConstDef(it) => it.syntax_element(),
            ModuleItem::StaticDef(it) => it.syntax_element(),
            ModuleItem::Module(it) => it.syntax_element(),
            ModuleItem::MacroCall(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            ModuleItem::StructDef(it) => it.into_syntax_element(),
            ModuleItem::UnionDef(it) => it.into_syntax_element(),
            ModuleItem::EnumDef(it) => it.into_syntax_element(),
            ModuleItem::FnDef(it) => it.into_syntax_element(),
            ModuleItem::TraitDef(it) => it.into_syntax_element(),
            ModuleItem::TypeAliasDef(it) => it.into_syntax_element(),
            ModuleItem::ImplDef(it) => it.into_syntax_element(),
            ModuleItem::UseItem(it) => it.into_syntax_element(),
            ModuleItem::ExternCrateItem(it) => it.into_syntax_element(),
            ModuleItem::ConstDef(it) => it.into_syntax_element(),
            ModuleItem::StaticDef(it) => it.into_syntax_element(),
            ModuleItem::Module(it) => it.into_syntax_element(),
            ModuleItem::MacroCall(it) => it.into_syntax_element(),
        }
    }
}
impl ast::AttrsOwner for ModuleItem {}
impl ast::VisibilityOwner for ModuleItem {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplItem {
    FnDef(FnDef),
    TypeAliasDef(TypeAliasDef),
    ConstDef(ConstDef),
}
impl From<FnDef> for ImplItem {
    fn from(node: FnDef) -> ImplItem {
        ImplItem::FnDef(node)
    }
}
impl From<TypeAliasDef> for ImplItem {
    fn from(node: TypeAliasDef) -> ImplItem {
        ImplItem::TypeAliasDef(node)
    }
}
impl From<ConstDef> for ImplItem {
    fn from(node: ConstDef) -> ImplItem {
        ImplItem::ConstDef(node)
    }
}
impl std::fmt::Display for ImplItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImplItem::FnDef(it) => std::fmt::Display::fmt(it, f),
            ImplItem::TypeAliasDef(it) => std::fmt::Display::fmt(it, f),
            ImplItem::ConstDef(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for ImplItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONST_DEF | FN_DEF | TYPE_ALIAS_DEF => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            FN_DEF => FnDef::cast_or_return(syntax).map(|x| ImplItem::FnDef(x)),
            TYPE_ALIAS_DEF => {
                TypeAliasDef::cast_or_return(syntax).map(|x| ImplItem::TypeAliasDef(x))
            }
            CONST_DEF => ConstDef::cast_or_return(syntax).map(|x| ImplItem::ConstDef(x)),
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ImplItem::FnDef(it) => it.syntax(),
            ImplItem::TypeAliasDef(it) => it.syntax(),
            ImplItem::ConstDef(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            ImplItem::FnDef(it) => it.into_syntax(),
            ImplItem::TypeAliasDef(it) => it.into_syntax(),
            ImplItem::ConstDef(it) => it.into_syntax(),
        }
    }
}
impl AstElement for ImplItem {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            CONST_DEF | FN_DEF | TYPE_ALIAS_DEF => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            FN_DEF => FnDef::cast_or_return_element(syntax).map(|x| ImplItem::FnDef(x)),
            TYPE_ALIAS_DEF => {
                TypeAliasDef::cast_or_return_element(syntax).map(|x| ImplItem::TypeAliasDef(x))
            }
            CONST_DEF => ConstDef::cast_or_return_element(syntax).map(|x| ImplItem::ConstDef(x)),
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            ImplItem::FnDef(it) => it.syntax_element(),
            ImplItem::TypeAliasDef(it) => it.syntax_element(),
            ImplItem::ConstDef(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            ImplItem::FnDef(it) => it.into_syntax_element(),
            ImplItem::TypeAliasDef(it) => it.into_syntax_element(),
            ImplItem::ConstDef(it) => it.into_syntax_element(),
        }
    }
}
impl ast::AttrsOwner for ImplItem {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    TupleExpr(TupleExpr),
    ArrayExpr(ArrayExpr),
    ParenExpr(ParenExpr),
    PathExpr(PathExpr),
    LambdaExpr(LambdaExpr),
    IfExpr(IfExpr),
    LoopExpr(LoopExpr),
    ForExpr(ForExpr),
    WhileExpr(WhileExpr),
    ContinueExpr(ContinueExpr),
    BreakExpr(BreakExpr),
    Label(Label),
    BlockExpr(BlockExpr),
    ReturnExpr(ReturnExpr),
    MatchExpr(MatchExpr),
    RecordLit(RecordLit),
    CallExpr(CallExpr),
    IndexExpr(IndexExpr),
    MethodCallExpr(MethodCallExpr),
    FieldExpr(FieldExpr),
    AwaitExpr(AwaitExpr),
    TryExpr(TryExpr),
    TryBlockExpr(TryBlockExpr),
    CastExpr(CastExpr),
    RefExpr(RefExpr),
    PrefixExpr(PrefixExpr),
    RangeExpr(RangeExpr),
    BinExpr(BinExpr),
    Literal(Literal),
    MacroCall(MacroCall),
    BoxExpr(BoxExpr),
}
impl From<TupleExpr> for Expr {
    fn from(node: TupleExpr) -> Expr {
        Expr::TupleExpr(node)
    }
}
impl From<ArrayExpr> for Expr {
    fn from(node: ArrayExpr) -> Expr {
        Expr::ArrayExpr(node)
    }
}
impl From<ParenExpr> for Expr {
    fn from(node: ParenExpr) -> Expr {
        Expr::ParenExpr(node)
    }
}
impl From<PathExpr> for Expr {
    fn from(node: PathExpr) -> Expr {
        Expr::PathExpr(node)
    }
}
impl From<LambdaExpr> for Expr {
    fn from(node: LambdaExpr) -> Expr {
        Expr::LambdaExpr(node)
    }
}
impl From<IfExpr> for Expr {
    fn from(node: IfExpr) -> Expr {
        Expr::IfExpr(node)
    }
}
impl From<LoopExpr> for Expr {
    fn from(node: LoopExpr) -> Expr {
        Expr::LoopExpr(node)
    }
}
impl From<ForExpr> for Expr {
    fn from(node: ForExpr) -> Expr {
        Expr::ForExpr(node)
    }
}
impl From<WhileExpr> for Expr {
    fn from(node: WhileExpr) -> Expr {
        Expr::WhileExpr(node)
    }
}
impl From<ContinueExpr> for Expr {
    fn from(node: ContinueExpr) -> Expr {
        Expr::ContinueExpr(node)
    }
}
impl From<BreakExpr> for Expr {
    fn from(node: BreakExpr) -> Expr {
        Expr::BreakExpr(node)
    }
}
impl From<Label> for Expr {
    fn from(node: Label) -> Expr {
        Expr::Label(node)
    }
}
impl From<BlockExpr> for Expr {
    fn from(node: BlockExpr) -> Expr {
        Expr::BlockExpr(node)
    }
}
impl From<ReturnExpr> for Expr {
    fn from(node: ReturnExpr) -> Expr {
        Expr::ReturnExpr(node)
    }
}
impl From<MatchExpr> for Expr {
    fn from(node: MatchExpr) -> Expr {
        Expr::MatchExpr(node)
    }
}
impl From<RecordLit> for Expr {
    fn from(node: RecordLit) -> Expr {
        Expr::RecordLit(node)
    }
}
impl From<CallExpr> for Expr {
    fn from(node: CallExpr) -> Expr {
        Expr::CallExpr(node)
    }
}
impl From<IndexExpr> for Expr {
    fn from(node: IndexExpr) -> Expr {
        Expr::IndexExpr(node)
    }
}
impl From<MethodCallExpr> for Expr {
    fn from(node: MethodCallExpr) -> Expr {
        Expr::MethodCallExpr(node)
    }
}
impl From<FieldExpr> for Expr {
    fn from(node: FieldExpr) -> Expr {
        Expr::FieldExpr(node)
    }
}
impl From<AwaitExpr> for Expr {
    fn from(node: AwaitExpr) -> Expr {
        Expr::AwaitExpr(node)
    }
}
impl From<TryExpr> for Expr {
    fn from(node: TryExpr) -> Expr {
        Expr::TryExpr(node)
    }
}
impl From<TryBlockExpr> for Expr {
    fn from(node: TryBlockExpr) -> Expr {
        Expr::TryBlockExpr(node)
    }
}
impl From<CastExpr> for Expr {
    fn from(node: CastExpr) -> Expr {
        Expr::CastExpr(node)
    }
}
impl From<RefExpr> for Expr {
    fn from(node: RefExpr) -> Expr {
        Expr::RefExpr(node)
    }
}
impl From<PrefixExpr> for Expr {
    fn from(node: PrefixExpr) -> Expr {
        Expr::PrefixExpr(node)
    }
}
impl From<RangeExpr> for Expr {
    fn from(node: RangeExpr) -> Expr {
        Expr::RangeExpr(node)
    }
}
impl From<BinExpr> for Expr {
    fn from(node: BinExpr) -> Expr {
        Expr::BinExpr(node)
    }
}
impl From<Literal> for Expr {
    fn from(node: Literal) -> Expr {
        Expr::Literal(node)
    }
}
impl From<MacroCall> for Expr {
    fn from(node: MacroCall) -> Expr {
        Expr::MacroCall(node)
    }
}
impl From<BoxExpr> for Expr {
    fn from(node: BoxExpr) -> Expr {
        Expr::BoxExpr(node)
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::TupleExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::ArrayExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::ParenExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::PathExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::LambdaExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::IfExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::LoopExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::ForExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::WhileExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::ContinueExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::BreakExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::Label(it) => std::fmt::Display::fmt(it, f),
            Expr::BlockExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::ReturnExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::MatchExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::RecordLit(it) => std::fmt::Display::fmt(it, f),
            Expr::CallExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::IndexExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::MethodCallExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::FieldExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::AwaitExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::TryExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::TryBlockExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::CastExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::RefExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::PrefixExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::RangeExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::BinExpr(it) => std::fmt::Display::fmt(it, f),
            Expr::Literal(it) => std::fmt::Display::fmt(it, f),
            Expr::MacroCall(it) => std::fmt::Display::fmt(it, f),
            Expr::BoxExpr(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_EXPR | AWAIT_EXPR | BIN_EXPR | BLOCK_EXPR | BOX_EXPR | BREAK_EXPR | CALL_EXPR
            | CAST_EXPR | CONTINUE_EXPR | FIELD_EXPR | FOR_EXPR | IF_EXPR | INDEX_EXPR | LABEL
            | LAMBDA_EXPR | LITERAL | LOOP_EXPR | MACRO_CALL | MATCH_EXPR | METHOD_CALL_EXPR
            | PAREN_EXPR | PATH_EXPR | PREFIX_EXPR | RANGE_EXPR | RECORD_LIT | REF_EXPR
            | RETURN_EXPR | TRY_BLOCK_EXPR | TRY_EXPR | TUPLE_EXPR | WHILE_EXPR => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            TUPLE_EXPR => TupleExpr::cast_or_return(syntax).map(|x| Expr::TupleExpr(x)),
            ARRAY_EXPR => ArrayExpr::cast_or_return(syntax).map(|x| Expr::ArrayExpr(x)),
            PAREN_EXPR => ParenExpr::cast_or_return(syntax).map(|x| Expr::ParenExpr(x)),
            PATH_EXPR => PathExpr::cast_or_return(syntax).map(|x| Expr::PathExpr(x)),
            LAMBDA_EXPR => LambdaExpr::cast_or_return(syntax).map(|x| Expr::LambdaExpr(x)),
            IF_EXPR => IfExpr::cast_or_return(syntax).map(|x| Expr::IfExpr(x)),
            LOOP_EXPR => LoopExpr::cast_or_return(syntax).map(|x| Expr::LoopExpr(x)),
            FOR_EXPR => ForExpr::cast_or_return(syntax).map(|x| Expr::ForExpr(x)),
            WHILE_EXPR => WhileExpr::cast_or_return(syntax).map(|x| Expr::WhileExpr(x)),
            CONTINUE_EXPR => ContinueExpr::cast_or_return(syntax).map(|x| Expr::ContinueExpr(x)),
            BREAK_EXPR => BreakExpr::cast_or_return(syntax).map(|x| Expr::BreakExpr(x)),
            LABEL => Label::cast_or_return(syntax).map(|x| Expr::Label(x)),
            BLOCK_EXPR => BlockExpr::cast_or_return(syntax).map(|x| Expr::BlockExpr(x)),
            RETURN_EXPR => ReturnExpr::cast_or_return(syntax).map(|x| Expr::ReturnExpr(x)),
            MATCH_EXPR => MatchExpr::cast_or_return(syntax).map(|x| Expr::MatchExpr(x)),
            RECORD_LIT => RecordLit::cast_or_return(syntax).map(|x| Expr::RecordLit(x)),
            CALL_EXPR => CallExpr::cast_or_return(syntax).map(|x| Expr::CallExpr(x)),
            INDEX_EXPR => IndexExpr::cast_or_return(syntax).map(|x| Expr::IndexExpr(x)),
            METHOD_CALL_EXPR => {
                MethodCallExpr::cast_or_return(syntax).map(|x| Expr::MethodCallExpr(x))
            }
            FIELD_EXPR => FieldExpr::cast_or_return(syntax).map(|x| Expr::FieldExpr(x)),
            AWAIT_EXPR => AwaitExpr::cast_or_return(syntax).map(|x| Expr::AwaitExpr(x)),
            TRY_EXPR => TryExpr::cast_or_return(syntax).map(|x| Expr::TryExpr(x)),
            TRY_BLOCK_EXPR => TryBlockExpr::cast_or_return(syntax).map(|x| Expr::TryBlockExpr(x)),
            CAST_EXPR => CastExpr::cast_or_return(syntax).map(|x| Expr::CastExpr(x)),
            REF_EXPR => RefExpr::cast_or_return(syntax).map(|x| Expr::RefExpr(x)),
            PREFIX_EXPR => PrefixExpr::cast_or_return(syntax).map(|x| Expr::PrefixExpr(x)),
            RANGE_EXPR => RangeExpr::cast_or_return(syntax).map(|x| Expr::RangeExpr(x)),
            BIN_EXPR => BinExpr::cast_or_return(syntax).map(|x| Expr::BinExpr(x)),
            LITERAL => Literal::cast_or_return(syntax).map(|x| Expr::Literal(x)),
            MACRO_CALL => MacroCall::cast_or_return(syntax).map(|x| Expr::MacroCall(x)),
            BOX_EXPR => BoxExpr::cast_or_return(syntax).map(|x| Expr::BoxExpr(x)),
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::TupleExpr(it) => it.syntax(),
            Expr::ArrayExpr(it) => it.syntax(),
            Expr::ParenExpr(it) => it.syntax(),
            Expr::PathExpr(it) => it.syntax(),
            Expr::LambdaExpr(it) => it.syntax(),
            Expr::IfExpr(it) => it.syntax(),
            Expr::LoopExpr(it) => it.syntax(),
            Expr::ForExpr(it) => it.syntax(),
            Expr::WhileExpr(it) => it.syntax(),
            Expr::ContinueExpr(it) => it.syntax(),
            Expr::BreakExpr(it) => it.syntax(),
            Expr::Label(it) => it.syntax(),
            Expr::BlockExpr(it) => it.syntax(),
            Expr::ReturnExpr(it) => it.syntax(),
            Expr::MatchExpr(it) => it.syntax(),
            Expr::RecordLit(it) => it.syntax(),
            Expr::CallExpr(it) => it.syntax(),
            Expr::IndexExpr(it) => it.syntax(),
            Expr::MethodCallExpr(it) => it.syntax(),
            Expr::FieldExpr(it) => it.syntax(),
            Expr::AwaitExpr(it) => it.syntax(),
            Expr::TryExpr(it) => it.syntax(),
            Expr::TryBlockExpr(it) => it.syntax(),
            Expr::CastExpr(it) => it.syntax(),
            Expr::RefExpr(it) => it.syntax(),
            Expr::PrefixExpr(it) => it.syntax(),
            Expr::RangeExpr(it) => it.syntax(),
            Expr::BinExpr(it) => it.syntax(),
            Expr::Literal(it) => it.syntax(),
            Expr::MacroCall(it) => it.syntax(),
            Expr::BoxExpr(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Expr::TupleExpr(it) => it.into_syntax(),
            Expr::ArrayExpr(it) => it.into_syntax(),
            Expr::ParenExpr(it) => it.into_syntax(),
            Expr::PathExpr(it) => it.into_syntax(),
            Expr::LambdaExpr(it) => it.into_syntax(),
            Expr::IfExpr(it) => it.into_syntax(),
            Expr::LoopExpr(it) => it.into_syntax(),
            Expr::ForExpr(it) => it.into_syntax(),
            Expr::WhileExpr(it) => it.into_syntax(),
            Expr::ContinueExpr(it) => it.into_syntax(),
            Expr::BreakExpr(it) => it.into_syntax(),
            Expr::Label(it) => it.into_syntax(),
            Expr::BlockExpr(it) => it.into_syntax(),
            Expr::ReturnExpr(it) => it.into_syntax(),
            Expr::MatchExpr(it) => it.into_syntax(),
            Expr::RecordLit(it) => it.into_syntax(),
            Expr::CallExpr(it) => it.into_syntax(),
            Expr::IndexExpr(it) => it.into_syntax(),
            Expr::MethodCallExpr(it) => it.into_syntax(),
            Expr::FieldExpr(it) => it.into_syntax(),
            Expr::AwaitExpr(it) => it.into_syntax(),
            Expr::TryExpr(it) => it.into_syntax(),
            Expr::TryBlockExpr(it) => it.into_syntax(),
            Expr::CastExpr(it) => it.into_syntax(),
            Expr::RefExpr(it) => it.into_syntax(),
            Expr::PrefixExpr(it) => it.into_syntax(),
            Expr::RangeExpr(it) => it.into_syntax(),
            Expr::BinExpr(it) => it.into_syntax(),
            Expr::Literal(it) => it.into_syntax(),
            Expr::MacroCall(it) => it.into_syntax(),
            Expr::BoxExpr(it) => it.into_syntax(),
        }
    }
}
impl AstElement for Expr {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            ARRAY_EXPR | AWAIT_EXPR | BIN_EXPR | BLOCK_EXPR | BOX_EXPR | BREAK_EXPR | CALL_EXPR
            | CAST_EXPR | CONTINUE_EXPR | FIELD_EXPR | FOR_EXPR | IF_EXPR | INDEX_EXPR | LABEL
            | LAMBDA_EXPR | LITERAL | LOOP_EXPR | MACRO_CALL | MATCH_EXPR | METHOD_CALL_EXPR
            | PAREN_EXPR | PATH_EXPR | PREFIX_EXPR | RANGE_EXPR | RECORD_LIT | REF_EXPR
            | RETURN_EXPR | TRY_BLOCK_EXPR | TRY_EXPR | TUPLE_EXPR | WHILE_EXPR => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            TUPLE_EXPR => TupleExpr::cast_or_return_element(syntax).map(|x| Expr::TupleExpr(x)),
            ARRAY_EXPR => ArrayExpr::cast_or_return_element(syntax).map(|x| Expr::ArrayExpr(x)),
            PAREN_EXPR => ParenExpr::cast_or_return_element(syntax).map(|x| Expr::ParenExpr(x)),
            PATH_EXPR => PathExpr::cast_or_return_element(syntax).map(|x| Expr::PathExpr(x)),
            LAMBDA_EXPR => LambdaExpr::cast_or_return_element(syntax).map(|x| Expr::LambdaExpr(x)),
            IF_EXPR => IfExpr::cast_or_return_element(syntax).map(|x| Expr::IfExpr(x)),
            LOOP_EXPR => LoopExpr::cast_or_return_element(syntax).map(|x| Expr::LoopExpr(x)),
            FOR_EXPR => ForExpr::cast_or_return_element(syntax).map(|x| Expr::ForExpr(x)),
            WHILE_EXPR => WhileExpr::cast_or_return_element(syntax).map(|x| Expr::WhileExpr(x)),
            CONTINUE_EXPR => {
                ContinueExpr::cast_or_return_element(syntax).map(|x| Expr::ContinueExpr(x))
            }
            BREAK_EXPR => BreakExpr::cast_or_return_element(syntax).map(|x| Expr::BreakExpr(x)),
            LABEL => Label::cast_or_return_element(syntax).map(|x| Expr::Label(x)),
            BLOCK_EXPR => BlockExpr::cast_or_return_element(syntax).map(|x| Expr::BlockExpr(x)),
            RETURN_EXPR => ReturnExpr::cast_or_return_element(syntax).map(|x| Expr::ReturnExpr(x)),
            MATCH_EXPR => MatchExpr::cast_or_return_element(syntax).map(|x| Expr::MatchExpr(x)),
            RECORD_LIT => RecordLit::cast_or_return_element(syntax).map(|x| Expr::RecordLit(x)),
            CALL_EXPR => CallExpr::cast_or_return_element(syntax).map(|x| Expr::CallExpr(x)),
            INDEX_EXPR => IndexExpr::cast_or_return_element(syntax).map(|x| Expr::IndexExpr(x)),
            METHOD_CALL_EXPR => {
                MethodCallExpr::cast_or_return_element(syntax).map(|x| Expr::MethodCallExpr(x))
            }
            FIELD_EXPR => FieldExpr::cast_or_return_element(syntax).map(|x| Expr::FieldExpr(x)),
            AWAIT_EXPR => AwaitExpr::cast_or_return_element(syntax).map(|x| Expr::AwaitExpr(x)),
            TRY_EXPR => TryExpr::cast_or_return_element(syntax).map(|x| Expr::TryExpr(x)),
            TRY_BLOCK_EXPR => {
                TryBlockExpr::cast_or_return_element(syntax).map(|x| Expr::TryBlockExpr(x))
            }
            CAST_EXPR => CastExpr::cast_or_return_element(syntax).map(|x| Expr::CastExpr(x)),
            REF_EXPR => RefExpr::cast_or_return_element(syntax).map(|x| Expr::RefExpr(x)),
            PREFIX_EXPR => PrefixExpr::cast_or_return_element(syntax).map(|x| Expr::PrefixExpr(x)),
            RANGE_EXPR => RangeExpr::cast_or_return_element(syntax).map(|x| Expr::RangeExpr(x)),
            BIN_EXPR => BinExpr::cast_or_return_element(syntax).map(|x| Expr::BinExpr(x)),
            LITERAL => Literal::cast_or_return_element(syntax).map(|x| Expr::Literal(x)),
            MACRO_CALL => MacroCall::cast_or_return_element(syntax).map(|x| Expr::MacroCall(x)),
            BOX_EXPR => BoxExpr::cast_or_return_element(syntax).map(|x| Expr::BoxExpr(x)),
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            Expr::TupleExpr(it) => it.syntax_element(),
            Expr::ArrayExpr(it) => it.syntax_element(),
            Expr::ParenExpr(it) => it.syntax_element(),
            Expr::PathExpr(it) => it.syntax_element(),
            Expr::LambdaExpr(it) => it.syntax_element(),
            Expr::IfExpr(it) => it.syntax_element(),
            Expr::LoopExpr(it) => it.syntax_element(),
            Expr::ForExpr(it) => it.syntax_element(),
            Expr::WhileExpr(it) => it.syntax_element(),
            Expr::ContinueExpr(it) => it.syntax_element(),
            Expr::BreakExpr(it) => it.syntax_element(),
            Expr::Label(it) => it.syntax_element(),
            Expr::BlockExpr(it) => it.syntax_element(),
            Expr::ReturnExpr(it) => it.syntax_element(),
            Expr::MatchExpr(it) => it.syntax_element(),
            Expr::RecordLit(it) => it.syntax_element(),
            Expr::CallExpr(it) => it.syntax_element(),
            Expr::IndexExpr(it) => it.syntax_element(),
            Expr::MethodCallExpr(it) => it.syntax_element(),
            Expr::FieldExpr(it) => it.syntax_element(),
            Expr::AwaitExpr(it) => it.syntax_element(),
            Expr::TryExpr(it) => it.syntax_element(),
            Expr::TryBlockExpr(it) => it.syntax_element(),
            Expr::CastExpr(it) => it.syntax_element(),
            Expr::RefExpr(it) => it.syntax_element(),
            Expr::PrefixExpr(it) => it.syntax_element(),
            Expr::RangeExpr(it) => it.syntax_element(),
            Expr::BinExpr(it) => it.syntax_element(),
            Expr::Literal(it) => it.syntax_element(),
            Expr::MacroCall(it) => it.syntax_element(),
            Expr::BoxExpr(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            Expr::TupleExpr(it) => it.into_syntax_element(),
            Expr::ArrayExpr(it) => it.into_syntax_element(),
            Expr::ParenExpr(it) => it.into_syntax_element(),
            Expr::PathExpr(it) => it.into_syntax_element(),
            Expr::LambdaExpr(it) => it.into_syntax_element(),
            Expr::IfExpr(it) => it.into_syntax_element(),
            Expr::LoopExpr(it) => it.into_syntax_element(),
            Expr::ForExpr(it) => it.into_syntax_element(),
            Expr::WhileExpr(it) => it.into_syntax_element(),
            Expr::ContinueExpr(it) => it.into_syntax_element(),
            Expr::BreakExpr(it) => it.into_syntax_element(),
            Expr::Label(it) => it.into_syntax_element(),
            Expr::BlockExpr(it) => it.into_syntax_element(),
            Expr::ReturnExpr(it) => it.into_syntax_element(),
            Expr::MatchExpr(it) => it.into_syntax_element(),
            Expr::RecordLit(it) => it.into_syntax_element(),
            Expr::CallExpr(it) => it.into_syntax_element(),
            Expr::IndexExpr(it) => it.into_syntax_element(),
            Expr::MethodCallExpr(it) => it.into_syntax_element(),
            Expr::FieldExpr(it) => it.into_syntax_element(),
            Expr::AwaitExpr(it) => it.into_syntax_element(),
            Expr::TryExpr(it) => it.into_syntax_element(),
            Expr::TryBlockExpr(it) => it.into_syntax_element(),
            Expr::CastExpr(it) => it.into_syntax_element(),
            Expr::RefExpr(it) => it.into_syntax_element(),
            Expr::PrefixExpr(it) => it.into_syntax_element(),
            Expr::RangeExpr(it) => it.into_syntax_element(),
            Expr::BinExpr(it) => it.into_syntax_element(),
            Expr::Literal(it) => it.into_syntax_element(),
            Expr::MacroCall(it) => it.into_syntax_element(),
            Expr::BoxExpr(it) => it.into_syntax_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pat {
    OrPat(OrPat),
    ParenPat(ParenPat),
    RefPat(RefPat),
    BoxPat(BoxPat),
    BindPat(BindPat),
    PlaceholderPat(PlaceholderPat),
    DotDotPat(DotDotPat),
    PathPat(PathPat),
    RecordPat(RecordPat),
    TupleStructPat(TupleStructPat),
    TuplePat(TuplePat),
    SlicePat(SlicePat),
    RangePat(RangePat),
    LiteralPat(LiteralPat),
}
impl From<OrPat> for Pat {
    fn from(node: OrPat) -> Pat {
        Pat::OrPat(node)
    }
}
impl From<ParenPat> for Pat {
    fn from(node: ParenPat) -> Pat {
        Pat::ParenPat(node)
    }
}
impl From<RefPat> for Pat {
    fn from(node: RefPat) -> Pat {
        Pat::RefPat(node)
    }
}
impl From<BoxPat> for Pat {
    fn from(node: BoxPat) -> Pat {
        Pat::BoxPat(node)
    }
}
impl From<BindPat> for Pat {
    fn from(node: BindPat) -> Pat {
        Pat::BindPat(node)
    }
}
impl From<PlaceholderPat> for Pat {
    fn from(node: PlaceholderPat) -> Pat {
        Pat::PlaceholderPat(node)
    }
}
impl From<DotDotPat> for Pat {
    fn from(node: DotDotPat) -> Pat {
        Pat::DotDotPat(node)
    }
}
impl From<PathPat> for Pat {
    fn from(node: PathPat) -> Pat {
        Pat::PathPat(node)
    }
}
impl From<RecordPat> for Pat {
    fn from(node: RecordPat) -> Pat {
        Pat::RecordPat(node)
    }
}
impl From<TupleStructPat> for Pat {
    fn from(node: TupleStructPat) -> Pat {
        Pat::TupleStructPat(node)
    }
}
impl From<TuplePat> for Pat {
    fn from(node: TuplePat) -> Pat {
        Pat::TuplePat(node)
    }
}
impl From<SlicePat> for Pat {
    fn from(node: SlicePat) -> Pat {
        Pat::SlicePat(node)
    }
}
impl From<RangePat> for Pat {
    fn from(node: RangePat) -> Pat {
        Pat::RangePat(node)
    }
}
impl From<LiteralPat> for Pat {
    fn from(node: LiteralPat) -> Pat {
        Pat::LiteralPat(node)
    }
}
impl std::fmt::Display for Pat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Pat::OrPat(it) => std::fmt::Display::fmt(it, f),
            Pat::ParenPat(it) => std::fmt::Display::fmt(it, f),
            Pat::RefPat(it) => std::fmt::Display::fmt(it, f),
            Pat::BoxPat(it) => std::fmt::Display::fmt(it, f),
            Pat::BindPat(it) => std::fmt::Display::fmt(it, f),
            Pat::PlaceholderPat(it) => std::fmt::Display::fmt(it, f),
            Pat::DotDotPat(it) => std::fmt::Display::fmt(it, f),
            Pat::PathPat(it) => std::fmt::Display::fmt(it, f),
            Pat::RecordPat(it) => std::fmt::Display::fmt(it, f),
            Pat::TupleStructPat(it) => std::fmt::Display::fmt(it, f),
            Pat::TuplePat(it) => std::fmt::Display::fmt(it, f),
            Pat::SlicePat(it) => std::fmt::Display::fmt(it, f),
            Pat::RangePat(it) => std::fmt::Display::fmt(it, f),
            Pat::LiteralPat(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for Pat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BIND_PAT | BOX_PAT | DOT_DOT_PAT | LITERAL_PAT | OR_PAT | PAREN_PAT | PATH_PAT
            | PLACEHOLDER_PAT | RANGE_PAT | RECORD_PAT | REF_PAT | SLICE_PAT | TUPLE_PAT
            | TUPLE_STRUCT_PAT => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            OR_PAT => OrPat::cast_or_return(syntax).map(|x| Pat::OrPat(x)),
            PAREN_PAT => ParenPat::cast_or_return(syntax).map(|x| Pat::ParenPat(x)),
            REF_PAT => RefPat::cast_or_return(syntax).map(|x| Pat::RefPat(x)),
            BOX_PAT => BoxPat::cast_or_return(syntax).map(|x| Pat::BoxPat(x)),
            BIND_PAT => BindPat::cast_or_return(syntax).map(|x| Pat::BindPat(x)),
            PLACEHOLDER_PAT => {
                PlaceholderPat::cast_or_return(syntax).map(|x| Pat::PlaceholderPat(x))
            }
            DOT_DOT_PAT => DotDotPat::cast_or_return(syntax).map(|x| Pat::DotDotPat(x)),
            PATH_PAT => PathPat::cast_or_return(syntax).map(|x| Pat::PathPat(x)),
            RECORD_PAT => RecordPat::cast_or_return(syntax).map(|x| Pat::RecordPat(x)),
            TUPLE_STRUCT_PAT => {
                TupleStructPat::cast_or_return(syntax).map(|x| Pat::TupleStructPat(x))
            }
            TUPLE_PAT => TuplePat::cast_or_return(syntax).map(|x| Pat::TuplePat(x)),
            SLICE_PAT => SlicePat::cast_or_return(syntax).map(|x| Pat::SlicePat(x)),
            RANGE_PAT => RangePat::cast_or_return(syntax).map(|x| Pat::RangePat(x)),
            LITERAL_PAT => LiteralPat::cast_or_return(syntax).map(|x| Pat::LiteralPat(x)),
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Pat::OrPat(it) => it.syntax(),
            Pat::ParenPat(it) => it.syntax(),
            Pat::RefPat(it) => it.syntax(),
            Pat::BoxPat(it) => it.syntax(),
            Pat::BindPat(it) => it.syntax(),
            Pat::PlaceholderPat(it) => it.syntax(),
            Pat::DotDotPat(it) => it.syntax(),
            Pat::PathPat(it) => it.syntax(),
            Pat::RecordPat(it) => it.syntax(),
            Pat::TupleStructPat(it) => it.syntax(),
            Pat::TuplePat(it) => it.syntax(),
            Pat::SlicePat(it) => it.syntax(),
            Pat::RangePat(it) => it.syntax(),
            Pat::LiteralPat(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Pat::OrPat(it) => it.into_syntax(),
            Pat::ParenPat(it) => it.into_syntax(),
            Pat::RefPat(it) => it.into_syntax(),
            Pat::BoxPat(it) => it.into_syntax(),
            Pat::BindPat(it) => it.into_syntax(),
            Pat::PlaceholderPat(it) => it.into_syntax(),
            Pat::DotDotPat(it) => it.into_syntax(),
            Pat::PathPat(it) => it.into_syntax(),
            Pat::RecordPat(it) => it.into_syntax(),
            Pat::TupleStructPat(it) => it.into_syntax(),
            Pat::TuplePat(it) => it.into_syntax(),
            Pat::SlicePat(it) => it.into_syntax(),
            Pat::RangePat(it) => it.into_syntax(),
            Pat::LiteralPat(it) => it.into_syntax(),
        }
    }
}
impl AstElement for Pat {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            BIND_PAT | BOX_PAT | DOT_DOT_PAT | LITERAL_PAT | OR_PAT | PAREN_PAT | PATH_PAT
            | PLACEHOLDER_PAT | RANGE_PAT | RECORD_PAT | REF_PAT | SLICE_PAT | TUPLE_PAT
            | TUPLE_STRUCT_PAT => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            OR_PAT => OrPat::cast_or_return_element(syntax).map(|x| Pat::OrPat(x)),
            PAREN_PAT => ParenPat::cast_or_return_element(syntax).map(|x| Pat::ParenPat(x)),
            REF_PAT => RefPat::cast_or_return_element(syntax).map(|x| Pat::RefPat(x)),
            BOX_PAT => BoxPat::cast_or_return_element(syntax).map(|x| Pat::BoxPat(x)),
            BIND_PAT => BindPat::cast_or_return_element(syntax).map(|x| Pat::BindPat(x)),
            PLACEHOLDER_PAT => {
                PlaceholderPat::cast_or_return_element(syntax).map(|x| Pat::PlaceholderPat(x))
            }
            DOT_DOT_PAT => DotDotPat::cast_or_return_element(syntax).map(|x| Pat::DotDotPat(x)),
            PATH_PAT => PathPat::cast_or_return_element(syntax).map(|x| Pat::PathPat(x)),
            RECORD_PAT => RecordPat::cast_or_return_element(syntax).map(|x| Pat::RecordPat(x)),
            TUPLE_STRUCT_PAT => {
                TupleStructPat::cast_or_return_element(syntax).map(|x| Pat::TupleStructPat(x))
            }
            TUPLE_PAT => TuplePat::cast_or_return_element(syntax).map(|x| Pat::TuplePat(x)),
            SLICE_PAT => SlicePat::cast_or_return_element(syntax).map(|x| Pat::SlicePat(x)),
            RANGE_PAT => RangePat::cast_or_return_element(syntax).map(|x| Pat::RangePat(x)),
            LITERAL_PAT => LiteralPat::cast_or_return_element(syntax).map(|x| Pat::LiteralPat(x)),
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            Pat::OrPat(it) => it.syntax_element(),
            Pat::ParenPat(it) => it.syntax_element(),
            Pat::RefPat(it) => it.syntax_element(),
            Pat::BoxPat(it) => it.syntax_element(),
            Pat::BindPat(it) => it.syntax_element(),
            Pat::PlaceholderPat(it) => it.syntax_element(),
            Pat::DotDotPat(it) => it.syntax_element(),
            Pat::PathPat(it) => it.syntax_element(),
            Pat::RecordPat(it) => it.syntax_element(),
            Pat::TupleStructPat(it) => it.syntax_element(),
            Pat::TuplePat(it) => it.syntax_element(),
            Pat::SlicePat(it) => it.syntax_element(),
            Pat::RangePat(it) => it.syntax_element(),
            Pat::LiteralPat(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            Pat::OrPat(it) => it.into_syntax_element(),
            Pat::ParenPat(it) => it.into_syntax_element(),
            Pat::RefPat(it) => it.into_syntax_element(),
            Pat::BoxPat(it) => it.into_syntax_element(),
            Pat::BindPat(it) => it.into_syntax_element(),
            Pat::PlaceholderPat(it) => it.into_syntax_element(),
            Pat::DotDotPat(it) => it.into_syntax_element(),
            Pat::PathPat(it) => it.into_syntax_element(),
            Pat::RecordPat(it) => it.into_syntax_element(),
            Pat::TupleStructPat(it) => it.into_syntax_element(),
            Pat::TuplePat(it) => it.into_syntax_element(),
            Pat::SlicePat(it) => it.into_syntax_element(),
            Pat::RangePat(it) => it.into_syntax_element(),
            Pat::LiteralPat(it) => it.into_syntax_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttrInput {
    Literal(Literal),
    TokenTree(TokenTree),
}
impl From<Literal> for AttrInput {
    fn from(node: Literal) -> AttrInput {
        AttrInput::Literal(node)
    }
}
impl From<TokenTree> for AttrInput {
    fn from(node: TokenTree) -> AttrInput {
        AttrInput::TokenTree(node)
    }
}
impl std::fmt::Display for AttrInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AttrInput::Literal(it) => std::fmt::Display::fmt(it, f),
            AttrInput::TokenTree(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for AttrInput {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL | TOKEN_TREE => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            LITERAL => Literal::cast_or_return(syntax).map(|x| AttrInput::Literal(x)),
            TOKEN_TREE => TokenTree::cast_or_return(syntax).map(|x| AttrInput::TokenTree(x)),
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AttrInput::Literal(it) => it.syntax(),
            AttrInput::TokenTree(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AttrInput::Literal(it) => it.into_syntax(),
            AttrInput::TokenTree(it) => it.into_syntax(),
        }
    }
}
impl AstElement for AttrInput {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL | TOKEN_TREE => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            LITERAL => Literal::cast_or_return_element(syntax).map(|x| AttrInput::Literal(x)),
            TOKEN_TREE => {
                TokenTree::cast_or_return_element(syntax).map(|x| AttrInput::TokenTree(x))
            }
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            AttrInput::Literal(it) => it.syntax_element(),
            AttrInput::TokenTree(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            AttrInput::Literal(it) => it.into_syntax_element(),
            AttrInput::TokenTree(it) => it.into_syntax_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    ExprStmt(ExprStmt),
    LetStmt(LetStmt),
}
impl From<ExprStmt> for Stmt {
    fn from(node: ExprStmt) -> Stmt {
        Stmt::ExprStmt(node)
    }
}
impl From<LetStmt> for Stmt {
    fn from(node: LetStmt) -> Stmt {
        Stmt::LetStmt(node)
    }
}
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stmt::ExprStmt(it) => std::fmt::Display::fmt(it, f),
            Stmt::LetStmt(it) => std::fmt::Display::fmt(it, f),
        }
    }
}
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXPR_STMT | LET_STMT => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return(syntax: SyntaxNode) -> Result<Self, SyntaxNode> {
        match syntax.kind() {
            EXPR_STMT => ExprStmt::cast_or_return(syntax).map(|x| Stmt::ExprStmt(x)),
            LET_STMT => LetStmt::cast_or_return(syntax).map(|x| Stmt::LetStmt(x)),
            _ => Err(syntax),
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::ExprStmt(it) => it.syntax(),
            Stmt::LetStmt(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Stmt::ExprStmt(it) => it.into_syntax(),
            Stmt::LetStmt(it) => it.into_syntax(),
        }
    }
}
impl AstElement for Stmt {
    fn can_cast_element(kind: SyntaxKind) -> bool {
        match kind {
            EXPR_STMT | LET_STMT => true,
            _ => false,
        }
    }
    #[allow(unreachable_patterns)]
    fn cast_or_return_element(syntax: SyntaxElement) -> Result<Self, SyntaxElement> {
        match syntax.kind() {
            EXPR_STMT => ExprStmt::cast_or_return_element(syntax).map(|x| Stmt::ExprStmt(x)),
            LET_STMT => LetStmt::cast_or_return_element(syntax).map(|x| Stmt::LetStmt(x)),
            _ => Err(syntax),
        }
    }
    fn syntax_element(&self) -> NodeOrToken<&SyntaxNode, &SyntaxToken> {
        match self {
            Stmt::ExprStmt(it) => it.syntax_element(),
            Stmt::LetStmt(it) => it.syntax_element(),
        }
    }
    fn into_syntax_element(self) -> SyntaxElement {
        match self {
            Stmt::ExprStmt(it) => it.into_syntax_element(),
            Stmt::LetStmt(it) => it.into_syntax_element(),
        }
    }
}
