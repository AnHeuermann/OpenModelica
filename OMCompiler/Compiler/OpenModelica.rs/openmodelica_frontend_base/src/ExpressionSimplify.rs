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

use crate::ComponentReference;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionDump;
use crate::Types;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_inst::ExpressionSimplifyTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
pub type ComponentRef = metamodelica::Ref<DAE::ComponentRef>;

pub type Ident = ArcStr;

pub type Operator = DAE::Operator;

pub type Type = metamodelica::Ref<DAE::Type>;

pub type Subscript = metamodelica::Ref<DAE::Subscript>;

// protected imports
pub(crate) static optionSimplifyOnly: std::sync::LazyLock<ExpressionSimplifyTypes::Evaluate> =
    std::sync::LazyLock::new(|| ExpressionSimplifyTypes::optionSimplifyOnly.clone());

pub fn simplify(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut hasChanged: bool;
    (outExp, hasChanged) = simplifyWithOptions(inExp, optionSimplifyOnly.clone())?;
    Ok((outExp, hasChanged))
}

pub fn condsimplify(
    mut cond: bool,
    mut ioExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut ioExp: metamodelica::Ref<DAE::Exp> = ioExp;
    let mut hasChanged: bool = false;
    if cond {
        (ioExp, hasChanged) = simplifyWithOptions(ioExp, optionSimplifyOnly.clone())?;
    }
    Ok((ioExp, hasChanged))
}

pub fn simplifyBinaryExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*inExp.clone() {
        DAE::Exp::BINARY {
            exp1: e1,
            operator: op,
            exp2: e2,
        } => simplifyBinary(&inExp, op.clone(), e1.clone(), e2.clone())?,
        _ => inExp,
    });
    Ok(outExp)
}

pub fn simplifyUnaryExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*inExp.clone() {
        DAE::Exp::UNARY { exp: e1, operator: op } => simplifyUnary(inExp, op.clone(), e1.clone()),
        _ => inExp,
    });
    outExp
}

fn simplifyWithOptions(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut options: ExpressionSimplifyTypes::Evaluate,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut hasChanged: bool;
    (outExp, hasChanged) = 'mc: {
        let __mc_input = (inExp, options);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, ExpressionSimplifyTypes::Evaluate::DO_EVAL { .. }) => {
                    let mut eNew: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    (eNew, _) = simplify1WithOptions(e.clone(), options.clone())?;
                    Error::assertionOrAddSourceMessage(Expression::isConstValue(&eNew)?, &(Error::INTERNAL_ERROR.clone()), list![literal!("eval exp failed")], &(Absyn::dummyInfo.clone()))?;
                    b = !(ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e), eNew.clone())?);
                    Ok((eNew.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let mut eNew: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let false = (Config::getNoSimplify()?) else { return Err("pattern mismatch") };
                    (eNew, _) = simplify1WithOptions(e.clone(), options)?;
                    eNew = simplify2(eNew.clone(), true, true)?;
                    (eNew, _) = simplify1WithOptions(eNew.clone(), options)?;
                    b = !(ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e), eNew.clone())?);
                    Ok((eNew.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let mut eNew: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    (eNew, b) = simplify1WithOptions(e.clone(), options)?;
                    Ok((eNew.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, hasChanged))
}

pub(crate) fn simplifyTraverseHelper<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inA: A,
) -> Result<(metamodelica::Ref<DAE::Exp>, A)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut a: A;
    a = inA;
    (exp, _) = simplify(inExp)?;
    Ok((exp, a))
}

pub fn simplify1TraverseHelper<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inA: A,
) -> Result<(metamodelica::Ref<DAE::Exp>, A)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut a: A;
    a = inA;
    (outExp, _) = simplify1(inExp)?;
    Ok((outExp, a))
}

pub(crate) fn simplify1time(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outE: metamodelica::Ref<DAE::Exp>;
    let mut t1: metamodelica::Real;
    let mut t2: metamodelica::Real;
    t1 = clock();
    (outE, _) = simplify1(e.clone())?;
    t2 = clock();
    metamodelica::print(if (t2 - t1 > metamodelica::OrderedFloat(0.01_f64)) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("simplify1 took "));
            __mm_s.push_str(&*realString(t2 - t1));
            __mm_s.push_str(&*literal!(" seconds for exp: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e)?);
            __mm_s.push_str(&*literal!(" \nsimplified to :"));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(outE.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        }
    } else {
        literal!("")
    });
    Ok(outE)
}

pub fn simplifyWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut options: ExpressionSimplifyTypes::Evaluate,
) -> Result<(metamodelica::Ref<DAE::Exp>, ExpressionSimplifyTypes::Evaluate)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outOptions: ExpressionSimplifyTypes::Evaluate;
    (outExp, outOptions) = (match &*inExp.clone() {
        DAE::Exp::SIZE { exp: e1, sz: oe } => (simplifySize(inExp, e1.clone(), oe.clone()), options),
        DAE::Exp::CAST { ty: tp, exp: e } => {
            let mut e = (*e).clone();
            e = simplifyCast(inExp, e.clone(), tp.clone())?;
            (e.clone(), options)
        }
        DAE::Exp::ASUB { exp: e, sub: subs } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e = (*e).clone();
            expl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            e = simplifyAsubExp(inExp, e.clone(), expl);
            (e.clone(), options)
        }
        DAE::Exp::TSUB { .. } => (simplifyTSub(inExp)?, options),
        DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = simplifyUnary(inExp, op.clone(), e1.clone());
            (e, options)
        }
        DAE::Exp::BINARY {
            exp1: e1,
            operator: op,
            exp2: e2,
        } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = simplifyBinary(&inExp, op.clone(), e1.clone(), e2.clone())?;
            (e, options)
        }
        DAE::Exp::RELATION {
            exp1: e1,
            operator: op,
            exp2: e2,
            index: index_,
            optionExpisASUB: isExpisASUB,
        } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = simplifyRelation(
                inExp,
                op.clone(),
                e1.clone(),
                e2.clone(),
                index_.clone(),
                isExpisASUB.clone(),
            );
            (e, options)
        }
        DAE::Exp::LUNARY { operator: op, exp: e1 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = simplifyUnary(inExp, op.clone(), e1.clone());
            (e, options)
        }
        DAE::Exp::LBINARY {
            exp1: e1,
            operator: op,
            exp2: e2,
        } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = simplifyLBinary(inExp, metamodelica::AsArg::as_arg(&op), e1.clone(), e2.clone())?;
            (e, options)
        }
        DAE::Exp::IFEXP {
            expCond: e1,
            expThen: e2,
            expElse: e3,
        } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = simplifyIfExp(inExp, e1.clone(), e2.clone(), e3.clone())?;
            (e, options)
        }
        DAE::Exp::CREF {
            componentRef: c_1,
            ty: t,
        } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = simplifyCref(inExp, metamodelica::AsArg::as_arg(&c_1), t.clone());
            (e, options)
        }
        DAE::Exp::REDUCTION {
            reductionInfo,
            expr: e1,
            iterators: riters,
        } => {
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut b2: bool;
            let mut riters = (*riters).clone();
            (riters, b2) = simplifyReductionIterators(riters.clone(), metamodelica::nil(), false)?;
            exp1 = if (b2) {
                metamodelica::Ref::new(DAE::Exp::REDUCTION {
                    reductionInfo: reductionInfo.clone(),
                    expr: e1.clone(),
                    iterators: riters.clone(),
                })
            } else {
                inExp
            };
            (simplifyReduction(exp1), options)
        }
        DAE::Exp::CALL { .. } => (simplifyCall(inExp), options),
        DAE::Exp::RSUB { .. } => (simplifyRSub(inExp)?, options),
        DAE::Exp::MATCHEXPRESSION { .. } => (simplifyMatch(inExp), options),
        DAE::Exp::UNBOX { .. } => (simplifyUnbox(inExp), options),
        DAE::Exp::BOX { .. } => (simplifyUnbox(inExp), options),
        DAE::Exp::CONS { .. } => (simplifyCons(inExp), options),
        _ => (inExp, options),
    });
    Ok((outExp, outOptions))
}

fn simplifyRSub(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::RSUB { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, ix: (-1), fieldName: __e_fieldName, ty: __e_ty } => {
            metamodelica::Ref::new(DAE::Exp::CREF { componentRef: ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&cr), ComponentReferenceBasics::makeCrefIdent(__e_fieldName.clone(), __e_ty.clone(), metamodelica::nil()))?, ty: __e_ty.clone() })
        },
        Deref @ DAE::Exp::RSUB { exp: Deref @ DAE::Exp::CALL { path: p1, expLst: exps, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, varLst: vars, .. }, .. } }, ix: (-1), fieldName: __e_fieldName, .. } if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) => {
            (exps).get(List::position1OnTrue(&(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), &fnptr!(stringEq, ArcStr, ArcStr), __e_fieldName.clone())?)?
        },
        Deref @ DAE::Exp::RSUB { exp: Deref @ DAE::Exp::RECORD { exps, comp, .. }, ix: (-1), fieldName: __e_fieldName, .. } => {
            (exps).get(List::position1OnTrue(metamodelica::AsArg::as_arg(&comp), &fnptr!(stringEq, ArcStr, ArcStr), __e_fieldName.clone())?)?
        },
        _ => {
            e
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(e)
}

fn simplifyAsubExp(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSubs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp.clone(), &*inSubs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Exp::CAST { ty: tp, exp: e }, _) => {
                            let mut tp = (*tp).clone();
                            let mut e = (*e).clone();
                            tp = Expression::unliftArray(metamodelica::AsArg::as_arg(&tp))?;
                            e = metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: metamodelica::Ref::new(DAE::Exp::ASUB { exp: e.clone(), sub: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
                for mut s in (inSubs.clone()).into_iter().cloned() {
                            let __x = Expression::makeIndexSubscript(s.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }) }) });
                            Ok(e.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: eLst }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: sub }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    if !((sub.clone() <= ((eLst).len() as i32))) { return Err("guard") }
                    Ok((eLst).get(sub.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(simplifyAsubSlicing(inExp.clone(), inSubs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    for mut exp in &*inSubs {
                        Expression::expInt(metamodelica::AsArg::as_arg(&exp))?;
                    }
                    Ok(List::foldr(&inSubs, &simplifyAsub, inExp.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (_, _) => {
                            let mut istart: i32;
                            let mut istep: i32;
                            let mut istop: i32;
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut hasRange: bool;
                            let mut step: Option<metamodelica::Ref<DAE::Exp>>;
                            hasRange = false;
                            subs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut exp in (inSubs.clone()).into_iter().cloned() {
                            let __x = (::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: __esc_istart }, step: __esc_step, stop: Deref @ DAE::Exp::ICONST { integer: __esc_istop }, .. } => {
                            istart = (*__esc_istart).clone();
                            step = (*__esc_step).clone();
                            istop = (*__esc_istop).clone();
                            e = Expression::makeArray(({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut i in (simplifyRange(istart.clone(), (::match_deref::match_deref! { match &(step.clone()) {
                None => 1,
                Some(Deref @ DAE::Exp::ICONST { integer: __esc_istep }) => {
                            istep = (*__esc_istep).clone();
                            istep.clone()
                },
                _ => return Err("match: no arm matched"),
            } }), istop.clone())?).into_iter().cloned() {
                            let __x = metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), DAE::T_INTEGER_DEFAULT().clone(), true);
                            hasRange = true;
                            e.clone()
                },
                _ => exp.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            let true = (hasRange) else { return Err("pattern mismatch") };
                            Ok(metamodelica::Ref::new(DAE::Exp::ASUB { exp: inExp.clone(), sub: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
                for mut s in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::makeIndexSubscript(s.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }) }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn simplifyCall(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::REDUCTION { reductionInfo: ri @ Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, .. }, iterators: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    if !((listMember(name.clone(), list![literal!("sum"), literal!("product"), literal!("min"), literal!("max")]))) { return Err("guard") }
                    let mut e = (*e).clone();
                    assign_variant_field!(e => DAE::Exp::REDUCTION; reductionInfo = metamodelica::Ref::new(DAE::ReductionInfo { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: tp.clone(), defaultValue: Some(reductionDefaultValue(metamodelica::AsArg::as_arg(&name), metamodelica::AsArg::as_arg(&tp))?), foldName: ri.foldName.clone(), resultName: ri.resultName.clone(), foldExp: Some(reductionExpression(metamodelica::AsArg::as_arg(&name), tp.clone(), ri.foldName.clone(), ri.resultName.clone())?) }));
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut b2: bool;
                    b2 = Expression::isRelation(metamodelica::AsArg::as_arg(&e)) || Expression::isEventTriggeringFunctionExp(metamodelica::AsArg::as_arg(&e));
                    Ok(if (!(b2)) {simplifyNoEvent(e.clone())?} else {inExp.clone()})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, tail: Deref @ metamodelica::ListNode::Nil }, attr } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: list![e1.clone()], attr: attr.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, tail: Deref @ metamodelica::ListNode::Nil }, attr } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: list![e1.clone()], attr: attr.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut b2: bool;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2) {e.clone()} else {inExp.clone()})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut b2: bool;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2) {e.clone()} else {inExp.clone()})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut b2: bool;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2) {metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })} else {inExp.clone()})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut b2: bool;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2) {metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })} else {inExp.clone()})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), &fnptr!(preCref, metamodelica::Ref<DAE::Exp>, bool), false)?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), &fnptr!(previousCref, metamodelica::Ref<DAE::Exp>, bool), false)?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), &fnptr!(changeCref, metamodelica::Ref<DAE::Exp>, bool), false)?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), &fnptr!(edgeCref, metamodelica::Ref<DAE::Exp>, bool), false)?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: expl, attr: Deref @ DAE::CallAttributes { isImpure: false, .. } } => {
                    if !((Expression::isConstWorkList(expl.clone())?)) { return Err("guard") }
                    Ok(simplifyBuiltinConstantCalls(metamodelica::AsArg::as_arg(&idn), &inExp)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. } => {
                    Ok(simplifyBuiltinCalls(inExp.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "identity" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: n }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                            let mut matrix: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            matrix = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut j in (1..=n.clone()).into_iter() {
                            let __x = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() })] }), scalar: true, array: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut i in (1..=n.clone()).into_iter() {
                            let __x = if (i.clone() == j.clone()) {metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 })} else {metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })};
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }) });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() }), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() })] }), scalar: false, array: matrix.clone() }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "diagonal" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, ty: tp, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                            let mut zero: metamodelica::Ref<DAE::Exp>;
                            let mut matrix: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut n: i32;
                            let mut tp = (*tp).clone();
                            n = ((expl).len() as i32);
                            tp = Types::arrayElementType(metamodelica::AsArg::as_arg(&tp));
                            zero = Expression::makeConstZero(metamodelica::AsArg::as_arg(&tp));
                            matrix = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut j in (1..=n).into_iter() {
                            let __x = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: n })] }), scalar: true, array: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut i in (1..=n).into_iter() {
                            let __x = if (i.clone() == j.clone()) {(expl).get(i.clone())?} else {zero.clone()};
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }) });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: n }), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: n })] }), scalar: false, array: matrix.clone() }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn2 }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    if !((metamodelica::stringEq(&idn, &(literal!("tan"))) && metamodelica::stringEq(&idn2, &(literal!("atan"))))) { return Err("guard") }
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !((r2.clone() != metamodelica::OrderedFloat(0.0_f64))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: realMod(r1.clone(), r2.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !((metamodelica::OrderedFloat((i2.clone()) as f64) != metamodelica::OrderedFloat(0.0_f64))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: intMod(i1.clone(), i2.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r1 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: ((r1.clone()).0.floor() as i32) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(Expression::makePureBuiltinCall(literal!("sqrt"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(Expression::makePureBuiltinCall(literal!("sqrt"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Expression::makePureBuiltinCall(literal!("sqrt"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Expression::makePureBuiltinCall(literal!("sqrt"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !((Expression::isZero(metamodelica::AsArg::as_arg(&e2))?)) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makePureBuiltinCall(literal!("sign"), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.570796326794896619231321691639751442_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1 @ Deref @ DAE::Exp::RCONST { real: __rlit_0 }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !(__rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).atan2(r2.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: e1 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makePureBuiltinCall(literal!("abs"), list![e1.clone()], tp.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { return Err("guard") }
                    Ok(simplifyMetaModelicaCalls(&inExp)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn preCref(mut ie: metamodelica::Ref<DAE::Exp>, mut ib: bool) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut oe: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut ob: bool;
    (oe, cont, ob) = (::match_deref::match_deref! { match &(ie) {
        e @ Deref @ DAE::Exp::CREF { ty, .. } => {
            (Expression::makeBuiltinCall(literal!("pre"), list![e.clone()], ty.clone(), false), false, true)
        },
        e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            let mut b = ib;
            (e.clone(), false, b)
        },
        e => {
            let mut b = ib;
            (e.clone(), !(b), b)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

fn previousCref(mut ie: metamodelica::Ref<DAE::Exp>, mut ib: bool) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut oe: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut ob: bool;
    (oe, cont, ob) = (::match_deref::match_deref! { match &(ie) {
        e @ Deref @ DAE::Exp::CREF { ty, .. } => {
            (Expression::makeBuiltinCall(literal!("previous"), list![e.clone()], ty.clone(), false), false, true)
        },
        e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
            let mut b = ib;
            (e.clone(), false, b)
        },
        e => {
            let mut b = ib;
            (e.clone(), !(b), b)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

fn changeCref(mut ie: metamodelica::Ref<DAE::Exp>, mut ib: bool) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut oe: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut ob: bool;
    (oe, cont, ob) = (::match_deref::match_deref! { match &(ie) {
        e @ Deref @ DAE::Exp::CREF { ty, .. } => {
            (Expression::makeBuiltinCall(literal!("change"), list![e.clone()], ty.clone(), false), false, true)
        },
        e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
            let mut b = ib;
            (e.clone(), false, b)
        },
        e => {
            let mut b = ib;
            (e.clone(), !(b), b)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

fn edgeCref(mut ie: metamodelica::Ref<DAE::Exp>, mut ib: bool) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut oe: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut ob: bool;
    (oe, cont, ob) = (::match_deref::match_deref! { match &(ie) {
        e @ Deref @ DAE::Exp::CREF { ty, .. } => {
            (Expression::makeBuiltinCall(literal!("edge"), list![e.clone()], ty.clone(), false), false, true)
        },
        e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
            let mut b = ib;
            (e.clone(), false, b)
        },
        e => {
            let mut b = ib;
            (e.clone(), !(b), b)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

pub fn simplify1(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut hasChanged: bool;
    (outExp, hasChanged) = simplify1WithOptions(inExp, optionSimplifyOnly.clone())?;
    Ok((outExp, hasChanged))
}

pub fn simplify1o(mut inExp: Option<metamodelica::Ref<DAE::Exp>>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(e) => {
            let mut e = (*e).clone();
            (e, _) = simplify1WithOptions(e.clone(), optionSimplifyOnly.clone())?;
            Some(e.clone())
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn simplify1WithOptions(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut options: ExpressionSimplifyTypes::Evaluate,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut hasChanged: bool;
    (outExp, hasChanged) = simplify1FixP(inExp.clone(), options, 100, true, false)?;
    checkSimplify(Flags::isSet(Flags::CHECK_SIMPLIFY.clone())?, inExp, outExp.clone())?;
    Ok((outExp, hasChanged))
}

fn checkSimplify(
    mut check: bool,
    mut before: metamodelica::Ref<DAE::Exp>,
    mut after: metamodelica::Ref<DAE::Exp>,
) -> Result<()> {
    let () = (match check {
        false => (),
        true => {
            let mut c1: i32;
            let mut c2: i32;
            let mut b: bool;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty2: metamodelica::Ref<DAE::Type>;
            ty1 = Expression::r#typeof(before.clone())?;
            ty2 = Expression::r#typeof(after.clone())?;
            b = ty1.clone() == ty2.clone();
            if !(b) {
                s1 = ExpressionBasics::printExpStr(before.clone())?;
                s2 = ExpressionBasics::printExpStr(after.clone())?;
                s3 = TypesDump::unparseType(ty1)?;
                s4 = TypesDump::unparseType(ty2)?;
                Error::addMessage(Error::SIMPLIFICATION_TYPE.clone(), list![s1, s2, s3, s4])?;
                return Err("fail");
            }
            c1 = Expression::complexity(&before)?;
            c2 = Expression::complexity(&after)?;
            b = c1 < c2;
            if b {
                s1 = intString(c2);
                s2 = intString(c1);
                s3 = ExpressionBasics::printExpStr(before)?;
                s4 = ExpressionBasics::printExpStr(after)?;
                Error::addMessage(Error::SIMPLIFICATION_COMPLEXITY.clone(), list![s1, s2, s3, s4])?;
                return Err("fail");
            }
            ()
        }
    });
    Ok(())
}

fn simplify1FixP(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inOptions: ExpressionSimplifyTypes::Evaluate,
    mut n: i32,
    mut cont: bool,
    mut hasChanged: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp, inOptions, n, cont)) {
            (exp, _, _, false) => {
                return Ok((exp.clone(), hasChanged))
            },
            (exp, options, 0, _) => {
                let mut str1: ArcStr;
                let mut str2: ArcStr;
                let mut exp = (*exp).clone();
                str1 = ExpressionBasics::printExpStr(exp.clone())?;
                (exp, _) = Expression::traverseExpBottomUp(exp.clone(), &simplifyWork, options.clone())?;
                str2 = ExpressionBasics::printExpStr(exp.clone())?;
                Error::addMessage(Error::SIMPLIFY_FIXPOINT_MAXIMUM.clone(), list![str1, str2])?;
                return Ok((exp.clone(), hasChanged))
            },
            (exp, options, _, true) => {
                let mut expAfterSimplify: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                let mut options = (*options).clone();
                ErrorExt::setCheckpoint(literal!("ExpressionSimplify"));
                (expAfterSimplify, options) = Expression::traverseExpBottomUp(exp.clone(), &simplifyWork, options.clone())?;
                b = !(referenceEq(&*(&*expAfterSimplify),&*(exp.clone())));
                if b {
                    ErrorExt::rollBack(literal!("ExpressionSimplify"));
                } else {
                    ErrorExt::delCheckpoint(literal!("ExpressionSimplify"));
                }
                { (inExp, inOptions, n, cont, hasChanged) = (expAfterSimplify, options.clone(), n - 1, b, b || hasChanged); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifyReductionIterators(
    mut inIters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut inChange: bool,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>, bool)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inIters, inAcc, inChange)) {
            (Deref @ metamodelica::ListNode::Nil, acc, change) => {
                return Ok((acc.clone().reverse(), change.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id, exp, guardExp: Some(Deref @ DAE::Exp::BCONST { bool: true }), ty }, tail: iters }, acc, _) => {
                let mut change: bool;
                let mut iters = (*iters).clone();
                { (inIters, inAcc, inChange) = (iters.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::ReductionIterator { id: id.clone(), exp: exp.clone(), guardExp: None, ty: ty.clone() }), acc.clone()), true); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id, exp: _, guardExp: Some(Deref @ DAE::Exp::BCONST { bool: false }), ty }, tail: _ }, _, _) => {
                return Ok((list![metamodelica::Ref::new(DAE::ReductionIterator { id: id.clone(), exp: metamodelica::Ref::new(DAE::Exp::LIST { valList: metamodelica::nil() }), guardExp: None, ty: ty.clone() })], true))
            },
            (Deref @ metamodelica::ListNode::Cons { head: iter, tail: iters }, acc, change) => {
                let mut iters = (*iters).clone();
                let mut change = (*change).clone();
                { (inIters, inAcc, inChange) = (iters.clone(), metamodelica::cons(iter.clone(), acc.clone()), change.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifyIfExp(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut cond: metamodelica::Ref<DAE::Exp>,
    mut tb: metamodelica::Ref<DAE::Exp>,
    mut fb: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &((cond, tb.clone(), fb.clone())) {
        (Deref @ DAE::Exp::BCONST { bool: true }, _, _) => {
            tb
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, _, _) => {
            fb
        },
        (__esc_exp, Deref @ DAE::Exp::BCONST { bool: true }, Deref @ DAE::Exp::BCONST { bool: false }) => {
            exp = (*__esc_exp).clone();
            exp.clone()
        },
        (__esc_exp, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            exp = (*__esc_exp).clone();
            exp = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: DAE::T_BOOL_DEFAULT().clone() }, exp: exp.clone() });
            exp.clone()
        },
        (e, Deref @ DAE::Exp::BOX { exp: e1 }, Deref @ DAE::Exp::BOX { exp: e2 }) => {
            let mut e = (*e).clone();
            e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e1.clone(), expElse: e2.clone() });
            metamodelica::Ref::new(DAE::Exp::BOX { exp: e.clone() })
        },
        _ => {
            if (ExpressionBasics::expEqual(&tb, fb)?) {tb} else {origExp}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn simplifyMetaModelicaCalls(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listAppend" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::LIST { valList: el }, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = List::fold(&(el.clone().reverse()), &fnptr!(Expression::makeCons, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), e2.clone())?;
            e
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listAppend" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::ListNode::Nil }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            e1.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "intString" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut s: ArcStr;
            s = intString(i.clone());
            metamodelica::Ref::new(DAE::Exp::SCONST { string: s })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "realString" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut s: ArcStr;
            s = realString(r.clone());
            metamodelica::Ref::new(DAE::Exp::SCONST { string: s })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "boolString" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: b }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut s: ArcStr;
            s = boolString(b.clone());
            metamodelica::Ref::new(DAE::Exp::SCONST { string: s })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::LIST { valList: el }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut el = (*el).clone();
            el = el.clone().reverse();
            e1_1 = metamodelica::Ref::new(DAE::Exp::LIST { valList: el.clone() });
            e1_1
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, iterType: rit, exprType: ty, defaultValue: v, foldName, resultName, foldExp }, expr: e1, iterators: riters }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut e1 = (*e1).clone();
            e1 = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("listReverse") }), iterType: rit.clone(), exprType: ty.clone(), defaultValue: v.clone(), foldName: foldName.clone(), resultName: resultName.clone(), foldExp: foldExp.clone() }), expr: e1.clone(), iterators: riters.clone() });
            e1.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, iterType: rit, exprType: ty, defaultValue: v, foldName, resultName, foldExp }, expr: e1, iterators: riters }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut e1 = (*e1).clone();
            e1 = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("list") }), iterType: rit.clone(), exprType: ty.clone(), defaultValue: v.clone(), foldName: foldName.clone(), resultName: resultName.clone(), foldExp: foldExp.clone() }), expr: e1.clone(), iterators: riters.clone() });
            e1.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listLength" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::LIST { valList: el }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut i: i32;
            i = ((el).len() as i32);
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mmc_mk_some" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: Some(e.clone()) })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sourceInfo" }, .. } => {
            metamodelica::print(literal!("sourceInfo() - simplify?\n"));
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyCons(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CONS { car: e, cdr: Deref @ DAE::Exp::LIST { valList: es } } => {
            metamodelica::Ref::new(DAE::Exp::LIST { valList: metamodelica::cons(e.clone(), es.clone()) })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn simplifyUnbox(mut exp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::UNBOX { exp: Deref @ DAE::Exp::BOX { exp: __esc_outExp }, .. } => {
            outExp = (*__esc_outExp).clone();
            outExp.clone()
        },
        Deref @ DAE::Exp::BOX { exp: Deref @ DAE::Exp::UNBOX { exp: __esc_outExp, .. } } => {
            outExp = (*__esc_outExp).clone();
            outExp.clone()
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn simplifyMatch(mut exp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::MATCHEXPRESSION { inputs: Deref @ metamodelica::ListNode::Nil, et: ty, localDecls: Deref @ metamodelica::ListNode::Nil, cases: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: Deref @ metamodelica::ListNode::Nil, localDecls: Deref @ metamodelica::ListNode::Nil, body: Deref @ metamodelica::ListNode::Nil, result: Some(e), .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } if (!(Types::isTuple(metamodelica::AsArg::as_arg(&ty)))) => {
            e.clone()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { inputs: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, et: ty, localDecls: Deref @ metamodelica::ListNode::Nil, cases: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::BCONST { bool: b1 }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, localDecls: Deref @ metamodelica::ListNode::Nil, body: Deref @ metamodelica::ListNode::Nil, result: Some(e1), .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::BCONST { bool: b2 }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, localDecls: Deref @ metamodelica::ListNode::Nil, body: Deref @ metamodelica::ListNode::Nil, result: Some(e2), .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } if (!(boolEq(b1.clone(), b2.clone())) && !(Types::isTuple(metamodelica::AsArg::as_arg(&ty)))) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e = (*e).clone();
            e1_1 = if (b1.clone()) {e1.clone()} else {e2.clone()};
            e2_1 = if (b1.clone()) {e2.clone()} else {e1.clone()};
            e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e1_1, expElse: e2_1 });
            e.clone()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: DAE::MatchType::MATCH { .. }, et: ty, inputs: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, localDecls: Deref @ metamodelica::ListNode::Nil, cases: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::BCONST { bool: b1 }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, localDecls: Deref @ metamodelica::ListNode::Nil, body: Deref @ metamodelica::ListNode::Nil, result: Some(e1), .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_WILD { .. }, tail: Deref @ metamodelica::ListNode::Nil }, localDecls: Deref @ metamodelica::ListNode::Nil, body: Deref @ metamodelica::ListNode::Nil, result: Some(e2), .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } if (!(Types::isTuple(metamodelica::AsArg::as_arg(&ty)))) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e = (*e).clone();
            e1_1 = if (b1.clone()) {e1.clone()} else {e2.clone()};
            e2_1 = if (b1.clone()) {e2.clone()} else {e1.clone()};
            e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e1_1, expElse: e2_1 });
            e.clone()
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn simplifyCast(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut tp: Type,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((exp, tp.clone())) {
        (Deref @ DAE::Exp::RCONST { real: r }, Deref @ DAE::Type::T_REAL { .. }) => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() })
        },
        (Deref @ DAE::Exp::ICONST { integer: i }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut r: metamodelica::Real;
            r = intReal(i.clone());
            metamodelica::Ref::new(DAE::Exp::RCONST { real: r })
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = addCast(e.clone(), tp.clone());
            metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp }, exp: e.clone() })
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = addCast(e.clone(), tp.clone());
            metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: e.clone() })
        },
        (Deref @ DAE::Exp::ARRAY { ty: _, scalar: b, array: exps }, _) => {
            let mut exps_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut tp_1: Type;
            tp_1 = Expression::unliftArray(&tp)?;
            exps_1 = List::map1(exps.clone(), &fnptr!(addCast, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>), tp_1)?;
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp, scalar: b.clone(), array: exps_1 })
        },
        (Deref @ DAE::Exp::RANGE { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { .. }, .. }, start: e1, step: eo, stop: e2 }, Deref @ DAE::Type::T_ARRAY { ty: tp2 @ Deref @ DAE::Type::T_REAL { .. }, .. }) => {
            let mut e1 = (*e1).clone();
            let mut eo = (*eo).clone();
            let mut e2 = (*e2).clone();
            e1 = addCast(e1.clone(), tp2.clone());
            e2 = addCast(e2.clone(), tp2.clone());
            eo = Util::applyOption1(eo.clone(), &fnptr!(addCast, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>), tp2.clone())?;
            metamodelica::Ref::new(DAE::Exp::RANGE { ty: tp, start: e1.clone(), step: eo.clone(), stop: e2.clone() })
        },
        (Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: e1, expElse: e2 }, _) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            e1_1 = metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1.clone() });
            e2_1 = metamodelica::Ref::new(DAE::Exp::CAST { ty: tp, exp: e2.clone() });
            metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: e1_1, expElse: e2_1 })
        },
        (Deref @ DAE::Exp::MATRIX { ty: _, integer: n, matrix: mexps }, _) => {
            let mut tp1: Type;
            let mut tp2: Type;
            let mut mexps_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            tp1 = Expression::unliftArray(&tp)?;
            tp2 = Expression::unliftArray(&tp1)?;
            mexps_1 = List::map1List(mexps.clone(), &fnptr!(addCast, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>), tp2)?;
            metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp, integer: n.clone(), matrix: mexps_1 })
        },
        (Deref @ DAE::Exp::CALL { path: p1, expLst: exps, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p3 }, .. }) if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) => {
            metamodelica::Ref::new(DAE::Exp::CALL { path: p3.clone(), expLst: exps.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: tp, tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) })
        },
        (Deref @ DAE::Exp::RECORD { path: _, exps, comp: fieldNames, ty: _ }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p3 }, .. }) => {
            metamodelica::Ref::new(DAE::Exp::RECORD { path: p3.clone(), exps: exps.clone(), comp: fieldNames.clone(), ty: tp })
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: exps }, .. }, _) => {
            let mut tp_1: Type;
            let mut e = (*e).clone();
            tp_1 = List::fold(metamodelica::AsArg::as_arg(&exps), &move |__a0: _, __a1: metamodelica::Ref<DAE::Type>| Expression::unliftArrayIgnoreFirst(__a0, &__a1), tp.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::CAST { ty: tp_1, exp: e.clone() });
            e = Expression::makePureBuiltinCall(literal!("fill"), metamodelica::cons(e.clone(), exps.clone()), tp);
            e.clone()
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::ICONST { integer: n }, tail: exps }, .. }, Deref @ DAE::Type::T_ARRAY { dims, .. }) if (Expression::dimensionUnknown(&((dims).get(n.clone())?))) => {
            let mut exps = (*exps).clone();
            exps = List::map1(exps.clone(), &fnptr!(addCast, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>), tp.clone())?;
            Expression::makePureBuiltinCall(literal!("cat"), metamodelica::cons(e.clone(), exps.clone()), tp)
        },
        (e, _) => {
            let mut t1: Type;
            let mut t2: Type;
            t1 = Expression::arrayEltType(&tp);
            t2 = Expression::arrayEltType(&(Expression::r#typeof(e.clone())?));
            if (t1 == t2) {e.clone()} else {origExp}
        },
        _ => {
            origExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn addCast(mut inExp: metamodelica::Ref<DAE::Exp>, mut inType: Type) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::CAST { ty: inType, exp: inExp });
    outExp
}

fn reductionDefaultValue(
    mut name: &ArcStr,
    mut ty: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut defaultValue: metamodelica::Ref<Values::Value>;
    defaultValue = (::match_deref::match_deref! { match &((name.clone(), &**ty)) {
        (Deref @ "min", Deref @ DAE::Type::T_BOOL { .. }) => metamodelica::Ref::new(Values::Value::BOOL { boolean: true }),
        (Deref @ "min", Deref @ DAE::Type::T_INTEGER { .. }) => metamodelica::Ref::new(Values::Value::INTEGER { integer: System::intMaxLit() }),
        (Deref @ "min", Deref @ DAE::Type::T_REAL { .. }) => metamodelica::Ref::new(Values::Value::REAL { real: System::realMaxLit() }),
        (Deref @ "min", Deref @ DAE::Type::T_ENUMERATION { .. }) => metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((**ty).path, DAE::Type::T_ENUMERATION), &(List::last(var_field!((**ty).names, DAE::Type::T_ENUMERATION))?)), index: ((var_field!((**ty).names, DAE::Type::T_ENUMERATION)).len() as i32) }),
        (Deref @ "max", Deref @ DAE::Type::T_BOOL { .. }) => metamodelica::Ref::new(Values::Value::BOOL { boolean: false }),
        (Deref @ "max", Deref @ DAE::Type::T_INTEGER { .. }) => metamodelica::Ref::new(Values::Value::INTEGER { integer: intNeg(System::intMaxLit()) }),
        (Deref @ "max", Deref @ DAE::Type::T_REAL { .. }) => metamodelica::Ref::new(Values::Value::REAL { real: -(System::realMaxLit()) }),
        (Deref @ "max", Deref @ DAE::Type::T_ENUMERATION { .. }) => metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((**ty).path, DAE::Type::T_ENUMERATION), &(List::last(var_field!((**ty).names, DAE::Type::T_ENUMERATION))?)), index: 1 }),
        (Deref @ "product", Deref @ DAE::Type::T_INTEGER { .. }) => metamodelica::Ref::new(Values::Value::INTEGER { integer: 1 }),
        (Deref @ "product", Deref @ DAE::Type::T_REAL { .. }) => metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(1.0_f64) }),
        (Deref @ "sum", Deref @ DAE::Type::T_INTEGER { .. }) => metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }),
        (Deref @ "sum", Deref @ DAE::Type::T_REAL { .. }) => metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }),
        _ => return Err("match: no arm matched"),
    } });
    Ok(defaultValue)
}

