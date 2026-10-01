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

use crate::BackendDAEUtil;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::Ceval;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
// =============================================================================
// section for postOptModule >>solveSimpleEquations<<
//
// solve simple equations otherwise detect EQUATIONSYSTEM
// =============================================================================
pub(crate) fn solveSimpleEquations(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    assign_field!(
        dae.eqs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
            for mut syst in (dae.eqs.clone()).into_iter().cloned() {
                let __x = (::match_deref::match_deref! { match &(syst.clone()) {
                    Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps, ass1, ass2 }, .. } => {
                        let mut comps = (*comps).clone();
                        comps = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = metamodelica::nil();
                    for mut comp in (comps.clone()).into_iter().cloned() {
                        let __x = (match &*comp.clone() {
                    BackendDAE::StrongComponent::SINGLEEQUATION { .. } => {
                        let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                        let mut var: metamodelica::Ref<BackendDAE::Var>;
                        let mut eindex: i32;
                        let mut vindx: i32;
                        let mut solved: bool;
                        let mut tmpComp: metamodelica::Ref<BackendDAE::StrongComponent>;
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp.clone()) {
                            Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: __pa0, var: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        eindex = metamodelica::Own::own(__pa0);
                        vindx = metamodelica::Own::own(__pa1);
                        eqn = BackendEquation::get(syst.orderedEqs.clone(), eindex)?;
                        tmpComp = comp.clone();
                        if BackendEquation::isEquation(&eqn) {
                            var = BackendVariable::getVarAt(&(syst.orderedVars.clone()), vindx)?;
                            (eqn, solved) = solveSimpleEquation(eqn.clone(), &var, &dae.shared)?;
                            assign_field!(syst.orderedEqs = BackendEquation::setAtIndex(syst.orderedEqs.clone(), eindex, eqn.clone())?);
                            if !(solved) {
                                tmpComp = metamodelica::Ref::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: list![eindex], vars: list![vindx], jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(), jacType: openmodelica_backend_types::BackendDAE::JacobianType::JAC_NONLINEAR, mixedSystem: false });
                            }
                        }
                        tmpComp.clone()
                    },
                    _ => {
                        comp.clone()
                    },
                });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                        assign_field!(syst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }));
                        syst.clone()
                    },
                    _ => {
                        syst.clone()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(dae)
}

fn solveSimpleEquation(
    mut eqn: metamodelica::Ref<BackendDAE::Equation>,
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, bool)> {
    let mut eqn: metamodelica::Ref<BackendDAE::Equation> = eqn;
    let mut solved: bool;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut varexp: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut attr: BackendDAE::EquationAttributes;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut isContinuousIntegration: bool = BackendDAEUtil::isSimulationDAE(shared);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2, attr: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    source = metamodelica::Own::own(__pa2);
    attr = metamodelica::Own::own(__pa3);
    let __arc5 = &(*var);
    let BackendDAE::VAR { varName: __pa4, .. } = &**__arc5;
    cr = metamodelica::Own::own(__pa4);
    varexp = Expression::crefExp(cr.clone())?;
    if BackendVariable::isStateVar(var) {
        varexp = Expression::expDer(varexp);
        cr = ComponentReference::crefPrefixDer(cr);
    }
    if Types::isIntegerOrRealOrSubTypeOfEither(Expression::r#typeof(e1.clone())?)
        && Types::isIntegerOrRealOrSubTypeOfEither(Expression::r#typeof(e2.clone())?)
    {
        (e1, e2, _, _, _) = preprocessingSolve(
            e1,
            e2,
            varexp.clone(),
            None,
            Some(shared.functionTree.clone()),
            None,
            0,
            false,
        )?;
    }
    match '__try6: {
        (e, _, _, _) = unwrap_break_err!(solve2(e1.clone(), e2.clone(), varexp.clone(), Some(shared.functionTree.clone()), None, false, isContinuousIntegration), '__try6);
        source = unwrap_break_err!(ElementSource::addSymbolicTransformationSolve(true, source.clone(), cr.clone(), e1.clone(), e2.clone(), e.clone(), metamodelica::nil()), '__try6);
        eqn = unwrap_break_err!(BackendEquation::generateEquation(varexp.clone(), e.clone(), source.clone(), attr), '__try6);
        solved = true;
        Ok::<_, &'static str>((solved.clone(),))
    } {
        Ok((__try6_o0,)) => {
            solved = __try6_o0;
        }
        Err(_) => {
            solved = false;
        }
    }
    Ok((eqn, solved))
}

fn printTryToSolve(
    mut instanceName: &ArcStr,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*instanceName);
        __mm_s.push_str(&*literal!(" tries to solve: "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1)?);
        __mm_s.push_str(&*literal!(" = "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2)?);
        __mm_s.push_str(&*literal!("\nwith respect to: "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp3)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub fn solve(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut dummy1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut dummy2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dummyI: i32;
    (outExp, outAsserts, dummy1, dummy2, dummyI) = 'mc: {
        let __mc_input = &*inExp1;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveSimple(&inExp1, inExp2.clone(), inExp3.clone(), 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveSimple(&inExp2, inExp1.clone(), inExp3.clone(), 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveWork(inExp1.clone(), inExp2.clone(), inExp3.clone(), None, functions.clone(), None, 0, false, false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to solve \"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); __mm_s.push_str(&*literal!("\" w.r.t. \"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp3.clone())?); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/ExpressionSolve.mo"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    (outExp, _) = ExpressionSimplify::simplify1(outExp)?;
    Ok((outExp, outAsserts))
}

pub(crate) fn solve2(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    mut uniqueEqIndex: Option<i32>,
    mut doInline: bool,
    mut isContinuousIntegration: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dummyI: i32;
    (outExp, outAsserts, eqnForNewVars, newVarsCrefs, dummyI) = 'mc: {
        let __mc_input = &*inExp1;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveSimple(&inExp1, inExp2.clone(), inExp3.clone(), 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveSimple(&inExp2, inExp1.clone(), inExp3.clone(), 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveWork(inExp1.clone(), inExp2.clone(), inExp3.clone(), None, functions.clone(), uniqueEqIndex.clone(), 0, doInline, isContinuousIntegration)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to solve \"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); __mm_s.push_str(&*literal!("\" w.r.t. \"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp3.clone())?); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/ExpressionSolve.mo"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outAsserts, eqnForNewVars, newVarsCrefs))
}

fn solveWork(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut optCond: Option<metamodelica::Ref<DAE::Exp>>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    mut uniqueEqIndex: Option<i32>,
    mut idepth: i32,
    mut doInline: bool,
    mut isContinuousIntegration: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut depth: i32;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut eqnForNewVars1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqnForNewVars2: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVarsCrefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut newVarsCrefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (e1, e2, eqnForNewVars1, newVarsCrefs1, depth) = 'mc: {
        let __mc_input = &*inExp1;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(preprocessingSolve(inExp1.clone(), inExp2.clone(), inExp3.clone(), optCond.clone(), functions.clone(), uniqueEqIndex.clone(), idepth, doInline)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("\n-ExpressionSolve.preprocessingSolve failed:\n"))?;
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); ArcStr::from(__mm_s) })?;
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" with respect to: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp3.clone())?); ArcStr::from(__mm_s) })?;
                    }
                    Ok((inExp1.clone(), inExp2.clone(), metamodelica::nil(), metamodelica::nil(), idepth))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    (outExp, outAsserts, eqnForNewVars2, newVarsCrefs2, depth) = 'mc: {
        let __mc_input = &*e1;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveIfExp(&e1, e2.clone(), inExp3.clone(), optCond.clone(), functions.clone(), uniqueEqIndex.clone(), depth, doInline, isContinuousIntegration)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveSimple(&e1, e2.clone(), inExp3.clone(), depth)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(solveLinearSystem(e1.clone(), e2.clone(), inExp3.clone(), functions.clone(), depth)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    eqnForNewVars = listAppend(eqnForNewVars1, eqnForNewVars2);
    newVarsCrefs = listAppend(newVarsCrefs1, newVarsCrefs2);
    Ok((outExp, outAsserts, eqnForNewVars, newVarsCrefs, depth))
}

fn solveSimple(
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut idepth: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut odepth: i32 = idepth;
    (outExp, outAsserts) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp3.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (inExp2.clone(), metamodelica::nil())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))? && !(Expression::expHasDerCref(inExp2.clone(), cr.clone())?)) => {
            (inExp2.clone(), metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr))? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr))? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr))? && !(Expression::expHasDerCref(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr))? && !(Expression::expHasDerCref(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr))? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))? && !(Expression::expHasCrefNoPreorDer(inExp2.clone(), cr.clone())?)) => {
            let mut asserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            asserts = generateAssertType(metamodelica::AsArg::as_arg(&tp), metamodelica::AsArg::as_arg(&cr), inExp3, metamodelica::nil())?;
            (metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: inExp2.clone() }), asserts)
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outAsserts, eqnForNewVars, newVarsCrefs, odepth))
}

