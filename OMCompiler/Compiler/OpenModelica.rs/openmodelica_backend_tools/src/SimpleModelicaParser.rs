// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(
    unreachable_patterns,
    unreachable_code,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unused_imports,
    unused_variables,
    non_upper_case_globals,
    unused_mut
)]

use arcstr::{ArcStr, format, literal};
use const_str;
use loop_unwrap::unwrap_break_err;
use metamodelica::Result;
use metamodelica::*; // Built-in types and functions
use std::sync::Arc;

use crate::LexerModelicaDiff;
use crate::LexerModelicaDiff::Token;
use crate::LexerModelicaDiff::TokenId;
use crate::LexerModelicaDiff::modelicaDiffTokenEq;
use crate::LexerModelicaDiff::printToken;
use crate::LexerModelicaDiff::tokenContent;
use openmodelica_util::AvlSetString;
use openmodelica_util::DiffAlgorithm;
use openmodelica_util::DiffAlgorithm::Diff;
use openmodelica_util::DiffAlgorithm::diff;
use openmodelica_util::Error;
use openmodelica_util::Print;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub(crate) static newlineToken: std::sync::LazyLock<Token> = std::sync::LazyLock::new(|| Token {
    fileName: literal!(""),
    id: TokenId::NEWLINE.clone(),
    fileContents: literal!("\n"),
    byteOffset: 1,
    length: 1,
    lineNumberStart: 1,
    columnNumberStart: 1,
    lineNumberEnd: 1,
    columnNumberEnd: 1,
});

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ParseTree {
    EMPTY,
    NODE {
        label: metamodelica::Ref<ParseTree>,
        nodes: metamodelica::List<metamodelica::Ref<ParseTree>>,
    },
    LEAF {
        token: Token,
    },
}
impl metamodelica::gc::MMTrace for ParseTree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ParseTree::EMPTY => Ok(()),
            ParseTree::NODE { label, nodes } => {
                metamodelica::gc::MMTrace::mm_accept(label, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nodes, __mmv)?;
                Ok(())
            }
            ParseTree::LEAF { token } => {
                metamodelica::gc::MMTrace::mm_accept(token, __mmv)?;
                Ok(())
            }
        }
    }
}
impl ParseTree {
    pub fn interned_EMPTY() -> metamodelica::Ref<ParseTree> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<ParseTree>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(ParseTree::EMPTY));
        (*INTERNED).clone()
    }
}
pub fn interned_EMPTY() -> metamodelica::Ref<ParseTree> {
    ParseTree::interned_EMPTY()
}
impl Default for ParseTree {
    fn default() -> Self {
        Self::EMPTY
    }
}
pub use self::ParseTree::{EMPTY, LEAF, NODE};

pub fn parseTreeStr(mut trees: &metamodelica::List<metamodelica::Ref<ParseTree>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut i: i32;
    i = Print::saveAndClearBuf()?;
    match '__try0: {
        for mut tree in &**trees {
            unwrap_break_err!(parseTreeStrWork(metamodelica::AsArg::as_arg(&tree)), '__try0);
        }
        r#str = unwrap_break_err!(Print::getString(), '__try0);
        unwrap_break_err!(Print::restoreBuf(i), '__try0);
        Ok::<_, &'static str>((r#str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            r#str = __try0_o0;
        }
        Err(__try0_err) => {
            Print::restoreBuf(i)?;
            return Err(__try0_err);
        }
    }
    Ok(r#str)
}

pub fn treeDiff(
    mut t1: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut t2: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut nTokens: i32,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut res: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut within1: metamodelica::Ref<ParseTree>;
    let mut within2: metamodelica::Ref<ParseTree>;
    let mut t1_updated: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut t2_updated: metamodelica::List<metamodelica::Ref<ParseTree>>;
    within1 = findWithin(&t1)?;
    within2 = findWithin(&t2)?;
    (t1_updated, t2_updated) = (::match_deref::match_deref! { match &((within1.clone(), within2.clone())) {
        (Deref @ ParseTree::EMPTY { .. }, Deref @ ParseTree::EMPTY { .. }) => (t1, t2),
        (_, Deref @ ParseTree::EMPTY { .. }) => (t1, metamodelica::cons(within1, metamodelica::cons(metamodelica::Ref::new(ParseTree::LEAF { token: newlineToken.clone() }), t2))),
        (Deref @ ParseTree::EMPTY { .. }, _) => (metamodelica::cons(within2, metamodelica::cons(metamodelica::Ref::new(ParseTree::LEAF { token: newlineToken.clone() }), metamodelica::cons(metamodelica::Ref::new(ParseTree::LEAF { token: newlineToken.clone() }), t1))), t2),
        _ => (t1, t2),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res = treeDiffWork1(t1_updated, t2_updated, nTokens)?;
    Ok(res)
}

pub type CmpParseTreeFunc = std::sync::Arc<
    dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>, metamodelica::Ref<ParseTree>) -> Result<bool> + 'static,
>;

pub fn parseTreeNodeStr(mut tree: &metamodelica::Ref<ParseTree>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut i: i32;
    i = Print::saveAndClearBuf()?;
    match '__try0: {
        unwrap_break_err!(parseTreeStrWork(tree), '__try0);
        r#str = unwrap_break_err!(Print::getString(), '__try0);
        unwrap_break_err!(Print::restoreBuf(i), '__try0);
        Ok::<_, &'static str>((r#str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            r#str = __try0_o0;
        }
        Err(__try0_err) => {
            Print::restoreBuf(i)?;
            return Err(__try0_err);
        }
    }
    Ok(r#str)
}

pub type partialParser = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::List<Token>,
            metamodelica::List<metamodelica::Ref<ParseTree>>,
        ) -> Result<(
            metamodelica::List<Token>,
            metamodelica::List<metamodelica::Ref<ParseTree>>,
        )> + 'static,
>;

pub fn stored_definition(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::WITHIN.clone())?;
    if b {
        (tokens, tree, b) = LA1(tokens, tree, First::name.clone(), false)?;
        if b {
            (tokens, tree) = name(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::SEMICOLON.clone())?;
        outTree = metamodelica::cons(
            makeNode(
                tree.reverse(),
                metamodelica::Ref::new(ParseTree::LEAF {
                    token: makeToken(TokenId::IDENT.clone(), literal!("$within")),
                }),
            ),
            metamodelica::nil(),
        );
        tree = metamodelica::nil();
    } else {
        outTree = metamodelica::nil();
    }
    (tokens, tree, b) = LA1(tokens, tree, First::class_definition.clone(), false)?;
    while b {
        (tokens, tree, _) = scanOpt(tokens, tree, TokenId::FINAL.clone())?;
        (tokens, tree, _) = class_definition(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::SEMICOLON.clone())?;
        (tokens, tree, b) = LA1(tokens, tree, First::class_definition.clone(), false)?;
        outTree = metamodelica::cons(
            makeNode(tree.reverse(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()),
            outTree,
        );
        tree = metamodelica::nil();
    }
    (tokens, tree) = eatWhitespace(tokens, tree)?;
    if !((tokens).is_empty()) {
        error(tokens.clone(), tree.clone(), metamodelica::nil())?;
    }
    outTree = metamodelica::cons(
        makeNode(
            listAppend(tree, listAppend(outTree, inTree)).reverse(),
            metamodelica::Ref::new(ParseTree::LEAF {
                token: makeToken(TokenId::IDENT.clone(), literal!("$program")),
            }),
        ),
        metamodelica::nil(),
    );
    Ok((tokens, outTree))
}

fn class_definition(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    tree = metamodelica::nil();
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::ENCAPSULATED.clone())?;
    (tokens, tree) = class_prefixes(tokens, tree)?;
    (tokens, tree, nodeName) = class_specifier(tokens, tree)?;
    outTree = metamodelica::cons(makeNode(tree.reverse(), nodeName.clone()), inTree);
    Ok((tokens, outTree, nodeName))
}

fn class_prefixes(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::PARTIAL.clone())?;
    (tokens, tree, id) = peek(tokens, tree)?;
    let () = (match id {
        TokenId::OPERATOR { .. } => {
            (tokens, tree) = consume(tokens, tree)?;
            (tokens, tree, _) = LA1(
                tokens,
                tree,
                list![TokenId::RECORD.clone(), TokenId::FUNCTION.clone()],
                true,
            )?;
            ()
        }
        TokenId::EXPANDABLE => {
            (tokens, tree) = consume(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::CONNECTOR.clone())?;
            ()
        }
        mut id if (listMember(id, list![TokenId::PURE.clone(), TokenId::IMPURE.clone()])) => {
            (tokens, tree) = consume(tokens, tree)?;
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::OPERATOR.clone())?;
            (tokens, tree) = scanOneOf(
                tokens,
                tree,
                if (b) {
                    list![TokenId::FUNCTION.clone()]
                } else {
                    list![TokenId::FUNCTION.clone(), TokenId::RECORD.clone()]
                },
            )?;
            ()
        }
        _ => {
            (tokens, tree) = scanOneOf(
                tokens,
                tree,
                list![
                    TokenId::CLASS.clone(),
                    TokenId::MODEL.clone(),
                    TokenId::RECORD.clone(),
                    TokenId::BLOCK.clone(),
                    TokenId::CONNECTOR.clone(),
                    TokenId::TYPE.clone(),
                    TokenId::PACKAGE.clone(),
                    TokenId::FUNCTION.clone(),
                    TokenId::OPERATOR.clone()
                ],
            )?;
            ()
        }
    });
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn class_specifier(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    tree = inTree;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::IDENT.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    nodeName = metamodelica::Own::own(__pa0);
    nodeName = parseTreeFilterWhitespace(nodeName);
    if b {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::EQUALS.clone())?;
        if b {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::DER.clone())?;
            if b {
                (tokens, tree) = scan(tokens, tree, TokenId::LPAR.clone())?;
                (tokens, tree) = name(tokens, tree)?;
                (tokens, tree) = scan(tokens, tree, TokenId::COMMA.clone())?;
                (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
                loop {
                    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
                    if !(b) {
                        break;
                    }
                    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
                }
                (tokens, tree) = scan(tokens, tree, TokenId::RPAR.clone())?;
                (tokens, tree) = comment(tokens, tree)?;
            } else {
                (tokens, tree) = short_class_specifier1(tokens, tree)?;
            }
        } else {
            (tokens, tree) = string_comment(tokens, tree)?;
            (tokens, tree) = composition(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
            (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
        }
    } else {
        (tokens, tree) = scan(tokens, tree, TokenId::EXTENDS.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
        (tokens, tree, b) = LA1(tokens, tree, First::class_modification.clone(), false)?;
        if b {
            (tokens, tree) = class_modification(tokens, tree)?;
        }
        (tokens, tree) = string_comment(tokens, tree)?;
        (tokens, tree) = composition(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    }
    outTree = tree;
    Ok((tokens, outTree, nodeName))
}

fn short_class_specifier1(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ENUMERATION.clone())?;
    if b {
        (tokens, tree) = scan(tokens, tree, TokenId::LPAR.clone())?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COLON.clone())?;
        if !(b) {
            loop {
                (tokens, tree) = enumeration_literal(tokens, tree)?;
                (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
                if !(b) {
                    break;
                }
            }
        }
        (tokens, tree) = scan(tokens, tree, TokenId::RPAR.clone())?;
    } else {
        (tokens, tree) = base_prefix(tokens, tree)?;
        (tokens, tree) = name(tokens, tree)?;
        (tokens, tree, b) = LA1(tokens, tree, list![TokenId::LBRACK.clone()], false)?;
        if b {
            (tokens, tree) = array_subscripts(tokens, tree)?;
        }
        (tokens, tree, b) = LA1(tokens, tree, First::class_modification.clone(), false)?;
        if b {
            (tokens, tree) = class_modification(tokens, tree)?;
        }
    }
    (tokens, tree) = comment(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn enumeration_literal(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    (tokens, tree) = comment(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn composition(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    (tokens, tree) = element_list(tokens, tree)?;
    loop {
        (tokens, tree, b) = LA1(
            tokens,
            tree,
            list![
                TokenId::PROTECTED.clone(),
                TokenId::PUBLIC.clone(),
                TokenId::INITIAL.clone(),
                TokenId::EQUATION.clone(),
                TokenId::ALGORITHM.clone()
            ],
            false,
        )?;
        if !(b) {
            break;
        }
        (tokens, tree, b) = LA1(
            tokens,
            tree,
            list![TokenId::PROTECTED.clone(), TokenId::PUBLIC.clone()],
            true,
        )?;
        if b {
            (tokens, tree) = element_list(tokens, tree)?;
        } else {
            (tokens, tree, _) = scanOpt(tokens, tree, TokenId::INITIAL.clone())?;
            (tokens, tree, b) = LA1(tokens, tree, list![TokenId::ALGORITHM.clone()], false)?;
            if b {
                (tokens, tree) = algorithm_section(tokens, tree)?;
            } else {
                (tokens, tree) = equation_section(tokens, tree)?;
            }
        }
    }
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::EXTERNAL.clone())?;
    if b {
        (tokens, tree, _) = scanOpt(tokens, tree, TokenId::STRING.clone())?;
        (tokens, tree, id) = peek(tokens, tree)?;
        if !(id == TokenId::ANNOTATION.clone() || id == TokenId::SEMICOLON.clone()) {
            (tokens, tree) = external_function_call(tokens, tree)?;
        }
        (tokens, tree, b) = LA1(tokens, tree, First::_annotation.clone(), false)?;
        if b {
            (tokens, tree) = _annotation(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::SEMICOLON.clone())?;
    }
    b = true;
    while b {
        (tokens, tree, b) = LA1(tokens, tree, First::_annotation.clone(), false)?;
        if b {
            (tokens, tree) = _annotation(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::SEMICOLON.clone())?;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn external_function_call(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = LAk(
        tokens,
        tree,
        list![list![TokenId::IDENT.clone()], list![TokenId::LPAR.clone()]],
    )?;
    if !(b) {
        (tokens, tree) = component_reference(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::EQUALS.clone())?;
    }
    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    (tokens, tree) = output_expression_list(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn algorithm_section(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::INITIAL.clone())?;
    (tokens, tree) = scan(tokens, tree, TokenId::ALGORITHM.clone())?;
    (tokens, tree) = statement_list(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        metamodelica::Ref::new(ParseTree::LEAF {
            token: makeToken(TokenId::IDENT.clone(), literal!("$algorithm_section")),
        }),
    );
    Ok((tokens, outTree))
}

fn statement(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    ArcStr,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut label: ArcStr = literal!("$statement");
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    (tokens, tree, id) = peek(tokens, tree)?;
    if id == TokenId::BREAK.clone() || id == TokenId::RETURN.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        label = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id)));
            ArcStr::from(__mm_s)
        };
    } else if listMember(id, First::component_reference.clone()) {
        (tokens, tree) = component_reference(tokens, tree)?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ASSIGN.clone())?;
        if b {
            (tokens, tree) = expression(tokens, tree)?;
            label = literal!("$assign");
        } else {
            (tokens, tree) = function_call_args(tokens, tree)?;
            label = literal!("$statement_call");
        }
    } else if id == TokenId::IF.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
        (tokens, tree) = statement_list(tokens, tree)?;
        loop {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ELSEIF.clone())?;
            if !(b) {
                break;
            }
            (tokens, tree) = expression(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
            (tokens, tree) = statement_list(tokens, tree)?;
        }
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ELSE.clone())?;
        if b {
            (tokens, tree) = statement_list(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::IF.clone())?;
        label = literal!("$if");
    } else if id == TokenId::WHEN.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
        (tokens, tree) = statement_list(tokens, tree)?;
        loop {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ELSEWHEN.clone())?;
            if !(b) {
                break;
            }
            (tokens, tree) = expression(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
            (tokens, tree) = statement_list(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::WHEN.clone())?;
        label = literal!("$when");
    } else if id == TokenId::FOR.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = for_indices(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::LOOP.clone())?;
        (tokens, tree) = statement_list(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::FOR.clone())?;
        label = literal!("$for");
    } else if id == TokenId::WHILE.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::LOOP.clone())?;
        (tokens, tree) = statement_list(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::WHILE.clone())?;
        label = literal!("$while");
    } else {
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ASSIGN.clone())?;
        if b {
            (tokens, tree) = expression(tokens, tree)?;
        }
        label = literal!("$assign_expression");
    }
    (tokens, tree) = comment(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, label))
}

fn statement_list(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut label: ArcStr;
    outTree = metamodelica::nil();
    loop {
        (tokens, tree, b) = LA1(tokens, tree, Follow::statement_equation.clone(), false)?;
        if b {
            break;
        }
        (tokens, tree, label) = statement(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::SEMICOLON.clone())?;
        outTree = metamodelica::cons(
            makeNode(
                tree.reverse(),
                metamodelica::Ref::new(ParseTree::LEAF {
                    token: makeToken(TokenId::IDENT.clone(), label),
                }),
            ),
            outTree,
        );
        tree = metamodelica::nil();
    }
    outTree = listAppend(tree, listAppend(outTree, inTree));
    Ok((tokens, outTree))
}

fn equation_section(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::INITIAL.clone())?;
    (tokens, tree) = scan(tokens, tree, TokenId::EQUATION.clone())?;
    (tokens, tree) = equation_list(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        metamodelica::Ref::new(ParseTree::LEAF {
            token: makeToken(TokenId::IDENT.clone(), literal!("$equation_section")),
        }),
    );
    Ok((tokens, outTree))
}