fn reductionExpression(
    mut name: &ArcStr,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut foldName: ArcStr,
    mut resultName: ArcStr,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut foldExp: metamodelica::Ref<DAE::Exp>;
    let mut foldNameExp: metamodelica::Ref<DAE::Exp>;
    let mut resultNameExp: metamodelica::Ref<DAE::Exp>;
    foldNameExp = Expression::makeCrefExp(
        ComponentReferenceBasics::makeCrefIdent(foldName, ty.clone(), metamodelica::nil()),
        ty.clone(),
    )?;
    resultNameExp = Expression::makeCrefExp(
        ComponentReferenceBasics::makeCrefIdent(resultName, ty.clone(), metamodelica::nil()),
        ty.clone(),
    )?;
    foldExp = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "min" => Expression::makeBuiltinCall(literal!("min"), list![foldNameExp, resultNameExp], ty, false),
        Deref @ "max" => Expression::makeBuiltinCall(literal!("max"), list![foldNameExp, resultNameExp], ty, false),
        Deref @ "product" => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: foldNameExp, operator: DAE::Operator::MUL { ty: ty }, exp2: resultNameExp }),
        Deref @ "sum" => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: foldNameExp, operator: DAE::Operator::ADD { ty: ty }, exp2: resultNameExp }),
        _ => return Err("match: no arm matched"),
    } });
    Ok(foldExp)
}

pub fn elabBuiltinFill2(
    mut inCache: FCore::Cache,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inValuesValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut constVar: DAE::Const,
    mut inDims: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inExp, inType, &**inValuesValueLst, constVar);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, s, sty, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v }, tail: Deref @ metamodelica::ListNode::Nil }, c1) => {
                    let mut arraylist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut is_scalar: bool;
                    let mut sty2: metamodelica::Ref<DAE::Type>;
                    let mut v = (*v).clone();
                    let true = (intLt(v.clone(), 0)) else { return Err("pattern mismatch") };
                    v = 0;
                    arraylist = List::fill(s.clone(), v.clone());
                    sty2 = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: sty.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: v.clone() })] });
                    at = Types::simplifyType(sty2.clone())?;
                    is_scalar = !(Types::isArray(metamodelica::AsArg::as_arg(&sty)));
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: is_scalar, array: arraylist.clone() }), DAE::Properties::PROP { type_: sty2.clone(), constFlag: c1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, s, sty, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v }, tail: Deref @ metamodelica::ListNode::Nil }, c1) => {
                    let mut arraylist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut is_scalar: bool;
                    let mut sty2: metamodelica::Ref<DAE::Type>;
                    arraylist = List::fill(s.clone(), v.clone());
                    sty2 = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: sty.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: v.clone() })] });
                    at = Types::simplifyType(sty2.clone())?;
                    is_scalar = !(Types::isArray(metamodelica::AsArg::as_arg(&sty)));
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: is_scalar, array: arraylist.clone() }), DAE::Properties::PROP { type_: sty2.clone(), constFlag: c1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, s, sty, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v }, tail: rest }, c1) => {
                    let mut arraylist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut sty2: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabBuiltinFill2(cache.clone(), s.clone(), sty.clone(), metamodelica::AsArg::as_arg(&rest), c1.clone(), inDims, inInfo)?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: _ }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    exp = metamodelica::Own::own(__pa1);
                    ty = metamodelica::Own::own(__pa2);
                    arraylist = List::fill(exp.clone(), v.clone());
                    sty2 = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: v.clone() })] });
                    at = Types::simplifyType(sty2.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: false, array: arraylist.clone() }), DAE::Properties::PROP { type_: sty2.clone(), constFlag: c1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ExpressionSimplify.elabBuiltinFill2 failed for expression: fill(")); __mm_s.push_str(&*Dump::printExpLstStr(inDims.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn simplifyBuiltinCalls(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    outExp = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::ARRAY { .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    if !((metamodelica::stringEq(&name, &(literal!("max"))) || metamodelica::stringEq(&name, &(literal!("min"))))) { return Err("guard") }
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    expl = Expression::flattenArrayExpToList(e.clone());
                    e1 = Expression::makeScalarArray(expl.clone(), tp.clone());
                    let false = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e), e1.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall(name.clone(), list![e1.clone()], tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl @ Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    if !((metamodelica::stringEq(&name, &(literal!("max"))) || metamodelica::stringEq(&name, &(literal!("min"))))) { return Err("guard") }
                    let mut e = (*e).clone();
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    if Expression::isArrayType(&(Expression::r#typeof(e.clone())?)) {
                        assign_variant_field!(exp => DAE::Exp::CALL; expLst = expl.clone());
                        e = exp.clone();
                    }
                    Ok((e.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut es = (*es).clone();
                    i1 = ((es).len() as i32);
                    es = List::union(metamodelica::AsArg::as_arg(&es), &(metamodelica::nil()));
                    i2 = ((es).len() as i32);
                    if i1 == i2 {
                        let __pa0 = ::match_deref::match_deref! { match &(List::fold(metamodelica::AsArg::as_arg(&es), &fnptr!(maxElement, metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Exp>>), None)?) {
                            Some(__pa0) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        e = metamodelica::Own::own(__pa0);
                        es = List::select(es.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(removeMinMaxFoldableValues(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>))?;
                        es = metamodelica::cons(e.clone(), es.clone());
                        i2 = ((es).len() as i32);
                        let true = (i2 < i1) else { return Err("pattern mismatch") };
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    } else {
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    }
                    Ok(Expression::makePureBuiltinCall(literal!("max"), list![e.clone()], tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut es = (*es).clone();
                    i1 = ((es).len() as i32);
                    es = List::union(metamodelica::AsArg::as_arg(&es), &(metamodelica::nil()));
                    i2 = ((es).len() as i32);
                    if i1 == i2 {
                        let __pa0 = ::match_deref::match_deref! { match &(List::fold(metamodelica::AsArg::as_arg(&es), &fnptr!(minElement, metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Exp>>), None)?) {
                            Some(__pa0) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        e = metamodelica::Own::own(__pa0);
                        es = List::select(es.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(removeMinMaxFoldableValues(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>))?;
                        es = metamodelica::cons(e.clone(), es.clone());
                        i2 = ((es).len() as i32);
                        let true = (i2 < i1) else { return Err("pattern mismatch") };
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    } else {
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    }
                    Ok(Expression::makePureBuiltinCall(literal!("min"), list![e.clone()], tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makePureBuiltinCall(literal!("min"), list![e1.clone(), e2.clone()], tp.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makePureBuiltinCall(literal!("max"), list![e1.clone(), e2.clone()], tp.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: e2.clone() });
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: e2.clone() });
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makeLBinary(metamodelica::AsArg::as_arg(&expl), &(DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }))?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makeLBinary(metamodelica::AsArg::as_arg(&expl), &(DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT().clone() }))?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut expl = (*expl).clone();
                    let true = (Config::scalarizeMinMax()?) else { return Err("pattern mismatch") };
                    let true = (stringEq(&name, &(literal!("max"))) || stringEq(&name, &(literal!("min")))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(expl.clone().reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    e2 = metamodelica::Own::own(__pa1);
                    expl = metamodelica::Own::own(__pa2);
                    e1 = Expression::makePureBuiltinCall(name.clone(), list![e2.clone(), e1.clone()], tp.clone());
                    e1 = List::fold2(metamodelica::AsArg::as_arg(&expl), &fnptr!(makeNestedReduction, metamodelica::Ref<DAE::Exp>, ArcStr, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Exp>), name.clone(), tp.clone(), e1.clone())?;
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cross" }, expLst: expl, .. } => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut v1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut v2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut scalar: bool;
                    let mut expl = (*expl).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expl.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    v1 = metamodelica::Own::own(__pa0);
                    v2 = metamodelica::Own::own(__pa1);
                    expl = simplifyCross(&v1, &v2)?;
                    tp = Expression::r#typeof(e.clone())?;
                    scalar = !(Expression::isArrayType(&(Expression::unliftArray(&tp)?)));
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: scalar, array: expl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "skew" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: v1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut mexpl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    mexpl = simplifySkew(metamodelica::AsArg::as_arg(&v1))?;
                    tp = Expression::r#typeof(e.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: 3, matrix: mexpl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: expl }, .. } => {
                    let mut valueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    valueLst = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| ValuesUtil::expValue(&__a0))?;
                    (_, outExp, _) = elabBuiltinFill2(FCore::noCache(), e.clone(), Expression::r#typeof(e.clone())?, &valueLst, openmodelica_frontend_types::DAE::Const::C_CONST, &(metamodelica::nil()), &(Absyn::dummyInfo.clone()))?;
                    Ok((outExp.clone(), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: len_exp, tail: Deref @ metamodelica::ListNode::Cons { head: just_exp, tail: Deref @ metamodelica::ListNode::Nil } } }, .. } => {
                    Ok(simplifyBuiltinStringFormat(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&len_exp), metamodelica::AsArg::as_arg(&just_exp))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "stringAppendList" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::LIST { valList: expl }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(simplifyStringAppendList(expl.clone(), metamodelica::nil(), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }) });
                    Ok(Expression::makePureBuiltinCall(literal!("abs"), list![e.clone()], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.25_f64) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::RCONST { real: r1 }, operator: DAE::Operator::MUL { ty: tp }, exp2: e2 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let true = (r1.clone() >= metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    e = Expression::makePureBuiltinCall(literal!("sqrt"), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    e3 = Expression::makePureBuiltinCall(literal!("sqrt"), list![e2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    expl = Expression::expandFactors(metamodelica::AsArg::as_arg(&e1))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::split1OnTrue(&expl, &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isFunCall(&__a0, &__a1)) }, literal!("log"))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2 = metamodelica::Own::own(__pa0);
                    es = metamodelica::Own::own(__pa1);
                    let __pa3 = ::match_deref::match_deref! { match &(e2.clone()) {
                        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa3.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa3);
                    e3 = Expression::makeProductLst(es.clone())?;
                    Ok(Expression::expPow(e.clone(), Expression::negate(e3.clone())?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    expl = Expression::expandFactors(metamodelica::AsArg::as_arg(&e1))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::split1OnTrue(&expl, &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isFunCall(&__a0, &__a1)) }, literal!("log"))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2 = metamodelica::Own::own(__pa0);
                    es = metamodelica::Own::own(__pa1);
                    let __pa3 = ::match_deref::match_deref! { match &(e2.clone()) {
                        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa3.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa3);
                    e3 = Expression::makeProductLst(es.clone())?;
                    Ok(Expression::expPow(e.clone(), e3.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 } => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    e3 = Expression::expMul(e.clone(), e2.clone())?;
                    Ok(Expression::makePureBuiltinCall(literal!("exp"), list![e3.clone()], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: r1 } }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let __rlit0 = (realMod(r1.clone(), metamodelica::OrderedFloat(2.0_f64)));
                    if !(__rlit0.eq(&metamodelica::OrderedFloat((1.0) as f64))) { return Err("pattern mismatch") }
                    e3 = Expression::makePureBuiltinCall(literal!("log"), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() }), e3.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, operator: DAE::Operator::DIV { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    if !(__rlit_1.eq(&metamodelica::OrderedFloat((1.0) as f64))) { return Err("guard") }
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    e3 = Expression::makePureBuiltinCall(literal!("log"), list![e2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: e3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    e3 = Expression::makePureBuiltinCall(literal!("log"), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !((Expression::isConst(e1.clone())?)) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_DF$DER" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    if !((Expression::isConst(e1.clone())?)) { return Err("guard") }
                    Ok(Expression::makeConstZeroE(e1.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    let mut e = (*e).clone();
                    assign_variant_field!(e => DAE::Exp::CALL; expLst = list![e1.clone(), e2.clone(), e2.clone()]);
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. } => {
                    if !((Expression::isConst(e1.clone())?)) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Cons { head: e4, tail: Deref @ metamodelica::ListNode::Nil } } }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isConst(e1.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makeImpureBuiltinCall(literal!("delay"), list![e2.clone(), e3.clone(), e4.clone()], tp.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Cons { head: e4, tail: Deref @ metamodelica::ListNode::Nil } } }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isConst(e2.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makeImpureBuiltinCall(literal!("delay"), list![e1.clone(), e3.clone(), e4.clone()], tp.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: op, exp: e }, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Cons { head: e4, tail: Deref @ metamodelica::ListNode::Nil } } }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e = (*e).clone();
                    e = Expression::makeImpureBuiltinCall(literal!("delay"), list![e.clone(), e3.clone(), e4.clone()], tp.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp1, .. } } => {
                    Ok(Expression::makeConstZero(metamodelica::AsArg::as_arg(&tp1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::MATRIX { ty: tp1, matrix: mexpl, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp2, .. } } => {
                    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut sc: bool;
                    let mut dim: i32;
                    let mut tp1 = (*tp1).clone();
                    es = List::flatten(mexpl.clone())?;
                    tp1 = Expression::unliftArray(&(Expression::unliftArray(metamodelica::AsArg::as_arg(&tp1))?))?;
                    sc = !(Expression::isArrayType(metamodelica::AsArg::as_arg(&tp1)));
                    tp1 = if (sc) {Expression::unliftArray(metamodelica::AsArg::as_arg(&tp1))?} else {tp1.clone()};
                    tp1 = if (sc) {Expression::liftArrayLeft(tp1.clone(), openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())} else {tp1.clone()};
                    dim = ((es).len() as i32);
                    tp1 = Expression::liftArrayLeft(tp1.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim }));
                    e = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp1.clone(), scalar: sc, array: es.clone() });
                    e = Expression::makePureBuiltinCall(literal!("sum"), list![e.clone()], tp2.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, ty: tp1, scalar: false }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp2, .. } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut sc: bool;
                    let mut dim: i32;
                    let mut es = (*es).clone();
                    let mut tp1 = (*tp1).clone();
                    es = simplifyCat(1, es.clone())?;
                    tp1 = Expression::unliftArray(metamodelica::AsArg::as_arg(&tp1))?;
                    sc = !(Expression::isArrayType(metamodelica::AsArg::as_arg(&tp1)));
                    tp1 = if (sc) {Expression::unliftArray(metamodelica::AsArg::as_arg(&tp1))?} else {tp1.clone()};
                    tp1 = if (sc) {Expression::liftArrayLeft(tp1.clone(), openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())} else {tp1.clone()};
                    dim = ((es).len() as i32);
                    tp1 = Expression::liftArrayLeft(tp1.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim }));
                    e = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp1.clone(), scalar: sc, array: es.clone() });
                    e = Expression::makePureBuiltinCall(literal!("sum"), list![e.clone()], tp2.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, scalar: false, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp2, .. } } => {
                    let mut e = (*e).clone();
                    e = Expression::makePureBuiltinCall(literal!("sum"), list![e.clone()], tp2.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, scalar: true, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makeSum(es.clone())?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: es }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut es = (*es).clone();
                    es = simplifyCat(i.clone(), es.clone())?;
                    e = Expression::makePureBuiltinCall(literal!("cat"), metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }), es.clone()), tp.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: es }, attr: Deref @ DAE::CallAttributes { .. } } => {
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut dims: metamodelica::List<i32>;
                            let mut es = (*es).clone();
                            (es, dims) = ExpressionBasics::evalCat(i.clone(), es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::getArrayOrMatrixContents(&__a0), &ExpressionBasics::printExpStr)?;
                            e = Expression::listToArray(metamodelica::AsArg::as_arg(&es), &(({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
                for mut d in (dims.clone()).into_iter().cloned() {
                            let __x = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: d.clone() });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })))?;
                            Ok(e.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !((Types::numberOfDimensions(&(Expression::r#typeof(e1.clone())?)) == i.clone())) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { ty: tp1 @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, scalar: sc, array: es }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: 2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut i: i32;
                    let mut es = (*es).clone();
                    tp = Types::liftArray(Types::unliftArray(metamodelica::AsArg::as_arg(&tp1))?, metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
                    es = List::map2(List::map(es.clone(), &fnptr!(List::create, _))?, &fnptr!(Expression::makeArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::Type>, bool), tp.clone(), sc.clone())?;
                    i = ((es).len() as i32);
                    tp = Expression::liftArrayLeft(tp.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i }));
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: false, array: es.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    if !((!(Types::isArray(&(Expression::r#typeof(e1.clone())?))))) { return Err("guard") }
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut tp1: metamodelica::Ref<DAE::Type>;
                    let mut e1 = (*e1).clone();
                    tp = Expression::r#typeof(e1.clone())?;
                    for mut j in 1..=i.clone() {
                        tp1 = Types::liftArray(tp.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
                        e1 = Expression::makeArray(list![e1.clone()], tp1.clone(), !(Types::isArray(&tp)));
                        tp = tp1.clone();
                    }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transpose" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { .. } } => {
                    let mut e = (*e).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::transposeArray(e.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "symmetric" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                            let mut mexpl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                            let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut tp1: metamodelica::Ref<DAE::Type>;
                            let mut marr: metamodelica::Array<metamodelica::Array<metamodelica::Ref<DAE::Exp>>>;
                            let mut e = (*e).clone();
                            mexpl = Expression::get2dArrayOrMatrixContent(metamodelica::AsArg::as_arg(&e))?;
                            e = (::match_deref::match_deref! { match &(mexpl.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Nil, tail: Deref @ metamodelica::ListNode::Nil } => e.clone(),
                Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, tail: Deref @ metamodelica::ListNode::Nil } => e.clone(),
                _ => {
                            marr = metamodelica::arrayFromVec(List::map(mexpl.clone(), &fnptr!(listArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>))?.into_iter().cloned().collect());
                            let true = (metamodelica::arrayLength(marr.clone()) == metamodelica::arrayLength(metamodelica::arrayGet(marr.clone(), 1)?)) else { return Err("pattern mismatch") };
                            let true = (metamodelica::arrayLength(marr.clone()) > 1) else { return Err("pattern mismatch") };
                            simplifySymmetric(marr.clone(), metamodelica::arrayLength(marr.clone()) - 1, metamodelica::arrayLength(marr.clone()))?;
                            mexpl = List::mapArray(marr.clone(), &fnptr!(arrayList, metamodelica::Array<metamodelica::Ref<DAE::Exp>>))?;
                            tp1 = Expression::unliftArray(metamodelica::AsArg::as_arg(&tp))?;
                            es = List::map2(mexpl.clone(), &fnptr!(Expression::makeArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::Type>, bool), tp1.clone(), !(Types::isArray(&tp1)))?;
                            e = Expression::makeArray(es.clone(), tp.clone(), false);
                            e.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            Ok(e.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "scalar" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e = (*e).clone();
                    e = simplifyScalar(e.clone(), tp.clone())?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" }, expLst: es @ Deref @ metamodelica::ListNode::Cons { head: e, tail: _ }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_ARRAY { ty: tp, dims: _ }, .. } } => {
                    let mut i: i32;
                    let mut tp = (*tp).clone();
                    let false = (Types::isArray(&(Expression::r#typeof(e.clone())?))) else { return Err("pattern mismatch") };
                    i = ((es).len() as i32);
                    tp = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i })] });
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: true, array: es.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::ARRAY { scalar: true, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { .. } } => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    es = List::flatten(mexpl.clone())?;
                    es = List::map1(es.clone(), &fnptr!(Expression::makeVectorCall, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>), tp.clone())?;
                    e = Expression::makePureBuiltinCall(literal!("cat"), metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), es.clone()), tp.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. } } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut es = (*es).clone();
                    es = List::map1(es.clone(), &fnptr!(Expression::makeVectorCall, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>), tp.clone())?;
                    e = Expression::makePureBuiltinCall(literal!("cat"), metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), es.clone()), tp.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "inferredClock" }, expLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "realClock" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK { interval: e1.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "booleanClock" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK { condition: e1.clone(), startInterval: e2.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rationalClock" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e1.clone(), resolution: e2.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "solverClock" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK { c: e1.clone(), solverMethod: e2.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "OpenModelica_uriToFilename" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::SCONST { string: s1 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut s2: ArcStr;
                    s2 = uriToFilename(s1.clone())?;
                    if Flags::getConfigBool(Flags::BUILDING_FMU.clone())? {
                        e = Expression::makeImpureBuiltinCall(literal!("OpenModelica_fmuLoadResource"), list![metamodelica::Ref::new(DAE::Exp::SCONST { string: s2.clone() })], DAE::T_STRING_DEFAULT().clone());
                    } else {
                        e = metamodelica::Ref::new(DAE::Exp::SCONST { string: s2.clone() });
                    }
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn simplifyScalar(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            exp = (*__esc_exp).clone();
            Expression::makePureBuiltinCall(literal!("scalar"), list![exp.clone()], tp)
        },
        Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: Deref @ metamodelica::ListNode::Nil }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            exp = (*__esc_exp).clone();
            Expression::makePureBuiltinCall(literal!("scalar"), list![exp.clone()], tp)
        },
        Deref @ DAE::Exp::SIZE { exp: __esc_exp, sz: None } => {
            exp = (*__esc_exp).clone();
            ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(&(Expression::r#typeof(inExp)?))) {
                (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }) => (),
                _ => return Err("pattern mismatch"),
            } };
            metamodelica::Ref::new(DAE::Exp::SIZE { exp: exp.clone(), sz: Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 })) })
        },
        _ => {
            ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(&(Expression::r#typeof(inExp.clone())?))) {
                (_, Deref @ metamodelica::ListNode::Nil) => (),
                _ => return Err("pattern mismatch"),
            } };
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn makeNestedReduction(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inName: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inCall: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outCall: metamodelica::Ref<DAE::Exp>;
    outCall = Expression::makePureBuiltinCall(inName, list![inExp, inCall], inType);
    outCall
}

fn simplifySymmetric(
    mut marr: metamodelica::Array<metamodelica::Array<metamodelica::Ref<DAE::Exp>>>,
    mut i1: i32,
    mut i2: i32,
) -> Result<()> {
    let () = (match (i1, i2) {
        (0, 1) => (),
        _ => {
            let mut v1: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
            let mut v2: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v1 = metamodelica::arrayGet(marr.clone(), i1)?;
            v2 = metamodelica::arrayGet(marr.clone(), i2)?;
            exp = metamodelica::arrayGet(v1.clone(), i2)?;
            metamodelica::arrayUpdate(v2.clone(), i1, exp)?;
            simplifySymmetric(
                marr.clone(),
                if (i1 == 1) { i2 - 2 } else { i1 - 1 },
                if (i1 == 1) { i2 - 1 } else { i2 },
            )?;
            ()
        }
    });
    Ok(())
}

fn simplifyCat(
    mut inDim: i32,
    mut inExpList: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpList = (match inDim {
        1 => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expl = List::map(inExpList, &simplifyCatArg)?;
            simplifyCat2(inDim, &expl, &(metamodelica::nil()), false)?
        }
        _ => simplifyCat2(inDim, &inExpList, &(metamodelica::nil()), false)?,
    });
    Ok(outExpList)
}

fn simplifyCatArg(mut arg: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outArg: metamodelica::Ref<DAE::Exp>;
    outArg = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::Exp::MATRIX { .. } => {
            Expression::matrixToArray(arg)?
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } if (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim))) => {
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: var_field!((*arg).ty, DAE::Exp::CREF).clone(), scalar: true, array: Expression::expandExpression(&arg, false)? })
        },
        _ => {
            arg
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

fn simplifyCat2(
    mut dim: i32,
    mut ies: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut acc: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut changed: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut oes: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    oes = 'mc: {
        let __mc_input = (dim, &**ies, changed);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, true) => {
                    Ok(acc.clone().reverse())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (1, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: es1, scalar: sc, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: dims }, ty: etp } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: es2, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: _ }, .. }, .. }, tail: es } }, _) => {
                    let mut esn: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut ndim: metamodelica::Ref<DAE::Dimension>;
                    let mut etp = (*etp).clone();
                    esn = listAppend(es1.clone(), es2.clone());
                    ndim = Expression::addDimensions(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2));
                    etp = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: etp.clone(), dims: metamodelica::cons(ndim.clone(), dims.clone()) });
                    e = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: etp.clone(), scalar: sc.clone(), array: esn.clone() });
                    Ok(simplifyCat2(dim, &(metamodelica::cons(e.clone(), es.clone())), acc, true)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (2, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: ms1, integer: i, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim11, tail: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: dims } }, ty: etp } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: ms2, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: _ } }, .. }, .. }, tail: es } }, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut ndim: metamodelica::Ref<DAE::Dimension>;
                    let mut mss: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut etp = (*etp).clone();
                    mss = List::threadMap(ms1.clone(), ms2.clone(), &fnptr!(listAppend, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::List<metamodelica::Ref<DAE::Exp>>))?;
                    ndim = Expression::addDimensions(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2));
                    etp = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: etp.clone(), dims: metamodelica::cons(dim11.clone(), metamodelica::cons(ndim.clone(), dims.clone())) });
                    e = metamodelica::Ref::new(DAE::Exp::MATRIX { ty: etp.clone(), integer: i.clone(), matrix: mss.clone() });
                    Ok(simplifyCat2(dim, &(metamodelica::cons(e.clone(), es.clone())), acc, true)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: e, tail: es }, _) => {
                    Ok(simplifyCat2(dim, metamodelica::AsArg::as_arg(&es), &(metamodelica::cons(e.clone(), acc.clone())), changed)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oes)
}