fn generateAssertType(
    mut tp: &metamodelica::Ref<DAE::Type>,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut iExp: metamodelica::Ref<DAE::Exp>,
    mut inAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outAsserts = (match &**tp {
        DAE::Type::T_ENUMERATION { path, names, .. } => {
            let mut p1: metamodelica::Ref<Absyn::Path>;
            let mut pn: metamodelica::Ref<Absyn::Path>;
            let mut n: i32;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut en: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut es: metamodelica::Ref<DAE::Exp>;
            let mut s1: ArcStr;
            let mut sn: ArcStr;
            let mut estr: ArcStr;
            let mut crstr: ArcStr;
            p1 = AbsynUtil::suffixPath(path, &((names).head().cloned()?));
            e1 = metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
                name: p1.clone(),
                index: 1,
            });
            n = ((names).len() as i32);
            pn = AbsynUtil::suffixPath(path, &((names).get(n)?));
            en = metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
                name: p1.clone(),
                index: n,
            });
            s1 = AbsynUtil::pathString(p1, literal!("."), true, false)?;
            sn = AbsynUtil::pathString(pn, literal!("."), true, false)?;
            crstr = ComponentReferenceBasics::printComponentRefStr(cr)?;
            estr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Expression for "));
                __mm_s.push_str(&*crstr);
                __mm_s.push_str(&*literal!(" out of min("));
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*literal!(")/max("));
                __mm_s.push_str(&*sn);
                __mm_s.push_str(&*literal!(") = "));
                ArcStr::from(__mm_s)
            };
            e = metamodelica::Ref::new(DAE::Exp::LBINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::RELATION {
                    exp1: iExp.clone(),
                    operator: DAE::Operator::GREATEREQ {
                        ty: DAE::T_INTEGER_DEFAULT().clone(),
                    },
                    exp2: e1,
                    index: -1,
                    optionExpisASUB: None,
                }),
                operator: DAE::Operator::AND {
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                },
                exp2: metamodelica::Ref::new(DAE::Exp::RELATION {
                    exp1: iExp.clone(),
                    operator: DAE::Operator::LESSEQ {
                        ty: DAE::T_INTEGER_DEFAULT().clone(),
                    },
                    exp2: en,
                    index: -1,
                    optionExpisASUB: None,
                }),
            });
            es = Expression::makePureBuiltinCall(
                literal!("String"),
                list![iExp, metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("d") })],
                DAE::T_STRING_DEFAULT().clone(),
            );
            es = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::SCONST { string: estr }),
                operator: DAE::Operator::ADD {
                    ty: DAE::T_STRING_DEFAULT().clone(),
                },
                exp2: es,
            });
            metamodelica::cons(
                metamodelica::Ref::new(DAE::Statement::STMT_ASSERT {
                    cond: e,
                    msg: es,
                    level: DAE::ASSERTIONLEVEL_ERROR().clone(),
                    source: DAE::emptyElementSource().clone(),
                }),
                inAsserts,
            )
        }
        _ => inAsserts,
    });
    Ok(outAsserts)
}

pub(crate) fn preprocessingSolve(
    mut x: metamodelica::Ref<DAE::Exp>,
    mut y: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut optCond: Option<metamodelica::Ref<DAE::Exp>>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    mut uniqueEqIndex: Option<i32>,
    mut idepth: i32,
    mut doInline: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
)> {
    let mut x: metamodelica::Ref<DAE::Exp> = x;
    let mut y: metamodelica::Ref<DAE::Exp> = y;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut depth: i32 = idepth;
    let mut lhsX: metamodelica::Ref<DAE::Exp>;
    let mut rhsX: metamodelica::Ref<DAE::Exp>;
    let mut lhsY: metamodelica::Ref<DAE::Exp>;
    let mut rhsY: metamodelica::Ref<DAE::Exp>;
    let mut N: metamodelica::Ref<DAE::Exp>;
    let mut con: bool;
    let mut new_x: bool;
    let mut inlineFun: bool = true;
    let mut iter: i32;
    let mut numSimplifed: i32 = 0;
    (lhsX, lhsY) = preprocessingSolve5(x, inExp3.clone(), true)?;
    (rhsX, rhsY) = preprocessingSolve5(y, inExp3.clone(), true)?;
    x = Expression::expSub(lhsX, rhsX)?;
    y = Expression::expSub(rhsY, lhsY)?;
    con = !(Expression::isCref(&x));
    iter = 0;
    if con {
        (x, _) = unifyFunCalls(x, inExp3.clone())?;
    }
    while con && iter < 1000 && !(Expression::isCref(&x)) {
        (x, y, con) = preprocessingSolve2(x, y, inExp3.clone())?;
        (x, y, new_x) = preprocessingSolve3(x, y, inExp3.clone())?;
        con = con || new_x;
        while new_x {
            (x, y, new_x) = preprocessingSolve3(x, y, inExp3.clone())?;
        }
        if Expression::isCref(&x) {
            break;
        }
        (x, y, new_x) = removeSimpleCalls(x, y, inExp3.clone());
        con = con || new_x;
        (x, y, new_x) = preprocessingSolve4(x, y, inExp3.clone())?;
        con = new_x || con;
        if (uniqueEqIndex).is_some() && !(stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp")))) {
            (x, y, new_x, eqnForNewVars, newVarsCrefs, depth) = preprocessingSolveTmpVars(
                x,
                y,
                inExp3.clone(),
                optCond.clone(),
                Util::getOption(uniqueEqIndex.clone())?,
                eqnForNewVars,
                newVarsCrefs,
                depth,
            );
            con = new_x || con;
        }
        if !(con) {
            if numSimplifed < 3 {
                (x, con) = ExpressionSimplify::simplify(x)?;
                numSimplifed = numSimplifed + 1;
            }
            (x, N) = Expression::makeFraction(x)?;
            if !(Expression::isOne(&N)) {
                new_x = true;
                y = Expression::expMul(y, N)?;
            }
            con = new_x || con;
        }
        if con {
            (lhsX, lhsY) = preprocessingSolve5(x, inExp3.clone(), true)?;
            (rhsX, rhsY) = preprocessingSolve5(y, inExp3.clone(), false)?;
            x = Expression::expSub(lhsX, rhsX)?;
            y = Expression::expSub(rhsY, lhsY)?;
        } else if doInline && inlineFun {
            iter = iter + 50;
            if inlineFun {
                (x, con) = solveFunCalls(x, inExp3.clone(), functions.clone());
                inlineFun = false;
                if con {
                    numSimplifed = 0;
                }
            }
        }
        iter = iter + 1;
    }
    (y, _) = ExpressionSimplify::simplify1(y)?;
    Ok((x, y, eqnForNewVars, newVarsCrefs, depth))
}

fn preprocessingSolve2(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, bool)> {
    let mut olhs: metamodelica::Ref<DAE::Exp>;
    let mut orhs: metamodelica::Ref<DAE::Exp>;
    let mut con: bool;
    (olhs, orhs, con) = (match &*inExp1 {
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS { .. },
            exp: fa,
        } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut b: metamodelica::Ref<DAE::Exp>;
            b = Expression::negate(inExp2.clone())?;
            (fa.clone(), b.clone(), true)
        }
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS_ARR { .. },
            exp: fa,
        } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut b: metamodelica::Ref<DAE::Exp>;
            b = Expression::negate(inExp2.clone())?;
            (fa.clone(), b.clone(), true)
        }
        DAE::Exp::BINARY {
            exp1: b,
            operator: DAE::Operator::DIV { ty: _ },
            exp2: fa,
        } if (expHasCref(fa.clone(), inExp3.clone())?
            && !(expHasCref(b.clone(), inExp3.clone())?)
            && !(expHasCref(inExp2.clone(), inExp3.clone())?)) =>
        {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::makeDiv(b.clone(), inExp2.clone())?;
            (fa.clone(), e, true)
        }
        DAE::Exp::BINARY {
            exp1: b,
            operator: DAE::Operator::MUL { ty: _ },
            exp2: fa,
        } if (expHasCref(fa.clone(), inExp3.clone())?
            && !(expHasCref(b.clone(), inExp3.clone())?)
            && !(expHasCref(inExp2.clone(), inExp3.clone())?)) =>
        {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut eWithX: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut factorWithX: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut factorWithoutX: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut pWithX: metamodelica::Ref<DAE::Exp>;
            let mut pWithoutX: metamodelica::Ref<DAE::Exp>;
            eWithX = Expression::expandFactors(&inExp1)?;
            (factorWithX, factorWithoutX) = List::split1OnTrue(&eWithX, &expHasCref, inExp3.clone())?;
            pWithX = makeProductLstSort(&factorWithX)?;
            pWithoutX = makeProductLstSort(&factorWithoutX)?;
            e = Expression::makeDiv(inExp2.clone(), pWithoutX)?;
            (pWithX, e, true)
        }
        DAE::Exp::BINARY {
            exp1: b,
            operator: DAE::Operator::MUL { ty: _ },
            exp2: fa,
        } if (expHasCref(fa.clone(), inExp3.clone())?
            && !(expHasCref(b.clone(), inExp3.clone())?)
            && !(expHasCref(inExp2.clone(), inExp3.clone())?)) =>
        {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::makeDiv(inExp2.clone(), b.clone())?;
            (fa.clone(), e, true)
        }
        DAE::Exp::BINARY {
            exp1: fa,
            operator: DAE::Operator::MUL { ty: _ },
            exp2: b,
        } if (expHasCref(fa.clone(), inExp3.clone())?
            && !(expHasCref(b.clone(), inExp3.clone())?)
            && !(expHasCref(inExp2.clone(), inExp3.clone())?)) =>
        {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::makeDiv(inExp2.clone(), b.clone())?;
            (fa.clone(), e, true)
        }
        DAE::Exp::BINARY {
            exp1: fa,
            operator: DAE::Operator::DIV { ty: _ },
            exp2: b,
        } if (expHasCref(fa.clone(), inExp3.clone())?
            && !(expHasCref(b.clone(), inExp3.clone())?)
            && !(expHasCref(inExp2.clone(), inExp3.clone())?)) =>
        {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::expMul(inExp2.clone(), b.clone())?;
            (fa.clone(), e, true)
        }
        DAE::Exp::BINARY {
            exp1: ga,
            operator: DAE::Operator::DIV { ty: tp },
            exp2: fa,
        } if (expHasCref(fa.clone(), inExp3.clone())?
            && expHasCref(ga.clone(), inExp3.clone())?
            && !(expHasCref(inExp2.clone(), inExp3.clone())?)) =>
        {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            e = Expression::expMul(inExp2.clone(), fa.clone())?;
            lhs = Expression::expSub(e, ga.clone())?;
            e = Expression::makeConstZero(metamodelica::AsArg::as_arg(&tp));
            (lhs, e, true)
        }
        _ => (inExp1, inExp2.clone(), false),
    });
    Ok((olhs, orhs, con))
}

