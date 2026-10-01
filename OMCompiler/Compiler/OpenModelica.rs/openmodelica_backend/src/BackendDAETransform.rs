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
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Sorting;
use crate::SymbolicJacobian;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// strongComponents and stuff
//
// =============================================================================
pub(crate) fn strongComponentsScalar(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
)> {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    (outSystem, outComps) = 'mc: {
        let __mc_input = inSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { mT: Some(mt), matching: Deref @ BackendDAE::Matching::MATCHING { ass1, ass2, .. }, .. } => {
                    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut markarray: metamodelica::Array<i32>;
                    let mut comps_m: metamodelica::List<metamodelica::List<i32>>;
                    let mut syst = (*syst).clone();
                    let mut ass1 = (*ass1).clone();
                    comps_m = Sorting::TarjanTransposed(mt.clone(), ass2.clone())?;
                    markarray = arrayCreate(BackendEquation::getNumberOfEquations(inSystem.orderedEqs.clone()), -1);
                    comps = analyseStrongComponentsScalar(&comps_m, &inSystem, inShared.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), 1, markarray.clone())?;
                    GCExt::free(markarray.clone());
                    ass1 = varAssignmentNonScalar(ass1.clone(), mapIncRowEqn.clone())?;
                    syst = metamodelica::Ref::new(BackendDAE::EqSystem { orderedVars: syst.orderedVars.clone(), orderedEqs: syst.orderedEqs.clone(), m: None, mT: None, mapping: None, matching: metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }), stateSets: syst.stateSets.clone(), partitionKind: syst.partitionKind.clone(), removedEqs: syst.removedEqs.clone() });
                    Ok((syst.clone(), comps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function strongComponentsScalar failed (sorting strong components)"), metamodelica::sourceInfo!("BackEnd/BackendDAETransform.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outSystem, outComps))
}

pub(crate) fn eqnAssignmentNonScalar(
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut outAcc: metamodelica::Array<metamodelica::List<i32>>;
    let mut elst: metamodelica::List<i32>;
    let mut vlst: metamodelica::List<i32>;
    let mut acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in 1..=metamodelica::arrayLength(mapEqnIncRow.clone()) {
        elst = ({
            let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), i)?).clone();
            __elt
        });
        vlst = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut e in (elst).into_iter().cloned() {
                if !(metamodelica::arrayGet(ass2.clone(), e.clone())? > 0) {
                    continue;
                }
                let __x = metamodelica::arrayGet(ass2.clone(), e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        acc = metamodelica::cons(vlst, acc);
    }
    outAcc = List::listArrayReverse(acc)?;
    Ok(outAcc)
}

pub(crate) fn varAssignmentNonScalar(
    mut ass1: metamodelica::Array<i32>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut outAcc: metamodelica::Array<i32>;
    outAcc = metamodelica::arrayCreate(metamodelica::arrayLength(ass1.clone()), -1);
    for mut i in 1..=metamodelica::arrayLength(ass1.clone()) {
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(
                outAcc.clone(),
                i,
                if (metamodelica::Dangerous::arrayGetNoBoundsChecking(ass1.clone(), i) > 0) {
                    (*metamodelica::index_checked(
                        &__ab_mapIncRowEqn,
                        metamodelica::Dangerous::arrayGetNoBoundsChecking(ass1.clone(), i),
                    )?)
                    .clone()
                } else {
                    -1
                },
            )
        };
    }
    Ok(outAcc)
}