fn simplifyBuiltinStringFormat(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut len_exp: &metamodelica::Ref<DAE::Exp>,
    mut just_exp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match (exp, len_exp, just_exp) {
        (Deref @ DAE::Exp::ICONST { integer: i }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr;
            r#str = intString(i.clone());
            r#str = cevalBuiltinStringFormat(r#str.clone(), ((r#str).len() as i32), len.clone(), just.clone());
            metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str })
        },
        (Deref @ DAE::Exp::RCONST { real: r }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr;
            r#str = realString(r.clone());
            r#str = cevalBuiltinStringFormat(r#str.clone(), ((r#str).len() as i32), len.clone(), just.clone());
            metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str })
        },
        (Deref @ DAE::Exp::BCONST { bool: b }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr;
            r#str = boolString(b.clone());
            r#str = cevalBuiltinStringFormat(r#str.clone(), ((r#str).len() as i32), len.clone(), just.clone());
            metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str })
        },
        (Deref @ DAE::Exp::ENUM_LITERAL { name, .. }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr;
            r#str = AbsynUtil::pathLastIdent(name);
            r#str = cevalBuiltinStringFormat(r#str.clone(), ((r#str).len() as i32), len.clone(), just.clone());
            metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn cevalBuiltinStringFormat(
    mut inString: ArcStr,
    mut stringLength: i32,
    mut minLength: i32,
    mut leftJustified: bool,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = if (stringLength >= minLength) {
        inString
    } else {
        if (leftJustified) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inString);
                __mm_s.push_str(&*stringAppendList(List::fill(literal!(" "), minLength - stringLength)));
                ArcStr::from(__mm_s)
            }
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringAppendList(List::fill(literal!(" "), minLength - stringLength)));
                __mm_s.push_str(&*inString);
                ArcStr::from(__mm_s)
            }
        }
    };
    outString
}

fn simplifyStringAppendList(
    mut iexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iacc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut ichange: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &((iexpl, iacc, ichange)) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") })
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
            exp = (*__esc_exp).clone();
            exp.clone()
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: DAE::Operator::ADD { ty: DAE::T_STRING_DEFAULT().clone() }, exp2: exp1.clone() })
        },
        (Deref @ metamodelica::ListNode::Nil, acc, true) => {
            let mut acc = (*acc).clone();
            acc = acc.clone().reverse();
            exp = metamodelica::Ref::new(DAE::Exp::LIST { valList: acc.clone() });
            Expression::makePureBuiltinCall(literal!("stringAppendList"), list![exp], DAE::T_STRING_DEFAULT().clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::SCONST { string: s1 }, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::SCONST { string: s2 }, tail: acc }, _) => {
            let mut s: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s2); __mm_s.push_str(&*s1); ArcStr::from(__mm_s) };
            simplifyStringAppendList(rest.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::Exp::SCONST { string: s }), acc.clone()), true)?
        },
        (Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: rest }, acc, change) => {
            exp = (*__esc_exp).clone();
            simplifyStringAppendList(rest.clone(), metamodelica::cons(exp.clone(), acc.clone()), change.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(exp)
}

fn simplifyBuiltinConstantCalls(
    mut name: &ArcStr,
    mut exp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (name.clone(), &**exp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "der", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    e1 = simplifyBuiltinConstantDer(metamodelica::AsArg::as_arg(&e))?;
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "pre", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "previous", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "edge", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "change", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sqrt", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = Expression::toReal(metamodelica::AsArg::as_arg(&e))?;
                    let true = (r >= metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    r = (r).sqrt();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "abs", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r = (*r).clone();
                    r = (r.clone()).abs();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "abs", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut i = (*i).clone();
                    i = (i.clone()).abs();
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sin", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = (Expression::toReal(metamodelica::AsArg::as_arg(&e))?).sin();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cos", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = (Expression::toReal(metamodelica::AsArg::as_arg(&e))?).cos();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "asin", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = Expression::toReal(metamodelica::AsArg::as_arg(&e))?;
                    let true = (r >= metamodelica::OrderedFloat(-1.0_f64) && r <= metamodelica::OrderedFloat(1.0_f64)) else { return Err("pattern mismatch") };
                    r = (r).asin();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "acos", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = Expression::toReal(metamodelica::AsArg::as_arg(&e))?;
                    let true = (r >= metamodelica::OrderedFloat(-1.0_f64) && r <= metamodelica::OrderedFloat(1.0_f64)) else { return Err("pattern mismatch") };
                    r = (Expression::toReal(metamodelica::AsArg::as_arg(&e))?).acos();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "tan", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = (Expression::toReal(metamodelica::AsArg::as_arg(&e))?).tan();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exp", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = (Expression::toReal(metamodelica::AsArg::as_arg(&e))?).exp();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "log", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = Expression::toReal(metamodelica::AsArg::as_arg(&e))?;
                    let true = (r > metamodelica::OrderedFloat((0) as f64)) else { return Err("pattern mismatch") };
                    r = (r).ln();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "log10", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut r: metamodelica::Real;
                    r = Expression::toReal(metamodelica::AsArg::as_arg(&e))?;
                    let true = (r > metamodelica::OrderedFloat((0) as f64)) else { return Err("pattern mismatch") };
                    r = (r).log10();
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "min", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: j }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }) => {
                    let mut i = (*i).clone();
                    i = std::cmp::min(i.clone(), j.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "min", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, .. }) => {
                    let mut r: metamodelica::Real;
                    let mut v1: metamodelica::Real;
                    let mut v2: metamodelica::Real;
                    v1 = Expression::toReal(metamodelica::AsArg::as_arg(&e))?;
                    v2 = Expression::toReal(metamodelica::AsArg::as_arg(&e1))?;
                    r = std::cmp::min(v1, v2);
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "min", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::ENUM_LITERAL { index: i, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: e1 @ Deref @ DAE::Exp::ENUM_LITERAL { index: j, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    e2 = if (i.clone() < j.clone()) {e.clone()} else {e1.clone()};
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "max", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: j }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }) => {
                    let mut i = (*i).clone();
                    i = std::cmp::max(i.clone(), j.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "max", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, .. }) => {
                    let mut r: metamodelica::Real;
                    let mut v1: metamodelica::Real;
                    let mut v2: metamodelica::Real;
                    v1 = Expression::toReal(metamodelica::AsArg::as_arg(&e))?;
                    v2 = Expression::toReal(metamodelica::AsArg::as_arg(&e1))?;
                    r = std::cmp::max(v1, v2);
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "max", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::ENUM_LITERAL { index: i, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: e1 @ Deref @ DAE::Exp::ENUM_LITERAL { index: j, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    e2 = if (i.clone() > j.clone()) {e.clone()} else {e1.clone()};
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sign", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: r }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut i: i32;
                    i = if (realEq(r.clone(), metamodelica::OrderedFloat(0.0_f64))) {0} else {if (realGt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) {1} else {-1}};
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn simplifyCref(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut inCREF: &ComponentRef,
    mut inType: Type,
) -> metamodelica::Ref<DAE::Exp> {
    let mut exp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    exp = 'mc: {
        let __mc_input = &**inCREF;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: idn, identType: t2, subscriptLst: ssl @ Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: _ } }, tail: _ } } => {
                    let mut cr: ComponentRef;
                    let mut expCref: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    cr = ComponentReferenceBasics::makeCrefIdent(idn.clone(), t2.clone(), metamodelica::nil());
                    expCref = Expression::makeCrefExp(cr.clone(), inType.clone())?;
                    exp = simplifyCref2(expCref.clone(), ssl.clone())?;
                    Ok((exp.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::RANGE { .. } }, tail: _ }, .. } => {
                    let mut cr: ComponentRef;
                    let mut expCref: metamodelica::Ref<DAE::Exp>;
                    cr = ComponentReference::crefStripSubs(inCREF)?;
                    expCref = Expression::makeCrefExp(cr.clone(), inType.clone())?;
                    Ok(simplifyCref2(expCref.clone(), var_field!((**inCREF).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: Deref @ DAE::Type::T_METATYPE { ty: t2 }, subscriptLst: ssl, componentRef: cr } => {
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    exp = simplifyCrefMM1(idn.clone(), t2.clone(), ssl.clone());
                    exp = simplifyCrefMM(exp.clone(), Expression::r#typeof(exp.clone())?, metamodelica::AsArg::as_arg(&cr))?;
                    Ok((exp.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    exp
}

fn simplifyCref2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSsl: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp, inSsl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp_1, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(exp_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl_1 } }, tail: ssl }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut dim: i32;
                    let mut t = (*t).clone();
                    subs = List::map(expl_1.clone(), &fnptr!(Expression::makeIndexSubscript, metamodelica::Ref<DAE::Exp>))?;
                    crefs = List::map1r(List::map(subs.clone(), &fnptr!(List::create, _))?, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>| ComponentReference::subscriptCref(&__a0, __a1), cr.clone())?;
                    t = Types::unliftArray(metamodelica::AsArg::as_arg(&t))?;
                    expl = List::map1(crefs.clone(), &Expression::makeCrefExp, t.clone())?;
                    dim = ((expl).len() as i32);
                    exp = simplifyCref2(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] }), scalar: true, array: expl.clone() }), ssl.clone())?;
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Exp::CREF { componentRef: cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: ss @ Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::RANGE { .. } }, tail: ssl }) => {
                            let mut exp: metamodelica::Ref<DAE::Exp>;
                            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut dim: i32;
                            let mut t = (*t).clone();
                            subs = Expression::expandSliceExp(var_field!((**ss).exp, DAE::Subscript::SLICE))?;
                            crefs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut s in (subs.clone()).into_iter().cloned() {
                            let __x = ComponentReference::subscriptCref(metamodelica::AsArg::as_arg(&cr), List::create(s.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            t = Types::unliftArray(metamodelica::AsArg::as_arg(&t))?;
                            expl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut cr in (crefs.clone()).into_iter().cloned() {
                            let __x = Expression::makeCrefExp(cr.clone(), t.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            dim = ((expl).len() as i32);
                            exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] }), scalar: true, array: expl.clone() });
                            Ok(simplifyCref2(exp.clone(), ssl.clone())?)
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: tp, scalar: sc, array: expl }, ssl) => {
                    let mut expl = (*expl).clone();
                    expl = List::map1(expl.clone(), &simplifyCref2, ssl.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: sc.clone(), array: expl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn simplifyCrefMM_index(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut ident: ArcStr,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut index: i32;
    let mut nty: metamodelica::Ref<DAE::Type>;
    let mut fields: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    fields = Types::getMetaRecordFields(ty)?;
    index = Types::findVarIndex(ident.clone(), &fields)? + 1;
    let __arc1 = (fields).get(index)?;
    let DAE::TYPES_VAR { ty: __pa0, .. } = &*__arc1;
    nty = metamodelica::Own::own(__pa0);
    exp = metamodelica::Ref::new(DAE::Exp::RSUB {
        exp: inExp,
        ix: index,
        fieldName: ident,
        ty: nty,
    });
    Ok(exp)
}

fn simplifyCrefMM<'__b>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inCref: &'__b ComponentRef,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT { .. } => {
            exp = simplifyCrefMM_index(
                inExp,
                var_field!((**inCref).ident, DAE::ComponentRef::CREF_IDENT).clone(),
                inType,
            )?;
            exp = if ((var_field!((**inCref).subscriptLst, DAE::ComponentRef::CREF_IDENT)).is_empty()) {
                exp
            } else {
                metamodelica::Ref::new(DAE::Exp::ASUB {
                    exp: exp,
                    sub: var_field!((**inCref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(),
                })
            };
            exp
        }
        DAE::ComponentRef::CREF_QUAL { .. } => {
            exp = simplifyCrefMM_index(
                inExp,
                var_field!((**inCref).ident, DAE::ComponentRef::CREF_QUAL).clone(),
                inType,
            )?;
            exp = if ((var_field!((**inCref).subscriptLst, DAE::ComponentRef::CREF_QUAL)).is_empty()) {
                exp
            } else {
                metamodelica::Ref::new(DAE::Exp::ASUB {
                    exp: exp,
                    sub: var_field!((**inCref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(),
                })
            };
            exp = simplifyCrefMM(
                exp.clone(),
                Expression::r#typeof(exp)?,
                var_field!((**inCref).componentRef, DAE::ComponentRef::CREF_QUAL),
            )?;
            exp
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(exp)
}

fn simplifyCrefMM1(
    mut ident: ArcStr,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut ssl: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(ssl.clone()) {
        Deref @ metamodelica::ListNode::Nil => metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident, identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty }),
        _ => metamodelica::Ref::new(DAE::Exp::ASUB { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident, identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty }), sub: ssl }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn simplify2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut simplifyAddOrSub: bool,
    mut simplifyMulOrDiv: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Expression::r#typeof(inExp.clone())?;
    if !(Expression::isIntegerOrReal(&ty)) {
        outExp = inExp;
        return Ok(outExp);
    }
    outExp = (match &*inExp.clone() {
        DAE::Exp::BINARY { operator: op, .. }
            if (simplifyAddOrSub && Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op))) =>
        {
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut exp_3: metamodelica::Ref<DAE::Exp>;
            let mut resConst: metamodelica::Ref<DAE::Exp>;
            let mut lstConstExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut lstExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut hasConst: bool;
            lstExp = Expression::terms(inExp.clone())?;
            (lstConstExp, lstExp) = List::splitOnTrue(&lstExp, &move |__a0: metamodelica::Ref<DAE::Exp>| {
                Expression::isConstValue(&__a0)
            })?;
            hasConst = !((lstConstExp).is_empty());
            resConst = if (hasConst) {
                simplifyBinaryAddConstants(&lstConstExp)?
            } else {
                Expression::makeConstZero(&ty)
            };
            exp_2 = if (hasConst) {
                Expression::makeSum1(lstExp, false)?
            } else {
                inExp
            };
            exp_3 = simplifyBinaryCoeff(exp_2);
            exp_3 = if (hasConst) {
                Expression::expAdd(resConst, simplify2(exp_3, false, true)?)?
            } else {
                simplify2(exp_3, false, true)?
            };
            exp_3
        }
        DAE::Exp::BINARY {
            exp1: e1,
            operator: op,
            exp2: e2,
        } if (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op))) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = simplify2(e1.clone(), false, true)?;
            e2 = simplify2(e2.clone(), false, true)?;
            metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: e1.clone(),
                operator: op.clone(),
                exp2: e2.clone(),
            })
        }
        DAE::Exp::BINARY { operator: op, .. }
            if (simplifyMulOrDiv && Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op))) =>
        {
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut exp_3: metamodelica::Ref<DAE::Exp>;
            let mut resConst: metamodelica::Ref<DAE::Exp>;
            let mut lstConstExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut lstExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            lstExp = Expression::factors(&inExp)?;
            (lstConstExp, lstExp) = List::splitOnTrue(&lstExp, &Expression::isConst)?;
            if !((lstConstExp).is_empty()) {
                resConst = simplifyBinaryMulConstants(&lstConstExp)?;
                exp_2 = Expression::makeProductLst(
                    if (Types::isScalarReal(&(Expression::typeofOp(metamodelica::AsArg::as_arg(&op))))) {
                        simplifyMul(lstExp)?
                    } else {
                        lstExp
                    },
                )?;
                if Expression::isConstOne(&resConst) {
                    exp_3 = simplify2(exp_2, true, false)?;
                } else if Expression::isConstMinusOne(&resConst) {
                    exp_3 = Expression::negate(simplify2(exp_2, true, false)?)?;
                } else {
                    exp_3 = Expression::expMul(resConst, simplify2(exp_2, true, false)?)?;
                }
            } else {
                exp_2 = simplifyBinaryCoeff(inExp);
                exp_3 = simplify2(exp_2, true, false)?;
            }
            exp_3
        }
        DAE::Exp::BINARY {
            exp1: e1,
            operator: op,
            exp2: e2,
        } if (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op))) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = simplify2(e1.clone(), true, false)?;
            e2 = simplify2(e2.clone(), true, false)?;
            metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: e1.clone(),
                operator: op.clone(),
                exp2: e2.clone(),
            })
        }
        DAE::Exp::BINARY {
            exp1: e1,
            operator: op,
            exp2: e2,
        } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = simplify2(e1.clone(), true, true)?;
            e2 = simplify2(e2.clone(), true, true)?;
            metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: e1.clone(),
                operator: op.clone(),
                exp2: e2.clone(),
            })
        }
        DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut e1 = (*e1).clone();
            e1 = simplify2(e1.clone(), true, true)?;
            metamodelica::Ref::new(DAE::Exp::UNARY {
                operator: op.clone(),
                exp: e1.clone(),
            })
        }
        _ => inExp,
    });
    Ok(outExp)
}