fn preprocessingSolve3(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, bool)> {
    let mut olhs: metamodelica::Ref<DAE::Exp>;
    let mut orhs: metamodelica::Ref<DAE::Exp>;
    let mut con: bool;
    (olhs, orhs, con) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::RCONST { real: r1 }, operator: DAE::Operator::POW { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: r2 }) if (r2.clone() > metamodelica::OrderedFloat(0.0_f64) && r1.clone() > metamodelica::OrderedFloat(0.0_f64) && !(Expression::isConstOne(metamodelica::AsArg::as_arg(&e1))) && expHasCref(e2.clone(), inExp3.clone())?) => {
            let mut r: metamodelica::Real;
            let mut res: metamodelica::Ref<DAE::Exp>;
            r = metamodelica::real_div_checked((r2.clone()).ln(), (r1.clone()).ln())?;
            res = metamodelica::Ref::new(DAE::Exp::RCONST { real: r });
            (e2.clone(), res, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: __rlit_0 }) if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (expHasCref(e1.clone(), inExp3.clone())? && !(expHasCref(e2.clone(), inExp3.clone())?)) => {
            (e1.clone(), inExp2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::RCONST { real: r } }, _) if (!(expHasCref(inExp2.clone(), inExp3.clone())?) && expHasCref(e1.clone(), inExp3.clone())? && metamodelica::OrderedFloat(1.0_f64) == realMod(r.clone(), metamodelica::OrderedFloat(2.0_f64))) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = Expression::makeDiv(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e2.clone())?;
            res = Expression::expPow(inExp2.clone(), res)?;
            (e1.clone(), res, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_1 } }, _) if __rlit_1.eq(&metamodelica::OrderedFloat((0.5) as f64)) && (!(expHasCref(inExp2.clone(), inExp3.clone())?) && expHasCref(e1.clone(), inExp3.clone())?) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = Expression::expPow(inExp2.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            (e1.clone(), res, true)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_2 }) if __rlit_2.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), inExp2.clone(), true)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_3 }) if __rlit_3.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), inExp2.clone(), true)
        },
        _ => {
            (inExp1, inExp2.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((olhs, orhs, con))
}

fn preprocessingSolve4(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, bool)> {
    let mut oExp1: metamodelica::Ref<DAE::Exp>;
    let mut oExp2: metamodelica::Ref<DAE::Exp>;
    let mut newX: bool;
    (oExp1, oExp2, newX) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_4 }) if __rlit_4.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_5 }) if __rlit_5.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_6 }) if __rlit_6.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_7 }) if __rlit_7.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_8 }) if __rlit_8.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_9 }) if __rlit_9.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_10 }) if __rlit_10.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            (e1.clone(), inExp2, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_11 }) if __rlit_11.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            (e1.clone(), inExp2, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, operator: DAE::Operator::SUB { ty: tp }, exp2: Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } } }, Deref @ DAE::Exp::RCONST { real: __rlit_12 }) if __rlit_12.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::makePureBuiltinCall(literal!("tanh"), list![e1.clone()], tp.clone());
            (Expression::expMul(e3.clone(), e)?, e4.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, operator: DAE::Operator::SUB { ty: tp }, exp2: Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } } }, Deref @ DAE::Exp::RCONST { real: __rlit_13 }) if __rlit_13.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::makePureBuiltinCall(literal!("tanh"), list![e1.clone()], tp.clone());
            (Expression::expMul(e3.clone(), e)?, e4.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: __rlit_14 }) if __rlit_14.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), Expression::expPow(e2.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_15 }) if __rlit_15.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), Expression::expPow(e2.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, operator: DAE::Operator::SUB { ty: tp }, exp2: Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::POW { .. }, exp2: e4 } }, Deref @ DAE::Exp::RCONST { real: __rlit_16 }) if __rlit_16.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e2), e4.clone())? && expHasCref(e1.clone(), inExp3.clone())? && expHasCref(e3.clone(), inExp3.clone())?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            e = Expression::expPow(Expression::makeDiv(e1.clone(), e3.clone())?, e2.clone())?;
            (e_1, e_2, _) = preprocessingSolve3(e, Expression::makeConstOne(metamodelica::AsArg::as_arg(&tp)), inExp3.clone())?;
            (e_1, e_2, true)
        },
        _ => {
            (inExp1, inExp2, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oExp1, oExp2, newX))
}

fn expAddX(
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
    mut inExp3: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut ores: metamodelica::Ref<DAE::Exp>;
    ores = 'mc: {
        let __mc_input = (&**inExp1, &**inExp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e, expThen: e1, expElse: e2 }, _) => {
                    if !((expHasCref(e1.clone(), inExp3.clone())? && expHasCref(e2.clone(), inExp3.clone())? && !(expHasCref(e.clone(), inExp3.clone())?))) { return Err("guard") }
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    e3 = expAddX(inExp2, metamodelica::AsArg::as_arg(&e1), inExp3)?;
                    e4 = expAddX(inExp2, metamodelica::AsArg::as_arg(&e2), inExp3)?;
                    res = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e3.clone(), expElse: e4.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::IFEXP { expCond: e, expThen: e1, expElse: e2 }) => {
                    if !((expHasCref(e1.clone(), inExp3.clone())? && expHasCref(e2.clone(), inExp3.clone())? && !(expHasCref(e.clone(), inExp3.clone())?))) { return Err("guard") }
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    e3 = expAddX(inExp1, metamodelica::AsArg::as_arg(&e1), inExp3)?;
                    e4 = expAddX(inExp1, metamodelica::AsArg::as_arg(&e2), inExp3)?;
                    res = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e3.clone(), expElse: e4.clone() });
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
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    res = expAddX2(inExp1.clone(), inExp2, inExp3.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ores)
}

fn expAddX2(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut ores: metamodelica::Ref<DAE::Exp>;
    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut e0: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut neg: bool;
    let mut factorWithX1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut factorWithoutX1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut factorWithX2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut factorWithoutX2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut pWithX1: metamodelica::Ref<DAE::Exp>;
    let mut pWithoutX1: metamodelica::Ref<DAE::Exp>;
    let mut pWithX2: metamodelica::Ref<DAE::Exp>;
    let mut pWithoutX2: metamodelica::Ref<DAE::Exp>;
    (e0, e1, neg) = (match &*inExp1 {
        DAE::Exp::BINARY {
            exp1: ee1,
            operator: DAE::Operator::ADD { .. },
            exp2: ee2,
        } => (ee1.clone(), ee2.clone(), false),
        DAE::Exp::BINARY {
            exp1: ee1,
            operator: DAE::Operator::SUB { .. },
            exp2: ee2,
        } => (ee1.clone(), ee2.clone(), true),
        _ => (
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            }),
            inExp1,
            false,
        ),
    });
    f1 = Expression::expandFactors(&e1)?;
    (factorWithX1, factorWithoutX1) = List::split1OnTrue(&f1, &expHasCref, inExp3.clone())?;
    pWithX1 = makeProductLstSort(&factorWithX1)?;
    pWithoutX1 = makeProductLstSort(&factorWithoutX1)?;
    f2 = Expression::expandFactors(inExp2)?;
    (factorWithX2, factorWithoutX2) = List::split1OnTrue(&f2, &expHasCref, inExp3)?;
    (pWithX2, _) = ExpressionSimplify::simplify1(makeProductLstSort(&factorWithX2)?)?;
    pWithoutX2 = makeProductLstSort(&factorWithoutX2)?;
    if ExpressionBasics::expEqual(&pWithX2, pWithX1.clone())? {
        if !(neg) {
            ores = Expression::expAdd(pWithoutX1, pWithoutX2)?;
        } else {
            ores = Expression::expSub(pWithoutX2, pWithoutX1)?;
        }
        ores = Expression::expMul(ores, pWithX2)?;
    } else if ExpressionBasics::expEqual(&pWithX2, Expression::negate(pWithX1.clone())?)? {
        if !(neg) {
            ores = Expression::expSub(pWithoutX2, pWithoutX1)?;
        } else {
            ores = Expression::expAdd(pWithoutX1, pWithoutX2)?;
        }
        ores = Expression::expMul(ores, pWithX2)?;
    } else {
        e1 = Expression::expMul(pWithoutX1, pWithX1)?;
        e2 = Expression::expMul(pWithoutX2, pWithX2)?;
        ores = Expression::expAdd(e1, e2)?;
    }
    ores = Expression::expAdd(e0, ores)?;
    Ok(ores)
}

pub(crate) fn collectX(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut expand: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut outLhs: metamodelica::Ref<DAE::Exp>;
    let mut outRhs: metamodelica::Ref<DAE::Exp>;
    (outLhs, outRhs) = preprocessingSolve5(inExp1, inExp3, expand)?;
    Ok((outLhs, outRhs))
}

fn preprocessingSolve5(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut expand: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut outLhs: metamodelica::Ref<DAE::Exp>;
    let mut outRhs: metamodelica::Ref<DAE::Exp>;
    let mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tmpLhs: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut b: bool;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    if expHasCref(inExp1.clone(), inExp3.clone())? {
        if expand {
            (cr, b) = Expression::expOrDerCref(&inExp3)?;
            if b {
                (lhs, rhs) = Expression::allTermsForCref(&inExp1, &cr, &Expression::expHasDerCref)?;
            } else {
                (lhs, rhs) = Expression::allTermsForCref(&inExp1, &cr, &Expression::expHasCrefNoPreOrStart)?;
            }
        } else {
            (lhs, rhs) = List::split1OnTrue(&(Expression::terms(inExp1)?), &expHasCref, inExp3.clone())?;
        }
        outLhs = metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        });
        tmpLhs = metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        });
        for mut e in &*lhs {
            if Expression::isNegativeUnary(metamodelica::AsArg::as_arg(&e)) {
                let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                    Deref @ DAE::Exp::UNARY { exp: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                e1 = metamodelica::Own::own(__pa0);
                tmpLhs = expAddX(&e1, &tmpLhs, &inExp3)?;
            } else {
                outLhs = expAddX(metamodelica::AsArg::as_arg(&e), &outLhs, &inExp3)?;
            }
        }
        outLhs = expAddX(&outLhs, &(Expression::negate(tmpLhs)?), &inExp3)?;
        outRhs = Expression::makeSum1(rhs, false)?;
        (outRhs, _) = ExpressionSimplify::simplify1(outRhs)?;
        (outLhs, _) = ExpressionSimplify::simplify1(outLhs)?;
    } else {
        outLhs = metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        });
        outRhs = inExp1;
    }
    Ok((outLhs, outRhs))
}

fn unifyFunCalls(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    let mut newX: bool;
    (oExp, _) = Expression::traverseExpTopDown(inExp1.clone(), &unifyFunCallsWork, inExp3)?;
    newX = ExpressionBasics::expEqual(&oExp, inExp1)?;
    Ok((oExp, newX))
}

