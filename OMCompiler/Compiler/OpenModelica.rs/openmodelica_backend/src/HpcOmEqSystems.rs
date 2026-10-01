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

use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::HpcOmScheduler;
use crate::HpcOmSimCodeMain;
use crate::HpcOmTaskGraph;
use crate::IndexReduction;
use crate::Matching;
use crate::Tearing;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
//--------------------------------------------------//
// matrix type
//-------------------------------------------------//
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EqSys {
    pub dim: i32,
    pub matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    pub vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    pub vectorX: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>,
}

impl metamodelica::gc::MMTrace for EqSys {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.dim, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.matrixA, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vectorB, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vectorX, __mmv)?;
        Ok(())
    }
}
impl Default for EqSys {
    fn default() -> Self {
        Self {
            dim: Default::default(),
            matrixA: Default::default(),
            vectorB: Default::default(),
            vectorX: Default::default(),
        }
    }
}

pub type LINSYS = EqSys;

//--------------------------------------------------//
// start functions for handling linearTornSystems from here
//-------------------------------------------------//
pub(crate) fn partitionLinearTornSystem(
    mut daeIn: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> metamodelica::Ref<BackendDAE::BackendDAE> {
    let mut daeOut: metamodelica::Ref<BackendDAE::BackendDAE>;
    daeOut = 'mc: {
        let __mc_input = &*daeIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { eqs, shared } => {
                    let mut eqs = (*eqs).clone();
                    let true = (intGt(Flags::getConfigInt(Flags::PARTLINTORN.clone())?, 0)) else { return Err("pattern mismatch") };
                    (eqs, _) = List::map1Fold(metamodelica::AsArg::as_arg(&eqs), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: metamodelica::Ref<BackendDAE::Shared>, __a2: i32| reduceLinearTornSystem(__a0, &__a1, __a2), shared.clone(), 1)?;
                    Ok(metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: eqs.clone(), shared: shared.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(daeIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    daeOut
}

fn reduceLinearTornSystem(
    mut systIn: metamodelica::Ref<BackendDAE::EqSystem>,
    mut sharedIn: &metamodelica::Ref<BackendDAE::Shared>,
    mut tornSysIdxIn: i32,
) -> Result<(metamodelica::Ref<BackendDAE::EqSystem>, i32)> {
    let mut systOut: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut tornSysIdxOut: i32;
    (systOut, tornSysIdxOut) = 'mc: {
        let __mc_input = tornSysIdxIn;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut tornSysIdx: i32;
            let mut ass1: metamodelica::Array<i32>;
            let mut ass2: metamodelica::Array<i32>;
            let mut systTmp: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut allComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(systIn.clone()) {
                Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ass1 = metamodelica::Own::own(__pa0);
            ass2 = metamodelica::Own::own(__pa1);
            allComps = metamodelica::Own::own(__pa2);
            (systTmp, tornSysIdx) = reduceLinearTornSystem1(
                1,
                &allComps,
                ass1.clone(),
                ass2.clone(),
                systIn.clone(),
                sharedIn,
                tornSysIdxIn,
            );
            Ok((systTmp.clone(), tornSysIdx))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("reduceLinearTornSystem failed!"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((systOut, tornSysIdxOut))
}

fn reduceLinearTornSystem1(
    mut compIdx: i32,
    mut compsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut systIn: metamodelica::Ref<BackendDAE::EqSystem>,
    mut sharedIn: &metamodelica::Ref<BackendDAE::Shared>,
    mut tornSysIdxIn: i32,
) -> (metamodelica::Ref<BackendDAE::EqSystem>, i32) {
    let mut systOut: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut tornSysIdxOut: i32;
    (systOut, tornSysIdxOut) = 'mc: {
        let __mc_input = systIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (((compsIn).len() as i32) < compIdx) else { return Err("pattern mismatch") };
                    Ok((systIn.clone(), tornSysIdxIn))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst => {
                    let mut numNewSingleEqs: i32;
                    let mut tornSysIdx: i32;
                    let mut linear: bool;
                    let mut ass1New: metamodelica::Array<i32>;
                    let mut ass2New: metamodelica::Array<i32>;
                    let mut ass1All: metamodelica::Array<i32>;
                    let mut ass2All: metamodelica::Array<i32>;
                    let mut tvarIdcs: metamodelica::List<i32>;
                    let mut resEqIdcs: metamodelica::List<i32>;
                    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
                    let mut matchingNew: metamodelica::Ref<BackendDAE::Matching>;
                    let mut matchingOther: metamodelica::Ref<BackendDAE::Matching>;
                    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                    let mut compsNew: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut compsTmp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut otherComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eqsNew: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eqsOld: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut resEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut varsNew: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut varsOld: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut syst = (*syst).clone();
                    let true = (((compsIn).len() as i32) >= compIdx) else { return Err("pattern mismatch") };
                    comp = (compsIn).get(compIdx)?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(comp.clone()) {
                        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __pa0, residualequations: __pa1, innerEquations: __pa2, .. }, linear: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    tvarIdcs = metamodelica::Own::own(__pa0);
                    resEqIdcs = metamodelica::Own::own(__pa1);
                    innerEquations = metamodelica::Own::own(__pa2);
                    linear = metamodelica::Own::own(__pa3);
                    let true = (linear) else { return Err("pattern mismatch") };
                    let true = (intLe(((tvarIdcs).len() as i32), Flags::getConfigInt(Flags::PARTLINTORN.clone())?)) else { return Err("pattern mismatch") };
                    (varsNew, eqsNew, _, resEqs, matchingNew) = reduceLinearTornSystem2(&systIn, sharedIn.clone(), tvarIdcs.clone(), resEqIdcs.clone(), &innerEquations, tornSysIdxIn)?;
                    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(matchingNew.clone()) {
                        Deref @ BackendDAE::Matching::MATCHING { ass1: __pa4, ass2: __pa5, comps: __pa6 } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ass1New = metamodelica::Own::own(__pa4);
                    ass2New = metamodelica::Own::own(__pa5);
                    compsNew = metamodelica::Own::own(__pa6);
                    varsOld = BackendVariable::varList(&syst.orderedVars)?;
                    eqsOld = BackendEquation::equationList(syst.orderedEqs.clone())?;
                    varLst = listAppend(varsOld.clone(), varsNew.clone());
                    eqLst = listAppend(eqsOld.clone(), eqsNew.clone());
                    eqLst = List::fold2(&(List::intRange(((resEqIdcs).len() as i32))), &move |__a0: i32, __a1: _, __a2: metamodelica::List<i32>, __a3: _| replaceAtPositionFromList(__a0, &__a1, &__a2, __a3), resEqs.clone(), resEqIdcs.clone(), eqLst.clone())?;
                    assign_field!(
                        syst.orderedVars = BackendVariable::listVar1(&varLst)?,
                        syst.orderedEqs = BackendEquation::listEquation(&eqLst)?
                    );
                    ass1All = arrayCreate(((varLst).len() as i32), -1);
                    ass2All = arrayCreate(((varLst).len() as i32), -1);
                    ass1All = Array::copy(ass1.clone(), ass1All.clone())?;
                    ass2All = Array::copy(ass2.clone(), ass2All.clone())?;
                    (ass1All, ass2All) = List::fold2(&(List::intRange(((tvarIdcs).len() as i32))), &move |__a0: i32, __a1: metamodelica::List<i32>, __a2: metamodelica::List<i32>, __a3: (metamodelica::Array<i32>, metamodelica::Array<i32>)| updateResidualMatching(__a0, &__a1, &__a2, __a3), tvarIdcs.clone(), resEqIdcs.clone(), (ass1All.clone(), ass2All.clone()))?;
                    matchingOther = getOtherComps(&innerEquations, ass1All.clone(), ass2All.clone())?;
                    let __pa7 = ::match_deref::match_deref! { match &(matchingOther.clone()) {
                        Deref @ BackendDAE::Matching::MATCHING { comps: __pa7, .. } => __pa7.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    otherComps = metamodelica::Own::own(__pa7);
                    numNewSingleEqs = ((compsNew).len() as i32) - ((tvarIdcs).len() as i32);
                    compsTmp = List::replaceAtWithList(listAppend(compsNew.clone(), otherComps.clone()), compIdx, compsIn.clone())?;
                    (ass1All, ass2All) = List::fold2(&(List::intRange(metamodelica::arrayLength(ass1New.clone()))), &updateMatching, (((eqsOld).len() as i32), ((varsOld).len() as i32)), (ass1New.clone(), ass2New.clone()), (ass1All.clone(), ass2All.clone()))?;
                    assign_field!(syst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1All.clone(), ass2: ass2All.clone(), comps: compsTmp.clone() }));
                    syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None);
                    (syst, _, _) = BackendDAEUtil::getAdjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(sharedIn))?;
                    (syst, tornSysIdx) = reduceLinearTornSystem1(compIdx + 1 + numNewSingleEqs, &compsTmp, ass1All.clone(), ass2All.clone(), syst.clone(), sharedIn, tornSysIdxIn + 1);
                    Ok((syst.clone(), tornSysIdx))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. } => {
                    let mut tornSysIdx: i32;
                    let mut ass1All: metamodelica::Array<i32>;
                    let mut ass2All: metamodelica::Array<i32>;
                    let mut eqIdcs: metamodelica::List<i32>;
                    let mut varIdcs: metamodelica::List<i32>;
                    let mut hpcSyst: EqSys;
                    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                    let mut compsNew: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut compsTmp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut otherComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut derRepl: BackendVarTransform::VariableReplacements;
                    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eqsNew: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eqsOld: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut varLstRepl: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut varsOld: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut addVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut syst = (*syst).clone();
                    let true = (((compsIn).len() as i32) >= compIdx) else { return Err("pattern mismatch") };
                    comp = (compsIn).get(compIdx)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp.clone()) {
                        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: __pa0, eqns: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    varIdcs = metamodelica::Own::own(__pa0);
                    eqIdcs = metamodelica::Own::own(__pa1);
                    let true = (intLe(((varIdcs).len() as i32), 2)) else { return Err("pattern mismatch") };
                    eqLst = BackendEquation::getList(eqIdcs.clone(), eqs.clone())?;
                    eqLst = BackendEquation::replaceDerOpInEquationList(&eqLst)?;
                    varLst = List::map1r(varIdcs.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    varLstRepl = List::map(varLst.clone(), &fnptr!(BackendVariable::transformXToXd, metamodelica::Ref<BackendDAE::Var>))?;
                    derRepl = BackendVarTransform::emptyReplacements();
                    derRepl = List::threadFold(&varLst, varLstRepl.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: metamodelica::Ref<BackendDAE::Var>, __a2: BackendVarTransform::VariableReplacements| addDerReplacement(&__a0, &__a1, __a2), derRepl.clone())?;
                    hpcSyst = getEqSystem(&eqLst, varLstRepl.clone())?;
                    (eqsNew, addEqs, addVars) = CramerRule(hpcSyst.clone());
                    (eqsNew, _) = BackendVarTransform::replaceEquations(eqsNew.clone(), &derRepl, None)?;
                    varsOld = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars))?;
                    eqsOld = BackendEquation::equationList(eqs.clone())?;
                    compsNew = matchComponent(&eqsNew, &varLstRepl, eqIdcs.clone(), varIdcs.clone(), sharedIn.clone())?;
                    otherComps = matchComponent(&(addEqs.clone()), &(addVars.clone()), List::intRange2(((eqsOld).len() as i32) + 1, ((eqsOld).len() as i32) + 1 + ((addEqs).len() as i32)), List::intRange2(((varsOld).len() as i32) + 1, ((varsOld).len() as i32) + 1 + ((addVars).len() as i32)), sharedIn.clone())?;
                    compsNew = listAppend(otherComps.clone(), compsNew.clone());
                    compsTmp = List::replaceAtWithList(compsNew.clone(), compIdx, compsIn.clone())?;
                    eqLst = listAppend(eqsOld.clone(), addEqs.clone());
                    varLst = listAppend(varsOld.clone(), addVars.clone());
                    eqLst = List::fold2(&(List::intRange(((eqsNew).len() as i32))), &move |__a0: i32, __a1: _, __a2: metamodelica::List<i32>, __a3: _| replaceAtPositionFromList(__a0, &__a1, &__a2, __a3), eqsNew.clone(), eqIdcs.clone(), eqLst.clone())?;
                    assign_field!(
                        syst.orderedEqs = BackendEquation::listEquation(&eqLst)?,
                        syst.orderedVars = BackendVariable::listVar1(&varLst)?
                    );
                    ass1All = arrayCreate(((varLst).len() as i32), -1);
                    ass2All = arrayCreate(((varLst).len() as i32), -1);
                    ass1All = Array::copy(ass1.clone(), ass1All.clone())?;
                    ass2All = Array::copy(ass2.clone(), ass2All.clone())?;
                    List::map2_0(&compsNew, &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>, __a1: metamodelica::Array<i32>, __a2: metamodelica::Array<i32>| updateAssignmentsByComp(&__a0, __a1, __a2), ass1All.clone(), ass2All.clone())?;
                    assign_field!(syst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1All.clone(), ass2: ass2All.clone(), comps: compsTmp.clone() }));
                    syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None);
                    (syst, tornSysIdx) = reduceLinearTornSystem1(compIdx + 1, &compsTmp, ass1All.clone(), ass2All.clone(), syst.clone(), sharedIn, tornSysIdxIn + 1);
                    Ok((syst.clone(), tornSysIdx))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tornSysIdx: i32;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    (syst, tornSysIdx) = reduceLinearTornSystem1(compIdx + 1, compsIn, ass1.clone(), ass2.clone(), systIn.clone(), sharedIn, tornSysIdxIn);
                    Ok((syst.clone(), tornSysIdx))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (systOut, tornSysIdxOut)
}

fn compHasDummyState(
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<bool> {
    let mut hasDummy: bool;
    hasDummy = (::match_deref::match_deref! { match (comp, syst) {
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: varIdcs, .. }, .. }, Deref @ BackendDAE::EqSystem { orderedVars: vars, .. }) => {
            let mut b: bool;
            let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            varLst = List::map1(varIdcs.clone(), &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1), vars.clone())?;
            b = List::fold(&(List::map(varLst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isDummyStateVar(&__a0)) })?), &fnptr!(boolOr, bool, bool), false)?;
            b = b && intGt(((varIdcs).len() as i32), 1);
            b
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: varIdcs, .. }, Deref @ BackendDAE::EqSystem { orderedVars: vars, .. }) => {
            let mut b: bool;
            let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            varLst = List::map1(varIdcs.clone(), &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1), vars.clone())?;
            b = List::fold(&(List::map(varLst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isDummyStateVar(&__a0)) })?), &fnptr!(boolOr, bool, bool), false)?;
            b
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasDummy)
}

fn updateAssignmentsByComp(
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let mut eqn: i32;
    let mut var: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*comp)) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: __pa0, var: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    eqn = metamodelica::Own::own(__pa0);
    var = metamodelica::Own::own(__pa1);
    metamodelica::arrayUpdate(ass2.clone(), eqn, var)?;
    metamodelica::arrayUpdate(ass1.clone(), var, eqn)?;
    Ok(())
}