fn simplifyBinaryArrayOp(mut inOperator: &Operator) -> bool {
    let mut found: bool;
    found = (match inOperator.clone() {
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => true,
        DAE::Operator::ADD_ARR { .. } => true,
        DAE::Operator::SUB_ARR { .. } => true,
        DAE::Operator::MUL_ARR { .. } => true,
        DAE::Operator::DIV_ARR { .. } => true,
        DAE::Operator::POW_ARR { .. } => true,
        DAE::Operator::POW_ARR2 { .. } => true,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => true,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => true,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => true,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => true,
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => true,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => true,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => true,
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => true,
        _ => false,
    });
    found
}

fn simplifyBinaryArray(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inOperator2: Operator,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp1, inOperator2, inExp3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::MUL_MATRIX_PRODUCT { .. }, e2) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    e_1 = simplifyMatrixProduct(e1.clone(), e2.clone())?;
                    Ok(e_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::ADD_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyVectorBinary0(e1.clone(), metamodelica::AsArg::as_arg(&op), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::SUB_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyVectorBinary0(e1.clone(), metamodelica::AsArg::as_arg(&op), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::MUL_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyVectorBinary(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&op), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::DIV_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyVectorBinary(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&op), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::POW_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    tp = Expression::r#typeof(e1.clone())?;
                    a1 = simplifyMatrixPow(e1.clone(), &tp, metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::POW_ARR2 { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    tp = Expression::r#typeof(e1.clone())?;
                    a1 = simplifyVectorBinary(metamodelica::AsArg::as_arg(&e1), &(DAE::Operator::POW_ARR2 { ty: tp.clone() }), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::SUB_ARR { ty: tp }, Deref @ DAE::Exp::UNARY { operator: _, exp: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::ADD_ARR { ty: tp }, Deref @ DAE::Exp::UNARY { operator: _, exp: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a1, op, s1) => {
                    let mut op = (*op).clone();
                    let true = (Expression::isArrayScalarOp(metamodelica::AsArg::as_arg(&op))) else { return Err("pattern mismatch") };
                    op = unliftOperator(metamodelica::AsArg::as_arg(&a1), op.clone())?;
                    Ok(simplifyVectorScalar(a1.clone(), op.clone(), s1.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (s1, op, a1) => {
                    let mut op = (*op).clone();
                    let true = (Expression::isScalarArrayOp(metamodelica::AsArg::as_arg(&op))) else { return Err("pattern mismatch") };
                    op = unliftOperator(metamodelica::AsArg::as_arg(&a1), op.clone())?;
                    Ok(simplifyVectorScalar(s1.clone(), op.clone(), a1.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::MUL_SCALAR_PRODUCT { .. }, e2) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    res = simplifyScalarProduct(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::ADD_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), metamodelica::AsArg::as_arg(&op), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::SUB_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), metamodelica::AsArg::as_arg(&op), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::MUL_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), metamodelica::AsArg::as_arg(&op), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::DIV_ARR { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), metamodelica::AsArg::as_arg(&op), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::POW_ARR2 { .. }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), metamodelica::AsArg::as_arg(&op), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::MUL_ARRAY_SCALAR { ty: tp }, e2) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    (a1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::DIV_ARR { .. }, _) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&e1))?) else { return Err("pattern mismatch") };
                    tp = Expression::r#typeof(e1.clone())?;
                    (a1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::DIV_ARRAY_SCALAR { .. }, _) => {
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&e1))?) else { return Err("pattern mismatch") };
                    tp = Expression::r#typeof(e1.clone())?;
                    (a1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
                    Ok(a1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub(crate) fn simplifyScalarProduct(
    mut inVector1: &metamodelica::Ref<DAE::Exp>,
    mut inVector2: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outProduct: metamodelica::Ref<DAE::Exp>;
    outProduct = (::match_deref::match_deref! { match (inVector1, inVector2) {
        (Deref @ DAE::Exp::ARRAY { ty: tp, array: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }) => {
            Expression::makeConstZero(tp)
        },
        (Deref @ DAE::Exp::ARRAY { array: expl1, .. }, Deref @ DAE::Exp::ARRAY { array: expl2, .. }) => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let true = (Expression::isVector(inVector1) && Expression::isVector(inVector2)) else { return Err("pattern mismatch") };
            expl = List::threadMap(expl1.clone(), expl2.clone(), &Expression::expMul)?;
            exp = List::reduce(&expl, &Expression::expAdd)?;
            exp
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) if (!metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("Cpp")))) => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            expl1 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut c in (ComponentReference::expandCref(cr1, true)?).into_iter().cloned() {
            let __x = Expression::crefToExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            let true = (((expl1).len() as i32) <= 3) else { return Err("pattern mismatch") };
            expl2 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut c in (ComponentReference::expandCref(cr2, true)?).into_iter().cloned() {
            let __x = Expression::crefToExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            let true = (((expl1).len() as i32) == ((expl2).len() as i32)) else { return Err("pattern mismatch") };
            let true = (List::none(&expl1, &isArrayTypedExp)? && List::none(&expl2, &isArrayTypedExp)?) else { return Err("pattern mismatch") };
            expl = List::threadMap(expl1, expl2, &Expression::expMul)?;
            exp = List::reduce(&expl, &Expression::expAdd)?;
            exp
        },
        (_, _) => {
            let true = (Expression::isZero(inVector1)? || Expression::isZero(inVector2)?) else { return Err("pattern mismatch") };
            Expression::makeConstZero(&(DAE::T_REAL_DEFAULT().clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outProduct)
}

fn isArrayTypedExp(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut b: bool = Expression::isArrayType(&(Expression::r#typeof(exp.clone())?));
    Ok(b)
}

fn unliftOperator(mut inArray: &metamodelica::Ref<DAE::Exp>, mut inOperator: Operator) -> Result<Operator> {
    let mut outOperator: Operator;
    outOperator = (match &**inArray {
        DAE::Exp::MATRIX { .. } => Expression::unliftOperatorX(inOperator, 2)?,
        _ => Expression::unliftOperator(inOperator)?,
    });
    Ok(outOperator)
}

fn simplifyVectorScalar(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inOperator: Operator,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inLhs.clone(), inRhs.clone())) {
        (_, Deref @ DAE::Exp::ARRAY { ty: tp, scalar: sc, array: es }) => {
            let mut es = (*es).clone();
            es = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (es.clone()).into_iter().cloned() {
            let __x = Expression::makeBinaryExp(inLhs.clone(), inOperator.clone(), e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: sc.clone(), array: es.clone() })
        },
        (s1, Deref @ DAE::Exp::MATRIX { ty: tp, integer: dims, matrix: mexpl }) => {
            let mut op = inOperator.clone();
            let mut mexpl = (*mexpl).clone();
            mexpl = simplifyVectorScalarMatrix(mexpl.clone(), op, s1.clone(), false);
            metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dims.clone(), matrix: mexpl.clone() })
        },
        (Deref @ DAE::Exp::ARRAY { ty: tp, scalar: sc, array: es }, _) => {
            let mut es = (*es).clone();
            es = List::map2(es.clone(), &fnptr!(Expression::makeBinaryExp, metamodelica::Ref<DAE::Exp>, DAE::Operator, metamodelica::Ref<DAE::Exp>), inOperator, inRhs)?;
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: sc.clone(), array: es.clone() })
        },
        (Deref @ DAE::Exp::MATRIX { ty: tp, integer: dims, matrix: mexpl }, s1) => {
            let mut op = inOperator.clone();
            let mut mexpl = (*mexpl).clone();
            mexpl = simplifyVectorScalarMatrix(mexpl.clone(), op, s1.clone(), true);
            metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dims.clone(), matrix: mexpl.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyVectorBinary0(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut op: &Operator,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = 'mc: {
        let __mc_input = op.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut a1: metamodelica::Ref<DAE::Exp>;
            a1 = simplifyVectorBinary(&e1, op, &e2)?;
            Ok(a1.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::ADD { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (Expression::isZero(&e1)?) else {
                return Err("pattern mismatch");
            };
            Ok(e2.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::ADD_ARR { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (Expression::isZero(&e1)?) else {
                return Err("pattern mismatch");
            };
            Ok(e2.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::SUB_ARR { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (Expression::isZero(&e1)?) else {
                return Err("pattern mismatch");
            };
            Ok(Expression::negate(e2.clone())?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::SUB { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (Expression::isZero(&e1)?) else {
                return Err("pattern mismatch");
            };
            Ok(Expression::negate(e2.clone())?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Expression::isZero(&e2)?) else {
                return Err("pattern mismatch");
            };
            Ok(e1.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(res)
}

fn simplifyVectorBinary(
    mut inLhs: &metamodelica::Ref<DAE::Exp>,
    mut inOperator: &Operator,
    mut inRhs: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outResult: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut sc: bool;
    let mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut op: Operator;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inLhs)) {
        Deref @ DAE::Exp::ARRAY { ty: __pa0, scalar: __pa1, array: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    sc = metamodelica::Own::own(__pa1);
    lhs = metamodelica::Own::own(__pa2);
    let __pa3 = ::match_deref::match_deref! { match &((*inRhs)) {
        Deref @ DAE::Exp::ARRAY { array: __pa3, .. } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    rhs = metamodelica::Own::own(__pa3);
    op = removeOperatorDimension(inOperator)?;
    res = List::threadMap1(
        lhs,
        rhs,
        &fnptr!(
            simplifyVectorBinary2,
            metamodelica::Ref<DAE::Exp>,
            metamodelica::Ref<DAE::Exp>,
            DAE::Operator
        ),
        op,
    )?;
    outResult = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: ty,
        scalar: sc,
        array: res,
    });
    Ok(outResult)
}

fn simplifyVectorBinary2(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
    mut inOperator: Operator,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: inLhs,
        operator: inOperator,
        exp2: inRhs,
    });
    outExp
}

fn simplifyMatrixBinary(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inOperator: &Operator,
    mut inRhs: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outResult: metamodelica::Ref<DAE::Exp>;
    let mut lhs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut rhs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut res: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut op: Operator;
    let mut sz: i32;
    let mut ty: metamodelica::Ref<DAE::Type>;
    lhs = Expression::get2dArrayOrMatrixContent(&inLhs)?;
    rhs = Expression::get2dArrayOrMatrixContent(inRhs)?;
    op = removeOperatorDimension(inOperator)?;
    res = List::threadMap1(lhs, rhs, &simplifyMatrixBinary1, op)?;
    sz = ((res).len() as i32);
    ty = Expression::r#typeof(inLhs)?;
    outResult = metamodelica::Ref::new(DAE::Exp::MATRIX {
        ty: ty,
        integer: sz,
        matrix: res,
    });
    Ok(outResult)
}

fn simplifyMatrixBinary1(
    mut inLhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inRhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inOperator: Operator,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpl = List::threadMap1(
        inLhs,
        inRhs,
        &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>, __a2: DAE::Operator| {
            simplifyMatrixBinary2(__a0, __a1, &__a2)
        },
        inOperator,
    )?;
    Ok(outExpl)
}

fn simplifyMatrixBinary2(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
    mut inOperator: &Operator,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut op: Operator;
    op = removeOperatorDimension(inOperator)?;
    outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: inLhs,
        operator: op,
        exp2: inRhs,
    });
    Ok(outExp)
}

fn simplifyMatrixPow(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inType: &Type,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp1, &**inExp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { ty: tp1, integer: size1, .. }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut expl_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut expl2: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut el: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut range: metamodelica::List<i32>;
                    let 0 = (i.clone()) else { return Err("pattern mismatch") };
                    el = List::fill(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), size1.clone());
                    expl2 = List::fill(el.clone(), size1.clone());
                    range = List::intRange2(0, size1.clone() - 1);
                    expl_1 = simplifyMatrixPow1(&range, &expl2, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp1.clone(), integer: size1.clone(), matrix: expl_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { .. }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let 1 = (i.clone()) else { return Err("pattern mismatch") };
                    Ok(m.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { .. }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let 2 = (i.clone()) else { return Err("pattern mismatch") };
                    res = simplifyMatrixProduct(m.clone(), m.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { ty: tp1, .. }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut i_1: i32;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (i.clone() > 3) else { return Err("pattern mismatch") };
                    let 0 = (intMod(i.clone(), 2)) else { return Err("pattern mismatch") };
                    i_1 = intDiv(i.clone(), 2);
                    e = simplifyMatrixPow(m.clone(), metamodelica::AsArg::as_arg(&tp1), &(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i_1 })))?;
                    res = simplifyMatrixProduct(e.clone(), e.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { ty: tp1, .. }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut i_1: i32;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (1 < i.clone()) else { return Err("pattern mismatch") };
                    i_1 = i.clone() - 1;
                    e = simplifyMatrixPow(m.clone(), metamodelica::AsArg::as_arg(&tp1), &(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i_1 })))?;
                    res = simplifyMatrixProduct(m.clone(), e.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn simplifyMatrixPow1(
    mut inRange: &metamodelica::List<i32>,
    mut inMatrix: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inValue: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut outMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    outMatrix = 'mc: {
        let __mc_input = (&**inRange, &**inMatrix, inValue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: row, tail: Deref @ metamodelica::ListNode::Nil }, e) => {
                    let mut row1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    row1 = List::replaceAt(e.clone(), i.clone() + 1, row.clone())?;
                    Ok(list![row1.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: i, tail: rr }, Deref @ metamodelica::ListNode::Cons { head: row, tail: rm }, e) => {
                    let mut rm1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut row1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    row1 = List::replaceAt(e.clone(), i.clone() + 1, row.clone())?;
                    rm1 = simplifyMatrixPow1(metamodelica::AsArg::as_arg(&rr), metamodelica::AsArg::as_arg(&rm), e.clone())?;
                    Ok(metamodelica::cons(row1.clone(), rm1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMatrix)
}

fn simplifyMatrixProduct(
    mut inMatrix1: metamodelica::Ref<DAE::Exp>,
    mut inMatrix2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outProduct: metamodelica::Ref<DAE::Exp>;
    let mut mat1: metamodelica::Ref<DAE::Exp>;
    let mut mat2: metamodelica::Ref<DAE::Exp>;
    mat1 = Expression::matrixToArray(inMatrix1)?;
    mat2 = Expression::matrixToArray(inMatrix2)?;
    (mat2, _) = Expression::transposeArray(mat2)?;
    outProduct = simplifyMatrixProduct2(mat1, mat2)?;
    Ok(outProduct)
}

fn simplifyMatrixProduct2(
    mut inMatrix1: metamodelica::Ref<DAE::Exp>,
    mut inMatrix2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outProduct: metamodelica::Ref<DAE::Exp>;
    outProduct = 'mc: {
        let __mc_input = (&*inMatrix1, &*inMatrix2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: ty @ Deref @ DAE::Type::T_ARRAY { dims, .. }, .. }, Deref @ DAE::Exp::ARRAY { .. }) => {
                    let mut zero: metamodelica::Ref<DAE::Exp>;
                    let mut dims = (*dims).clone();
                    let true = (Expression::arrayContainZeroDimension(metamodelica::AsArg::as_arg(&dims))) else { return Err("pattern mismatch") };
                    zero = Expression::makeConstZero(metamodelica::AsArg::as_arg(&ty));
                    dims = simplifyMatrixProduct4(&inMatrix1, &inMatrix2)?;
                    Ok(Expression::arrayFill(dims.clone(), zero.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: n, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, array: expl1, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }) => {
                    let mut ty = (*ty).clone();
                    let mut expl1 = (*expl1).clone();
                    expl1 = List::map1(expl1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| simplifyScalarProduct(&__a0, &__a1), inMatrix2.clone())?;
                    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![n.clone()] });
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: m, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, array: expl2, .. }) => {
                    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ty = (*ty).clone();
                    expl1 = List::map1r(expl2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| simplifyScalarProduct(&__a0, &__a1), inMatrix1.clone())?;
                    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![m.clone()] });
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: n, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, array: expl1, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: p, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, array: expl2, .. }) => {
                    let mut row_ty: metamodelica::Ref<DAE::Type>;
                    let mut matrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut expl1 = (*expl1).clone();
                    matrix = List::map1(expl1.clone(), &simplifyMatrixProduct3, expl2.clone())?;
                    row_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![p.clone()] });
                    expl1 = List::map2(matrix.clone(), &fnptr!(Expression::makeArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::Type>, bool), row_ty.clone(), true)?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![n.clone(), p.clone()] }), scalar: false, array: expl1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outProduct)
}

fn simplifyMatrixProduct3(
    mut inRow: metamodelica::Ref<DAE::Exp>,
    mut inMatrix: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outRow: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outRow = List::map1r(
        inMatrix,
        &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| {
            simplifyScalarProduct(&__a0, &__a1)
        },
        inRow,
    )?;
    Ok(outRow)
}

fn simplifyMatrixProduct4(
    mut inMatrix1: &metamodelica::Ref<DAE::Exp>,
    mut inMatrix2: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    outDimensions = (::match_deref::match_deref! { match (inMatrix1, inMatrix2) {
        (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: n, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }) => {
            list![n.clone()]
        },
        (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: m, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }) => {
            list![m.clone()]
        },
        (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: n, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: p, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }) => {
            list![n.clone(), p.clone()]
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDimensions)
}

fn simplifyBinarySortConstants(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL { .. }, .. } => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    res = simplifyBinarySortConstantsMul(e.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e2 } => {
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    e1 = simplifyBinarySortConstantsMul(e1.clone())?;
                    e2 = simplifyBinarySortConstantsMul(e2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD { .. }, .. } => {
                    let mut e_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut const_es1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut notconst_es1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut res1: metamodelica::Ref<DAE::Exp>;
                    let mut res2: metamodelica::Ref<DAE::Exp>;
                    e_lst = Expression::terms(e.clone())?;
                    (const_es1, notconst_es1) = List::splitOnTrue(&e_lst, &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::isConstValue(&__a0))?;
                    if !((const_es1).is_empty()) {
                        res1 = simplifyBinaryAddConstants(&const_es1)?;
                        res2 = Expression::makeSum1(notconst_es1.clone(), false)?;
                        res = Expression::expAdd(res1.clone(), res2.clone())?;
                    } else {
                        res = inExp.clone();
                    }
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn simplifyBinaryCoeff(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL { ty: tp }, .. } => {
                    if !((Types::isScalarReal(metamodelica::AsArg::as_arg(&tp)))) { return Err("guard") }
                    let mut e_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_lst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    e_lst = Expression::factors(metamodelica::AsArg::as_arg(&e))?;
                    e_lst_1 = simplifyMul(e_lst.clone())?;
                    res = Expression::makeProductLst(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                    let mut e_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_lst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e1_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e2_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e2_lst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let false = (Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    e1_lst = Expression::factors(metamodelica::AsArg::as_arg(&e1))?;
                    e2_lst = Expression::factors(metamodelica::AsArg::as_arg(&e2))?;
                    e2_lst_1 = List::map(e2_lst.clone(), &Expression::inverseFactors)?;
                    e_lst = listAppend(e1_lst.clone(), e2_lst_1.clone());
                    e_lst_1 = simplifyMul(e_lst.clone())?;
                    res = Expression::makeProductLst(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD { .. }, .. } => {
                    let mut e_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_lst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    e_lst = Expression::terms(e.clone())?;
                    e_lst_1 = simplifyAdd(e_lst.clone())?;
                    res = Expression::makeSum(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                    let mut e_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_lst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e1_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e2_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    e1_lst = Expression::terms(e1.clone())?;
                    e2_lst = Expression::terms(e2.clone())?;
                    e2_lst = List::map(e2_lst.clone(), &Expression::negate)?;
                    e_lst = listAppend(e1_lst.clone(), e2_lst.clone());
                    e_lst_1 = simplifyAdd(e_lst.clone())?;
                    res = Expression::makeSum(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn simplifyBinaryAddConstants(
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tp: Type;
    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inExpLst)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outExp = metamodelica::Own::own(__pa0);
    es = metamodelica::Own::own(__pa1);
    tp = Expression::r#typeof(outExp.clone())?;
    for mut e in &*es {
        outExp = simplifyBinaryConst(
            &(DAE::Operator::ADD { ty: tp.clone() }),
            &outExp,
            metamodelica::AsArg::as_arg(&e),
        )?;
    }
    Ok(outExp)
}

fn simplifyBinaryMulConstants(
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tp: Type;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inExpLst)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outExp = metamodelica::Own::own(__pa0);
    es = metamodelica::Own::own(__pa1);
    tp = Expression::r#typeof(outExp.clone())?;
    for mut e in &*es {
        outExp = simplifyBinaryConst(
            &(DAE::Operator::MUL { ty: tp.clone() }),
            &outExp,
            metamodelica::AsArg::as_arg(&e),
        )?;
    }
    Ok(outExp)
}

fn simplifyMul(
    mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut exp_const: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>;
    let mut exp_const_1: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>;
    exp_const = List::map(expl, &simplifyBinaryMulCoeff2)?;
    exp_const_1 = simplifyMulJoinFactors(exp_const)?;
    expl_1 = simplifyMulMakePow(&exp_const_1);
    Ok(expl_1)
}

fn simplifyMulJoinFactors(
    mut inTplExpRealLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>> {
    let mut outTplExpRealLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)> =
        metamodelica::nil();
    let mut tplExpRealLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)> = inTplExpRealLst;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut coeff: metamodelica::Real;
    let mut coeff2: metamodelica::Real;
    while !((tplExpRealLst).is_empty()) {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(tplExpRealLst) {
            Deref @ metamodelica::ListNode::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        coeff = metamodelica::Own::own(__pa1);
        tplExpRealLst = metamodelica::Own::own(__pa2);
        (coeff2, tplExpRealLst) = simplifyMulJoinFactorsFind(&e, &tplExpRealLst)?;
        coeff = coeff + coeff2;
        outTplExpRealLst = metamodelica::cons((e, coeff), outTplExpRealLst);
    }
    Ok(outTplExpRealLst)
}

fn simplifyMulJoinFactorsFind(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inTplExpRealLst: &metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>,
) -> Result<(
    metamodelica::Real,
    metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>,
)> {
    let mut outReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut outTplExpRealLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)> =
        metamodelica::nil();
    let mut tplExpReal: (metamodelica::Ref<DAE::Exp>, metamodelica::Real) = (
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default(),
        metamodelica::OrderedFloat(0.0_f64),
    );
    for mut tplExpReal in &**inTplExpRealLst {
        let mut tplExpReal = tplExpReal.clone();
        (outReal, outTplExpRealLst) = (::match_deref::match_deref! { match &(tplExpReal.clone()) {
            (e2, coeff) if (ExpressionBasics::expEqual(inExp, e2.clone())?) => {
                (coeff.clone() + outReal, outTplExpRealLst)
            },
            (Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV { .. }, exp2: e2 }, coeff) if (if (Expression::isOne(metamodelica::AsArg::as_arg(&e1))) {ExpressionBasics::expEqual(inExp, e2.clone())?} else {ExpressionBasics::expEqual(inExp, metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))?}) => {
                (outReal - coeff.clone(), outTplExpRealLst)
            },
            _ => {
                (outReal, metamodelica::cons(tplExpReal, outTplExpRealLst))
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outTplExpRealLst = outTplExpRealLst.reverse();
    Ok((outReal, outTplExpRealLst))
}

fn simplifyMulMakePow(
    mut inTplExpRealLst: &metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut tplExpReal: (metamodelica::Ref<DAE::Exp>, metamodelica::Real) = (
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default(),
        metamodelica::OrderedFloat(0.0_f64),
    );
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut r: metamodelica::Real;
    for mut tplExpReal in &**inTplExpRealLst {
        let mut tplExpReal = tplExpReal.clone();
        (e, r) = tplExpReal;
        outExpLst = if (r == metamodelica::OrderedFloat(1.0_f64)) {
            metamodelica::cons(e, outExpLst)
        } else {
            metamodelica::cons(
                metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: e,
                    operator: DAE::Operator::POW {
                        ty: DAE::T_REAL_DEFAULT().clone(),
                    },
                    exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: r }),
                }),
                outExpLst,
            )
        };
    }
    outExpLst
}

fn simplifyAdd(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut coeffs: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>;
    match '__try0: {
        coeffs = unwrap_break_err!(List::map(inExpLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| simplifyBinaryAddCoeff2(&__a0)), '__try0);
        coeffs = unwrap_break_err!(simplifyAddJoinTerms(coeffs.clone()), '__try0);
        outExpLst = unwrap_break_err!(simplifyAddMakeMul(coeffs.clone()), '__try0);
        Ok::<_, &'static str>((coeffs.clone(), outExpLst.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            coeffs = __try0_o0;
            outExpLst = __try0_o1;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- ExpressionSimplify.simplifyAdd failed\n"))?;
            }
            return Err(__try0_err);
        }
    }
    Ok(outExpLst)
}

fn simplifyAddJoinTerms(
    mut inTplExpRealLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>> {
    fn addCoeff(
        mut oldCoeff: Option<metamodelica::Real>,
        mut newCoeff: metamodelica::Real,
    ) -> Result<metamodelica::Real> {
        let mut coeff: metamodelica::Real;
        coeff = if ((oldCoeff).is_some()) {
            oldCoeff.ok_or("pattern mismatch")? + newCoeff
        } else {
            newCoeff
        };
        Ok(coeff)
    }

    let mut outTplExpRealLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>;
    outTplExpRealLst = (::match_deref::match_deref! { match &(inTplExpRealLst.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
            inTplExpRealLst
        },
        Deref @ metamodelica::ListNode::Cons { head: (exp1, coeff1), tail: Deref @ metamodelica::ListNode::Cons { head: (exp2, coeff2), tail: Deref @ metamodelica::ListNode::Nil } } => {
            if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&exp1), exp2.clone())?) {list![(exp1.clone(), coeff1.clone() + coeff2.clone())]} else {inTplExpRealLst}
        },
        Deref @ metamodelica::ListNode::Cons { head: (exp1, coeff1), tail: Deref @ metamodelica::ListNode::Cons { head: (exp2, coeff2), tail: Deref @ metamodelica::ListNode::Cons { head: (exp3, coeff3), tail: Deref @ metamodelica::ListNode::Nil } } } => {
            if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&exp1), exp2.clone())? {
                if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&exp1), exp3.clone())? {
                    outTplExpRealLst = list![(exp1.clone(), coeff1.clone() + coeff2.clone() + coeff3.clone())];
                } else {
                    outTplExpRealLst = list![(exp1.clone(), coeff1.clone() + coeff2.clone()), (exp3.clone(), coeff3.clone())];
                }
            } else if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&exp1), exp3.clone())? {
                outTplExpRealLst = list![(exp1.clone(), coeff1.clone() + coeff3.clone()), (exp2.clone(), coeff2.clone())];
            } else if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&exp2), exp3.clone())? {
                outTplExpRealLst = list![(exp1.clone(), coeff1.clone()), (exp2.clone(), coeff2.clone() + coeff3.clone())];
            } else {
                outTplExpRealLst = inTplExpRealLst;
            }
            outTplExpRealLst
        },
        _ => {
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut coeff1: metamodelica::Real;
            let mut coeff_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::Exp>, metamodelica::Real>>;
            coeff_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| ExpressionBasics::hashExp(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| ExpressionBasics::expEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>), ((inTplExpRealLst).len() as i32));
            for mut tpl in &*inTplExpRealLst {
                (exp1, coeff1) = tpl.clone();
                UnorderedMap::addUpdate(exp1, &({ let __pe_b1 = coeff1; move |__pe_a0| addCoeff(__pe_a0, __pe_b1.clone()) }), coeff_map.clone())?;
            }
            UnorderedMap::toList(coeff_map)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTplExpRealLst)
}

