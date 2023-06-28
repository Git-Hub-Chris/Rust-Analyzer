//! This module add real world mbe example for benchmark tests

use rustc_hash::FxHashMap;
use syntax::{
    ast::{self, HasName},
    AstNode, SmolStr,
};
use test_utils::{bench, bench_fixture, skip_slow_tests};

use crate::{
    parser::{MetaVarKind, Op, RepeatKind, Separator},
    syntax_node_to_token_tree, tt, DeclarativeMacro,
};

#[test]
fn benchmark_parse_macro_rules() {
    if skip_slow_tests() {
        return;
    }
    let rules = macro_rules_fixtures_tt();
    let hash: usize = {
        let _pt = bench("mbe parse macro rules");
        rules
            .values()
            .map(|it| DeclarativeMacro::parse_macro_rules(it, true).unwrap().rules.len())
            .sum()
    };
    assert_eq!(hash, 1144);
}

#[test]
fn benchmark_expand_macro_rules() {
    if skip_slow_tests() {
        return;
    }
    let rules = macro_rules_fixtures();
    let invocations = invocation_fixtures(&rules);

    let hash: usize = {
        let _pt = bench("mbe expand macro rules");
        invocations
            .into_iter()
            .map(|(id, tt)| {
                let res = rules[&id].expand(&tt);
                assert!(res.err.is_none());
                res.value.token_trees.len()
            })
            .sum()
    };
    assert_eq!(hash, 69413);
}

fn macro_rules_fixtures() -> FxHashMap<String, DeclarativeMacro> {
    macro_rules_fixtures_tt()
        .into_iter()
        .map(|(id, tt)| (id, DeclarativeMacro::parse_macro_rules(&tt, true).unwrap()))
        .collect()
}

fn macro_rules_fixtures_tt() -> FxHashMap<String, tt::Subtree> {
    let fixture = bench_fixture::numerous_macro_rules();
    let source_file = ast::SourceFile::parse(&fixture).ok().unwrap();

    source_file
        .syntax()
        .descendants()
        .filter_map(ast::MacroRules::cast)
        .map(|rule| {
            let id = rule.name().unwrap().to_string();
            let (def_tt, _) = syntax_node_to_token_tree(rule.token_tree().unwrap().syntax());
            (id, def_tt)
        })
        .collect()
}