fn matchComponent(
    mut eqLstIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut varLstIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut eqIdcs: metamodelica::List<i32>,
    mut varIdcs: metamodelica::List<i32>,
    mut sharedIn: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>> {
    let mut compsOut: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    matching = buildSingleEquationSystem(
        ((eqLstIn).len() as i32),
        eqLstIn,
        varLstIn,
        sharedIn,
        metamodelica::nil(),
    )?;
    let __pa0 = ::match_deref::match_deref! { match &(matching) {
        Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    compsOut = List::map2(
        comps,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
               __a1: metamodelica::Array<i32>,
               __a2: metamodelica::Array<i32>| replaceIndecesInComp(&__a0, __a1, __a2),
        metamodelica::arrayFromVec(eqIdcs.into_iter().cloned().collect()),
        metamodelica::arrayFromVec(varIdcs.into_iter().cloned().collect()),
    )?;
    Ok(compsOut)
}

fn replaceIndecesInComp(
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut eqMap: metamodelica::Array<i32>,
    mut varMap: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<BackendDAE::StrongComponent>> {
    let mut compOut: metamodelica::Ref<BackendDAE::StrongComponent>;
    compOut = (match &**comp {
        BackendDAE::StrongComponent::SINGLEEQUATION { eqn, var } => {
            let mut eqn = (*eqn).clone();
            let mut var = (*var).clone();
            eqn = metamodelica::arrayGet(eqMap.clone(), eqn.clone())?;
            var = metamodelica::arrayGet(varMap.clone(), var.clone())?;
            metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEEQUATION {
                eqn: eqn.clone(),
                var: var.clone(),
            })
        }
        _ => return Err("fail"),
    });
    Ok(compOut)
}

fn reduceLinearTornSystem2(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut tVarIdcs0: metamodelica::List<i32>,
    mut resEqIdcs0: metamodelica::List<i32>,
    mut innerEquations: &metamodelica::List<BackendDAE::InnerEquation>,
    mut tornSysIdx: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<BackendDAE::Matching>,
)> {
    let mut varsNewOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqsNewOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut tVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut resEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut matchingOut: metamodelica::Ref<BackendDAE::Matching>;
    let mut ass1New: metamodelica::Array<i32>;
    let mut ass2New: metamodelica::Array<i32>;
    let mut size: i32;
    let mut otherEqSize: i32;
    let mut compSize: i32;
    let mut otherEqnsInts: metamodelica::List<i32>;
    let mut otherVarsInts: metamodelica::List<i32>;
    let mut tVarRange: metamodelica::List<i32>;
    let mut otherVarsIntsLst: metamodelica::List<metamodelica::List<i32>>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut oeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut matchingNew: metamodelica::Ref<BackendDAE::Matching>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut compsNew: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut oComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut compsEqSys: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut vars: BackendDAE::Variables;
    let mut ovars: BackendDAE::Variables;
    let mut derRepl: BackendVarTransform::VariableReplacements;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut otherEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut otherEqnsLstReplaced: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut hs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut tvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut tvarsReplaced: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut ovarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut a_0: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut addVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut hs_i_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut a_i_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut a_i_lst1: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut g_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut hs_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut h_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut xa_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut a_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut replArr: metamodelica::Array<BackendVarTransform::VariableReplacements>;
    let mut tcrs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut ovcrs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*isyst)) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    vars = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    eqLst = BackendEquation::equationList(eqns.clone())?;
    varLst = BackendVariable::varList(&vars)?;
    tvars = List::map1r(
        tVarIdcs0.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        vars.clone(),
    )?;
    tvarsReplaced = List::map(
        tvars.clone(),
        &fnptr!(BackendVariable::transformXToXd, metamodelica::Ref<BackendDAE::Var>),
    )?;
    tcrs = List::map(tvarsReplaced.clone(), &move |__a0: metamodelica::Ref<
        BackendDAE::Var,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
    })?;
    derRepl = BackendVarTransform::emptyReplacements();
    derRepl = List::threadFold(
        &tvars,
        tvarsReplaced.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::Var>,
               __a1: metamodelica::Ref<BackendDAE::Var>,
               __a2: BackendVarTransform::VariableReplacements| addDerReplacement(&__a0, &__a1, __a2),
        derRepl,
    )?;
    reqns = BackendEquation::getList(resEqIdcs0.clone(), eqns.clone())?;
    reqns = BackendEquation::replaceDerOpInEquationList(&reqns)?;
    (otherEqnsInts, otherVarsIntsLst, _) = List::map_3(
        innerEquations,
        &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0))
        },
    )?;
    otherEqnsLst = BackendEquation::getList(otherEqnsInts, eqns)?;
    oeqns = BackendEquation::listEquation(&otherEqnsLst)?;
    otherEqnsLstReplaced = BackendEquation::replaceDerOpInEquationList(&otherEqnsLst)?;
    otherVarsInts = List::unionList(&otherVarsIntsLst)?;
    ovarsLst = List::map1r(
        otherVarsInts,
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        vars,
    )?;
    ovarsLst = List::map(
        ovarsLst,
        &fnptr!(BackendVariable::transformXToXd, metamodelica::Ref<BackendDAE::Var>),
    )?;
    ovars = BackendVariable::listVar1(&ovarsLst)?;
    ovcrs = List::map(
        ovarsLst.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        },
    )?;
    size = ((tvars).len() as i32);
    otherEqSize = ((otherEqnsLst).len() as i32);
    compSize = ((comps).len() as i32);
    tVarRange = List::intRange2(0, size);
    replArr = arrayCreate(size + 1, BackendVarTransform::emptyReplacements());
    g_iArr = arrayCreate(size + 1, metamodelica::nil());
    h_iArr = arrayCreate(size + 1, metamodelica::nil());
    hs_iArr = arrayCreate(size + 1, metamodelica::nil());
    xa_iArr = arrayCreate(size + 1, metamodelica::nil());
    a_iArr = arrayCreate(size + 1, metamodelica::nil());
    (g_iArr, xa_iArr, replArr) = getAlgebraicEquationsForEI(
        &tVarRange,
        size,
        &otherEqnsLstReplaced,
        &tvarsReplaced,
        &tcrs,
        &ovarsLst,
        &ovcrs,
        g_iArr.clone(),
        xa_iArr.clone(),
        replArr.clone(),
        tornSysIdx,
    )?;
    h_iArr = getResidualExpressions(&tVarRange, reqns, replArr.clone(), h_iArr.clone())?;
    (hs_iArr, a_iArr) = getTornSystemCoefficients(
        &tVarRange,
        size,
        tornSysIdx,
        h_iArr.clone(),
        hs_iArr.clone(),
        a_iArr.clone(),
    )?;
    a_i_lst = a_iArr
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    hs_i_lst = hs_iArr
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    eqsNewOut = List::flatten(listAppend(
        g_iArr
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
        hs_i_lst,
    ))?;
    varsNewOut = List::flatten(listAppend(
        xa_iArr
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
        a_i_lst.clone(),
    ))?;
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(a_i_lst) {
        Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    a_0 = metamodelica::Own::own(__pa4);
    a_i_lst1 = metamodelica::Own::own(__pa5);
    hs = buildNewResidualEquation(1, &a_i_lst1, &a_0, &tvarsReplaced, &(metamodelica::nil()))?;
    tVarsOut = tvarsReplaced;
    resEqsOut = hs;
    (eqsNewOut, varsNewOut, resEqsOut) = simplifyNewEquations(
        eqsNewOut,
        varsNewOut,
        resEqsOut,
        ({
            let mut __acc: i32 = 0;
            for mut l in (xa_iArr.clone()).borrow().iter() {
                let __x = ((l).len() as i32);
                __acc += __x;
            }
            __acc
        }),
        2,
        &ishared,
    )?;
    (compsEqSys, resEqsOut, tVarsOut, addEqLst, addVarLst) =
        buildEqSystemComponent(resEqIdcs0, tVarIdcs0, resEqsOut, tVarsOut, a_iArr.clone(), &ishared)?;
    (resEqsOut, _) = BackendVarTransform::replaceEquations(resEqsOut, &derRepl, None)?;
    eqsNewOut = listAppend(eqsNewOut, addEqLst);
    varsNewOut = listAppend(varsNewOut, addVarLst);
    matchingNew = buildSingleEquationSystem(compSize, &eqsNewOut, &varsNewOut, ishared, metamodelica::nil())?;
    let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(matchingNew) {
        Deref @ BackendDAE::Matching::MATCHING { ass1: __pa6, ass2: __pa7, comps: __pa8 } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ass1New = metamodelica::Own::own(__pa6);
    ass2New = metamodelica::Own::own(__pa7);
    compsNew = metamodelica::Own::own(__pa8);
    compsNew = List::map2(
        compsNew,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>, __a1: i32, __a2: i32| {
            updateIndicesInComp(&__a0, __a1, __a2)
        },
        ((varLst).len() as i32),
        ((eqLst).len() as i32),
    )?;
    oComps = listAppend(compsNew, compsEqSys);
    matchingOut = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
        ass1: ass1New.clone(),
        ass2: ass2New.clone(),
        comps: oComps,
    });
    Ok((varsNewOut, eqsNewOut, tVarsOut, resEqsOut, matchingOut))
}

fn addDerReplacement(
    mut var1: &metamodelica::Ref<BackendDAE::Var>,
    mut var2: &metamodelica::Ref<BackendDAE::Var>,
    mut replIn: BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = (match &**var1 {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { .. },
            ..
        } => {
            let mut dest: metamodelica::Ref<DAE::Exp>;
            let mut source: metamodelica::Ref<DAE::ComponentRef>;
            let mut repl: BackendVarTransform::VariableReplacements;
            source = BackendVariable::varCref(var2);
            dest = BackendVariable::varExp(var1)?;
            dest = IndexReduction::makeder(dest)?;
            repl = BackendVarTransform::addReplacement(replIn, source, dest, None)?;
            repl
        }
        _ => replIn,
    });
    Ok(replOut)
}

fn simplifyNewEquations<'__b>(
    mut eqsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut varsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut resEqsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut numAuxiliaryVars: i32,
    mut numIter: i32,
    mut shared: &'__b metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut eqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut resEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqArr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut varArr: BackendDAE::Variables;
    let mut eqSys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut size: i32;
    let mut numIterNew: i32;
    let mut numAux: i32;
    let mut varIdcs: metamodelica::List<i32>;
    let mut eqIdcs: metamodelica::List<i32>;
    eqArr = BackendEquation::listEquation(&eqsIn)?;
    varArr = BackendVariable::listVar1(&varsIn)?;
    eqSys = BackendDAEUtil::createEqSystem(
        varArr.clone(),
        eqArr.clone(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (m, mT) = BackendDAEUtil::adjacencyMatrix(
        &eqSys,
        openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE,
        None,
        BackendDAEUtil::isInitializationDAE(shared),
    )?;
    size = ((eqsIn).len() as i32);
    (eqIdcs, varIdcs, resEqsOut) = List::fold(
        &(List::intRange(size)),
        &({
            let __pe_b1 = eqArr.clone();
            let __pe_b2 = varArr.clone();
            let __pe_b3 = m.clone();
            let __pe_b4 = mT.clone();
            let __pe_b5 = numAuxiliaryVars;
            let __pe_b6 = shared.clone();
            move |__pe_a0, __pe_a7| {
                Ok(simplifyNewEquations1(
                    __pe_a0,
                    __pe_b1.clone(),
                    &__pe_b2,
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    &__pe_b6,
                    __pe_a7,
                ))
            }
        }),
        (metamodelica::nil(), metamodelica::nil(), resEqsIn),
    )?;
    numAux = numAuxiliaryVars - ((varIdcs).len() as i32);
    if (varIdcs).is_empty() {
        numIterNew = 0;
    } else {
        numIterNew = numIter;
    }
    (_, varIdcs, _) = List::intersection1OnTrue(List::intRange(size), varIdcs, &fnptr!(intEq, i32, i32))?;
    (_, eqIdcs, _) = List::intersection1OnTrue(List::intRange(size), eqIdcs, &fnptr!(intEq, i32, i32))?;
    eqsOut = BackendEquation::getList(eqIdcs, eqArr)?;
    varsOut = List::map1(
        varIdcs,
        &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1),
        varArr,
    )?;
    if numIterNew != 0 {
        (eqsOut, varsOut, resEqsOut) =
            simplifyNewEquations(eqsOut, varsOut, resEqsOut, numAux, numIterNew - 1, shared)?;
    } else {
        (eqsOut, varsOut, resEqsOut) = (eqsOut, varsOut, resEqsOut);
    }
    Ok((eqsOut, varsOut, resEqsOut))
}