fn simplifyAddMakeMul(
    mut inTplExpRealLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    outExpLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut tplExpReal in (inTplExpRealLst).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(tplExpReal.clone()) {
                (e, __rlit_2) if __rlit_2.eq(&metamodelica::OrderedFloat((1.0) as f64)) => {
                    e.clone()
                },
                (e, __rlit_3) if __rlit_3.eq(&metamodelica::OrderedFloat((-1.0) as f64)) => {
                    Expression::negate(e.clone())?
                },
                (e, r) => {
                    (match &*(Expression::r#typeof(e.clone())?) {
                DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::ICONST { integer: ((r.clone()).0.floor() as i32) }), operator: DAE::Operator::MUL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: e.clone() }),
                _ => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }),
            })
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpLst)
}

fn simplifyBinaryAddCoeff2(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)> {
    let mut outRes: (metamodelica::Ref<DAE::Exp>, metamodelica::Real);
    outRes = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { .. } => {
            (inExp.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: Deref @ DAE::Type::T_REAL { .. } }, exp } => {
            let mut coeff: metamodelica::Real;
            let mut exp = (*exp).clone();
            (exp, coeff) = simplifyBinaryAddCoeff2(metamodelica::AsArg::as_arg(&exp))?;
            coeff = (metamodelica::OrderedFloat(-1.0_f64)) * (coeff);
            (exp.clone(), coeff)
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: coeff }, operator: DAE::Operator::MUL { .. }, exp2: e1 } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::RCONST { real: coeff } } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::ICONST { integer: icoeff } } => {
            let mut coeff_1: metamodelica::Real;
            coeff_1 = intReal(icoeff.clone());
            (e1.clone(), coeff_1)
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::ICONST { integer: icoeff }, operator: DAE::Operator::MUL { .. }, exp2: e1 } => {
            let mut coeff_1: metamodelica::Real;
            coeff_1 = intReal(icoeff.clone());
            (e1.clone(), coeff_1)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } if (ExpressionBasics::expEqual(e1, e2.clone())?) => {
            (e1.clone(), metamodelica::OrderedFloat(2.0_f64))
        },
        _ => {
            (inExp.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRes)
}

fn simplifyBinaryMulCoeff2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Real)> {
    let mut outRes: (metamodelica::Ref<DAE::Exp>, metamodelica::Real);
    outRes = (::match_deref::match_deref! { match &(inExp.clone()) {
        e @ Deref @ DAE::Exp::CREF { .. } => {
            (e.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::RCONST { real: coeff } } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::RCONST { real: coeff } } } => {
            let mut coeff_1: metamodelica::Real;
            coeff_1 = metamodelica::OrderedFloat(0.0_f64) - coeff.clone();
            (e1.clone(), coeff_1)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::ICONST { integer: icoeff } } => {
            let mut coeff_1: metamodelica::Real;
            coeff_1 = intReal(icoeff.clone());
            (e1.clone(), coeff_1)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::ICONST { integer: icoeff } } } => {
            let mut coeff_1: metamodelica::Real;
            coeff_1 = metamodelica::OrderedFloat(0.0_f64) - intReal(icoeff.clone());
            (e1.clone(), coeff_1)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            (e1.clone(), metamodelica::OrderedFloat(2.0_f64))
        },
        _ => {
            (inExp, metamodelica::OrderedFloat(1.0_f64))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRes)
}

pub fn simplifySumOperatorExpression(
    mut iSum: metamodelica::Ref<DAE::Exp>,
    mut iop: DAE::Operator,
    mut iExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    let mut T: metamodelica::List<metamodelica::Ref<DAE::Exp>> = Expression::termsExpandUnary(iSum.clone())?;
    let mut b: bool;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut newE: metamodelica::Ref<DAE::Exp>;
    let mut sE: metamodelica::Ref<DAE::Exp>;
    let mut tp: metamodelica::Ref<DAE::Type> = Expression::typeofOp(&iop);
    oExp = Expression::makeConstZero(&tp);
    sE = oExp.clone();
    for mut elem in &*T {
        e = metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: elem.clone(),
            operator: iop.clone(),
            exp2: iExp.clone(),
        });
        newE = simplifyBinaryCoeff(e.clone());
        b = !(ExpressionBasics::expEqual(&e, newE.clone())?);
        if b {
            sE = Expression::expAdd(sE, newE)?;
        } else {
            oExp = Expression::expAdd(oExp, elem.clone())?;
        }
    }
    e = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: oExp,
        operator: iop,
        exp2: iExp,
    });
    oExp = Expression::expAdd(sE, e)?;
    Ok(oExp)
}

fn simplifyAsub0(
    mut ie: &metamodelica::Ref<DAE::Exp>,
    mut sub: i32,
    mut inSubExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = (::match_deref::match_deref! { match ie {
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: exps } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = (exps).get(sub)?;
            exp
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::BCONST { bool: bstart }, stop: Deref @ DAE::Exp::BCONST { bool: bstop }, .. } => {
            let mut b: bool;
            b = ((simplifyRangeBool(bstart.clone(), bstop.clone()))).get(sub)?;
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: b })
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step: None, stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut ival: i32;
            ival = ((simplifyRange(istart.clone(), 1, istop.clone())?)).get(sub)?;
            exp = metamodelica::Ref::new(DAE::Exp::ICONST { integer: ival });
            exp
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step: Some(Deref @ DAE::Exp::ICONST { integer: istep }), stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut ival: i32;
            ival = ((simplifyRange(istart.clone(), istep.clone(), istop.clone())?)).get(sub)?;
            exp = metamodelica::Ref::new(DAE::Exp::ICONST { integer: ival });
            exp
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::RCONST { real: rstart }, step: None, stop: Deref @ DAE::Exp::RCONST { real: rstop }, .. } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut rval: metamodelica::Real;
            rval = ((simplifyRangeReal(rstart.clone(), metamodelica::OrderedFloat(1.0_f64), rstop.clone())?)).get(sub)?;
            exp = metamodelica::Ref::new(DAE::Exp::RCONST { real: rval });
            exp
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::RCONST { real: rstart }, step: Some(Deref @ DAE::Exp::RCONST { real: rstep }), stop: Deref @ DAE::Exp::RCONST { real: rstop }, .. } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut rval: metamodelica::Real;
            rval = ((simplifyRangeReal(rstart.clone(), rstep.clone(), rstop.clone())?)).get(sub)?;
            exp = metamodelica::Ref::new(DAE::Exp::RCONST { real: rval });
            exp
        },
        Deref @ DAE::Exp::MATRIX { ty: t, integer: _, matrix: mexps } => {
            let mut t1: Type;
            let mut mexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            t1 = Expression::unliftArray(t)?;
            mexpl = (mexps).get(sub)?;
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: t1, scalar: true, array: mexpl })
        },
        Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: e1, expElse: e2 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = Expression::makeASUB(e1.clone(), list![inSubExp.clone()])?;
            e2 = Expression::makeASUB(e2.clone(), list![inSubExp])?;
            e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: e1.clone(), expElse: e2.clone() });
            e
        },
        Deref @ DAE::Exp::CREF { componentRef: c, ty: t } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut c_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut t = (*t).clone();
            let true = (Types::isArray(metamodelica::AsArg::as_arg(&t))) else { return Err("pattern mismatch") };
            t = Expression::unliftArray(metamodelica::AsArg::as_arg(&t))?;
            c_1 = simplifyAsubCref(c, &inSubExp)?;
            exp = Expression::makeCrefExp(c_1, t.clone())?;
            exp
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (Expression::isMulOrDiv(op) || Expression::isAddOrSub(op)) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = Expression::makeASUB(e1.clone(), list![inSubExp.clone()])?;
            e2 = Expression::makeASUB(e2.clone(), list![inSubExp])?;
            e = if (Expression::isMul(op)) {Expression::expMul(e1.clone(), e2.clone())?} else if (Expression::isDiv(op)) {Expression::makeDiv(e1.clone(), e2.clone())?} else if (Expression::isAdd(op)) {Expression::expAdd(e1.clone(), e2.clone())?} else {Expression::expSub(e1.clone(), e2.clone())?};
            e
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn simplifyAsubCref(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut sub: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut res: metamodelica::Ref<DAE::ComponentRef>;
    res = 'mc: {
        let __mc_input = &**cr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: idn, identType: t2, subscriptLst: s } => {
                    let mut c_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut s_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    s_1 = Expression::subscriptsAppend(metamodelica::AsArg::as_arg(&s), sub)?;
                    c_1 = ComponentReferenceBasics::makeCrefIdent(idn.clone(), t2.clone(), s_1.clone());
                    Ok(c_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: t2 @ Deref @ DAE::Type::T_ARRAY { dims, .. }, subscriptLst: s, componentRef: c } => {
                    let mut c_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut s_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let true = (((dims).len() as i32) > ((s).len() as i32)) else { return Err("pattern mismatch") };
                    s_1 = Expression::subscriptsAppend(metamodelica::AsArg::as_arg(&s), sub)?;
                    c_1 = ComponentReferenceBasics::makeCrefQual(idn.clone(), t2.clone(), s_1.clone(), c.clone());
                    Ok(c_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: t2, subscriptLst: s, componentRef: c } => {
                    let mut s = (*s).clone();
                    s = Expression::subscriptsReplaceSlice(metamodelica::AsArg::as_arg(&s), &(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: sub.clone() })))?;
                    Ok(ComponentReferenceBasics::makeCrefQual(idn.clone(), t2.clone(), s.clone(), c.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: t2, subscriptLst: s, componentRef: c } => {
                    let mut c_1: metamodelica::Ref<DAE::ComponentRef>;
                    c_1 = simplifyAsubCref(metamodelica::AsArg::as_arg(&c), sub)?;
                    c_1 = ComponentReferenceBasics::makeCrefQual(idn.clone(), t2.clone(), s.clone(), c_1.clone());
                    Ok(c_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(res)
}

fn simplifyAsub(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSub: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp, inSub.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, sub) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = simplifyAsub0(metamodelica::AsArg::as_arg(&e), Expression::expInt(metamodelica::AsArg::as_arg(&sub))?, inSub.clone())?;
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }, sub) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op2: Operator;
                    let mut b: bool;
                    e_1 = simplifyAsub(e.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op2 = if (b) {DAE::Operator::UMINUS_ARR { ty: t2.clone() }} else {DAE::Operator::UMINUS { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { .. }, exp: e }, sub) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    e_1 = simplifyAsub(e.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e_1.clone())?;
                    exp = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: t2.clone() }, exp: e_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op2: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op2 = if (b) {DAE::Operator::SUB_ARR { ty: t2.clone() }} else {DAE::Operator::SUB { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARRAY_SCALAR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op = if (b) {DAE::Operator::MUL_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::MUL { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARRAY_SCALAR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op = if (b) {DAE::Operator::ADD_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::ADD { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_SCALAR_ARRAY { .. }, exp2: e2 }, sub) => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e2_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op = if (b) {DAE::Operator::SUB_SCALAR_ARRAY { ty: t2.clone() }} else {DAE::Operator::SUB { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp2: e2 }, sub) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = simplifyMatrixProduct(e1.clone(), e2.clone())?;
                    e = simplifyAsub(e.clone(), sub.clone())?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_SCALAR_ARRAY { .. }, exp2: e2 }, sub) => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e2_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op = if (b) {DAE::Operator::DIV_SCALAR_ARRAY { ty: t2.clone() }} else {DAE::Operator::DIV { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op = if (b) {DAE::Operator::DIV_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::DIV { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW_SCALAR_ARRAY { .. }, exp2: e2 }, sub) => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e2_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op = if (b) {DAE::Operator::POW_SCALAR_ARRAY { ty: t2.clone() }} else {DAE::Operator::POW { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW_ARRAY_SCALAR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op = if (b) {DAE::Operator::POW_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::POW { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op2: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op2 = if (b) {DAE::Operator::ADD_ARR { ty: t2.clone() }} else {DAE::Operator::ADD { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op2: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op2 = if (b) {DAE::Operator::MUL_ARR { ty: t2.clone() }} else {DAE::Operator::MUL { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARR { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op2: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op2 = if (b) {DAE::Operator::DIV_ARR { ty: t2.clone() }} else {DAE::Operator::DIV { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW_ARR2 { .. }, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op2: Operator;
                    let mut b: bool;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(&t2);
                    op2 = if (b) {DAE::Operator::POW_ARR2 { ty: t2.clone() }} else {DAE::Operator::POW { ty: t2.clone() }};
                    exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t2: Type;
                    let mut op = (*op).clone();
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    op = Expression::setOpType(op.clone(), t2.clone())?;
                    exp = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: exps, .. }, sub) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut indx: i32;
                    indx = Expression::expInt(metamodelica::AsArg::as_arg(&sub))?;
                    exp = (exps).get(indx)?;
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { ty: t, matrix: lstexps, .. }, sub) => {
                    let mut t_1: Type;
                    let mut indx: i32;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    indx = Expression::expInt(metamodelica::AsArg::as_arg(&sub))?;
                    expl = (lstexps).get(indx)?;
                    t_1 = Expression::unliftArray(metamodelica::AsArg::as_arg(&t))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: t_1.clone(), scalar: true, array: expl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: e1, expElse: e2 }, sub) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: e1_1.clone(), expElse: e2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, iterType: Absyn::ReductionIterType::THREAD { .. }, .. }, expr: exp, iterators: iters }, sub) => {
                    let mut exp = (*exp).clone();
                    exp = List::fold1(metamodelica::AsArg::as_arg(&iters), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>, __a1: metamodelica::Ref<DAE::Exp>, __a2: metamodelica::Ref<DAE::Exp>| simplifyAsubArrayReduction(&__a0, __a1, __a2), sub.clone(), exp.clone())?;
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, iterType: Absyn::ReductionIterType::COMBINE { .. }, .. }, expr: exp, iterators: Deref @ metamodelica::ListNode::Cons { head: iter, tail: Deref @ metamodelica::ListNode::Nil } }, sub) => {
                    let mut exp = (*exp).clone();
                    exp = simplifyAsubArrayReduction(metamodelica::AsArg::as_arg(&iter), sub.clone(), exp.clone())?;
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn simplifyAsubArrayReduction(
    mut iter: &metamodelica::Ref<DAE::ReductionIterator>,
    mut sub: metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = (::match_deref::match_deref! { match iter {
        Deref @ DAE::ReductionIterator { id, exp, guardExp: None, .. } => {
            let mut exp = (*exp).clone();
            exp = Expression::makeASUB(exp.clone(), list![sub])?;
            exp = replaceIteratorWithExp(exp.clone(), acc, id.clone())?;
            exp.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn simplifyAsubOperator(
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inOperator2: Operator,
    mut inOperator3: Operator,
) -> Operator {
    let mut outOperator: Operator;
    outOperator = (match &**inExp1 {
        DAE::Exp::ARRAY { .. } => inOperator3,
        DAE::Exp::MATRIX { .. } => inOperator3,
        DAE::Exp::RANGE { .. } => inOperator3,
        _ => inOperator2,
    });
    outOperator
}

fn simplifyAsubSlicing(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outAsubArray: metamodelica::Ref<DAE::Exp>;
    let mut indices: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut asubs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut didSplit: bool = false;
    let mut b: bool;
    indices = ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
        for mut e in (inSubscripts).into_iter().cloned() {
            let __x = (match () {
                () => {
                    (es, b) = Expression::splitArray((simplify1(e.clone())?).0)?;
                    didSplit = didSplit || b;
                    es.clone()
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let true = (didSplit) else {
        return Err("pattern mismatch");
    };
    for mut is in &*indices {
        for mut i in &*is.clone() {
            let () = (match &*(Expression::r#typeof(i.clone())?) {
                DAE::Type::T_INTEGER { .. } => (),
                DAE::Type::T_BOOL { .. } => (),
                DAE::Type::T_ENUMERATION { .. } => (),
                _ => return Err("fail"),
            });
        }
    }
    asubs = List::combinationMap(
        &indices,
        &({
            let __pe_b1 = inExp.clone();
            move |__pe_a0| simplifyAsubSlicing2(__pe_a0, __pe_b1.clone())
        }),
    )?;
    outAsubArray = Expression::makeScalarArray(asubs, Types::unliftArray(&(Expression::r#typeof(inExp)?))?);
    Ok(outAsubArray)
}

fn simplifyAsubSlicing2(
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outAsub: metamodelica::Ref<DAE::Exp>;
    outAsub = Expression::makeASUB(inExp, inSubscripts)?;
    Ok(outAsub)
}

fn simplifyBinaryConst(
    mut inOperator1: &Operator,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
    mut inExp3: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inOperator1, &**inExp2, &**inExp3)) {
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: metamodelica::Ref<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::ADDOP);
            val
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real;
            re3 = re1.clone() + re2.clone();
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e2_1 = intReal(ie2.clone());
            re3 = re1.clone() + e2_1;
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e1_1 = intReal(ie1.clone());
            re3 = e1_1 + re2.clone();
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
            metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str })
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: metamodelica::Ref<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::SUBOP);
            val
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real;
            re3 = re1.clone() - re2.clone();
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e2_1 = intReal(ie2.clone());
            re3 = re1.clone() - e2_1;
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e1_1 = intReal(ie1.clone());
            re3 = e1_1 - re2.clone();
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: metamodelica::Ref<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::MULOP);
            val
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real;
            re3 = re1.clone() * re2.clone();
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e2_1 = intReal(ie2.clone());
            re3 = re1.clone() * e2_1;
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e1_1 = intReal(ie1.clone());
            re3 = e1_1 * re2.clone();
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: metamodelica::Ref<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::DIVOP);
            val
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real;
            re3 = metamodelica::real_div_checked(re1.clone(), re2.clone())?;
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e2_1 = intReal(ie2.clone());
            re3 = metamodelica::real_div_checked(re1.clone(), e2_1)?;
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real;
            let mut re3: metamodelica::Real;
            e1_1 = intReal(ie1.clone());
            re3 = metamodelica::real_div_checked(e1_1, re2.clone())?;
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        (DAE::Operator::POW { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real;
            re3 = (re1.clone()).powf(re2.clone());
            metamodelica::Ref::new(DAE::Exp::RCONST { real: re3 })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyRelationConst(
    mut op: &Operator,
    mut e1: &metamodelica::Ref<DAE::Exp>,
    mut e2: &metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((op, &**e1, &**e2)) {
        (DAE::Operator::LESS { .. }, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            true
        },
        (DAE::Operator::LESS { .. }, Deref @ DAE::Exp::BCONST { bool: _ }, Deref @ DAE::Exp::BCONST { bool: _ }) => {
            false
        },
        (DAE::Operator::LESS { .. }, _, _) => {
            let mut v1: metamodelica::Real;
            let mut v2: metamodelica::Real;
            v1 = Expression::toReal(e1)?;
            v2 = Expression::toReal(e2)?;
            b = v1 < v2;
            b
        },
        (DAE::Operator::LESSEQ { .. }, Deref @ DAE::Exp::BCONST { bool: true }, Deref @ DAE::Exp::BCONST { bool: false }) => {
            false
        },
        (DAE::Operator::LESSEQ { .. }, Deref @ DAE::Exp::BCONST { bool: _ }, Deref @ DAE::Exp::BCONST { bool: _ }) => {
            true
        },
        (DAE::Operator::LESSEQ { .. }, _, _) => {
            let mut v1: metamodelica::Real;
            let mut v2: metamodelica::Real;
            v1 = Expression::toReal(e1)?;
            v2 = Expression::toReal(e2)?;
            b = v1 <= v2;
            b
        },
        (DAE::Operator::EQUAL { .. }, Deref @ DAE::Exp::BCONST { bool: b1 }, Deref @ DAE::Exp::BCONST { bool: b2 }) => {
            boolEq(b1.clone(), b2.clone())
        },
        (DAE::Operator::EQUAL { .. }, Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
            stringEqual(&s1, &s2)
        },
        (DAE::Operator::EQUAL { .. }, _, _) => {
            let mut v1: metamodelica::Real;
            let mut v2: metamodelica::Real;
            v1 = Expression::toReal(e1)?;
            v2 = Expression::toReal(e2)?;
            realEq(v1, v2)
        },
        (DAE::Operator::GREATER { .. }, _, _) => {
            !(simplifyRelationConst(&(DAE::Operator::LESSEQ { ty: DAE::T_REAL_DEFAULT().clone() }), e1, e2)?)
        },
        (DAE::Operator::GREATEREQ { .. }, _, _) => {
            !(simplifyRelationConst(&(DAE::Operator::LESS { ty: DAE::T_REAL_DEFAULT().clone() }), e1, e2)?)
        },
        (DAE::Operator::GREATER { .. }, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            !(simplifyRelationConst(&(DAE::Operator::LESSEQ { ty: DAE::T_REAL_DEFAULT().clone() }), e1, e2)?)
        },
        (DAE::Operator::GREATEREQ { .. }, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            !(simplifyRelationConst(&(DAE::Operator::LESS { ty: DAE::T_REAL_DEFAULT().clone() }), e1, e2)?)
        },
        (DAE::Operator::NEQUAL { .. }, _, _) => {
            !(simplifyRelationConst(&(DAE::Operator::EQUAL { ty: DAE::T_REAL_DEFAULT().clone() }), e1, e2)?)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(b)
}

pub(crate) fn safeIntOp(
    mut val1: i32,
    mut val2: i32,
    mut op: ExpressionSimplifyTypes::IntOp,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outv: metamodelica::Ref<DAE::Exp>;
    outv = (match op {
        ExpressionSimplifyTypes::IntOp::MULOP { .. } => {
            let mut rv1: metamodelica::Real;
            let mut rv2: metamodelica::Real;
            let mut rv3: metamodelica::Real;
            rv1 = intReal(val1);
            rv2 = intReal(val2);
            rv3 = rv1 * rv2;
            outv = Expression::realToIntIfPossible(rv3);
            outv
        }
        ExpressionSimplifyTypes::IntOp::DIVOP { .. } => {
            let mut ires: i32;
            ires = intDiv(val1, val2);
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: ires })
        }
        ExpressionSimplifyTypes::IntOp::SUBOP { .. } => {
            let mut rv1: metamodelica::Real;
            let mut rv2: metamodelica::Real;
            let mut rv3: metamodelica::Real;
            rv1 = intReal(val1);
            rv2 = intReal(val2);
            rv3 = rv1 - rv2;
            outv = Expression::realToIntIfPossible(rv3);
            outv
        }
        ExpressionSimplifyTypes::IntOp::ADDOP { .. } => {
            let mut rv1: metamodelica::Real;
            let mut rv2: metamodelica::Real;
            let mut rv3: metamodelica::Real;
            rv1 = intReal(val1);
            rv2 = intReal(val2);
            rv3 = rv1 + rv2;
            outv = Expression::realToIntIfPossible(rv3);
            outv
        }
        ExpressionSimplifyTypes::IntOp::POWOP { .. } => {
            let mut rv1: metamodelica::Real;
            let mut rv2: metamodelica::Real;
            let mut rv3: metamodelica::Real;
            rv1 = intReal(val1);
            rv2 = intReal(val2);
            rv3 = realPow(rv1, rv2);
            outv = Expression::realToIntIfPossible(rv3);
            outv
        }
    });
    outv
}

fn simplifyBinaryCommutativeWork(
    mut op: Operator,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    exp = 'mc: {
        let __mc_input = (op, lhs, rhs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: _ }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut op1: Operator;
                    op1 = DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() };
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: op1.clone(), exp2: e1.clone() });
                    e = Expression::makePureBuiltinCall(literal!("sin"), list![e.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: op1.clone(), exp2: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_4 } }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_5 } }) => {
                    if !(__rlit_4.eq(&metamodelica::OrderedFloat((2.0) as f64)) && __rlit_5.eq(&metamodelica::OrderedFloat((2.0) as f64)) && (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: tp }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(Expression::makePureBuiltinCall(literal!("sin"), list![e1.clone()], tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_6 } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_7 } } }) => {
                    if !(__rlit_6.eq(&metamodelica::OrderedFloat((2.0) as f64)) && __rlit_7.eq(&metamodelica::OrderedFloat((2.0) as f64)) && (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: tp }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(Expression::makePureBuiltinCall(literal!("sinh"), list![e1.clone()], tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: tp }, e1, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }, operator: op2, exp2: e3 }) => {
                    if !((Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2)))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op2.clone(), exp2: e3.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, e1, e2) => {
                    if !((Expression::isZero(metamodelica::AsArg::as_arg(&e1))?)) { return Err("guard") }
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::DIV { ty: tp2 }, exp2: e3 }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: e3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, _, e2) => {
                    if !((Expression::isZero(metamodelica::AsArg::as_arg(&e2))?)) { return Err("guard") }
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, e2) => {
                    if !((Expression::isConstOne(metamodelica::AsArg::as_arg(&e2)))) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty }, e1, e2) => {
                    if !((Expression::isConstMinusOne(metamodelica::AsArg::as_arg(&e2)))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::MUL { ty }, exp2: e3 }, e1) => {
                    if !((Types::isScalarReal(metamodelica::AsArg::as_arg(&ty)) && ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e1.clone())?)) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::MUL { ty }, exp2: e3 }) => {
                    if !((Types::isScalarReal(metamodelica::AsArg::as_arg(&ty)) && ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?)) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r2 }, operator: DAE::Operator::MUL { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() * r2.clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::MUL { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: r2 } }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() * r2.clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r2 }, operator: DAE::Operator::SUB { ty }, exp2: e1 @ Deref @ DAE::Exp::CREF { componentRef: _, .. } }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()) + (r2.clone()) }), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, e2) => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(Expression::makePureBuiltinCall(literal!("sign"), list![e1.clone()], ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::ADD { ty }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2 @ DAE::Operator::MUL { .. }, exp2: e3 }) => {
                    if !((!(Expression::isConstValue(metamodelica::AsArg::as_arg(&e1))?))) { return Err("guard") }
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())? {
                        exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty)), operator: op1.clone(), exp2: e2.clone() }) });
                    } else {
                        if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())? {
                            exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty)), operator: op1.clone(), exp2: e3.clone() }) });
                        } else {
                            return Err("fail");
                        }
                    }
                    Ok((exp.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, e2) => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.5_f64) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::POW { .. }, exp2: e }) => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e3, operator: op1 @ DAE::Operator::POW { ty: tp }, exp2: e4 }) => {
                    if !((ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?)) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = Expression::makeConstOne(metamodelica::AsArg::as_arg(&tp));
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: e4.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(exp)
}