fn unifyFunCallsWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut iT: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, metamodelica::Ref<DAE::Exp>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut oT: metamodelica::Ref<DAE::Exp>;
    (outExp, cont, oT) = (::match_deref::match_deref! { match &((inExp.clone(), iT.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, _) if (!(Expression::isZero(metamodelica::AsArg::as_arg(&e1))?)) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(e1.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: Expression::makeConstZero(&tp), index: -1, optionExpisASUB: None }), expThen: Expression::expMul(e1.clone(), e2.clone())?, expElse: Expression::expMul(e1.clone(), e3.clone())? });
            (e, true, iT)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_DF$DER" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, X) if (expHasCref(e1.clone(), X.clone())?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(e1.clone())?;
            e2 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent(arcstr::literal!(BackendDAE::symSolverDT), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            e3 = Expression::makePureBuiltinCall(literal!("pre"), list![e1.clone()], tp);
            e3 = Expression::expSub(e1.clone(), e3)?;
            e = Expression::expDiv(e3, e2)?;
            (e, true, iT)
        },
        _ => {
            (inExp, true, iT)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, oT))
}

fn solveFunCalls(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut x: metamodelica::Ref<DAE::Exp>;
    let mut con: bool;
    (x, con) = 'mc: {
        let __mc_input = &*inExp1;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut funX: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    (funX, _) = Expression::traverseExpTopDown(inExp1.clone(), &fnptr!(inlineCallX, metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<AvlTreePathFunction::Tree>>)), (inExp3.clone(), functions.clone()))?;
                    b = !(ExpressionBasics::expEqual(&funX, inExp1.clone())?);
                    Ok((funX.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp1.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (x, con)
}

fn removeSimpleCalls(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, bool) {
    let mut outLhs: metamodelica::Ref<DAE::Exp>;
    let mut outRhs: metamodelica::Ref<DAE::Exp>;
    let mut con: bool;
    (outLhs, outRhs, con) = (match &*inExp1 {
        DAE::Exp::CALL { .. } => removeSimpleCalls2(inExp1, inExp2, inExp3),
        _ => (inExp1, inExp2, false),
    });
    (outLhs, outRhs, con)
}

fn removeSimpleCalls2(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
) -> (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, bool) {
    let mut outLhs: metamodelica::Ref<DAE::Exp>;
    let mut outRhs: metamodelica::Ref<DAE::Exp>;
    let mut con: bool;
    (outLhs, outRhs, con) = 'mc: {
        let __mc_input = (&*inExp1, &*inExp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let true = (!(Expression::isCref(&inExp2) || Expression::isConst(inExp2.clone())?)) else { return Err("pattern mismatch") };
                    e2 = Expression::expAdd(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), inExp2.clone())?;
                    e3 = Expression::expSub(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), inExp2.clone())?;
                    e2 = Expression::makeDiv(e2.clone(), e3.clone())?;
                    e2 = Expression::makePureBuiltinCall(literal!("log"), list![e2.clone()], DAE::T_REAL_DEFAULT().clone());
                    e2 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), e2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let true = (!(Expression::isCref(&inExp2) || Expression::isConst(inExp2.clone())?)) else { return Err("pattern mismatch") };
                    e2 = Expression::expPow(inExp2.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
                    e3 = Expression::expAdd(e2.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
                    e2 = Expression::makePureBuiltinCall(literal!("sqrt"), list![e3.clone()], DAE::T_REAL_DEFAULT().clone());
                    e3 = Expression::expAdd(inExp2.clone(), e2.clone())?;
                    e2 = Expression::makePureBuiltinCall(literal!("log"), list![e3.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    e2 = Expression::expPow(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) }), inExp2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    e2 = Expression::makePureBuiltinCall(literal!("exp"), list![inExp2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    e2 = Expression::makePureBuiltinCall(literal!("log"), list![inExp2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { return Err("pattern mismatch") };
                    e2 = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) });
                    e2 = Expression::expPow(inExp2.clone(), e2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: __rlit_17 }, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_18 }) => {
                    if !(__rlit_17.eq(&metamodelica::OrderedFloat((0.0) as f64)) && __rlit_18.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp1.clone(), inExp2.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outLhs, outRhs, con)
}

fn inlineCallX(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut iT: (
        metamodelica::Ref<DAE::Exp>,
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        metamodelica::Ref<DAE::Exp>,
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut oT: (
        metamodelica::Ref<DAE::Exp>,
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    );
    (outExp, cont, oT) = 'mc: {
        let __mc_input = (&*inExp, &iT);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { .. }, (X, functions)) => {
                    if !((expHasCref(inExp.clone(), X.clone())?)) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    (e, _, b) = Inline::forceInlineExp(inExp.clone(), (functions.clone(), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone(), &Ceval::cevalSimpleWithFunctionTreeReturnExp)?;
                    Ok((e.clone(), !(b), iT.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, iT.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, oT)
}

fn preprocessingSolveTmpVars(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut optCond: Option<metamodelica::Ref<DAE::Exp>>,
    mut uniqueEqIndex: i32,
    mut ieqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inewVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut idepth: i32,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    bool,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
) {
    let mut x: metamodelica::Ref<DAE::Exp>;
    let mut y: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut new_x: bool = false;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut odepth: i32 = 0;
    (x, y, new_x, eqnForNewVars, newVarsCrefs, odepth) = 'mc: {
        let __mc_input = &*inExp1;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, expLst: Deref @ metamodelica::ListNode::Cons { head: arg, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                            if !((expHasCref(arg.clone(), inExp3.clone())? && !(expHasCref(inExp2.clone(), inExp3.clone())?))) { return Err("guard") }
                            let mut eqnForNewVars_: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                            let mut newVarsCrefs_: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut new_x: bool = new_x.clone();
                            let mut odepth: i32 = odepth.clone();
                            let mut y: metamodelica::Ref<DAE::Exp> = y.clone();
                            (y, new_x, eqnForNewVars_, newVarsCrefs_, odepth) = preprocessingSolveFunctionCall(metamodelica::AsArg::as_arg(&name), arg.clone(), inExp2.clone(), inExp3.clone(), optCond.clone(), uniqueEqIndex, idepth)?;
                            if (eqnForNewVars_).is_empty() {
                                eqnForNewVars_ = ieqnForNewVars.clone();
                            } else {
                                eqnForNewVars_ = (::match_deref::match_deref! { match &(&optCond) {
                Some(cond) => {
                            metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: list![cond.clone()], eqnstrue: list![eqnForNewVars_.clone()], eqnsfalse: metamodelica::nil(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), ieqnForNewVars.clone())
                },
                _ => {
                            listAppend(eqnForNewVars_.clone(), ieqnForNewVars.clone())
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            }
                            Ok(((if (new_x) {arg.clone()} else {inExp1.clone()}, y.clone(), new_x, eqnForNewVars_.clone(), listAppend(newVarsCrefs_.clone(), inewVarsCrefs.clone()), odepth), new_x.clone(), odepth.clone(), y.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            new_x = __wb0;
            odepth = __wb1;
            y = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } => {
                    if !((expHasCref(e1.clone(), inExp3.clone())? && !(expHasCref(e2.clone(), inExp3.clone())?))) { return Err("guard") }
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut exP: metamodelica::Ref<DAE::Exp>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut eqnForNewVars_: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut newVarsCrefs_: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut tp = (*tp).clone();
                    tp = Expression::r#typeof(e1.clone())?;
                    exP = makeInitialGuess(tp.clone(), inExp3.clone(), e1.clone())?;
                    (exP, eqnForNewVars_, newVarsCrefs_) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), &(literal!("X$ABS")), uniqueEqIndex, idepth, ieqnForNewVars.clone(), inewVarsCrefs.clone(), false)?;
                    e_1 = Expression::makePureBuiltinCall(literal!("$_signNoNull"), list![exP.clone()], tp.clone());
                    lhs = Expression::expPow(inExp2.clone(), Expression::inverseFactors(e2.clone())?)?;
                    lhs = Expression::makePureBuiltinCall(literal!("abs"), list![lhs.clone()], tp.clone());
                    lhs = Expression::expMul(e_1.clone(), lhs.clone())?;
                    Ok((e1.clone(), lhs.clone(), true, eqnForNewVars_.clone(), newVarsCrefs_.clone(), idepth + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: ee1, operator: op1, exp2: ee2 } => {
                    if !((Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op1)))) { return Err("guard") }
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut e6: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut a1: metamodelica::Ref<DAE::Exp>;
                    let mut x1: metamodelica::Ref<DAE::Exp>;
                    let mut a2: metamodelica::Ref<DAE::Exp>;
                    let mut x2: metamodelica::Ref<DAE::Exp>;
                    let mut z1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut z2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut z3: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut z4: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut eqnForNewVars_: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut newVarsCrefs_: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    (z1, z2) = List::split1OnTrue(&(Expression::factors(metamodelica::AsArg::as_arg(&ee1))?), &expHasCref, inExp3.clone())?;
                    (z3, z4) = List::split1OnTrue(&(Expression::factors(metamodelica::AsArg::as_arg(&ee2))?), &expHasCref, inExp3.clone())?;
                    x1 = makeProductLstSort(&z1)?;
                    a1 = makeProductLstSort(&z2)?;
                    x2 = makeProductLstSort(&z3)?;
                    a2 = if (Expression::isAdd(metamodelica::AsArg::as_arg(&op1))) {makeProductLstSort(&z4)?} else {Expression::negate(makeProductLstSort(&z4)?)?};
                    (e2, e3) = simplifyBinaryMulCoeff(x1.clone())?;
                    (e5, e6) = simplifyBinaryMulCoeff(x2.clone())?;
                    (lhs, rhs, eqnForNewVars_, newVarsCrefs_) = solveQE(a1.clone(), e2.clone(), e3.clone(), a2.clone(), e5.clone(), e6.clone(), inExp2.clone(), inExp3.clone(), ieqnForNewVars.clone(), inewVarsCrefs.clone(), uniqueEqIndex, idepth)?;
                    Ok((lhs.clone(), rhs.clone(), true, eqnForNewVars_.clone(), newVarsCrefs_.clone(), idepth + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp1.clone(), inExp2.clone(), false, ieqnForNewVars.clone(), inewVarsCrefs.clone(), idepth))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (x, y, new_x, eqnForNewVars, newVarsCrefs, odepth)
}

fn preprocessingSolveFunctionCall(
    mut name: &ArcStr,
    mut arg: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut optCond: Option<metamodelica::Ref<DAE::Exp>>,
    mut uniqueEqIndex: i32,
    mut idepth: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
)> {
    let mut result: metamodelica::Ref<DAE::Exp>;
    let mut new_x: bool;
    let mut newEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut odepth: i32;
    (result, new_x, newEqns, newVars, odepth) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "tanh" => {
            let mut y: metamodelica::Ref<DAE::Exp>;
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), &(literal!("Y$TANH")), uniqueEqIndex, idepth, metamodelica::nil(), metamodelica::nil(), false)?;
            e1 = Expression::expAdd(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), y.clone())?;
            e2 = Expression::expSub(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), y)?;
            e1 = Expression::makeDiv(e1, e2)?;
            e1 = Expression::makePureBuiltinCall(literal!("log"), list![e1], tp);
            inv = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), e1)?;
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(-1.0_f64), false)), Some((metamodelica::OrderedFloat(1.0_f64), false)))?;
            (inv, true, metamodelica::cons(ass, eqns), vars, idepth + 1)
        },
        Deref @ "sinh" => {
            let mut y: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs, tp.clone(), &(literal!("Y$SINH")), uniqueEqIndex, idepth, metamodelica::nil(), metamodelica::nil(), false)?;
            e1 = Expression::expPow(y.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            e1 = Expression::expAdd(e1, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
            e1 = Expression::makePureBuiltinCall(literal!("sqrt"), list![e1], tp.clone());
            e1 = Expression::expAdd(y, e1)?;
            e1 = Expression::makePureBuiltinCall(literal!("log"), list![e1], tp);
            (e1, true, eqns, vars, idepth + 1)
        },
        Deref @ "cosh" => {
            let mut y: metamodelica::Ref<DAE::Exp>;
            let mut exP: metamodelica::Ref<DAE::Exp>;
            let mut sgn: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), &(literal!("Y$COSH")), uniqueEqIndex, idepth, metamodelica::nil(), metamodelica::nil(), false)?;
            exP = makeInitialGuess(tp.clone(), inExp3, arg)?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP, tp.clone(), &(literal!("SIGN$COSH")), uniqueEqIndex, idepth, eqns, vars, false)?;
            sgn = Expression::makePureBuiltinCall(literal!("$_signNoNull"), list![exP], tp.clone());
            e1 = Expression::expPow(y.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            e1 = Expression::expSub(e1, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
            e1 = Expression::makePureBuiltinCall(literal!("sqrt"), list![e1], tp.clone());
            e1 = Expression::expMul(sgn, e1)?;
            e1 = Expression::expAdd(y, e1)?;
            e1 = Expression::makePureBuiltinCall(literal!("log"), list![e1], tp);
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(1.0_f64), true)), None)?;
            (e1, true, metamodelica::cons(ass, eqns), vars, idepth + 1)
        },
        Deref @ "cos" => {
            let mut y: metamodelica::Ref<DAE::Exp>;
            let mut exP: metamodelica::Ref<DAE::Exp>;
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut k1: metamodelica::Ref<DAE::Exp>;
            let mut k2: metamodelica::Ref<DAE::Exp>;
            let mut x1: metamodelica::Ref<DAE::Exp>;
            let mut x2: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), &(literal!("Y$COS")), uniqueEqIndex, idepth, metamodelica::nil(), metamodelica::nil(), false)?;
            inv = Expression::makePureBuiltinCall(literal!("acos"), list![y], tp.clone());
            (inv, eqns, vars) = makeTmpEqnAndCrefFromExp(inv, tp.clone(), &(literal!("INV$COS")), uniqueEqIndex, idepth, eqns, vars, false)?;
            exP = makeInitialGuess(tp.clone(), inExp3, arg)?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP, tp.clone(), &(literal!("PREX$COS")), uniqueEqIndex, idepth, eqns, vars, false)?;
            k1 = helpInvCos(inv.clone(), exP.clone(), tp.clone(), true)?;
            k2 = helpInvCos(inv.clone(), exP.clone(), tp.clone(), false)?;
            (k1, eqns, vars) = makeTmpEqnAndCrefFromExp(k1, tp.clone(), &(literal!("k1$COS")), uniqueEqIndex, idepth, eqns, vars, false)?;
            (k2, eqns, vars) = makeTmpEqnAndCrefFromExp(k2, tp.clone(), &(literal!("k2$COS")), uniqueEqIndex, idepth, eqns, vars, false)?;
            x1 = helpInvCos2(k1, inv.clone(), &tp, true)?;
            x2 = helpInvCos2(k2, inv, &tp, false)?;
            (x1, eqns, vars) = makeTmpEqnAndCrefFromExp(x1, tp.clone(), &(literal!("x1$COS")), uniqueEqIndex, idepth, eqns, vars, false)?;
            (x2, eqns, vars) = makeTmpEqnAndCrefFromExp(x2, tp.clone(), &(literal!("x2$COS")), uniqueEqIndex, idepth, eqns, vars, false)?;
            e1 = helpInvCos3(x1, x2, exP, tp)?;
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(-1.0_f64), true)), Some((metamodelica::OrderedFloat(1.0_f64), true)))?;
            (e1, true, metamodelica::cons(ass, eqns), vars, idepth + 1)
        },
        Deref @ "sin" => {
            let mut y: metamodelica::Ref<DAE::Exp>;
            let mut exP: metamodelica::Ref<DAE::Exp>;
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut k1: metamodelica::Ref<DAE::Exp>;
            let mut k2: metamodelica::Ref<DAE::Exp>;
            let mut x1: metamodelica::Ref<DAE::Exp>;
            let mut x2: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), &(literal!("Y$SIN")), uniqueEqIndex, idepth, metamodelica::nil(), metamodelica::nil(), false)?;
            inv = Expression::makePureBuiltinCall(literal!("asin"), list![y], tp.clone());
            (inv, eqns, vars) = makeTmpEqnAndCrefFromExp(inv, tp.clone(), &(literal!("INV$SIN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            exP = makeInitialGuess(tp.clone(), inExp3, arg.clone())?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP, tp.clone(), &(literal!("PREX$SIN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            k1 = helpInvSin(inv.clone(), arg.clone(), tp.clone(), true)?;
            k2 = helpInvSin(inv.clone(), arg, tp.clone(), false)?;
            (k1, eqns, vars) = makeTmpEqnAndCrefFromExp(k1, tp.clone(), &(literal!("k1$SIN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            (k2, eqns, vars) = makeTmpEqnAndCrefFromExp(k2, tp.clone(), &(literal!("k2$SIN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            x1 = helpInvSin2(k1, inv.clone(), &tp, true)?;
            x2 = helpInvSin2(k2, inv, &tp, false)?;
            (x1, eqns, vars) = makeTmpEqnAndCrefFromExp(x1, tp.clone(), &(literal!("x1$SIN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            (x2, eqns, vars) = makeTmpEqnAndCrefFromExp(x2, tp.clone(), &(literal!("x2$SIN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            e1 = helpInvCos3(x1, x2, exP, tp)?;
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(-1.0_f64), true)), Some((metamodelica::OrderedFloat(1.0_f64), true)))?;
            (e1, true, metamodelica::cons(ass, eqns), vars, idepth + 1)
        },
        Deref @ "tan" => {
            let mut y: metamodelica::Ref<DAE::Exp>;
            let mut exP: metamodelica::Ref<DAE::Exp>;
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut k1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs, tp.clone(), &(literal!("Y$TAN")), uniqueEqIndex, idepth, metamodelica::nil(), metamodelica::nil(), false)?;
            inv = Expression::makePureBuiltinCall(literal!("atan"), list![y], tp.clone());
            (inv, eqns, vars) = makeTmpEqnAndCrefFromExp(inv, tp.clone(), &(literal!("INV$TAN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            exP = makeInitialGuess(tp.clone(), inExp3, arg)?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP, tp.clone(), &(literal!("PREX$TAN")), uniqueEqIndex, idepth, eqns, vars, false)?;
            k1 = Expression::expSub(exP, inv.clone())?;
            k1 = Expression::makeDiv(k1, DAE::PI().clone())?;
            k1 = Expression::makePureBuiltinCall(literal!("$_round"), list![k1], tp);
            e1 = Expression::expMul(k1, DAE::PI().clone())?;
            e1 = Expression::expAdd(inv, e1)?;
            (e1, true, eqns, vars, idepth + 1)
        },
        Deref @ "abs" => {
            let mut exP: metamodelica::Ref<DAE::Exp>;
            let mut sgn: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(arg.clone())?;
            exP = makeInitialGuess(tp.clone(), inExp3, arg)?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP, tp.clone(), &(literal!("SIGN$ABS")), uniqueEqIndex, idepth, metamodelica::nil(), metamodelica::nil(), false)?;
            sgn = Expression::makePureBuiltinCall(literal!("$_signNoNull"), list![exP], tp);
            e1 = Expression::expMul(sgn, rhs.clone())?;
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(0.0_f64), true)), None)?;
            (e1, true, metamodelica::cons(ass, eqns), vars, idepth + 1)
        },
        Deref @ "sqrt" => {
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            inv = Expression::expPow(rhs.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(0.0_f64), true)), None)?;
            (inv, true, list![ass], metamodelica::nil(), idepth + 1)
        },
        Deref @ "asin" => {
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall(literal!("sin"), list![rhs.clone()], tp);
            ass = makeDomainAssert(name, rhs, Some((-(metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(&(DAE::PI().clone()))?), true)), Some((metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(&(DAE::PI().clone()))?, true)))?;
            (inv, true, list![ass], metamodelica::nil(), idepth + 1)
        },
        Deref @ "acos" => {
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall(literal!("cos"), list![rhs.clone()], tp);
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(0.0_f64), true)), Some((Expression::toReal(&(DAE::PI().clone()))?, true)))?;
            (inv, true, list![ass], metamodelica::nil(), idepth + 1)
        },
        Deref @ "atan" => {
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall(literal!("tan"), list![rhs.clone()], tp);
            ass = makeDomainAssert(name, rhs, Some((-(metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(&(DAE::PI().clone()))?), true)), Some((metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(&(DAE::PI().clone()))?, true)))?;
            (inv, true, list![ass], metamodelica::nil(), idepth + 1)
        },
        Deref @ "exp" => {
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ass: metamodelica::Ref<BackendDAE::Equation>;
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall(literal!("log"), list![rhs.clone()], tp);
            ass = makeDomainAssert(name, rhs, Some((metamodelica::OrderedFloat(0.0_f64), false)), None)?;
            (inv, true, list![ass], metamodelica::nil(), idepth + 1)
        },
        Deref @ "log" => {
            let mut inv: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall(literal!("exp"), list![rhs], tp);
            (inv, true, metamodelica::nil(), metamodelica::nil(), idepth + 1)
        },
        Deref @ "log10" => {
            let mut inv: metamodelica::Ref<DAE::Exp>;
            inv = Expression::expPow(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) }), rhs)?;
            (inv, true, metamodelica::nil(), metamodelica::nil(), idepth + 1)
        },
        Deref @ "sign" => {
            (rhs, false, metamodelica::nil(), metamodelica::nil(), idepth)
        },
        Deref @ "$_DF$DER" => {
            let mut exP: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            e1 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent(arcstr::literal!(BackendDAE::symSolverDT), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            exP = Expression::makePureBuiltinCall(literal!("pre"), list![arg.clone()], Expression::r#typeof(arg)?);
            e1 = Expression::expAdd(Expression::expMul(rhs, e1)?, exP)?;
            (e1, true, metamodelica::nil(), metamodelica::nil(), idepth + 1)
        },
        _ => {
            (rhs, false, metamodelica::nil(), metamodelica::nil(), idepth)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((result, new_x, newEqns, newVars, odepth))
}

fn simplifyBinaryMulCoeff(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut exp2: metamodelica::Ref<DAE::Exp>;
    (exp1, exp2) = (::match_deref::match_deref! { match &(inExp.clone()) {
        e @ Deref @ DAE::Exp::CREF { .. } => {
            (e.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: coeff } } => {
            (e1.clone(), Expression::negate(coeff.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: coeff } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) => {
            (e1.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } if (Expression::isOne(metamodelica::AsArg::as_arg(&e1))) => {
            (e2.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1.0_f64) }))
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            (e.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }))
        },
        _ => {
            (inExp, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp1, exp2))
}

fn solveQE(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut e3: metamodelica::Ref<DAE::Exp>,
    mut e4: metamodelica::Ref<DAE::Exp>,
    mut e5: metamodelica::Ref<DAE::Exp>,
    mut e6: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut ieqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inewVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut uniqueEqIndex: i32,
    mut idepth: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut e7: metamodelica::Ref<DAE::Exp>;
    let mut con: metamodelica::Ref<DAE::Exp>;
    let mut invExp: metamodelica::Ref<DAE::Exp>;
    let mut x1: metamodelica::Ref<DAE::Exp>;
    let mut x2: metamodelica::Ref<DAE::Exp>;
    let mut x: metamodelica::Ref<DAE::Exp>;
    let mut exP: metamodelica::Ref<DAE::Exp>;
    let mut a: metamodelica::Ref<DAE::Exp>;
    let mut b: metamodelica::Ref<DAE::Exp>;
    let mut c: metamodelica::Ref<DAE::Exp>;
    let mut n: metamodelica::Ref<DAE::Exp>;
    let mut sgnb: metamodelica::Ref<DAE::Exp>;
    let mut b2: metamodelica::Ref<DAE::Exp>;
    let mut ac: metamodelica::Ref<DAE::Exp>;
    let mut sExp1: metamodelica::Ref<DAE::Exp>;
    let mut sExp2: metamodelica::Ref<DAE::Exp>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut b1: bool;
    let mut b3: bool;
    let false = (Expression::isZero(&e1)? && Expression::isZero(&e2)?) else {
        return Err("pattern mismatch");
    };
    let true = (ExpressionBasics::expEqual(&e2, e5.clone())?) else {
        return Err("pattern mismatch");
    };
    b1 = ExpressionBasics::expEqual(
        &e3,
        Expression::expMul(
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(2.0_f64),
            }),
            e6.clone(),
        )?,
    )?;
    b3 = ExpressionBasics::expEqual(
        &e6,
        Expression::expMul(
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(2.0_f64),
            }),
            e3.clone(),
        )?,
    )?;
    let true = (b1 || b3) else {
        return Err("pattern mismatch");
    };
    let false = (expHasCref(e1.clone(), inExp3.clone())?) else {
        return Err("pattern mismatch");
    };
    let true = (expHasCref(e2.clone(), inExp3.clone())?) else {
        return Err("pattern mismatch");
    };
    let false = (expHasCref(e3.clone(), inExp3.clone())?) else {
        return Err("pattern mismatch");
    };
    let false = (expHasCref(e4.clone(), inExp3.clone())?) else {
        return Err("pattern mismatch");
    };
    let true = (expHasCref(e5, inExp3.clone())?) else {
        return Err("pattern mismatch");
    };
    let false = (expHasCref(e6.clone(), inExp3.clone())?) else {
        return Err("pattern mismatch");
    };
    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else {
        return Err("pattern mismatch");
    };
    a = if (b1) { e1.clone() } else { e4.clone() };
    b = if (b1) { e4 } else { e1 };
    c = Expression::negate(inExp2.clone())?;
    n = if (b1) { e6.clone() } else { e3.clone() };
    tp = Expression::r#typeof(a.clone())?;
    (a, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        a,
        tp.clone(),
        &(literal!("a$QE")),
        uniqueEqIndex,
        idepth,
        ieqnForNewVars,
        inewVarsCrefs,
        false,
    )?;
    con = metamodelica::Ref::new(DAE::Exp::RELATION {
        exp1: a.clone(),
        operator: DAE::Operator::EQUAL { ty: tp },
        exp2: metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
        index: -1,
        optionExpisASUB: None,
    });
    tp = Expression::r#typeof(b.clone())?;
    (b, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        b,
        tp.clone(),
        &(literal!("b$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    sgnb = Expression::makePureBuiltinCall(literal!("$_signNoNull"), list![b.clone()], tp.clone());
    b2 = Expression::expPow(
        b.clone(),
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(2.0_f64),
        }),
    )?;
    (b2, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        b2,
        tp,
        &(literal!("bPow2$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    tp = Expression::r#typeof(c.clone())?;
    (c, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        c,
        tp.clone(),
        &(literal!("c$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    ac = Expression::expMul(a.clone(), c.clone())?;
    ac = Expression::expMul(
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(4.0_f64),
        }),
        ac,
    )?;
    sExp1 = Expression::expSub(b2, ac)?;
    sExp2 = Expression::makePureBuiltinCall(literal!("sqrt"), list![sExp1], tp.clone());
    sExp2 = Expression::expMul(sgnb, sExp2)?;
    a = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: con.clone(),
        expThen: Expression::makeConstOne(&tp),
        expElse: a,
    });
    (a, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        a,
        tp,
        &(literal!("a1$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    x1 = Expression::expAdd(b.clone(), sExp2)?;
    x1 = Expression::makeDiv(x1, a.clone())?;
    x1 = Expression::expMul(
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(-0.5_f64),
        }),
        x1,
    )?;
    tp = Expression::r#typeof(x1.clone())?;
    x1 = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: con.clone(),
        expThen: Expression::makeConstOne(&tp),
        expElse: x1,
    });
    (x1, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        x1,
        tp.clone(),
        &(literal!("x1$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    x2 = Expression::expMul(a, x1.clone())?;
    x2 = Expression::makeDiv(c, x2)?;
    x2 = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: con.clone(),
        expThen: Expression::makeConstOne(&tp),
        expElse: x2,
    });
    x2 = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: x1.clone(),
            operator: DAE::Operator::EQUAL { ty: tp.clone() },
            exp2: metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            }),
            index: -1,
            optionExpisASUB: None,
        }),
        expThen: metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
        expElse: x2,
    });
    (x2, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        x2,
        tp,
        &(literal!("x2$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    tp = Expression::r#typeof(e2.clone())?;
    exP = makeInitialGuess(tp.clone(), inExp3, e2.clone())?;
    (exP, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        exP,
        tp.clone(),
        &(literal!("prex$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    x = helpInvCos3(x1, x2, exP, tp.clone())?;
    (x, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(
        x,
        tp,
        &(literal!("x$QE")),
        uniqueEqIndex,
        idepth,
        eqnForNewVars,
        newVarsCrefs,
        false,
    )?;
    e7 = Expression::makeDiv(inExp2, b)?;
    invExp = Expression::inverseFactors(n)?;
    (invExp, _) = ExpressionSimplify::simplify1(invExp)?;
    e7 = Expression::expPow(e7, invExp)?;
    rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: con,
        expThen: e7,
        expElse: x,
    });
    lhs = if (b1) {
        Expression::expPow(e2, e6)?
    } else {
        Expression::expPow(e2, e3)?
    };
    Ok((rhs, lhs, eqnForNewVars, newVarsCrefs))
}

fn solveIfExp(
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut inCond: Option<metamodelica::Ref<DAE::Exp>>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    mut uniqueEqIndex: Option<i32>,
    mut idepth: i32,
    mut doInline: bool,
    mut isContinuousIntegration: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut odepth: i32;
    (outExp, outAsserts, eqnForNewVars, newVarsCrefs, odepth) = (match &**inExp1 {
        DAE::Exp::IFEXP {
            expCond: eCond,
            expThen: eThen,
            expElse: eElse,
        } if (isContinuousIntegration || !(expHasCref(eCond.clone(), inExp3.clone())?)) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            let mut rhs: metamodelica::Ref<DAE::Exp>;
            let mut cond1: metamodelica::Ref<DAE::Exp>;
            let mut cond2: metamodelica::Ref<DAE::Exp>;
            let mut asserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut asserts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut eqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut var: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut var1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut depth: i32;
            (cond1, cond2) = (::match_deref::match_deref! { match &(inCond) {
                Some(theCond) => {
                    (metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: theCond.clone(), operator: DAE::Operator::AND { ty: Expression::r#typeof(eCond.clone())? }, exp2: eCond.clone() }), metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: theCond.clone(), operator: DAE::Operator::AND { ty: Expression::r#typeof(eCond.clone())? }, exp2: Expression::negate(eCond.clone())? }))
                },
                _ => {
                    (eCond.clone(), Expression::negate(eCond.clone())?)
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            (lhs, asserts1, eqns, var, depth) = solveWork(
                eThen.clone(),
                inExp2.clone(),
                inExp3.clone(),
                Some(cond1),
                functions.clone(),
                uniqueEqIndex.clone(),
                idepth,
                doInline,
                isContinuousIntegration,
            )?;
            (rhs, _, eqns1, var1, depth) = solveWork(
                eElse.clone(),
                inExp2,
                inExp3.clone(),
                Some(cond2),
                functions,
                uniqueEqIndex,
                depth,
                doInline,
                isContinuousIntegration,
            )?;
            res = metamodelica::Ref::new(DAE::Exp::IFEXP {
                expCond: eCond.clone(),
                expThen: lhs,
                expElse: rhs,
            });
            asserts = listAppend(asserts1.clone(), asserts1);
            (res, asserts, listAppend(eqns1, eqns), listAppend(var1, var), depth)
        }
        _ => return Err("fail"),
    });
    Ok((outExp, outAsserts, eqnForNewVars, newVarsCrefs, odepth))
}

fn solveLinearSystem(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    mut idepth: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut eqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut newVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut odepth: i32 = idepth;
    (outExp, outAsserts) = (match &*inExp3 {
        DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut dere: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut z: metamodelica::Ref<DAE::Exp>;
            let mut rhs: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut i: i32;
            let false = (hasOnlyFactors(inExp1.clone(), inExp2.clone())) else {
                return Err("pattern mismatch");
            };
            e = Expression::expSub(inExp1, inExp2)?;
            (e, _) = ExpressionSimplify::simplify1(e)?;
            dere = Differentiate::differentiateExpSolve(e.clone(), cr.clone(), functions)?;
            (dere, _) = ExpressionSimplify::simplify(dere)?;
            let false = (Expression::isZero(&dere)?) else {
                return Err("pattern mismatch");
            };
            let false = (Expression::expHasCrefNoPreOrStart(dere.clone(), cr.clone())?) else {
                return Err("pattern mismatch");
            };
            tp = Expression::r#typeof(inExp3.clone())?;
            (z, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            (e, i) = Expression::replaceExp(e, inExp3, z)?;
            if i < 1 {
                return Err("fail");
            }
            (e, _) = ExpressionSimplify::simplify(e)?;
            rhs = Expression::negate(Expression::makeDiv(e, dere)?)?;
            (rhs, metamodelica::nil())
        }
        _ => return Err("fail"),
    });
    Ok((outExp, outAsserts, eqnForNewVars, newVarsCrefs, odepth))
}

fn hasOnlyFactors(mut e1: metamodelica::Ref<DAE::Exp>, mut e2: metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = &*e2;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::factors(&e2)?) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(e2.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::factors(&e1)?) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(e1.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res
}

fn expHasCref(mut inExp1: metamodelica::Ref<DAE::Exp>, mut inExp3: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(inExp3.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            Expression::expHasCrefNoPreOrStart(inExp1, cr.clone())?
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            Expression::expHasDerCref(inExp1, cr.clone())?
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                metamodelica::print(literal!("\n-ExpressionSolve.solve failed:"));
                metamodelica::print(literal!(" with respect to: "));
                metamodelica::print(ExpressionBasics::printExpStr(inExp3)?);
                metamodelica::print(literal!(" not support!"));
                metamodelica::print(literal!("\n"));
            }
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn makeProductLstSort(
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut expLstDiv: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expLst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut op: DAE::Operator;
    if (inExpLst).is_empty() {
        outExp = metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(1.0_f64),
        });
        return Ok(outExp);
    }
    tp = Expression::r#typeof((inExpLst).head().cloned()?)?;
    (expLstDiv, expLst) = List::splitOnTrue(
        inExpLst,
        &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Expression::isDivBinary(&__a0))
        },
    )?;
    outExp = makeProductLstSort2(expLst, &tp)?;
    if !((expLstDiv).is_empty()) {
        expLst2 = metamodelica::nil();
        expLst = metamodelica::nil();
        for mut elem in &*expLstDiv {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elem.clone()) {
                Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            expLst = metamodelica::cons(e1, expLst);
            expLst2 = metamodelica::cons(e2, expLst2);
        }
        if !((expLst2).is_empty()) {
            e = makeProductLstSort(&expLst2)?;
            if !(Expression::isOne(&e)) {
                outExp = Expression::makeDiv(outExp, e)?;
            }
        }
        if !((expLst).is_empty()) {
            e = makeProductLstSort(&expLst)?;
            outExp = Expression::expMul(outExp, e)?;
        }
    }
    Ok(outExp)
}

fn makeProductLstSort2(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut tp: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = Expression::makeConstOne(tp);
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    rest = ExpressionSimplify::simplifyList(inExpLst)?;
    for mut elem in &*rest {
        if !(Expression::isOne(metamodelica::AsArg::as_arg(&elem))) {
            outExp = (match &*elem.clone() {
                DAE::Exp::IFEXP {
                    expCond: e1,
                    expThen: e2,
                    expElse: e3,
                } => metamodelica::Ref::new(DAE::Exp::IFEXP {
                    expCond: e1.clone(),
                    expThen: Expression::expMul(outExp.clone(), e2.clone())?,
                    expElse: Expression::expMul(outExp, e3.clone())?,
                }),
                _ => Expression::expMul(outExp, elem.clone())?,
            });
        }
    }
    Ok(outExp)
}

fn makeTmpEqnAndCrefFromExp(
    mut iExp: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
    mut name: &ArcStr,
    mut index1: i32,
    mut index2: i32,
    mut ieqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inewVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut need: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    let mut oeqnForNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut onewVarsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    (oExp, _) = ExpressionSimplify::simplify1(iExp)?;
    if need || !(Expression::isCref(&oExp) || Expression::isConst(oExp.clone())?) {
        cr = ComponentReferenceBasics::makeCrefIdent(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("$TMP$VAR$"));
                __mm_s.push_str(&*intString(index1));
                __mm_s.push_str(&*literal!("$"));
                __mm_s.push_str(&*intString(index2));
                __mm_s.push_str(&*name);
                ArcStr::from(__mm_s)
            },
            tp,
            metamodelica::nil(),
        );
        eqn = metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION {
            componentRef: cr.clone(),
            exp: oExp,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
        });
        oExp = Expression::crefExp(cr.clone())?;
        oeqnForNewVars = metamodelica::cons(eqn, ieqnForNewVars);
        onewVarsCrefs = metamodelica::cons(cr, inewVarsCrefs);
    } else {
        oeqnForNewVars = ieqnForNewVars;
        onewVarsCrefs = inewVarsCrefs;
    }
    Ok((oExp, oeqnForNewVars, onewVarsCrefs))
}

fn makeDomainAssert(
    mut name: &ArcStr,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut lowerBound: Option<(metamodelica::Real, bool)>,
    mut upperBound: Option<(metamodelica::Real, bool)>,
) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut assEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut msg: ArcStr;
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let mut algo: metamodelica::Ref<DAE::Algorithm>;
    let mut tp: metamodelica::Ref<DAE::Type> = Expression::r#typeof(rhs.clone())?;
    (msg, cond) = (match (lowerBound, upperBound) {
        (Some((mut lower, true)), Some((mut upper, true))) => {
            let mut r#str: ArcStr;
            let mut l: metamodelica::Ref<DAE::Exp>;
            let mut u: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" outside the range "));
                __mm_s.push_str(&*realString(lower.clone()));
                __mm_s.push_str(&*literal!(" <= "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" <= "));
                __mm_s.push_str(&*realString(upper.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            l = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: lower.clone() }),
                operator: DAE::Operator::LESSEQ { ty: tp.clone() },
                exp2: rhs.clone(),
                index: -1,
                optionExpisASUB: None,
            });
            u = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: rhs,
                operator: DAE::Operator::LESSEQ { ty: tp.clone() },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: upper.clone() }),
                index: -1,
                optionExpisASUB: None,
            });
            (
                r#str,
                metamodelica::Ref::new(DAE::Exp::LBINARY {
                    exp1: l,
                    operator: DAE::Operator::AND { ty: tp },
                    exp2: u,
                }),
            )
        }
        (Some((mut lower, true)), Some((mut upper, false))) => {
            let mut r#str: ArcStr;
            let mut l: metamodelica::Ref<DAE::Exp>;
            let mut u: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" outside the range "));
                __mm_s.push_str(&*realString(lower.clone()));
                __mm_s.push_str(&*literal!(" <= "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" < "));
                __mm_s.push_str(&*realString(upper.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            l = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: lower.clone() }),
                operator: DAE::Operator::LESSEQ { ty: tp.clone() },
                exp2: rhs.clone(),
                index: -1,
                optionExpisASUB: None,
            });
            u = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: rhs,
                operator: DAE::Operator::LESS { ty: tp.clone() },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: upper.clone() }),
                index: -1,
                optionExpisASUB: None,
            });
            (
                r#str,
                metamodelica::Ref::new(DAE::Exp::LBINARY {
                    exp1: l,
                    operator: DAE::Operator::AND { ty: tp },
                    exp2: u,
                }),
            )
        }
        (Some((mut lower, false)), Some((mut upper, true))) => {
            let mut r#str: ArcStr;
            let mut l: metamodelica::Ref<DAE::Exp>;
            let mut u: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" outside the range "));
                __mm_s.push_str(&*realString(lower.clone()));
                __mm_s.push_str(&*literal!(" < "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" <= "));
                __mm_s.push_str(&*realString(upper.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            l = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: lower.clone() }),
                operator: DAE::Operator::LESS { ty: tp.clone() },
                exp2: rhs.clone(),
                index: -1,
                optionExpisASUB: None,
            });
            u = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: rhs,
                operator: DAE::Operator::LESSEQ { ty: tp.clone() },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: upper.clone() }),
                index: -1,
                optionExpisASUB: None,
            });
            (
                r#str,
                metamodelica::Ref::new(DAE::Exp::LBINARY {
                    exp1: l,
                    operator: DAE::Operator::AND { ty: tp },
                    exp2: u,
                }),
            )
        }
        (Some((mut lower, false)), Some((mut upper, false))) => {
            let mut r#str: ArcStr;
            let mut l: metamodelica::Ref<DAE::Exp>;
            let mut u: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" outside the range "));
                __mm_s.push_str(&*realString(lower.clone()));
                __mm_s.push_str(&*literal!(" < "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" < "));
                __mm_s.push_str(&*realString(upper.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            l = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: lower.clone() }),
                operator: DAE::Operator::LESS { ty: tp.clone() },
                exp2: rhs.clone(),
                index: -1,
                optionExpisASUB: None,
            });
            u = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: rhs,
                operator: DAE::Operator::LESS { ty: tp.clone() },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: upper.clone() }),
                index: -1,
                optionExpisASUB: None,
            });
            (
                r#str,
                metamodelica::Ref::new(DAE::Exp::LBINARY {
                    exp1: l,
                    operator: DAE::Operator::AND { ty: tp },
                    exp2: u,
                }),
            )
        }
        (Some((mut lower, true)), None) => {
            let mut r#str: ArcStr;
            let mut l: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" should be "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" >= "));
                __mm_s.push_str(&*realString(lower.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            l = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: lower.clone() }),
                operator: DAE::Operator::LESSEQ { ty: tp },
                exp2: rhs,
                index: -1,
                optionExpisASUB: None,
            });
            (r#str, l)
        }
        (Some((mut lower, true)), None) => {
            let mut r#str: ArcStr;
            let mut l: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" should be "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" > "));
                __mm_s.push_str(&*realString(lower.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            l = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: lower.clone() }),
                operator: DAE::Operator::LESS { ty: tp },
                exp2: rhs,
                index: -1,
                optionExpisASUB: None,
            });
            (r#str, l)
        }
        (None, Some((mut upper, true))) => {
            let mut r#str: ArcStr;
            let mut u: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" should be "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" <= "));
                __mm_s.push_str(&*realString(upper.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            u = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: rhs,
                operator: DAE::Operator::LESSEQ { ty: tp },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: upper.clone() }),
                index: -1,
                optionExpisASUB: None,
            });
            (r#str, u)
        }
        (None, Some((mut upper, false))) => {
            let mut r#str: ArcStr;
            let mut u: metamodelica::Ref<DAE::Exp>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Model error: Result of "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" should be "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                __mm_s.push_str(&*literal!(" < "));
                __mm_s.push_str(&*realString(upper.clone()));
                __mm_s.push_str(&*literal!(". Unable to invert."));
                ArcStr::from(__mm_s)
            };
            u = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: rhs,
                operator: DAE::Operator::LESS { ty: tp },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: upper.clone() }),
                index: -1,
                optionExpisASUB: None,
            });
            (r#str, u)
        }
        _ => return Err("match: no arm matched"),
    });
    algo = metamodelica::Ref::new(DAE::Algorithm {
        statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_ASSERT {
            cond: cond,
            msg: metamodelica::Ref::new(DAE::Exp::SCONST { string: msg }),
            level: DAE::ASSERTIONLEVEL_ERROR().clone(),
            source: DAE::emptyElementSource().clone()
        })],
    });
    assEq = metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM {
        size: 0,
        alg: algo,
        source: DAE::emptyElementSource().clone(),
        expand: openmodelica_frontend_types::DAE::Expand::EXPAND,
        attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
    });
    Ok(assEq)
}