fn analyseStrongComponentsScalar(
    mut inComps: &metamodelica::List<metamodelica::List<i32>>,
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAss1: metamodelica::Array<i32>,
    mut inAss2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut imark: i32,
    mut markarray: metamodelica::Array<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>> {
    let mut outComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = metamodelica::nil();
    let mut acomp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut mark: i32 = imark;
    for mut comp in &**inComps {
        (acomp, mark) = analyseStrongComponentScalar(
            comp.clone(),
            syst,
            shared.clone(),
            inAss1.clone(),
            inAss2.clone(),
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            mark,
            markarray.clone(),
        )?;
        outComps = listAppend(acomp, outComps);
    }
    outComps = Dangerous::listReverseInPlace(outComps);
    Ok(outComps)
}

fn analyseStrongComponentScalar(
    mut inComp: metamodelica::List<i32>,
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAss1: metamodelica::Array<i32>,
    mut inAss2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut imark: i32,
    mut markarray: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>, i32)> {
    let mut outComp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut omark: i32 = imark + 1;
    let mut comp: metamodelica::List<i32>;
    let mut vlst: metamodelica::List<i32>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut vars: BackendDAE::Variables;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    match '__try0: {
        let __arc3 = &(*syst);
        let BackendDAE::EQSYSTEM {
            orderedVars: __pa1,
            orderedEqs: __pa2,
            ..
        } = &**__arc3;
        vars = metamodelica::Own::own(__pa1);
        eqns = metamodelica::Own::own(__pa2);
        vlst = unwrap_break_err!(List::map1r(inComp.clone(), &arrayGet, inAss2.clone()), '__try0);
        vlst = unwrap_break_err!(List::select1(vlst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0), '__try0);
        varlst = unwrap_break_err!(List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone()), '__try0);
        comp = unwrap_break_err!(List::map1r(inComp.clone(), &arrayGet, mapIncRowEqn.clone()), '__try0);
        comp =
            unwrap_break_err!(List::fold2(&comp, &uniqueComp, imark, markarray.clone(), metamodelica::nil()), '__try0);
        eqn_lst = unwrap_break_err!(List::map1r(comp.clone(), &BackendEquation::get, eqns.clone()), '__try0);
        outComp = unwrap_break_err!(analyseStrongComponentBlock(comp.clone(), eqn_lst.clone(), varlst.clone(), vlst.clone(), syst, shared.clone(), mapEqnIncRow.clone()), '__try0);
        Ok::<_, &'static str>((
            comp.clone(),
            eqn_lst.clone(),
            eqns.clone(),
            outComp.clone(),
            varlst.clone(),
            vars.clone(),
            vlst.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6)) => {
            comp = __try0_o0;
            eqn_lst = __try0_o1;
            eqns = __try0_o2;
            outComp = __try0_o3;
            varlst = __try0_o4;
            vars = __try0_o5;
            vlst = __try0_o6;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function analyseStrongComponentScalar failed"),
                metamodelica::sourceInfo!("BackEnd/BackendDAETransform.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok((outComp, omark))
}

fn uniqueComp(
    mut c: i32,
    mut mark: i32,
    mut markarray: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oAcc: metamodelica::List<i32> = iAcc.clone();
    if mark
        != ({
            let __elt = (*metamodelica::index_checked(&markarray.borrow(), c)?).clone();
            __elt
        })
    {
        metamodelica::arrayUpdate(markarray.clone(), c, mark)?;
        oAcc = metamodelica::cons(c, iAcc);
    }
    Ok(oAcc)
}

fn analyseStrongComponentBlock(
    mut inComp: metamodelica::List<i32>,
    mut inEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVarindxLst: metamodelica::List<i32>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>> {
    let __ab_mapEqnIncRow = mapEqnIncRow.borrow();
    let mut outComp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    outComp = 'mc: {
        let __mc_input = (inComp, inEqnLst.clone(), inVarLst, inVarindxLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compelem, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { .. }, tail: Deref @ metamodelica::ListNode::Nil }, _, varindxs) => {
                    Ok(list![metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compelem, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, var_lst, varindxs) => {
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut b1: bool;
                    crlst = List::map(var_lst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?;
                    b1 = List::applyAndFold(&crlst, &fnptr!(boolAnd, bool, bool), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(ComponentReference::isArrayElement(&__a0)) }, true)?;
                    if !(b1) {
                        expLst = List::map(crlst.clone(), &Expression::crefExp)?;
                        let true = (List::exist1(&inEqnLst, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: metamodelica::List<metamodelica::Ref<DAE::Exp>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(crefsAreArray(&__a0, __a1)) }, expLst.clone())?) else { return Err("pattern mismatch") };
                    }
                    Ok(list![metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEARRAY { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compelem, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::IF_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, _, varindxs) => {
                    Ok(list![metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compelem, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, _, varindxs) => {
                    Ok(list![metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compelem, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, _, varindxs) => {
                    Ok(list![metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compelem, tail: Deref @ metamodelica::ListNode::Nil }, _, _, Deref @ metamodelica::ListNode::Cons { head: v, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(list![metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: compelem.clone(), var: v.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comp, eqn_lst, var_lst, varindxs) => {
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
                    let mut vars_1: BackendDAE::Variables;
                    let mut eqn_lst1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut var_lst_1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqns_1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut jac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>;
                    let mut jac_tp: BackendDAE::JacobianType;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut jacConstant: bool;
                    let mut mixedSystem: bool;
                    let true = (BackendVariable::hasContinuousVar(metamodelica::AsArg::as_arg(&var_lst))) else { return Err("pattern mismatch") };
                    eqn_lst1 = BackendEquation::replaceDerOpInEquationList(metamodelica::AsArg::as_arg(&eqn_lst))?;
                    var_lst_1 = List::map(var_lst.clone(), &fnptr!(transformXToXd, metamodelica::Ref<BackendDAE::Var>))?;
                    vars_1 = BackendVariable::listVar1(&var_lst_1)?;
                    eqns_1 = BackendEquation::listEquation(&eqn_lst1)?;
                    (mixedSystem, _) = BackendEquation::iterationVarsinRelations(&eqn_lst1, vars_1.clone())?;
                    if !(Flags::isSet(Flags::DISABLE_JACSCC.clone())?) {
                        syst = BackendDAEUtil::createEqSystem(vars_1.clone(), eqns_1.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                        (m, mt) = BackendDAEUtil::adjacencyMatrix(&syst, openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(&ishared))?;
                        (jac, shared) = SymbolicJacobian::calculateJacobian(vars_1.clone(), eqns_1.clone(), m.clone(), true, ishared.clone());
                        (jac_tp, jacConstant) = SymbolicJacobian::analyzeJacobian(vars_1.clone(), eqns_1.clone(), jac.clone());
                        if jacConstant && (jac).is_some() {
                            let true = (analyzeConstantJacobian(&(jac.clone().ok_or("pattern mismatch")?), metamodelica::arrayLength(mt.clone()), var_lst.clone(), eqn_lst.clone(), &shared)?) else { return Err("pattern mismatch") };
                        }
                    } else {
                        jac = None;
                        jac_tp = openmodelica_backend_types::BackendDAE::JacobianType::JAC_NO_ANALYTIC;
                    }
                    Ok(list![metamodelica::Ref::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: comp.clone(), vars: varindxs.clone(), jac: metamodelica::Ref::new(BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: jac.clone() }), jacType: jac_tp, mixedSystem: mixedSystem })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comp, eqn_lst, var_lst, _) => {
                    if !((BackendEquation::allAlgorithmsLst(metamodelica::AsArg::as_arg(&eqn_lst)))) { return Err("guard") }
                    let mut ass2: metamodelica::Array<i32>;
                    let mut indxdisc_var: metamodelica::List<i32>;
                    let mut algorithmComp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let true = (BackendVariable::hasDiscreteVar(metamodelica::AsArg::as_arg(&var_lst))) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::hasContinuousVar(metamodelica::AsArg::as_arg(&var_lst))) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(isyst.matching.clone()) {
                        Deref @ BackendDAE::Matching::MATCHING { ass1: _, ass2: __pa0, comps: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ass2 = metamodelica::Own::own(__pa0);
                    algorithmComp = metamodelica::nil();
                    for mut c in &*comp.clone() {
                        indxdisc_var = metamodelica::nil();
                        for mut j in &*(*metamodelica::index_checked(&__ab_mapEqnIncRow, c.clone())?).clone() {
                            indxdisc_var = metamodelica::cons(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), j.clone())?).clone(); __elt}), indxdisc_var.clone());
                        }
                        algorithmComp = metamodelica::cons(metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: c.clone(), vars: indxdisc_var.clone() }), algorithmComp.clone());
                    }
                    Ok(algorithmComp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, eqn_lst, var_lst, _) => {
                    let mut msg: ArcStr;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut slst: metamodelica::List<ArcStr>;
                    let true = (BackendVariable::hasDiscreteVar(metamodelica::AsArg::as_arg(&var_lst))) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::hasContinuousVar(metamodelica::AsArg::as_arg(&var_lst))) else { return Err("pattern mismatch") };
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAETransform.analyseStrongComponentBlock")); __mm_s.push_str(&*literal!(" failed (Purely discrete algebraic loops cannot be solved by iterative processes. Try to break them open using the delay() operator.)\n")); ArcStr::from(__mm_s) };
                    crlst = List::map(var_lst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?;
                    slst = List::map(crlst.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*msg); __mm_s.push_str(&*stringDelimitList(slst.clone(), literal!("\n"))); ArcStr::from(__mm_s) };
                    slst = List::map(eqn_lst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| BackendDump::equationString(&__a0))?;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*msg); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(slst.clone(), literal!("\n"))); ArcStr::from(__mm_s) };
                    Error::addInternalError(msg.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAETransform.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, eqn_lst, var_lst, _) => {
                    let mut msg: ArcStr;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut slst: metamodelica::List<ArcStr>;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAETransform.analyseStrongComponentBlock")); __mm_s.push_str(&*literal!(" failed\nvariables:\n  ")); ArcStr::from(__mm_s) };
                    crlst = List::map(var_lst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?;
                    slst = List::map(crlst.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*msg); __mm_s.push_str(&*stringDelimitList(slst.clone(), literal!("\n  "))); ArcStr::from(__mm_s) };
                    slst = List::map(eqn_lst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| BackendDump::equationString(&__a0))?;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*msg); __mm_s.push_str(&*literal!("\nequations:\n  ")); __mm_s.push_str(&*stringDelimitList(slst.clone(), literal!("\n  "))); ArcStr::from(__mm_s) };
                    Error::addInternalError(msg.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAETransform.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function analyseStrongComponentBlock failed"), metamodelica::sourceInfo!("BackEnd/BackendDAETransform.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComp)
}

fn crefsAreArray(
    mut eqIn: &metamodelica::Ref<BackendDAE::Equation>,
    mut crefLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> bool {
    let mut isUnsolvable: bool;
    isUnsolvable = 'mc: {
        let __mc_input = &**eqIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: Deref @ DAE::Exp::ARRAY { array: expLst, .. }, .. } => {
                    let mut expLst = (*expLst).clone();
                    (_, _, expLst) = List::intersection1OnTrue(expLst.clone(), crefLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| ExpressionBasics::expEqual(&__a0, __a1))?;
                    Ok((expLst).is_empty())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: Deref @ DAE::Exp::ARRAY { array: expLst, .. }, .. } => {
                    let mut expLst = (*expLst).clone();
                    (_, _, expLst) = List::intersection1OnTrue(expLst.clone(), crefLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| ExpressionBasics::expEqual(&__a0, __a1))?;
                    Ok((expLst).is_empty())
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
    isUnsolvable
}

fn analyzeConstantJacobian(
    mut inJac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut inSize: i32,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<bool> {
    let mut outValid: bool = true;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut info: i32;
    let mut infoStr: ArcStr;
    let mut syst: ArcStr;
    let mut varnames: ArcStr;
    let mut varname: ArcStr;
    let mut rhsStr: ArcStr;
    let mut jacStr: ArcStr;
    let mut eqnstr: ArcStr;
    let mut beqs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rhsVals: metamodelica::List<metamodelica::Real>;
    let mut jacVals: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    jacVals = SymbolicJacobian::evaluateConstantJacobian(inSize, inJac)?;
    rhsVals = List::fill(metamodelica::OrderedFloat(0.0_f64), inSize);
    (_, info) = System::dgesv(jacVals.clone(), rhsVals)?;
    if info < 0 {
        varnames = stringDelimitList(
            List::mapMap(
                inVars.clone(),
                &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
                },
                &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                },
            )?,
            literal!(" ;\n  "),
        );
        eqns = BackendEquation::listEquation(&inEqns)?;
        vars = BackendVariable::listVar1(&inVars)?;
        funcs = BackendDAEUtil::getFunctions(inShared);
        (beqs, _) = BackendDAEUtil::getEqnSysRhs(eqns, vars, Some(funcs))?;
        beqs = beqs.reverse();
        rhsStr = stringDelimitList(List::map(beqs, &ExpressionBasics::printExpStr)?, literal!(" ;\n  "));
        jacStr = stringDelimitList(
            List::map1(
                List::mapList(jacVals, &fnptr!(realString, metamodelica::Real))?,
                &fnptr!(stringDelimitList, metamodelica::List<ArcStr>, ArcStr),
                literal!(" , "),
            )?,
            literal!(" ;\n  "),
        );
        eqnstr = BackendDump::dumpEqnsStr(inEqns)?;
        syst = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*eqnstr);
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*jacStr);
            __mm_s.push_str(&*literal!("] * ["));
            __mm_s.push_str(&*varnames);
            __mm_s.push_str(&*literal!("] = ["));
            __mm_s.push_str(&*rhsStr);
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        Error::addMessage(
            Error::LINEAR_SYSTEM_INVALID.clone(),
            list![literal!("LAPACK/dgesv"), syst],
        )?;
        outValid = false;
    } else if info > 0 {
        varname = ComponentReferenceBasics::printComponentRefStr(&(BackendVariable::varCref(&((inVars).get(info)?))))?;
        infoStr = intString(info);
        varnames = stringDelimitList(
            List::mapMap(
                inVars.clone(),
                &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
                },
                &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                },
            )?,
            literal!(" ;\n  "),
        );
        eqns = BackendEquation::listEquation(&inEqns)?;
        vars = BackendVariable::listVar1(&inVars)?;
        funcs = BackendDAEUtil::getFunctions(inShared);
        (beqs, _) = BackendDAEUtil::getEqnSysRhs(eqns, vars, Some(funcs))?;
        beqs = beqs.reverse();
        rhsStr = stringDelimitList(List::map(beqs, &ExpressionBasics::printExpStr)?, literal!(" ;\n  "));
        jacStr = stringDelimitList(
            List::map1(
                List::mapList(jacVals, &fnptr!(realString, metamodelica::Real))?,
                &fnptr!(stringDelimitList, metamodelica::List<ArcStr>, ArcStr),
                literal!(" , "),
            )?,
            literal!(" ;\n  "),
        );
        eqnstr = BackendDump::dumpEqnsStr(inEqns)?;
        syst = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*eqnstr);
            __mm_s.push_str(&*literal!("\n[\n  "));
            __mm_s.push_str(&*jacStr);
            __mm_s.push_str(&*literal!("\n]\n  *\n[\n  "));
            __mm_s.push_str(&*varnames);
            __mm_s.push_str(&*literal!("\n]\n  =\n[\n  "));
            __mm_s.push_str(&*rhsStr);
            __mm_s.push_str(&*literal!("\n]"));
            ArcStr::from(__mm_s)
        };
        Error::addMessage(Error::LINEAR_SYSTEM_SINGULAR.clone(), list![syst, infoStr, varname])?;
    }
    Ok(outValid)
}

fn transformXToXd(mut inVar: metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    if BackendVariable::isStateVar(&inVar) {
        assign_field!(
            outVar.varName = ComponentReference::crefPrefixDer(inVar.varName.clone()),
            outVar.varKind = openmodelica_backend_types::BackendDAE::VarKind::STATE_DER,
            outVar.unreplaceable = false
        );
    }
    outVar
}

pub(crate) fn getEquationAndSolvedVar(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut inEquationArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inVariables: BackendDAE::Variables,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    i32,
)> {
    let mut outEquation: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outVar: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outIndex: i32;
    (outEquation, outVar, outIndex) = (match &**inComp {
        BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, var: v } => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            eqn = BackendEquation::get(inEquationArray, e.clone())?;
            var = BackendVariable::getVarAt(&inVariables, v.clone())?;
            (list![eqn], list![var], e.clone())
        }
        BackendDAE::StrongComponent::EQUATIONSYSTEM {
            eqns: elst, vars: vlst, ..
        } => {
            let mut e: i32;
            let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            eqnlst = BackendEquation::getList(elst.clone(), inEquationArray)?;
            varlst = List::map1r(
                vlst.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables,
            )?;
            e = (elst).head().cloned()?;
            (eqnlst, varlst, e)
        }
        BackendDAE::StrongComponent::SINGLEARRAY { eqn: e, vars: vlst } => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            eqn = BackendEquation::get(inEquationArray, e.clone())?;
            varlst = List::map1r(
                vlst.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables,
            )?;
            (list![eqn], varlst, e.clone())
        }
        BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: e, vars: vlst } => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            eqn = BackendEquation::get(inEquationArray, e.clone())?;
            varlst = List::map1r(
                vlst.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables,
            )?;
            (list![eqn], varlst, e.clone())
        }
        BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: e, vars: vlst } => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            eqn = BackendEquation::get(inEquationArray, e.clone())?;
            varlst = List::map1r(
                vlst.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables,
            )?;
            (list![eqn], varlst, e.clone())
        }
        BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: e, vars: vlst } => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            eqn = BackendEquation::get(inEquationArray, e.clone())?;
            varlst = List::map1r(
                vlst.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables,
            )?;
            (list![eqn], varlst, e.clone())
        }
        BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: e, vars: vlst } => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            eqn = BackendEquation::get(inEquationArray, e.clone())?;
            varlst = List::map1r(
                vlst.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables,
            )?;
            (list![eqn], varlst, e.clone())
        }
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet:
                BackendDAE::TearingSet {
                    tearingvars: vlst,
                    residualequations: elst,
                    innerEquations,
                    ..
                },
            ..
        } => {
            let mut e: i32;
            let mut otherEqns: metamodelica::List<i32>;
            let mut otherVars: metamodelica::List<i32>;
            let mut otherVarsLst: metamodelica::List<metamodelica::List<i32>>;
            let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut eqnlst1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut varlst1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            eqnlst = BackendEquation::getList(elst.clone(), inEquationArray.clone())?;
            varlst = List::map1r(
                vlst.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables.clone(),
            )?;
            (otherEqns, otherVarsLst, _) = List::map_3(
                metamodelica::AsArg::as_arg(&innerEquations),
                &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0))
                },
            )?;
            otherVars = List::flatten(otherVarsLst)?;
            eqnlst1 = BackendEquation::getList(otherEqns, inEquationArray)?;
            varlst1 = List::map1r(
                otherVars,
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables,
            )?;
            e = (elst).head().cloned()?;
            (listAppend(eqnlst, eqnlst1), listAppend(varlst, varlst1), e)
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln(literal!("BackendDAETransform.getEquationAndSolvedVar failed!"))?;
            return Err("fail");
        }
    });
    Ok((outEquation, outVar, outIndex))
}