fn simplifyBinary(
    mut origExp: &metamodelica::Ref<DAE::Exp>,
    mut inOperator2: Operator,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut lhsIsConstValue: bool = Expression::isConstValue(&lhs)?;
    let mut rhsIsConstValue: bool = Expression::isConstValue(&rhs)?;
    outExp = 'mc: {
        let __mc_input = (
            inOperator2.clone(),
            lhs.clone(),
            rhs.clone(),
            lhsIsConstValue,
            rhsIsConstValue,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op, e1, e2, _, _) => {
                    if !((simplifyBinaryArrayOp(metamodelica::AsArg::as_arg(&op)))) { return Err("guard") }
                    Ok(simplifyBinaryArray(e1.clone(), op.clone(), e2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op, e1, e2, _, _) => {
                    Ok(simplifyBinaryCommutativeWork(op.clone(), e1.clone(), e2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op, e1, e2, _, _) => {
                    Ok(simplifyBinaryCommutativeWork(op.clone(), e2.clone(), e1.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oper, e1, e2, true, true) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    e3 = simplifyBinaryConst(metamodelica::AsArg::as_arg(&oper), metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(e3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oper, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: op2, exp2: e4 }, _, _) => {
                    Ok(simplifyTwoBinaryExpressions(e1.clone(), op1.clone(), e2.clone(), oper.clone(), e3.clone(), op2.clone(), e4.clone(), ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?, ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e4.clone())?, ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e3.clone())?, ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e4.clone())?, Expression::isConstValue(metamodelica::AsArg::as_arg(&e1))?, Expression::isConstValue(metamodelica::AsArg::as_arg(&e2))?, Expression::isConstValue(metamodelica::AsArg::as_arg(&e3))?, Expression::operatorEqual(metamodelica::AsArg::as_arg(&op1), metamodelica::AsArg::as_arg(&op2))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oper, e1, e2, _, _) => {
                    let true = (Expression::isConstZeroLength(metamodelica::AsArg::as_arg(&e1)) || Expression::isConstZeroLength(metamodelica::AsArg::as_arg(&e2))) else { return Err("pattern mismatch") };
                    checkZeroLengthArrayOp(metamodelica::AsArg::as_arg(&oper))?;
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp2: Deref @ DAE::Exp::UNARY { exp: e3, operator: DAE::Operator::UMINUS { .. } } }, _, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: ty2.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp2: Deref @ DAE::Exp::UNARY { exp: e3, operator: DAE::Operator::UMINUS { .. } } }, _, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty2.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp2: Deref @ DAE::Exp::RCONST { real: r } }, _, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut r = (*r).clone();
                    let true = (realLt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) else { return Err("pattern mismatch") };
                    r = -(r.clone());
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: ty2.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }) }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp2: Deref @ DAE::Exp::RCONST { real: r } }, _, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut r = (*r).clone();
                    let true = (realLt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) else { return Err("pattern mismatch") };
                    r = -(r.clone());
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty2.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }) }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 }, operator: op1, exp2: e3 }, e4, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e4.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makeDiv(e3.clone(), e4.clone())?;
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: op1, exp2: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } }, e4, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e4.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makeDiv(e3.clone(), e4.clone())?;
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op1.clone(), exp2: e1.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op2 @ DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::MUL { ty: _ }, exp2: e3 } }, operator: op1, exp2: e4 }, e5, _, _) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e3), e5.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makeDiv(e4.clone(), e3.clone())?;
                    e1_1 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e2.clone() });
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut ty: Type;
                    let true = (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) else { return Err("pattern mismatch") };
                    ty = Expression::r#typeof(e1.clone())?;
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e2.clone() });
                    Ok(Expression::makePureBuiltinCall(literal!("abs"), list![res.clone()], ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, e1, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    e = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() });
                    (e, _) = simplify1(e.clone())?;
                    e3 = Expression::makePureBuiltinCall(literal!("exp"), list![e.clone()], ty.clone());
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e3.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let false = (Expression::isConstValue(metamodelica::AsArg::as_arg(&e1))? || Expression::isConstValue(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e2.clone() });
                    res = Expression::makePureBuiltinCall(literal!("exp"), list![e.clone()], ty.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op2 @ DAE::Operator::ADD { .. }, exp2: e2 }, e3, _, true) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut b2: bool;
                    (e, b) = simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    (e4, b2) = simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    let true = (b || b2) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e4.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op2 @ DAE::Operator::SUB { .. }, exp2: e2 }, e3, _, true) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut b2: bool;
                    (e, b) = simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    (e4, b2) = simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    let true = (b || b2) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e4.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2 @ DAE::Operator::DIV { .. }, exp2: e3 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e3.clone() }), operator: op2.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty: tp }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp2 }, exp2: e2 }, e3, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e3.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::MUL { ty: tp2 }, exp2: e3 }, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::MUL { ty: tp2 }, exp2: e3 }, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: e3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 }, e3, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 }, e3, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e3.clone())?) else { return Err("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, operator: DAE::Operator::MUL { .. }, exp2: e2 }, e3, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut tp2: Type;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    tp2 = Expression::r#typeof(e2.clone())?;
                    e = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e2.clone() });
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, operator: DAE::Operator::MUL { .. }, exp2: e2 }, e3, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut tp2: Type;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e3.clone())?) else { return Err("pattern mismatch") };
                    tp2 = Expression::r#typeof(e1.clone())?;
                    e = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e1.clone() });
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e3 }, _, _) => {
                    let mut tp2: Type;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e3.clone())?) else { return Err("pattern mismatch") };
                    tp2 = Expression::r#typeof(e1.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e3 }, _, _) => {
                    let mut tp2: Type;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    tp2 = Expression::r#typeof(e2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { ty }, e1, e2, true, _) => {
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&e1))?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { .. }, e1, e2, _, true) => {
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { ty }, e1, e2, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::makeConstZero(metamodelica::AsArg::as_arg(&ty)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty }, e1, e2, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (Types::isRealOrSubTypeReal(ty.clone())) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makeConstNumber(metamodelica::AsArg::as_arg(&ty), 2);
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { ty }, e1, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { ty }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }, operator: op1 @ DAE::Operator::MUL { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { ty }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }, operator: op1 @ DAE::Operator::DIV { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e1, e2, true, false) => {
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&e1))?) else { return Err("pattern mismatch") };
                    let false = (Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e1, e2, false, true) => {
                    let true = (Expression::isConstOne(metamodelica::AsArg::as_arg(&e2))) else { return Err("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, e1, e2, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isConstMinusOne(metamodelica::AsArg::as_arg(&e2))) else { return Err("pattern mismatch") };
                    e = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() });
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, e1, e2, _, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let false = (Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    res = Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty));
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty }, e1, e2, _, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let false = (Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    let true = (Types::isRealOrSubTypeReal(ty.clone())) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty: tp }, e1, Deref @ DAE::Exp::RCONST { real: r1 }, _, _) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut r: metamodelica::Real;
                    let mut r1 = (*r1).clone();
                    let true = (realAbs(r1.clone()) > metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    r = metamodelica::real_div_checked(metamodelica::OrderedFloat(1.0_f64), r1.clone())?;
                    r1 = metamodelica::OrderedFloat(1e12_f64) * r;
                    let __rlit0 = (realMod(r1.clone(), metamodelica::OrderedFloat(1.0_f64)));
                    if !(__rlit0.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("pattern mismatch") }
                    e3 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: r }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e1.clone() });
                    Ok(e3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::DIV { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r1 }, operator: DAE::Operator::MUL { ty: _ }, exp2: e3 }, _, _) => {
                    let mut r: metamodelica::Real;
                    let mut r1 = (*r1).clone();
                    let true = (realAbs(r1.clone()) > metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    r = metamodelica::real_div_checked(metamodelica::OrderedFloat(1.0_f64), r1.clone())?;
                    r1 = metamodelica::OrderedFloat(1e12_f64) * r;
                    let __rlit0 = (realMod(r1.clone(), metamodelica::OrderedFloat(1.0_f64)));
                    if !(__rlit0.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("pattern mismatch") }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: r }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e1.clone() }), operator: op2.clone(), exp2: e3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::MUL { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::SUB { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e2.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::SUB { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e2.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: op3 @ DAE::Operator::UMINUS { .. }, exp: e2 }, operator: DAE::Operator::SUB { ty }, exp2: e3 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op3.clone(), exp: e1.clone() }), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e3.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: op3 @ DAE::Operator::UMINUS { .. }, exp: e2 }, operator: DAE::Operator::SUB { ty }, exp2: e3 }, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op3.clone(), exp: e1.clone() }), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e3.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::POW { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, e2 @ Deref @ DAE::Exp::RCONST { real: __rlit_8 }, _, _) => {
                    if !(__rlit_8.eq(&metamodelica::OrderedFloat((2.0) as f64))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::DIV { ty }, e1, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }, _, _) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    e1_1 = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() });
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2 @ DAE::Operator::MUL { .. }, exp2: e3 }, e1, _, true) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isConstValue(metamodelica::AsArg::as_arg(&e3))?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e1.clone() }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2 @ DAE::Operator::MUL { .. }, exp2: e3 }, e1, _, true) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isConstValue(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e1.clone() }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, e1, e, _, true) => {
                    let true = (Expression::isConstOne(metamodelica::AsArg::as_arg(&e))) else { return Err("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { ty: tp }, e2, e, _, _) => {
                    let mut one: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isConstMinusOne(metamodelica::AsArg::as_arg(&e))) else { return Err("pattern mismatch") };
                    one = Expression::makeConstOne(metamodelica::AsArg::as_arg(&tp));
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: one.clone(), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, e1, e, _, true) => {
                    let mut tp: Type;
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    tp = Expression::r#typeof(e1.clone())?;
                    Ok(Expression::makeConstOne(&tp))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_9 }, _, _) => {
                    if !(__rlit_9.eq(&metamodelica::OrderedFloat((2.0) as f64))) { return Err("guard") }
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oper @ DAE::Operator::POW { .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, e, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: oper.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall(literal!("sqrt"), list![e1.clone()], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::POW { ty }, exp2: e2 }, e3, _, _) => {
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    e4 = Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty));
                    e4 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e4.clone() });
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e4.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::POW { ty }, exp2: e2 }, operator: op2 @ DAE::Operator::MUL { ty: _ }, exp2: e5 }, e3, _, _) => {
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    e4 = Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty));
                    e4 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e4.clone() });
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e4.clone() }), operator: op2.clone(), exp2: e5.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, e3, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::POW { ty }, exp2: e2 }, _, _) => {
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    e4 = Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty));
                    e4 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e2.clone() });
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e4.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::POW { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::DIV { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: r }, _, _) => {
                    let mut r = (*r).clone();
                    let true = (realLt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) else { return Err("pattern mismatch") };
                    r = -(r.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e1.clone() }), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, e1, _, true, _) => {
                    let true = (Expression::isConstOne(metamodelica::AsArg::as_arg(&e1))) else { return Err("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1, Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, Deref @ DAE::Exp::IFEXP { expCond: e4, expThen: e5, expElse: e6 }, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e4.clone())?) else { return Err("pattern mismatch") };
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e5.clone() });
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e6.clone() });
                    Ok(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e.clone(), expElse: res.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { ty }, Deref @ DAE::Exp::BINARY { exp1: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, operator: op2 @ DAE::Operator::MUL { .. }, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::MUL { .. }, exp2: e4 }, false, false) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e4.clone() }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { ty }, Deref @ DAE::Exp::BINARY { exp1: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, operator: DAE::Operator::DIV { .. }, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::DIV { .. }, exp2: e4 }, false, false) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e3.clone())?) else { return Err("pattern mismatch") };
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: Expression::inverseFactors(e2.clone())?, operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: Expression::inverseFactors(e4.clone())? }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1, Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2, exp2: e3 } }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e5, operator: op3, exp2: e6 } }, false, false) => {
                    let true = (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) else { return Err("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) else { return Err("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op3))) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e5.clone())?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: op3.clone(), exp2: e6.clone() }) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1, Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e5, operator: op3, exp2: e6 } }, false, false) => {
                    let true = (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) else { return Err("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op3))) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e5.clone())?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: op3.clone(), exp2: e6.clone() }) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1, Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2, exp2: e3 } }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: e5 }, false, false) => {
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    let true = (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) else { return Err("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) else { return Err("pattern mismatch") };
                    if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e5.clone())? {
                        outExp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e4.clone() }) });
                    } else {
                        if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e4.clone())? {
                            outExp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: oper.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e5.clone() }) });
                        } else {
                            return Err("fail");
                        }
                    }
                    Ok((outExp.clone(), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: e2 }, operator: op2, exp2: e3 }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: e5 }, false, false) => {
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    let true = (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) else { return Err("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) else { return Err("pattern mismatch") };
                    if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e5.clone())? {
                        outExp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e4.clone() }) });
                    } else {
                        if ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e4.clone())? {
                            outExp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: oper.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e5.clone() }) });
                        } else {
                            return Err("fail");
                        }
                    }
                    Ok((outExp.clone(), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, e1, e2 @ Deref @ DAE::Exp::RCONST { real: r }, _, true) => {
                    if !((r.clone() != intReal(((r.clone()).0.floor() as i32)))) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut exp_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_lst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::factors(metamodelica::AsArg::as_arg(&e1))?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp_lst = metamodelica::Own::own(__pa0);
                    let true = (List::any(&exp_lst, &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isEvaluatedConst(&__a0)) })?) else { return Err("pattern mismatch") };
                    (exp_lst, exp_lst_1) = List::splitOnTrue(&exp_lst, &Expression::isPositiveOrZero)?;
                    exp_lst = simplifyBinaryDistributePow(exp_lst.clone(), e2.clone())?;
                    e = Expression::makeProductLst(exp_lst_1.clone())?;
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: inOperator2.clone(), exp2: e2.clone() });
                    outExp = Expression::makeProductLst(metamodelica::cons(e.clone(), exp_lst.clone()))?;
                    Ok((outExp.clone(), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, e1, e2, _, true) => {
                    if !((Expression::isEvaluatedConst(metamodelica::AsArg::as_arg(&e2)))) { return Err("guard") }
                    let mut exp_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_lst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::factors(metamodelica::AsArg::as_arg(&e1))?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp_lst = metamodelica::Own::own(__pa0);
                    let true = (List::any(&exp_lst, &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isEvaluatedConst(&__a0)) })?) else { return Err("pattern mismatch") };
                    exp_lst_1 = simplifyBinaryDistributePow(exp_lst.clone(), e2.clone())?;
                    Ok(Expression::makeProductLst(exp_lst_1.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, e3, _, _) => {
                    if !((Expression::isEven(metamodelica::AsArg::as_arg(&e2)))) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    if Expression::isEvaluatedConst(metamodelica::AsArg::as_arg(&e3)) {
                        e = simplifyBinaryConst(&(DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }), metamodelica::AsArg::as_arg(&e2), metamodelica::AsArg::as_arg(&e3))?;
                        let false = (Expression::isEven(&e)) else { return Err("pattern mismatch") };
                    } else {
                        e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e3.clone() });
                    }
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: Expression::makePureBuiltinCall(literal!("abs"), list![e1.clone()], Expression::r#typeof(e1.clone())?), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, e3, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e3.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall(literal!("tan"), list![e1.clone()], ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    e3 = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) });
                    e4 = Expression::makePureBuiltinCall(literal!("cos"), list![e2.clone()], ty.clone());
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op2.clone(), exp2: e4.clone() });
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makePureBuiltinCall(literal!("cos"), list![e2.clone()], ty.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::DIV { ty }, e1, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    e3 = Expression::makePureBuiltinCall(literal!("sin"), list![e2.clone()], ty.clone());
                    e4 = Expression::makePureBuiltinCall(literal!("cos"), list![e2.clone()], ty.clone());
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: op2.clone(), exp2: e3.clone() });
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall(literal!("tanh"), list![e1.clone()], ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    e3 = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) });
                    e4 = Expression::makePureBuiltinCall(literal!("cosh"), list![e2.clone()], ty.clone());
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op2.clone(), exp2: e4.clone() });
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    e = Expression::makePureBuiltinCall(literal!("cosh"), list![e2.clone()], ty.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty }, e1, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e2 }, _, _) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    e1_1 = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() });
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RANGE { ty, start: e1, step: oexp, stop: e2 }, _, _, _) => {
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    e1 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: inOperator2.clone(), exp2: rhs.clone() })), inOperator2.clone(), e1.clone(), rhs.clone())?;
                    e2 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: inOperator2.clone(), exp2: rhs.clone() })), inOperator2.clone(), e2.clone(), rhs.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, _, Deref @ DAE::Exp::RANGE { ty, start: e1, step: oexp, stop: e2 }, _, _) => {
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    e1 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() })), inOperator2.clone(), lhs.clone(), e1.clone())?;
                    e2 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() })), inOperator2.clone(), lhs.clone(), e2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::RANGE { ty, start: e1, step: oexp, stop: e2 }, _, _, _) => {
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    e1 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: inOperator2.clone(), exp2: rhs.clone() })), inOperator2.clone(), e1.clone(), rhs.clone())?;
                    e2 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: inOperator2.clone(), exp2: rhs.clone() })), inOperator2.clone(), e2.clone(), rhs.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { .. }, _, Deref @ DAE::Exp::RANGE { ty, start: e1, step: oexp, stop: e2 }, _, _) => {
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    e1 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() })), inOperator2.clone(), lhs.clone(), e1.clone())?;
                    e2 = simplifyBinary(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() })), inOperator2.clone(), lhs.clone(), e2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn simplifyTwoBinaryExpressions(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut lhsOperator: Operator,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut mainOperator: Operator,
    mut e3: metamodelica::Ref<DAE::Exp>,
    mut rhsOperator: Operator,
    mut e4: metamodelica::Ref<DAE::Exp>,
    mut expEqual_e1_e3: bool,
    mut expEqual_e1_e4: bool,
    mut expEqual_e2_e3: bool,
    mut expEqual_e2_e4: bool,
    mut isConst_e1: bool,
    mut isConst_e2: bool,
    mut isConst_e3: bool,
    mut operatorEqualLhsRhs: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), lhsOperator.clone(), e2.clone(), mainOperator.clone(), e3.clone(), rhsOperator, e4.clone(), expEqual_e1_e3, expEqual_e1_e4, expEqual_e2_e3, expEqual_e2_e4, isConst_e1, isConst_e2, operatorEqualLhsRhs)) {
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, true, _, _, _, _, _, _) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2, operator: op1.clone(), exp2: e4 }) })
        },
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, _, true, _, _, _, _, _) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2, operator: op1.clone(), exp2: e3 }) })
        },
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, _, _, true, _, _, _, _) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2, operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: op1.clone(), exp2: e4 }) })
        },
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, _, _, _, true, _, _, _) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2, operator: op2.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: op1.clone(), exp2: e3 }) })
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::MUL { .. }, _, DAE::Operator::POW { .. }, _, _, _, _, true, _, _, _) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: mainOperator, exp2: e3 }), operator: lhsOperator, exp2: e2 });
            res
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::DIV { .. }, _, DAE::Operator::POW { .. }, _, _, _, _, true, _, _, _) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: mainOperator, exp2: e3 }), operator: lhsOperator, exp2: e2 });
            res
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::MUL { .. }, _, DAE::Operator::POW { .. }, _, true, _, _, _, _, _, _) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = Expression::expAdd(e2, e4)?;
            res = Expression::expPow(e1, res)?;
            res
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::DIV { .. }, _, DAE::Operator::POW { .. }, _, true, _, _, _, _, _, _) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = Expression::expSub(e2, e4)?;
            res = Expression::expPow(e1, res)?;
            res
        },
        (_, op2, _, op1, _, _, _, _, _, _, true, _, false, true) if (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1)) && Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: op1.clone(), exp2: e3 }), operator: op2.clone(), exp2: e4 })
        },
        (_, op @ DAE::Operator::MUL { ty }, _, op1, _, DAE::Operator::DIV { ty: _ }, _, true, _, _, _, false, _, _) if (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut one: metamodelica::Ref<DAE::Exp>;
            one = Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty));
            e = Expression::makeDiv(one, e4)?;
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2, operator: op1.clone(), exp2: e }), operator: op.clone(), exp2: e1 })
        },
        (_, DAE::Operator::DIV { ty }, _, op1, _, DAE::Operator::MUL { ty: _ }, _, true, _, _, _, false, _, _) if (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1))) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut one: metamodelica::Ref<DAE::Exp>;
            one = Expression::makeConstOne(metamodelica::AsArg::as_arg(&ty));
            e = Expression::makeDiv(one, e2)?;
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e, operator: op1.clone(), exp2: e4 }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e1 })
        },
        (e1_1, op2, e_3, op1, e, _, _, _, _, _, true, _, false, true) if (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1)) && Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e.clone() });
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res, operator: op2.clone(), exp2: e_3.clone() })
        },
        (Deref @ DAE::Exp::BINARY { exp1: e_1, operator: op2, exp2: e_2 }, op @ DAE::Operator::MUL { ty: _ }, e_3, op1, e, op3, e_6, _, _, _, _, _, _, _) if (!(Expression::isConstValue(metamodelica::AsArg::as_arg(&e_2))?) && ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e_2), e_6.clone())? && Expression::operatorEqual(metamodelica::AsArg::as_arg(&op2), metamodelica::AsArg::as_arg(&op3))? && Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1)) && Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::Ref<DAE::Exp>;
            e1_1 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_3.clone() });
            res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1, operator: op1.clone(), exp2: e.clone() });
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res, operator: op2.clone(), exp2: e_2.clone() })
        },
        (Deref @ DAE::Exp::BINARY { exp1: e_1, operator: op2, exp2: e_2 }, op @ DAE::Operator::MUL { ty: _ }, e_3, op1, Deref @ DAE::Exp::BINARY { exp1: e_4, operator: op3, exp2: e_5 }, DAE::Operator::MUL { ty: _ }, e_6, _, _, _, _, _, _, _) if (!(Expression::isConstValue(metamodelica::AsArg::as_arg(&e_2))?) && ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e_2), e_5.clone())? && Expression::operatorEqual(metamodelica::AsArg::as_arg(&op2), metamodelica::AsArg::as_arg(&op3))? && Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1)) && Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::Ref<DAE::Exp>;
            e1_1 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_3.clone() });
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e_4.clone(), operator: op.clone(), exp2: e_6.clone() });
            res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1, operator: op1.clone(), exp2: e });
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res, operator: op2.clone(), exp2: e_2.clone() })
        },
        (e_1, op2, e_3, op1, Deref @ DAE::Exp::BINARY { exp1: e_4, operator: op3, exp2: e_5 }, op @ DAE::Operator::MUL { ty: _ }, e_6, _, _, _, _, _, false, _) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e_3), e_5.clone())? && Expression::operatorEqual(metamodelica::AsArg::as_arg(&op2), metamodelica::AsArg::as_arg(&op3))? && Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1)) && Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op2))) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::Ref<DAE::Exp>;
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e_4.clone(), operator: op.clone(), exp2: e_6.clone() });
            res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op1.clone(), exp2: e });
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res, operator: op2.clone(), exp2: e_3.clone() })
        },
        (_, DAE::Operator::MUL { .. }, _, DAE::Operator::SUB { .. }, _, DAE::Operator::MUL { .. }, _, true, _, _, _, _, _, _) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: lhsOperator, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2, operator: mainOperator, exp2: e4 }) })
        },
        (_, DAE::Operator::MUL { .. }, _, DAE::Operator::SUB { .. }, _, DAE::Operator::MUL { .. }, _, _, true, _, _, _, _, _) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: lhsOperator, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2, operator: mainOperator, exp2: e3 }) })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyLBinary(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut inOperator2: &Operator,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut inExp4: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inOperator2, inExp3, inExp4)) {
        (_, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, _) => {
            origExp
        },
        (_, _, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }) => {
            origExp
        },
        (DAE::Operator::AND { ty: Deref @ DAE::Type::T_BOOL { .. } }, e1, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 }) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
        },
        (DAE::Operator::AND { ty: Deref @ DAE::Type::T_BOOL { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 }, e2) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
        },
        (DAE::Operator::OR { ty: Deref @ DAE::Type::T_BOOL { .. } }, e1, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 }) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })
        },
        (DAE::Operator::OR { ty: Deref @ DAE::Type::T_BOOL { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 }, e2) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })
        },
        (DAE::Operator::AND { ty: _ }, e1 @ Deref @ DAE::Exp::BCONST { bool: b }, e2) => {
            if (b.clone()) {e2.clone()} else {e1.clone()}
        },
        (DAE::Operator::AND { ty: _ }, e1, e2 @ Deref @ DAE::Exp::BCONST { bool: b }) => {
            if (b.clone()) {e1.clone()} else {e2.clone()}
        },
        (DAE::Operator::OR { ty: _ }, e1 @ Deref @ DAE::Exp::BCONST { bool: b }, e2) => {
            if (b.clone()) {e1.clone()} else {e2.clone()}
        },
        (DAE::Operator::OR { ty: _ }, e1, e2 @ Deref @ DAE::Exp::BCONST { bool: b }) => {
            if (b.clone()) {e2.clone()} else {e1.clone()}
        },
        (DAE::Operator::AND { ty: _ }, e1, e2) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            e1.clone()
        },
        (DAE::Operator::OR { ty: _ }, e1, e2) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            e1.clone()
        },
        _ => {
            origExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn simplifyRelation(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut inOperator2: Operator,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut inExp4: metamodelica::Ref<DAE::Exp>,
    mut index: i32,
    mut optionExpisASUB: Option<(metamodelica::Ref<DAE::Exp>, i32, i32)>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inOperator2.clone(), inExp3.clone(), inExp4.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oper, e1, e2) => {
                    let mut b: bool;
                    let true = (Expression::isConstValue(metamodelica::AsArg::as_arg(&e1))?) else { return Err("pattern mismatch") };
                    let true = (Expression::isConstValue(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    b = simplifyRelationConst(metamodelica::AsArg::as_arg(&oper), metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&e2))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::BCONST { bool: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::EQUAL { ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr1, ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: _ }) => {
                    let true = (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::NEQUAL { ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr1, ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: _ }) => {
                    let true = (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::GREATEREQ { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), &inOperator2, inExp3.clone(), inExp4.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::GREATER { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), &inOperator2, inExp3.clone(), inExp4.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::LESSEQ { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), &inOperator2, inExp4.clone(), inExp3.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::LESS { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), &inOperator2, inExp4.clone(), inExp3.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn simplifyRelation2(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut inOp: &Operator,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    (oExp, _) = simplify(Expression::expSub(lhs, rhs)?)?;
    if Expression::isGreatereqOrLesseq(inOp) {
        oExp = if (Expression::isPositiveOrZero(oExp)?) {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })
        } else {
            origExp
        };
    } else {
        oExp = if (Expression::isNegativeOrZero(oExp)?) {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
        } else {
            origExp
        };
    }
    Ok(oExp)
}

fn simplifyBinaryDistributePow(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (inExpLst).into_iter().cloned() {
            if !(!(Expression::isConstOne(&(e.clone())))) {
                continue;
            }
            let __x = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: e.clone(),
                operator: DAE::Operator::POW {
                    ty: Expression::r#typeof(e.clone())?,
                },
                exp2: inExp.clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpLst)
}

fn simplifyUnary(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut inOperator2: Operator,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inOperator2, inExp3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::NOT { .. }, e1) => {
                    let mut b1: bool;
                    b1 = Expression::toBool(metamodelica::AsArg::as_arg(&e1))?;
                    b1 = !(b1);
                    Ok(metamodelica::Ref::new(DAE::Exp::BCONST { bool: b1 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::NOT { ty: _ }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 }) => {
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut i_1: i32;
                    i_1 = intNeg(i.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i_1 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::RCONST { real: r }) => {
                    let mut r_1: metamodelica::Real;
                    r_1 = -(r.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r_1 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::MUL { .. }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::MUL_ARR { .. }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, e1) => {
                    if !((Expression::isZero(metamodelica::AsArg::as_arg(&e1))?)) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS_ARR { .. }, e1) => {
                    if !((Expression::isZero(metamodelica::AsArg::as_arg(&e1))?)) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::SUB { .. }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::SUB_ARR { .. }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::ADD { .. }, exp2: e2 }) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e2.clone() }) }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    Ok(e_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::ADD_ARR { .. }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e2.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV { .. }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op2 @ DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV_ARR { .. }, exp2: e2 }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }) => {
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e1 }) => {
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { exp: e1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Nil } } }, attr }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e1.clone(), e3.clone(), e2.clone()], attr: attr.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::ARRAY { ty: ty1, scalar: b1, array: expl }) => {
                    let mut expl = (*expl).clone();
                    expl = List::map(expl.clone(), &Expression::negate)?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty1.clone(), scalar: b1.clone(), array: expl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::MATRIX { ty: ty1, integer: i, matrix: mat }) => {
                    let mut mat = (*mat).clone();
                    mat = List::mapList(mat.clone(), &Expression::negate)?;
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: ty1.clone(), integer: i.clone(), matrix: mat.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn simplifyVectorScalarMatrix(
    mut imexpl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut op: Operator,
    mut s1: metamodelica::Ref<DAE::Exp>,
    mut arrayScalar: bool,
) -> metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    outExp = if (arrayScalar) {
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
            for mut row in (imexpl).into_iter().cloned() {
                let __x = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                    for mut e in (row.clone()).into_iter().cloned() {
                        let __x = metamodelica::Ref::new(DAE::Exp::BINARY {
                            exp1: e.clone(),
                            operator: op.clone(),
                            exp2: s1.clone(),
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    } else {
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
            for mut row in (imexpl).into_iter().cloned() {
                let __x = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                    for mut e in (row.clone()).into_iter().cloned() {
                        let __x = metamodelica::Ref::new(DAE::Exp::BINARY {
                            exp1: s1.clone(),
                            operator: op.clone(),
                            exp2: e.clone(),
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    };
    outExp
}

fn simplifyBinarySortConstantsMul(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut e_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut const_es1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut notconst_es1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut res1: metamodelica::Ref<DAE::Exp>;
    let mut res2: metamodelica::Ref<DAE::Exp>;
    e_lst = Expression::factors(&inExp)?;
    (const_es1, notconst_es1) = List::splitOnTrue(&e_lst, &Expression::isConst)?;
    if !((const_es1).is_empty()) {
        res1 = simplifyBinaryMulConstants(&const_es1)?;
        (res1, _) = simplify1(res1)?;
        res2 = Expression::makeProductLst(notconst_es1)?;
        outExp = Expression::expMul(res1, res2)?;
    } else {
        outExp = inExp;
    }
    Ok(outExp)
}

fn simplifyBuiltinConstantDer(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::RCONST { real: _ } => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ DAE::Exp::ICONST { integer: _ } => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { .. }, dims }, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            (e, _) = Expression::makeZeroExpression(metamodelica::AsArg::as_arg(&dims))?;
            e
        },
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { .. }, dims }, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            (e, _) = Expression::makeZeroExpression(metamodelica::AsArg::as_arg(&dims))?;
            e
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn removeOperatorDimension(mut inop: &Operator) -> Result<Operator> {
    let mut outop: Operator;
    outop = (match inop.clone() {
        DAE::Operator::ADD_ARR { ty: ref ty1 } => {
            let mut ty2: Type;
            let mut b: bool;
            let mut op: Operator;
            ty2 = Expression::unliftArray(metamodelica::AsArg::as_arg(&ty1))?;
            b = DAEUtil::expTypeArray(&ty2);
            op = if (b) {
                DAE::Operator::ADD_ARR { ty: ty2 }
            } else {
                DAE::Operator::ADD { ty: ty2 }
            };
            op
        }
        DAE::Operator::SUB_ARR { ty: ref ty1 } => {
            let mut ty2: Type;
            let mut b: bool;
            let mut op: Operator;
            ty2 = Expression::unliftArray(metamodelica::AsArg::as_arg(&ty1))?;
            b = DAEUtil::expTypeArray(&ty2);
            op = if (b) {
                DAE::Operator::SUB_ARR { ty: ty2 }
            } else {
                DAE::Operator::SUB { ty: ty2 }
            };
            op
        }
        DAE::Operator::DIV_ARR { ty: ref ty1 } => {
            let mut ty2: Type;
            let mut b: bool;
            let mut op: Operator;
            ty2 = Expression::unliftArray(metamodelica::AsArg::as_arg(&ty1))?;
            b = DAEUtil::expTypeArray(&ty2);
            op = if (b) {
                DAE::Operator::DIV_ARR { ty: ty2 }
            } else {
                DAE::Operator::DIV { ty: ty2 }
            };
            op
        }
        DAE::Operator::MUL_ARR { ty: ref ty1 } => {
            let mut ty2: Type;
            let mut b: bool;
            let mut op: Operator;
            ty2 = Expression::unliftArray(metamodelica::AsArg::as_arg(&ty1))?;
            b = DAEUtil::expTypeArray(&ty2);
            op = if (b) {
                DAE::Operator::MUL_ARR { ty: ty2 }
            } else {
                DAE::Operator::MUL { ty: ty2 }
            };
            op
        }
        DAE::Operator::POW_ARR2 { ty: ref ty1 } => {
            let mut ty2: Type;
            let mut b: bool;
            let mut op: Operator;
            ty2 = Expression::unliftArray(metamodelica::AsArg::as_arg(&ty1))?;
            b = DAEUtil::expTypeArray(&ty2);
            op = if (b) {
                DAE::Operator::POW_ARR2 { ty: ty2 }
            } else {
                DAE::Operator::POW { ty: ty2 }
            };
            op
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outop)
}

pub fn simplifyRangeBool(mut inStart: bool, mut inStop: bool) -> metamodelica::List<bool> {
    let mut outRange: metamodelica::List<bool>;
    outRange = if (inStart) {
        if (inStop) { list![true] } else { metamodelica::nil() }
    } else {
        if (inStop) { list![false, true] } else { list![false] }
    };
    outRange
}

pub fn simplifyRange(mut inStart: i32, mut inStep: i32, mut inStop: i32) -> Result<metamodelica::List<i32>> {
    let mut outValues: metamodelica::List<i32>;
    outValues = List::intRange3(inStart, inStep, inStop)?;
    Ok(outValues)
}

pub fn simplifyRangeReal(
    mut inStart: metamodelica::Real,
    mut inStep: metamodelica::Real,
    mut inStop: metamodelica::Real,
) -> Result<metamodelica::List<metamodelica::Real>> {
    let mut outValues: metamodelica::List<metamodelica::Real>;
    outValues = 'mc: {
        let __mc_input = inStop;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut error_str: ArcStr;
            let true = (realAbs(inStep) <= metamodelica::OrderedFloat(1e-14_f64)) else {
                return Err("pattern mismatch");
            };
            error_str = stringDelimitList(
                List::map(list![inStart, inStep, inStop], &fnptr!(realString, metamodelica::Real))?,
                literal!(":"),
            );
            Error::addMessage(Error::ZERO_STEP_IN_ARRAY_CONSTRUCTOR.clone(), list![error_str.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if !(inStart == inStop) {
                return Err("guard");
            }
            Ok(list![inStart])
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut steps: i32;
            steps = Util::realRangeSize(inStart, inStep, inStop)? - 1;
            Ok(simplifyRangeReal2(inStart, inStep, steps, metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValues)
}

fn simplifyRangeReal2(
    mut inStart: metamodelica::Real,
    mut inStep: metamodelica::Real,
    mut inSteps: i32,
    mut inValues: metamodelica::List<metamodelica::Real>,
) -> metamodelica::List<metamodelica::Real> {
    '__tco: loop {
        match inSteps {
            (-1) => return inValues,
            _ => {
                let mut next: metamodelica::Real;
                let mut vals: metamodelica::List<metamodelica::Real>;
                next = inStart + inStep * intReal(inSteps);
                vals = metamodelica::cons(next, inValues);
                {
                    (inStart, inStep, inSteps, inValues) = (inStart, inStep, inSteps - 1, vals);
                    continue '__tco;
                }
            }
        }
    }
}

fn simplifyReduction(mut inReduction: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outValue: metamodelica::Ref<DAE::Exp>;
    outValue = 'mc: {
        let __mc_input = &*inReduction;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators, reductionInfo: Deref @ DAE::ReductionInfo { defaultValue: Some(v), .. }, .. } => {
                    let mut expr: metamodelica::Ref<DAE::Exp>;
                    let true = (hasZeroLengthIterator(metamodelica::AsArg::as_arg(&iterators))) else { return Err("pattern mismatch") };
                    expr = ValuesUtil::valueExp(v.clone(), None)?;
                    Ok(expr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators, .. } => {
                    let mut expr: metamodelica::Ref<DAE::Exp>;
                    let true = (hasZeroLengthIterator(metamodelica::AsArg::as_arg(&iterators))) else { return Err("pattern mismatch") };
                    expr = ValuesUtil::valueExp(openmodelica_frontend_types::Values::Value::interned_META_FAIL(), None)?;
                    Ok(expr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path, foldName, resultName, foldExp, exprType: ty, defaultValue, .. }, expr, iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id: iter_name, guardExp: None, exp: range, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut values: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ety: metamodelica::Ref<DAE::Type>;
                    let mut expr = (*expr).clone();
                    values = Expression::getArrayOrRangeContents(range.clone())?;
                    ety = Types::simplifyType(ty.clone())?;
                    values = replaceIteratorWithValues(values.clone(), expr.clone(), iter_name.clone())?;
                    expr = simplifyReductionFoldPhase(metamodelica::AsArg::as_arg(&path), foldExp.clone(), metamodelica::AsArg::as_arg(&foldName), metamodelica::AsArg::as_arg(&resultName), ety.clone(), values.clone(), defaultValue.clone())?;
                    Ok(expr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path, iterType: Absyn::ReductionIterType::THREAD { .. }, foldName, resultName, exprType: ty, foldExp, defaultValue }, expr, iterators } => {
                    let mut range: metamodelica::Ref<DAE::Exp>;
                    let mut iter_name: ArcStr;
                    let mut values: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ety: metamodelica::Ref<DAE::Type>;
                    let mut expr = (*expr).clone();
                    let mut iterators = (*iterators).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(iterators.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id: __pa0, guardExp: None, exp: __pa1, .. }, tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    iter_name = metamodelica::Own::own(__pa0);
                    range = metamodelica::Own::own(__pa1);
                    iterators = metamodelica::Own::own(__pa2);
                    values = Expression::getArrayOrRangeContents(range.clone())?;
                    ety = Types::simplifyType(ty.clone())?;
                    values = List::map2(values.clone(), &replaceIteratorWithExp, expr.clone(), iter_name.clone())?;
                    values = List::fold(metamodelica::AsArg::as_arg(&iterators), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>, __a1: metamodelica::List<metamodelica::Ref<DAE::Exp>>| getIteratorValues(&__a0, __a1), values.clone())?;
                    expr = simplifyReductionFoldPhase(metamodelica::AsArg::as_arg(&path), foldExp.clone(), metamodelica::AsArg::as_arg(&foldName), metamodelica::AsArg::as_arg(&resultName), ety.clone(), values.clone(), defaultValue.clone())?;
                    Ok(expr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, iterType: Absyn::ReductionIterType::COMBINE { .. }, foldName, resultName, exprType: ty, .. }, expr, iterators: Deref @ metamodelica::ListNode::Cons { head: iter, tail: iterators @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } } => {
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut foldName2: ArcStr;
                    let mut resultName2: ArcStr;
                    let mut expr = (*expr).clone();
                    foldName2 = Util::getTempVariableIndex();
                    resultName2 = Util::getTempVariableIndex();
                    ty1 = Expression::unliftArray(metamodelica::AsArg::as_arg(&ty))?;
                    expr = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty1.clone(), defaultValue: None, foldName: foldName2.clone(), resultName: resultName2.clone(), foldExp: None }), expr: expr.clone(), iterators: list![iter.clone()] });
                    expr = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: None, foldName: foldName.clone(), resultName: resultName.clone(), foldExp: None }), expr: expr.clone(), iterators: iterators.clone() });
                    Ok(expr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path, iterType: Absyn::ReductionIterType::COMBINE { .. }, foldName, resultName, exprType: ty, foldExp: None, defaultValue }, expr, iterators: Deref @ metamodelica::ListNode::Cons { head: iter, tail: iterators @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } } => {
                    let mut foldName2: ArcStr;
                    let mut resultName2: ArcStr;
                    let mut expr = (*expr).clone();
                    foldName2 = Util::getTempVariableIndex();
                    resultName2 = Util::getTempVariableIndex();
                    expr = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: foldName2.clone(), resultName: resultName2.clone(), foldExp: None }), expr: expr.clone(), iterators: list![iter.clone()] });
                    expr = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: foldName.clone(), resultName: resultName.clone(), foldExp: None }), expr: expr.clone(), iterators: iterators.clone() });
                    Ok(expr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path, iterType: Absyn::ReductionIterType::COMBINE { .. }, foldName, resultName, exprType: ty, foldExp: Some(foldExpr), defaultValue }, expr, iterators: Deref @ metamodelica::ListNode::Cons { head: iter, tail: iterators @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } } => {
                    let mut foldExpr2: metamodelica::Ref<DAE::Exp>;
                    let mut foldName2: ArcStr;
                    let mut resultName2: ArcStr;
                    let mut expr = (*expr).clone();
                    foldName2 = Util::getTempVariableIndex();
                    resultName2 = Util::getTempVariableIndex();
                    (foldExpr2, _) = Expression::traverseExpBottomUp(foldExpr.clone(), &fnptr!(Expression::renameExpCrefIdent, metamodelica::Ref<DAE::Exp>, (ArcStr, ArcStr)), (foldName.clone(), foldName2.clone()))?;
                    (foldExpr2, _) = Expression::traverseExpBottomUp(foldExpr2.clone(), &fnptr!(Expression::renameExpCrefIdent, metamodelica::Ref<DAE::Exp>, (ArcStr, ArcStr)), (resultName.clone(), resultName2.clone()))?;
                    expr = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: foldName2.clone(), resultName: resultName2.clone(), foldExp: Some(foldExpr2.clone()) }), expr: expr.clone(), iterators: list![iter.clone()] });
                    expr = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: foldName.clone(), resultName: resultName.clone(), foldExp: Some(foldExpr.clone()) }), expr: expr.clone(), iterators: iterators.clone() });
                    Ok(expr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inReduction.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outValue
}

