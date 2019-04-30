/// This module takes a (parsed) definition of `macro_rules` invocation, a
/// `tt::TokenTree` representing an argument of macro invocation, and produces a
/// `tt::TokenTree` for the result of the expansion.
use rustc_hash::FxHashMap;
use ra_syntax::SmolStr;
use tt::TokenId;

use crate::ExpandError;
use crate::tt_cursor::TtCursor;

pub(crate) fn expand(
    rules: &crate::MacroRules,
    input: &tt::Subtree,
) -> Result<tt::Subtree, ExpandError> {
    rules.rules.iter().find_map(|it| expand_rule(it, input).ok()).ok_or(ExpandError::NoMatchingRule)
}

fn expand_rule(rule: &crate::Rule, input: &tt::Subtree) -> Result<tt::Subtree, ExpandError> {
    let mut input = TtCursor::new(input);
    let bindings = match_lhs(&rule.lhs, &mut input)?;
    if !input.is_eof() {
        return Err(ExpandError::UnexpectedToken);
    }

    let mut ctx = ExpandCtx {
        bindings: &bindings,
        nesting: Vec::new(),
        var_expanded: false,
        macro_rules_checker: Default::default(),
    };

    expand_subtree(&rule.rhs, &mut ctx)
}

/// The actual algorithm for expansion is not too hard, but is pretty tricky.
/// `Bindings` structure is the key to understanding what we are doing here.
///
/// On the high level, it stores mapping from meta variables to the bits of
/// syntax it should be substituted with. For example, if `$e:expr` is matched
/// with `1 + 1` by macro_rules, the `Binding` will store `$e -> 1 + 1`.
///
/// The tricky bit is dealing with repetitions (`$()*`). Consider this example:
///
/// ```not_rust
/// macro_rules! foo {
///     ($($ i:ident $($ e:expr),*);*) => {
///         $(fn $ i() { $($ e);*; })*
///     }
/// }
/// foo! { foo 1,2,3; bar 4,5,6 }
/// ```
///
/// Here, the `$i` meta variable is matched first with `foo` and then with
/// `bar`, and `$e` is matched in turn with `1`, `2`, `3`, `4`, `5`, `6`.
///
/// To represent such "multi-mappings", we use a recursive structures: we map
/// variables not to values, but to *lists* of values or other lists (that is,
/// to the trees).
///
/// For the above example, the bindings would store
///
/// ```not_rust
/// i -> [foo, bar]
/// e -> [[1, 2, 3], [4, 5, 6]]
/// ```
///
/// We construct `Bindings` in the `match_lhs`. The interesting case is
/// `TokenTree::Repeat`, where we use `push_nested` to create the desired
/// nesting structure.
///
/// The other side of the puzzle is `expand_subtree`, where we use the bindings
/// to substitute meta variables in the output template. When expanding, we
/// maintain a `nesting` stack of indices which tells us which occurrence from
/// the `Bindings` we should take. We push to the stack when we enter a
/// repetition.
///
/// In other words, `Bindings` is a *multi* mapping from `SmolStr` to
/// `tt::TokenTree`, where the index to select a particular `TokenTree` among
/// many is not a plain `usize`, but an `&[usize]`.
#[derive(Debug, Default)]
struct Bindings {
    inner: FxHashMap<SmolStr, Binding>,
}

#[derive(Debug)]
enum Binding {
    Simple(tt::TokenTree),
    Nested(FxHashMap<usize, Binding>),
}

impl Bindings {
    fn get(&self, name: &SmolStr, nesting: &[usize]) -> Result<&tt::TokenTree, ExpandError> {
        let mut b = self
            .inner
            .get(name)
            .ok_or(ExpandError::BindingError(format!("could not find binding `{}`", name)))?;
        for idx in nesting.iter() {
            b = match b {
                Binding::Simple(_) => break,
                Binding::Nested(bs) => bs.get(idx).ok_or(ExpandError::BindingError(format!(
                    "could not find nested binding `{}`",
                    name
                )))?,
            };
        }
        match b {
            Binding::Simple(it) => Ok(it),
            Binding::Nested(_) => Err(ExpandError::BindingError(format!(
                "expected simple binding, found nested binding `{}`",
                name
            ))),
        }
    }