fn _equation(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    ArcStr,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut label: ArcStr = literal!("$equation");
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    (tokens, tree, id) = peek(tokens, tree)?;
    if id == TokenId::IF.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
        (tokens, tree) = equation_list(tokens, tree)?;
        loop {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ELSEIF.clone())?;
            if !(b) {
                break;
            }
            (tokens, tree) = expression(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
            (tokens, tree) = equation_list(tokens, tree)?;
        }
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ELSE.clone())?;
        if b {
            (tokens, tree) = equation_list(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::IF.clone())?;
        label = literal!("$if_equation");
    } else if id == TokenId::WHEN.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
        (tokens, tree) = equation_list(tokens, tree)?;
        loop {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ELSEWHEN.clone())?;
            if !(b) {
                break;
            }
            (tokens, tree) = expression(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
            (tokens, tree) = equation_list(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::WHEN.clone())?;
        label = literal!("$when_equation");
    } else if id == TokenId::FOR.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = for_indices(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::LOOP.clone())?;
        (tokens, tree) = equation_list(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::END.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::FOR.clone())?;
        label = literal!("$for_equation");
    } else if id == TokenId::CONNECT.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::LPAR.clone())?;
        (tokens, tree) = component_reference(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::COMMA.clone())?;
        (tokens, tree) = component_reference(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::RPAR.clone())?;
        label = literal!("$connect_equation");
    } else {
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::EQUALS.clone())?;
        if b {
            (tokens, tree) = expression(tokens, tree)?;
            label = literal!("$equality_equation");
        } else {
            label = literal!("$singleton_equation");
        }
    }
    (tokens, tree) = comment(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, label))
}

fn equation_list(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut label: ArcStr;
    outTree = metamodelica::nil();
    loop {
        (tokens, tree, b) = LA1(tokens, tree, Follow::statement_equation.clone(), false)?;
        if b {
            break;
        }
        (tokens, tree, label) = _equation(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::SEMICOLON.clone())?;
        outTree = metamodelica::cons(
            makeNode(
                tree.reverse(),
                metamodelica::Ref::new(ParseTree::LEAF {
                    token: makeToken(TokenId::IDENT.clone(), label),
                }),
            ),
            outTree,
        );
        tree = metamodelica::nil();
    }
    outTree = listAppend(tree, listAppend(outTree, inTree));
    Ok((tokens, outTree))
}

fn element_list(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut isAnnotation: bool;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    outTree = metamodelica::nil();
    loop {
        (tokens, tree, b) = LA1(tokens, tree, First::element.clone(), false)?;
        if !(b) {
            break;
        }
        (tokens, tree, nodeName, isAnnotation) = element(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::SEMICOLON.clone())?;
        if !(isAnnotation) {
            outTree = metamodelica::cons(makeNode(tree.reverse(), nodeName), outTree);
            tree = metamodelica::nil();
        }
    }
    outTree = listAppend(tree, listAppend(outTree, inTree));
    Ok((tokens, outTree))
}

fn element(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
    bool,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree> = metamodelica::Ref::new(ParseTree::LEAF {
        token: makeToken(TokenId::IDENT.clone(), literal!("$element")),
    });
    let mut isAnnotation: bool = false;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    let mut b1: bool;
    (tokens, tree, id) = peek(tokens, tree)?;
    nodeName = (match id {
        TokenId::IMPORT { .. } => {
            (tokens, tree) = import_clause(tokens, tree)?;
            metamodelica::Ref::new(ParseTree::LEAF {
                token: makeToken(TokenId::IDENT.clone(), literal!("$import")),
            })
        }
        TokenId::EXTENDS { .. } => {
            (tokens, tree) = extends_clause(tokens, tree)?;
            metamodelica::Ref::new(ParseTree::LEAF {
                token: makeToken(TokenId::IDENT.clone(), literal!("$extends")),
            })
        }
        TokenId::ANNOTATION { .. } => {
            (tokens, tree) = _annotation(tokens, tree)?;
            isAnnotation = true;
            metamodelica::Ref::new(ParseTree::LEAF {
                token: makeToken(TokenId::IDENT.clone(), literal!("$annotation")),
            })
        }
        _ => {
            (tokens, tree, _) = scanOpt(tokens, tree, TokenId::REDECLARE.clone())?;
            (tokens, tree, _) = scanOpt(tokens, tree, TokenId::FINAL.clone())?;
            (tokens, tree, _) = scanOpt(tokens, tree, TokenId::INNER.clone())?;
            (tokens, tree, _) = scanOpt(tokens, tree, TokenId::OUTER.clone())?;
            (tokens, tree, b1) = scanOpt(tokens, tree, TokenId::REPLACEABLE.clone())?;
            (tokens, tree, b) = LA1(tokens, tree, First::class_definition.clone(), false)?;
            if b {
                (tokens, tree, nodeName) = class_definition(tokens, tree)?;
            } else {
                (tokens, tree, nodeName) = component_clause(tokens, tree)?;
            }
            if b1 {
                (tokens, tree, b) = LA1(tokens, tree, list![TokenId::CONSTRAINEDBY.clone()], false)?;
                if b {
                    (tokens, tree) = constraining_clause(tokens, tree)?;
                    (tokens, tree) = comment(tokens, tree)?;
                }
            }
            nodeName
        }
    });
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName, isAnnotation))
}