pub(crate) fn getEquationAndSolvedVarIndxes(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut outEquation: metamodelica::List<i32>;
    let mut outVar: metamodelica::List<i32>;
    (outEquation, outVar) = 'mc: {
        let __mc_input = &**inComp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, var: v } => {
                    Ok((list![e.clone()], list![v.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: elst, vars: vlst, .. } => {
                    Ok((elst.clone(), vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: e, vars: vlst } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: e, vars: vlst } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: e, vars: vlst } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: e, vars: vlst } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: e, vars: vlst } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: elst, innerEquations, .. }, .. } => {
                    let mut elst1: metamodelica::List<i32>;
                    let mut vlst1: metamodelica::List<i32>;
                    let mut vLstLst: metamodelica::List<metamodelica::List<i32>>;
                    let mut vlst = (*vlst).clone();
                    let mut elst = (*elst).clone();
                    (elst1, vLstLst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    vlst1 = List::flatten(vLstLst.clone())?;
                    elst = listAppend(elst1.clone(), elst.clone());
                    vlst = listAppend(vlst1.clone(), vlst.clone());
                    Ok((elst.clone(), vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("BackendDAETransform.getEquationAndSolvedVarIndxes failed!"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquation, outVar))
}

// =============================================================================
// traverseBackendDAEExps stuff
//
// =============================================================================
pub(crate) fn traverseBackendDAEExpsEqnWithSymbolicOperation<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            )> + 'static,
    >,
    mut inTypeA: Type_a,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            )> + 'static,
    >;

    let mut outEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTypeA: Type_a;
    (outEquation, outTypeA) = 'mc: {
        let __mc_input = &**inEquation;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_1 = metamodelica::Own::own(__pa2);
                    let (__pa3, (__pa4, __pa5)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = metamodelica::Own::own(__pa3);
                    ops = metamodelica::Own::own(__pa4);
                    ext_arg_2 = metamodelica::Own::own(__pa5);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1.clone(), scalar: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr: eqAttr, recordSize } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_1 = metamodelica::Own::own(__pa2);
                    let (__pa3, (__pa4, __pa5)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = metamodelica::Own::own(__pa3);
                    ops = metamodelica::Own::own(__pa4);
                    ext_arg_2 = metamodelica::Own::own(__pa5);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), ext_arg_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::FOR_EQUATION { iter, start, stop, body: eqn, source, attr: eqAttr } => {
                    let mut eqn = (*eqn).clone();
                    let mut outTypeA: Type_a;
                    (eqn, outTypeA) = traverseBackendDAEExpsEqnWithSymbolicOperation(metamodelica::AsArg::as_arg(&eqn), func.clone(), inTypeA.clone())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::FOR_EQUATION { iter: iter.clone(), start: start.clone(), stop: stop.clone(), body: eqn.clone(), source: source.clone(), attr: eqAttr.clone() }), outTypeA.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, source, attr: eqAttr } => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut source = (*source).clone();
                    e1 = Expression::crefExp(cr.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?) {
                        (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, (__pa1, __pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr1 = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_1 = metamodelica::Own::own(__pa2);
                    let (__pa4, (__pa5, _)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = metamodelica::Own::own(__pa4);
                    ops = metamodelica::Own::own(__pa5);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1.clone(), exp: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source, attr: eqAttr } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_1 = metamodelica::Own::own(__pa2);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst }, source, expand: crefExpand, attr: eqAttr } => {
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut statementLst = (*statementLst).clone();
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), func.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    statementLst = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_1 = metamodelica::Own::own(__pa2);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: statementLst.clone() }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: oelsepart }, source, attr: eqAttr } => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut elsepartRes: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut elsepart: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut ext_arg_3: Type_a;
                    let mut cond = (*cond).clone();
                    let mut whenStmtLst = (*whenStmtLst).clone();
                    let mut oelsepart = (*oelsepart).clone();
                    let mut source = (*source).clone();
                    (whenStmtLst, ext_arg_1) = traverseBackendDAEExpsWhenOperatorWithSymbolicOperation(metamodelica::AsArg::as_arg(&whenStmtLst), &*func, inTypeA.clone())?;
                    let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), ext_arg_1.clone()))?;
                    cond = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_2 = metamodelica::Own::own(__pa2);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    if (oelsepart).is_some() {
                        let __pa3 = ::match_deref::match_deref! { match &(oelsepart.clone()) {
                            Some(__pa3) => __pa3.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        elsepart = metamodelica::Own::own(__pa3);
                        let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(traverseBackendDAEExpsEqnWithSymbolicOperation(&(metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: elsepart.clone(), source: source.clone(), attr: eqAttr.clone() })), func.clone(), ext_arg_2.clone())?) {
                            (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: __pa4, source: __pa5, .. }, __pa6) => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        elsepartRes = metamodelica::Own::own(__pa4);
                        source = metamodelica::Own::own(__pa5);
                        ext_arg_3 = metamodelica::Own::own(__pa6);
                        oelsepart = Some(elsepartRes.clone());
                    } else {
                        oelsepart = None;
                        ext_arg_3 = ext_arg_2.clone();
                    }
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: metamodelica::Ref::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: oelsepart.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    Ok((eqn.clone(), ext_arg_3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, source, attr: eqAttr } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_1 = metamodelica::Own::own(__pa2);
                    let (__pa3, (__pa4, __pa5)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = metamodelica::Own::own(__pa3);
                    ops = metamodelica::Own::own(__pa4);
                    ext_arg_2 = metamodelica::Own::own(__pa5);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { conditions: expl, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr: eqAttr } => {
                    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                    let mut ext_arg_1: Type_a;
                    let mut expl = (*expl).clone();
                    let mut eqnslst = (*eqnslst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = traverseBackendDAEExpsLstEqnWithSymbolicOperation(metamodelica::AsArg::as_arg(&expl), &*func, (metamodelica::nil(), inTypeA.clone()), metamodelica::nil())?;
                    expl = metamodelica::Own::own(__pa0);
                    ops = metamodelica::Own::own(__pa1);
                    ext_arg_1 = metamodelica::Own::own(__pa2);
                    source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, source.clone())?;
                    (eqnslst, ext_arg_1) = traverseBackendDAEExpsEqnLstLstWithSymbolicOperation(metamodelica::AsArg::as_arg(&eqnslst), func.clone(), ext_arg_1.clone(), metamodelica::nil())?;
                    (eqns, ext_arg_1) = traverseBackendDAEExpsEqnLstWithSymbolicOperation(metamodelica::AsArg::as_arg(&eqns), func.clone(), ext_arg_1.clone(), metamodelica::nil())?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: expl.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function traverseBackendDAEExpsEqnWithSymbolicOperation failed"), metamodelica::sourceInfo!("BackEnd/BackendDAETransform.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquation, outTypeA))
}