    fn push_nested(&mut self, id: usize, nested: Bindings) -> Result<(), ExpandError> {
        for (key, value) in nested.inner {
            if !self.inner.contains_key(&key) {
                self.inner.insert(key.clone(), Binding::Nested(Default::default()));
            }
            match self.inner.get_mut(&key) {
                Some(Binding::Nested(it)) => {
                    it.insert(id, value);
                }
                _ => {
                    return Err(ExpandError::BindingError(format!(
                        "could not find binding `{}`",
                        key
                    )));
                }
            }
        }
        Ok(())
    }

    fn merge(&mut self, nested: Bindings) {
        self.inner.extend(nested.inner);
    }
}

fn match_lhs(pattern: &crate::Subtree, input: &mut TtCursor) -> Result<Bindings, ExpandError> {
    let mut res = Bindings::default();
    for pat in pattern.token_trees.iter() {
        match pat {
            crate::TokenTree::Leaf(leaf) => match leaf {
                crate::Leaf::Var(crate::Var { text, kind }) => {
                    let kind = kind.clone().ok_or(ExpandError::UnexpectedToken)?;
                    match kind.as_str() {
                        "ident" => {
                            let ident =
                                input.eat_ident().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(
                                text.clone(),
                                Binding::Simple(tt::Leaf::from(ident).into()),
                            );
                        }
                        "path" => {
                            let path =
                                input.eat_path().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(path.into()));
                        }
                        "expr" => {
                            let expr =
                                input.eat_expr().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(expr.into()));
                        }
                        "ty" => {
                            let ty = input.eat_ty().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(ty.into()));
                        }
                        "pat" => {
                            let pat = input.eat_pat().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(pat.into()));
                        }
                        "stmt" => {
                            let pat = input.eat_stmt().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(pat.into()));
                        }
                        "block" => {
                            let block =
                                input.eat_block().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(block.into()));
                        }
                        "meta" => {
                            let meta =
                                input.eat_meta().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(meta.into()));
                        }
                        "tt" => {
                            let token = input.eat().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(token.into()));
                        }
                        "item" => {
                            let item =
                                input.eat_item().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(item.into()));
                        }
                        "lifetime" => {
                            let lifetime =
                                input.eat_lifetime().ok_or(ExpandError::UnexpectedToken)?.clone();
                            res.inner.insert(text.clone(), Binding::Simple(lifetime.into()));
                        }
                        "literal" => {
                            let literal =
                                input.eat_literal().ok_or(ExpandError::UnexpectedToken)?.clone();

                            res.inner.insert(
                                text.clone(),
                                Binding::Simple(tt::Leaf::from(literal).into()),
                            );
                        }
                        "vis" => {
                            // `vis` is optional
                            if let Some(vis) = input.try_eat_vis() {
                                let vis = vis.clone();
                                res.inner.insert(text.clone(), Binding::Simple(vis.into()));
                            } else {
                                // FIXME: Do we have a better way to represent an empty token ?
                                // Insert an empty subtree for empty token
                                res.inner.insert(
                                    text.clone(),
                                    Binding::Simple(
                                        tt::Subtree {
                                            delimiter: tt::Delimiter::None,
                                            token_trees: vec![],
                                        }
                                        .into(),
                                    ),
                                );
                            }
                        }

                        _ => return Err(ExpandError::UnexpectedToken),
                    }
                }
                crate::Leaf::Punct(punct) => {
                    if !input.eat_punct().map(|p| p.char == punct.char).unwrap_or(false) {
                        return Err(ExpandError::UnexpectedToken);
                    }
                }
                crate::Leaf::Ident(ident) => {
                    if input.eat_ident().map(|i| &i.text) != Some(&ident.text) {
                        return Err(ExpandError::UnexpectedToken);
                    }
                }
                _ => return Err(ExpandError::UnexpectedToken),
            },
            crate::TokenTree::Repeat(crate::Repeat { subtree, kind, separator }) => {
                // Dirty hack to make macro-expansion terminate.
                // This should be replaced by a propper macro-by-example implementation
                let mut limit = 65536;
                let mut counter = 0;

                let mut memento = input.save();

                loop {
                    match match_lhs(subtree, input) {
                        Ok(nested) => {
                            limit -= 1;
                            if limit == 0 {
                                log::warn!("match_lhs excced in repeat pattern exceed limit => {:#?}\n{:#?}\n{:#?}\n{:#?}", subtree, input, kind, separator);
                                break;
                            }

                            memento = input.save();
                            res.push_nested(counter, nested)?;
                            counter += 1;
                            if counter == 1 {
                                if let crate::RepeatKind::ZeroOrOne = kind {
                                    break;
                                }
                            }

                            if let Some(separator) = separator {
                                if !input
                                    .eat_seperator()
                                    .map(|sep| sep == *separator)
                                    .unwrap_or(false)
                                {
                                    input.rollback(memento);
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            input.rollback(memento);
                            break;
                        }
                    }
                }

                match kind {
                    crate::RepeatKind::OneOrMore if counter == 0 => {
                        return Err(ExpandError::UnexpectedToken);
                    }
                    _ => {}
                }
            }
            crate::TokenTree::Subtree(subtree) => {
                let input_subtree =
                    input.eat_subtree().map_err(|_| ExpandError::UnexpectedToken)?;
                if subtree.delimiter != input_subtree.delimiter {
                    return Err(ExpandError::UnexpectedToken);
                }

                let mut input = TtCursor::new(input_subtree);
                let bindings = match_lhs(&subtree, &mut input)?;
                if !input.is_eof() {
                    return Err(ExpandError::UnexpectedToken);
                }

                res.merge(bindings);
            }
        }
    }
    Ok(res)
}