fn makeInitialGuess(
    mut tp: metamodelica::Ref<DAE::Type>,
    mut iExp1: metamodelica::Ref<DAE::Exp>,
    mut iExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    let mut con: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    con = Expression::makePureBuiltinCall(literal!("initial"), metamodelica::nil(), tp.clone());
    (e, _) = Expression::traverseExpBottomUp(
        iExp2.clone(),
        &makeInitialGuess2,
        (iExp1.clone(), literal!("pre"), tp.clone(), true),
    )?;
    (oExp, _) = Expression::traverseExpBottomUp(iExp2, &makeInitialGuess2, (iExp1, literal!("pre"), tp, false))?;
    oExp = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: con,
        expThen: e,
        expElse: oExp,
    });
    Ok(oExp)
}

fn makeInitialGuess2(
    mut iExp: metamodelica::Ref<DAE::Exp>,
    mut itpl: (metamodelica::Ref<DAE::Exp>, ArcStr, metamodelica::Ref<DAE::Type>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::Ref<DAE::Exp>, ArcStr, metamodelica::Ref<DAE::Type>, bool),
)> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    let mut otpl: (metamodelica::Ref<DAE::Exp>, ArcStr, metamodelica::Ref<DAE::Type>, bool) = itpl.clone();
    oExp = (::match_deref::match_deref! { match &((iExp.clone(), itpl)) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, fun, tp, _)) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::makePureBuiltinCall(fun.clone(), list![iExp], tp.clone());
            e
        },
        (_, (_, _, tp, true)) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            match '__try0: {
                let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(makeInitialGuess3(iExp.clone(), tp.clone()), '__try0)) {
                    Some(__pa1) => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa1);
                Ok::<_, &'static str>((e.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    e = __try0_o0;
                }
                Err(_) => {
                    e = iExp.clone();
                }
            }
            e
        },
        _ => {
            iExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oExp, otpl))
}