fn getIteratorValues(
    mut iter: &metamodelica::Ref<DAE::ReductionIterator>,
    mut inValues: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut values: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut iter_name: ArcStr;
    let mut range: metamodelica::Ref<DAE::Exp>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*iter)) {
        Deref @ DAE::ReductionIterator { id: __pa0, guardExp: None, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iter_name = metamodelica::Own::own(__pa0);
    range = metamodelica::Own::own(__pa1);
    values = Expression::getArrayOrRangeContents(range)?;
    values = List::threadMap1(values, inValues, &replaceIteratorWithExp, iter_name)?;
    Ok(values)
}

fn replaceIteratorWithExp(
    mut iterExp: metamodelica::Ref<DAE::Exp>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut name: ArcStr,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(exp, &fnptr!(replaceIteratorWithExpTraverser, metamodelica::Ref<DAE::Exp>, (ArcStr, metamodelica::Ref<DAE::Exp>, bool)), (name, iterExp, true))?) {
        (__pa0, (_, _, true)) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outExp = metamodelica::Own::own(__pa0);
    Ok(outExp)
}

fn replaceIteratorWithExpTraverser(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (ArcStr, metamodelica::Ref<DAE::Exp>, bool),
) -> (metamodelica::Ref<DAE::Exp>, (ArcStr, metamodelica::Ref<DAE::Exp>, bool)) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (ArcStr, metamodelica::Ref<DAE::Exp>, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, _, false)) => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: _, subscriptLst: Deref @ metamodelica::ListNode::Nil }, ty: _ }, tpl @ (name, iterExp, _)) => {
                    if !((stringEq(&name, &id))) { return Err("guard") }
                    Ok((iterExp.clone(), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }, .. }, (name, iterExp, _)) => {
                    if !((stringEq(&name, &id))) { return Err("guard") }
                    Ok((exp.clone(), (name.clone(), iterExp.clone(), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty1, subscriptLst: ss, componentRef: cr }, ty }, tpl @ (name, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: replName, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, .. }, _)) => {
                    if !((stringEq(&name, &id))) { return Err("guard") }
                    Ok((metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: replName.clone(), identType: ty1.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }), ty: ty.clone() }), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty1, subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: cr }, ty }, tpl @ (name, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: replName, subscriptLst: ss, .. }, .. }, _)) => {
                    if !((stringEq(&name, &id))) { return Err("guard") }
                    Ok((metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: replName.clone(), identType: ty1.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }), ty: ty.clone() }), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: _, subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id2, identType: _, subscriptLst: Deref @ metamodelica::ListNode::Nil } }, .. }, tpl @ (name, Deref @ DAE::Exp::CALL { expLst: exps, path: callPath, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: recordPath }, .. }, .. } }, _)) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut i: i32;
                    let true = (stringEq(&name, &id)) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&callPath), metamodelica::AsArg::as_arg(&recordPath))) else { return Err("pattern mismatch") };
                    let true = (((varLst).len() as i32) == ((exps).len() as i32)) else { return Err("pattern mismatch") };
                    i = List::position1OnTrue(metamodelica::AsArg::as_arg(&varLst), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::typeVarIdentEqual(&__a0, &__a1)) }, id2.clone())?;
                    exp = (exps).get(i)?;
                    Ok((exp.clone(), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, .. }, .. }, (name, iterExp, _)) => {
                    if !((stringEq(&name, &id))) { return Err("guard") }
                    Ok((exp.clone(), (name.clone(), iterExp.clone(), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn replaceIteratorWithValues(
    mut values: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut name: ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut body: metamodelica::Ref<DAE::Exp> = exp.clone();
    let mut sites: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut sitesArr: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    if List::all(
        &values,
        &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isIntegerConstant(&__a0))
        },
    )? {
        let (__pa0, (_, __pa1)) = Expression::traverseExpBottomUp(
            exp.clone(),
            &extractIteratorSubscript,
            (name.clone(), metamodelica::nil()),
        )?;
        body = metamodelica::Own::own(__pa0);
        sites = metamodelica::Own::own(__pa1);
    }
    if (sites).is_empty() {
        exps = List::map2(values, &replaceIteratorWithExp, exp, name)?;
    } else {
        sitesArr = metamodelica::arrayFromVec(sites.reverse().into_iter().cloned().collect());
        exps = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut v in (values).into_iter().cloned() {
                let __x = (Expression::traverseExpBottomUp(
                    replaceIteratorWithExp(v.clone(), body.clone(), name.clone())?,
                    &fillIteratorSubscript,
                    (sitesArr.clone(), v.clone()),
                )?)
                .0;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    Ok(exps)
}

fn isIntegerConstant(mut exp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (match &**exp {
        DAE::Exp::ICONST { .. } => true,
        _ => false,
    });
    b
}

fn extractIteratorSubscript(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (ArcStr, metamodelica::List<metamodelica::Ref<DAE::Exp>>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (ArcStr, metamodelica::List<metamodelica::Ref<DAE::Exp>>),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (ArcStr, metamodelica::List<metamodelica::Ref<DAE::Exp>>);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::ASUB { exp: arr, sub: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, .. } }, tail: Deref @ metamodelica::ListNode::Nil } }, (name, sites)) if (stringEq(&id, &name) && !(iteratorOrPlaceholderOccurs(arr.clone(), name.clone())?)) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            ty = Expression::r#typeof(inExp)?;
            (metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("$iterSub"), identType: ty.clone(), subscriptLst: list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: ((sites).len() as i32) + 1 }) })] }), ty: ty }), (name.clone(), metamodelica::cons(arr.clone(), sites.clone())))
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn iteratorOrPlaceholderOccurs(mut exp: metamodelica::Ref<DAE::Exp>, mut name: ArcStr) -> Result<bool> {
    let mut occurs: bool;
    let (_, (_, __pa0)) = Expression::traverseExpBottomUp(
        exp,
        &fnptr!(
            iteratorOrPlaceholderOccursTraverser,
            metamodelica::Ref<DAE::Exp>,
            (ArcStr, bool)
        ),
        (name, false),
    )?;
    occurs = metamodelica::Own::own(__pa0);
    Ok(occurs)
}