#[derive(Debug, Eq, PartialEq)]
enum InnerMacroRulesState {
    None,
    MacroRules,
    Ident,
    Excl,
    Subtree,
}

/// A state machine to check whether we are inside macro_rules
#[derive(Debug, Default)]
struct MacroRulesChecker {
    inner: Vec<InnerMacroRulesState>,
    last: Option<usize>,
}

impl MacroRulesChecker {
    fn begin_subtree(&mut self) {
        if let Some(InnerMacroRulesState::Ident) = self.inner.last() {
            let old = self.inner.last_mut();
            *old.unwrap() = InnerMacroRulesState::Subtree;
            self.last = Some(self.inner.len());
        } else {
            self.last = None;
        }
    }

    fn end_subtree(&mut self) {
        if let Some(len) = self.last {
            self.inner.truncate(len);
        }
    }

    fn is_inside(&self) -> bool {
        if let Some(InnerMacroRulesState::Subtree) = self.inner.last() {
            true
        } else {
            false
        }
    }

    fn update(&mut self, new_state: InnerMacroRulesState) {
        match self.inner.last() {
            Some(InnerMacroRulesState::Subtree) | None
                if new_state == InnerMacroRulesState::MacroRules =>
            {
                self.inner.push(InnerMacroRulesState::MacroRules);
            }
            Some(InnerMacroRulesState::MacroRules) if new_state == InnerMacroRulesState::Excl => {
                *self.inner.last_mut().unwrap() = InnerMacroRulesState::Excl;
            }
            Some(InnerMacroRulesState::Excl) if new_state == InnerMacroRulesState::Ident => {
                *self.inner.last_mut().unwrap() = InnerMacroRulesState::Ident;
            }
            Some(InnerMacroRulesState::Subtree) | None => {
                // Do nothing
            }
            Some(_) => {
                self.inner.pop();
            }
        }
    }
}

#[derive(Debug)]
struct ExpandCtx<'a> {
    bindings: &'a Bindings,
    nesting: Vec<usize>,
    var_expanded: bool,
    macro_rules_checker: MacroRulesChecker,
}

fn expand_subtree(
    template: &crate::Subtree,
    ctx: &mut ExpandCtx,
) -> Result<tt::Subtree, ExpandError> {
    let token_trees = template
        .token_trees
        .iter()
        .map(|it| expand_tt(it, ctx))
        .filter(|it| {
            // Filter empty subtree
            if let Ok(tt::TokenTree::Subtree(subtree)) = it {
                subtree.delimiter != tt::Delimiter::None || !subtree.token_trees.is_empty()
            } else {
                true
            }
        })
        .collect::<Result<Vec<_>, ExpandError>>()?;

    Ok(tt::Subtree { token_trees, delimiter: template.delimiter })
}