fn simplifyNewEquations1(
    mut eqIdx: i32,
    mut eqArr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut varArr: &BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut numAuxiliaryVars: i32,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut tplIn: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ),
) -> (
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) {
    let mut tplOut: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    );
    tplOut = 'mc: {
        let __mc_input = &tplIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varIdx: i32;
                    let mut varIdcs: metamodelica::List<i32>;
                    let mut eqIdcs: metamodelica::List<i32>;
                    let mut updEqIdcs: metamodelica::List<i32>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut varCref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut varExp: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut resEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (eqIdcs, varIdcs, resEqLst) = tplIn.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(m.clone(), eqIdx)?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varIdx = metamodelica::Own::own(__pa0);
                    let true = (varIdx <= numAuxiliaryVars) else { return Err("pattern mismatch") };
                    var = BackendVariable::getVarAt(varArr, varIdx)?;
                    eq = BackendEquation::get(eqArr.clone(), eqIdx)?;
                    varCref = BackendVariable::varCref(&var);
                    varExp = Expression::crefExp(varCref.clone())?;
                    rhs = BackendEquation::getEquationRHS(&eq)?;
                    lhs = BackendEquation::getEquationLHS(&eq)?;
                    (rhs, _) = ExpressionSolve::solve(lhs.clone(), rhs.clone(), varExp.clone(), None)?;
                    if Expression::isAsubExp(&rhs) {
                        rhs = List::fold1(&(Expression::allTerms(&rhs)), &fnptr!(Expression::makeBinaryExp, metamodelica::Ref<DAE::Exp>, DAE::Operator, metamodelica::Ref<DAE::Exp>), DAE::Operator::ADD { ty: Expression::r#typeof(varExp.clone())? }, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    }
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    repl = BackendVarTransform::emptyReplacements();
                    repl = BackendVarTransform::addReplacement(repl.clone(), varCref.clone(), rhs.clone(), None)?;
                    updEqIdcs = metamodelica::arrayGet(mt.clone(), varIdx)?;
                    eqLst = BackendEquation::getList(updEqIdcs.clone(), eqArr.clone())?;
                    (eqLst, _) = BackendVarTransform::replaceEquations(eqLst.clone(), &repl, None)?;
                    (resEqLst, _) = BackendVarTransform::replaceEquations(resEqLst.clone(), &repl, None)?;
                    List::threadFold(&updEqIdcs, eqLst.clone(), &BackendEquation::setAtIndexFirst, eqArr.clone())?;
                    varIdcs = metamodelica::cons(varIdx, varIdcs.clone());
                    eqIdcs = metamodelica::cons(eqIdx, eqIdcs.clone());
                    Ok((eqIdcs.clone(), varIdcs.clone(), resEqLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tplIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    tplOut
}

fn buildEqSystemComponent(
    mut eqIdcsIn: metamodelica::List<i32>,
    mut varIdcsIn: metamodelica::List<i32>,
    mut resEqsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut tVarsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut jacValuesIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outComp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut resEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut tVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut addEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (outComp, resEqsOut, tVarsOut, addEqsOut, addVarsOut) = 'mc: {
        let __mc_input = (&*eqIdcsIn, &*varIdcsIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: eqIdx, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: varIdx, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                    let true = (intEq(((eqIdcsIn).len() as i32), 1)) else { return Err("pattern mismatch") };
                    comp = metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx.clone(), var: varIdx.clone() });
                    Ok((list![comp.clone()], resEqsIn.clone(), tVarsIn.clone(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut resEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut addVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let true = (intLe(((tVarsIn).len() as i32), 3)) else { return Err("pattern mismatch") };
                    (resEqs, _, addEqs, addVars) = applyCramerRule(jacValuesIn.clone(), tVarsIn.clone())?;
                    comps = List::threadMap(eqIdcsIn.clone(), varIdcsIn.clone(), &fnptr!(BackendDAEUtil::makeSingleEquationComp, i32, i32))?;
                    Ok((comps.clone(), resEqs.clone(), tVarsIn.clone(), addEqs.clone(), addVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut jac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>;
                    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                    let mut jacValues: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
                    let mut mixedSystem: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(jacValuesIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    jacValues = metamodelica::Own::own(__pa0);
                    jac = buildLinearJacobian(jacValues.clone(), &(List::intRange(((resEqsIn).len() as i32))), List::intRange(((tVarsIn).len() as i32)))?;
                    mixedSystem = BackendVariable::hasDiscreteVar(&tVarsIn);
                    comp = metamodelica::Ref::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqIdcsIn.clone(), vars: varIdcsIn.clone(), jac: metamodelica::Ref::new(BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: jac.clone() }), jacType: openmodelica_backend_types::BackendDAE::JacobianType::JAC_LINEAR, mixedSystem: mixedSystem });
                    Ok((list![comp.clone()], resEqsIn.clone(), tVarsIn.clone(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outComp, resEqsOut, tVarsOut, addEqsOut, addVarsOut))
}

fn buildLinearJacobian(
    mut inElements: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut eqIdcs: &metamodelica::List<i32>,
    mut varIdcs: metamodelica::List<i32>,
) -> Result<Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>> {
    let mut outJac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>;
    let mut jac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
    jac = List::fold2(
        eqIdcs,
        &move |__a0: i32,
               __a1: metamodelica::List<i32>,
               __a2: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
               __a3: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>| {
            buildLinearJacobian1(__a0, __a1, &__a2, __a3)
        },
        varIdcs,
        inElements,
        metamodelica::nil(),
    )?;
    jac = jac.reverse();
    outJac = Some(jac);
    Ok(outJac)
}

fn buildLinearJacobian1(
    mut rowIdx: i32,
    mut columns: metamodelica::List<i32>,
    mut inElements: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut inJac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>> {
    let mut outJac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
    let mut elements: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    elements = (inElements).get(rowIdx)?;
    elements = List::map1(
        columns.clone(),
        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
        elements,
    )?;
    outJac = List::fold2(
        &columns,
        &move |__a0: i32,
               __a1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
               __a2: i32,
               __a3: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>| {
            buildLinearJacobian2(__a0, &__a1, __a2, __a3)
        },
        elements,
        rowIdx,
        inJac,
    )?;
    Ok(outJac)
}

fn buildLinearJacobian2(
    mut colIdx: i32,
    mut inElements: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut rowIdx: i32,
    mut inJac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>> {
    let mut outJac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut elem: metamodelica::Ref<BackendDAE::Var>;
    let mut entry: (i32, i32, metamodelica::Ref<BackendDAE::Equation>);
    elem = (inElements).get(colIdx)?;
    cref = BackendVariable::varCref(&elem);
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: cref,
        ty: DAE::T_REAL_DEFAULT().clone(),
    });
    exp = metamodelica::Ref::new(DAE::Exp::UNARY {
        operator: DAE::Operator::UMINUS {
            ty: DAE::T_REAL_DEFAULT().clone(),
        },
        exp: exp,
    });
    eq = metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION {
        exp: exp,
        source: DAE::emptyElementSource().clone(),
        attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
    });
    entry = (colIdx, rowIdx, eq);
    outJac = metamodelica::cons(entry, inJac);
    Ok(outJac)
}

fn updateMatching(
    mut idx: i32,
    mut offsetTpl: (i32, i32),
    mut matching2: (metamodelica::Array<i32>, metamodelica::Array<i32>),
    mut matching1In: (metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut matching1Out: (metamodelica::Array<i32>, metamodelica::Array<i32>);
    let mut eqOffset: i32;
    let mut varOffset: i32;
    let mut eqValue: i32;
    let mut varValue: i32;
    let mut ass11: metamodelica::Array<i32>;
    let mut ass21: metamodelica::Array<i32>;
    let mut ass12: metamodelica::Array<i32>;
    let mut ass22: metamodelica::Array<i32>;
    (eqOffset, varOffset) = offsetTpl;
    (ass12, ass22) = matching2;
    (ass11, ass21) = matching1In;
    eqValue = idx + eqOffset;
    varValue = metamodelica::arrayGet(ass22.clone(), idx)? + varOffset;
    ass11 = metamodelica::arrayUpdate(ass11.clone(), varValue, eqValue)?;
    ass21 = metamodelica::arrayUpdate(ass21.clone(), eqValue, varValue)?;
    matching1Out = (ass11.clone(), ass21.clone());
    Ok(matching1Out)
}

fn updateResidualMatching(
    mut idx: i32,
    mut tvars: &metamodelica::List<i32>,
    mut resEqs: &metamodelica::List<i32>,
    mut tplIn: (metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut tplOut: (metamodelica::Array<i32>, metamodelica::Array<i32>);
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut eqIdx: i32;
    let mut varIdx: i32;
    (ass1, ass2) = tplIn;
    eqIdx = (resEqs).get(idx)?;
    varIdx = (tvars).get(idx)?;
    ass1 = metamodelica::arrayUpdate(ass1.clone(), varIdx, eqIdx)?;
    ass2 = metamodelica::arrayUpdate(ass2.clone(), eqIdx, varIdx)?;
    tplOut = (ass1.clone(), ass2.clone());
    Ok(tplOut)
}

fn getOtherComps(
    mut innerEquations: &metamodelica::List<BackendDAE::InnerEquation>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<BackendDAE::Matching>> {
    let mut matchingOut: metamodelica::Ref<BackendDAE::Matching>;
    let mut ass1Tmp: metamodelica::Array<i32>;
    let mut ass2Tmp: metamodelica::Array<i32>;
    let mut compsTmp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    (ass1Tmp, ass2Tmp, compsTmp) = List::fold(
        innerEquations,
        &move |__a0: BackendDAE::InnerEquation,
               __a1: (
            metamodelica::Array<i32>,
            metamodelica::Array<i32>,
            metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        )| getOtherComps1(&__a0, &__a1),
        (ass1.clone(), ass2.clone(), metamodelica::nil()),
    )?;
    compsTmp = compsTmp.reverse();
    matchingOut = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
        ass1: ass1Tmp.clone(),
        ass2: ass2Tmp.clone(),
        comps: compsTmp,
    });
    Ok(matchingOut)
}

fn getOtherComps1(
    mut innerEquation: &BackendDAE::InnerEquation,
    mut tplIn: &(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
)> {
    let mut tplOut: (
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    );
    tplOut = 'mc: {
        let __mc_input = tplIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ass1, ass2, compsIn) => {
                    let mut eqIdx: i32;
                    let mut varIdx: i32;
                    let mut varIdcs: metamodelica::List<i32>;
                    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                    let mut compsTmp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut ass1 = (*ass1).clone();
                    let mut ass2 = (*ass2).clone();
                    (eqIdx, varIdcs, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation);
                    let true = (((varIdcs).len() as i32) == 1) else { return Err("pattern mismatch") };
                    varIdx = (varIdcs).get(1)?;
                    comp = metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx, var: varIdx });
                    ass1 = metamodelica::arrayUpdate(ass1.clone(), varIdx, eqIdx)?;
                    ass2 = metamodelica::arrayUpdate(ass2.clone(), eqIdx, varIdx)?;
                    compsTmp = metamodelica::cons(comp.clone(), compsIn.clone());
                    Ok((ass1.clone(), ass2.clone(), compsTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getOtherComps failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(tplOut)
}

fn replaceAtPositionFromList<ElementType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut n: i32,
    mut replacingLst: &metamodelica::List<ElementType>,
    mut positionLst: &metamodelica::List<i32>,
    mut inLst: metamodelica::List<ElementType>,
) -> Result<metamodelica::List<ElementType>> {
    let mut outLst: metamodelica::List<ElementType>;
    let mut idx: i32;
    let mut entry: ElementType;
    idx = (positionLst).get(n)?;
    entry = (replacingLst).get(n)?;
    outLst = List::replaceAt(entry, idx, inLst)?;
    Ok(outLst)
}

fn updateIndicesInComp(
    mut compIn: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut varOffset: i32,
    mut eqOffset: i32,
) -> Result<metamodelica::Ref<BackendDAE::StrongComponent>> {
    let mut compOut: metamodelica::Ref<BackendDAE::StrongComponent>;
    compOut = (match &**compIn {
        BackendDAE::StrongComponent::SINGLEEQUATION {
            eqn: eqIdx,
            var: varIdx,
        } => {
            let mut compTmp: metamodelica::Ref<BackendDAE::StrongComponent>;
            let mut eqIdx = (*eqIdx).clone();
            let mut varIdx = (*varIdx).clone();
            varIdx = varIdx.clone() + varOffset;
            eqIdx = eqIdx.clone() + eqOffset;
            compTmp = metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEEQUATION {
                eqn: eqIdx.clone(),
                var: varIdx.clone(),
            });
            compTmp
        }
        _ => {
            metamodelica::print(literal!("updateVarEqIndices failed\n"));
            return Err("fail");
        }
    });
    Ok(compOut)
}

fn buildNewResidualEquation(
    mut resIdx: i32,
    mut aCoeffLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut a0CoeffLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut tvars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut resEqsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut resEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    resEqsOut = 'mc: {
        let __mc_input = &**resEqsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqLstTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let true = (resIdx > ((tvars).len() as i32)) else { return Err("pattern mismatch") };
                    eqLstTmp = resEqsIn.clone().reverse();
                    Ok(eqLstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqLstTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut aCoeffs: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut hs: metamodelica::Ref<BackendDAE::Equation>;
                    let mut a0Coeff: metamodelica::Ref<BackendDAE::Var>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut a0Exp: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let true = (resIdx <= ((tvars).len() as i32)) else { return Err("pattern mismatch") };
                    aCoeffs = List::map1(aCoeffLst.clone(), &listGet, resIdx)?;
                    a0Coeff = (a0CoeffLst).get(resIdx)?;
                    a0Exp = varExp(&a0Coeff);
                    ty = DAE::T_REAL_DEFAULT().clone();
                    rhs = buildNewResidualEquation2(1, &aCoeffs, tvars, &(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))?;
                    rhs = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: rhs.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: a0Exp.clone() });
                    lhs = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
                    hs = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    eqLstTmp = metamodelica::cons(hs.clone(), resEqsIn.clone());
                    eqLstTmp = buildNewResidualEquation(resIdx + 1, aCoeffLst, a0CoeffLst, tvars, &eqLstTmp)?;
                    Ok(eqLstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("buildNewResidualEquation failed"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(resEqsOut)
}

fn buildNewResidualEquation2(
    mut idx: i32,
    mut coeffs: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut tVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut expIn: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    expOut = 'mc: {
        let __mc_input = &**expIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut coeff: metamodelica::Ref<BackendDAE::Var>;
                    let mut tVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut coeffExp: metamodelica::Ref<DAE::Exp>;
                    let mut tVarExp: metamodelica::Ref<DAE::Exp>;
                    let mut expTmp: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let true = (idx == 1) else { return Err("pattern mismatch") };
                    coeff = (coeffs).get(idx)?;
                    coeffExp = varExp(&coeff);
                    tVar = (tVars).get(idx)?;
                    tVarExp = varExp(&tVar);
                    tVarExp = if (BackendVariable::isStateVar(&tVar)) {Expression::expDer(tVarExp.clone())} else {tVarExp.clone()};
                    ty = DAE::T_REAL_DEFAULT().clone();
                    expTmp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: coeffExp.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: tVarExp.clone() });
                    expTmp = buildNewResidualEquation2(idx + 1, coeffs, tVars, &expTmp)?;
                    Ok(expTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut coeff: metamodelica::Ref<BackendDAE::Var>;
                    let mut tVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut expTmp: metamodelica::Ref<DAE::Exp>;
                    let true = (idx <= ((tVars).len() as i32)) else { return Err("pattern mismatch") };
                    coeff = (coeffs).get(idx)?;
                    tVar = (tVars).get(idx)?;
                    expTmp = addProductToExp(&coeff, &tVar, expIn.clone());
                    expTmp = buildNewResidualEquation2(idx + 1, coeffs, tVars, &expTmp)?;
                    Ok(expTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (idx > ((tVars).len() as i32)) else { return Err("pattern mismatch") };
                    Ok(expIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("buildNewResidualEquation2 failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(expOut)
}

fn addProductToExp(
    mut var1: &metamodelica::Ref<BackendDAE::Var>,
    mut var2: &metamodelica::Ref<BackendDAE::Var>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    let mut fac1: metamodelica::Ref<DAE::Exp>;
    let mut fac2: metamodelica::Ref<DAE::Exp>;
    let mut prod: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    fac1 = varExp(var1);
    fac2 = varExp(var2);
    fac2 = if (BackendVariable::isStateVar(var2)) {
        Expression::expDer(fac2)
    } else {
        fac2
    };
    ty = DAE::T_REAL_DEFAULT().clone();
    prod = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: fac1,
        operator: DAE::Operator::MUL { ty: ty.clone() },
        exp2: fac2,
    });
    expOut = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: inExp,
        operator: DAE::Operator::ADD { ty: ty },
        exp2: prod,
    });
    expOut
}

fn buildSingleEquationSystem(
    mut eqSizeOrig: i32,
    mut inEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut compsIn: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
) -> Result<metamodelica::Ref<BackendDAE::Matching>> {
    let mut matchingOut: metamodelica::Ref<BackendDAE::Matching>;
    matchingOut = 'mc: {
        let __mc_input = &*compsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut nVars: i32;
                    let mut nEqs: i32;
                    let mut eqArr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut sysTmp: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
                    let mut matchingTmp: metamodelica::Ref<BackendDAE::Matching>;
                    let mut compsTmp: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut vars: BackendDAE::Variables;
                    vars = BackendVariable::listVar1(inVars)?;
                    eqArr = BackendEquation::listEquation(inEqs)?;
                    sysTmp = BackendDAEUtil::createEqSystem(vars.clone(), eqArr.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (sysTmp, m, _) = BackendDAEUtil::getAdjacencyMatrix(sysTmp.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(&shared))?;
                    nVars = ((inVars).len() as i32);
                    nEqs = ((inEqs).len() as i32);
                    ass1 = arrayCreate(nVars, -1);
                    ass2 = arrayCreate(nEqs, -1);
                    Matching::matchingExternalsetAdjacencyMatrix(nVars, nEqs, m.clone())?;
                    BackendDAEEXT::matching(nVars, nEqs, 5, -1, metamodelica::OrderedFloat(0.0_f64), 1);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() });
                    sysTmp = BackendDAEUtil::createEqSystem(vars.clone(), eqArr.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (sysTmp, _, _) = BackendDAEUtil::getAdjacencyMatrix(sysTmp.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(&shared))?;
                    sysTmp = BackendDAEUtil::setEqSystMatching(sysTmp.clone(), matching.clone());
                    mapIncRowEqn = Array::createIntRange(nEqs);
                    mapEqnIncRow = Array::map(mapIncRowEqn.clone(), &fnptr!(List::create, _))?;
                    (sysTmp, compsTmp) = BackendDAETransform::strongComponentsScalar(sysTmp.clone(), shared.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
                    compsTmp = listAppend(compsIn.clone(), compsTmp.clone());
                    matchingTmp = metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: compsTmp.clone() });
                    Ok(matchingTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("buildSingleEquationSystem failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(matchingOut)
}

fn getTornSystemCoefficients(
    mut iValueRange: &metamodelica::List<i32>,
    mut numTVars: i32,
    mut tornSysIdx: i32,
    mut h_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut hs_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut a_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
)> {
    let mut hs_iArrOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut a_iArrOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    (hs_iArrOut, a_iArrOut) = 'mc: {
        let __mc_input = &**iValueRange;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((hs_iArrIn.clone(), a_iArrIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: iValue, tail: iLstRest } => {
                    let mut hs_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut a_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients1(&(List::intRange(numTVars).reverse()), iValue.clone(), h_iArr.clone(), hs_iArrIn.clone(), a_iArrIn.clone(), tornSysIdx)?;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients(metamodelica::AsArg::as_arg(&iLstRest), numTVars, tornSysIdx, h_iArr.clone(), hs_iArrTmp.clone(), a_iArrTmp.clone())?;
                    Ok((hs_iArrTmp.clone(), a_iArrTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getTornSystemCoefficients failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((hs_iArrOut, a_iArrOut))
}

fn getTornSystemCoefficients1(
    mut resIdxLst: &metamodelica::List<i32>,
    mut iIdx: i32,
    mut h_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut hs_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut a_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut tornSysIdx: i32,
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
)> {
    let mut hs_iArrOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut a_iArrOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    (hs_iArrOut, a_iArrOut) = 'mc: {
        let __mc_input = &**resIdxLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((hs_iArrIn.clone(), a_iArrIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: resIdx, tail: resIdxRest } => {
                    let mut aName: ArcStr;
                    let mut hs_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut a_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
                    let mut hs_iTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut a_iTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut hs_ii: metamodelica::Ref<BackendDAE::Equation>;
                    let mut a_ii: metamodelica::Ref<BackendDAE::Var>;
                    let mut aCRef: metamodelica::Ref<DAE::ComponentRef>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let true = (intEq(0, iIdx)) else { return Err("pattern mismatch") };
                    aName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$a")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(resIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(iIdx)); ArcStr::from(__mm_s) };
                    ty = DAE::T_REAL_DEFAULT().clone();
                    aCRef = ComponentReferenceBasics::makeCrefIdent(aName.clone(), ty.clone(), metamodelica::nil());
                    a_ii = metamodelica::Ref::new(BackendDAE::Var { varName: aCRef.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
                    a_ii = BackendVariable::setVarStartValue(a_ii.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    lhs = varExp(&a_ii);
                    rhs = ((metamodelica::arrayGet(h_iArr.clone(), iIdx + 1)?)).get(resIdx.clone())?;
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    hs_ii = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    hs_iTmp = metamodelica::arrayGet(hs_iArrIn.clone(), iIdx + 1)?;
                    hs_iTmp = metamodelica::cons(hs_ii.clone(), hs_iTmp.clone());
                    hs_iArrTmp = metamodelica::arrayUpdate(hs_iArrIn.clone(), iIdx + 1, hs_iTmp.clone())?;
                    a_iArrTmp = a_iArrIn.clone();
                    a_iTmp = metamodelica::arrayGet(a_iArrIn.clone(), iIdx + 1)?;
                    a_iTmp = metamodelica::cons(a_ii.clone(), a_iTmp.clone());
                    a_iArrTmp = metamodelica::arrayUpdate(a_iArrIn.clone(), iIdx + 1, a_iTmp.clone())?;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients1(metamodelica::AsArg::as_arg(&resIdxRest), iIdx, h_iArr.clone(), hs_iArrTmp.clone(), a_iArrTmp.clone(), tornSysIdx)?;
                    Ok((hs_iArrTmp.clone(), a_iArrTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: resIdx, tail: resIdxRest } => {
                    let mut aName: ArcStr;
                    let mut hs_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut a_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
                    let mut hs_iTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut a_iTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut d_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut hs_ii: metamodelica::Ref<BackendDAE::Equation>;
                    let mut a_ii: metamodelica::Ref<BackendDAE::Var>;
                    let mut dVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut aCRef: metamodelica::Ref<DAE::ComponentRef>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut dExp: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let true = (iIdx > 0) else { return Err("pattern mismatch") };
                    aName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$a")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(resIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(iIdx)); ArcStr::from(__mm_s) };
                    ty = DAE::T_REAL_DEFAULT().clone();
                    aCRef = ComponentReferenceBasics::makeCrefIdent(aName.clone(), ty.clone(), metamodelica::nil());
                    a_ii = metamodelica::Ref::new(BackendDAE::Var { varName: aCRef.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
                    a_ii = BackendVariable::setVarStartValue(a_ii.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    d_lst = metamodelica::arrayGet(a_iArrIn.clone(), 1)?;
                    dVar = (d_lst).get(resIdx.clone())?;
                    dExp = varExp(&dVar);
                    lhs = varExp(&a_ii);
                    rhs = ((metamodelica::arrayGet(h_iArr.clone(), iIdx + 1)?)).get(resIdx.clone())?;
                    rhs = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: rhs.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: dExp.clone() });
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    hs_ii = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    hs_iTmp = metamodelica::arrayGet(hs_iArrIn.clone(), iIdx + 1)?;
                    hs_iTmp = metamodelica::cons(hs_ii.clone(), hs_iTmp.clone());
                    hs_iArrTmp = metamodelica::arrayUpdate(hs_iArrIn.clone(), iIdx + 1, hs_iTmp.clone())?;
                    a_iArrTmp = a_iArrIn.clone();
                    a_iTmp = metamodelica::arrayGet(a_iArrIn.clone(), iIdx + 1)?;
                    a_iTmp = metamodelica::cons(a_ii.clone(), a_iTmp.clone());
                    a_iArrTmp = metamodelica::arrayUpdate(a_iArrIn.clone(), iIdx + 1, a_iTmp.clone())?;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients1(metamodelica::AsArg::as_arg(&resIdxRest), iIdx, h_iArr.clone(), hs_iArrTmp.clone(), a_iArrTmp.clone(), tornSysIdx)?;
                    Ok((hs_iArrTmp.clone(), a_iArrTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getTornSystemCoefficients1 failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((hs_iArrOut, a_iArrOut))
}

fn varExp(mut varIn: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<DAE::Exp> {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = BackendVariable::varType(varIn);
    cr = BackendVariable::varCref(varIn);
    expOut = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: cr,
        ty: ty,
    });
    expOut
}

fn getResidualExpressions(
    mut iIn: &metamodelica::List<i32>,
    mut resEqLstIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut replArrIn: metamodelica::Array<BackendVarTransform::VariableReplacements>,
    mut h_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut h_iArrOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut resExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    resExps = List::map(resEqLstIn, &move |__a0: metamodelica::Ref<BackendDAE::Equation>| {
        getResidualExpressionForEquation(&__a0)
    })?;
    h_iArrOut = List::fold2(
        iIn,
        &move |__a0: i32,
               __a1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
               __a2: metamodelica::Array<BackendVarTransform::VariableReplacements>,
               __a3: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>| {
            getResidualExpressions1(__a0, &__a1, __a2, __a3)
        },
        resExps,
        replArrIn.clone(),
        h_iArrIn.clone(),
    )?;
    Ok(h_iArrOut)
}

fn getResidualExpressions1(
    mut i: i32,
    mut resExpsIn: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut replArr: metamodelica::Array<BackendVarTransform::VariableReplacements>,
    mut h_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut h_iArrOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut repl: BackendVarTransform::VariableReplacements =
        <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut h_i: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut h_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = Default::default();
    h_iArrOut = 'mc: {
        let __mc_input = h_iArrIn.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut h_i: metamodelica::List<metamodelica::Ref<DAE::Exp>> = h_i.clone();
            let mut h_iArr: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = h_iArr.clone();
            let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
            repl = metamodelica::arrayGet(replArr.clone(), i + 1)?;
            (h_i, _) = BackendVarTransform::replaceExpList1(resExpsIn, &repl, None);
            h_iArr = metamodelica::arrayUpdate(h_iArrIn.clone(), i + 1, h_i.clone())?;
            Ok((h_iArr.clone(), h_i.clone(), h_iArr.clone(), repl.clone()))
        })() {
            h_i = __wb0;
            h_iArr = __wb1;
            repl = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("getResidualExpressions failed \n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(h_iArrOut)
}

fn getResidualExpressionForEquation(
    mut eq: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (match &**eq {
        BackendDAE::Equation::EQUATION {
            exp: lhs, scalar: rhs, ..
        } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut rhs = (*rhs).clone();
            ty = Expression::r#typeof(lhs.clone())?;
            rhs = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: rhs.clone(),
                operator: DAE::Operator::SUB { ty: ty },
                exp2: lhs.clone(),
            });
            (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
            rhs.clone()
        }
        _ => {
            metamodelica::print(literal!("getResidualExpressionForEquation failed\n"));
            return Err("fail");
        }
    });
    Ok(exp)
}

fn varInFrontList(
    mut varIn: metamodelica::Ref<BackendDAE::Var>,
    mut lstLstIn: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>> {
    let mut lstLstOut: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    lstLstOut = (::match_deref::match_deref! { match &(lstLstIn.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            lstLstIn
        },
        _ => {
            let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            varLst = (lstLstIn).head().cloned()?;
            varLst = metamodelica::cons(varIn, varLst);
            lstLstOut = List::replaceAt(varLst, 1, lstLstIn)?;
            lstLstOut
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lstLstOut)
}

fn eqInFrontList(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut lstLstIn: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>> {
    let mut lstLstOut: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    lstLstOut = (::match_deref::match_deref! { match &(lstLstIn.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            lstLstIn
        },
        _ => {
            let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            eqLst = (lstLstIn).head().cloned()?;
            eqLst = metamodelica::cons(eqIn, eqLst);
            lstLstOut = List::replaceAt(eqLst, 1, lstLstIn)?;
            lstLstOut
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lstLstOut)
}

fn getAlgebraicEquationsForEI(
    mut iIn: &metamodelica::List<i32>,
    mut size: i32,
    mut otherEqLstIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut tvarLstIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut tVarCRefLstIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut otherVarLstIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut oVarCRefLstIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut g_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut xa_iArrIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut replacementArrIn: metamodelica::Array<BackendVarTransform::VariableReplacements>,
    mut tornSysIdx: i32,
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    metamodelica::Array<BackendVarTransform::VariableReplacements>,
)> {
    let mut g_i_Out: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut xa_i_Out: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut replacementArrOut: metamodelica::Array<BackendVarTransform::VariableReplacements>;
    (g_i_Out, xa_i_Out, replacementArrOut) = 'mc: {
        let __mc_input = &**iIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((g_iArrIn.clone(), xa_iArrIn.clone(), replacementArrIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: iValue, tail: iLstRest } => {
                    let mut gEqLstTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut xaVarLstTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut replArrTmp: metamodelica::Array<BackendVarTransform::VariableReplacements>;
                    let mut g_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut xa_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
                    let mut replTmp: BackendVarTransform::VariableReplacements;
                    let true = (iValue.clone() == 0) else { return Err("pattern mismatch") };
                    replTmp = BackendVarTransform::emptyReplacementsSized(size);
                    replTmp = List::fold1(tVarCRefLstIn, &replaceTVarWithReal, metamodelica::OrderedFloat(0.0_f64), replTmp.clone())?;
                    (xaVarLstTmp, replTmp) = List::fold2(&(List::intRange(((oVarCRefLstIn).len() as i32))), &move |__a0: i32, __a1: ArcStr, __a2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, __a3: (metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)| replaceOtherVarsWithPrefixCref(__a0, __a1, &__a2, &__a3), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$xa")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*literal!("0")); ArcStr::from(__mm_s) }, oVarCRefLstIn.clone(), (metamodelica::nil(), replTmp.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(otherEqLstIn.clone(), &replTmp, None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    gEqLstTmp = metamodelica::Own::own(__pa0);
                    g_iArrTmp = metamodelica::arrayUpdate(g_iArrIn.clone(), iValue.clone() + 1, gEqLstTmp.clone())?;
                    xa_iArrTmp = metamodelica::arrayUpdate(xa_iArrIn.clone(), iValue.clone() + 1, xaVarLstTmp.clone())?;
                    replArrTmp = metamodelica::arrayUpdate(replacementArrIn.clone(), iValue.clone() + 1, replTmp.clone())?;
                    (g_iArrTmp, xa_iArrTmp, replArrTmp) = getAlgebraicEquationsForEI(metamodelica::AsArg::as_arg(&iLstRest), size, otherEqLstIn, tvarLstIn, tVarCRefLstIn, otherVarLstIn, oVarCRefLstIn, g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone(), tornSysIdx)?;
                    Ok((g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: iValue, tail: iLstRest } => {
                    let mut str1: ArcStr;
                    let mut gEqLstTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut xaVarLstTmp: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut replArrTmp: metamodelica::Array<BackendVarTransform::VariableReplacements>;
                    let mut tVarCRefLst1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut g_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut xa_iArrTmp: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
                    let mut replTmp: BackendVarTransform::VariableReplacements;
                    let mut tVarCRef: metamodelica::Ref<DAE::ComponentRef>;
                    let true = (iValue.clone() > 0) else { return Err("pattern mismatch") };
                    str1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$xa")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*intString(iValue.clone())); ArcStr::from(__mm_s) };
                    tVarCRef = (tVarCRefLstIn).get(iValue.clone())?;
                    tVarCRefLst1 = listDelete(tVarCRefLstIn.clone(), iValue.clone())?;
                    replTmp = BackendVarTransform::emptyReplacementsSized(size);
                    replTmp = replaceTVarWithReal(tVarCRef.clone(), metamodelica::OrderedFloat(1.0_f64), replTmp.clone())?;
                    replTmp = List::fold1(&tVarCRefLst1, &replaceTVarWithReal, metamodelica::OrderedFloat(0.0_f64), replTmp.clone())?;
                    (xaVarLstTmp, replTmp) = List::fold2(&(List::intRange(((oVarCRefLstIn).len() as i32))), &move |__a0: i32, __a1: ArcStr, __a2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, __a3: (metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)| replaceOtherVarsWithPrefixCref(__a0, __a1, &__a2, &__a3), str1.clone(), oVarCRefLstIn.clone(), (metamodelica::nil(), replTmp.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(otherEqLstIn.clone(), &replTmp, None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    gEqLstTmp = metamodelica::Own::own(__pa0);
                    g_iArrTmp = metamodelica::arrayUpdate(g_iArrIn.clone(), iValue.clone() + 1, gEqLstTmp.clone())?;
                    xa_iArrTmp = metamodelica::arrayUpdate(xa_iArrIn.clone(), iValue.clone() + 1, xaVarLstTmp.clone())?;
                    replArrTmp = metamodelica::arrayUpdate(replacementArrIn.clone(), iValue.clone() + 1, replTmp.clone())?;
                    (g_iArrTmp, xa_iArrTmp, replArrTmp) = getAlgebraicEquationsForEI(metamodelica::AsArg::as_arg(&iLstRest), size, otherEqLstIn, tvarLstIn, tVarCRefLstIn, otherVarLstIn, oVarCRefLstIn, g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone(), tornSysIdx)?;
                    Ok((g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getAlgebraicEquationsForEI failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((g_i_Out, xa_i_Out, replacementArrOut))
}

fn replaceTVarWithReal(
    mut tVarCRefIn: metamodelica::Ref<DAE::ComponentRef>,
    mut realIn: metamodelica::Real,
    mut replacementIn: BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replacementOut: BackendVarTransform::VariableReplacements;
    replacementOut = BackendVarTransform::addReplacement(
        replacementIn,
        tVarCRefIn,
        metamodelica::Ref::new(DAE::Exp::RCONST { real: realIn }),
        None,
    )?;
    Ok(replacementOut)
}

fn replaceOtherVarsWithPrefixCref(
    mut indxIn: i32,
    mut prefix: ArcStr,
    mut oVarCRefLstIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut tplIn: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        BackendVarTransform::VariableReplacements,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut tplOut: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        BackendVarTransform::VariableReplacements,
    );
    let mut replVarLstIn: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut replVarLstOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut replVar: metamodelica::Ref<BackendDAE::Var>;
    let mut replacementIn: BackendVarTransform::VariableReplacements;
    let mut replacementOut: BackendVarTransform::VariableReplacements;
    let mut cRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut oVarCRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut varExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    (replVarLstIn, replacementIn) = tplIn.clone();
    oVarCRef = (oVarCRefLstIn).get(indxIn)?;
    cRef = ComponentReferenceBasics::makeCrefQual(
        prefix,
        DAE::T_COMPLEX_DEFAULT().clone(),
        metamodelica::nil(),
        oVarCRef.clone(),
    );
    cRef = ComponentReference::replaceSubsWithString(&cRef)?;
    cRef = ComponentReference::crefSetLastType(&cRef, &(DAE::T_REAL_DEFAULT().clone()))?;
    varExp = Expression::crefExp(cRef.clone())?;
    replacementOut = BackendVarTransform::addReplacement(replacementIn, oVarCRef, varExp, None)?;
    ty = ComponentReference::crefLastType(&cRef)?;
    replVar = metamodelica::Ref::new(BackendDAE::Var {
        varName: cRef,
        varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: ty,
        bindExp: None,
        tplExp: None,
        arryDim: metamodelica::nil(),
        source: DAE::emptyElementSource().clone(),
        values: None,
        tearingSelectOption: None,
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: false,
    });
    replVar = BackendVariable::setVarStartValue(
        replVar,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    )?;
    replVarLstOut = metamodelica::cons(replVar, replVarLstIn);
    tplOut = (replVarLstOut, replacementOut);
    Ok(tplOut)
}

//--------------------------------------------------//
// get EqSystem object
//-------------------------------------------------//
fn getEqSystem(
    mut eqLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<EqSys> {
    let mut syst: EqSys;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    syst = createEqSystem(varLst.clone());
    crefs = List::map(
        varLst,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        },
    )?;
    (syst, _) = List::fold1(
        eqLst,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
               __a2: (EqSys, i32)| getEqSystem2(&__a0, &__a1, &__a2),
        crefs,
        (syst, 1),
    )?;
    Ok(syst)
}

fn createEqSystem(mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> EqSys {
    let mut sys: EqSys;
    let mut dim: i32;
    let mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    dim = ((varLst).len() as i32);
    matrixA = arrayCreate(dim, metamodelica::nil());
    vectorB = arrayCreate(
        dim,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    sys = EqSys {
        dim: dim,
        matrixA: matrixA.clone(),
        vectorB: vectorB.clone(),
        vectorX: metamodelica::arrayFromVec(varLst.into_iter().cloned().collect()),
    };
    sys
}

fn getEqSystem2(
    mut eq: &metamodelica::Ref<BackendDAE::Equation>,
    mut crefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut foldIn: &(EqSys, i32),
) -> Result<(EqSys, i32)> {
    let mut foldOut: (EqSys, i32);
    let mut idx: i32;
    let mut dim: i32;
    let mut summands: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut coeffs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut offsetLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut offset: metamodelica::Ref<DAE::Exp>;
    let mut sys: EqSys;
    let mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>;
    (sys, idx) = foldIn.clone();
    summands = getSummands(eq)?;
    (summands, _) = List::map_2(&summands, &ExpressionSimplify::simplify)?;
    (offsetLst, coeffs) = List::fold(
        crefs,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>,
               __a1: (
            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
        )| getEqSystem3(__a0, &__a1),
        (summands, metamodelica::nil()),
    )?;
    if (offsetLst).is_empty() {
        offset = metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        });
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(offsetLst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        offset = metamodelica::Own::own(__pa0);
        offsetLst = metamodelica::Own::own(__pa1);
    }
    offset = List::fold(&offsetLst, &Expression::expAdd, offset)?;
    offset = Expression::negate(offset)?;
    let EqSys {
        dim: __pa2,
        matrixA: __pa3,
        vectorB: __pa4,
        vectorX: __pa5,
    } = sys;
    dim = metamodelica::Own::own(__pa2);
    matrixA = metamodelica::Own::own(__pa3);
    vectorB = metamodelica::Own::own(__pa4);
    vectorX = metamodelica::Own::own(__pa5);
    matrixA = metamodelica::arrayUpdate(matrixA.clone(), idx, coeffs.reverse())?;
    vectorB = metamodelica::arrayUpdate(vectorB.clone(), idx, offset)?;
    sys = EqSys {
        dim: dim,
        matrixA: matrixA.clone(),
        vectorB: vectorB.clone(),
        vectorX: vectorX.clone(),
    };
    foldOut = (sys, idx + 1);
    Ok(foldOut)
}

fn getEqSystem3(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut foldIn: &(
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut foldOut: (
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    let mut coeff: metamodelica::Ref<DAE::Exp>;
    let mut allTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut coeffs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut coeffsIn: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (allTerms, coeffsIn) = foldIn.clone();
    (coeffs, allTerms) = List::extract1OnTrue(&allTerms, &Expression::expHasCref, cref.clone())?;
    coeff = List::fold(
        &coeffs,
        &Expression::expAdd,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat((0) as f64),
        }),
    )?;
    if containsFunctioncallOfCref(coeff.clone(), cref.clone())? {
        metamodelica::print(literal!(
            "This system of equations cannot be decomposed because its actually not linear (the coeffs are function calls of x).\n"
        ));
        return Err("fail");
    }
    (coeff, _) = Expression::replaceExp(
        coeff,
        Expression::crefExp(cref)?,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(1.0_f64),
        }),
    )?;
    (coeff, _) = ExpressionSimplify::simplify(coeff)?;
    foldOut = (allTerms, metamodelica::cons(coeff, coeffsIn));
    Ok(foldOut)
}

fn containsFunctioncallOfCref(
    mut expIn: metamodelica::Ref<DAE::Exp>,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut hasCrefInCall: bool;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    if Expression::containFunctioncall(expIn.clone())? {
        (_, expLst) = Expression::traverseExpBottomUp(
            expIn,
            &fnptr!(
                getCallExpLst,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::Exp>>
            ),
            metamodelica::nil(),
        )?;
        hasCrefInCall = List::fold(
            &(List::map1(expLst, &Expression::expHasCref, cref)?),
            &fnptr!(boolOr, bool, bool),
            false,
        )?;
    } else {
        hasCrefInCall = false;
    }
    Ok(hasCrefInCall)
}

fn getCallExpLst(
    mut eIn: metamodelica::Ref<DAE::Exp>,
    mut eLstIn: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) {
    let mut eOut: metamodelica::Ref<DAE::Exp>;
    let mut eLstOut: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (eOut, eLstOut) = (match &*eIn.clone() {
        DAE::Exp::CALL { expLst, .. } => (eIn, listAppend(expLst.clone(), eLstIn)),
        _ => (eIn, eLstIn),
    });
    (eOut, eLstOut)
}

fn getSummands(
    mut eq: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    exps = 'mc: {
        let __mc_input = &**eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, .. } => {
                    let mut expLst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expLst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst1 = Expression::allTerms(metamodelica::AsArg::as_arg(&lhs));
                    expLst1 = List::map(expLst1.clone(), &Expression::negate)?;
                    expLst2 = Expression::allTerms(metamodelica::AsArg::as_arg(&rhs));
                    expLst2 = listAppend(expLst1.clone(), expLst2.clone());
                    Ok(expLst2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSummands failed! for")); __mm_s.push_str(&*BackendDump::equationString(eq)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(exps)
}

//--------------------------------------------------//
// Chios Condensation
//-------------------------------------------------//
fn chiosCondensation(
    mut systemIn: EqSys,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut newResEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut dim: i32;
    let mut vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>;
    let mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let EqSys {
        dim: __pa0,
        matrixA: __pa1,
        vectorB: __pa2,
        vectorX: __pa3,
    } = &systemIn;
    dim = metamodelica::Own::own(__pa0);
    matrixA = metamodelica::Own::own(__pa1);
    vectorB = metamodelica::Own::own(__pa2);
    vectorX = metamodelica::Own::own(__pa3);
    (addEqsOut, addVarsOut) = ChiosCondensation2(&systemIn, 1, &(metamodelica::nil()), &(metamodelica::nil()))?;
    addEqsOut = addEqsOut.reverse();
    addVarsOut = addVarsOut.reverse();
    newResEqs = generateCramerEqs(
        &(List::intRange(dim).reverse()),
        dim,
        vectorX.clone(),
        vectorB.clone(),
        matrixA.clone(),
        &(metamodelica::nil()),
    )?;
    newResEqs = newResEqs.reverse();
    Ok((newResEqs, addEqsOut, addVarsOut))
}

fn ChiosCondensation2(
    mut systemIn: &EqSys,
    mut iterIdx: i32,
    mut addEqsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut addVarsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut addEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (addEqsOut, addVarsOut) = 'mc: {
        let __mc_input = systemIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys {
                dim: mut dim,
                vectorX: mut vectorX,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut syst: EqSys;
            let mut matrixB: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut vecAi: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
            let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut addVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let true = (intGt(dim.clone(), 1)) else {
                return Err("pattern mismatch");
            };
            matrixB = arrayCreate(dim.clone() - 1, metamodelica::nil());
            vecAi = arrayCreate(
                dim.clone() - 1,
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                }),
            );
            (matrixB, vecAi, addEqs, addVars) = List::fold(
                &(List::intRange2(2, dim.clone())),
                &({
                    let __pe_b1 = systemIn.clone();
                    let __pe_b2 = iterIdx;
                    move |__pe_a0, __pe_a3| getNewChioRow(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
                }),
                (matrixB.clone(), vecAi.clone(), addEqsIn.clone(), addVarsIn.clone()),
            )?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("matrixB"));
                __mm_s.push_str(&*intString(dim.clone()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            dumpMatrix(matrixB.clone())?;
            metamodelica::print(literal!("vecAi\n"));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut e in (vecAi.clone()).borrow().iter() {
                            let __x = ExpressionDump::dumpExpStr(e.clone(), 0)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            BackendDump::dumpEquationList(&addEqs, &(literal!("new det eqs")))?;
            syst = EqSys {
                dim: dim.clone() - 1,
                matrixA: matrixB.clone(),
                vectorB: vecAi.clone(),
                vectorX: vectorX.clone(),
            };
            Ok(ChiosCondensation2(&syst, iterIdx + 1, &addEqs, &addVars)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys {
                dim: mut dim,
                matrixA: mut matrixA,
                vectorB: mut vecAi,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("end matrixB"));
                __mm_s.push_str(&*intString(dim.clone()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            dumpMatrix(matrixA.clone())?;
            metamodelica::print(literal!("end vecAi\n"));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut e in (vecAi.clone()).borrow().iter() {
                            let __x = ExpressionDump::dumpExpStr(e.clone(), 0)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            BackendDump::dumpEquationList(addEqsIn, &(literal!("new det eqs")))?;
            Ok((addEqsIn.clone(), addVarsIn.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((addEqsOut, addVarsOut))
}

fn generateCramerEqs(
    mut varIdcs: &metamodelica::List<i32>,
    mut dim: i32,
    mut vectorX: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>,
    mut vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut eqsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut eqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    eqsOut = 'mc: {
        let __mc_input = &**varIdcs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(eqsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: varIdx, tail: rest } => {
                    let mut rangeAi: metamodelica::List<i32>;
                    let mut rangeX: metamodelica::List<i32>;
                    let mut detAexp: metamodelica::Ref<DAE::Exp>;
                    let mut detAiexp: metamodelica::Ref<DAE::Exp>;
                    let mut xExp: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut detAiExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut xLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut xEq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut xVar: metamodelica::Ref<BackendDAE::Var>;
                    let true = (intNe(varIdx.clone(), 1)) else { return Err("pattern mismatch") };
                    xVar = metamodelica::arrayGet(vectorX.clone(), varIdx.clone())?;
                    xExp = BackendVariable::varExp(&xVar)?;
                    ty = Expression::r#typeof(xExp.clone())?;
                    detAexp = makeDetExp(varIdx.clone() - 1, &(literal!("a")), 1, 1, ty.clone())?;
                    if intNe(varIdx.clone(), dim) {
                        rangeAi = List::intRange2(2, 1 + dim - varIdx.clone());
                        rangeX = List::intRange2(varIdx.clone() + 1, dim);
                    } else {
                        rangeAi = metamodelica::nil();
                        rangeX = metamodelica::nil();
                    }
                    detAiexp = makeDetExp(varIdx.clone() - 1, &(literal!("b")), 1, dim - varIdx.clone() + 1, ty.clone())?;
                    detAiExpLst = List::map(rangeAi.clone(), &({ let __pe_b0 = varIdx.clone() - 1; let __pe_b1 = literal!("a"); let __pe_b2 = 1; let __pe_b4 = ty.clone(); move |__pe_a3| makeDetExp(__pe_b0.clone(), &__pe_b1, __pe_b2.clone(), __pe_a3, __pe_b4.clone()) }))?;
                    xLst = List::map(List::map1(rangeX.clone(), &Array::getIndexFirst, vectorX.clone())?, &move |__a0: metamodelica::Ref<BackendDAE::Var>| BackendVariable::varExp(&__a0))?;
                    detAiExpLst = List::threadMap(xLst.clone(), detAiExpLst.clone(), &({ let __pe_b1 = DAE::Operator::MUL { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }))?;
                    detAiexp = List::foldr(&detAiExpLst, &({ let __pe_b1 = DAE::Operator::SUB { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }), detAiexp.clone())?;
                    (detAiexp, _) = ExpressionSimplify::simplify(detAiexp.clone())?;
                    rhs = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: detAiexp.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: detAexp.clone() });
                    xEq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: xExp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    BackendDump::dumpEquationList(&(list![xEq.clone()]), &(literal!("the new equation to solve x")))?;
                    Ok(generateCramerEqs(metamodelica::AsArg::as_arg(&rest), dim, vectorX.clone(), vectorB.clone(), matrixA.clone(), &(metamodelica::cons(xEq.clone(), eqsIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: 1, tail: rest } => {
                    let mut varIdx: i32;
                    let mut rangeX: metamodelica::List<i32>;
                    let mut detAexp: metamodelica::Ref<DAE::Exp>;
                    let mut detAiexp: metamodelica::Ref<DAE::Exp>;
                    let mut xExp: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut detAiExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut xLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut xEq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut xVar: metamodelica::Ref<BackendDAE::Var>;
                    varIdx = 1;
                    xVar = metamodelica::arrayGet(vectorX.clone(), varIdx)?;
                    xExp = BackendVariable::varExp(&xVar)?;
                    ty = Expression::r#typeof(xExp.clone())?;
                    detAexp = ((metamodelica::arrayGet(matrixA.clone(), 1)?)).get(1)?;
                    rangeX = List::intRange2(2, dim);
                    detAiexp = metamodelica::arrayGet(vectorB.clone(), 1)?;
                    detAiExpLst = List::map1(rangeX.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), metamodelica::arrayGet(matrixA.clone(), 1)?)?;
                    xLst = List::map(List::map1(rangeX.clone(), &Array::getIndexFirst, vectorX.clone())?, &move |__a0: metamodelica::Ref<BackendDAE::Var>| BackendVariable::varExp(&__a0))?;
                    detAiExpLst = List::threadMap(xLst.clone(), detAiExpLst.clone(), &({ let __pe_b1 = DAE::Operator::MUL { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }))?;
                    detAiexp = List::foldr(&detAiExpLst, &({ let __pe_b1 = DAE::Operator::SUB { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }), detAiexp.clone())?;
                    (detAiexp, _) = ExpressionSimplify::simplify(detAiexp.clone())?;
                    rhs = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: detAiexp.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: detAexp.clone() });
                    xEq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: xExp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    BackendDump::dumpEquationList(&(list![xEq.clone()]), &(literal!("the new equation to solve x")))?;
                    Ok(generateCramerEqs(metamodelica::AsArg::as_arg(&rest), dim, vectorX.clone(), vectorB.clone(), matrixA.clone(), &(metamodelica::cons(xEq.clone(), eqsIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(eqsOut)
}

fn makeDetExp(
    mut iterIdx: i32,
    mut ident: &ArcStr,
    mut row: i32,
    mut col: i32,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut detExp: metamodelica::Ref<DAE::Exp>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut name: ArcStr;
    name = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$det_"));
        __mm_s.push_str(&*ident);
        __mm_s.push_str(&*intString(iterIdx));
        __mm_s.push_str(&*literal!("__"));
        __mm_s.push_str(&*intString(row));
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(col));
        ArcStr::from(__mm_s)
    };
    cr = ComponentReferenceBasics::makeCrefIdent(name, ty.clone(), metamodelica::nil());
    detExp = Expression::makeCrefExp(cr, ty)?;
    Ok(detExp)
}

fn makeVarOfIdent(mut ident: ArcStr, mut ty: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<BackendDAE::Var> {
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = ComponentReferenceBasics::makeCrefIdent(ident, ty.clone(), metamodelica::nil());
    var = metamodelica::Ref::new(BackendDAE::Var {
        varName: cr,
        varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: ty,
        bindExp: None,
        tplExp: None,
        arryDim: metamodelica::nil(),
        source: DAE::emptyElementSource().clone(),
        values: None,
        tearingSelectOption: None,
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: false,
    });
    var
}

fn getNewChioRow(
    mut row: i32,
    mut systemIn: EqSys,
    mut iterIdx: i32,
    mut foldIn: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut foldOut: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    let mut dim: i32;
    let mut columns: metamodelica::List<i32>;
    let EqSys { dim: __pa0, .. } = &systemIn;
    dim = metamodelica::Own::own(__pa0);
    columns = List::intRange2(2, dim).reverse();
    foldOut = List::fold(
        &columns,
        &({
            let __pe_b1 = row;
            let __pe_b2 = systemIn;
            let __pe_b3 = iterIdx;
            move |__pe_a0, __pe_a4| {
                getNewChioEntry(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), &__pe_a4)
            }
        }),
        foldIn,
    )?;
    Ok(foldOut)
}

fn getNewChioEntry(
    mut col: i32,
    mut row: i32,
    mut syst: EqSys,
    mut iter: i32,
    mut foldIn: &(
        metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut foldOut: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    let mut dim: i32;
    let mut a11: metamodelica::Ref<DAE::Exp>;
    let mut ar1: metamodelica::Ref<DAE::Exp>;
    let mut a1c: metamodelica::Ref<DAE::Exp>;
    let mut arc: metamodelica::Ref<DAE::Exp>;
    let mut br: metamodelica::Ref<DAE::Exp>;
    let mut b1: metamodelica::Ref<DAE::Exp>;
    let mut detExp: metamodelica::Ref<DAE::Exp>;
    let mut detVarExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut detCR: metamodelica::Ref<DAE::ComponentRef>;
    let mut detAeq: metamodelica::Ref<BackendDAE::Equation>;
    let mut detAieq: metamodelica::Ref<BackendDAE::Equation>;
    let mut detAVar: metamodelica::Ref<BackendDAE::Var>;
    let mut detAiVar: metamodelica::Ref<BackendDAE::Var>;
    let mut detVarName: ArcStr;
    let mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut matrixB: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut vecAi: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>;
    let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let EqSys {
        dim: __pa0,
        matrixA: __pa1,
        vectorB: __pa2,
        vectorX: __pa3,
    } = syst;
    dim = metamodelica::Own::own(__pa0);
    matrixA = metamodelica::Own::own(__pa1);
    vectorB = metamodelica::Own::own(__pa2);
    vectorX = metamodelica::Own::own(__pa3);
    (matrixB, vecAi, addEqs, addVars) = foldIn.clone();
    a11 = (metamodelica::arrayGet(matrixA.clone(), 1)?).get(1)?;
    ar1 = (metamodelica::arrayGet(matrixA.clone(), row)?).get(1)?;
    a1c = (metamodelica::arrayGet(matrixA.clone(), 1)?).get(col)?;
    arc = (metamodelica::arrayGet(matrixA.clone(), row)?).get(col)?;
    ty = Expression::r#typeof(a11.clone())?;
    detExp = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: a11.clone(),
            operator: DAE::Operator::MUL { ty: ty.clone() },
            exp2: arc,
        }),
        operator: DAE::Operator::SUB { ty: ty.clone() },
        exp2: metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: ar1.clone(),
            operator: DAE::Operator::MUL { ty: ty.clone() },
            exp2: a1c,
        }),
    });
    (detExp, _) = ExpressionSimplify::simplify(detExp)?;
    detVarName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$det_a"));
        __mm_s.push_str(&*intString(iter));
        __mm_s.push_str(&*literal!("__"));
        __mm_s.push_str(&*intString(row - 1));
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(col - 1));
        ArcStr::from(__mm_s)
    };
    detCR = ComponentReferenceBasics::makeCrefIdent(detVarName, ty.clone(), metamodelica::nil());
    detAVar = metamodelica::Ref::new(BackendDAE::Var {
        varName: detCR.clone(),
        varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: ty.clone(),
        bindExp: None,
        tplExp: None,
        arryDim: metamodelica::nil(),
        source: DAE::emptyElementSource().clone(),
        values: None,
        tearingSelectOption: None,
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: false,
    });
    detVarExp = Expression::crefExp(detCR)?;
    detAeq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
        exp: detVarExp.clone(),
        scalar: detExp,
        source: DAE::emptyElementSource().clone(),
        attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
    });
    matrixB = Array::consToElement(row - 1, detVarExp, matrixB.clone())?;
    addEqs = metamodelica::cons(detAeq, addEqs);
    addVars = metamodelica::cons(detAVar, addVars);
    if col == dim {
        b1 = metamodelica::arrayGet(vectorB.clone(), 1)?;
        br = metamodelica::arrayGet(vectorB.clone(), row)?;
        detExp = metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: a11,
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: br,
            }),
            operator: DAE::Operator::SUB { ty: ty.clone() },
            exp2: metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: ar1,
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: b1,
            }),
        });
        (detExp, _) = ExpressionSimplify::simplify(detExp)?;
        detVarName = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$det_b"));
            __mm_s.push_str(&*intString(iter));
            __mm_s.push_str(&*literal!("__"));
            __mm_s.push_str(&*intString(row - 1));
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(col - 1));
            ArcStr::from(__mm_s)
        };
        detCR = ComponentReferenceBasics::makeCrefIdent(detVarName, ty.clone(), metamodelica::nil());
        detAiVar = metamodelica::Ref::new(BackendDAE::Var {
            varName: detCR.clone(),
            varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
            varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
            varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
            varType: ty,
            bindExp: None,
            tplExp: None,
            arryDim: metamodelica::nil(),
            source: DAE::emptyElementSource().clone(),
            values: None,
            tearingSelectOption: None,
            hideResult: None,
            comment: None,
            connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
            innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
            unreplaceable: false,
            initNonlinear: false,
            encrypted: false,
        });
        detVarExp = Expression::crefExp(detCR)?;
        detAieq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: detVarExp.clone(),
            scalar: detExp,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
        });
        metamodelica::arrayUpdate(vecAi.clone(), row - 1, detVarExp)?;
        addEqs = metamodelica::cons(detAieq, addEqs);
        addVars = metamodelica::cons(detAiVar, addVars);
    }
    foldOut = (matrixB.clone(), vecAi.clone(), addEqs, addVars);
    Ok(foldOut)
}

//--------------------------------------------------//
// Cramers Rule
//-------------------------------------------------//
fn applyCramerRule(
    mut jacValuesIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut varsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut resEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut tvarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut addEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (resEqsOut, tvarsOut, addEqsOut, addVarsOut) = (::match_deref::match_deref! { match &(varsIn.clone()) {
        _ => {
            let mut syst: EqSys;
            let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut resEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut addVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            syst = getMatrixFromJac(jacValuesIn.clone(), varsIn.clone())?;
            (resEqs, addEqs, addVars) = CramerRule(syst);
            (resEqs, varsIn, addEqs, addVars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((resEqsOut, tvarsOut, addEqsOut, addVarsOut))
}

fn CramerRule(
    mut system: EqSys,
) -> (
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) {
    let mut newResEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut otherEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut otherVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (newResEqs, otherEqsOut, otherVarsOut) = 'mc: {
        let __mc_input = system.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys {
                dim: mut dim,
                matrixA: mut matrixA,
                vectorX: mut vectorX,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut matrixAT: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut detA: metamodelica::Ref<DAE::Exp>;
            let mut detLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut varExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let true = (intEq(dim.clone(), 2)) else {
                return Err("pattern mismatch");
            };
            matrixAT = transposeMatrix(matrixA.clone())?;
            detA = determinant(matrixA.clone())?;
            detLst = List::map2(
                List::intRange(dim.clone()),
                &move |__a0: i32,
                       __a1: EqSys,
                       __a2: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>| {
                    CramerRule1(__a0, &__a1, __a2)
                },
                system.clone(),
                matrixAT.clone(),
            )?;
            varExp = List::mapArray(vectorX.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| {
                BackendVariable::varExp(&__a0)
            })?;
            detLst = List::map1(
                detLst.clone(),
                &({
                    let __pe_b1 = DAE::Operator::DIV {
                        ty: DAE::T_ANYTYPE_DEFAULT().clone(),
                    };
                    move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2))
                }),
                detA.clone(),
            )?;
            (detLst, _) = List::map_2(&detLst, &ExpressionSimplify::simplify)?;
            eqLst = List::threadMap2(
                varExp.clone(),
                detLst.clone(),
                &BackendEquation::generateEquation,
                DAE::emptyElementSource().clone(),
                BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
            )?;
            Ok((eqLst.clone(), metamodelica::nil(), metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys {
                dim: mut dim,
                matrixA: mut matrixA,
                vectorX: mut vectorX,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut matrixAT: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut detA: metamodelica::Ref<DAE::Exp>;
            let mut detLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut varExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let true = (intEq(dim.clone(), 3)) else {
                return Err("pattern mismatch");
            };
            matrixAT = transposeMatrix(matrixA.clone())?;
            detA = determinant(matrixA.clone())?;
            detLst = List::map2(
                List::intRange(dim.clone()),
                &move |__a0: i32,
                       __a1: EqSys,
                       __a2: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>| {
                    CramerRule1(__a0, &__a1, __a2)
                },
                system.clone(),
                matrixAT.clone(),
            )?;
            varExp = List::mapArray(vectorX.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| {
                BackendVariable::varExp(&__a0)
            })?;
            detLst = List::map1(
                detLst.clone(),
                &({
                    let __pe_b1 = DAE::Operator::DIV {
                        ty: DAE::T_ANYTYPE_DEFAULT().clone(),
                    };
                    move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2))
                }),
                detA.clone(),
            )?;
            (detLst, _) = List::map_2(&detLst, &ExpressionSimplify::simplify)?;
            eqLst = List::threadMap2(
                varExp.clone(),
                detLst.clone(),
                &BackendEquation::generateEquation,
                DAE::emptyElementSource().clone(),
                BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
            )?;
            Ok((eqLst.clone(), metamodelica::nil(), metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys { dim: mut dim, .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut addEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut addVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let true = (intGt(dim.clone(), 3)) else {
                return Err("pattern mismatch");
            };
            (eqLst, addEqLst, addVarLst) = chiosCondensation(system.clone())?;
            Ok((eqLst.clone(), addEqLst.clone(), addVarLst.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (newResEqs, otherEqsOut, otherVarsOut)
}

fn CramerRule1(
    mut idx: i32,
    mut syst: &EqSys,
    mut matrixAT: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut det: metamodelica::Ref<DAE::Exp>;
    det = (match syst.clone() {
        EqSys {
            vectorB: mut vectorB, ..
        } => {
            let mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            matrixA = metamodelica::arrayFromVec(matrixAT.clone().borrow().clone());
            matrixA = replaceColumnInMatrix(
                matrixA.clone(),
                idx,
                vectorB
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>(),
            )?;
            determinant(matrixA.clone())?
        }
    });
    Ok(det)
}

fn determinant(
    mut matrix: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut detOut: metamodelica::Ref<DAE::Exp>;
    detOut = 'mc: {
        let __mc_input = matrix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut a11: metamodelica::Ref<DAE::Exp>;
            let mut a12: metamodelica::Ref<DAE::Exp>;
            let mut a21: metamodelica::Ref<DAE::Exp>;
            let mut a22: metamodelica::Ref<DAE::Exp>;
            let mut det: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let true = (metamodelica::arrayLength(matrix.clone()) == 2) else {
                return Err("pattern mismatch");
            };
            a11 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(1)?;
            a12 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(2)?;
            a21 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(1)?;
            a22 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(2)?;
            ty = Expression::r#typeof(a11.clone())?;
            det = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a11.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a22.clone(),
                }),
                operator: DAE::Operator::SUB { ty: ty.clone() },
                exp2: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a12.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a21.clone(),
                }),
            });
            (det, _) = ExpressionSimplify::simplify(det.clone())?;
            Ok(det.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut a11: metamodelica::Ref<DAE::Exp>;
            let mut a12: metamodelica::Ref<DAE::Exp>;
            let mut a21: metamodelica::Ref<DAE::Exp>;
            let mut a22: metamodelica::Ref<DAE::Exp>;
            let mut a13: metamodelica::Ref<DAE::Exp>;
            let mut a23: metamodelica::Ref<DAE::Exp>;
            let mut a33: metamodelica::Ref<DAE::Exp>;
            let mut a31: metamodelica::Ref<DAE::Exp>;
            let mut a32: metamodelica::Ref<DAE::Exp>;
            let mut s1: metamodelica::Ref<DAE::Exp>;
            let mut s2: metamodelica::Ref<DAE::Exp>;
            let mut s3: metamodelica::Ref<DAE::Exp>;
            let mut s4: metamodelica::Ref<DAE::Exp>;
            let mut s5: metamodelica::Ref<DAE::Exp>;
            let mut s6: metamodelica::Ref<DAE::Exp>;
            let mut det: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let true = (metamodelica::arrayLength(matrix.clone()) == 3) else {
                return Err("pattern mismatch");
            };
            a11 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(1)?;
            a12 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(2)?;
            a13 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(3)?;
            a21 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(1)?;
            a22 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(2)?;
            a23 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(3)?;
            a31 = (metamodelica::arrayGet(matrix.clone(), 3)?).get(1)?;
            a32 = (metamodelica::arrayGet(matrix.clone(), 3)?).get(2)?;
            a33 = (metamodelica::arrayGet(matrix.clone(), 3)?).get(3)?;
            ty = Expression::r#typeof(a11.clone())?;
            s1 = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a11.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a22.clone(),
                }),
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: a33.clone(),
            });
            s2 = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a12.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a23.clone(),
                }),
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: a31.clone(),
            });
            s3 = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a13.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a21.clone(),
                }),
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: a32.clone(),
            });
            s4 = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a13.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a22.clone(),
                }),
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: a31.clone(),
            });
            s5 = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a23.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a32.clone(),
                }),
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: a11.clone(),
            });
            s6 = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: a33.clone(),
                    operator: DAE::Operator::MUL { ty: ty.clone() },
                    exp2: a12.clone(),
                }),
                operator: DAE::Operator::MUL { ty: ty.clone() },
                exp2: a21.clone(),
            });
            det = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                        exp1: s1.clone(),
                        operator: DAE::Operator::ADD { ty: ty.clone() },
                        exp2: s2.clone(),
                    }),
                    operator: DAE::Operator::ADD { ty: ty.clone() },
                    exp2: s3.clone(),
                }),
                operator: DAE::Operator::SUB { ty: ty.clone() },
                exp2: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
                        exp1: s4.clone(),
                        operator: DAE::Operator::ADD { ty: ty.clone() },
                        exp2: s5.clone(),
                    }),
                    operator: DAE::Operator::ADD { ty: ty.clone() },
                    exp2: s6.clone(),
                }),
            });
            (det, _) = ExpressionSimplify::simplify(det.clone())?;
            Ok(det.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("computation fo determinant failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(detOut)
}

fn replaceColumnInMatrix(
    mut matrixT: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut col: i32,
    mut vectorB: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut matrixOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut matrix: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    matrix = metamodelica::arrayUpdate(matrixT.clone(), col, vectorB)?;
    matrixOut = transposeMatrix(matrix.clone())?;
    Ok(matrixOut)
}

fn getMatrixFromJac(
    mut jacValuesIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<EqSys> {
    let mut matrixOut: EqSys;
    let mut AVars: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut bVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(jacValuesIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    bVars = metamodelica::Own::own(__pa0);
    AVars = metamodelica::Own::own(__pa1);
    matrixA = metamodelica::arrayFromVec(
        List::mapList(AVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>| {
            BackendVariable::varExp(&__a0)
        })?
        .into_iter()
        .cloned()
        .collect(),
    );
    matrixA = transposeMatrix(matrixA.clone())?;
    vectorB = metamodelica::arrayFromVec(
        List::mapMap(
            bVars.clone(),
            &move |__a0: metamodelica::Ref<BackendDAE::Var>| BackendVariable::varExp(&__a0),
            &Expression::negate,
        )?
        .into_iter()
        .cloned()
        .collect(),
    );
    vectorX = metamodelica::arrayFromVec(vars.into_iter().cloned().collect());
    matrixOut = EqSys {
        dim: ((bVars).len() as i32),
        matrixA: matrixA.clone(),
        vectorB: vectorB.clone(),
        vectorX: vectorX.clone(),
    };
    Ok(matrixOut)
}

fn transposeMatrix(
    mut matrixIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut matrixOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut size: i32;
    size = metamodelica::arrayLength(matrixIn.clone());
    matrixOut = arrayCreate(size, metamodelica::nil());
    matrixOut = List::fold1(
        &(List::intRange(size).reverse()),
        &transposeMatrix1,
        matrixIn.clone(),
        matrixOut.clone(),
    )?;
    Ok(matrixOut)
}

fn transposeMatrix1(
    mut idx: i32,
    mut matrixOrig: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut matrixIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut matrixOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut row: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    row = metamodelica::arrayGet(matrixOrig.clone(), idx)?;
    matrixOut = List::threadFold(
        &(List::intRange(metamodelica::arrayLength(matrixOrig.clone()))),
        row,
        &Array::consToElement,
        matrixIn.clone(),
    )?;
    Ok(matrixOut)
}

//--------------------------------------------------//
// Printing stuff
//-------------------------------------------------//
fn dumpEqSys(mut matrix: EqSys) -> Result<()> {
    let mut dim: i32;
    let mut sLst: metamodelica::List<ArcStr>;
    let mut matrixA: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut vectorB: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>;
    let EqSys {
        dim: __pa0,
        matrixA: __pa1,
        vectorB: __pa2,
        vectorX: __pa3,
    } = matrix;
    dim = metamodelica::Own::own(__pa0);
    matrixA = metamodelica::Own::own(__pa1);
    vectorB = metamodelica::Own::own(__pa2);
    vectorX = metamodelica::Own::own(__pa3);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Matrix("));
        __mm_s.push_str(&*intString(dim));
        __mm_s.push_str(&*literal!(")\n"));
        ArcStr::from(__mm_s)
    });
    sLst = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        let __thr_src0 = matrixA.clone();
        let __thr_borrow0 = __thr_src0.borrow();
        let mut __thr_it0 = __thr_borrow0.iter().cloned();
        let __thr_src1 = vectorX.clone();
        let __thr_borrow1 = __thr_src1.borrow();
        let mut __thr_it1 = __thr_borrow1.iter().cloned();
        let __thr_src2 = vectorB.clone();
        let __thr_borrow2 = __thr_src2.borrow();
        let mut __thr_it2 = __thr_borrow2.iter().cloned();
        loop {
            match (__thr_it0.next(), __thr_it1.next(), __thr_it2.next()) {
                (Some(Arow), Some(x), Some(b)) => {
                    let __x = EqSysRowString(Arow.clone(), &(x.clone()), b.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*stringDelimitList(sLst, literal!("\n")));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn EqSysRowString(
    mut Arow: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut x: &metamodelica::Ref<BackendDAE::Var>,
    mut b: metamodelica::Ref<DAE::Exp>,
) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    s1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{ "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(Arow, &ExpressionBasics::printExpStr)?,
            literal!("  \t  "),
        ));
        __mm_s.push_str(&*literal!("} "));
        ArcStr::from(__mm_s)
    };
    s2 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{ "));
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
            &(BackendVariable::varCref(x)),
        )?);
        __mm_s.push_str(&*literal!(" } "));
        ArcStr::from(__mm_s)
    };
    s3 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(" = { "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?);
        __mm_s.push_str(&*literal!(" }"));
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s1);
        __mm_s.push_str(&*literal!(" * "));
        __mm_s.push_str(&*s2);
        __mm_s.push_str(&*s3);
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

fn dumpMatrix(mut matrix: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>) -> Result<()> {
    let mut sLst: metamodelica::List<ArcStr>;
    let mut s: ArcStr;
    sLst = List::mapArray(matrix.clone(), &ExpressionDump::printExpListStr)?;
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{ "));
        __mm_s.push_str(&*stringDelimitList(sLst, literal!("  \n  ")));
        __mm_s.push_str(&*literal!("} \n"));
        ArcStr::from(__mm_s)
    };
    metamodelica::print(s);
    Ok(())
}

fn dumpVarArrLst(
    mut inArrLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut heading: ArcStr,
) -> Result<()> {
    let mut r#str: ArcStr;
    let mut inLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    inLstLst = inArrLst
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("---------\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("-variables\n---------\n"));
        ArcStr::from(__mm_s)
    });
    r#str = List::fold1(
        &(List::intRange(metamodelica::arrayLength(inArrLst.clone()))),
        &move |__a0: i32,
               __a1: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
               __a2: ArcStr| dumpVarArrLst1(__a0, &__a1, __a2),
        inLstLst,
        heading,
    )?;
    Ok(())
}

