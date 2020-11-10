//! There are many AstNodes, but only a few tokens, so we hand-write them here.

use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
};

use rustc_lexer::unescape::{unescape_literal, Mode};

use crate::{
    ast::{self, AstToken},
    TextRange, TextSize,
};

impl ast::Comment {
    pub fn kind(&self) -> CommentKind {
        kind_by_prefix(self.text())
    }

    pub fn prefix(&self) -> &'static str {
        for (prefix, k) in COMMENT_PREFIX_TO_KIND.iter() {
            if *k == self.kind() && self.text().starts_with(prefix) {
                return prefix;
            }
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommentKind {
    pub shape: CommentShape,
    pub doc: Option<CommentPlacement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentShape {
    Line,
    Block,
}

impl CommentShape {
    pub fn is_line(self) -> bool {
        self == CommentShape::Line
    }

    pub fn is_block(self) -> bool {
        self == CommentShape::Block
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentPlacement {
    Inner,
    Outer,
}

const COMMENT_PREFIX_TO_KIND: &[(&str, CommentKind)] = {
    use {CommentPlacement::*, CommentShape::*};
    &[
        ("////", CommentKind { shape: Line, doc: None }),
        ("///", CommentKind { shape: Line, doc: Some(Outer) }),
        ("//!", CommentKind { shape: Line, doc: Some(Inner) }),
        ("/**", CommentKind { shape: Block, doc: Some(Outer) }),
        ("/*!", CommentKind { shape: Block, doc: Some(Inner) }),
        ("//", CommentKind { shape: Line, doc: None }),
        ("/*", CommentKind { shape: Block, doc: None }),
    ]
};

fn kind_by_prefix(text: &str) -> CommentKind {
    if text == "/**/" {
        return CommentKind { shape: CommentShape::Block, doc: None };
    }
    for (prefix, kind) in COMMENT_PREFIX_TO_KIND.iter() {
        if text.starts_with(prefix) {
            return *kind;
        }
    }
    panic!("bad comment text: {:?}", text)
}

impl ast::Whitespace {
    pub fn spans_multiple_lines(&self) -> bool {
        let text = self.text();
        text.find('\n').map_or(false, |idx| text[idx + 1..].contains('\n'))
    }
}

pub struct QuoteOffsets {
    pub quotes: (TextRange, TextRange),
    pub contents: TextRange,
}

impl QuoteOffsets {
    fn new(literal: &str) -> Option<QuoteOffsets> {
        let left_quote = literal.find('"')?;
        let right_quote = literal.rfind('"')?;
        if left_quote == right_quote {
            // `literal` only contains one quote
            return None;
        }

        let start = TextSize::from(0);
        let left_quote = TextSize::try_from(left_quote).unwrap() + TextSize::of('"');
        let right_quote = TextSize::try_from(right_quote).unwrap();
        let end = TextSize::of(literal);

        let res = QuoteOffsets {
            quotes: (TextRange::new(start, left_quote), TextRange::new(right_quote, end)),
            contents: TextRange::new(left_quote, right_quote),
        };
        Some(res)
    }
}

impl ast::String {
    pub fn is_raw(&self) -> bool {
        self.text().starts_with('r')
    }
    pub fn map_range_up(&self, range: TextRange) -> Option<TextRange> {
        let contents_range = self.text_range_between_quotes()?;
        assert!(TextRange::up_to(contents_range.len()).contains_range(range));
        Some(range + contents_range.start())
    }

    pub fn value(&self) -> Option<Cow<'_, str>> {
        if self.is_raw() {
            let text = self.text().as_str();
            let text =
                &text[self.text_range_between_quotes()? - self.syntax().text_range().start()];
            return Some(Cow::Borrowed(text));
        }

        let text = self.text().as_str();
        let text = &text[self.text_range_between_quotes()? - self.syntax().text_range().start()];

        let mut buf = String::with_capacity(text.len());
        let mut has_error = false;
        unescape_literal(text, Mode::Str, &mut |_, unescaped_char| match unescaped_char {
            Ok(c) => buf.push(c),
            Err(_) => has_error = true,
        });

        if has_error {
            return None;
        }
        // FIXME: don't actually allocate for borrowed case
        let res = if buf == text { Cow::Borrowed(text) } else { Cow::Owned(buf) };
        Some(res)
    }

    pub fn quote_offsets(&self) -> Option<QuoteOffsets> {
        let text = self.text().as_str();
        let offsets = QuoteOffsets::new(text)?;
        let o = self.syntax().text_range().start();
        let offsets = QuoteOffsets {
            quotes: (offsets.quotes.0 + o, offsets.quotes.1 + o),
            contents: offsets.contents + o,
        };
        Some(offsets)
    }
    pub fn text_range_between_quotes(&self) -> Option<TextRange> {
        self.quote_offsets().map(|it| it.contents)
    }
    pub fn open_quote_text_range(&self) -> Option<TextRange> {
        self.quote_offsets().map(|it| it.quotes.0)
    }
    pub fn close_quote_text_range(&self) -> Option<TextRange> {
        self.quote_offsets().map(|it| it.quotes.1)
    }
}

impl ast::ByteString {
    pub fn is_raw(&self) -> bool {
        self.text().starts_with("br")
    }
}

#[derive(Debug)]
pub enum FormatSpecifier {
    Open,
    Close,
    Integer,
    Identifier,
    Colon,
    Fill,
    Align,
    Sign,
    NumberSign,
    Zero,
    DollarSign,
    Dot,
    Asterisk,
    QuestionMark,
}

pub trait HasFormatSpecifier: AstToken {
    fn char_ranges(
        &self,
    ) -> Option<Vec<(TextRange, Result<char, rustc_lexer::unescape::EscapeError>)>>;

    fn lex_format_specifier<F>(&self, mut callback: F)
    where
        F: FnMut(TextRange, FormatSpecifier),
    {
        let char_ranges = if let Some(char_ranges) = self.char_ranges() {
            char_ranges
        } else {
            return;
        };
        let mut chars = char_ranges.iter().peekable();

        while let Some((range, first_char)) = chars.next() {
            match first_char {
                Ok('{') => {
                    // Format specifier, see syntax at https://doc.rust-lang.org/std/fmt/index.html#syntax
                    if let Some((_, Ok('{'))) = chars.peek() {
                        // Escaped format specifier, `{{`
                        chars.next();
                        continue;
                    }

                    callback(*range, FormatSpecifier::Open);

                    // check for integer/identifier
                    match chars
                        .peek()
                        .and_then(|next| next.1.as_ref().ok())
                        .copied()
                        .unwrap_or_default()
                    {
                        '0'..='9' => {
                            // integer
                            read_integer(&mut chars, &mut callback);
                        }
                        c if c == '_' || c.is_alphabetic() => {
                            // identifier
                            read_identifier(&mut chars, &mut callback);
                        }
                        _ => {}
                    }

                    if let Some((_, Ok(':'))) = chars.peek() {
                        skip_char_and_emit(&mut chars, FormatSpecifier::Colon, &mut callback);

                        // check for fill/align
                        let mut cloned = chars.clone().take(2);
                        let first = cloned
                            .next()
                            .and_then(|next| next.1.as_ref().ok())
                            .copied()
                            .unwrap_or_default();
                        let second = cloned
                            .next()
                            .and_then(|next| next.1.as_ref().ok())
                            .copied()
                            .unwrap_or_default();
                        match second {
                            '<' | '^' | '>' => {
                                // alignment specifier, first char specifies fillment
                                skip_char_and_emit(
                                    &mut chars,
                                    FormatSpecifier::Fill,
                                    &mut callback,
                                );
                                skip_char_and_emit(
                                    &mut chars,
                                    FormatSpecifier::Align,
                                    &mut callback,
                                );
                            }
                            _ => match first {
                                '<' | '^' | '>' => {
                                    skip_char_and_emit(
                                        &mut chars,
                                        FormatSpecifier::Align,
                                        &mut callback,
                                    );
                                }
                                _ => {}
                            },
                        }

                        // check for sign
                        match chars
                            .peek()
                            .and_then(|next| next.1.as_ref().ok())
                            .copied()
                            .unwrap_or_default()
                        {
                            '+' | '-' => {
                                skip_char_and_emit(
                                    &mut chars,
                                    FormatSpecifier::Sign,
                                    &mut callback,
                                );
                            }
                            _ => {}
                        }

                        // check for `#`
                        if let Some((_, Ok('#'))) = chars.peek() {
                            skip_char_and_emit(
                                &mut chars,
                                FormatSpecifier::NumberSign,
                                &mut callback,
                            );
                        }

                        // check for `0`
                        let mut cloned = chars.clone().take(2);
                        let first = cloned.next().and_then(|next| next.1.as_ref().ok()).copied();
                        let second = cloned.next().and_then(|next| next.1.as_ref().ok()).copied();

                        if first == Some('0') && second != Some('$') {
                            skip_char_and_emit(&mut chars, FormatSpecifier::Zero, &mut callback);
                        }

                        // width
                        match chars
                            .peek()
                            .and_then(|next| next.1.as_ref().ok())
                            .copied()
                            .unwrap_or_default()
                        {
                            '0'..='9' => {
                                read_integer(&mut chars, &mut callback);
                                if let Some((_, Ok('$'))) = chars.peek() {
                                    skip_char_and_emit(
                                        &mut chars,
                                        FormatSpecifier::DollarSign,
                                        &mut callback,
                                    );
                                }
                            }
                            c if c == '_' || c.is_alphabetic() => {
                                read_identifier(&mut chars, &mut callback);
                                // can be either width (indicated by dollar sign, or type in which case
                                // the next sign has to be `}`)
                                let next =
                                    chars.peek().and_then(|next| next.1.as_ref().ok()).copied();
                                match next {
                                    Some('$') => skip_char_and_emit(
                                        &mut chars,
                                        FormatSpecifier::DollarSign,
                                        &mut callback,
                                    ),
                                    Some('}') => {
                                        skip_char_and_emit(
                                            &mut chars,
                                            FormatSpecifier::Close,
                                            &mut callback,
                                        );
                                        continue;
                                    }
                                    _ => continue,
                                };
                            }
                            _ => {}
                        }

                        // precision
                        if let Some((_, Ok('.'))) = chars.peek() {
                            skip_char_and_emit(&mut chars, FormatSpecifier::Dot, &mut callback);

                            match chars
                                .peek()
                                .and_then(|next| next.1.as_ref().ok())
                                .copied()
                                .unwrap_or_default()
                            {
                                '*' => {
                                    skip_char_and_emit(
                                        &mut chars,
                                        FormatSpecifier::Asterisk,
                                        &mut callback,
                                    );
                                }
                                '0'..='9' => {
                                    read_integer(&mut chars, &mut callback);
                                    if let Some((_, Ok('$'))) = chars.peek() {
                                        skip_char_and_emit(
                                            &mut chars,
                                            FormatSpecifier::DollarSign,
                                            &mut callback,
                                        );
                                    }
                                }
                                c if c == '_' || c.is_alphabetic() => {
                                    read_identifier(&mut chars, &mut callback);
                                    if chars.peek().and_then(|next| next.1.as_ref().ok()).copied()
                                        != Some('$')
                                    {
                                        continue;
                                    }
                                    skip_char_and_emit(
                                        &mut chars,
                                        FormatSpecifier::DollarSign,
                                        &mut callback,
                                    );
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }

                        // type
                        match chars
                            .peek()
                            .and_then(|next| next.1.as_ref().ok())
                            .copied()
                            .unwrap_or_default()
                        {
                            '?' => {
                                skip_char_and_emit(
                                    &mut chars,
                                    FormatSpecifier::QuestionMark,
                                    &mut callback,
                                );
                            }
                            c if c == '_' || c.is_alphabetic() => {
                                read_identifier(&mut chars, &mut callback);
                            }
                            _ => {}
                        }
                    }

                    if let Some((_, Ok('}'))) = chars.peek() {
                        skip_char_and_emit(&mut chars, FormatSpecifier::Close, &mut callback);
                    } else {
                        continue;
                    }
                }
                _ => {
                    while let Some((_, Ok(next_char))) = chars.peek() {
                        match next_char {
                            '{' => break,
                            _ => {}
                        }
                        chars.next();
                    }
                }
            };
        }

        fn skip_char_and_emit<'a, I, F>(
            chars: &mut std::iter::Peekable<I>,
            emit: FormatSpecifier,
            callback: &mut F,
        ) where
            I: Iterator<Item = &'a (TextRange, Result<char, rustc_lexer::unescape::EscapeError>)>,
            F: FnMut(TextRange, FormatSpecifier),
        {
            let (range, _) = chars.next().unwrap();
            callback(*range, emit);
        }

        fn read_integer<'a, I, F>(chars: &mut std::iter::Peekable<I>, callback: &mut F)
        where
            I: Iterator<Item = &'a (TextRange, Result<char, rustc_lexer::unescape::EscapeError>)>,
            F: FnMut(TextRange, FormatSpecifier),
        {
            let (mut range, c) = chars.next().unwrap();
            assert!(c.as_ref().unwrap().is_ascii_digit());
            while let Some((r, Ok(next_char))) = chars.peek() {
                if next_char.is_ascii_digit() {
                    chars.next();
                    range = range.cover(*r);
                } else {
                    break;
                }
            }
            callback(range, FormatSpecifier::Integer);
        }

        fn read_identifier<'a, I, F>(chars: &mut std::iter::Peekable<I>, callback: &mut F)
        where
            I: Iterator<Item = &'a (TextRange, Result<char, rustc_lexer::unescape::EscapeError>)>,
            F: FnMut(TextRange, FormatSpecifier),
        {
            let (mut range, c) = chars.next().unwrap();
            assert!(c.as_ref().unwrap().is_alphabetic() || *c.as_ref().unwrap() == '_');
            while let Some((r, Ok(next_char))) = chars.peek() {
                if *next_char == '_' || next_char.is_ascii_digit() || next_char.is_alphabetic() {
                    chars.next();
                    range = range.cover(*r);
                } else {
                    break;
                }
            }
            callback(range, FormatSpecifier::Identifier);
        }
    }
}

impl HasFormatSpecifier for ast::String {
    fn char_ranges(
        &self,
    ) -> Option<Vec<(TextRange, Result<char, rustc_lexer::unescape::EscapeError>)>> {
        let text = self.text().as_str();
        let text = &text[self.text_range_between_quotes()? - self.syntax().text_range().start()];
        let offset = self.text_range_between_quotes()?.start() - self.syntax().text_range().start();

        let mut res = Vec::with_capacity(text.len());
        unescape_literal(text, Mode::Str, &mut |range, unescaped_char| {
            res.push((
                TextRange::new(range.start.try_into().unwrap(), range.end.try_into().unwrap())
                    + offset,
                unescaped_char,
            ))
        });

        Some(res)
    }
}

impl ast::IntNumber {
    const SUFFIXES: &'static [&'static str] = &[
        "u8", "u16", "u32", "u64", "u128", "usize", // Unsigned.
        "i8", "i16", "i32", "i64", "i128", "isize", // Signed.
    ];

    pub fn radix(&self) -> Radix {
        match self.text().get(..2).unwrap_or_default() {
            "0b" => Radix::Binary,
            "0o" => Radix::Octal,
            "0x" => Radix::Hexadecimal,
            _ => Radix::Decimal,
        }
    }

    pub fn value(&self) -> Option<u128> {
        let token = self.syntax();

        let mut text = token.text().as_str();
        if let Some(suffix) = self.suffix() {
            text = &text[..text.len() - suffix.len()]
        }

        let radix = self.radix();
        text = &text[radix.prefix_len()..];

        let buf;
        if text.contains("_") {
            buf = text.replace('_', "");
            text = buf.as_str();
        };

        let value = u128::from_str_radix(text, radix as u32).ok()?;
        Some(value)
    }

    pub fn suffix(&self) -> Option<&str> {
        let text = self.text();
        // FIXME: don't check a fixed set of suffixes, `1_0_1_l_o_l` is valid
        // syntax, suffix is `l_o_l`.
        ast::IntNumber::SUFFIXES.iter().chain(ast::FloatNumber::SUFFIXES.iter()).find_map(
            |suffix| {
                if text.ends_with(suffix) {
                    return Some(&text[text.len() - suffix.len()..]);
                }
                None
            },
        )
    }
}

impl ast::FloatNumber {
    const SUFFIXES: &'static [&'static str] = &["f32", "f64"];
    pub fn suffix(&self) -> Option<&str> {
        let text = self.text();
        ast::FloatNumber::SUFFIXES.iter().find_map(|suffix| {
            if text.ends_with(suffix) {
                return Some(&text[text.len() - suffix.len()..]);
            }
            None
        })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Radix {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}

impl Radix {
    pub const ALL: &'static [Radix] =
        &[Radix::Binary, Radix::Octal, Radix::Decimal, Radix::Hexadecimal];

    const fn prefix_len(&self) -> usize {
        match self {
            Self::Decimal => 0,
            _ => 2,
        }
    }
}