fn traverseBackendDAEExpsLstEqnWithSymbolicOperation<'__b, Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExps: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut func: &'__b dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut inTypeA: Type_a,
    mut iAcc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    '__tco: loop {
        ::match_deref::match_deref! { match inExps {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iAcc.reverse(), inTypeA))
            },
            Deref @ metamodelica::ListNode::Cons { head: exp, tail: rest } => {
                let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut arg: Type_a;
                let mut exp = (*exp).clone();
                (exp, arg) = func(exp.clone(), inTypeA)?;
                { (inExps, func, inTypeA, iAcc) = (rest, func, arg, metamodelica::cons(exp.clone(), iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn traverseBackendDAEExpsEqnLstWithSymbolicOperation<
    '__b,
    Type_a: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inEqns: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            )> + 'static,
    >,
    mut inTypeA: Type_a,
    mut iAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            )> + 'static,
    >;

    '__tco: loop {
        ::match_deref::match_deref! { match inEqns {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iAcc.reverse(), inTypeA))
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn, tail: rest } => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut arg: Type_a;
                let mut eqn = (*eqn).clone();
                (eqn, arg) = traverseBackendDAEExpsEqnWithSymbolicOperation(metamodelica::AsArg::as_arg(&eqn), func.clone(), inTypeA)?;
                { (inEqns, func, inTypeA, iAcc) = (rest, func.clone(), arg, metamodelica::cons(eqn.clone(), iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn traverseBackendDAEExpsEqnLstLstWithSymbolicOperation<'__b, Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEqns: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            )> + 'static,
    >,
    mut inTypeA: Type_a,
    mut iAcc: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    Type_a,
)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, Type_a),
            )> + 'static,
    >;

    '__tco: loop {
        ::match_deref::match_deref! { match inEqns {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iAcc.reverse(), inTypeA))
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn, tail: rest } => {
                let mut eqnslst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut arg: Type_a;
                let mut eqn = (*eqn).clone();
                (eqn, arg) = traverseBackendDAEExpsEqnLstWithSymbolicOperation(metamodelica::AsArg::as_arg(&eqn), func.clone(), inTypeA, metamodelica::nil())?;
                { (inEqns, func, inTypeA, iAcc) = (rest, func.clone(), arg, metamodelica::cons(eqn.clone(), iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn traverseBackendDAEExpsWhenOperatorWithSymbolicOperation<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStmtLst: &metamodelica::List<BackendDAE::WhenOperator>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<DAE::Exp>,
        (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, ArgT),
    ) -> Result<(
        metamodelica::Ref<DAE::Exp>,
        (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, ArgT),
    )>,
    mut inArg: ArgT,
) -> Result<(metamodelica::List<BackendDAE::WhenOperator>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, ArgT),
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, ArgT),
            )> + 'static,
    >;

    let mut outStmtLst: metamodelica::List<BackendDAE::WhenOperator> = metamodelica::nil();
    let mut outArg: ArgT = inArg.clone();
    for mut rs in &**inStmtLst {
        let mut rs = rs.clone();
        rs = (match rs.clone() {
            BackendDAE::WhenOperator::ASSIGN {
                left: ref lhs,
                right: ref cond,
                source: ref src,
            } => {
                let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                let mut lhs = lhs.clone();
                let mut cond = cond.clone();
                let mut src = src.clone();
                let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), inArg.clone()))?;
                cond = metamodelica::Own::own(__pa0);
                ops = metamodelica::Own::own(__pa1);
                outArg = metamodelica::Own::own(__pa2);
                let (__pa3, (__pa4, __pa5)) = func(lhs.clone(), (ops, outArg))?;
                lhs = metamodelica::Own::own(__pa3);
                ops = metamodelica::Own::own(__pa4);
                outArg = metamodelica::Own::own(__pa5);
                src = List::foldr(&ops, &ElementSource::addSymbolicTransformation, src.clone())?;
                BackendDAE::WhenOperator::ASSIGN {
                    left: lhs.clone(),
                    right: cond.clone(),
                    source: src.clone(),
                }
            }
            BackendDAE::WhenOperator::REINIT {
                stateVar: ref cr,
                value: ref cond,
                source: ref src,
            } => {
                let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                let mut cr = cr.clone();
                let mut cond = cond.clone();
                let mut src = src.clone();
                let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), inArg.clone()))?;
                cond = metamodelica::Own::own(__pa0);
                ops = metamodelica::Own::own(__pa1);
                outArg = metamodelica::Own::own(__pa2);
                let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr.clone())?, (ops, outArg))?) {
                    (Deref @ DAE::Exp::CREF { componentRef: __pa3, .. }, (__pa4, __pa5)) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                cr = metamodelica::Own::own(__pa3);
                ops = metamodelica::Own::own(__pa4);
                outArg = metamodelica::Own::own(__pa5);
                src = List::foldr(&ops, &ElementSource::addSymbolicTransformation, src.clone())?;
                BackendDAE::WhenOperator::REINIT {
                    stateVar: cr.clone(),
                    value: cond.clone(),
                    source: src.clone(),
                }
            }
            BackendDAE::WhenOperator::ASSERT {
                condition: ref cond,
                message: ref msg,
                level: mut level,
                source: ref src,
            } => {
                let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                let mut cond = cond.clone();
                let mut src = src.clone();
                let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), inArg.clone()))?;
                cond = metamodelica::Own::own(__pa0);
                ops = metamodelica::Own::own(__pa1);
                outArg = metamodelica::Own::own(__pa2);
                src = List::foldr(&ops, &ElementSource::addSymbolicTransformation, src.clone())?;
                BackendDAE::WhenOperator::ASSERT {
                    condition: cond.clone(),
                    message: msg.clone(),
                    level: level.clone(),
                    source: src.clone(),
                }
            }
            BackendDAE::WhenOperator::NORETCALL {
                exp: mut exp,
                source: ref src,
            } => {
                let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
                let mut exp = exp.clone();
                let mut src = src.clone();
                let (__pa0, (__pa1, __pa2)) =
                    Expression::traverseExpBottomUp(exp.clone(), func, (metamodelica::nil(), outArg))?;
                exp = metamodelica::Own::own(__pa0);
                ops = metamodelica::Own::own(__pa1);
                outArg = metamodelica::Own::own(__pa2);
                src = List::foldr(&ops, &ElementSource::addSymbolicTransformation, src.clone())?;
                BackendDAE::WhenOperator::NORETCALL {
                    exp: exp.clone(),
                    source: src.clone(),
                }
            }
            _ => rs,
        });
        outStmtLst = metamodelica::cons(rs, outStmtLst);
    }
    outStmtLst = outStmtLst.reverse();
    Ok((outStmtLst, outArg))
}