fn constraining_clause(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::CONSTRAINEDBY.clone())?;
    (tokens, tree) = name(tokens, tree)?;
    (tokens, tree, b) = LA1(tokens, tree, First::class_modification.clone(), false)?;
    if b {
        (tokens, tree) = class_modification(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn component_clause(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut nodeNames: metamodelica::List<metamodelica::Ref<ParseTree>>;
    (tokens, tree) = type_prefix(tokens, tree)?;
    (tokens, tree) = type_specifier(tokens, tree)?;
    (tokens, tree, b) = LA1(tokens, tree, list![TokenId::LBRACK.clone()], false)?;
    if b {
        (tokens, tree) = array_subscripts(tokens, tree)?;
    }
    tree = metamodelica::cons(
        makeNode(
            tree.reverse(),
            metamodelica::Ref::new(ParseTree::LEAF {
                token: makeToken(TokenId::IDENT.clone(), literal!("$type_specifier")),
            }),
        ),
        metamodelica::nil(),
    );
    (tokens, tree, nodeNames) = component_list(tokens, tree)?;
    nodeName = metamodelica::Ref::new(ParseTree::LEAF {
        token: makeToken(TokenId::IDENT.clone(), {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$component:"));
            __mm_s.push_str(&*stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut name in (nodeNames).into_iter().cloned() {
                        let __x = parseTreeStr(&(metamodelica::cons(name.clone(), metamodelica::nil())))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(","),
            ));
            ArcStr::from(__mm_s)
        }),
    });
    outTree = makeNodePrependTree(tree.reverse(), inTree, nodeName.clone());
    Ok((tokens, outTree, nodeName))
}

fn import_clause(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::IMPORT.clone())?;
    (tokens, tree, b) = LAk(
        tokens,
        tree,
        list![list![TokenId::IDENT.clone()], list![TokenId::EQUALS.clone()]],
    )?;
    if b {
        (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::EQUALS.clone())?;
        (tokens, tree) = name(tokens, tree)?;
    } else {
        (tokens, tree) = name(tokens, tree)?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::STAR_EW.clone())?;
        if !(b) {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::DOT.clone())?;
            if b {
                (tokens, tree) = scan(tokens, tree, TokenId::LBRACE.clone())?;
                (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
                loop {
                    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
                    if !(b) {
                        break;
                    }
                    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::IDENT.clone())?;
                }
                (tokens, tree) = scan(tokens, tree, TokenId::RBRACE.clone())?;
            }
        }
    }
    (tokens, tree) = comment(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

use name as type_specifier;

use type_prefix as base_prefix;

fn type_prefix(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    (tokens, tree, _) = LA1(
        tokens,
        tree,
        list![TokenId::FLOW.clone(), TokenId::STREAM.clone()],
        true,
    )?;
    (tokens, tree, _) = LA1(
        tokens,
        tree,
        list![
            TokenId::DISCRETE.clone(),
            TokenId::PARAMETER.clone(),
            TokenId::CONSTANT.clone()
        ],
        true,
    )?;
    (tokens, tree, _) = LA1(
        tokens,
        tree,
        list![TokenId::INPUT.clone(), TokenId::OUTPUT.clone()],
        true,
    )?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn array_subscripts(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::LBRACK.clone())?;
    (tokens, tree) = subscript(tokens, tree)?;
    loop {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
        if !(b) {
            break;
        }
        (tokens, tree) = subscript(tokens, tree)?;
    }
    (tokens, tree) = scan(tokens, tree, TokenId::RBRACK.clone())?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn subscript(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COLON.clone())?;
    if !(b) {
        (tokens, tree) = expression(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn component_list(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeNames: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    (tokens, tree, nodeName) = component_declaration(tokens, tree)?;
    nodeNames = metamodelica::cons(nodeName, nodeNames);
    loop {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
        if !(b) {
            break;
        }
        (tokens, tree, nodeName) = component_declaration(tokens, tree)?;
        nodeNames = metamodelica::cons(nodeName, nodeNames);
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeNames))
}

fn component_declaration(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, nodeName) = declaration(tokens, tree)?;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::IF.clone())?;
    if b {
        (tokens, tree) = expression(tokens, tree)?;
    }
    (tokens, tree) = comment(tokens, tree)?;
    outTree = makeNodePrependTree(tree.reverse(), inTree, nodeName.clone());
    Ok((tokens, outTree, nodeName))
}

fn declaration(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    nodeName = metamodelica::Own::own(__pa0);
    nodeName = parseTreeFilterWhitespace(nodeName);
    (tokens, tree, b) = LA1(tokens, tree, list![TokenId::LBRACK.clone()], false)?;
    if b {
        (tokens, tree) = array_subscripts(tokens, tree)?;
    }
    (tokens, tree, b) = LA1(tokens, tree, First::modification.clone(), false)?;
    if b {
        (tokens, tree) = modification(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn component_clause1(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    (tokens, tree) = type_prefix(tokens, tree)?;
    (tokens, tree) = type_specifier(tokens, tree)?;
    (tokens, tree, nodeName) = component_declaration1(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn component_declaration1(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    (tokens, tree, nodeName) = declaration(tokens, tree)?;
    (tokens, tree) = comment(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn extends_clause(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::EXTENDS.clone())?;
    (tokens, tree) = name(tokens, tree)?;
    (tokens, tree, b) = LA1(tokens, tree, First::class_modification.clone(), false)?;
    if b {
        (tokens, tree) = class_modification(tokens, tree)?;
    }
    (tokens, tree, b) = LA1(tokens, tree, First::_annotation.clone(), false)?;
    if b {
        (tokens, tree) = _annotation(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn class_modification(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::LPAR.clone())?;
    (tokens, tree, b) = LA1(tokens, tree, First::argument.clone(), false)?;
    if b {
        (tokens, tree) = argument_list(tokens, tree)?;
    }
    (tokens, tree) = scan(tokens, tree, TokenId::RPAR.clone())?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn argument_list(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    (tokens, tree, nodeName) = argument(tokens, tree)?;
    b = true;
    while b {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
        if b {
            (tokens, tree, nodeName) = argument(tokens, tree)?;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn argument(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut node: metamodelica::Ref<ParseTree>;
    (tokens, tree, b) = LA1(tokens, tree, list![TokenId::REDECLARE.clone()], false)?;
    if b {
        (tokens, tree, nodeName) = element_redeclaration(tokens, tree)?;
    } else {
        (tokens, tree, nodeName) = element_modification_or_replaceable(tokens, tree)?;
    }
    node = makeNode(tree.reverse(), nodeName.clone());
    outTree = metamodelica::cons(node, inTree);
    Ok((tokens, outTree, nodeName))
}

fn element_redeclaration(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::REDECLARE.clone())?;
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::EACH.clone())?;
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::FINAL.clone())?;
    (tokens, tree, b) = LA1(tokens, tree, list![TokenId::REPLACEABLE.clone()], false)?;
    if b {
        (tokens, tree, nodeName) = element_replaceable(tokens, tree)?;
    } else {
        (tokens, tree, b) = LA1(tokens, tree, First::class_prefixes.clone(), false)?;
        if b {
            (tokens, tree, nodeName) = short_class_definition(tokens, tree)?;
        } else {
            (tokens, tree, nodeName) = component_clause1(tokens, tree)?;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn short_class_definition(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    (tokens, tree) = class_prefixes(tokens, tree)?;
    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    nodeName = metamodelica::Own::own(__pa0);
    nodeName = parseTreeFilterWhitespace(nodeName);
    (tokens, tree) = scan(tokens, tree, TokenId::EQUALS.clone())?;
    (tokens, tree) = short_class_specifier1(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn element_modification_or_replaceable(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::EACH.clone())?;
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::FINAL.clone())?;
    (tokens, tree, b) = LA1(tokens, tree, list![TokenId::REPLACEABLE.clone()], false)?;
    if b {
        (tokens, tree, nodeName) = element_replaceable(tokens, tree)?;
    } else {
        (tokens, tree, nodeName) = element_modification(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn element_replaceable(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::REPLACEABLE.clone())?;
    (tokens, tree, b) = LA1(tokens, tree, First::component_clause.clone(), false)?;
    if b {
        (tokens, tree, nodeName) = component_clause1(tokens, tree)?;
    } else {
        (tokens, tree, nodeName) = short_class_definition(tokens, tree)?;
    }
    (tokens, tree, b) = LA1(tokens, tree, list![TokenId::CONSTRAINEDBY.clone()], false)?;
    if b {
        (tokens, tree) = constraining_clause(tokens, tree)?;
        (tokens, tree) = comment(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn element_modification(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodeName: metamodelica::Ref<ParseTree>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = name(tokens, tree)?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    nodeName = metamodelica::Own::own(__pa0);
    nodeName = parseTreeFilterWhitespace(nodeName);
    (tokens, tree, b) = LA1(tokens, tree, First::modification.clone(), false)?;
    if b {
        (tokens, tree) = modification(tokens, tree)?;
    }
    (tokens, tree) = string_comment(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree, nodeName))
}

fn modification(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = LA1(tokens, tree, First::class_modification.clone(), false)?;
    if b {
        (tokens, tree) = class_modification(tokens, tree)?;
        (tokens, tree) = eatWhitespace(tokens, tree)?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::EQUALS.clone())?;
        (tokens, tree) = eatWhitespace(tokens, tree)?;
        if b {
            (tokens, tree) = expression(tokens, tree)?;
        }
    } else {
        (tokens, tree) = eatWhitespace(tokens, tree)?;
        (tokens, tree) = scanOneOf(tokens, tree, list![TokenId::EQUALS.clone(), TokenId::ASSIGN.clone()])?;
        (tokens, tree) = eatWhitespace(tokens, tree)?;
        (tokens, tree) = expression(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn expression_list(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    loop {
        (tokens, tree) = expression(tokens, tree)?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
        if !(b) {
            break;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn expression(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut ifTrees: metamodelica::List<metamodelica::Ref<ParseTree>> = inTree.clone();
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::IF.clone())?;
    if b {
        (tokens, tree) = expression(tokens, tree)?;
        ifTrees = listAppend(
            makeNodePrependTree(
                tree.reverse(),
                metamodelica::nil(),
                metamodelica::Ref::new(ParseTree::LEAF {
                    token: makeToken(TokenId::IDENT.clone(), literal!("$if_cond")),
                }),
            ),
            ifTrees,
        );
        tree = metamodelica::nil();
        (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
        (tokens, tree) = expression(tokens, tree)?;
        ifTrees = listAppend(
            makeNodePrependTree(
                tree.reverse(),
                metamodelica::nil(),
                metamodelica::Ref::new(ParseTree::LEAF {
                    token: makeToken(TokenId::IDENT.clone(), literal!("$then")),
                }),
            ),
            ifTrees,
        );
        tree = metamodelica::nil();
        loop {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::ELSEIF.clone())?;
            if !(b) {
                break;
            }
            (tokens, tree) = expression(tokens, tree)?;
            (tokens, tree) = scan(tokens, tree, TokenId::THEN.clone())?;
            (tokens, tree) = expression(tokens, tree)?;
            ifTrees = listAppend(
                makeNodePrependTree(
                    tree.reverse(),
                    metamodelica::nil(),
                    metamodelica::Ref::new(ParseTree::LEAF {
                        token: makeToken(TokenId::IDENT.clone(), literal!("$else_if")),
                    }),
                ),
                ifTrees,
            );
            tree = metamodelica::nil();
        }
        (tokens, tree) = scan(tokens, tree, TokenId::ELSE.clone())?;
        (tokens, tree) = expression(tokens, tree)?;
        ifTrees = listAppend(
            makeNodePrependTree(
                tree.reverse(),
                metamodelica::nil(),
                metamodelica::Ref::new(ParseTree::LEAF {
                    token: makeToken(TokenId::IDENT.clone(), literal!("$else")),
                }),
            ),
            ifTrees,
        );
        tree = metamodelica::nil();
        outTree = makeNodePrependTree(
            metamodelica::nil(),
            ifTrees,
            crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        );
        return Ok((tokens, outTree));
    }
    (tokens, tree) = simple_expression(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn simple_expression(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = logical_expression(tokens, tree)?;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COLON.clone())?;
    if b {
        (tokens, tree) = logical_expression(tokens, tree)?;
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COLON.clone())?;
        if b {
            (tokens, tree) = logical_expression(tokens, tree)?;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn logical_expression(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = logical_term(tokens, tree)?;
    loop {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::OR.clone())?;
        if !(b) {
            break;
        }
        (tokens, tree) = logical_term(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn logical_term(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = logical_factor(tokens, tree)?;
    loop {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::AND.clone())?;
        if !(b) {
            break;
        }
        (tokens, tree) = logical_factor(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn logical_factor(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::NOT.clone())?;
    (tokens, tree) = relation(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn relation(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let rel_op: metamodelica::List<TokenId> = list![
        TokenId::LESS.clone(),
        TokenId::LESSEQ.clone(),
        TokenId::GREATER.clone(),
        TokenId::GREATEREQ.clone(),
        TokenId::EQEQ.clone(),
        TokenId::LESSGT.clone()
    ];
    (tokens, tree) = arithmetic_expression(tokens, tree)?;
    loop {
        (tokens, tree, b) = LA1(tokens, tree, rel_op.clone(), true)?;
        if !(b) {
            break;
        }
        (tokens, tree) = arithmetic_expression(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn arithmetic_expression(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let add_op: metamodelica::List<TokenId> = list![
        TokenId::PLUS.clone(),
        TokenId::MINUS.clone(),
        TokenId::PLUS_EW.clone(),
        TokenId::MINUS_EW.clone()
    ];
    (tokens, tree, _) = LA1(tokens, tree, add_op.clone(), true)?;
    (tokens, tree) = term(tokens, tree)?;
    loop {
        (tokens, tree, b) = LA1(tokens, tree, add_op.clone(), true)?;
        if !(b) {
            break;
        }
        (tokens, tree) = term(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn term(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mul_op: metamodelica::List<TokenId> = list![
        TokenId::STAR.clone(),
        TokenId::STAR_EW.clone(),
        TokenId::SLASH.clone(),
        TokenId::SLASH_EW.clone()
    ];
    (tokens, tree) = factor(tokens, tree)?;
    loop {
        (tokens, tree, b) = LA1(tokens, tree, mul_op.clone(), true)?;
        if !(b) {
            break;
        }
        (tokens, tree) = factor(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn factor(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let pow_op: metamodelica::List<TokenId> = list![TokenId::POWER.clone(), TokenId::POWER_EW.clone()];
    (tokens, tree) = primary(tokens, tree)?;
    loop {
        (tokens, tree, b) = LA1(tokens, tree, pow_op.clone(), true)?;
        if !(b) {
            break;
        }
        (tokens, tree) = primary(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn primary(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    let mut label: ArcStr = literal!("expression");
    (tokens, tree, b) = LA1(
        tokens,
        tree,
        list![
            TokenId::UNSIGNED_INTEGER.clone(),
            TokenId::UNSIGNED_REAL.clone(),
            TokenId::FALSE.clone(),
            TokenId::TRUE.clone(),
            TokenId::END.clone(),
            TokenId::STRING.clone()
        ],
        false,
    )?;
    if b {
        (tokens, tree) = consume(tokens, tree)?;
        outTree = makeNodePrependTree(
            tree.reverse(),
            inTree,
            crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        );
        return Ok((tokens, outTree));
    }
    (tokens, tree, id) = peek(tokens, tree)?;
    if id == TokenId::LPAR.clone() {
        (tokens, tree) = output_expression_list(tokens, tree)?;
        label = literal!("$parenthesis");
    } else if id == TokenId::LBRACE.clone() {
        (tokens, tree) = scan(tokens, tree, TokenId::LBRACE.clone())?;
        (tokens, tree, b) = LA1(tokens, tree, list![TokenId::RBRACE.clone()], false)?;
        if !(b) {
            (tokens, tree) = function_arguments(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::RBRACE.clone())?;
        label = literal!("$array");
    } else if id == TokenId::LBRACK.clone() {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree) = expression_list(tokens, tree)?;
        loop {
            (tokens, tree, b) = scanOpt(tokens, tree, TokenId::SEMICOLON.clone())?;
            if !(b) {
                break;
            }
            (tokens, tree) = expression_list(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::RBRACK.clone())?;
        label = literal!("$matrix");
    } else if listMember(id, list![TokenId::DER.clone(), TokenId::INITIAL.clone()]) {
        (tokens, tree) = consume(tokens, tree)?;
        (tokens, tree, b) = LA1(tokens, tree, list![TokenId::LPAR.clone()], false)?;
        if b {
            (tokens, tree) = function_call_args(tokens, tree)?;
        }
        label = literal!("$initial");
    } else if listMember(
        id,
        list![TokenId::DOT.clone(), TokenId::IDENT.clone(), TokenId::FUNCTION.clone()],
    ) {
        if id == TokenId::FUNCTION.clone() {
            (tokens, tree) = consume(tokens, tree)?;
        }
        (tokens, tree) = component_reference(tokens, tree)?;
        (tokens, tree, b) = LA1(tokens, tree, list![TokenId::LPAR.clone()], false)?;
        if b {
            (tokens, tree) = function_call_args(tokens, tree)?;
            label = literal!("$call");
        }
    } else {
        error(tokens.clone(), tree.clone(), metamodelica::nil())?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        metamodelica::Ref::new(ParseTree::LEAF {
            token: makeToken(TokenId::IDENT.clone(), label),
        }),
    );
    Ok((tokens, outTree))
}

fn function_call_args(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::LPAR.clone())?;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::RPAR.clone())?;
    (tokens, tree) = eatWhitespace(tokens, tree)?;
    if !(b) {
        (tokens, tree) = function_arguments(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::RPAR.clone())?;
    }
    (tokens, tree) = eatWhitespace(tokens, tree)?;
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn function_arguments(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    let mut tree2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut trees: metamodelica::List<metamodelica::List<metamodelica::Ref<ParseTree>>>;
    trees = metamodelica::nil();
    loop {
        (tokens, tree, b) = LAk(
            tokens,
            tree,
            list![list![TokenId::IDENT.clone()], list![TokenId::EQUALS.clone()]],
        )?;
        if b {
            (tokens, tree) = named_arguments(tokens, tree)?;
            trees = metamodelica::cons(tree, trees);
            tree = metamodelica::nil();
            break;
        } else {
            (tokens, tree) = function_argument(tokens, tree)?;
            (tokens, tree2, b) = scanOpt(tokens, metamodelica::nil(), TokenId::COMMA.clone())?;
            if b {
                (tokens, tree2) = eatWhitespace(tokens, tree2)?;
            }
            if b {
                tree = metamodelica::cons(
                    makeNode(
                        tree2.reverse(),
                        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
                    ),
                    tree,
                );
            } else {
                (tokens, tree, b) = scanOpt(tokens, tree, TokenId::FOR.clone())?;
                if b {
                    (tokens, tree) = for_indices(tokens, tree)?;
                }
                trees = metamodelica::cons(tree, trees);
                tree = metamodelica::nil();
                break;
            }
        }
    }
    outTree = inTree;
    for mut tree in &*trees.reverse() {
        let mut tree = tree.clone();
        outTree = makeNodePrependTree(
            tree.reverse(),
            outTree,
            metamodelica::Ref::new(ParseTree::LEAF {
                token: makeToken(TokenId::IDENT.clone(), literal!("function_arguments")),
            }),
        );
    }
    Ok((tokens, outTree))
}

fn function_argument(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::FUNCTION.clone())?;
    if b {
        (tokens, tree) = name(tokens, tree)?;
        (tokens, tree) = scan(tokens, tree, TokenId::LPAR.clone())?;
        (tokens, tree, b) = LA1(tokens, tree, list![TokenId::IDENT.clone()], false)?;
        if b {
            (tokens, tree) = named_arguments(tokens, tree)?;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::RPAR.clone())?;
    } else {
        (tokens, tree) = expression(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn named_arguments(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = named_argument(tokens, tree)?;
    loop {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
        if !(b) {
            break;
        }
        (tokens, tree) = named_argument(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn named_argument(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut label: metamodelica::Ref<ParseTree>;
    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    label = (tree).head().cloned()?;
    (tokens, tree) = scan(tokens, tree, TokenId::EQUALS.clone())?;
    (tokens, tree) = expression(tokens, tree)?;
    outTree = makeNodePrependTree(tree.reverse(), inTree, label);
    Ok((tokens, outTree))
}

fn for_indices(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = for_index(tokens, tree)?;
    loop {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
        if !(b) {
            break;
        }
        (tokens, tree) = for_index(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn for_index(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::IN.clone())?;
    if b {
        (tokens, tree) = expression(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn string_comment(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens, tree, TokenId::STRING.clone())?;
    while b {
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::PLUS.clone())?;
        if b {
            (tokens, tree) = scan(tokens, tree, TokenId::STRING.clone())?;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn output_expression_list(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b1: bool;
    let mut b2: bool;
    (tokens, tree) = scan(tokens, tree, TokenId::LPAR.clone())?;
    loop {
        (tokens, tree, b1) = scanOpt(tokens, tree, TokenId::COMMA.clone())?;
        (tokens, tree, b2) = scanOpt(tokens, tree, TokenId::RPAR.clone())?;
        if b2 {
            break;
        }
        if !(b1) {
            (tokens, tree) = expression(tokens, tree)?;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn name(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::DOT.clone())?;
    (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    loop {
        (tokens, tree, b) = LAk(
            tokens,
            tree,
            list![list![TokenId::DOT.clone()], list![TokenId::IDENT.clone()]],
        )?;
        if !(b) {
            break;
        }
        (tokens, tree) = scan(tokens, tree, TokenId::DOT.clone())?;
        (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn component_reference(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, _) = scanOpt(tokens, tree, TokenId::DOT.clone())?;
    loop {
        (tokens, tree) = scan(tokens, tree, TokenId::IDENT.clone())?;
        (tokens, tree, b) = LA1(tokens, tree, list![TokenId::LBRACK.clone()], false)?;
        if b {
            (tokens, tree) = array_subscripts(tokens, tree)?;
        }
        (tokens, tree, b) = scanOpt(tokens, tree, TokenId::DOT.clone())?;
        if !(b) {
            break;
        }
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn comment(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = string_comment(tokens, tree)?;
    (tokens, tree, b) = LA1(tokens, tree, First::_annotation.clone(), false)?;
    if b {
        (tokens, tree) = _annotation(tokens, tree)?;
    }
    outTree = makeNodePrependTree(
        tree.reverse(),
        inTree,
        crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    );
    Ok((tokens, outTree))
}

fn _annotation(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    tree = metamodelica::nil();
    (tokens, tree) = scan(tokens, tree, TokenId::ANNOTATION.clone())?;
    (tokens, tree) = class_modification(tokens, tree)?;
    outTree = metamodelica::cons(
        makeNode(
            tree.reverse(),
            metamodelica::Ref::new(ParseTree::LEAF {
                token: makeToken(TokenId::IDENT.clone(), literal!("annotation")),
            }),
        ),
        inTree,
    );
    Ok((tokens, outTree))
}

fn findWithin(mut tree: &metamodelica::List<metamodelica::Ref<ParseTree>>) -> Result<metamodelica::Ref<ParseTree>> {
    let mut w: metamodelica::Ref<ParseTree> = crate::SimpleModelicaParser::ParseTree::interned_EMPTY();
    let mut tok: Token;
    let mut tok2: Token;
    let mut rest: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut rest2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    w = (::match_deref::match_deref! { match tree {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ ParseTree::NODE { label: Deref @ ParseTree::LEAF { token: tok }, nodes: Deref @ metamodelica::ListNode::Cons { head: __esc_w @ Deref @ ParseTree::NODE { label: Deref @ ParseTree::LEAF { token: tok2 }, .. }, tail: __esc_rest } }, tail: __esc_rest2 } if (metamodelica::stringEq(&(tokenContent(tok.clone())?), &(literal!("$program"))) && metamodelica::stringEq(&(tokenContent(tok2.clone())?), &(literal!("$within")))) => {
            w = (*__esc_w).clone();
            rest = (*__esc_rest).clone();
            rest2 = (*__esc_rest2).clone();
            w.clone()
        },
        _ => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(w)
}

fn moveComments(
    mut t1: &metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut t2: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<metamodelica::List<metamodelica::Ref<ParseTree>>> {
    let mut t2: metamodelica::List<metamodelica::Ref<ParseTree>> = t2;
    let mut c1: metamodelica::List<(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr)>;
    let mut c2: metamodelica::List<(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr)>;
    let mut tok: Token;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    let mut path1: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut path2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tempTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    c1 = findCommentsWithLabels(t1, &(metamodelica::nil()), metamodelica::nil())?;
    c2 = findCommentsWithLabels(&t2, &(metamodelica::nil()), metamodelica::nil())?;
    (_, c1, c2) = List::intersection1OnTrue(c1, c2, &move |__a0: (
        Token,
        metamodelica::List<metamodelica::Ref<ParseTree>>,
        ArcStr,
    ),
                                                           __a1: (
        Token,
        metamodelica::List<metamodelica::Ref<ParseTree>>,
        ArcStr,
    )| foundCommentEqual(&__a0, &__a1))?;
    for mut c in &*c2 {
        if '__try0: {
            (tok, path1, str1) = c.clone();
            let ((_, __pa1, __pa2), __pa3) = unwrap_break_err!(List::findAndRemove1(c1.clone(), &move |__a0: (Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr), __a1: (Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr)| foundCommentTokenEqual(&__a0, &__a1), c.clone()), '__try0);
            path2 = metamodelica::Own::own(__pa1);
            str2 = metamodelica::Own::own(__pa2);
            c1 = metamodelica::Own::own(__pa3);
            let __pa4 = ::match_deref::match_deref! { match &(unwrap_break_err!(removeCommentAtLabelPath(t2.clone(), &tok, &(path1.clone().reverse())), '__try0)) {
                (__pa4, true) => __pa4.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            tempTree = metamodelica::Own::own(__pa4);
            let __pa5 = ::match_deref::match_deref! { match &(unwrap_break_err!(addCommentAtLabelPath(tempTree.clone(), tok.clone(), &(path2.clone().reverse())), '__try0)) {
                (__pa5, true) => __pa5.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            tempTree = metamodelica::Own::own(__pa5);
            t2 = tempTree.clone();
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    Ok(t2)
}

fn moveCommentsAfterDiff(
    mut res: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut res: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)> = res;
    let mut foundComment: ArcStr;
    let mut tree: metamodelica::Ref<ParseTree>;
    let mut foundTree: metamodelica::Ref<ParseTree>;
    let mut trees: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut before: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut after: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut acc2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut comments: metamodelica::Ref<AvlSetString::Tree>;
    let mut acc: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut lst: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut diff: (Diff, metamodelica::List<metamodelica::Ref<ParseTree>>);
    let mut found: bool;
    comments = findAddedComments(&res)?;
    acc = metamodelica::nil();
    lst = res.clone();
    if AvlSetString::isEmpty(&comments) {
        return Ok(res);
    }
    while !((lst).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        diff = metamodelica::Own::own(__pa0);
        lst = metamodelica::Own::own(__pa1);
        let () = (::match_deref::match_deref! { match &(&diff) {
            (DiffAlgorithm::Diff::Delete, __esc_trees) => {
                trees = (*__esc_trees).clone();
                acc2 = metamodelica::nil();
                while !((trees).is_empty()) {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(trees.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    tree = metamodelica::Own::own(__pa0);
                    trees = metamodelica::Own::own(__pa1);
                    (found, before, foundTree, after, foundComment) = fixDeletedComments(tree.clone(), &comments)?;
                    if found {
                        acc = metamodelica::cons((Diff::Delete.clone(), listAppend(acc2.reverse(), before)), acc);
                        res = listAppend(acc.reverse(), metamodelica::cons((Diff::Equal.clone(), list![foundTree]), metamodelica::cons((Diff::Delete.clone(), listAppend(after, trees.clone())), lst)));
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(res.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static>))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        res = removeAddedCommentFromDiff(res, &foundComment)?;
                        res = moveCommentsAfterDiff(res)?;
                        return Ok(res);
                    }
                    acc2 = metamodelica::cons(tree, acc2);
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        acc = metamodelica::cons(diff, acc);
    }
    Ok(res)
}

fn findAddedComments(
    mut tree: &metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut comments: metamodelica::Ref<AvlSetString::Tree> = openmodelica_util::AvlSetString::Tree::interned_EMPTY();
    let mut addedTrees: metamodelica::List<metamodelica::Ref<ParseTree>>;
    (addedTrees, _) = extractAdditionsDeletions(tree)?;
    for mut t in &*addedTrees {
        comments = findAddedComments2(metamodelica::AsArg::as_arg(&t), comments)?;
    }
    Ok(comments)
}

fn findAddedComments2(
    mut tree: &metamodelica::Ref<ParseTree>,
    mut comments: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut comments: metamodelica::Ref<AvlSetString::Tree> = comments;
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    comments = (match &**tree {
        ParseTree::LEAF { token: __tree_token } if (parseTreeIsComment(tree)) => {
            AvlSetString::add(comments, &(tokenContent(__tree_token.clone())?))?
        }
        ParseTree::NODE { nodes: __esc_nodes, .. } => {
            nodes = (*__esc_nodes).clone();
            for mut n in &*nodes.clone() {
                comments = findAddedComments2(metamodelica::AsArg::as_arg(&n), comments)?;
            }
            comments
        }
        _ => comments,
    });
    Ok(comments)
}

fn removeAddedCommentFromDiff(
    mut tree: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
    mut comment: &ArcStr,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut tree: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)> = tree;
    let mut acc: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut lst: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut diff: (Diff, metamodelica::List<metamodelica::Ref<ParseTree>>);
    let mut lst2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut b: bool;
    lst = tree.clone();
    acc = metamodelica::nil();
    while !((lst).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        diff = metamodelica::Own::own(__pa0);
        lst = metamodelica::Own::own(__pa1);
        let () = (::match_deref::match_deref! { match &(&diff) {
            (DiffAlgorithm::Diff::Add, __esc_lst2) => {
                lst2 = (*__esc_lst2).clone();
                (b, lst2) = removeAddedCommentFromDiff2(lst2.clone(), comment)?;
                if b {
                    tree = listAppend(acc.reverse(), metamodelica::cons((Diff::Add.clone(), lst2.clone()), lst));
                    return Ok(tree);
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        acc = metamodelica::cons(diff, acc);
    }
    Error::addInternalError(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Failed to remove comment `"));
            __mm_s.push_str(&*comment);
            __mm_s.push_str(&*literal!("` from diff; but we know it is in there somewhere"));
            ArcStr::from(__mm_s)
        },
        metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"),
    )?;
    Ok(tree)
}

fn removeAddedCommentFromDiff2(
    mut trees: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut comment: &ArcStr,
) -> Result<(bool, metamodelica::List<metamodelica::Ref<ParseTree>>)> {
    let mut removed: bool = false;
    let mut trees: metamodelica::List<metamodelica::Ref<ParseTree>> = trees;
    let mut acc: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut lst: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::Ref<ParseTree>;
    let mut content: ArcStr;
    acc = metamodelica::nil();
    lst = trees.clone();
    while !((lst).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        tree = metamodelica::Own::own(__pa0);
        lst = metamodelica::Own::own(__pa1);
        (removed, tree) = (match &*tree {
            ParseTree::LEAF { token: __tree_token } if (parseTreeIsComment(&tree)) => {
                content = tokenContent(__tree_token.clone())?;
                (
                    metamodelica::stringEq(&content, &comment),
                    if (metamodelica::stringEq(&content, &comment)) {
                        crate::SimpleModelicaParser::ParseTree::interned_EMPTY()
                    } else {
                        tree.clone()
                    },
                )
            }
            ParseTree::NODE { nodes: __esc_nodes, .. } => {
                nodes = (*__esc_nodes).clone();
                (removed, nodes) = removeAddedCommentFromDiff2(nodes.clone(), comment)?;
                if removed {
                    assign_variant_field!(tree => ParseTree::NODE; nodes = nodes.clone());
                }
                (removed, tree.clone())
            }
            _ => (false, tree.clone()),
        });
        if removed {
            lst = if (isEmpty(&tree)) {
                lst
            } else {
                metamodelica::cons(tree, lst)
            };
            lst = listAppend(acc.reverse(), lst);
            trees = lst;
            return Ok((removed, trees));
        }
        acc = metamodelica::cons(tree, acc);
    }
    Ok((removed, trees))
}

fn fixDeletedComments(
    mut tree: metamodelica::Ref<ParseTree>,
    mut addedComments: &metamodelica::Ref<AvlSetString::Tree>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::Ref<ParseTree>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    ArcStr,
)> {
    let mut found: bool = false;
    let mut before: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut foundTree: metamodelica::Ref<ParseTree> = tree.clone();
    let mut after: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut foundComment: ArcStr = literal!("");
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut before2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut after2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut b: bool;
    let mut t: metamodelica::Ref<ParseTree>;
    let mut content: ArcStr;
    found = (match &*tree {
        ParseTree::LEAF { token: __tree_token } if (parseTreeIsComment(&tree)) => {
            content = tokenContent(__tree_token.clone())?;
            b = AvlSetString::hasKey(addedComments.clone(), content.clone())?;
            if b {
                foundComment = content;
            }
            b
        }
        ParseTree::NODE { nodes: __esc_nodes, .. } => {
            nodes = (*__esc_nodes).clone();
            while !((nodes).is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                t = metamodelica::Own::own(__pa0);
                nodes = metamodelica::Own::own(__pa1);
                (found, before2, foundTree, after2, foundComment) = fixDeletedComments(t.clone(), addedComments)?;
                if found {
                    before = listAppend(before.reverse(), before2);
                    after = listAppend(after2, nodes.clone());
                    return Ok((found, before, foundTree, after, foundComment));
                }
                before = metamodelica::cons(t, before);
            }
            before = metamodelica::nil();
            foundTree = tree.clone();
            after = metamodelica::nil();
            foundComment = literal!("");
            false
        }
        _ => false,
    });
    Ok((found, before, foundTree, after, foundComment))
}

fn addCommentAtLabelPath(
    mut tree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut tok: Token,
    mut path: &metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(metamodelica::List<metamodelica::Ref<ParseTree>>, bool)> {
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = tree;
    let mut success: bool = false;
    let mut n: metamodelica::Ref<ParseTree>;
    let mut n2: metamodelica::Ref<ParseTree>;
    let mut label: metamodelica::Ref<ParseTree>;
    let mut pathFirst: metamodelica::Ref<ParseTree>;
    let mut rest: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut pathRest: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut delst: DoubleEnded::MutableList<metamodelica::Ref<ParseTree>>;
    let mut b: bool;
    if (path).is_empty() {
        success = true;
        tree = metamodelica::cons(metamodelica::Ref::new(ParseTree::LEAF { token: tok }), tree);
        return Ok((tree, success));
    }
    delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
    rest = tree.clone();
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        n = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        (n2, b) = (::match_deref::match_deref! { match &((n.clone(), path.clone())) {
            (Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, .. }, _) => {
                (nodes, b) = addCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok.clone(), path)?;
                if b {
                    n2 = metamodelica::Ref::new(ParseTree::NODE { label: crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), nodes: nodes });
                } else {
                    n2 = n;
                }
                (n2, b)
            },
            (Deref @ ParseTree::NODE { label, .. }, Deref @ metamodelica::ListNode::Cons { head: pathFirst, tail: __esc_pathRest }) if (stringEq(&(labelPathStr(list![label.clone()])?), &(labelPathStr(list![pathFirst.clone()])?))) => {
                pathRest = (*__esc_pathRest).clone();
                (nodes, b) = addCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok.clone(), metamodelica::AsArg::as_arg(&pathRest))?;
                if b {
                    n2 = metamodelica::Ref::new(ParseTree::NODE { label: label.clone(), nodes: nodes });
                } else {
                    n2 = n;
                }
                (n2, b)
            },
            _ => (n, false),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        DoubleEnded::push_back(delst.clone(), n2)?;
        if b {
            tree = DoubleEnded::toListAndClear(delst, rest)?;
            success = true;
            return Ok((tree, success));
        }
    }
    Ok((tree, success))
}

fn removeCommentAtLabelPath(
    mut tree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut tok: &Token,
    mut path: &metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(metamodelica::List<metamodelica::Ref<ParseTree>>, bool)> {
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = tree;
    let mut success: bool = false;
    let mut n: metamodelica::Ref<ParseTree>;
    let mut n2: metamodelica::Ref<ParseTree>;
    let mut label: metamodelica::Ref<ParseTree>;
    let mut pathFirst: metamodelica::Ref<ParseTree>;
    let mut rest: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut pathRest: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut delst: DoubleEnded::MutableList<metamodelica::Ref<ParseTree>>;
    let mut b: bool;
    if (path).is_empty() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(removeCommentAtThisLabel(tree, tok)?) {
            (__pa0, __pa1 @ true) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        tree = metamodelica::Own::own(__pa0);
        success = metamodelica::Own::own(__pa1);
        return Ok((tree, success));
    }
    delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
    rest = tree.clone();
    while !((rest).is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        n = metamodelica::Own::own(__pa2);
        rest = metamodelica::Own::own(__pa3);
        (n2, b) = (::match_deref::match_deref! { match &((n.clone(), path.clone())) {
            (Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, .. }, _) => {
                (nodes, b) = removeCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok, path)?;
                if b {
                    n2 = metamodelica::Ref::new(ParseTree::NODE { label: crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), nodes: nodes });
                } else {
                    n2 = n;
                }
                (n2, b)
            },
            (Deref @ ParseTree::NODE { label, .. }, Deref @ metamodelica::ListNode::Cons { head: pathFirst, tail: __esc_pathRest }) if (stringEq(&(labelPathStr(list![label.clone()])?), &(labelPathStr(list![pathFirst.clone()])?))) => {
                pathRest = (*__esc_pathRest).clone();
                (nodes, b) = removeCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok, metamodelica::AsArg::as_arg(&pathRest))?;
                if b {
                    n2 = metamodelica::Ref::new(ParseTree::NODE { label: label.clone(), nodes: nodes });
                } else {
                    n2 = n;
                }
                (n2, b)
            },
            _ => (n, false),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        DoubleEnded::push_back(delst.clone(), n2)?;
        if b {
            tree = DoubleEnded::toListAndClear(delst, rest)?;
            success = true;
            return Ok((tree, success));
        }
    }
    Ok((tree, success))
}

fn removeCommentAtThisLabel(
    mut tree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut tok: &Token,
) -> Result<(metamodelica::List<metamodelica::Ref<ParseTree>>, bool)> {
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = tree;
    let mut success: bool = false;
    let mut delst: DoubleEnded::MutableList<metamodelica::Ref<ParseTree>>;
    let mut rest: metamodelica::List<metamodelica::Ref<ParseTree>> = tree.clone();
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut n: metamodelica::Ref<ParseTree>;
    delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        n = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        let () = (::match_deref::match_deref! { match &(&*n) {
            Deref @ ParseTree::LEAF { token: __n_token } if (modelicaDiffTokenEq(__n_token.clone(), tok.clone())?) => {
                success = true;
                tree = DoubleEnded::toListAndClear(delst, rest)?;
                return Ok((tree, success));
                return Err("fail")
            },
            Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, nodes: __n_nodes } => {
                (nodes, success) = removeCommentAtThisLabel(__n_nodes.clone(), tok)?;
                if success {
                    DoubleEnded::push_back(delst.clone(), metamodelica::Ref::new(ParseTree::NODE { label: crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), nodes: nodes }))?;
                    tree = DoubleEnded::toListAndClear(delst, rest)?;
                    return Ok((tree, success));
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        DoubleEnded::push_back(delst.clone(), n)?;
    }
    Ok((tree, success))
}

fn findCommentsWithLabels(
    mut t1: &metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut labelPath: &metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut acc: metamodelica::List<(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr)>,
) -> Result<metamodelica::List<(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr)>> {
    let mut acc: metamodelica::List<(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr)> = acc;
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tok: Token;
    let mut id: TokenId;
    let mut pathStr: ArcStr;
    for mut n in &**t1 {
        let () = (::match_deref::match_deref! { match &(n.clone()) {
            Deref @ ParseTree::EMPTY { .. } => (),
            Deref @ ParseTree::LEAF { token: __esc_tok @ Token { id: __esc_id, .. } } if (parseTreeIsComment(metamodelica::AsArg::as_arg(&n))) => {
                tok = (*__esc_tok).clone();
                id = (*__esc_id).clone();
                pathStr = labelPathStr(labelPath.clone())?;
                acc = metamodelica::cons((tok.clone(), labelPath.clone(), pathStr), acc);
                ()
            },
            Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, nodes: __esc_nodes } => {
                nodes = (*__esc_nodes).clone();
                acc = findCommentsWithLabels(metamodelica::AsArg::as_arg(&nodes), labelPath, acc)?;
                ()
            },
            Deref @ ParseTree::NODE { nodes: __esc_nodes, label: __n_label } => {
                nodes = (*__esc_nodes).clone();
                acc = findCommentsWithLabels(metamodelica::AsArg::as_arg(&nodes), &(metamodelica::cons(__n_label.clone(), labelPath.clone())), acc)?;
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(acc)
}

fn foundCommentEqual(
    mut c1: &(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr),
    mut c2: &(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr),
) -> Result<bool> {
    let mut eq: bool;
    let mut tok1: Token;
    let mut tok2: Token;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    (tok1, _, s1) = c1.clone();
    (tok2, _, s2) = c2.clone();
    eq = modelicaDiffTokenEq(tok1, tok2)?;
    if !(eq) {
        return Ok(eq);
    }
    eq = stringEq(&s1, &s2);
    Ok(eq)
}

fn foundCommentTokenEqual(
    mut c1: &(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr),
    mut c2: &(Token, metamodelica::List<metamodelica::Ref<ParseTree>>, ArcStr),
) -> Result<bool> {
    let mut eq: bool;
    let mut tok1: Token;
    let mut tok2: Token;
    (tok1, _, _) = c1.clone();
    (tok2, _, _) = c2.clone();
    eq = modelicaDiffTokenEq(tok1, tok2)?;
    Ok(eq)
}

fn labelPathStr(mut labelPath: metamodelica::List<metamodelica::Ref<ParseTree>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t in (labelPath).into_iter().cloned() {
                let __x = parseTreeStr(&(list![t.clone()]))?;
                __acc = cons(__x, __acc);
            }
            __acc
        }),
        literal!("."),
    );
    Ok(r#str)
}

fn treeDiffWork1(
    mut t1: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut t2: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut nTokens: i32,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut res: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut diffSubtreeWorkArray1: metamodelica::Array<Token>;
    let mut diffSubtreeWorkArray2: metamodelica::Array<Token>;
    if (t1).is_empty() {
        res = list![(Diff::Add.clone(), t2)];
        return Ok(res);
    } else if (t2).is_empty() {
        res = list![(Diff::Delete.clone(), t1)];
        return Ok(res);
    }
    diffSubtreeWorkArray1 = metamodelica::arrayCreate(nTokens, LexerModelicaDiff::noToken.clone());
    diffSubtreeWorkArray2 = metamodelica::arrayCreate(nTokens, LexerModelicaDiff::noToken.clone());
    if parseTreeEq(
        makeNode(t1.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()),
        makeNode(t2.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()),
        diffSubtreeWorkArray1.clone(),
        diffSubtreeWorkArray2.clone(),
    )? {
        res = list![(Diff::Equal.clone(), t1)];
        return Ok(res);
    }
    res = treeDiffWork(
        t1,
        t2,
        1,
        (std::sync::Arc::new({
            let __pe_b2 = diffSubtreeWorkArray1.clone();
            let __pe_b3 = diffSubtreeWorkArray2.clone();
            move |__pe_a0, __pe_a1| parseTreeEq(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>, metamodelica::Ref<ParseTree>) -> Result<bool>
                    + 'static,
            >),
    )?;
    Ok(res)
}

fn treeDiffWork(
    mut t1: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut t2: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut depth: i32,
    mut compare: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>, metamodelica::Ref<ParseTree>) -> Result<bool> + 'static,
    >,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut res: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut resLocal: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut t2_strip: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut before: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut middle: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut after: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut addedTrees: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut deletedTrees: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut ts: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut addList: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut delList: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut nadd: i32;
    let mut ndel: i32;
    let mut addedTree: metamodelica::Ref<ParseTree>;
    let mut deletedTree: metamodelica::Ref<ParseTree>;
    let mut deleted: metamodelica::Ref<ParseTree> = crate::SimpleModelicaParser::ParseTree::interned_EMPTY();
    let mut addedBeforeDeleted: bool;
    let mut joinTrees: bool;
    let mut tryFind: bool;
    let mut r#str: ArcStr;
    let mut debugString1: ArcStr = literal!("");
    let mut debugString2: ArcStr = literal!("");
    let mut d: Diff;
    let () = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_before, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_after, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            before = (*__esc_before).clone();
            after = (*__esc_after).clone();
            res = treeDiffWork(before.clone(), after.clone(), depth, compare.clone())?;
            return Ok(res);
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_before, .. }, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
            before = (*__esc_before).clone();
            res = treeDiffWork(before.clone(), t2, depth, compare.clone())?;
            return Ok(res);
            ()
        },
        (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_after, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            after = (*__esc_after).clone();
            res = treeDiffWork(t1, after.clone(), depth, compare.clone())?;
            return Ok(res);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if parseTreeIsNewLine(&((t2).head().cloned()?)) {
        t2_strip = (t2).rest()?;
    } else {
        t2_strip = t2.clone();
    }
    if debug.clone() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Do diff at depth="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", depth)));
            __mm_s.push_str(&*literal!(", len(t1)="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ((t1).len() as i32))));
            __mm_s.push_str(&*literal!(", len(t2)="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ((t2).len() as i32))));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("top t1="));
            __mm_s.push_str(&*firstTokenDebugStr(t1.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("top t2="));
            __mm_s.push_str(&*firstTokenDebugStr(t2.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("all t1="));
            __mm_s.push_str(&*parseTreeStr(&t1)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("all t2="));
            __mm_s.push_str(&*parseTreeStr(&t2)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    res = diff(
        t1,
        t2,
        &*compare,
        &move |__a0: metamodelica::Ref<ParseTree>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(parseTreeIsWhitespace(&__a0))
        },
        &move |__a0: metamodelica::Ref<ParseTree>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(parseTreeIsWhitespaceNotComment(&__a0))
        },
        &move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0),
    )?;
    (nadd, ndel) = countDiffAddDelete(&res);
    if nadd > 1 {
        res = fixMoveOperations(res, &*compare)?;
        (nadd, ndel) = countDiffAddDelete(&res);
    }
    res = filterDiffWhitespace(res)?;
    if debug.clone() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("nadd: "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", nadd)));
            __mm_s.push_str(&*literal!(" ndel: "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ndel)));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(
                res.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static>),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        if nadd != ndel {
            for mut r in &*res {
                (d, ts) = r.clone();
                if d == Diff::Equal.clone() {
                    continue;
                }
                for mut t in &*ts {
                    if isLabeledNode(metamodelica::AsArg::as_arg(&t)) {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", d)));
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*parseTreeStr(
                                &(metamodelica::cons(nodeLabel(metamodelica::AsArg::as_arg(&t)), metamodelica::nil())),
                            )?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                }
            }
        }
    }
    if depth > 300 {
    } else if nadd == 1 && ndel == 1 {
        (addedTree, deletedTree, before, middle, after, addedBeforeDeleted) = extractSingleAddDiffBeforeAndAfter(&res)?;
        if if (!((middle).is_empty())) {
            ({
                let mut __acc: Option<bool> = None;
                for mut middleItem in (middle.clone()).into_iter().cloned() {
                    let __x = parseTreeIsWhitespace(&(middleItem.clone()));
                    __acc = Some(match __acc {
                        None => __x,
                        Some(__cur) => {
                            if __x < __cur {
                                __x
                            } else {
                                __cur
                            }
                        }
                    });
                }
                __acc.unwrap_or(true)
            })
        } else {
            false
        } {
            if addedBeforeDeleted {
                before = listAppend(before, middle);
            } else {
                after = listAppend(middle, after);
            }
            middle = metamodelica::nil();
        }
        joinTrees = true;
        if compare(addedTree.clone(), deletedTree.clone())? {
            res = list![(Diff::Equal.clone(), list![deletedTree.clone()])];
        } else if isLeaf(&deletedTree) && isLeaf(&addedTree) {
            res = res;
            joinTrees = false;
        } else if (before).is_empty() && (after).is_empty() {
            if debug.clone() {
                metamodelica::print(literal!("before and after empty\n"));
            }
            res = res;
        } else {
            res = treeDiffWork(
                getNodes(deletedTree.clone()),
                getNodes(addedTree.clone()),
                depth + 1,
                compare.clone(),
            )?;
        }
        if !(joinTrees) {
            res = res;
            if debug.clone() {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("not joining trees"));
                    __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(
                        res.clone(),
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static,
                            >),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        } else if (middle).is_empty() {
            if debug.clone() {
                metamodelica::print(literal!("middle empty\n"));
            }
            res = metamodelica::cons(
                (Diff::Equal.clone(), before.clone()),
                listAppend(res, list![(Diff::Equal.clone(), after.clone())]),
            );
        } else {
            res = ({
                let mut __acc: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)> =
                    metamodelica::nil();
                for mut i in (res).into_iter().cloned() {
                    if !(::match_deref::match_deref! { match &(i.clone()) {
                        (DiffAlgorithm::Diff::Delete, _) => false,
                        _ => true,
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } }) {
                        continue;
                    }
                    let __x = i.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if addedBeforeDeleted {
                res = metamodelica::cons(
                    (Diff::Equal.clone(), before.clone()),
                    listAppend(
                        res,
                        metamodelica::cons(
                            (Diff::Equal.clone(), middle.clone()),
                            metamodelica::cons(
                                (Diff::Delete.clone(), list![deletedTree.clone()]),
                                list![(Diff::Equal.clone(), after.clone())],
                            ),
                        ),
                    ),
                );
            } else {
                res = metamodelica::cons(
                    (Diff::Equal.clone(), before.clone()),
                    metamodelica::cons(
                        (Diff::Delete.clone(), list![deletedTree.clone()]),
                        metamodelica::cons(
                            (Diff::Equal.clone(), middle.clone()),
                            listAppend(res, list![(Diff::Equal.clone(), after.clone())]),
                        ),
                    ),
                );
            }
        }
        if debug.clone() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", depth)));
                __mm_s.push_str(&*literal!(" merged tree size: "));
                __mm_s.push_str(&*ArcStr::from(::std::format!(
                    "{}",
                    ((DiffAlgorithm::printActual(
                        res.clone(),
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static,
                            >)
                    )?)
                    .len() as i32)
                )));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", depth)));
                __mm_s.push_str(&*literal!(" before top="));
                __mm_s.push_str(&*firstTokenDebugStr(before.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" before all="));
                __mm_s.push_str(&*parseTreeStr(&before)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" middle all="));
                __mm_s.push_str(&*parseTreeStr(&middle)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" after all="));
                __mm_s.push_str(&*parseTreeStr(&after)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("middle top="));
                __mm_s.push_str(&*firstTokenDebugStr(middle)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("after top="));
                __mm_s.push_str(&*firstTokenDebugStr(after)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("added top="));
                __mm_s.push_str(&*firstTokenDebugStr(metamodelica::cons(
                    addedTree,
                    metamodelica::nil(),
                ))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("deleted top="));
                __mm_s.push_str(&*firstTokenDebugStr(metamodelica::cons(
                    deletedTree,
                    metamodelica::nil(),
                ))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
    } else if nadd > 1 && ndel > 1 {
        (addedTrees, deletedTrees) = extractAdditionsDeletions(&res)?;
        addedTrees = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
            for mut t in (addedTrees).into_iter().cloned() {
                if !(isLabeledNode(&(t.clone()))) {
                    continue;
                }
                let __x = t.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        deletedTrees = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
            for mut t in (deletedTrees).into_iter().cloned() {
                if !(isLabeledNode(&(t.clone()))) {
                    continue;
                }
                let __x = t.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if debug.clone() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("number of labeled nodes. add="));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ((addedTrees).len() as i32))));
                __mm_s.push_str(&*literal!(" del="));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ((deletedTrees).len() as i32))));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(
                    res.clone(),
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static,
                        >),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        for mut x in &*res {
            (d, ts) = x.clone();
            if d == Diff::Equal.clone() {
                continue;
            }
            addList = metamodelica::nil();
            delList = metamodelica::nil();
            for mut t in &*ts {
                if isEmpty(metamodelica::AsArg::as_arg(&t))
                    || parseTreeIsWhitespace(metamodelica::AsArg::as_arg(&t))
                    || isEmpty(&(nodeLabel(metamodelica::AsArg::as_arg(&t))))
                {
                    continue;
                }
                r#str = parseTreeStr(
                    &(metamodelica::cons(nodeLabel(metamodelica::AsArg::as_arg(&t)), metamodelica::nil())),
                )?;
                if d == Diff::Add.clone() {
                    addList = metamodelica::cons(r#str, addList);
                } else {
                    delList = metamodelica::cons(r#str, delList);
                }
            }
        }
        for mut added in &*addedTrees {
            tryFind = false;
            if '__try0: {
                (deleted, deletedTrees) = unwrap_break_err!(List::findAndRemove1(deletedTrees.clone(), &({ let __pe_b2 = compare.clone(); move |__pe_a0, __pe_a1| compareNodeLabels(&__pe_a0, &__pe_a1, &*__pe_b2) }), added.clone()), '__try0);
                Ok::<(), &'static str>(())
            }.is_err() {
                if '__try1: {
                    (deleted, deletedTrees) = unwrap_break_err!(List::findAndRemove1(deletedTrees.clone(), &({ let __pe_b2 = compare.clone(); let __pe_b3 = delList.clone(); move |__pe_a0, __pe_a1| compareNodeLabelsSpecial(&__pe_a0, &__pe_a1, &*__pe_b2, __pe_b3.clone()) }), added.clone()), '__try1);
                    Ok::<(), &'static str>(())
                }.is_err() {
                    tryFind = true;
                }
            }
            if tryFind {
                continue;
            }
            resLocal = treeDiffWork(
                getNodes(deleted.clone()),
                getNodes(added.clone()),
                depth + 1,
                compare.clone(),
            )?;
            if debug.clone() {
                debugString1 = DiffAlgorithm::printDiffTerminalColor(
                    res.clone(),
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static,
                        >),
                )?;
            }
            res = replaceLabeledDiff(
                &res,
                resLocal,
                nodeLabel(metamodelica::AsArg::as_arg(&added)),
                nodeLabel(&deleted),
                &*compare,
                labelOrderDidNotChange(&addList, delList.clone())?,
            )?;
            if debug.clone() {
                debugString2 = DiffAlgorithm::printDiffTerminalColor(
                    res.clone(),
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static,
                        >),
                )?;
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("replaceLabeledDiff change for label:"));
                    __mm_s.push_str(&*parseTreeNodeStr(&(nodeLabel(metamodelica::AsArg::as_arg(&added))))?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("before replaceLabeledDiff: "));
                    __mm_s.push_str(&*debugString1);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("after replaceLabeledDiff: "));
                    __mm_s.push_str(&*debugString2);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
    }
    if debug.clone() {
        metamodelica::print(literal!("Before filter WS\n"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*DiffAlgorithm::printDiffXml(
                res.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static>),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    res = filterDiffWhitespace(res)?;
    if debug.clone() {
        metamodelica::print(literal!("After filter WS\n"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*DiffAlgorithm::printDiffXml(
                res.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ParseTree>| parseTreeNodeStr(&__a0))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>) -> Result<ArcStr> + 'static>),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    if depth == 1 {}
    Ok(res)
}

fn compareNodeLabels(
    mut t1: &metamodelica::Ref<ParseTree>,
    mut t2: &metamodelica::Ref<ParseTree>,
    mut compare: &dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>, metamodelica::Ref<ParseTree>) -> Result<bool>,
) -> Result<bool> {
    let mut b: bool;
    b = compare(nodeLabel(t1), nodeLabel(t2))?;
    Ok(b)
}

fn compareNodeLabelsSpecial(
    mut t1: &metamodelica::Ref<ParseTree>,
    mut t2: &metamodelica::Ref<ParseTree>,
    mut compare: &dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>, metamodelica::Ref<ParseTree>) -> Result<bool>,
    mut delList: metamodelica::List<ArcStr>,
) -> Result<bool> {
    let mut b: bool;
    b = nodeLabelIsComponent(t1)
        && nodeLabelIsComponent(t2)
        && !(listMember(
            parseTreeStr(&(metamodelica::cons(nodeLabel(t1), metamodelica::nil())))?,
            delList,
        ));
    Ok(b)
}

fn nodeLabelIsComponent(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut contents: ArcStr;
    b = (match &*(nodeLabel(t1)) {
        ParseTree::LEAF {
            token:
                Token {
                    id: TokenId::IDENT { .. },
                    fileContents: __esc_contents,
                    ..
                },
        } => {
            contents = (*__esc_contents).clone();
            0 == System::strncmp(contents.clone(), literal!("$component:"), 11)
        }
        _ => false,
    });
    b
}

fn filterDiffWhitespace(
    mut inDiff: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut diff: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut diffLocal: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)> = inDiff;
    let mut diff1: (Diff, metamodelica::List<metamodelica::Ref<ParseTree>>);
    let mut diff2: (Diff, metamodelica::List<metamodelica::Ref<ParseTree>>);
    let mut diff3: (Diff, metamodelica::List<metamodelica::Ref<ParseTree>>);
    let mut diff4: (Diff, metamodelica::List<metamodelica::Ref<ParseTree>>);
    let mut firstIter: bool;
    let mut lastTokenNewline: bool;
    let mut hasAddedWS: bool;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut treeLocal: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree1: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree3: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree4: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut treeLast: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree4First: metamodelica::Ref<ParseTree>;
    let mut t1: metamodelica::Ref<ParseTree>;
    let mut t2: metamodelica::Ref<ParseTree>;
    let mut t3: metamodelica::Ref<ParseTree>;
    let mut firstTreeSecondLast: metamodelica::Ref<ParseTree>;
    let mut firstTreeLast: metamodelica::Ref<ParseTree>;
    let mut length: i32;
    let mut level: i32;
    let mut indentation: metamodelica::List<i32>;
    let mut diffEnum: Diff;
    let mut diffEnum1: Diff;
    let mut diffEnum2: Diff;
    let mut indentationStr: ArcStr;
    let mut tok: Token;
    diff = metamodelica::nil();
    firstIter = true;
    while !((diffLocal).is_empty()) {
        (diffEnum, treeLast) = (diffLocal).head().cloned()?;
        (firstTreeSecondLast, firstTreeLast) = (::match_deref::match_deref! { match &(treeLast.clone()) {
            Deref @ metamodelica::ListNode::Nil => (crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()),
            Deref @ metamodelica::ListNode::Cons { head: __esc_firstTreeLast, tail: Deref @ metamodelica::ListNode::Nil } => {
                firstTreeLast = (*__esc_firstTreeLast).clone();
                (crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), firstTreeLast.clone())
            },
            _ => {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::lastN(treeLast, 2)?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                firstTreeSecondLast = metamodelica::Own::own(__pa0);
                firstTreeLast = metamodelica::Own::own(__pa1);
                (firstTreeSecondLast, firstTreeLast)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        diffLocal = (::match_deref::match_deref! { match &(diffLocal.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: (_, Deref @ metamodelica::ListNode::Nil), tail: __esc_diffLocal } => {
                diffLocal = (*__esc_diffLocal).clone();
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, tree), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } if (if (firstIter) {({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })} else {false}) => {
                diffLocal = (*__esc_diffLocal).clone();
                diff = metamodelica::cons((Diff::Equal.clone(), tree.clone()), diff);
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, tree), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })) => {
                diff1 = (*__esc_diff1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                diff = metamodelica::cons((Diff::Equal.clone(), tree.clone()), metamodelica::cons(diff1.clone(), diff));
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, tree), tail: Deref @ metamodelica::ListNode::Nil } } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })) => {
                diff1 = (*__esc_diff1).clone();
                diff = metamodelica::cons((Diff::Equal.clone(), tree.clone()), metamodelica::cons(diff1.clone(), diff));
                metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: (_, tree), tail: __esc_diffLocal } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = isEmpty(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })) => {
                diffLocal = (*__esc_diffLocal).clone();
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1, tail: Deref @ metamodelica::ListNode::Cons { head: (_, tree), tail: __esc_diffLocal } } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = isEmpty(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })) => {
                diff1 = (*__esc_diff1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), diffLocal.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: (__esc_diffEnum, tree), tail: __esc_diffLocal } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = isEmpty(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(false)
        })) => {
                diffEnum = (*__esc_diffEnum).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons((diffEnum.clone(), ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
            for mut t in (tree.clone()).into_iter().cloned() {
                if !(!(isEmpty(&(t.clone())))) { continue; }
                let __x = t.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), diffLocal.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1, tail: Deref @ metamodelica::ListNode::Cons { head: (__esc_diffEnum, tree), tail: __esc_diffLocal } } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = isEmpty(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(false)
        })) => {
                diff1 = (*__esc_diff1).clone();
                diffEnum = (*__esc_diffEnum).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons((diffEnum.clone(), ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
            for mut t in (tree.clone()).into_iter().cloned() {
                if !(!(isEmpty(&(t.clone())))) { continue; }
                let __x = t.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, __esc_tree1), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff2 @ (_, tree2), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff3 @ (_, tree3), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Deref @ metamodelica::ListNode::Cons { head: tree4First, tail: __esc_tree4 }), tail: __esc_diffLocal } } } } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree2.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        }) && ({
            let mut __acc: Option<bool> = None;
            for mut t in (tree3.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        }) && modelicaDiffTokenEq(lastToken(firstTreeLast.clone())?, firstTokenInTree(tree4First.clone())?)?) => {
                tree1 = (*__esc_tree1).clone();
                diff2 = (*__esc_diff2).clone();
                diff3 = (*__esc_diff3).clone();
                tree4 = (*__esc_tree4).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons((Diff::Delete.clone(), removeLastTokenInTrees(tree1.clone())?), metamodelica::cons((Diff::Equal.clone(), list![metamodelica::Ref::new(ParseTree::LEAF { token: lastToken(firstTreeLast.clone())? })]), metamodelica::cons((Diff::Add.clone(), metamodelica::cons(removeFirstTokenInTree(tree4First.clone())?, tree4.clone())), metamodelica::cons(diff2.clone(), metamodelica::cons(diff3.clone(), diffLocal.clone())))))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, __esc_tree), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, tree2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }), tail: _ } } } if (needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree((tree2).get(1)?)?)) => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                tree = (*__esc_tree).clone();
                diffLocal = (*__esc_diffLocal).clone();
                diff = metamodelica::cons((Diff::Equal.clone(), list![metamodelica::Ref::new(ParseTree::LEAF { token: makeToken(TokenId::WHITESPACE.clone(), literal!(" ")) })]), metamodelica::cons(diff1.clone(), diff));
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Add, tree2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }), tail: __esc_diffLocal } } if (needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree((tree2).get(1)?)?)) => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                diff2 = (*__esc_diff2).clone();
                diffLocal = (*__esc_diffLocal).clone();
                diffLocal = metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Equal.clone(), list![metamodelica::Ref::new(ParseTree::LEAF { token: makeToken(TokenId::WHITESPACE.clone(), literal!(" ")) })]), metamodelica::cons(diff2.clone(), diffLocal.clone())));
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, __esc_tree), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, __esc_tree2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }), tail: _ } } } => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                tree = (*__esc_tree).clone();
                diffLocal = (*__esc_diffLocal).clone();
                tree2 = (*__esc_tree2).clone();
                diff = metamodelica::cons(diff1.clone(), diff);
                metamodelica::cons((Diff::Delete.clone(), tree.clone()), diffLocal.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, tree), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } if (if (firstIter) {({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })} else {false}) => {
                diffLocal = (*__esc_diffLocal).clone();
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, tree), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } } if (({
            let mut __acc: Option<bool> = None;
            for mut t in (tree.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })) => {
                diff1 = (*__esc_diff1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                diff = metamodelica::cons(diff1.clone(), diff);
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Delete, __esc_tree1), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, tree2), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, __esc_tree3), tail: _ } } } if (!(parseTreeIsNewLine(&firstTreeLast)) && parseTreeIsNewLine(&(List::last(metamodelica::AsArg::as_arg(&tree2))?))) => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                tree3 = (*__esc_tree3).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), List::stripLast(tree2.clone())?), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::ListNode::Cons { head: t1, tail: Deref @ metamodelica::ListNode::Nil }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Deref @ metamodelica::ListNode::Cons { head: t2, tail: Deref @ metamodelica::ListNode::Cons { head: t3, tail: Deref @ metamodelica::ListNode::Nil } }), tail: __esc_diffLocal } } if (parseTreeIsOnlyIdent(metamodelica::AsArg::as_arg(&t1)) && parseTreeIsOnlyIdent(metamodelica::AsArg::as_arg(&t3)) && parseTreeIsWhitespaceNotComment(metamodelica::AsArg::as_arg(&t2))) => {
                diff1 = (*__esc_diff1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), list![t3.clone()]), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::ListNode::Cons { head: t1, tail: Deref @ metamodelica::ListNode::Nil }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Deref @ metamodelica::ListNode::Cons { head: t2, tail: Deref @ metamodelica::ListNode::Cons { head: t3, tail: Deref @ metamodelica::ListNode::Nil } }), tail: __esc_diffLocal } } if (parseTreeIsOnlyIdent(metamodelica::AsArg::as_arg(&t1)) && parseTreeIsOnlyIdent(metamodelica::AsArg::as_arg(&t2)) && parseTreeIsWhitespaceNotComment(metamodelica::AsArg::as_arg(&t3))) => {
                diff1 = (*__esc_diff1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), list![t2.clone()]), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, tree2), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, Deref @ metamodelica::ListNode::Cons { head: t1, tail: _ }), tail: _ } } } if (!(parseTreeIsNewLine(&(List::last(metamodelica::AsArg::as_arg(&tree2))?))) && parseTreeIsOnlyEnd(metamodelica::AsArg::as_arg(&t1))) => {
                diff1 = (*__esc_diff1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), listAppend(tree2.clone(), list![metamodelica::Ref::new(ParseTree::LEAF { token: makeToken(TokenId::NEWLINE.clone(), literal!("\n")) })])), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, tree2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }), tail: __esc_diffLocal } } if (!(needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree(List::second(metamodelica::AsArg::as_arg(&tree2))?)?)) && parseTreeIsWhitespaceNotComment(&((tree2).head().cloned()?)) && !(parseTreeIsNewLine(&firstTreeLast))) => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), (tree2).rest()?), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, __esc_tree1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Equal, tree2), tail: __esc_diffLocal } } if (!(needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree((tree2).head().cloned()?)?)) && !(parseTreeIsNewLine(&firstTreeSecondLast) || parseTreeIsLineComment(&firstTreeSecondLast)) && parseTreeIsWhitespaceNotCommentOrNewline(&firstTreeLast)) => {
                tree1 = (*__esc_tree1).clone();
                diff2 = (*__esc_diff2).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons((Diff::Add.clone(), List::stripLast(tree1.clone())?), metamodelica::cons(diff2.clone(), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, tree2), tail: __esc_diffLocal } } if (parseTreeIsNewLine(&firstTreeLast) && parseTreeIsNewLine(&((tree2).head().cloned()?))) => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), (tree2).rest()?), diffLocal.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Add, __esc_tree2), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff3 @ (DiffAlgorithm::Diff::Equal, tree3), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff4 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::ListNode::Cons { head: __esc_tree4First, tail: __esc_tree4 }), tail: __esc_diffLocal } } } } if (parseTreeIsNewLine(&firstTreeLast) && ({
            let mut __acc: Option<bool> = None;
            for mut t in (tree3.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })) => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                diff2 = (*__esc_diff2).clone();
                tree2 = (*__esc_tree2).clone();
                diff3 = (*__esc_diff3).clone();
                diff4 = (*__esc_diff4).clone();
                tree4First = (*__esc_tree4First).clone();
                tree4 = (*__esc_tree4).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons(diff3.clone(), metamodelica::cons(diff2.clone(), metamodelica::cons(diff4.clone(), diffLocal.clone()))))
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Add, __esc_tree2), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff3 @ (DiffAlgorithm::Diff::Equal, tree3), tail: Deref @ metamodelica::ListNode::Cons { head: __esc_diff4 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::ListNode::Cons { head: __esc_tree4First, tail: __esc_tree4 }), tail: __esc_diffLocal } } } } if (parseTreeIsNewLine(&firstTreeLast) && ({
            let mut __acc: Option<bool> = None;
            for mut t in (tree3.clone()).into_iter().cloned() {
                let __x = parseTreeIsWhitespaceNotComment(&(t.clone()));
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(true)
        })) => {
                diff1 = (*__esc_diff1).clone();
                tree1 = (*__esc_tree1).clone();
                diff2 = (*__esc_diff2).clone();
                tree2 = (*__esc_tree2).clone();
                diff3 = (*__esc_diff3).clone();
                diff4 = (*__esc_diff4).clone();
                tree4First = (*__esc_tree4First).clone();
                tree4 = (*__esc_tree4).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons(diff1.clone(), metamodelica::cons(diff3.clone(), metamodelica::cons(diff2.clone(), metamodelica::cons(diff4.clone(), diffLocal.clone()))))
            },
            Deref @ metamodelica::ListNode::Cons { head: (diffEnum1, __esc_tree1), tail: Deref @ metamodelica::ListNode::Cons { head: (diffEnum2, __esc_tree2), tail: __esc_diffLocal } } if (diffEnum1.clone() == diffEnum2.clone()) => {
                tree1 = (*__esc_tree1).clone();
                tree2 = (*__esc_tree2).clone();
                diffLocal = (*__esc_diffLocal).clone();
                metamodelica::cons((diffEnum1.clone(), listAppend(tree1.clone(), tree2.clone())), diffLocal.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, __esc_tree1), tail: __esc_diffLocal @ Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, tree2), tail: _ } } if (tokenId(lastToken(firstTreeLast.clone())?) == TokenId::WHITESPACE.clone() && tokenId(firstToken(tree2.clone())) == TokenId::NEWLINE.clone()) => {
                tree1 = (*__esc_tree1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                diff = metamodelica::cons((Diff::Add.clone(), removeLastTokenInTrees(tree1.clone())?), diff);
                diffLocal.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_diff1, tail: __esc_diffLocal } => {
                diff1 = (*__esc_diff1).clone();
                diffLocal = (*__esc_diffLocal).clone();
                diff = metamodelica::cons(diff1.clone(), diff);
                diffLocal.clone()
            },
            _ => return Err("match: no arm matched"),
        } });
        firstIter = false;
    }
    diff = metamodelica::Dangerous::listReverseInPlace(diff);
    lastTokenNewline = false;
    indentation = metamodelica::nil();
    hasAddedWS = false;
    for mut d in &*diff {
        let () = (::match_deref::match_deref! { match &(d.clone()) {
            (DiffAlgorithm::Diff::Add, __esc_tree) => {
                tree = (*__esc_tree).clone();
                for mut t in &*tree.clone() {
                    let () = (::match_deref::match_deref! { match &(firstNTokensInTree_reverse(metamodelica::AsArg::as_arg(&t), 2, metamodelica::nil())) {
            Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                length = (*__esc_length).clone();
                hasAddedWS = true;
                ()
            },
            Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::ListNode::Nil } if (lastTokenNewline) => {
                length = (*__esc_length).clone();
                hasAddedWS = true;
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                }
                ()
            },
            (_, __esc_tree) => {
                tree = (*__esc_tree).clone();
                for mut t in &*tree.clone() {
                    let () = (::match_deref::match_deref! { match &(firstNTokensInTree_reverse(metamodelica::AsArg::as_arg(&t), 2, metamodelica::nil())) {
            Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                length = (*__esc_length).clone();
                indentation = metamodelica::cons(length.clone(), indentation);
                lastTokenNewline = false;
                ()
            },
            Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::ListNode::Nil } if (lastTokenNewline) => {
                length = (*__esc_length).clone();
                indentation = metamodelica::cons(length.clone(), indentation);
                lastTokenNewline = false;
                ()
            },
            Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                lastTokenNewline = true;
                ()
            },
            _ => {
                lastTokenNewline = false;
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                }
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if (indentation).is_empty() || !(hasAddedWS) {
        if debug.clone() {
            metamodelica::print(literal!(
                "Skipping indentation as we could not auto-detect suitable indentation levels\n"
            ));
        }
        return Ok(diff);
    }
    level = ({
        let mut __acc: Option<i32> = None;
        for mut l in (indentation).into_iter().cloned() {
            let __x = l.clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => {
                    if __x < __cur {
                        __x
                    } else {
                        __cur
                    }
                }
            });
        }
        __acc.unwrap_or(i32::MAX)
    });
    indentationStr = StringUtil::repeat(literal!(" "), level)?;
    diffLocal = metamodelica::nil();
    for mut d in &*diff {
        let () = (::match_deref::match_deref! { match &(d.clone()) {
            (DiffAlgorithm::Diff::Delete, __esc_tree) => {
                tree = (*__esc_tree).clone();
                diffLocal = metamodelica::cons(d.clone(), diffLocal);
                ()
            },
            (__esc_diffEnum, __esc_tree) => {
                diffEnum = (*__esc_diffEnum).clone();
                tree = (*__esc_tree).clone();
                treeLocal = metamodelica::nil();
                hasAddedWS = false;
                for mut t in &*tree.clone() {
                    let () = (::match_deref::match_deref! { match &((diffEnum.clone(), firstNTokensInTree_reverse(metamodelica::AsArg::as_arg(&t), 2, metamodelica::nil()))) {
            (DiffAlgorithm::Diff::Equal, _) => (),
            (_, Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_tok @ Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                length = (*__esc_length).clone();
                tok = (*__esc_tok).clone();
                treeLocal = metamodelica::cons(replaceFirstTokensInTree(t.clone(), list![tok.clone(), makeToken(TokenId::WHITESPACE.clone(), indentationStr.clone())])?, treeLocal);
                hasAddedWS = true;
                ()
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::ListNode::Nil }) if (lastTokenNewline) => {
                length = (*__esc_length).clone();
                treeLocal = metamodelica::cons(replaceFirstTokensInTree(t.clone(), list![makeToken(TokenId::WHITESPACE.clone(), indentationStr.clone())])?, treeLocal);
                hasAddedWS = true;
                ()
            },
            _ => {
                treeLocal = metamodelica::cons(t.clone(), treeLocal);
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                    lastTokenNewline = (match lastToken(t.clone())? {
            Token { id: TokenId::NEWLINE, .. } => true,
            _ => false,
        });
                }
                diffLocal = if (hasAddedWS) {metamodelica::cons((diffEnum.clone(), treeLocal.reverse()), diffLocal)} else {metamodelica::cons(d.clone(), diffLocal)};
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    diff = metamodelica::Dangerous::listReverseInPlace(diffLocal);
    Ok(diff)
}

fn labelOrderDidNotChange(
    mut addList: &metamodelica::List<ArcStr>,
    mut delList: metamodelica::List<ArcStr>,
) -> Result<bool> {
    let mut b: bool;
    let mut acc: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut del: metamodelica::List<ArcStr> = delList.clone();
    let mut s: ArcStr;
    b = false;
    for mut item in &**addList {
        if listMember(item.clone(), acc.clone()) {
            return Ok(b);
        }
        if listMember(item.clone(), del.clone()) {
            while !metamodelica::stringEq(&item, &((del).head().cloned()?)) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(del) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                s = metamodelica::Own::own(__pa0);
                del = metamodelica::Own::own(__pa1);
                if listMember(s.clone(), acc.clone()) {
                    return Ok(b);
                }
                acc = metamodelica::cons(s, acc);
            }
            del = (del).rest()?;
        }
        acc = metamodelica::cons(item.clone(), acc);
    }
    for mut item in &*delList {
        if listMember(item.clone(), acc.clone()) {
            return Ok(b);
        }
        acc = metamodelica::cons(item.clone(), acc);
    }
    b = true;
    Ok(b)
}

fn makeToken(mut id: TokenId, mut r#str: ArcStr) -> Token {
    let mut token: Token;
    token = Token {
        fileName: literal!("<dummy>"),
        id: id,
        fileContents: r#str.clone(),
        byteOffset: 1,
        length: ((r#str).len() as i32),
        lineNumberStart: 0,
        columnNumberStart: 0,
        lineNumberEnd: 0,
        columnNumberEnd: 0,
    };
    token
}

fn replaceLabeledDiff(
    mut inDiff: &metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
    mut diffedNodes: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
    mut labelOfDiffedAddedNodes: metamodelica::Ref<ParseTree>,
    mut labelOfDiffedDeletedNodes: metamodelica::Ref<ParseTree>,
    mut compare: &dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>, metamodelica::Ref<ParseTree>) -> Result<bool>,
    mut inAllLabelsAreInOrder: bool,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut res: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)> = metamodelica::nil();
    let mut filtered: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>;
    let mut lst: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut acc: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut found: bool = false;
    let mut allLabelsAreInOrder: bool = inAllLabelsAreInOrder;
    let mut d: Diff;
    if metamodelica::stringEq(
        &(parseTreeStr(&(metamodelica::cons(labelOfDiffedDeletedNodes.clone(), metamodelica::nil())))?),
        &(literal!("$equation_section")),
    ) {
        allLabelsAreInOrder = false;
    }
    for mut diff in &**inDiff {
        res = (::match_deref::match_deref! { match &(diff.clone()) {
            (DiffAlgorithm::Diff::Equal, _) => metamodelica::cons(diff.clone(), res),
            (DiffAlgorithm::Diff::Add, lst) if (!(({
            let mut __acc: Option<bool> = None;
            for mut t in (lst.clone()).into_iter().cloned() {
                let __x = compare(nodeLabel(&(t.clone())), labelOfDiffedAddedNodes.clone())?;
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(false)
        }))) => metamodelica::cons(diff.clone(), res),
            (DiffAlgorithm::Diff::Delete, lst) if (!(({
            let mut __acc: Option<bool> = None;
            for mut t in (lst.clone()).into_iter().cloned() {
                let __x = compare(nodeLabel(&(t.clone())), labelOfDiffedDeletedNodes.clone())?;
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
            }
            __acc.unwrap_or(false)
        }))) => metamodelica::cons(diff.clone(), res),
            (DiffAlgorithm::Diff::Add, __esc_lst) if (allLabelsAreInOrder) => {
                lst = (*__esc_lst).clone();
                metamodelica::cons((Diff::Add.clone(), ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
            for mut t in (lst.clone()).into_iter().cloned() {
                if !(!(compare(nodeLabel(&(t.clone())), labelOfDiffedAddedNodes.clone())?)) { continue; }
                let __x = t.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), res)
            },
            (DiffAlgorithm::Diff::Delete, __esc_lst) if (!(allLabelsAreInOrder)) => {
                lst = (*__esc_lst).clone();
                metamodelica::cons((Diff::Delete.clone(), ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
            for mut t in (lst.clone()).into_iter().cloned() {
                if !(!(compare(nodeLabel(&(t.clone())), labelOfDiffedDeletedNodes.clone())?)) { continue; }
                let __x = t.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), res)
            },
            (__esc_d, __esc_lst) => {
                d = (*__esc_d).clone();
                lst = (*__esc_lst).clone();
                acc = metamodelica::nil();
                for mut t in &*lst.clone() {
                    if !(found) && compare(nodeLabel(metamodelica::AsArg::as_arg(&t)), if (allLabelsAreInOrder) {labelOfDiffedDeletedNodes.clone()} else {labelOfDiffedAddedNodes.clone()})? {
                        if !((acc).is_empty()) {
                            res = metamodelica::cons((Diff::Add.clone(), acc.reverse()), res);
                            acc = metamodelica::nil();
                        }
                        filtered = ({
            let mut __acc: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)> = metamodelica::nil();
            for mut i in (diffedNodes.clone()).into_iter().cloned() {
                if !((::match_deref::match_deref! { match &(i.clone()) {
            (DiffAlgorithm::Diff::Delete, _) => false,
            _ => true,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })) { continue; }
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc
        });
                        res = listAppend(filtered, res);
                        found = true;
                    } else {
                        res = metamodelica::cons((d.clone(), list![t.clone()]), res);
                    }
                }
                if !((acc).is_empty()) {
                    res = metamodelica::cons((Diff::Add.clone(), acc.reverse()), res);
                }
                res
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    res = res.reverse();
    Ok(res)
}

fn isEmpty(mut tree: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    b = (match &**tree {
        ParseTree::EMPTY { .. } => true,
        _ => false,
    });
    b
}

fn isLabeledNode(mut tree: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match tree {
        Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, .. } => false,
        Deref @ ParseTree::NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn nodeLabel(mut tree: &metamodelica::Ref<ParseTree>) -> metamodelica::Ref<ParseTree> {
    let mut label: metamodelica::Ref<ParseTree>;
    label = (match &**tree {
        ParseTree::NODE {
            label: __tree_label, ..
        } => __tree_label.clone(),
        _ => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
    });
    label
}

fn parseTreeEq(
    mut t1: metamodelica::Ref<ParseTree>,
    mut t2: metamodelica::Ref<ParseTree>,
    mut diffSubtreeWorkArray1: metamodelica::Array<Token>,
    mut diffSubtreeWorkArray2: metamodelica::Array<Token>,
) -> Result<bool> {
    let mut b: bool;
    let mut len1: i32;
    let mut len2: i32;
    let mut commentLen1: i32;
    let mut commentLen2: i32;
    (len1, commentLen1) = findTokens(t1, diffSubtreeWorkArray1.clone(), 0, 0)?;
    (len2, commentLen2) = findTokens(t2, diffSubtreeWorkArray2.clone(), 0, 0)?;
    b = false;
    if len1 != len2 || commentLen1 != commentLen2 {
        return Ok(b);
    }
    for mut i in 1..=len1 {
        if !(modelicaDiffTokenEq(
            ({
                let __elt = (*metamodelica::index_checked(&diffSubtreeWorkArray1.borrow(), i)?).clone();
                __elt
            }),
            ({
                let __elt = (*metamodelica::index_checked(&diffSubtreeWorkArray2.borrow(), i)?).clone();
                __elt
            }),
        )?) {
            return Ok(b);
        }
    }
    for mut i in 1..=commentLen1 {
        if !(modelicaDiffTokenEq(
            ({
                let __elt = (*metamodelica::index_checked(
                    &diffSubtreeWorkArray1.borrow(),
                    metamodelica::arrayLength(diffSubtreeWorkArray1.clone()) - (i - 1),
                )?)
                .clone();
                __elt
            }),
            ({
                let __elt = (*metamodelica::index_checked(
                    &diffSubtreeWorkArray2.borrow(),
                    metamodelica::arrayLength(diffSubtreeWorkArray2.clone()) - (i - 1),
                )?)
                .clone();
                __elt
            }),
        )?) {
            return Ok(b);
        }
    }
    b = true;
    Ok(b)
}

fn findTokens(
    mut t: metamodelica::Ref<ParseTree>,
    mut work: metamodelica::Array<Token>,
    mut inCount: i32,
    mut inCommentCount: i32,
) -> Result<(i32, i32)> {
    let mut count: i32 = inCount;
    let mut commentCount: i32 = inCommentCount;
    if parseTreeIsComment(&t) {
        metamodelica::arrayUpdate(
            work.clone(),
            metamodelica::arrayLength(work.clone()) - commentCount,
            firstTokenInTree(t)?,
        )?;
        commentCount = commentCount + 1;
        return Ok((count, commentCount));
    } else if parseTreeIsWhitespace(&t) {
        return Ok((count, commentCount));
    }
    let () = (match &*t {
        ParseTree::EMPTY { .. } => (),
        ParseTree::LEAF { token: __t_token } => {
            count = count + 1;
            metamodelica::arrayUpdate(work.clone(), count, __t_token.clone())?;
            ()
        }
        ParseTree::NODE { nodes: __t_nodes, .. } => {
            for mut n in &*__t_nodes.clone() {
                (count, commentCount) = findTokens(n.clone(), work.clone(), count, commentCount)?;
            }
            ()
        }
    });
    Ok((count, commentCount))
}

fn replaceFirstTokensInTree(
    mut t: metamodelica::Ref<ParseTree>,
    mut tokens: metamodelica::List<Token>,
) -> Result<metamodelica::Ref<ParseTree>> {
    let mut tree: metamodelica::Ref<ParseTree>;
    let __pa0 = ::match_deref::match_deref! { match &(replaceFirstTokensInTreeWork(t, tokens)?) {
        (__pa0, Deref @ metamodelica::ListNode::Nil) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    tree = metamodelica::Own::own(__pa0);
    Ok(tree)
}

fn replaceFirstTokensInTreeWork(
    mut t: metamodelica::Ref<ParseTree>,
    mut inTokens: metamodelica::List<Token>,
) -> Result<(metamodelica::Ref<ParseTree>, metamodelica::List<Token>)> {
    let mut tree: metamodelica::Ref<ParseTree> = t;
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut work: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut acc: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut n: metamodelica::Ref<ParseTree>;
    let mut tok: Token;
    (tree, tokens) = (::match_deref::match_deref! { match &((tree.clone(), tokens.clone())) {
        (__esc_tree, Deref @ metamodelica::ListNode::Nil) => {
            tree = (*__esc_tree).clone();
            (tree.clone(), tokens)
        },
        (Deref @ ParseTree::EMPTY { .. }, _) => (tree, tokens),
        (Deref @ ParseTree::LEAF { .. }, Deref @ metamodelica::ListNode::Cons { head: __esc_tok, tail: __esc_tokens }) => {
            tokens = (*__esc_tokens).clone();
            tok = (*__esc_tok).clone();
            (metamodelica::Ref::new(ParseTree::LEAF { token: tok.clone() }), tokens.clone())
        },
        (Deref @ ParseTree::NODE { .. }, __esc_tokens) => {
            tokens = (*__esc_tokens).clone();
            work = var_field!((*tree).nodes, ParseTree::NODE).clone();
            acc = metamodelica::nil();
            while !((work).is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(work) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                n = metamodelica::Own::own(__pa0);
                work = metamodelica::Own::own(__pa1);
                (n, tokens) = replaceFirstTokensInTreeWork(n, tokens.clone())?;
                if (tokens).is_empty() {
                    assign_variant_field!(tree => ParseTree::NODE; nodes = List::append_reverse(&acc, metamodelica::cons(n, work)));
                    return Ok((tree, tokens.clone()));
                } else {
                    acc = metamodelica::cons(n, acc);
                }
            }
            assign_variant_field!(tree => ParseTree::NODE; nodes = acc.reverse());
            (tree, tokens.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((tree, tokens))
}

fn firstNTokensInTree_reverse(
    mut t: &metamodelica::Ref<ParseTree>,
    mut n: i32,
    mut acc: metamodelica::List<Token>,
) -> metamodelica::List<Token> {
    let mut tokens: metamodelica::List<Token> = acc.clone();
    if ((tokens).len() as i32) > 1 {
        return tokens;
    }
    tokens = (match &**t {
        ParseTree::EMPTY { .. } => tokens,
        ParseTree::LEAF { token: __t_token } => metamodelica::cons(__t_token.clone(), tokens),
        ParseTree::NODE { nodes: __t_nodes, .. } => {
            for mut node in &*__t_nodes.clone() {
                tokens = firstNTokensInTree_reverse(metamodelica::AsArg::as_arg(&node), n, tokens);
                if ((tokens).len() as i32) > 1 {
                    return tokens;
                }
            }
            acc
        }
    });
    tokens
}

fn removeFirstTokenInTree(mut t: metamodelica::Ref<ParseTree>) -> Result<metamodelica::Ref<ParseTree>> {
    let mut t: metamodelica::Ref<ParseTree> = t;
    t = (::match_deref::match_deref! { match &(t) {
        Deref @ ParseTree::EMPTY { .. } => {
            return Err("fail")
        },
        Deref @ ParseTree::LEAF { .. } => {
            crate::SimpleModelicaParser::ParseTree::interned_EMPTY()
        },
        Deref @ ParseTree::NODE { label, nodes: Deref @ metamodelica::ListNode::Cons { head: node, tail: nodes } } => {
            makeNode(metamodelica::cons(removeFirstTokenInTree(node.clone())?, nodes.clone()), label.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(t)
}

fn removeLastTokenInTree(mut t: metamodelica::Ref<ParseTree>) -> Result<metamodelica::Ref<ParseTree>> {
    let mut t: metamodelica::Ref<ParseTree> = t;
    t = (match &*t {
        ParseTree::EMPTY { .. } => return Err("fail"),
        ParseTree::LEAF { .. } => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        ParseTree::NODE { label, nodes } => {
            let mut node: metamodelica::Ref<ParseTree>;
            let mut nodes = (*nodes).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes.clone().reverse()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            node = metamodelica::Own::own(__pa0);
            nodes = metamodelica::Own::own(__pa1);
            makeNode(
                metamodelica::cons(removeLastTokenInTree(node)?, nodes.clone()).reverse(),
                label.clone(),
            )
        }
    });
    Ok(t)
}

fn removeLastTokenInTrees(
    mut ts: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<metamodelica::List<metamodelica::Ref<ParseTree>>> {
    let mut ts: metamodelica::List<metamodelica::Ref<ParseTree>> = ts;
    let mut t: metamodelica::Ref<ParseTree>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ts.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    t = metamodelica::Own::own(__pa0);
    ts = metamodelica::Own::own(__pa1);
    ts = metamodelica::cons(removeLastTokenInTree(t)?, ts).reverse();
    Ok(ts)
}

fn firstTokenInTree(mut t: metamodelica::Ref<ParseTree>) -> Result<Token> {
    '__tco: loop {
        match &*t {
            ParseTree::EMPTY { .. } => {
                metamodelica::print(literal!("No first token in tree\n"));
                return Ok(return Err("fail"));
            }
            ParseTree::LEAF { token: __t_token } => return Ok(__t_token.clone()),
            ParseTree::NODE { nodes: __t_nodes, .. } => {
                t = (__t_nodes).get(1)?;
                continue '__tco;
            }
        }
    }
}

fn lastToken(mut t: metamodelica::Ref<ParseTree>) -> Result<Token> {
    '__tco: loop {
        match &*t {
            ParseTree::EMPTY { .. } => {
                if debug.clone() {
                    metamodelica::print(literal!("lastToken fail\n"));
                }
                return Ok(return Err("fail"));
            }
            ParseTree::LEAF { token: __t_token } => return Ok(__t_token.clone()),
            ParseTree::NODE { nodes: __t_nodes, .. } => {
                t = List::last(metamodelica::AsArg::as_arg(&__t_nodes))?;
                continue '__tco;
            }
        }
    }
}

fn fixMoveOperations(
    mut inDiff: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
    mut compare: &dyn ::std::ops::Fn(metamodelica::Ref<ParseTree>, metamodelica::Ref<ParseTree>) -> Result<bool>,
) -> Result<metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>> {
    let mut diff: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)> = metamodelica::nil();
    let mut lst: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut deleted: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut lst2: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut changeFound: bool = false;
    let mut d1: (Diff, metamodelica::List<metamodelica::Ref<ParseTree>>);
    for mut d in &*inDiff {
        let () = (::match_deref::match_deref! { match &(d.clone()) {
            (DiffAlgorithm::Diff::Delete, __esc_lst) => {
                lst = (*__esc_lst).clone();
                deleted = listAppend(lst.clone(), deleted);
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if (deleted).is_empty() {
        diff = inDiff;
        return Ok(diff);
    }
    for mut d in &*inDiff {
        d1 = (::match_deref::match_deref! { match &(d.clone()) {
            (DiffAlgorithm::Diff::Add, __esc_lst) => {
                lst = (*__esc_lst).clone();
                d1 = d.clone();
                for mut l1 in &*lst.clone() {
                    if List::isMemberOnTrue(l1.clone(), &deleted, compare)? {
                        changeFound = true;
                        lst2 = metamodelica::nil();
                        for mut l2 in &*lst.clone() {
                            match '__try0: {
                                lst2 = metamodelica::cons(unwrap_break_err!(List::getMemberOnTrue(l2.clone(), &deleted, compare), '__try0), lst2.clone());
                                Ok::<_, &'static str>((lst2.clone(),))
                            } {
                                Ok((__try0_o0,)) => {
                                    lst2 = __try0_o0;
                                }
                                Err(_) => {
                                    lst2 = metamodelica::cons(l2.clone(), lst2.clone());
                                }
                            }
                        }
                        d1 = (Diff::Add.clone(), metamodelica::Dangerous::listReverseInPlace(lst2));
                        break;
                    }
                }
                d1
            },
            _ => d.clone(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        diff = metamodelica::cons(d1, diff);
    }
    diff = if (changeFound) {
        metamodelica::Dangerous::listReverseInPlace(diff)
    } else {
        inDiff
    };
    Ok(diff)
}

fn makeNode(
    mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut label: metamodelica::Ref<ParseTree>,
) -> metamodelica::Ref<ParseTree> {
    let mut node: metamodelica::Ref<ParseTree>;
    node = (::match_deref::match_deref! { match &((({
        let mut __acc: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
        for mut n in (nodes.clone()).into_iter().cloned() {
            if !(!(isEmpty(&(n.clone())))) { continue; }
            let __x = n.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), label.clone())) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ ParseTree::EMPTY { .. }) => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        (Deref @ metamodelica::ListNode::Cons { head: __esc_node, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ ParseTree::EMPTY { .. }) => {
            node = (*__esc_node).clone();
            node.clone()
        },
        _ => metamodelica::Ref::new(ParseTree::NODE { label: label, nodes: nodes }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    node
}

fn makeNodePrependTree(
    mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut tree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut label: metamodelica::Ref<ParseTree>,
) -> metamodelica::List<metamodelica::Ref<ParseTree>> {
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    outTree = if (!((nodes).is_empty())) {
        metamodelica::cons(makeNode(nodes, label), tree)
    } else {
        tree
    };
    outTree
}

fn isLeaf(mut t: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    b = (match &**t {
        ParseTree::LEAF { .. } => true,
        _ => false,
    });
    b
}

fn firstToken(mut t: metamodelica::List<metamodelica::Ref<ParseTree>>) -> Token {
    let mut token: Token;
    token = (::match_deref::match_deref! { match &(t) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ ParseTree::NODE { nodes, .. }, tail: _ } => {
            firstToken(nodes.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ ParseTree::LEAF { token: __esc_token }, tail: _ } => {
            token = (*__esc_token).clone();
            token.clone()
        },
        _ => {
            LexerModelicaDiff::noToken.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    token
}

fn firstTokenDebugStr(mut t: metamodelica::List<metamodelica::Ref<ParseTree>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut l: metamodelica::List<Token>;
    l = metamodelica::cons(firstToken(t), metamodelica::nil());
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Error::infoStr(&(topTokenSourceInfo(&l)))?);
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*topTokenStr(&l)?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn getNodes(mut t: metamodelica::Ref<ParseTree>) -> metamodelica::List<metamodelica::Ref<ParseTree>> {
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    nodes = (match &*t {
        ParseTree::NODE { nodes: __t_nodes, .. } => __t_nodes.clone(),
        _ => list![t],
    });
    nodes
}

fn extractSingleAddDiffBeforeAndAfter(
    mut diffs: &metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
) -> Result<(
    metamodelica::Ref<ParseTree>,
    metamodelica::Ref<ParseTree>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    bool,
)> {
    let mut addedTree: metamodelica::Ref<ParseTree> = crate::SimpleModelicaParser::ParseTree::interned_EMPTY();
    let mut deletedTree: metamodelica::Ref<ParseTree> = crate::SimpleModelicaParser::ParseTree::interned_EMPTY();
    let mut before: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut middle: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut after: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut addedBeforeDeleted: bool = false;
    let mut foundAdded: bool = false;
    let mut foundDeleted: bool = false;
    let mut acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ParseTree>>> = metamodelica::nil();
    let mut trees: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut lst: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut d: Diff;
    let mut addCount: i32;
    for mut diff in &**diffs {
        let () = (::match_deref::match_deref! { match &(diff.clone()) {
            (DiffAlgorithm::Diff::Add, __esc_lst) => {
                lst = (*__esc_lst).clone();
                addCount = 0;
                for mut tree in &*lst.clone() {
                    addCount = addCount + 1;
                    if parseTreeIsNewLine(metamodelica::AsArg::as_arg(&tree)) && addCount > 1 && addCount == ((lst).len() as i32) {
                        acc = metamodelica::cons(list![tree.clone()], acc);
                    } else if parseTreeIsWhitespace(metamodelica::AsArg::as_arg(&tree)) {
                        acc = acc;
                    } else {
                        if foundAdded {
                            Error::addInternalError(literal!("Found multiple Add subtrees"), metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"))?;
                            return Err("fail");
                        }
                        addedTree = tree.clone();
                        foundAdded = true;
                        if foundDeleted {
                            middle = List::flattenReverse(acc)?;
                        } else {
                            addedBeforeDeleted = true;
                            before = List::flattenReverse(acc)?;
                        }
                        acc = metamodelica::nil();
                    }
                }
                ()
            },
            (DiffAlgorithm::Diff::Delete, __esc_lst) => {
                lst = (*__esc_lst).clone();
                for mut tree in &*lst.clone() {
                    if parseTreeIsWhitespace(metamodelica::AsArg::as_arg(&tree)) {
                        acc = metamodelica::cons(list![tree.clone()], acc);
                    } else {
                        if foundDeleted {
                            Error::addInternalError(literal!("Found multiple Delete subtrees"), metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"))?;
                            return Err("fail");
                        }
                        deletedTree = tree.clone();
                        foundDeleted = true;
                        if foundAdded {
                            middle = List::flattenReverse(acc)?;
                        } else {
                            addedBeforeDeleted = false;
                            before = List::flattenReverse(acc)?;
                        }
                        acc = metamodelica::nil();
                    }
                }
                ()
            },
            (DiffAlgorithm::Diff::Equal, __esc_trees) => {
                trees = (*__esc_trees).clone();
                acc = metamodelica::cons(trees.clone(), acc);
                ()
            },
            (__esc_d, _) => {
                d = (*__esc_d).clone();
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", d.clone()))); __mm_s.push_str(&*literal!(" subtrees with multiple or zero entries")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    let true = (foundAdded) else {
        return Err("pattern mismatch");
    };
    let true = (foundDeleted) else {
        return Err("pattern mismatch");
    };
    after = List::flattenReverse(acc)?;
    Ok((addedTree, deletedTree, before, middle, after, addedBeforeDeleted))
}

fn extractAdditionsDeletions(
    mut diffs: &metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut addedTrees: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut deletedTrees: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut addedTreesAcc: metamodelica::List<metamodelica::List<metamodelica::Ref<ParseTree>>> = metamodelica::nil();
    let mut deletedTreesAcc: metamodelica::List<metamodelica::List<metamodelica::Ref<ParseTree>>> = metamodelica::nil();
    let mut lst: metamodelica::List<metamodelica::Ref<ParseTree>>;
    for mut diff in &**diffs {
        let () = (::match_deref::match_deref! { match &(diff.clone()) {
            (DiffAlgorithm::Diff::Add, __esc_lst) => {
                lst = (*__esc_lst).clone();
                addedTreesAcc = metamodelica::cons(lst.clone(), addedTreesAcc);
                ()
            },
            (DiffAlgorithm::Diff::Delete, __esc_lst) => {
                lst = (*__esc_lst).clone();
                deletedTreesAcc = metamodelica::cons(lst.clone(), deletedTreesAcc);
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    addedTrees = List::flattenReverse(addedTreesAcc)?;
    deletedTrees = List::flattenReverse(deletedTreesAcc)?;
    Ok((addedTrees, deletedTrees))
}

fn countDiffAddDelete(
    mut diffs: &metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<ParseTree>>)>,
) -> (i32, i32) {
    let mut nadd: i32 = 0;
    let mut ndel: i32 = 0;
    let mut d: Diff;
    let mut l: metamodelica::List<metamodelica::Ref<ParseTree>>;
    for mut diff in &**diffs {
        (d, l) = diff.clone();
        if d == Diff::Add.clone() {
            nadd = nadd
                + ({
                    let mut __acc: i32 = 0;
                    for mut t in (l).into_iter().cloned() {
                        let __x = if (parseTreeIsWhitespace(&(t.clone()))) { 0 } else { 1 };
                        __acc += __x;
                    }
                    __acc
                });
        } else if d == Diff::Delete.clone() {
            ndel = ndel
                + ({
                    let mut __acc: i32 = 0;
                    for mut t in (l).into_iter().cloned() {
                        let __x = if (parseTreeIsWhitespace(&(t.clone()))) { 0 } else { 1 };
                        __acc += __x;
                    }
                    __acc
                });
        }
    }
    (nadd, ndel)
}

pub(crate) static whiteSpaceTokenIds: std::sync::LazyLock<metamodelica::List<TokenId>> =
    std::sync::LazyLock::new(|| {
        list![
            TokenId::LINE_COMMENT.clone(),
            TokenId::BLOCK_COMMENT.clone(),
            TokenId::NEWLINE.clone(),
            TokenId::WHITESPACE.clone()
        ]
    });

pub(crate) static whiteSpaceTokenIdsNotComment: std::sync::LazyLock<metamodelica::List<TokenId>> =
    std::sync::LazyLock::new(|| list![TokenId::NEWLINE.clone(), TokenId::WHITESPACE.clone()]);

pub(crate) static tokenIdsComment: std::sync::LazyLock<metamodelica::List<TokenId>> =
    std::sync::LazyLock::new(|| list![TokenId::LINE_COMMENT.clone(), TokenId::BLOCK_COMMENT.clone()]);

fn parseTreeIsWhitespace(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => listMember(__t1_token.id.clone(), whiteSpaceTokenIds.clone()),
        _ => false,
    });
    b
}

fn parseTreeIsNewLine(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => __t1_token.id.clone() == TokenId::NEWLINE.clone(),
        _ => false,
    });
    b
}

fn parseTreeIsWhitespaceNotComment(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => {
            listMember(__t1_token.id.clone(), whiteSpaceTokenIdsNotComment.clone())
        }
        _ => false,
    });
    b
}

fn parseTreeIsWhitespaceNotCommentOrNewline(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => __t1_token.id.clone() == TokenId::WHITESPACE.clone(),
        _ => false,
    });
    b
}

fn parseTreeIsComment(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => listMember(__t1_token.id.clone(), tokenIdsComment.clone()),
        _ => false,
    });
    b
}

fn parseTreeIsLineComment(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => __t1_token.id.clone() == TokenId::LINE_COMMENT.clone(),
        _ => false,
    });
    b
}

fn parseTreeIsOnlyIdent(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => __t1_token.id.clone() == TokenId::IDENT.clone(),
        _ => false,
    });
    b
}

fn parseTreeIsOnlyEnd(mut t1: &metamodelica::Ref<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (match &**t1 {
        ParseTree::LEAF { token: __t1_token } => __t1_token.id.clone() == TokenId::END.clone(),
        _ => false,
    });
    b
}

fn parseTreeFilterWhitespace(mut t: metamodelica::Ref<ParseTree>) -> metamodelica::Ref<ParseTree> {
    let mut t: metamodelica::Ref<ParseTree> = t;
    let mut id: TokenId;
    let mut changed: bool;
    let mut n2: metamodelica::Ref<ParseTree>;
    let mut nodes: metamodelica::List<metamodelica::Ref<ParseTree>>;
    t = (match &*t {
        ParseTree::LEAF { token: __t_token } if (listMember(__t_token.id.clone(), whiteSpaceTokenIds.clone())) => {
            crate::SimpleModelicaParser::ParseTree::interned_EMPTY()
        }
        ParseTree::NODE {
            label: __t_label,
            nodes: __t_nodes,
        } => {
            changed = false;
            nodes = metamodelica::nil();
            for mut n in &*__t_nodes.clone() {
                n2 = parseTreeFilterWhitespace(n.clone());
                if !(referenceEq(&*(n.clone()), &*(&*n2))) {
                    changed = true;
                }
                if !(isEmpty(&n2)) {
                    nodes = metamodelica::cons(n2, nodes);
                }
            }
            if (changed) {
                metamodelica::Ref::new(ParseTree::NODE {
                    label: __t_label.clone(),
                    nodes: nodes.reverse(),
                })
            } else {
                t
            }
        }
        _ => t,
    });
    t
}

fn eatWhitespace(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut t: Token;
    tree = inTree;
    while (::match_deref::match_deref! { match &(&*tokens) {
        Deref @ metamodelica::ListNode::Cons { head: Token { id: __esc_id, .. }, tail: _ } => {
            id = (*__esc_id).clone();
            listMember(id.clone(), list![TokenId::LINE_COMMENT.clone(), TokenId::BLOCK_COMMENT.clone(), TokenId::NEWLINE.clone(), TokenId::WHITESPACE.clone()])
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        t = metamodelica::Own::own(__pa0);
        tokens = metamodelica::Own::own(__pa1);
        tree = metamodelica::cons(metamodelica::Ref::new(ParseTree::LEAF { token: t }), tree);
    }
    outTree = tree;
    Ok((tokens, outTree))
}

fn scanOpt(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut id: TokenId,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    bool,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens.clone();
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut found: bool;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id2: TokenId;
    let mut t: Token;
    let mut tokens2: metamodelica::List<Token>;
    (tokens, tree) = eatWhitespace(tokens, inTree.clone())?;
    (tokens, tree, found) = (::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_t @ Token { id: id2, .. }, tail: __esc_tokens2 } if (id == id2.clone()) => {
            t = (*__esc_t).clone();
            tokens2 = (*__esc_tokens2).clone();
            (tokens2.clone(), metamodelica::cons(metamodelica::Ref::new(ParseTree::LEAF { token: t.clone() }), tree), true)
        },
        _ => (tokens, tree, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(found) {
        outTree = inTree;
        tokens = inTokens;
    } else {
        outTree = tree;
    }
    Ok((tokens, outTree, found))
}

fn scan(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut id: TokenId,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut found: bool;
    tree = inTree;
    (tokens, tree, found) = scanOpt(tokens, tree, id)?;
    if !(found) {
        error(tokens.clone(), tree.clone(), list![id])?;
    }
    outTree = tree;
    Ok((tokens, outTree))
}

fn scanOneOf(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut ids: metamodelica::List<TokenId>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut found: bool;
    tree = inTree;
    (tokens, tree, found) = LA1(tokens, tree, ids.clone(), true)?;
    if !(found) {
        error(tokens.clone(), tree.clone(), ids)?;
    }
    outTree = tree;
    Ok((tokens, outTree))
}

fn error(
    mut tokens: metamodelica::List<Token>,
    mut tree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut expected: metamodelica::List<TokenId>,
) -> Result<()> {
    let mut i: i32;
    let mut s: ArcStr = arcstr::literal!("");
    let mut strs: metamodelica::List<ArcStr>;
    let mut res: metamodelica::List<ArcStr>;
    let mut info: SourceInfo;
    info = topTokenSourceInfo(&tokens);
    res = metamodelica::cons(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Failed to scan top of input: "));
            __mm_s.push_str(&*if (debug.clone()) {
                debugTokenStr(tokens)?
            } else {
                topTokenStr(&tokens)?
            });
            __mm_s.push_str(&*literal!("\n  Expected one of: "));
            __mm_s.push_str(&*if ((expected).is_empty()) {
                literal!("<EOF>")
            } else {
                stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut id in (expected).into_iter().cloned() {
                            let __x = tokenIdStr(id.clone());
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(", "),
                )
            });
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        },
        metamodelica::nil(),
    );
    res = metamodelica::cons(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("  Current parse tree is:\n"));
            __mm_s.push_str(&*parseTreeStr(&(tree.reverse()))?);
            __mm_s.push_str(&*literal!("\n  The parser stack is:\n"));
            ArcStr::from(__mm_s)
        },
        res,
    );
    StackOverflow::setStacktraceMessages(0, 100);
    for mut s in &*StackOverflow::readableStacktraceMessages()? {
        let mut s = s.clone();
        (i, strs) = System::regex(
            s.clone(),
            literal!("SimpleModelicaParser[^A-Za-z]([A-Za-z_0-9_]*)"),
            2,
            true,
            false,
        );
        let () = (::match_deref::match_deref! { match &((i, strs)) {
            (2, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: s, tail: Deref @ metamodelica::ListNode::Nil } }) if (!metamodelica::stringEq(&s, &(literal!("error")))) => {
                res = metamodelica::cons(literal!("\n"), metamodelica::cons(s.clone(), res));
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Error::addInternalError(stringAppendList(res.reverse()), info)?;
    return Err("fail");
    Ok(())
}

fn tokenIdStr(mut id: TokenId) -> ArcStr {
    let mut r#str: ArcStr = ArcStr::from(::std::format!("{:?}", id));
    r#str
}

fn peek(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    TokenId,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut id: TokenId;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    tree = inTree;
    (tokens, tree) = eatWhitespace(tokens, tree)?;
    id = (::match_deref::match_deref! { match &(&*tokens) {
        Deref @ metamodelica::ListNode::Cons { head: Token { id: __esc_id, .. }, tail: _ } => {
            id = (*__esc_id).clone();
            id.clone()
        },
        _ => TokenId::_NO_TOKEN.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTree = tree;
    Ok((tokens, outTree, id))
}

fn consume(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut t: Token;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    t = metamodelica::Own::own(__pa0);
    tokens = metamodelica::Own::own(__pa1);
    outTree = metamodelica::cons(metamodelica::Ref::new(ParseTree::LEAF { token: t }), inTree);
    Ok((tokens, outTree))
}

fn LA1(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut ids: metamodelica::List<TokenId>,
    mut consume: bool,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    bool,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens.clone();
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut found: bool;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    tree = inTree.clone();
    (tokens, tree) = eatWhitespace(tokens, tree)?;
    found = (::match_deref::match_deref! { match &(&*tokens) {
        Deref @ metamodelica::ListNode::Cons { head: Token { id: __esc_id, .. }, tail: _ } => {
            id = (*__esc_id).clone();
            listMember(id.clone(), ids)
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if found && consume {
        (tokens, tree) = self::consume(tokens, tree)?;
    }
    if !(found) {
        outTree = inTree;
        tokens = inTokens;
    } else {
        outTree = tree;
    }
    Ok((tokens, outTree, found))
}

fn LAk(
    mut inTokens: metamodelica::List<Token>,
    mut inTree: metamodelica::List<metamodelica::Ref<ParseTree>>,
    mut idsLst: metamodelica::List<metamodelica::List<TokenId>>,
) -> Result<(
    metamodelica::List<Token>,
    metamodelica::List<metamodelica::Ref<ParseTree>>,
    bool,
)> {
    let mut tokens: metamodelica::List<Token> = inTokens;
    let mut outTree: metamodelica::List<metamodelica::Ref<ParseTree>>;
    let mut found: bool = true;
    let mut tree: metamodelica::List<metamodelica::Ref<ParseTree>> = metamodelica::nil();
    let mut id: TokenId;
    let mut tmp: metamodelica::List<Token>;
    tree = inTree;
    (tokens, tree) = eatWhitespace(tokens, tree)?;
    outTree = tree;
    tmp = tokens.clone();
    for mut ids in &*idsLst {
        found = (::match_deref::match_deref! { match &(&*tmp) {
            Deref @ metamodelica::ListNode::Cons { head: Token { id: __esc_id, .. }, tail: __esc_tmp } => {
                id = (*__esc_id).clone();
                tmp = (*__esc_tmp).clone();
                listMember(id.clone(), ids.clone())
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if !(found) {
            return Ok((tokens, outTree, found));
        }
        (tmp, _) = eatWhitespace(tmp, metamodelica::nil())?;
    }
    Ok((tokens, outTree, found))
}

fn parseTreeStrWork(mut tree: &metamodelica::Ref<ParseTree>) -> Result<()> {
    let () = (match &**tree {
        ParseTree::LEAF { token: __tree_token } => {
            Print::printBuf(tokenContent(__tree_token.clone())?)?;
            ()
        }
        ParseTree::EMPTY { .. } => (),
        ParseTree::NODE {
            nodes: __tree_nodes, ..
        } => {
            for mut n in &*__tree_nodes.clone() {
                parseTreeStrWork(metamodelica::AsArg::as_arg(&n))?;
            }
            ()
        }
    });
    Ok(())
}

fn topTokenStr(mut tokens: &metamodelica::List<Token>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut id: TokenId;
    let mut t: Token;
    r#str = (::match_deref::match_deref! { match tokens {
        Deref @ metamodelica::ListNode::Cons { head: __esc_t @ Token { id: __esc_id, .. }, tail: _ } => {
            t = (*__esc_t).clone();
            id = (*__esc_id).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id.clone()))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*tokenContent(t.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => literal!("EOF"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

fn debugTokenStr(mut tokens: metamodelica::List<Token>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t in (tokens).into_iter().cloned() {
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", t.id.clone())));
                    __mm_s.push_str(&*literal!(" ("));
                    __mm_s.push_str(&*tokenContent(t.clone())?);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        literal!("\n"),
    );
    Ok(r#str)
}

fn topTokenSourceInfo(mut tokens: &metamodelica::List<Token>) -> SourceInfo {
    let mut info: SourceInfo;
    let mut t: Token;
    info = (::match_deref::match_deref! { match tokens {
        Deref @ metamodelica::ListNode::Cons { head: __esc_t, tail: _ } => {
            t = (*__esc_t).clone();
            LexerModelicaDiff::tokenSourceInfo(t.clone())
        },
        _ => SourceInfo { fileName: literal!("<SimpleModelicaParser>"), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    info
}

fn needsWhitespaceBetweenTokens(mut first: Token, mut last: Token) -> bool {
    let mut b: bool;
    let notident: metamodelica::List<TokenId> = list![
        TokenId::ASSIGN.clone(),
        TokenId::BLOCK_COMMENT.clone(),
        TokenId::COLON.clone(),
        TokenId::COLONCOLON.clone(),
        TokenId::COMMA.clone(),
        TokenId::DOT.clone(),
        TokenId::EQEQ.clone(),
        TokenId::EQUALS.clone(),
        TokenId::GREATER.clone(),
        TokenId::GREATEREQ.clone(),
        TokenId::LBRACE.clone(),
        TokenId::LBRACK.clone(),
        TokenId::LESS.clone(),
        TokenId::LESSEQ.clone(),
        TokenId::LESSGT.clone(),
        TokenId::LINE_COMMENT.clone(),
        TokenId::LPAR.clone(),
        TokenId::MINUS.clone(),
        TokenId::MINUS_EW.clone(),
        TokenId::NEWLINE.clone(),
        TokenId::OPERATOR.clone(),
        TokenId::PLUS.clone(),
        TokenId::PLUS_EW.clone(),
        TokenId::POWER.clone(),
        TokenId::POWER_EW.clone(),
        TokenId::RBRACE.clone(),
        TokenId::RBRACK.clone(),
        TokenId::RPAR.clone(),
        TokenId::SEMICOLON.clone(),
        TokenId::SLASH.clone(),
        TokenId::SLASH_EW.clone(),
        TokenId::STAR.clone(),
        TokenId::STAR_EW.clone(),
        TokenId::UNSIGNED_INTEGER.clone(),
        TokenId::UNSIGNED_REAL.clone(),
        TokenId::WHITESPACE.clone()
    ];
    if listMember(tokenId(first), notident.clone()) || listMember(tokenId(last), notident) {
        b = false;
        return b;
    }
    b = true;
    b
}

fn tokenId(mut t: Token) -> TokenId {
    let mut id: TokenId;
    let LexerModelicaDiff::TOKEN { id: __pa0, .. } = t;
    id = metamodelica::Own::own(__pa0);
    id
}

pub mod First {
    use super::*;
    pub(crate) static class_prefixes: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| {
            list![
                TokenId::PARTIAL.clone(),
                TokenId::CLASS.clone(),
                TokenId::MODEL.clone(),
                TokenId::OPERATOR.clone(),
                TokenId::RECORD.clone(),
                TokenId::BLOCK.clone(),
                TokenId::EXPANDABLE.clone(),
                TokenId::CONNECTOR.clone(),
                TokenId::TYPE.clone(),
                TokenId::PACKAGE.clone(),
                TokenId::PURE.clone(),
                TokenId::IMPURE.clone(),
                TokenId::FUNCTION.clone()
            ]
        });

    pub(crate) static class_definition: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| {
            metamodelica::cons(
                TokenId::FINAL.clone(),
                metamodelica::cons(TokenId::ENCAPSULATED.clone(), class_prefixes.clone()),
            )
        });

    pub(crate) static type_prefix: std::sync::LazyLock<metamodelica::List<TokenId>> = std::sync::LazyLock::new(|| {
        list![
            TokenId::FLOW.clone(),
            TokenId::STREAM.clone(),
            TokenId::DISCRETE.clone(),
            TokenId::PARAMETER.clone(),
            TokenId::CONSTANT.clone(),
            TokenId::INPUT.clone(),
            TokenId::OUTPUT.clone()
        ]
    });

    pub(crate) static class_modification: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| list![TokenId::LPAR.clone()]);

    pub(crate) static _annotation: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| list![TokenId::ANNOTATION.clone()]);

    pub(crate) static element_redeclaration: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| list![TokenId::REDECLARE.clone()]);

    pub(crate) static name: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| list![TokenId::DOT.clone(), TokenId::IDENT.clone()]);

    pub(crate) static element_modification_or_replaceable: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| {
            metamodelica::cons(
                TokenId::EACH.clone(),
                metamodelica::cons(
                    TokenId::FINAL.clone(),
                    metamodelica::cons(TokenId::REPLACEABLE.clone(), name.clone()),
                ),
            )
        });

    pub(crate) static argument: std::sync::LazyLock<metamodelica::List<TokenId>> = std::sync::LazyLock::new(|| {
        listAppend(
            element_modification_or_replaceable.clone(),
            element_redeclaration.clone(),
        )
    });

    pub(crate) static modification: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| list![TokenId::LPAR.clone(), TokenId::EQUALS.clone(), TokenId::ASSIGN.clone()]);

    pub(crate) static component_clause: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| listAppend(type_prefix.clone(), name.clone()));

    pub(crate) static element: std::sync::LazyLock<metamodelica::List<TokenId>> = std::sync::LazyLock::new(|| {
        listAppend(
            component_clause.clone(),
            listAppend(
                class_definition.clone(),
                list![
                    TokenId::ANNOTATION.clone(),
                    TokenId::IMPORT.clone(),
                    TokenId::EXTENDS.clone(),
                    TokenId::REDECLARE.clone(),
                    TokenId::FINAL.clone(),
                    TokenId::INNER.clone(),
                    TokenId::OUTER.clone(),
                    TokenId::REPLACEABLE.clone()
                ],
            ),
        )
    });

    pub(crate) static statement: std::sync::LazyLock<metamodelica::List<TokenId>> = std::sync::LazyLock::new(|| {
        list![
            TokenId::DOT.clone(),
            TokenId::IDENT.clone(),
            TokenId::LPAR.clone(),
            TokenId::BREAK.clone(),
            TokenId::RETURN.clone(),
            TokenId::IF.clone(),
            TokenId::FOR.clone(),
            TokenId::WHILE.clone(),
            TokenId::WHEN.clone()
        ]
    });

    pub(crate) static component_reference: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| list![TokenId::DOT.clone(), TokenId::IDENT.clone()]);

    /*  constant list<TokenId> function_arguments =
      TokenId.FUNCTION ::
      TokenId.IDENT ::
      expression
    ; */
}

pub mod Follow {
    use super::*;
    pub(crate) static statement_equation: std::sync::LazyLock<metamodelica::List<TokenId>> =
        std::sync::LazyLock::new(|| {
            list![
                TokenId::INITIAL.clone(),
                TokenId::EQUATION.clone(),
                TokenId::ALGORITHM.clone(),
                TokenId::PUBLIC.clone(),
                TokenId::PROTECTED.clone(),
                TokenId::EXTERNAL.clone(),
                TokenId::ANNOTATION.clone(),
                TokenId::ELSE.clone(),
                TokenId::ELSEIF.clone(),
                TokenId::END.clone(),
                TokenId::ELSEWHEN.clone()
            ]
        });
}

pub(crate) const debug: bool = false;