/// Reduce single token subtree to single token
/// In `tt` matcher case, all tt tokens will be braced by a Delimiter::None
/// which makes all sort of problems.
fn reduce_single_token(mut subtree: tt::Subtree) -> tt::TokenTree {
    if subtree.delimiter != tt::Delimiter::None || subtree.token_trees.len() != 1 {
        return subtree.into();
    }

    match subtree.token_trees.pop().unwrap() {
        tt::TokenTree::Subtree(subtree) => reduce_single_token(subtree),
        tt::TokenTree::Leaf(token) => token.into(),
    }
}

fn expand_tt(
    template: &crate::TokenTree,
    ctx: &mut ExpandCtx,
) -> Result<tt::TokenTree, ExpandError> {
    let mut macro_rules_state = InnerMacroRulesState::None;

    let res: tt::TokenTree = match template {
        crate::TokenTree::Subtree(subtree) => {
            ctx.macro_rules_checker.begin_subtree();
            let res = expand_subtree(subtree, ctx)?.into();
            ctx.macro_rules_checker.end_subtree();

            res
        }
        crate::TokenTree::Repeat(repeat) => {
            let mut token_trees: Vec<tt::TokenTree> = Vec::new();
            ctx.nesting.push(0);
            // Dirty hack to make macro-expansion terminate.
            // This should be replaced by a propper macro-by-example implementation
            let mut limit = 65536;
            let mut has_seps = 0;
            let mut counter = 0;

            // We store the old var expaned value, and restore it later
            // It is because before this `$repeat`,
            // it is possible some variables already expanad in the same subtree
            //
            // `some_var_expanded` keep check if the deeper subtree has expanded variables
            let mut some_var_expanded = false;
            let old_var_expanded = ctx.var_expanded;
            ctx.var_expanded = false;

            while let Ok(t) = expand_subtree(&repeat.subtree, ctx) {
                // if no var expaned in the child, we count it as a fail
                if !ctx.var_expanded {
                    break;
                }

                // Reset `ctx.var_expaneded` to see if there is other expaned variable
                // in the next matching
                some_var_expanded = true;
                ctx.var_expanded = false;

                counter += 1;
                limit -= 1;
                if limit == 0 {
                    log::warn!(
                        "expand_tt excced in repeat pattern exceed limit => {:#?}\n{:#?}",
                        template,
                        ctx
                    );
                    break;
                }

                let idx = ctx.nesting.pop().unwrap();
                ctx.nesting.push(idx + 1);
                token_trees.push(reduce_single_token(t).into());

                if let Some(ref sep) = repeat.separator {
                    match sep {
                        crate::Separator::Ident(ident) => {
                            has_seps = 1;
                            token_trees.push(tt::Leaf::from(ident.clone()).into());
                        }
                        crate::Separator::Literal(lit) => {
                            has_seps = 1;
                            token_trees.push(tt::Leaf::from(lit.clone()).into());
                        }

                        crate::Separator::Puncts(puncts) => {
                            has_seps = puncts.len();
                            for punct in puncts {
                                token_trees.push(tt::Leaf::from(*punct).into());
                            }
                        }
                    }
                }

                if let crate::RepeatKind::ZeroOrOne = repeat.kind {
                    break;
                }
            }

            // Restore the `var_expanded` by combining old one and the new one
            ctx.var_expanded = some_var_expanded || old_var_expanded;

            ctx.nesting.pop().unwrap();
            for _ in 0..has_seps {
                token_trees.pop();
            }

            if crate::RepeatKind::OneOrMore == repeat.kind && counter == 0 {
                return Err(ExpandError::UnexpectedToken);
            }

            // Check if it is a singel token subtree without any delimiter
            // e.g {Delimiter:None> ['>'] /Delimiter:None>}
            reduce_single_token(tt::Subtree { token_trees, delimiter: tt::Delimiter::None })
        }
        crate::TokenTree::Leaf(leaf) => match leaf {
            crate::Leaf::Ident(ident) => {
                if ident.text == "macro_rules" {
                    macro_rules_state = InnerMacroRulesState::MacroRules;
                } else {
                    macro_rules_state = InnerMacroRulesState::Ident;
                }

                tt::Leaf::from(tt::Ident { text: ident.text.clone(), id: TokenId::unspecified() })
                    .into()
            }
            crate::Leaf::Punct(punct) => {
                if punct.char == '!' {
                    macro_rules_state = InnerMacroRulesState::Excl;
                }

                tt::Leaf::from(punct.clone()).into()
            }
            crate::Leaf::Var(v) => {
                if v.text == "crate" {
                    // FIXME: Properly handle $crate token
                    tt::Leaf::from(tt::Ident { text: "$crate".into(), id: TokenId::unspecified() })
                        .into()
                } else {
                    match ctx.bindings.get(&v.text, &ctx.nesting) {
                        Ok(tkn) => {
                            let tkn = tkn.clone();
                            ctx.var_expanded = true;

                            if let tt::TokenTree::Subtree(subtree) = tkn {
                                reduce_single_token(subtree)
                            } else {
                                tkn
                            }
                        }
                        // Note that it is possible if $var is not part of the current macro
                        // For example:
                        // ```
                        // macro_rules! foo {
                        //  ($a:ident, $b:ident, $c:tt) => {
                        //     macro_rules! bar {
                        //         ($bi:ident) => {
                        //             fn $bi() -> u8 {$c}
                        //         }
                        //     }
                        // }
                        //
                        // $bi is not part of the current macro variabl
                        Err(err) => {
                            if ctx.macro_rules_checker.is_inside() {
                                tt::Leaf::from(tt::Ident {
                                    text: v.text.clone(),
                                    id: TokenId::unspecified(),
                                })
                                .into()
                            } else {
                                return Err(err);
                            }
                        }
                    }
                }
            }
            crate::Leaf::Literal(l) => tt::Leaf::from(tt::Literal { text: l.text.clone() }).into(),
        },
    };

    ctx.macro_rules_checker.update(macro_rules_state);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use ra_syntax::{ast, AstNode};

    use super::*;
    use crate::ast_to_token_tree;

    #[test]
    fn test_expand_rule() {
        assert_err(
            "($i:ident) => ($j)",
            "foo!{a}",
            ExpandError::BindingError(String::from("could not find binding `j`")),
        );

        assert_err(
            "($($i:ident);*) => ($i)",
            "foo!{a}",
            ExpandError::BindingError(String::from(
                "expected simple binding, found nested binding `i`",
            )),
        );

        assert_err("($i) => ($i)", "foo!{a}", ExpandError::UnexpectedToken);
        assert_err("($i:) => ($i)", "foo!{a}", ExpandError::UnexpectedToken);

        // FIXME:
        // Add an err test case for ($($i:ident)) => ($())
    }

    fn assert_err(macro_body: &str, invocation: &str, err: ExpandError) {
        assert_eq!(expand_first(&create_rules(&format_macro(macro_body)), invocation), Err(err));
    }

    fn format_macro(macro_body: &str) -> String {
        format!(
            "
        macro_rules! foo {{
            {}
        }}
",
            macro_body
        )
    }

    fn create_rules(macro_definition: &str) -> crate::MacroRules {
        let source_file = ast::SourceFile::parse(macro_definition);
        let macro_definition =
            source_file.syntax().descendants().find_map(ast::MacroCall::cast).unwrap();

        let (definition_tt, _) = ast_to_token_tree(macro_definition.token_tree().unwrap()).unwrap();
        crate::MacroRules::parse(&definition_tt).unwrap()
    }

    fn expand_first(
        rules: &crate::MacroRules,
        invocation: &str,
    ) -> Result<tt::Subtree, ExpandError> {
        let source_file = ast::SourceFile::parse(invocation);
        let macro_invocation =
            source_file.syntax().descendants().find_map(ast::MacroCall::cast).unwrap();

        let (invocation_tt, _) = ast_to_token_tree(macro_invocation.token_tree().unwrap()).unwrap();

        expand_rule(&rules.rules[0], &invocation_tt)
    }
}