/// Generate random invocation fixtures from rules
fn invocation_fixtures(rules: &FxHashMap<String, DeclarativeMacro>) -> Vec<(String, tt::Subtree)> {
    let mut seed = 123456789;
    let mut res = Vec::new();

    for (name, it) in rules {
        for rule in it.rules.iter() {
            // Generate twice
            for _ in 0..2 {
                // The input are generated by filling the `Op` randomly.
                // However, there are some cases generated are ambiguous for expanding, for example:
                // ```rust
                // macro_rules! m {
                //    ($($t:ident),* as $ty:ident) => {}
                // }
                // m!(as u32);  // error: local ambiguity: multiple parsing options: built-in NTs ident ('t') or 1 other option.
                // ```
                //
                // So we just skip any error cases and try again
                let mut try_cnt = 0;
                loop {
                    let mut subtree = tt::Subtree {
                        delimiter: tt::Delimiter {
                            open: tt::TokenId::UNSPECIFIED,
                            close: tt::TokenId::UNSPECIFIED,
                            kind: tt::DelimiterKind::Invisible,
                        },
                        token_trees: vec![],
                    };
                    for op in rule.lhs.iter() {
                        collect_from_op(op, &mut subtree, &mut seed);
                    }
                    if it.expand(&subtree).err.is_none() {
                        res.push((name.clone(), subtree));
                        break;
                    }
                    try_cnt += 1;
                    if try_cnt > 100 {
                        panic!("invocation fixture {name} cannot be generated.\n");
                    }
                }
            }
        }
    }
    return res;

    fn collect_from_op(op: &Op, parent: &mut tt::Subtree, seed: &mut usize) {
        return match op {
            Op::Var { kind, .. } => match kind.as_ref() {
                Some(MetaVarKind::Ident) => parent.token_trees.push(make_ident("foo")),
                Some(MetaVarKind::Ty) => parent.token_trees.push(make_ident("Foo")),
                Some(MetaVarKind::Tt) => parent.token_trees.push(make_ident("foo")),
                Some(MetaVarKind::Vis) => parent.token_trees.push(make_ident("pub")),
                Some(MetaVarKind::Pat) => parent.token_trees.push(make_ident("foo")),
                Some(MetaVarKind::Path) => parent.token_trees.push(make_ident("foo")),
                Some(MetaVarKind::Literal) => parent.token_trees.push(make_literal("1")),
                Some(MetaVarKind::Expr) => parent.token_trees.push(make_ident("foo")),
                Some(MetaVarKind::Lifetime) => {
                    parent.token_trees.push(make_punct('\''));
                    parent.token_trees.push(make_ident("a"));
                }
                Some(MetaVarKind::Block) => {
                    parent.token_trees.push(make_subtree(tt::DelimiterKind::Brace, None))
                }
                Some(MetaVarKind::Item) => {
                    parent.token_trees.push(make_ident("fn"));
                    parent.token_trees.push(make_ident("foo"));
                    parent.token_trees.push(make_subtree(tt::DelimiterKind::Parenthesis, None));
                    parent.token_trees.push(make_subtree(tt::DelimiterKind::Brace, None));
                }
                Some(MetaVarKind::Meta) => {
                    parent.token_trees.push(make_ident("foo"));
                    parent.token_trees.push(make_subtree(tt::DelimiterKind::Parenthesis, None));
                }

                None => (),
                Some(kind) => panic!("Unhandled kind {kind:?}"),
            },
            Op::Literal(it) => parent.token_trees.push(tt::Leaf::from(it.clone()).into()),
            Op::Ident(it) => parent.token_trees.push(tt::Leaf::from(it.clone()).into()),
            Op::Punct(puncts) => {
                for punct in puncts {
                    parent.token_trees.push(tt::Leaf::from(*punct).into());
                }
            }
            Op::Repeat { tokens, kind, separator } => {
                let max = 10;
                let cnt = match kind {
                    RepeatKind::ZeroOrMore => rand(seed) % max,
                    RepeatKind::OneOrMore => 1 + rand(seed) % max,
                    RepeatKind::ZeroOrOne => rand(seed) % 2,
                };
                for i in 0..cnt {
                    for it in tokens.iter() {
                        collect_from_op(it, parent, seed);
                    }
                    if i + 1 != cnt {
                        if let Some(sep) = separator {
                            match sep {
                                Separator::Literal(it) => {
                                    parent.token_trees.push(tt::Leaf::Literal(it.clone()).into())
                                }
                                Separator::Ident(it) => {
                                    parent.token_trees.push(tt::Leaf::Ident(it.clone()).into())
                                }
                                Separator::Puncts(puncts) => {
                                    for it in puncts {
                                        parent.token_trees.push(tt::Leaf::Punct(*it).into())
                                    }
                                }
                            };
                        }
                    }
                }
            }
            Op::Subtree { tokens, delimiter } => {
                let mut subtree = tt::Subtree { delimiter: *delimiter, token_trees: Vec::new() };
                tokens.iter().for_each(|it| {
                    collect_from_op(it, &mut subtree, seed);
                });
                parent.token_trees.push(subtree.into());
            }
            Op::Ignore { .. } | Op::Index { .. } | Op::Count { .. } => {}
        };

        // Simple linear congruential generator for deterministic result
        fn rand(seed: &mut usize) -> usize {
            let a = 1664525;
            let c = 1013904223;
            *seed = usize::wrapping_add(usize::wrapping_mul(*seed, a), c);
            *seed
        }
        fn make_ident(ident: &str) -> tt::TokenTree {
            tt::Leaf::Ident(tt::Ident {
                span: tt::TokenId::unspecified(),
                text: SmolStr::new(ident),
            })
            .into()
        }
        fn make_punct(char: char) -> tt::TokenTree {
            tt::Leaf::Punct(tt::Punct {
                span: tt::TokenId::unspecified(),
                char,
                spacing: tt::Spacing::Alone,
            })
            .into()
        }
        fn make_literal(lit: &str) -> tt::TokenTree {
            tt::Leaf::Literal(tt::Literal {
                span: tt::TokenId::unspecified(),
                text: SmolStr::new(lit),
            })
            .into()
        }
        fn make_subtree(
            kind: tt::DelimiterKind,
            token_trees: Option<Vec<tt::TokenTree>>,
        ) -> tt::TokenTree {
            tt::Subtree {
                delimiter: tt::Delimiter {
                    open: tt::TokenId::unspecified(),
                    close: tt::TokenId::unspecified(),
                    kind,
                },
                token_trees: token_trees.unwrap_or_default(),
            }
            .into()
        }
    }
}