fn makeInitialGuess3(
    mut iExp: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut oExp: Option<metamodelica::Ref<DAE::Exp>>;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut con: metamodelica::Ref<DAE::Exp>;
            let mut o: metamodelica::Ref<DAE::Exp>;
            con = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: con, expThen: metamodelica::Ref::new(DAE::Exp::RCONST { real: -(metamodelica::real_div_checked(metamodelica::OrderedFloat((1) as f64), metamodelica::OrderedFloat(0.000000001_f64))?) }), expElse: iExp });
            Some(o)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut con: metamodelica::Ref<DAE::Exp>;
            let mut o: metamodelica::Ref<DAE::Exp>;
            con = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: con, expThen: metamodelica::Ref::new(DAE::Exp::RCONST { real: -(metamodelica::real_div_checked(metamodelica::OrderedFloat((1) as f64), metamodelica::OrderedFloat(0.000000001_f64))?) }), expElse: iExp });
            Some(o)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut con: metamodelica::Ref<DAE::Exp>;
            let mut o: metamodelica::Ref<DAE::Exp>;
            con = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: con, expThen: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: iExp });
            Some(o)
        },
        Deref @ DAE::Exp::BINARY { exp2: e, .. } => {
            let mut con: metamodelica::Ref<DAE::Exp>;
            let mut o: metamodelica::Ref<DAE::Exp>;
            con = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::EQUAL { ty: tp }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: con, expThen: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), expElse: iExp });
            Some(o)
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