pub(crate) fn collapseArrayExpressions(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    for mut syst in &*dae.eqs.clone() {
        BackendEquation::traverseEquationArray_WithUpdate(
            syst.orderedEqs.clone(),
            &({
                let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> =
                    (std::sync::Arc::new(collapseArrayCrefExp)
                        as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>);
                move |__pe_a0, __pe_a2| {
                    traverseBackendDAEExpsEqnWithSymbolicOperation(&__pe_a0, __pe_b1.clone(), __pe_a2)
                }
            }),
            0,
        )?;
        BackendEquation::traverseEquationArray_WithUpdate(
            syst.removedEqs.clone(),
            &({
                let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> =
                    (std::sync::Arc::new(collapseArrayCrefExp)
                        as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>);
                move |__pe_a0, __pe_a2| {
                    traverseBackendDAEExpsEqnWithSymbolicOperation(&__pe_a0, __pe_b1.clone(), __pe_a2)
                }
            }),
            0,
        )?;
    }
    Ok(dae)
}

pub(crate) fn collapseArrayCrefExp<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, T),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, T),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, T);
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let mut t: T;
    (ops, t) = inTpl.clone();
    (outExp, t) = Expression::traverseExpTopDown(
        inExp.clone(),
        &fnptr!(collapseArrayCrefExpWork, metamodelica::Ref<DAE::Exp>, _),
        t,
    )?;
    if !(ExpressionBasics::expEqual(&inExp, outExp.clone())?) {
        outTpl = (
            metamodelica::cons(
                metamodelica::Ref::new(DAE::SymbolicOperation::SIMPLIFY {
                    before: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: inExp }),
                    after: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: outExp.clone() }),
                }),
                ops,
            ),
            t,
        );
    } else {
        outTpl = inTpl;
    }
    Ok((outExp, outTpl))
}

