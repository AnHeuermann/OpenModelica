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

use crate::GlobalScriptDump;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_loader::Parser;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

/// rule to rewrite fromExp -> toExp,
///  there are FrontEnd and BackEnd rules
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Rule {
    /// rule to rewrite fromExp -> toExp, apply to FrontEnd AST exps
    FRONTEND_RULE {
        from: metamodelica::Ref<Absyn::Exp>,
        to: metamodelica::Ref<Absyn::Exp>,
    },
    /// rule to rewrite fromExp -> toExp, apply to the BackEnd AST exps
    BACKEND_RULE {
        from: metamodelica::Ref<Absyn::Exp>,
        to: metamodelica::Ref<Absyn::Exp>,
    },
}
impl metamodelica::gc::MMTrace for Rule {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Rule::FRONTEND_RULE { from, to } => {
                metamodelica::gc::MMTrace::mm_accept(from, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(to, __mmv)?;
                Ok(())
            }
            Rule::BACKEND_RULE { from, to } => {
                metamodelica::gc::MMTrace::mm_accept(from, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(to, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Rule::{BACKEND_RULE, FRONTEND_RULE};

pub type Rules = metamodelica::List<Rule>;

/// a bind '$1' bound to an exp
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Bind {
    /// a bind '$1' bound to an exp (frontend)
    FRONTEND_BIND {
        slot: metamodelica::Ref<Absyn::Exp>,
        value: metamodelica::Ref<Absyn::Exp>,
    },
    /// a bind '$1' bound to an exp (backend)
    BACKEND_BIND {
        slot: metamodelica::Ref<DAE::Exp>,
        value: metamodelica::Ref<DAE::Exp>,
    },
}
impl metamodelica::gc::MMTrace for Bind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Bind::FRONTEND_BIND { slot, value } => {
                metamodelica::gc::MMTrace::mm_accept(slot, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            Bind::BACKEND_BIND { slot, value } => {
                metamodelica::gc::MMTrace::mm_accept(slot, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Bind::{BACKEND_BIND, FRONTEND_BIND};

pub type Binds = metamodelica::List<Bind>;

// frontend rewrite stuff
// ----------------------
pub fn rewriteFrontEnd(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut isChanged: bool;
    (outExp, isChanged) = (match &**inExp {
        _ => {
            let mut rules: Rules;
            let mut b: bool;
            rules = getRulesFrontEnd(&(getAllRules()?));
            (outExp, b) = matchAndRewriteExpFrontEnd(inExp, &rules)?;
            (outExp, b)
        }
    });
    Ok((outExp, isChanged))
}

pub(crate) fn matchAndRewriteExpFrontEnd(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
    mut inRules: &Rules,
) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut changed: bool;
    (outExp, changed) = 'mc: {
        let __mc_input = &**inRules;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inExp.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Rule::FRONTEND_RULE { from, to }, tail: _ } => {
                    let mut binds: Binds;
                    let mut b: bool;
                    let mut outExp: metamodelica::Ref<Absyn::Exp> = outExp.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(matchesFrontEnd(inExp, metamodelica::AsArg::as_arg(&from), &(metamodelica::nil()))?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    binds = metamodelica::Own::own(__pa0);
                    outExp = rewriteExpFrontEnd(to.clone(), binds.clone())?;
                    b = boolNot(referenceEq(&*(&**inExp),&*(&*outExp)));
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FrontEnd Exp:     ")); __mm_s.push_str(&*Dump::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("FrontEnd From:    ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("FrontEnd To:      ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("FrontEnd Rewrite: ")); __mm_s.push_str(&*Dump::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!("\n---------\n")); ArcStr::from(__mm_s) });
                    Ok(((outExp.clone(), b), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut b: bool;
                    let mut outExp: metamodelica::Ref<Absyn::Exp> = outExp.clone();
                    (outExp, b) = matchAndRewriteExpFrontEnd(inExp, metamodelica::AsArg::as_arg(&rest))?;
                    Ok(((outExp.clone(), b), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, changed))
}

pub(crate) fn rewriteExpFrontEnd(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inBinds: Binds,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    (outExp, _) = AbsynUtil::traverseExp(
        inExp,
        (std::sync::Arc::new(replaceBindsFrontEnd)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::List<Bind>,
                    )
                        -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<Bind>)>
                    + 'static,
            >),
        inBinds,
    )?;
    Ok(outExp)
}

pub(crate) fn replaceBindsFrontEnd(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inBinds: Binds,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Binds)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outBinds: Binds;
    (outExp, outBinds) = (::match_deref::match_deref! { match &(inExp.clone()) {
        e1 @ Deref @ Absyn::Exp::CREF { componentRef: _ } => {
            let mut bnds = inBinds.clone();
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            e2 = replaceBindFrontEnd(e1.clone(), &bnds)?;
            (e2, bnds)
        },
        _ => {
            (inExp, inBinds)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outBinds))
}

pub(crate) fn replaceBindFrontEnd(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inBinds: &Binds,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    for mut bind in &**inBinds {
        let Bind::FRONTEND_BIND {
            slot: __pa0,
            value: __pa1,
        } = (bind.clone())
        else {
            return Err("pattern mismatch");
        };
        e = metamodelica::Own::own(__pa0);
        outExp = metamodelica::Own::own(__pa1);
        if AbsynUtil::expEqual(inExp.clone(), e)? {
            return Ok(outExp);
        }
    }
    outExp = inExp;
    Ok(outExp)
}

pub(crate) fn matchesFrontEnd(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
    mut inUnifyWith: &metamodelica::Ref<Absyn::Exp>,
    mut inAcc: &Binds,
) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = 'mc: {
        let __mc_input = (&**inExp, &**inUnifyWith);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Exp::CREF { componentRef: _ }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (isPlaceHolderFrontEnd(inUnifyWith)?) else { return Err("pattern mismatch") };
                    outBinds = metamodelica::cons(Bind::FRONTEND_BIND { slot: inUnifyWith.clone(), value: inExp.clone() }, inAcc.clone());
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { value: _ }, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::REAL { value: _ }, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::STRING { value: _ }, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BOOL { value: _ }, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: _ }, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BINARY { exp1: e1a, op: op1a, exp2: e2a }, Deref @ Absyn::Exp::BINARY { exp1: e1b, op: op1b, exp2: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { return Err("pattern mismatch") };
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: op1a, exp: e1a }, Deref @ Absyn::Exp::UNARY { op: op1b, exp: e1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { return Err("pattern mismatch") };
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LBINARY { exp1: e1a, op: op1a, exp2: e2a }, Deref @ Absyn::Exp::LBINARY { exp1: e1b, op: op1b, exp2: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { return Err("pattern mismatch") };
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LUNARY { op: op1a, exp: e1a }, Deref @ Absyn::Exp::LUNARY { op: op1b, exp: e1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { return Err("pattern mismatch") };
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RELATION { exp1: e1a, op: op1a, exp2: e2a }, Deref @ Absyn::Exp::RELATION { exp1: e1b, op: op1b, exp2: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { return Err("pattern mismatch") };
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::IFEXP { ifExp: cond1a, trueBranch: e1a, elseBranch: e2a, elseIfBranch: _ }, Deref @ Absyn::Exp::IFEXP { ifExp: cond1b, trueBranch: e1b, elseBranch: e2b, elseIfBranch: _ }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&cond1a), metamodelica::AsArg::as_arg(&cond1b), inAcc)?;
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), &outBinds)?;
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CALL { function_: cr1a, functionArgs: fargs1a, .. }, Deref @ Absyn::Exp::CALL { function_: cr1b, functionArgs: fargs1b, .. }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cr1a), metamodelica::AsArg::as_arg(&cr1b))?) else { return Err("pattern mismatch") };
                    outBinds = matchesFargsFrontEnd(metamodelica::AsArg::as_arg(&fargs1a), metamodelica::AsArg::as_arg(&fargs1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cr1a, functionArgs: fargs1a }, Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cr1b, functionArgs: fargs1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cr1a), metamodelica::AsArg::as_arg(&cr1b))?) else { return Err("pattern mismatch") };
                    outBinds = matchesFargsFrontEnd(metamodelica::AsArg::as_arg(&fargs1a), metamodelica::AsArg::as_arg(&fargs1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::ARRAY { arrayExp: exps1a }, Deref @ Absyn::Exp::ARRAY { arrayExp: exps1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstFrontEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::MATRIX { matrix: expsLst1a }, Deref @ Absyn::Exp::MATRIX { matrix: expsLst1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstLstFrontEnd(metamodelica::AsArg::as_arg(&expsLst1a), metamodelica::AsArg::as_arg(&expsLst1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RANGE { start: e1a, step: oe1a, stop: e2a }, Deref @ Absyn::Exp::RANGE { start: e1b, step: oe1b, stop: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesExpOptFrontEnd(oe1a.clone(), oe1b.clone(), outBinds.clone())?;
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::TUPLE { expressions: exps1a }, Deref @ Absyn::Exp::TUPLE { expressions: exps1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstFrontEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::END { .. }, Deref @ Absyn::Exp::END { .. }) => {
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code: _ }, Deref @ Absyn::Exp::CODE { code: _ }) => {
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::AS { id: id1a, exp: e1a }, Deref @ Absyn::Exp::AS { id: id1b, exp: e1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (stringEq(&id1a, &id1b)) else { return Err("pattern mismatch") };
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CONS { head: e1a, rest: e2a }, Deref @ Absyn::Exp::CONS { head: e1b, rest: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::MATCHEXP { .. }, Deref @ Absyn::Exp::MATCHEXP { .. }) => {
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LIST { exps: exps1a }, Deref @ Absyn::Exp::LIST { exps: exps1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstFrontEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outBinds)
}

pub(crate) fn matchesExpOptFrontEnd(
    mut inOExp1: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inOExp2: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match &((inOExp1, inOExp2)) {
        (None, None) => {
            inAcc
        },
        (Some(e1a), Some(e1b)) => {
            outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), &inAcc)?;
            outBinds
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinds)
}

pub(crate) fn matchesExpLstFrontEnd<'__b>(
    mut inExps1: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inExps2: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match (inExps1, inExps2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            inAcc
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::ListNode::Cons { head: e1b, tail: exps1b }) => {
            outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), &inAcc)?;
            outBinds = matchesExpLstFrontEnd(exps1a, exps1b, outBinds)?;
            outBinds
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub(crate) fn matchesFargsFrontEnd(
    mut inFargs1: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut inFargs2: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match (inFargs1, inFargs2) {
        (Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: exps1a, argNames: nargs1a }, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: exps1b, argNames: nargs1b }) => {
            outBinds = matchesExpLstFrontEnd(exps1a, exps1b, inAcc)?;
            let true = (intEq(((nargs1a).len() as i32), ((nargs1b).len() as i32))) else { return Err("pattern mismatch") };
            outBinds = matchesNargsFrontEnd(&(sortNargsFrontEnd(nargs1a.clone())?), &(sortNargsFrontEnd(nargs1b.clone())?), outBinds)?;
            outBinds
        },
        (Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp: e1a, iterType: _, iterators: _ }, Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp: e1b, iterType: _, iterators: _ }) => {
            outBinds = matchesFrontEnd(e1a, e1b, &inAcc)?;
            outBinds
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub(crate) fn sortNargsFrontEnd(
    mut inNargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>> {
    let mut outNargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    outNargs = List::sort(
        inNargs,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Absyn::NamedArg>,
                  __a1: metamodelica::Ref<Absyn::NamedArg>|
                  -> metamodelica::Result<_> { ::std::result::Result::Ok(inNargComp(&__a0, &__a1)) },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::NamedArg>,
                        metamodelica::Ref<Absyn::NamedArg>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    Ok(outNargs)
}

pub(crate) fn inNargComp(
    mut inNarg1: &metamodelica::Ref<Absyn::NamedArg>,
    mut inNarg2: &metamodelica::Ref<Absyn::NamedArg>,
) -> bool {
    let mut isGreater: bool;
    let mut id1: ArcStr;
    let mut id2: ArcStr;
    let __arc1 = &(*inNarg1);
    let Absyn::NAMEDARG { argName: __pa0, .. } = &**__arc1;
    id1 = metamodelica::Own::own(__pa0);
    let __arc3 = &(*inNarg2);
    let Absyn::NAMEDARG { argName: __pa2, .. } = &**__arc3;
    id2 = metamodelica::Own::own(__pa2);
    isGreater = intGt(stringCompare(&id1, &id2), 0);
    isGreater
}

pub(crate) fn matchesNargsFrontEnd<'__b>(
    mut inNargs1: &'__b metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inNargs2: &'__b metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match (inNargs1, inNargs2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            inAcc
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: n1a, argValue: e1a }, tail: nargs1a }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: n1b, argValue: e1b }, tail: nargs1b }) => {
            let true = (stringEq(&n1a, &n1b)) else { return Err("pattern mismatch") };
            outBinds = matchesFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), &inAcc)?;
            outBinds = matchesNargsFrontEnd(nargs1a, nargs1b, outBinds)?;
            outBinds
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub(crate) fn matchesExpLstLstFrontEnd<'__b>(
    mut inExps1: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
    mut inExps2: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match (inExps1, inExps2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            inAcc
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::ListNode::Cons { head: e1b, tail: exps1b }) => {
            outBinds = matchesExpLstFrontEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
            outBinds = matchesExpLstLstFrontEnd(exps1a, exps1b, outBinds)?;
            outBinds
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub(crate) fn isPlaceHolderFrontEnd(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<bool> {
    let mut isHolder: bool;
    isHolder = (::match_deref::match_deref! { match inExp {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: _ } } => {
            let mut b: bool;
            b = intEq(System::stringFind(name.clone(), literal!("'$"))?, 0);
            b
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isHolder)
}

// backend rewrite stuff
// ----------------------
pub(crate) fn rewriteBackEnd(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut isChanged: bool;
    (outExp, isChanged) = (match &**inExp {
        _ => {
            let mut rules: Rules;
            let mut b: bool;
            rules = getRulesBackEnd(&(getAllRules()?));
            (outExp, b) = matchAndRewriteExpBackEnd(inExp, &rules)?;
            (outExp, b)
        }
    });
    Ok((outExp, isChanged))
}

pub(crate) fn matchAndRewriteExpBackEnd(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inRules: &Rules,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut changed: bool;
    (outExp, changed) = 'mc: {
        let __mc_input = &**inRules;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inExp.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Rule::BACKEND_RULE { from: afrom, to: ato }, tail: _ } => {
                    let mut from: metamodelica::Ref<DAE::Exp>;
                    let mut to: metamodelica::Ref<DAE::Exp>;
                    let mut binds: Binds;
                    let mut b: bool;
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    from = Expression::fromAbsynExp(afrom.clone())?;
                    to = Expression::fromAbsynExp(ato.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(matchesBackEnd(inExp, &from, &(metamodelica::nil()))?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    binds = metamodelica::Own::own(__pa0);
                    outExp = rewriteExpBackEnd(to.clone(), binds.clone())?;
                    b = boolNot(referenceEq(&*(&**inExp),&*(&*outExp)));
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackEnd Exp:     ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("BackEnd From:    ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(from.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("BackEnd To:      ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("BackEnd Rewrite: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!("\n---------\n")); ArcStr::from(__mm_s) });
                    Ok(((outExp.clone(), b), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut b: bool;
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    (outExp, b) = matchAndRewriteExpBackEnd(inExp, metamodelica::AsArg::as_arg(&rest))?;
                    Ok(((outExp.clone(), b), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, changed))
}

pub(crate) fn rewriteExpBackEnd(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBinds: Binds,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outExp, _) = Expression::traverseExpBottomUp(inExp, &replaceBindsBackEnd, inBinds)?;
    Ok(outExp)
}

pub(crate) fn replaceBindsBackEnd(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBinds: Binds,
) -> Result<(metamodelica::Ref<DAE::Exp>, Binds)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outBinds: Binds;
    (outExp, outBinds) = (::match_deref::match_deref! { match &(inExp.clone()) {
        e1 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => {
            let mut bnds = inBinds.clone();
            let mut e2: metamodelica::Ref<DAE::Exp>;
            e2 = replaceBindBackEnd(e1.clone(), &bnds)?;
            (e2, bnds)
        },
        _ => {
            (inExp, inBinds)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outBinds))
}

pub(crate) fn replaceBindBackEnd(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBinds: &Binds,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut to: metamodelica::Ref<DAE::Exp>;
    for mut bind in &**inBinds {
        let Bind::BACKEND_BIND {
            slot: __pa0,
            value: __pa1,
        } = (bind.clone())
        else {
            return Err("pattern mismatch");
        };
        e = metamodelica::Own::own(__pa0);
        to = metamodelica::Own::own(__pa1);
        if expEqual(&inExp, e)? {
            outExp = to;
            return Ok(outExp);
        }
    }
    outExp = inExp;
    Ok(outExp)
}

pub(crate) fn matchesBackEnd(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inUnifyWith: &metamodelica::Ref<DAE::Exp>,
    mut inAcc: &Binds,
) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = 'mc: {
        let __mc_input = (&**inExp, &**inUnifyWith);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (isPlaceHolderBackEnd(inUnifyWith)?) else { return Err("pattern mismatch") };
                    outBinds = metamodelica::cons(Bind::BACKEND_BIND { slot: inUnifyWith.clone(), value: inExp.clone() }, inAcc.clone());
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: _ }, _) => {
                    let true = (expEqual(inExp, inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: _ }, _) => {
                    let true = (expEqual(inExp, inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: _ }, _) => {
                    let true = (expEqual(inExp, inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: _ }, _) => {
                    let true = (expEqual(inExp, inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, _) => {
                    let true = (expEqual(inExp, inUnifyWith.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1a, operator: op1a, exp2: e2a }, Deref @ DAE::Exp::BINARY { exp1: e1b, operator: op1b, exp2: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (operatorMatches(metamodelica::AsArg::as_arg(&op1a), metamodelica::AsArg::as_arg(&op1b))?) else { return Err("pattern mismatch") };
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: op1a, exp: e1a }, Deref @ DAE::Exp::UNARY { operator: op1b, exp: e1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (operatorMatches(metamodelica::AsArg::as_arg(&op1a), metamodelica::AsArg::as_arg(&op1b))?) else { return Err("pattern mismatch") };
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1a, operator: op1a, exp2: e2a }, Deref @ DAE::Exp::LBINARY { exp1: e1b, operator: op1b, exp2: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (operatorMatches(metamodelica::AsArg::as_arg(&op1a), metamodelica::AsArg::as_arg(&op1b))?) else { return Err("pattern mismatch") };
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: op1a, exp: e1a }, Deref @ DAE::Exp::LUNARY { operator: op1b, exp: e1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (operatorMatches(metamodelica::AsArg::as_arg(&op1a), metamodelica::AsArg::as_arg(&op1b))?) else { return Err("pattern mismatch") };
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1a, operator: op1a, exp2: e2a, index: _, optionExpisASUB: _ }, Deref @ DAE::Exp::RELATION { exp1: e1b, operator: op1b, exp2: e2b, index: _, optionExpisASUB: _ }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (operatorMatches(metamodelica::AsArg::as_arg(&op1a), metamodelica::AsArg::as_arg(&op1b))?) else { return Err("pattern mismatch") };
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: cond1a, expThen: e1a, expElse: e2a }, Deref @ DAE::Exp::IFEXP { expCond: cond1b, expThen: e1b, expElse: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&cond1a), metamodelica::AsArg::as_arg(&cond1b), inAcc)?;
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), &outBinds)?;
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: p1a, expLst: exps1a, attr: _ }, Deref @ DAE::Exp::CALL { path: p1b, expLst: exps1b, attr: _ }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1a), metamodelica::AsArg::as_arg(&p1b))) else { return Err("pattern mismatch") };
                    outBinds = matchesExpLstBackEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { path: p1a, expList: exps1a, ty: _, origType: _ }, Deref @ DAE::Exp::PARTEVALFUNCTION { path: p1b, expList: exps1b, ty: _, origType: _ }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1a), metamodelica::AsArg::as_arg(&p1b))) else { return Err("pattern mismatch") };
                    outBinds = matchesExpLstBackEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: exps1a, .. }, Deref @ DAE::Exp::ARRAY { array: exps1b, .. }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstBackEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: expsLst1a, .. }, Deref @ DAE::Exp::MATRIX { matrix: expsLst1b, .. }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstLstBackEnd(metamodelica::AsArg::as_arg(&expsLst1a), metamodelica::AsArg::as_arg(&expsLst1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { ty: _, start: e1a, step: oe1a, stop: e2a }, Deref @ DAE::Exp::RANGE { ty: _, start: e1b, step: oe1b, stop: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesExpOptBackEnd(oe1a.clone(), oe1b.clone(), outBinds.clone())?;
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: exps1a }, Deref @ DAE::Exp::TUPLE { PR: exps1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstBackEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CONS { car: e1a, cdr: e2a }, Deref @ DAE::Exp::CONS { car: e1b, cdr: e2b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
                    outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e2a), metamodelica::AsArg::as_arg(&e2b), &outBinds)?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATCHEXPRESSION { .. }, Deref @ DAE::Exp::MATCHEXPRESSION { .. }) => {
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: exps1a }, Deref @ DAE::Exp::LIST { valList: exps1b }) => {
                    let mut outBinds: metamodelica::List<Bind> = outBinds.clone();
                    outBinds = matchesExpLstBackEnd(metamodelica::AsArg::as_arg(&exps1a), metamodelica::AsArg::as_arg(&exps1b), inAcc.clone())?;
                    Ok((outBinds.clone(), outBinds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBinds = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outBinds)
}

pub(crate) fn matchesExpOptBackEnd(
    mut inOExp1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inOExp2: Option<metamodelica::Ref<DAE::Exp>>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match &((inOExp1, inOExp2)) {
        (None, None) => {
            inAcc
        },
        (Some(e1a), Some(e1b)) => {
            outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), &inAcc)?;
            outBinds
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinds)
}

pub(crate) fn matchesExpLstBackEnd<'__b>(
    mut inExps1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExps2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match (inExps1, inExps2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            inAcc
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::ListNode::Cons { head: e1b, tail: exps1b }) => {
            outBinds = matchesBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), &inAcc)?;
            outBinds = matchesExpLstBackEnd(exps1a, exps1b, outBinds)?;
            outBinds
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub(crate) fn matchesExpLstLstBackEnd<'__b>(
    mut inExps1: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inExps2: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inAcc: Binds,
) -> Result<Binds> {
    let mut outBinds: Binds;
    outBinds = (::match_deref::match_deref! { match (inExps1, inExps2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            inAcc
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::ListNode::Cons { head: e1b, tail: exps1b }) => {
            outBinds = matchesExpLstBackEnd(metamodelica::AsArg::as_arg(&e1a), metamodelica::AsArg::as_arg(&e1b), inAcc)?;
            outBinds = matchesExpLstLstBackEnd(exps1a, exps1b, outBinds)?;
            outBinds
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub(crate) fn isPlaceHolderBackEnd(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut isHolder: bool;
    isHolder = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, ty: _ } => {
            let mut b: bool;
            b = intEq(System::stringFind(name.clone(), literal!("'$"))?, 0);
            b
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isHolder)
}

fn expEqual(mut e1: &metamodelica::Ref<DAE::Exp>, mut e2: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ DAE::Exp::ICONST { integer: i }, Deref @ DAE::Exp::RCONST { real: r }) if (realEq(intReal(i.clone()), r.clone())) => {
            true
        },
        (Deref @ DAE::Exp::RCONST { real: r }, Deref @ DAE::Exp::ICONST { integer: i }) if (realEq(intReal(i.clone()), r.clone())) => {
            true
        },
        _ => {
            ExpressionBasics::expEqual(e1, e2)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

fn operatorMatches(mut op1: &DAE::Operator, mut op2: &DAE::Operator) -> Result<bool> {
    let mut b: bool;
    b = (match (op1.clone(), op2.clone()) {
        (DAE::Operator::UMINUS_ARR { .. }, DAE::Operator::UMINUS { .. }) => true,
        (DAE::Operator::ADD_ARR { .. }, DAE::Operator::ADD { .. }) => true,
        (DAE::Operator::SUB_ARR { .. }, DAE::Operator::SUB { .. }) => true,
        (DAE::Operator::MUL_ARR { .. }, DAE::Operator::MUL { .. }) => true,
        (DAE::Operator::DIV_ARR { .. }, DAE::Operator::DIV { .. }) => true,
        (DAE::Operator::MUL_ARRAY_SCALAR { .. }, DAE::Operator::MUL { .. }) => true,
        (DAE::Operator::ADD_ARRAY_SCALAR { .. }, DAE::Operator::ADD { .. }) => true,
        (DAE::Operator::SUB_SCALAR_ARRAY { .. }, DAE::Operator::SUB { .. }) => true,
        (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, DAE::Operator::MUL { .. }) => true,
        (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, DAE::Operator::MUL { .. }) => true,
        (DAE::Operator::DIV_SCALAR_ARRAY { .. }, DAE::Operator::DIV { .. }) => true,
        (DAE::Operator::DIV_ARRAY_SCALAR { .. }, DAE::Operator::DIV { .. }) => true,
        (DAE::Operator::POW_SCALAR_ARRAY { .. }, DAE::Operator::POW { .. }) => true,
        (DAE::Operator::POW_ARRAY_SCALAR { .. }, DAE::Operator::POW { .. }) => true,
        (DAE::Operator::POW_ARR { .. }, DAE::Operator::POW { .. }) => true,
        (DAE::Operator::POW_ARR2 { .. }, DAE::Operator::POW { .. }) => true,
        _ => Expression::operatorEqual(op1, op2)?,
    });
    Ok(b)
}

pub fn loadRules() -> Result<()> {
    let () = (match () {
        () => {
            let mut file: ArcStr;
            file = Flags::getConfigString(Flags::REWRITE_RULES_FILE.clone())?;
            loadRulesFromFile(file)?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn noRewriteRules() -> bool {
    let mut noRules: bool;
    noRules = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) {
                None => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    noRules
}

pub fn noRewriteRulesFrontEnd() -> bool {
    let mut noRules: bool;
    noRules = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) {
                None => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(getRulesFrontEnd(&(getAllRules()?))) {
                Deref @ metamodelica::ListNode::Nil => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    noRules
}

pub fn noRewriteRulesBackEnd() -> bool {
    let mut noRules: bool;
    noRules = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) {
                None => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(getRulesBackEnd(&(getAllRules()?))) {
                Deref @ metamodelica::ListNode::Nil => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    noRules
}

pub(crate) fn loadRulesFromFile(mut inFile: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    { let __v = None; crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut oR: Option<metamodelica::List<Rule>>;
                    oR = crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone());
                    let true = ((oR).is_some()) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut stmts: metamodelica::List<GlobalScript::Statement>;
                    let mut rules: Rules;
                    ::match_deref::match_deref! { match &(crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) {
                        None => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    let GlobalScript::ISTMTS { interactiveStmtLst: __pa0, semicolon: _ } = Parser::parseexp(inFile.clone())?;
                    stmts = metamodelica::Own::own(__pa0);
                    rules = stmtsToRules(&stmts, &(metamodelica::nil()))?;
                    metamodelica::print(literal!("-------------\n"));
                    { let __v = Some(rules.clone()); crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unable to parse rewrite rules file: ")); __mm_s.push_str(&*inFile); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("Script/RewriteRules.mo"))?;
                    { let __v = None; crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub fn clearRules() -> () {
    {
        let __v = None;
        crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

pub(crate) fn getAllRules() -> Result<Rules> {
    let mut outRules: Rules;
    let mut orules: Option<metamodelica::List<Rule>>;
    orules = crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone());
    let __pa0 = ::match_deref::match_deref! { match &(orules) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outRules = metamodelica::Own::own(__pa0);
    Ok(outRules)
}

pub(crate) fn getRulesFrontEnd<'__b>(mut inRules: &'__b Rules) -> Rules {
    '__tco: loop {
        ::match_deref::match_deref! { match inRules {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: r @ Rule::FRONTEND_RULE { .. }, tail: rest } => {
                let mut lst: Rules;
                lst = getRulesFrontEnd(rest);
                return metamodelica::cons(r.clone(), lst)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inRules = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn getRulesBackEnd<'__b>(mut inRules: &'__b Rules) -> Rules {
    '__tco: loop {
        ::match_deref::match_deref! { match inRules {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: r @ Rule::BACKEND_RULE { .. }, tail: rest } => {
                let mut lst: Rules;
                lst = getRulesBackEnd(rest);
                return metamodelica::cons(r.clone(), lst)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inRules = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn stmtsToRules(mut inStmts: &metamodelica::List<GlobalScript::Statement>, mut inAcc: &Rules) -> Result<Rules> {
    let mut outRules: Rules;
    outRules = 'mc: {
        let __mc_input = &**inStmts;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inAcc.clone().reverse())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rewrite", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: from, tail: Deref @ metamodelica::ListNode::Cons { head: to, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, tail: rest } => {
                    let mut acc: Rules;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FrontEnd rule: ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    acc = stmtsToRules(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(Rule::FRONTEND_RULE { from: from.clone(), to: to.clone() }, inAcc.clone())))?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rewriteFrontEnd", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: from, tail: Deref @ metamodelica::ListNode::Cons { head: to, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, tail: rest } => {
                    let mut acc: Rules;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FrontEnd rule: ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    acc = stmtsToRules(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(Rule::FRONTEND_RULE { from: from.clone(), to: to.clone() }, inAcc.clone())))?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rewriteBackEnd", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: from, tail: Deref @ metamodelica::ListNode::Cons { head: to, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, tail: rest } => {
                    let mut acc: Rules;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackEnd rule: ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    acc = stmtsToRules(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(Rule::BACKEND_RULE { from: from.clone(), to: to.clone() }, inAcc.clone())))?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: s, tail: _ } => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unable to parse rewrite rule: ")); __mm_s.push_str(&*GlobalScriptDump::printIstmtStr(metamodelica::AsArg::as_arg(&s))?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("Script/RewriteRules.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRules)
}