fn helpInvCos(
    mut acosy: metamodelica::Ref<DAE::Exp>,
    mut x: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
    mut neg: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut k: metamodelica::Ref<DAE::Exp>;
    k = if (neg) {
        Expression::expAdd(x, acosy)?
    } else {
        Expression::expSub(x, acosy)?
    };
    k = Expression::makeDiv(
        k,
        Expression::expMul(
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(2.0_f64),
            }),
            DAE::PI().clone(),
        )?,
    )?;
    k = Expression::makePureBuiltinCall(literal!("$_round"), list![k], tp);
    Ok(k)
}

fn helpInvSin(
    mut asiny: metamodelica::Ref<DAE::Exp>,
    mut x: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
    mut neg: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut k: metamodelica::Ref<DAE::Exp>;
    k = if (neg) {
        Expression::expAdd(x, asiny)?
    } else {
        Expression::expSub(x, asiny)?
    };
    k = Expression::makeDiv(
        k,
        Expression::expMul(
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(2.0_f64),
            }),
            DAE::PI().clone(),
        )?,
    )?;
    if neg {
        k = Expression::expSub(
            k,
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.5_f64),
            }),
        )?;
    }
    k = Expression::makePureBuiltinCall(literal!("$_round"), list![k], tp);
    Ok(k)
}