fn dumpVarArrLst1(
    mut lstIdx: i32,
    mut inLstLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut heading: ArcStr,
) -> Result<ArcStr> {
    let mut headingOut: ArcStr;
    let mut str1: ArcStr;
    let mut inLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    inLst = (inLstLst).get(lstIdx)?;
    str1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(lstIdx - 1));
        ArcStr::from(__mm_s)
    };
    BackendDump::dumpVarList(&inLst, &str1)?;
    headingOut = heading;
    Ok(headingOut)
}

fn dumpEqArrLst(
    mut inArrLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut heading: ArcStr,
) -> Result<()> {
    let mut r#str: ArcStr;
    let mut inLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    inLstLst = inArrLst
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("---------\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("-equations\n---------\n"));
        ArcStr::from(__mm_s)
    });
    r#str = List::fold1(
        &(List::intRange(metamodelica::arrayLength(inArrLst.clone()))),
        &move |__a0: i32,
               __a1: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
               __a2: ArcStr| dumpEqArrLst1(__a0, &__a1, __a2),
        inLstLst,
        heading,
    )?;
    Ok(())
}

fn dumpEqArrLst1(
    mut lstIdx: i32,
    mut inLstLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut heading: ArcStr,
) -> Result<ArcStr> {
    let mut headingOut: ArcStr;
    let mut str1: ArcStr;
    let mut inLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    inLst = (inLstLst).get(lstIdx)?;
    str1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(lstIdx - 1));
        ArcStr::from(__mm_s)
    };
    BackendDump::dumpEquationList(&inLst, &str1)?;
    headingOut = heading;
    Ok(headingOut)
}