fn iteratorOrPlaceholderOccursTraverser(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (ArcStr, bool),
) -> (metamodelica::Ref<DAE::Exp>, (ArcStr, bool)) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outTpl: (ArcStr, bool);
    outTpl = (::match_deref::match_deref! { match &((inExp, inTpl.clone())) {
        (_, (_, true)) => {
            inTpl
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }, .. }, (name, _)) => {
            (name.clone(), stringEq(&id, &name) || stringEq(&id, &(literal!("$iterSub"))))
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, .. }, .. }, (name, _)) => {
            (name.clone(), stringEq(&id, &name))
        },
        _ => {
            inTpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outTpl)
}

fn fillIteratorSubscript(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::Exp>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::Exp>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::Exp>,
    ) = inTpl.clone();
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), inTpl)) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$iterSub", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: n } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, (sites, sub)) => {
            selectElement(&(metamodelica::arrayGet(sites.clone(), n.clone())?), metamodelica::AsArg::as_arg(&sub))?
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn selectElement(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut sub: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut i: i32 = Expression::expInt(sub)?;
    outExp = 'mc: {
        let __mc_input = &**exp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: exps, .. } => {
                    if !((i >= 1 && i <= ((exps).len() as i32))) { return Err("guard") }
                    Ok((exps).get(i)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { ty, matrix: rows, .. } => {
                    if !((i >= 1 && i <= ((rows).len() as i32))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: Expression::unliftArray(metamodelica::AsArg::as_arg(&ty))?, scalar: true, array: (rows).get(i)? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
                    if !((Types::isArray(metamodelica::AsArg::as_arg(&ty)) && referenceEq(&*(simplifyCref(exp.clone(), metamodelica::AsArg::as_arg(&cr), ty.clone())),&*(&**exp)))) { return Err("guard") }
                    Ok(Expression::makeCrefExp(simplifyAsubCref(metamodelica::AsArg::as_arg(&cr), sub)?, Expression::unliftArray(metamodelica::AsArg::as_arg(&ty))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e1 } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut e1 = (*e1).clone();
                    e1 = selectElement(metamodelica::AsArg::as_arg(&e1), sub)?;
                    ty = Expression::r#typeof(e1.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: if (DAEUtil::expTypeArray(&ty)) {DAE::Operator::UMINUS_ARR { ty: ty.clone() }} else {DAE::Operator::UMINUS { ty: ty.clone() }}, exp: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    if !((isSelectableOperator(metamodelica::AsArg::as_arg(&op)))) { return Err("guard") }
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    if !(Expression::isScalarArrayOp(metamodelica::AsArg::as_arg(&op))) {
                        e1 = selectElement(metamodelica::AsArg::as_arg(&e1), sub)?;
                    }
                    if !(Expression::isArrayScalarOp(metamodelica::AsArg::as_arg(&op))) {
                        e2 = selectElement(metamodelica::AsArg::as_arg(&e2), sub)?;
                    }
                    ty = Expression::r#typeof(if (Expression::isScalarArrayOp(metamodelica::AsArg::as_arg(&op))) {e2.clone()} else {e1.clone()})?;
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: selectedOperator(metamodelica::AsArg::as_arg(&op), ty.clone(), DAEUtil::expTypeArray(&ty))?, exp2: e2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::Ref::new(DAE::Exp::ASUB { exp: exp.clone(), sub: list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: sub.clone() })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn isSelectableOperator(mut op: &Operator) -> bool {
    let mut b: bool;
    b = (match op.clone() {
        DAE::Operator::ADD_ARR { .. } => true,
        DAE::Operator::SUB_ARR { .. } => true,
        DAE::Operator::MUL_ARR { .. } => true,
        DAE::Operator::DIV_ARR { .. } => true,
        DAE::Operator::POW_ARR2 { .. } => true,
        _ => Expression::isArrayScalarOp(op) || Expression::isScalarArrayOp(op),
    });
    b
}

fn selectedOperator(mut op: &Operator, mut ty: metamodelica::Ref<DAE::Type>, mut isArray: bool) -> Result<Operator> {
    let mut outOp: Operator;
    outOp = (match op.clone() {
        DAE::Operator::ADD_ARR { .. } => {
            if (isArray) {
                DAE::Operator::ADD_ARR { ty: ty }
            } else {
                DAE::Operator::ADD { ty: ty }
            }
        }
        DAE::Operator::SUB_ARR { .. } => {
            if (isArray) {
                DAE::Operator::SUB_ARR { ty: ty }
            } else {
                DAE::Operator::SUB { ty: ty }
            }
        }
        DAE::Operator::MUL_ARR { .. } => {
            if (isArray) {
                DAE::Operator::MUL_ARR { ty: ty }
            } else {
                DAE::Operator::MUL { ty: ty }
            }
        }
        DAE::Operator::DIV_ARR { .. } => {
            if (isArray) {
                DAE::Operator::DIV_ARR { ty: ty }
            } else {
                DAE::Operator::DIV { ty: ty }
            }
        }
        DAE::Operator::POW_ARR2 { .. } => {
            if (isArray) {
                DAE::Operator::POW_ARR2 { ty: ty }
            } else {
                DAE::Operator::POW { ty: ty }
            }
        }
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => {
            if (isArray) {
                DAE::Operator::MUL_ARRAY_SCALAR { ty: ty }
            } else {
                DAE::Operator::MUL { ty: ty }
            }
        }
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => {
            if (isArray) {
                DAE::Operator::ADD_ARRAY_SCALAR { ty: ty }
            } else {
                DAE::Operator::ADD { ty: ty }
            }
        }
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => {
            if (isArray) {
                DAE::Operator::DIV_ARRAY_SCALAR { ty: ty }
            } else {
                DAE::Operator::DIV { ty: ty }
            }
        }
        DAE::Operator::POW_ARRAY_SCALAR { .. } => {
            if (isArray) {
                DAE::Operator::POW_ARRAY_SCALAR { ty: ty }
            } else {
                DAE::Operator::POW { ty: ty }
            }
        }
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => {
            if (isArray) {
                DAE::Operator::SUB_SCALAR_ARRAY { ty: ty }
            } else {
                DAE::Operator::SUB { ty: ty }
            }
        }
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => {
            if (isArray) {
                DAE::Operator::DIV_SCALAR_ARRAY { ty: ty }
            } else {
                DAE::Operator::DIV { ty: ty }
            }
        }
        DAE::Operator::POW_SCALAR_ARRAY { .. } => {
            if (isArray) {
                DAE::Operator::POW_SCALAR_ARRAY { ty: ty }
            } else {
                DAE::Operator::POW { ty: ty }
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outOp)
}

fn simplifyReductionFoldPhase(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut optFoldExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut foldName: &ArcStr,
    mut resultName: &ArcStr,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut defaultValue: Option<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut checkForSimplifications: bool;
    (exp, checkForSimplifications) = (::match_deref::match_deref! { match &((path.clone(), optFoldExp, inExps.clone(), defaultValue)) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, _, _, _) => {
            let mut aty: metamodelica::Ref<DAE::Type>;
            let mut ty2: metamodelica::Ref<DAE::Type>;
            let mut length: i32;
            aty = Types::unliftArray(&(Types::expTypetoTypesType(&ty)))?;
            length = ((inExps).len() as i32);
            ty2 = Types::liftArray(aty.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: length }));
            exp = Expression::makeArray(inExps, ty2, !(Types::isArray(&aty)));
            (exp, false)
        },
        (_, _, Deref @ metamodelica::ListNode::Nil, Some(val)) => {
            (ValuesUtil::valueExp(val.clone(), None)?, false)
        },
        (_, _, Deref @ metamodelica::ListNode::Nil, None) => {
            return Err("fail")
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, _, _, _) => {
            let mut arr_exp: metamodelica::Ref<DAE::Exp>;
            arr_exp = Expression::makeScalarArray(inExps, ty.clone());
            (Expression::makePureBuiltinCall(literal!("min"), list![arr_exp], ty), true)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, _, _, _) => {
            let mut arr_exp: metamodelica::Ref<DAE::Exp>;
            arr_exp = Expression::makeScalarArray(inExps, ty.clone());
            (Expression::makePureBuiltinCall(literal!("max"), list![arr_exp], ty), true)
        },
        (_, Some(_), Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
            exp = (*__esc_exp).clone();
            (exp.clone(), false)
        },
        (_, Some(foldExp), Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: exps }, _) => {
            exp = (*__esc_exp).clone();
            exp = simplifyReductionFoldPhase2(metamodelica::AsArg::as_arg(&exps), metamodelica::AsArg::as_arg(&foldExp), foldName, resultName, exp.clone())?;
            (exp.clone(), false)
        },
        _ => return Err("match: no arm matched"),
    } });
    if checkForSimplifications {
        let __pa0 = ::match_deref::match_deref! { match &(simplify1(exp)?) {
            (__pa0, true) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
    }
    Ok(exp)
}

fn simplifyReductionFoldPhase2<'__b>(
    mut inExps: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut foldExp: &'__b metamodelica::Ref<DAE::Exp>,
    mut foldName: &'__b ArcStr,
    mut resultName: &'__b ArcStr,
    mut acc: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (::match_deref::match_deref! { match inExps {
        Deref @ metamodelica::ListNode::Nil => {
            acc
        },
        Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: exps } => {
            exp = (*__esc_exp).clone();
            exp = replaceIteratorWithExp(exp.clone(), foldExp.clone(), foldName.clone())?;
            exp = replaceIteratorWithExp(acc, exp.clone(), resultName.clone())?;
            simplifyReductionFoldPhase2(exps, foldExp, foldName, resultName, exp.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn hasZeroLengthIterator<'__b>(
    mut inIters: &'__b metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inIters {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { guardExp: Some(Deref @ DAE::Exp::BCONST { bool: false }), .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { exp: Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::ListNode::Nil }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { exp: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: iters } => {
                { inIters = iters; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn simplifyList(
    mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut exp in (expl).into_iter().cloned() {
            let __x = (simplify1(exp.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpl)
}

pub(crate) fn simplifyList1(
    mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<bool>,
)> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outBool: metamodelica::List<bool> = metamodelica::nil();
    outExpl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut exp in (expl).into_iter().cloned() {
            let __x = (match &*exp.clone() {
                _ => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut b2: bool;
                    (e, b2) = simplify(exp.clone())?;
                    outBool = metamodelica::cons(b2, outBool.clone());
                    e.clone()
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outBool = Dangerous::listReverseInPlace(outBool);
    Ok((outExpl, outBool))
}

pub fn condsimplifyList1(
    mut blst: &metamodelica::List<bool>,
    mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<bool>,
)> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outBool: metamodelica::List<bool> = metamodelica::nil();
    let mut rest_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expl;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut b2: bool;
    for mut b in &**blst {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        rest_expl = metamodelica::Own::own(__pa1);
        (exp, b2) = condsimplify(b.clone(), exp)?;
        outExpl = metamodelica::cons(exp, outExpl);
        outBool = metamodelica::cons(b2, outBool);
    }
    outExpl = Dangerous::listReverseInPlace(outExpl);
    outBool = Dangerous::listReverseInPlace(outBool);
    Ok((outExpl, outBool))
}

fn checkZeroLengthArrayOp(mut op: &DAE::Operator) -> Result<()> {
    let () = (match op.clone() {
        DAE::Operator::ADD_ARR { .. } => (),
        DAE::Operator::SUB_ARR { .. } => (),
        DAE::Operator::MUL_ARR { .. } => (),
        DAE::Operator::DIV_ARR { .. } => (),
        DAE::Operator::POW_ARR { .. } => (),
        DAE::Operator::POW_ARR2 { .. } => (),
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => (),
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => (),
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => (),
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => (),
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => (),
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub fn simplifyAddSymbolicOperation(
    mut exp: metamodelica::Ref<DAE::EquationExp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<DAE::EquationExp>,
    metamodelica::Ref<DAE::ElementSource>,
)> {
    let mut outExp: metamodelica::Ref<DAE::EquationExp>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    (outExp, outSource) = (match &*exp {
        DAE::EquationExp::PARTIAL_EQUATION { exp: e } => {
            let mut changed: bool;
            let mut e = (*e).clone();
            (e, changed) = simplify(e.clone())?;
            outExp = if (changed) {
                metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() })
            } else {
                exp.clone()
            };
            outSource = ElementSource::condAddSymbolicTransformation(
                changed,
                source,
                metamodelica::Ref::new(DAE::SymbolicOperation::SIMPLIFY {
                    before: exp,
                    after: outExp.clone(),
                }),
            )?;
            (outExp, outSource)
        }
        DAE::EquationExp::RESIDUAL_EXP { exp: e } => {
            let mut changed: bool;
            let mut e = (*e).clone();
            (e, changed) = simplify(e.clone())?;
            outExp = if (changed) {
                metamodelica::Ref::new(DAE::EquationExp::RESIDUAL_EXP { exp: e.clone() })
            } else {
                exp.clone()
            };
            outSource = ElementSource::condAddSymbolicTransformation(
                changed,
                source,
                metamodelica::Ref::new(DAE::SymbolicOperation::SIMPLIFY {
                    before: exp,
                    after: outExp.clone(),
                }),
            )?;
            (outExp, outSource)
        }
        DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2 } => {
            let mut changed: bool;
            let mut changed1: bool;
            let mut changed2: bool;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, changed1) = simplify(e1.clone())?;
            (e2, changed2) = simplify(e2.clone())?;
            changed = changed1 || changed2;
            outExp = if (changed) {
                metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS {
                    lhs: e1.clone(),
                    rhs: e2.clone(),
                })
            } else {
                exp.clone()
            };
            outSource = ElementSource::condAddSymbolicTransformation(
                changed,
                source,
                metamodelica::Ref::new(DAE::SymbolicOperation::SIMPLIFY {
                    before: exp,
                    after: outExp.clone(),
                }),
            )?;
            (outExp, outSource)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("ExpressionSimplify.simplifyAddSymbolicOperation failed")],
            )?;
            return Err("fail");
        }
    });
    Ok((outExp, outSource))
}

pub(crate) fn condSimplifyAddSymbolicOperation(
    mut cond: bool,
    mut exp: metamodelica::Ref<DAE::EquationExp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<DAE::EquationExp>,
    metamodelica::Ref<DAE::ElementSource>,
)> {
    let mut exp: metamodelica::Ref<DAE::EquationExp> = exp;
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if cond {
        (exp, source) = simplifyAddSymbolicOperation(exp, source)?;
    }
    Ok((exp, source))
}

fn simplifySize(
    mut origExp: metamodelica::Ref<DAE::Exp>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut optDim: Option<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = optDim;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(dimExp) => {
                    let mut i: i32;
                    let mut n: i32;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    i = Expression::expInt(metamodelica::AsArg::as_arg(&dimExp))?;
                    t = Expression::r#typeof(exp.clone())?;
                    dims = Expression::arrayDimension(&t);
                    dim = (dims).get(i)?;
                    n = Expression::dimensionSize(&dim)?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn simplifyTSub(mut origExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(origExp.clone()) {
        Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::TUPLE { PR: expl }, .. }, ix: i, .. } => {
            (expl).get(i.clone())?
        },
        Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::TUPLE { PR: expl }, ix: i, .. } => {
            (expl).get(i.clone())?
        },
        Deref @ DAE::Exp::TSUB { exp: e @ Deref @ DAE::Exp::RCONST { .. }, .. } => {
            e.clone()
        },
        _ => {
            origExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn simplifyNoEvent(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = Expression::addNoEventToEventTriggeringFunctions(Expression::addNoEventToRelations(
        Expression::stripNoEvent(inExp)?,
    )?)?;
    Ok(e)
}

fn maxElement(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut elt: Option<metamodelica::Ref<DAE::Exp>>;
    elt = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ DAE::Exp::RCONST { real: _ }, None) => {
            Some(e1)
        },
        (Deref @ DAE::Exp::ICONST { integer: _ }, None) => {
            Some(e1)
        },
        (Deref @ DAE::Exp::BCONST { bool: _ }, None) => {
            Some(e1)
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Some(Deref @ DAE::Exp::RCONST { real: r2 })) => {
            if (r1.clone() > r2.clone()) {Some(e1)} else {e2}
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Some(Deref @ DAE::Exp::ICONST { integer: i2 })) => {
            if (intGt(i1.clone(), i2.clone())) {Some(e1)} else {e2}
        },
        (Deref @ DAE::Exp::BCONST { bool: b1 }, Some(Deref @ DAE::Exp::BCONST { bool: b2 })) => {
            if (b2.clone() || b1.clone() == b2.clone()) {e2} else {Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: b1.clone() }))}
        },
        _ => {
            e2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elt
}

fn minElement(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut elt: Option<metamodelica::Ref<DAE::Exp>>;
    elt = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ DAE::Exp::RCONST { real: _ }, None) => {
            Some(e1)
        },
        (Deref @ DAE::Exp::ICONST { integer: _ }, None) => {
            Some(e1)
        },
        (Deref @ DAE::Exp::BCONST { bool: _ }, None) => {
            Some(e1)
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Some(Deref @ DAE::Exp::RCONST { real: r2 })) => {
            if (r1.clone() < r2.clone()) {Some(e1)} else {e2}
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Some(Deref @ DAE::Exp::ICONST { integer: i2 })) => {
            if (intLt(i1.clone(), i2.clone())) {Some(e1)} else {e2}
        },
        (Deref @ DAE::Exp::BCONST { bool: b1 }, Some(Deref @ DAE::Exp::BCONST { bool: b2 })) => {
            if (!(b2.clone()) || b1.clone() == b2.clone()) {e2} else {Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: b1.clone() }))}
        },
        _ => {
            e2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elt
}

fn removeMinMaxFoldableValues(mut e: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut filter: bool;
    filter = (match &**e {
        DAE::Exp::RCONST { real: _ } => false,
        DAE::Exp::ICONST { integer: _ } => false,
        DAE::Exp::BCONST { bool: _ } => false,
        _ => true,
    });
    filter
}

pub(crate) fn simplifySkew(
    mut v1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut res: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut x1: metamodelica::Ref<DAE::Exp>;
    let mut x2: metamodelica::Ref<DAE::Exp>;
    let mut x3: metamodelica::Ref<DAE::Exp>;
    let mut zero: metamodelica::Ref<DAE::Exp>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*v1)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    x1 = metamodelica::Own::own(__pa0);
    x2 = metamodelica::Own::own(__pa1);
    x3 = metamodelica::Own::own(__pa2);
    zero = Expression::makeConstZero(&(Expression::r#typeof(x1.clone())?));
    res = list![
        list![zero.clone(), Expression::negate(x3.clone())?, x2.clone()],
        list![x3, zero.clone(), Expression::negate(x1.clone())?],
        list![Expression::negate(x2)?, x1, zero]
    ];
    Ok(res)
}

pub(crate) fn simplifyCross(
    mut v1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut v2: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut x1: metamodelica::Ref<DAE::Exp>;
    let mut x2: metamodelica::Ref<DAE::Exp>;
    let mut x3: metamodelica::Ref<DAE::Exp>;
    let mut y1: metamodelica::Ref<DAE::Exp>;
    let mut y2: metamodelica::Ref<DAE::Exp>;
    let mut y3: metamodelica::Ref<DAE::Exp>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*v1)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    x1 = metamodelica::Own::own(__pa0);
    x2 = metamodelica::Own::own(__pa1);
    x3 = metamodelica::Own::own(__pa2);
    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &((*v2)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Nil } } } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    y1 = metamodelica::Own::own(__pa4);
    y2 = metamodelica::Own::own(__pa5);
    y3 = metamodelica::Own::own(__pa6);
    res = list![
        Expression::expSub(
            Expression::makeProduct(x2.clone(), y3.clone())?,
            Expression::makeProduct(x3.clone(), y2.clone())?
        )?,
        Expression::expSub(
            Expression::makeProduct(x3, y1.clone())?,
            Expression::makeProduct(x1.clone(), y3)?
        )?,
        Expression::expSub(Expression::makeProduct(x1, y2)?, Expression::makeProduct(x2, y1)?)?
    ];
    Ok(res)
}