fn helpInvCos2(
    mut k: metamodelica::Ref<DAE::Exp>,
    mut acosy: metamodelica::Ref<DAE::Exp>,
    mut tp: &metamodelica::Ref<DAE::Type>,
    mut neg: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut x: metamodelica::Ref<DAE::Exp>;
    x = if (neg) { Expression::negate(acosy)? } else { acosy };
    x = Expression::expAdd(
        x,
        Expression::expMul(
            k,
            Expression::expMul(
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(2.0_f64),
                }),
                DAE::PI().clone(),
            )?,
        )?,
    )?;
    Ok(x)
}

fn helpInvSin2(
    mut k: metamodelica::Ref<DAE::Exp>,
    mut asiny: metamodelica::Ref<DAE::Exp>,
    mut tp: &metamodelica::Ref<DAE::Type>,
    mut neg: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut x: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    x = if (neg) { Expression::negate(asiny)? } else { asiny };
    e = Expression::expMul(
        k,
        Expression::expMul(
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(2.0_f64),
            }),
            DAE::PI().clone(),
        )?,
    )?;
    e = if (neg) {
        Expression::expAdd(e, DAE::PI().clone())?
    } else {
        e
    };
    x = Expression::expAdd(x, e)?;
    Ok(x)
}

fn helpInvCos3(
    mut x1: metamodelica::Ref<DAE::Exp>,
    mut x2: metamodelica::Ref<DAE::Exp>,
    mut x: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut y: metamodelica::Ref<DAE::Exp>;
    let mut diffx1: metamodelica::Ref<DAE::Exp> = absDiff(x1.clone(), x.clone(), tp.clone())?;
    let mut diffx2: metamodelica::Ref<DAE::Exp> = absDiff(x2.clone(), x.clone(), tp.clone())?;
    let mut con: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::RELATION {
        exp1: diffx1.clone(),
        operator: DAE::Operator::LESS { ty: tp.clone() },
        exp2: diffx2.clone(),
        index: -1,
        optionExpisASUB: None,
    });
    con = Expression::makeNoEvent(con);
    y = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: con,
        expThen: x1,
        expElse: x2,
    });
    Ok(y)
}

fn absDiff(
    mut x: metamodelica::Ref<DAE::Exp>,
    mut y: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut z: metamodelica::Ref<DAE::Exp>;
    z = Expression::expSub(x, y)?;
    z = Expression::makePureBuiltinCall(literal!("abs"), list![z], tp);
    Ok(z)
}