//--------------------------------------------------//
// solve torn systems in parallel
//-------------------------------------------------//
pub(crate) fn parallelizeTornSystems(
    mut graphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: &HpcOmTaskGraph::TaskGraphMeta,
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut simVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> (
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    metamodelica::List<i32>,
) {
    let mut scheduledTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut daeNodeIdcs: metamodelica::List<i32>;
    (scheduledTasks, daeNodeIdcs) = 'mc: {
        let __mc_input = &**inDAE;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
                    let mut taskLst: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut daeNodes: metamodelica::List<i32>;
                    let true = (false) else { return Err("pattern mismatch") };
                    let __arc1 = &(*inDAE);
                    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
                    eqSysts = metamodelica::Own::own(__pa0);
                    (_, taskLst) = pts_traverseEqSystems(&eqSysts, sccSimEqMapping.clone(), simVarMapping.clone(), 1, &(metamodelica::nil()), BackendDAEUtil::isInitializationDAE(&inDAE.shared))?;
                    daeNodes = List::map(taskLst.clone(), &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>| getScheduledTaskCompIdx(&__a0))?;
                    Ok((taskLst.clone(), daeNodes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (scheduledTasks, daeNodeIdcs)
}

fn getScheduledTaskCompIdx(mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>) -> Result<i32> {
    let mut compIdx: i32;
    compIdx = (match &**taskIn {
        HpcOmSimCode::Task::SCHEDULED_TASK {
            compIdx: __esc_compIdx, ..
        } => {
            compIdx = (*__esc_compIdx).clone();
            compIdx.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(compIdx)
}

fn pts_traverseEqSystems(
    mut eqSysIn: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut simVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut compIdxIn: i32,
    mut taskLstIn: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut isInitial: bool,
) -> Result<(i32, metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>)> {
    let mut compIdxOut: i32;
    let mut taskLstOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    (compIdxOut, taskLstOut) = 'mc: {
        let __mc_input = &**eqSysIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, .. }, tail: eqSysRest } => {
                    let mut compIdx: i32;
                    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut taskLst: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    eqLst = BackendEquation::equationList(eqs.clone())?;
                    varLst = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars))?;
                    (compIdx, taskLst) = pts_traverseCompsAndParallelize(metamodelica::AsArg::as_arg(&comps), &eqLst, &varLst, sccSimEqMapping.clone(), simVarMapping.clone(), compIdxIn, taskLstIn, isInitial);
                    (compIdx, taskLst) = pts_traverseEqSystems(metamodelica::AsArg::as_arg(&eqSysRest), sccSimEqMapping.clone(), simVarMapping.clone(), compIdx, &taskLst, isInitial)?;
                    Ok((compIdx, taskLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((compIdxIn, taskLstIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("pts_traverseEqSystems failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((compIdxOut, taskLstOut))
}

fn pts_traverseCompsAndParallelize(
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut eqsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut varsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut simVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut compIdxIn: i32,
    mut taskLstIn: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut isInitial: bool,
) -> (i32, metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>) {
    let mut compIdxOut: i32;
    let mut taskLstOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    (compIdxOut, taskLstOut) = 'mc: {
        let __mc_input = &**inComps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((compIdxIn, taskLstIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqs, innerEquations, .. }, .. }, tail: rest } => {
                    let mut numEqs: i32;
                    let mut numVars: i32;
                    let mut compIdx: i32;
                    let mut numResEqs: i32;
                    let mut eqIdcs: metamodelica::List<i32>;
                    let mut varIdcs: metamodelica::List<i32>;
                    let mut eqIdcsSys: metamodelica::List<i32>;
                    let mut simEqSysIdcs: metamodelica::List<i32>;
                    let mut resSimEqSysIdcs: metamodelica::List<i32>;
                    let mut otherSimEqSysIdcs: metamodelica::List<i32>;
                    let mut varIdcLstSys: metamodelica::List<metamodelica::List<i32>>;
                    let mut varIdcsLsts: metamodelica::List<metamodelica::List<i32>>;
                    let mut otherSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
                    let mut otherEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
                    let mut otherVars: BackendDAE::Variables;
                    let mut graph: metamodelica::Array<metamodelica::List<i32>>;
                    let mut graphMerged: metamodelica::Array<metamodelica::List<i32>>;
                    let mut meta: HpcOmTaskGraph::TaskGraphMeta;
                    let mut metaMerged: HpcOmTaskGraph::TaskGraphMeta;
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut taskLst: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut otherEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut otherVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (eqIdcs, varIdcsLsts, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    varIdcs = List::flatten(varIdcsLsts.clone())?;
                    numEqs = ((eqIdcs).len() as i32);
                    numVars = ((varIdcs).len() as i32);
                    numResEqs = ((resEqs).len() as i32);
                    eqIdcsSys = List::intRange(numEqs);
                    (varIdcLstSys, _) = List::mapFold(&varIdcsLsts, &move |__a0: metamodelica::List<i32>, __a1: i32| -> metamodelica::Result<_> { ::std::result::Result::Ok(genSystemVarIdcs(&__a0, __a1)) }, 1)?;
                    otherEqLst = List::map1(eqIdcs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), eqsIn.clone())?;
                    otherVarLst = List::map1(varIdcs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), varsIn.clone())?;
                    otherVars = BackendVariable::listVar1(&otherVarLst)?;
                    otherEqs = BackendEquation::listEquation(&otherEqLst)?;
                    (m, mT) = BackendDAEUtil::adjacencyMatrixDispatch(&otherVars, otherEqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, isInitial)?;
                    (graph, meta) = HpcOmTaskGraph::getEmptyTaskGraph(numEqs, numEqs, numVars);
                    graph = buildMatchedGraphForTornSystem(1, &eqIdcsSys, &varIdcLstSys, m.clone(), mT.clone(), graph.clone())?;
                    meta = buildTaskgraphMetaForTornSystem(graph.clone(), otherEqLst.clone(), otherVarLst.clone(), meta.clone())?;
                    simEqSysIdcs = metamodelica::arrayGet(sccSimEqMapping.clone(), compIdxIn)?;
                    resSimEqSysIdcs = List::map1r(List::intRange(numResEqs), &fnptr!(intSub, i32, i32), (simEqSysIdcs).head().cloned()?)?;
                    otherSimEqSysIdcs = List::map1r(List::intRange2(numResEqs + 1, numResEqs + numEqs), &fnptr!(intSub, i32, i32), (simEqSysIdcs).head().cloned()?)?;
                    otherSimEqMapping = metamodelica::arrayFromVec(List::map(otherSimEqSysIdcs.clone(), &fnptr!(List::create, _))?.into_iter().cloned().collect());
                    BackendDump::dumpBipartiteGraphStrongComponent1(metamodelica::AsArg::as_arg(&comp), eqsIn.clone(), varsIn.clone(), None, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tornSys_bipartite_")); __mm_s.push_str(&*intString(compIdxIn)); ArcStr::from(__mm_s) }))?;
                    BackendDump::dumpDAGStrongComponent(graph.clone(), meta.clone(), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tornSys_matched_")); __mm_s.push_str(&*intString(compIdxIn)); ArcStr::from(__mm_s) }))?;
                    (graphMerged, metaMerged) = (graph.clone(), meta.clone());
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("function pts_traverseCompsAndParallelize failed. GRS is temporarily disabled.")])?;
                    BackendDump::dumpDAGStrongComponent(graphMerged.clone(), metaMerged.clone(), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tornSys_matched2_")); __mm_s.push_str(&*intString(compIdxIn)); ArcStr::from(__mm_s) }))?;
                    schedule = HpcOmScheduler::createListSchedule(graphMerged.clone(), metaMerged.clone(), 2, otherSimEqMapping.clone(), simVarMapping.clone())?;
                    HpcOmScheduler::printSchedule(&schedule)?;
                    task = pts_transformScheduleToTask(&schedule, &resSimEqSysIdcs, compIdxIn)?;
                    (compIdx, taskLst) = pts_traverseCompsAndParallelize(metamodelica::AsArg::as_arg(&rest), eqsIn, varsIn, sccSimEqMapping.clone(), simVarMapping.clone(), compIdxIn + 1, &(metamodelica::cons(task.clone(), taskLstIn.clone())), isInitial);
                    Ok((compIdx, taskLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut compIdx: i32;
                    let mut taskLst: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    (compIdx, taskLst) = pts_traverseCompsAndParallelize(metamodelica::AsArg::as_arg(&rest), eqsIn, varsIn, sccSimEqMapping.clone(), simVarMapping.clone(), compIdxIn + 1, taskLstIn, isInitial);
                    Ok((compIdx, taskLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (compIdxOut, taskLstOut)
}

fn pts_transformScheduleToTask(
    mut otherEqSys: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut resSimEqs: &metamodelica::List<i32>,
    mut compIdx: i32,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
    task = 'mc: {
        let __mc_input = &**otherEqSys;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { .. } => {
                    metamodelica::print(literal!("levelScheduling is not supported for heterogenious scheduling\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. } => {
                    let mut numThreads: i32;
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    numThreads = metamodelica::arrayLength(threadTasks.clone());
                    schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
                    Ok(metamodelica::Ref::new(HpcOmSimCode::Task::SCHEDULED_TASK { compIdx: compIdx, numThreads: numThreads, taskSchedule: schedule.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("pts_transformScheduleToTask failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(task)
}

fn genSystemVarIdcs(mut idcsIn: &metamodelica::List<i32>, mut idx: i32) -> (metamodelica::List<i32>, i32) {
    let mut idcsOut: metamodelica::List<i32>;
    let mut idx2: i32;
    idx2 = ((idcsIn).len() as i32) + idx;
    idcsOut = List::intRange2(idx, idx2 - 1);
    (idcsOut, idx2)
}

//05-09-2014 marcusw: Changed because of dependency-task restructuring for MPI
//protected function appendStringToLockIdcs "author: Waurich TUD 2014-07
//  appends the suffix to the lockIds of the given tasks
//"
//  input list<HpcOmSimCode.Task> taskLstIn;
//  input String suffix;
//  output list<HpcOmSimCode.Task> taskLstOut;
//algorithm
//  taskLstOut := List.map1(taskLstIn,appendStringToLockIdcs1,suffix);
//end appendStringToLockIdcs;
//
//protected function appendStringToLockIdcs1 "author: Waurich TUD 2014-07
//  appends the suffix to the lockIds of the given tasks
//"
//  input HpcOmSimCode.Task taskIn;
//  input String suffix;
//  output HpcOmSimCode.Task taskOut;
//algorithm
//  taskOut := match(taskIn,suffix)
//    local
//      String lockId;
//    case(HpcOmSimCode.ASSIGNLOCKTASK(lockId=lockId),_)
//      equation
//        lockId = stringAppend(lockId,suffix);
//    then HpcOmSimCode.ASSIGNLOCKTASK(lockId);
//     case(HpcOmSimCode.RELEASELOCKTASK(lockId=lockId),_)
//      equation
//        lockId = stringAppend(lockId,suffix);
//    then HpcOmSimCode.RELEASELOCKTASK(lockId);
//    else
//      then taskIn;
//  end match;
//end appendStringToLockIdcs1;
fn buildMatchedGraphForTornSystem(
    mut idx: i32,
    mut eqsIn: &metamodelica::List<i32>,
    mut varsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut graphIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut graphOut: metamodelica::Array<metamodelica::List<i32>>;
    graphOut = 'mc: {
        let __mc_input = graphIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut eq: i32;
            let mut vars: metamodelica::List<i32>;
            let mut depEqs: metamodelica::List<i32>;
            let mut graph: metamodelica::Array<metamodelica::List<i32>>;
            let true = (((eqsIn).len() as i32) >= idx) else {
                return Err("pattern mismatch");
            };
            vars = (varsIn).get(idx)?;
            eq = (eqsIn).get(idx)?;
            depEqs = List::flatten(List::map1(vars.clone(), &Array::getIndexFirst, mt.clone())?)?;
            (depEqs, _) = List::deleteMemberOnTrue(eq, depEqs.clone(), &fnptr!(intEq, i32, i32))?;
            graph = metamodelica::arrayUpdate(graphIn.clone(), eq, depEqs.clone())?;
            graph = buildMatchedGraphForTornSystem(idx + 1, eqsIn, varsIn, m.clone(), mt.clone(), graph.clone())?;
            Ok(graph.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (((eqsIn).len() as i32) > idx) else {
                return Err("pattern mismatch");
            };
            Ok(graphIn.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(graphOut)
}

fn buildTaskgraphMetaForTornSystem(
    mut graph: metamodelica::Array<metamodelica::List<i32>>,
    mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut metaIn: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<HpcOmTaskGraph::TaskGraphMeta> {
    let mut metaOut: HpcOmTaskGraph::TaskGraphMeta;
    let mut numNodes: i32;
    let mut eqStrings: metamodelica::List<ArcStr>;
    let mut varStrings: metamodelica::List<ArcStr>;
    let mut descLst: metamodelica::List<ArcStr>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        varCompMapping: __pa0,
        eqCompMapping: __pa1,
        compParamMapping: __pa2,
        nodeMark: __pa3,
        compInformations: __pa4,
        ..
    } = metaIn;
    varCompMapping = metamodelica::Own::own(__pa0);
    eqCompMapping = metamodelica::Own::own(__pa1);
    compParamMapping = metamodelica::Own::own(__pa2);
    nodeMark = metamodelica::Own::own(__pa3);
    compInformations = metamodelica::Own::own(__pa4);
    numNodes = metamodelica::arrayLength(graph.clone());
    inComps = metamodelica::arrayFromVec(
        List::map(List::intRange(numNodes), &fnptr!(List::create, _))?
            .into_iter()
            .cloned()
            .collect(),
    );
    compNames = metamodelica::arrayFromVec(
        List::map(List::intRange(numNodes), &fnptr!(intString, i32))?
            .into_iter()
            .cloned()
            .collect(),
    );
    exeCosts = arrayCreate(numNodes, (3, metamodelica::OrderedFloat(20.0_f64)));
    commCosts = Array::map(graph.clone(), &buildDummyCommCosts)?;
    eqStrings = List::map(eqLst, &move |__a0: metamodelica::Ref<BackendDAE::Equation>| {
        BackendDump::equationString(&__a0)
    })?;
    varStrings = List::map(varLst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| {
        HpcOmTaskGraph::getVarString(&__a0)
    })?;
    descLst = List::map1(eqStrings, &fnptr!(stringAppend, ArcStr, ArcStr), literal!(" FOR "))?;
    descLst = List::threadMap(descLst, varStrings, &fnptr!(stringAppend, ArcStr, ArcStr))?;
    compDescs = metamodelica::arrayFromVec(descLst.into_iter().cloned().collect());
    metaOut = HpcOmTaskGraph::TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    Ok(metaOut)
}

fn buildDummyCommCosts(
    mut childNodes: metamodelica::List<i32>,
) -> Result<metamodelica::List<HpcOmTaskGraph::Communication>> {
    let mut commCosts: metamodelica::List<HpcOmTaskGraph::Communication>;
    commCosts = List::map(childNodes, &fnptr!(buildDummyCommCost, i32))?;
    Ok(commCosts)
}

fn buildDummyCommCost(mut iChildNodeIdx: i32) -> HpcOmTaskGraph::Communication {
    let mut oCommCost: HpcOmTaskGraph::Communication;
    oCommCost = HpcOmTaskGraph::Communication {
        numberOfVars: 1,
        integerVars: metamodelica::nil(),
        floatVars: list![-1],
        booleanVars: metamodelica::nil(),
        stringVars: metamodelica::nil(),
        childNode: iChildNodeIdx,
        requiredTime: metamodelica::OrderedFloat(70.0_f64),
    };
    oCommCost
}

pub(crate) fn createSingleBlockSchedule(
    mut graphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut scheduledTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut nodes: metamodelica::List<i32>;
    let mut comps: metamodelica::List<metamodelica::List<i32>>;
    let mut simEqSys: metamodelica::List<metamodelica::List<i32>>;
    let mut thread1: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = metaIn;
    inComps = metamodelica::Own::own(__pa0);
    nodes = List::intRange(metamodelica::arrayLength(graphIn.clone()));
    comps = List::map1(nodes.clone(), &Array::getIndexFirst, inComps.clone())?;
    simEqSys = HpcOmScheduler::getSimEqSysIdcsForNodeLst(comps, sccSimEqMapping.clone())?;
    simEqSys = List::map1(
        simEqSys,
        &List::sort,
        (std::sync::Arc::new(fnptr!(intGt, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    thread1 = List::threadMap1(
        simEqSys,
        nodes,
        &fnptr!(HpcOmScheduler::makeCalcTask, metamodelica::List<i32>, i32, i32),
        1,
    )?;
    threadTasks = arrayCreate(4, metamodelica::nil());
    threadTasks = metamodelica::arrayUpdate(threadTasks.clone(), 1, thread1.clone())?;
    allCalcTasks = arrayCreate(
        ((thread1).len() as i32),
        (openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY(), 0),
    );
    schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: threadTasks.clone(),
        outgoingDepTasks: metamodelica::nil(),
        scheduledTasks: scheduledTasks,
        allCalcTasks: allCalcTasks.clone(),
    });
    Ok(schedule)
}