fn collapseArrayCrefExpWork<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut t: T,
) -> (metamodelica::Ref<DAE::Exp>, bool, T) {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    let mut cont: bool;
    let mut t: T = t;
    (e, cont) = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { .. } => {
                    Ok((collapseArrayCrefExpWork2(e.clone())?, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { .. } => {
                    Ok((collapseArrayCrefExpWork2(e.clone())?, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((e.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (e, cont, t)
}

fn collapseArrayCrefExpWork2(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut ds: metamodelica::List<i32>;
    let mut len: i32;
    let mut exp_count: i32;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut ndim: i32;
    (dims, ty) = (::match_deref::match_deref! { match &(&*e) {
        Deref @ DAE::Exp::MATRIX { ty: __esc_ty @ Deref @ DAE::Type::T_ARRAY { dims: __esc_dims, .. }, .. } => {
            ty = (*__esc_ty).clone();
            dims = (*__esc_dims).clone();
            (dims.clone(), ty.clone())
        },
        Deref @ DAE::Exp::ARRAY { ty: __esc_ty @ Deref @ DAE::Type::T_ARRAY { dims: __esc_dims, .. }, .. } => {
            ty = (*__esc_ty).clone();
            dims = (*__esc_dims).clone();
            (dims.clone(), ty.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    let () = (match &*(Types::arrayElementType(&ty)) {
        DAE::Type::T_COMPLEX { .. } => return Err("fail"),
        _ => (),
    });
    ds = Expression::dimensionsSizes(dims)?;
    ndim = ((ds).len() as i32);
    len = ({
        let mut __acc: i32 = 1;
        for mut i in (ds).into_iter().cloned() {
            let __x = i.clone();
            __acc *= __x;
        }
        __acc
    });
    let true = (len > 0) else {
        return Err("pattern mismatch");
    };
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Expression::flattenArrayExpToList(e)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    exps = metamodelica::Own::own(__pa1);
    let __pa2 = ::match_deref::match_deref! { match &(exp1) {
        Deref @ DAE::Exp::CREF { componentRef: __pa2, .. } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr1 = metamodelica::Own::own(__pa2);
    subs = ComponentReference::crefLastSubs(&cr1)?;
    let true = (ndim == ((subs).len() as i32)) else {
        return Err("pattern mismatch");
    };
    let true = (((subs).len() as i32) == ((ComponentReferenceBasics::crefSubs(&cr1)?).len() as i32)) else {
        return Err("pattern mismatch");
    };
    for mut sub in &*subs {
        ::match_deref::match_deref! { match &(sub.clone()) {
            Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } } => (),
            _ => return Err("pattern mismatch"),
        } };
    }
    exp_count = ((exps).len() as i32) + 1;
    let true = (exp_count == len) else {
        return Err("pattern mismatch");
    };
    dims = TypesDump::getDimensions(&(ComponentReference::crefLastType(&cr1)?));
    let true = (exp_count
        == ({
            let mut __acc: i32 = 1;
            for mut i in (Expression::dimensionsSizes(dims)?).into_iter().cloned() {
                let __x = i.clone();
                __acc *= __x;
            }
            __acc
        }))
    else {
        return Err("pattern mismatch");
    };
    for mut exp in &*exps {
        let __pa4 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: __pa4, .. } => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cr2 = metamodelica::Own::own(__pa4);
        let true = (ndim == ((ComponentReference::crefLastSubs(&cr2)?).len() as i32)) else {
            return Err("pattern mismatch");
        };
        let true = (ComponentReferenceBasics::crefEqualWithoutSubs(cr1.clone(), cr2.clone())) else {
            return Err("pattern mismatch");
        };
        let true = (1 == ComponentReferenceBasics::crefCompareIntSubscript(&cr2, &cr1)?) else {
            return Err("pattern mismatch");
        };
        cr1 = cr2;
    }
    e = Expression::makeCrefExp(ComponentReferenceBasics::crefStripLastSubs(&cr1)?, ty)?;
    Ok(e)
}
