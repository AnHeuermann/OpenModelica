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

use crate::AdjacencyMatrix;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::DumpGraphML;
use crate::ExpressionSolve;
use crate::Matching;
use crate::Sorting;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_codegen_util::SimCodeCodegenUtil;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// =============================================================================
// section for type definitions
//
//
// =============================================================================
pub(crate) const BORDER: &'static str = "****************************************";

pub(crate) const UNDERLINE: &'static str = "========================================";

pub(crate) static withLSS: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| list![literal!("C"), literal!("wasm-jit"), literal!("wasm")]);

pub(crate) static withNSS: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| list![literal!("C"), literal!("wasm-jit"), literal!("wasm")]);

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum TearingMethod {
    /// Only tear discrete variables from loops
    MINIMAL_TEARING,
    OMC_TEARING,
    CELLIER_TEARING,
    TOTAL_TEARING,
    USER_DEFINED_TEARING,
}
impl metamodelica::gc::MMTrace for TearingMethod {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TearingMethod::MINIMAL_TEARING => Ok(()),
            TearingMethod::OMC_TEARING => Ok(()),
            TearingMethod::CELLIER_TEARING => Ok(()),
            TearingMethod::TOTAL_TEARING => Ok(()),
            TearingMethod::USER_DEFINED_TEARING => Ok(()),
        }
    }
}
impl Default for TearingMethod {
    fn default() -> Self {
        Self::MINIMAL_TEARING
    }
}
pub(crate) use self::TearingMethod::{
    CELLIER_TEARING, MINIMAL_TEARING, OMC_TEARING, TOTAL_TEARING, USER_DEFINED_TEARING,
};

// =============================================================================
// section for all public functions
//
// main function to divide to the selected tearing method
// =============================================================================
pub(crate) fn tearingSystem(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut methodString: ArcStr = Config::getTearingMethod()?;
    let mut method: TearingMethod;
    let mut DAEtype: BackendDAE::BackendDAEType;
    let mut strongComponentIndex: i32 = System::tmpTickIndex(Global::strongComponent_index.clone());
    if Flags::getConfigInt(Flags::MAX_SIZE_LINEAR_TEARING.clone())? < 0 {
        Error::addMessage(
            Error::INVALID_FLAG_TYPE.clone(),
            list![
                literal!("maxSizeLinearTearing"),
                literal!("non-negative integer"),
                intString(Flags::getConfigInt(Flags::MAX_SIZE_LINEAR_TEARING.clone())?)
            ],
        )?;
        return Err("fail");
    } else if Flags::getConfigInt(Flags::MAX_SIZE_NONLINEAR_TEARING.clone())? < 0 {
        Error::addMessage(
            Error::INVALID_FLAG_TYPE.clone(),
            list![
                literal!("maxSizeNonlinearTearing"),
                literal!("non-negative integer"),
                intString(Flags::getConfigInt(Flags::MAX_SIZE_NONLINEAR_TEARING.clone())?)
            ],
        )?;
        return Err("fail");
    }
    match '__try0: {
        method = unwrap_break_err!(getTearingMethod(&methodString), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::TEARING_DUMP.clone()), '__try0)
            || unwrap_break_err!(Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone()), '__try0)
        {
            let __arc2 = inDAE.shared.clone();
            let BackendDAE::SHARED {
                backendDAEType: __pa1, ..
            } = &*__arc2;
            DAEtype = metamodelica::Own::own(__pa1);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\n\n\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\nCalling Tearing for "));
                __mm_s.push_str(&*unwrap_break_err!(BackendDump::printBackendDAEType2String(DAEtype), '__try0));
                __mm_s.push_str(&*literal!("!\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        (outDAE, strongComponentIndex) = unwrap_break_err!(BackendDAEUtil::mapEqSystemAndFold(inDAE, &({ let __pe_b0 = method; move |__pe_a1, __pe_a2, __pe_a3| tearingSystemWork(__pe_b0.clone(), __pe_a1, __pe_a2, __pe_a3) }), strongComponentIndex), '__try0);
        System::tmpTickSetIndex(strongComponentIndex, Global::strongComponent_index.clone());
        Ok::<_, &'static str>((method.clone(), outDAE.clone(), strongComponentIndex.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            method = __try0_o0;
            outDAE = __try0_o1;
            strongComponentIndex = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Tearing.tearingSystem"));
                    __mm_s.push_str(&*literal!(" failed"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/Tearing.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok(outDAE)
}

// =============================================================================
// protected
//
//
// =============================================================================
fn getTearingMethod(mut inTearingMethod: &ArcStr) -> Result<TearingMethod> {
    let mut outTearingMethod: TearingMethod;
    outTearingMethod = (::match_deref::match_deref! { match &(inTearingMethod.clone()) {
        Deref @ "minimalTearing" => crate::Tearing::TearingMethod::MINIMAL_TEARING,
        Deref @ "omcTearing" => crate::Tearing::TearingMethod::OMC_TEARING,
        Deref @ "cellier" => crate::Tearing::TearingMethod::CELLIER_TEARING,
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing.getTearingMethod")); __mm_s.push_str(&*literal!(" got invalid name \"")); __mm_s.push_str(&*inTearingMethod); __mm_s.push_str(&*literal!("\".")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Tearing.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTearingMethod)
}

fn callTearingMethod(
    mut inTearingMethod: TearingMethod,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut ojac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut jacType: BackendDAE::JacobianType,
    mut mixedSystem: bool,
    mut strongComponentIndex: i32,
) -> Result<(metamodelica::Ref<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool;
    let debug: bool = false;
    let mut userTVars: metamodelica::List<i32> = metamodelica::nil();
    let mut userResiduals: metamodelica::List<i32> = metamodelica::nil();
    let mut tearingMethod: TearingMethod = inTearingMethod;
    if listMember(
        strongComponentIndex,
        Flags::getConfigIntList(Flags::TOTAL_TEARING.clone())?,
    ) {
        tearingMethod = crate::Tearing::TearingMethod::TOTAL_TEARING;
    } else {
        userTVars = Flags::getConfigIntList(Flags::SET_TEARING_VARS.clone())?;
        userResiduals = Flags::getConfigIntList(Flags::SET_RESIDUAL_EQNS.clone())?;
        (userTVars, userResiduals) = getUserTearingSet(userTVars, userResiduals, strongComponentIndex)?;
        if !((userTVars).is_empty()) && !((userResiduals).is_empty()) {
            tearingMethod = crate::Tearing::TearingMethod::USER_DEFINED_TEARING;
        }
    }
    (ocomp, outRunMatching) = (match tearingMethod {
        TearingMethod::OMC_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\nTearing type: heuristic\n"));
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Tearing strictness: "));
                    __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (ocomp, outRunMatching) = omcTearing(isyst, ishared, eindex, vindx, ojac, jacType, mixedSystem)?;
            if debug {
                execStat(&(literal!("Tearing.omcTearing")))?;
            }
            (ocomp, outRunMatching)
        }
        TearingMethod::CELLIER_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\nTearing type: heuristic\n"));
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Tearing strictness: "));
                    __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (ocomp, outRunMatching) = CellierTearing(
                isyst,
                ishared,
                eindex,
                vindx,
                userTVars,
                ojac,
                jacType,
                mixedSystem,
                strongComponentIndex,
            )?;
            if debug {
                execStat(&(literal!("Tearing.CellierTearing")))?;
            }
            (ocomp, outRunMatching)
        }
        TearingMethod::TOTAL_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\nTearing type: total\n"));
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Tearing strictness: "));
                    __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (ocomp, outRunMatching) = totalTearing(isyst, ishared, eindex, vindx, ojac, jacType, mixedSystem)?;
            if debug {
                execStat(&(literal!("Tearing.totalTearing")))?;
            }
            (ocomp, outRunMatching)
        }
        TearingMethod::MINIMAL_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\nTearing type: minimal\n"));
            }
            ocomp = minimalTearing(isyst, ishared, eindex, vindx, jacType, mixedSystem)?;
            if debug {
                execStat(&(literal!("Tearing.minimalTearing")))?;
            }
            (ocomp, true)
        }
        TearingMethod::USER_DEFINED_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\nTearing type: user defined\n"));
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Tearing strictness: "));
                    __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (ocomp, outRunMatching) = userDefinedTearing(
                isyst,
                ishared,
                eindex,
                vindx,
                ojac,
                jacType,
                mixedSystem,
                userTVars,
                userResiduals,
            )?;
            if debug {
                execStat(&(literal!("Tearing.userDefinedTearing")))?;
            }
            (ocomp, outRunMatching)
        }
    });
    Ok((ocomp, outRunMatching))
}

fn tearingSystemWork(
    mut tearingMethod: TearingMethod,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inStrongComponentIndex: i32,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut outStrongComponentIndex: i32;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut runMatching: bool;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ass1 = metamodelica::Own::own(__pa0);
    ass2 = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of traverseComponents\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (comps, runMatching, outStrongComponentIndex) =
        traverseComponents(comps, &isyst, &inShared, tearingMethod, inStrongComponentIndex)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of traverseComponents\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    osyst = if (runMatching) {
        BackendDAEUtil::setEqSystMatching(
            isyst,
            metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                ass1: ass1.clone(),
                ass2: ass2.clone(),
                comps: comps,
            }),
        )
    } else {
        isyst
    };
    Ok((osyst, outShared, outStrongComponentIndex))
}

fn traverseComponents(
    mut inComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMethod: TearingMethod,
    mut strongComponentIndexIn: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    bool,
    i32,
)> {
    let mut oComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut outRunMatching: bool = false;
    let mut strongComponentIndexOut: i32 = strongComponentIndexIn;
    oComps = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = metamodelica::nil();
        for mut co in (inComps).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(co.clone()) {
                comp => {
                    let mut b: bool;
                    let mut comp = (*comp).clone();
                    (comp, b, strongComponentIndexOut) = traverseComponent(comp.clone(), isyst, ishared, inMethod, strongComponentIndexOut)?;
                    outRunMatching = outRunMatching || b;
                    comp.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((oComps, outRunMatching, strongComponentIndexOut))
}

fn traverseComponent(
    mut inComp: metamodelica::Ref<BackendDAE::StrongComponent>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMethod: TearingMethod,
    mut strongComponentIndexIn: i32,
) -> Result<(metamodelica::Ref<BackendDAE::StrongComponent>, bool, i32)> {
    let mut oComp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool;
    let mut strongComponentIndexOut: i32 = strongComponentIndexIn;
    let debug: bool = false;
    let mut debugFlag: bool =
        Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?;
    strongComponentIndexOut = (::match_deref::match_deref! { match &(&*inComp) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { .. }, .. } => {
            if debugFlag {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Handle strong component with index: ")); __mm_s.push_str(&*intString(strongComponentIndexOut + 1)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                if !(listMember(strongComponentIndexOut + 1, Flags::getConfigIntList(Flags::NO_TEARING_FOR_COMPONENT.clone())?)) {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("To disable tearing of this component use '--noTearingForComponent=")); __mm_s.push_str(&*intString(strongComponentIndexOut + 1)); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) });
                }
            }
            strongComponentIndexOut + 1
        },
        _ => strongComponentIndexOut,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oComp, outRunMatching) = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: ojac }, jacType, mixedSystem } => {
            let mut isLinear: bool;
            let mut useTearing: bool;
            isLinear = BackendDAEUtil::getLinearfromJacType(jacType.clone())?;
            useTearing = checkTearingSettings(isLinear, strongComponentIndexOut, ((vindx).len() as i32))?;
            if useTearing {
                if debugFlag {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTearing of ")); __mm_s.push_str(&*if (isLinear) {literal!("LINEAR")} else {literal!("NONLINEAR")}); __mm_s.push_str(&*literal!(" component\n")); ArcStr::from(__mm_s) });
                    let () = (match (Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?, Flags::isSet(Flags::ITERATION_VARS.clone())?) {
        (false, false) => {
            metamodelica::print(literal!("Use Flag '-d=tearingdumpV' and '-d=iterationVars' for more details\n\n"));
            ()
        },
        (false, true) => {
            metamodelica::print(literal!("Use Flag '-d=tearingdumpV' for more details\n\n"));
            ()
        },
        (true, false) => {
            metamodelica::print(literal!("Use Flag '-d=iterationVars' for more details\n\n"));
            ()
        },
        (true, true) => {
            metamodelica::print(literal!("\n"));
            ()
        },
        _ => return Err("match: no arm matched"),
    });
                }
                if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Jacobian:\n")); __mm_s.push_str(&*BackendDump::dumpJacobianStr(ojac.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                }
                if debug {
                    execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing.traverseComponent ")); __mm_s.push_str(&*if (isLinear) {literal!("LS")} else {literal!("NLS")}); __mm_s.push_str(&*literal!(" start")); ArcStr::from(__mm_s) }))?;
                }
                match '__try0: {
                    (oComp, _) = unwrap_break_err!(callTearingMethod(inMethod, isyst, ishared, eindex.clone(), vindx.clone(), ojac.clone(), jacType.clone(), mixedSystem.clone(), strongComponentIndexOut), '__try0);
                    outRunMatching = true;
                    if !(unwrap_break_err!(tearingPaysOff(&oComp, ishared, metamodelica::AsArg::as_arg(&eindex), metamodelica::AsArg::as_arg(&vindx), ojac.clone(), jacType.clone(), strongComponentIndexOut, isLinear), '__try0)) {
                        oComp = inComp.clone();
                        outRunMatching = false;
                    }
                    Ok::<_, &'static str>((oComp.clone(), outRunMatching.clone()))
                } {
                    Ok((__try0_o0, __try0_o1)) => {
                        oComp = __try0_o0;
                        outRunMatching = __try0_o1;
                    }
                    Err(_) => {
                        oComp = inComp.clone();
                        outRunMatching = false;
                    }
                }
            } else {
                oComp = inComp;
                outRunMatching = false;
            }
            (oComp, outRunMatching)
        },
        _ => {
            (inComp, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oComp, outRunMatching, strongComponentIndexOut))
}

fn checkTearingSettings(mut isLinear: bool, mut strongComponentIndex: i32, mut numVars: i32) -> Result<bool> {
    let mut activateTearing: bool = false;
    let mut debugFlag: bool =
        Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?;
    let mut maxSize: i32;
    let mut isDense: bool;
    let mut hasSparseSolver: bool;
    let mut forcedTearing: bool;
    maxSize = Flags::getConfigInt(if (isLinear) {
        Flags::MAX_SIZE_LINEAR_TEARING.clone()
    } else {
        Flags::MAX_SIZE_NONLINEAR_TEARING.clone()
    })?;
    if maxSize == 0 {
        return Ok(activateTearing);
    }
    isDense = metamodelica::stringEq(
        &(Flags::getConfigString(Flags::MATRIX_FORMAT.clone())?),
        &(literal!("dense")),
    );
    hasSparseSolver = targetHasSparseSolver(isLinear)?;
    forcedTearing = isDense && !(hasSparseSolver);
    if numVars > maxSize && !(forcedTearing) {
        Error::addMessage(
            Error::MAX_TEARING_SIZE.clone(),
            list![
                intString(strongComponentIndex),
                intString(numVars),
                if (isLinear) {
                    literal!("linear")
                } else {
                    literal!("nonlinear")
                },
                intString(maxSize),
                if (isLinear) {
                    literal!("maxSizeLinearTearing")
                } else {
                    literal!("maxSizeNonlinearTearing")
                }
            ],
        )?;
        return Ok(activateTearing);
    }
    if listMember(
        strongComponentIndex,
        Flags::getConfigIntList(Flags::NO_TEARING_FOR_COMPONENT.clone())?,
    ) {
        if debugFlag {
            metamodelica::print(literal!("\nTearing deactivated by user.\n"));
        }
        Error::addMessage(
            Error::NO_TEARING_FOR_COMPONENT.clone(),
            list![intString(strongComponentIndex)],
        )?;
        return Ok(activateTearing);
    }
    activateTearing = true;
    Ok(activateTearing)
}

fn targetHasSparseSolver(mut isLinear: bool) -> Result<bool> {
    let mut hasSparseSolver: bool = listMember(
        Config::simCodeTarget()?,
        if (isLinear) { withLSS.clone() } else { withNSS.clone() },
    );
    Ok(hasSparseSolver)
}

fn tearingPaysOff(
    mut tornComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eindex: &metamodelica::List<i32>,
    mut vindx: &metamodelica::List<i32>,
    mut ojac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut jacType: BackendDAE::JacobianType,
    mut strongComponentIndex: i32,
    mut isLinear: bool,
) -> Result<bool> {
    let mut keep: bool = true;
    let entryCost: metamodelica::Real = metamodelica::OrderedFloat(4.0_f64);
    let fillFactor: metamodelica::Real = metamodelica::OrderedFloat(14.0_f64);
    let precisionDigits: metamodelica::Real = metamodelica::OrderedFloat(16.0_f64);
    let trustedCoeffs: metamodelica::Real = metamodelica::OrderedFloat(0.9_f64);
    let mut jac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
    let mut tvars: metamodelica::List<i32>;
    let mut residuals: metamodelica::List<i32>;
    let mut innerVars: metamodelica::List<i32>;
    let mut stack: metamodelica::List<(i32, metamodelica::Real)>;
    let mut dep: (i32, metamodelica::Real) = (0, metamodelica::OrderedFloat(0.0_f64));
    let mut dEqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut eqDeps: metamodelica::Array<metamodelica::List<(i32, metamodelica::Real)>>;
    let mut tvarId: metamodelica::Array<i32>;
    let mut solvedBy: metamodelica::Array<i32>;
    let mut stamp: metamodelica::Array<i32>;
    let mut depth: metamodelica::Array<i32>;
    let mut growth: metamodelica::Array<metamodelica::Real>;
    let mut n: i32;
    let mut nnz: i32;
    let mut t: i32;
    let mut m: i32;
    let mut row: i32;
    let mut col: i32;
    let mut v: i32 = 0;
    let mut w: i32;
    let mut eq: i32;
    let mut cnt: i32;
    let mut nnzA: i32;
    let mut maxRowA: i32;
    let mut maxDepth: i32;
    let mut d: i32;
    let mut valued: i32 = 0;
    let mut density: metamodelica::Real;
    let mut densityA: metamodelica::Real;
    let mut buildTorn: metamodelica::Real;
    let mut costDense: metamodelica::Real;
    let mut costSparse: metamodelica::Real;
    let mut costTornDense: metamodelica::Real;
    let mut costTornSparse: metamodelica::Real;
    let mut costTorn: metamodelica::Real;
    let mut costUntorn: metamodelica::Real;
    let mut rn: metamodelica::Real;
    let mut rt: metamodelica::Real;
    let mut g: metamodelica::Real;
    let mut coeff: metamodelica::Real;
    let mut pivot: metamodelica::Real;
    let mut maxGrowth: metamodelica::Real;
    let mut digitsLost: metamodelica::Real;
    let mut known: bool;
    let mut isValued: bool;
    (jac, tvars, residuals, innerEquations, known) = (::match_deref::match_deref! { match &((ojac, &**tornComp)) {
        (Some(__esc_jac), Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __esc_tvars, residualequations: __esc_residuals, innerEquations: __esc_innerEquations, .. }, .. }) => {
            jac = (*__esc_jac).clone();
            tvars = (*__esc_tvars).clone();
            residuals = (*__esc_residuals).clone();
            innerEquations = (*__esc_innerEquations).clone();
            (jac.clone(), tvars.clone(), residuals.clone(), innerEquations.clone(), true)
        },
        _ => (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(known) {
        return Ok(keep);
    }
    n = ((vindx).len() as i32);
    nnz = ((jac).len() as i32);
    t = ((tvars).len() as i32);
    m = ((innerEquations).len() as i32);
    globalKnownVars = BackendDAEUtil::getGlobalKnownVarsFromShared(shared);
    eqDeps = arrayCreate(n, metamodelica::nil());
    for mut entry in &*jac {
        (row, col, dEqn) = entry.clone();
        (coeff, isValued) = jacEntryMagnitude(&dEqn, globalKnownVars.clone());
        valued = valued + if (isValued) { 1 } else { 0 };
        metamodelica::arrayUpdate(
            eqDeps.clone(),
            row,
            metamodelica::cons(
                (col, coeff),
                ({
                    let __elt = (*metamodelica::index_checked(&eqDeps.borrow(), row)?).clone();
                    __elt
                }),
            ),
        )?;
    }
    tvarId = arrayCreate(n, 0);
    growth = arrayCreate(n, metamodelica::OrderedFloat(0.0_f64));
    cnt = 0;
    for mut gv in &*tvars {
        cnt = cnt + 1;
        v = List::position(gv.clone(), vindx)?;
        metamodelica::arrayUpdate(tvarId.clone(), v, cnt)?;
        metamodelica::arrayUpdate(growth.clone(), v, metamodelica::OrderedFloat(1.0_f64))?;
    }
    solvedBy = arrayCreate(n, 0);
    depth = arrayCreate(n, 0);
    maxDepth = 0;
    for mut ie in &*innerEquations {
        (eq, innerVars) = (match ie.clone() {
            BackendDAE::InnerEquation::INNEREQUATION { .. } => (
                List::position(
                    var_field!(ie.eqn, BackendDAE::InnerEquation::INNEREQUATION).clone(),
                    eindex,
                )?,
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut gv in (var_field!(ie.vars, BackendDAE::InnerEquation::INNEREQUATION).clone())
                        .into_iter()
                        .cloned()
                    {
                        let __x = List::position(gv.clone(), vindx)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            ),
            BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS { .. } => (
                List::position(
                    var_field!(ie.eqn, BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS).clone(),
                    eindex,
                )?,
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut gv in (var_field!(ie.vars, BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS).clone())
                        .into_iter()
                        .cloned()
                    {
                        let __x = List::position(gv.clone(), vindx)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            ),
        });
        d = 0;
        g = metamodelica::OrderedFloat(0.0_f64);
        pivot = metamodelica::OrderedFloat(0.0_f64);
        let __range0 = &*({
            let __elt = (*metamodelica::index_checked(&eqDeps.borrow(), eq)?).clone();
            __elt
        });
        for mut dep in __range0 {
            let mut dep = dep.clone();
            w = Util::tuple21(dep);
            coeff = Util::tuple22(dep);
            if listMember(w, innerVars.clone()) {
                pivot = realMax(pivot, coeff);
            } else {
                d = intMax(
                    d,
                    ({
                        let __elt = (*metamodelica::index_checked(&depth.borrow(), w)?).clone();
                        __elt
                    }),
                );
                g = g + coeff
                    * ({
                        let __elt = (*metamodelica::index_checked(&growth.borrow(), w)?).clone();
                        __elt
                    });
            }
        }
        g = if (pivot > metamodelica::OrderedFloat(0.0_f64)) {
            metamodelica::real_div_checked(g, pivot)?
        } else {
            g
        };
        for mut v in &*innerVars {
            let mut v = v.clone();
            metamodelica::arrayUpdate(solvedBy.clone(), v, eq)?;
            metamodelica::arrayUpdate(depth.clone(), v, d + 1)?;
            metamodelica::arrayUpdate(
                growth.clone(),
                v,
                if (g > metamodelica::OrderedFloat(1e300_f64)) {
                    metamodelica::OrderedFloat(1e300_f64)
                } else {
                    g
                },
            )?;
        }
        maxDepth = intMax(maxDepth, d + 1);
    }
    stamp = arrayCreate(n, 0);
    nnzA = 0;
    maxRowA = 0;
    maxGrowth = metamodelica::OrderedFloat(0.0_f64);
    row = 0;
    for mut ge in &*residuals {
        row = row + 1;
        cnt = 0;
        eq = List::position(ge.clone(), eindex)?;
        g = metamodelica::OrderedFloat(0.0_f64);
        pivot = metamodelica::OrderedFloat(0.0_f64);
        let __range1 = &*({
            let __elt = (*metamodelica::index_checked(&eqDeps.borrow(), eq)?).clone();
            __elt
        });
        for mut dep in __range1 {
            let mut dep = dep.clone();
            w = Util::tuple21(dep);
            coeff = Util::tuple22(dep);
            g = g + coeff
                * ({
                    let __elt = (*metamodelica::index_checked(&growth.borrow(), w)?).clone();
                    __elt
                });
            pivot = realMax(pivot, coeff);
        }
        maxGrowth = realMax(
            maxGrowth,
            if (pivot > metamodelica::OrderedFloat(0.0_f64)) {
                metamodelica::real_div_checked(g, pivot)?
            } else {
                g
            },
        );
        stack = ({
            let __elt = (*metamodelica::index_checked(&eqDeps.borrow(), eq)?).clone();
            __elt
        });
        while !((stack).is_empty()) {
            v = Util::tuple21((stack).head().cloned()?);
            stack = (stack).rest()?;
            if ({
                let __elt = (*metamodelica::index_checked(&stamp.borrow(), v)?).clone();
                __elt
            }) != row
            {
                metamodelica::arrayUpdate(stamp.clone(), v, row)?;
                if ({
                    let __elt = (*metamodelica::index_checked(&tvarId.borrow(), v)?).clone();
                    __elt
                }) > 0
                {
                    cnt = cnt + 1;
                } else if ({
                    let __elt = (*metamodelica::index_checked(&solvedBy.borrow(), v)?).clone();
                    __elt
                }) > 0
                {
                    stack = listAppend(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &eqDeps.borrow(),
                                ({
                                    let __elt = (*metamodelica::index_checked(&solvedBy.borrow(), v)?).clone();
                                    __elt
                                }),
                            )?)
                            .clone();
                            __elt
                        }),
                        stack,
                    );
                }
            }
        }
        nnzA = nnzA + cnt;
        maxRowA = intMax(maxRowA, cnt);
    }
    rn = intReal(n);
    rt = intReal(t);
    density = metamodelica::real_div_checked(intReal(nnz), (rn * rn))?;
    densityA = if (t > 0) {
        metamodelica::real_div_checked(intReal(nnzA), (rt * rt))?
    } else {
        metamodelica::OrderedFloat(1.0_f64)
    };
    digitsLost = if (maxGrowth > metamodelica::OrderedFloat(1.0_f64)) {
        (maxGrowth).log10()
    } else {
        metamodelica::OrderedFloat(0.0_f64)
    };
    buildTorn = entryCost * intReal(maxRowA * nnz);
    costDense = entryCost * intReal(nnz) + solveCost(rn, intReal(nnz), false, metamodelica::OrderedFloat(1.0_f64))?;
    costSparse = entryCost * intReal(nnz) + solveCost(rn, intReal(nnz), true, fillFactor)?;
    costTornDense = buildTorn + solveCost(rt, intReal(nnzA), false, metamodelica::OrderedFloat(1.0_f64))?;
    costTornSparse = buildTorn + solveCost(rt, intReal(nnzA), true, metamodelica::OrderedFloat(1.0_f64))?;
    costUntorn = if (SimCodeCodegenUtil::useSparseSolver(n, nnz, isLinear)?) {
        costSparse
    } else {
        costDense
    };
    costTorn = if (SimCodeCodegenUtil::useSparseSolver(t, nnzA, isLinear)?) {
        costTornSparse
    } else {
        costTornDense
    };
    if Flags::isSet(Flags::TEARING_COST.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[tearingCost] component "));
            __mm_s.push_str(&*intString(strongComponentIndex));
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*if (isLinear) { literal!("LS") } else { literal!("NLS") });
            __mm_s.push_str(&*literal!(" ["));
            __mm_s.push_str(&*BackendDump::jacobianTypeStr(jacType));
            __mm_s.push_str(&*literal!("]"));
            __mm_s.push_str(&*literal!(" n="));
            __mm_s.push_str(&*intString(n));
            __mm_s.push_str(&*literal!(" nnz="));
            __mm_s.push_str(&*intString(nnz));
            __mm_s.push_str(&*literal!(" density="));
            __mm_s.push_str(&*realString(density));
            __mm_s.push_str(&*literal!(" t="));
            __mm_s.push_str(&*intString(t));
            __mm_s.push_str(&*literal!(" inner="));
            __mm_s.push_str(&*intString(m));
            __mm_s.push_str(&*literal!(" nnzA="));
            __mm_s.push_str(&*intString(nnzA));
            __mm_s.push_str(&*literal!(" densityA="));
            __mm_s.push_str(&*realString(densityA));
            __mm_s.push_str(&*literal!(" colors="));
            __mm_s.push_str(&*intString(maxRowA));
            __mm_s.push_str(&*literal!(" chain="));
            __mm_s.push_str(&*intString(maxDepth));
            __mm_s.push_str(&*literal!(" digitsLost="));
            __mm_s.push_str(&*realString(digitsLost));
            __mm_s.push_str(&*literal!(" valuedCoeffs="));
            __mm_s.push_str(&*intString(valued));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*intString(nnz));
            __mm_s.push_str(&*literal!(" cost dense="));
            __mm_s.push_str(&*realString(costDense));
            __mm_s.push_str(&*literal!(" sparse="));
            __mm_s.push_str(&*realString(costSparse));
            __mm_s.push_str(&*literal!(" tornDense="));
            __mm_s.push_str(&*realString(costTornDense));
            __mm_s.push_str(&*literal!(" tornSparse="));
            __mm_s.push_str(&*realString(costTornSparse));
            __mm_s.push_str(&*literal!(" format="));
            __mm_s.push_str(&*if (SimCodeCodegenUtil::useSparseSolver(n, nnz, isLinear)?) {
                literal!("sparse")
            } else {
                literal!("dense")
            });
            __mm_s.push_str(&*literal!(" tornFormat="));
            __mm_s.push_str(&*if (SimCodeCodegenUtil::useSparseSolver(t, nnzA, isLinear)?) {
                literal!("sparse")
            } else {
                literal!("dense")
            });
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    if !(isLinear) {
        return Ok(keep);
    }
    if digitsLost >= precisionDigits && intReal(valued) >= trustedCoeffs * intReal(nnz) {
        Error::addMessage(
            Error::TEARING_AMPLIFIES_ERROR.clone(),
            list![
                intString(strongComponentIndex),
                intString(m),
                intString(((digitsLost).0.floor() as i32))
            ],
        )?;
        keep = false;
    } else if costUntorn * Flags::getConfigReal(Flags::TEARING_COST_MARGIN.clone())? < costTorn {
        Error::addMessage(
            Error::TEARING_NOT_WORTH_IT.clone(),
            list![
                intString(strongComponentIndex),
                intString(t),
                intString(((costTorn).0.floor() as i32)),
                intString(((costUntorn).0.floor() as i32)),
                intString(n)
            ],
        )?;
        keep = false;
    }
    Ok(keep)
}

fn jacEntryMagnitude(
    mut dEqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut globalKnownVars: BackendDAE::Variables,
) -> (metamodelica::Real, bool) {
    let mut magnitude: metamodelica::Real;
    let mut known: bool;
    let mut exp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    (magnitude, known) = 'mc: {
        let __mc_input = &**dEqn;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    exp = var_field!((**dEqn).exp, BackendDAE::Equation::RESIDUAL_EQUATION).clone();
                    for mut i in 1..=3 {
                        if Expression::isScalarConst(&exp) {
                            break;
                        }
                        (exp, _) = Expression::traverseExpBottomUp(exp.clone(), &fnptr!(substituteKnownVar, metamodelica::Ref<DAE::Exp>, BackendDAE::Variables), globalKnownVars.clone())?;
                        (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
                    }
                    Ok((((Expression::toReal(&exp)?).abs(), true), exp.clone()))
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
                    Ok((metamodelica::OrderedFloat(1.0_f64), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (magnitude, known)
}

fn substituteKnownVar(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut globalKnownVars: BackendDAE::Variables,
) -> (metamodelica::Ref<DAE::Exp>, BackendDAE::Variables) {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    exp = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    (var, _) = BackendVariable::getVarSingle(var_field!((*exp).componentRef, DAE::Exp::CREF), &globalKnownVars)?;
                    Ok(BackendVariable::varBindExp(&var)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (exp, globalKnownVars)
}

fn solveCost(
    mut size: metamodelica::Real,
    mut nnz: metamodelica::Real,
    mut sparse: bool,
    mut fill: metamodelica::Real,
) -> Result<metamodelica::Real> {
    let mut cost: metamodelica::Real;
    cost = if (size <= metamodelica::OrderedFloat(0.0_f64)) {
        metamodelica::OrderedFloat(0.0_f64)
    } else if (sparse) {
        metamodelica::real_div_checked(fill * nnz * nnz, size)? + metamodelica::OrderedFloat(2.0_f64) * nnz
    } else {
        metamodelica::real_div_checked(metamodelica::OrderedFloat(2.0_f64), metamodelica::OrderedFloat(3.0_f64))?
            * size
            * size
            * size
            + metamodelica::OrderedFloat(2.0_f64) * size * size
    };
    Ok(cost)
}

fn getUserTearingSet(
    mut userTVars: metamodelica::List<i32>,
    mut userResiduals: metamodelica::List<i32>,
    mut strongComponentIndex: i32,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut userTvarsThisComponent: metamodelica::List<i32> = metamodelica::nil();
    let mut userResidualsThisComponent: metamodelica::List<i32> = metamodelica::nil();
    let mut i: i32;
    let mut len: i32;
    let mut start: i32;
    let mut end_: i32;
    let mut arr_TVars: metamodelica::Array<i32>;
    let mut arr_residuals: metamodelica::Array<i32>;
    arr_TVars = metamodelica::arrayFromVec(userTVars.clone().into_iter().cloned().collect());
    arr_residuals = metamodelica::arrayFromVec(userResiduals.clone().into_iter().cloned().collect());
    i = 1;
    len = ((userTVars).len() as i32);
    while i < len {
        if ({
            let __elt = (*metamodelica::index_checked(&arr_TVars.borrow(), i)?).clone();
            __elt
        }) == strongComponentIndex
        {
            start = i + 2;
            end_ = i
                + 1
                + ({
                    let __elt = (*metamodelica::index_checked(&arr_TVars.borrow(), i + 1)?).clone();
                    __elt
                });
            userTvarsThisComponent = List::unique(
                &({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut j in (start..=end_).into_iter() {
                        let __x = ({
                            let __elt = (*metamodelica::index_checked(&arr_TVars.borrow(), j.clone())?).clone();
                            __elt
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            );
            if ((userTvarsThisComponent).len() as i32)
                != ({
                    let __elt = (*metamodelica::index_checked(&arr_TVars.borrow(), i + 1)?).clone();
                    __elt
                })
            {
                Error::addMessage(
                    Error::USER_DEFINED_TEARING_ERROR.clone(),
                    list![literal!("The selected tearing variables must have unique indexes.")],
                )?;
                return Err("fail");
            }
            break;
        } else {
            i = i
                + 2
                + ({
                    let __elt = (*metamodelica::index_checked(&arr_TVars.borrow(), i + 1)?).clone();
                    __elt
                });
        }
    }
    if !((userTvarsThisComponent).is_empty()) {
        i = 1;
        len = ((userResiduals).len() as i32);
        while i < len {
            if ({
                let __elt = (*metamodelica::index_checked(&arr_residuals.borrow(), i)?).clone();
                __elt
            }) == strongComponentIndex
            {
                start = i + 2;
                end_ = i
                    + 1
                    + ({
                        let __elt = (*metamodelica::index_checked(&arr_residuals.borrow(), i + 1)?).clone();
                        __elt
                    });
                userResidualsThisComponent = List::unique(
                    &({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut j in (start..=end_).into_iter() {
                            let __x = ({
                                let __elt = (*metamodelica::index_checked(&arr_residuals.borrow(), j.clone())?).clone();
                                __elt
                            });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                );
                if ((userResidualsThisComponent).len() as i32)
                    != ({
                        let __elt = (*metamodelica::index_checked(&arr_residuals.borrow(), i + 1)?).clone();
                        __elt
                    })
                {
                    Error::addMessage(
                        Error::USER_DEFINED_TEARING_ERROR.clone(),
                        list![literal!("The selected residual equations must have unique indexes.")],
                    )?;
                    return Err("fail");
                }
                break;
            } else {
                i = i
                    + 2
                    + ({
                        let __elt = (*metamodelica::index_checked(&arr_residuals.borrow(), i + 1)?).clone();
                        __elt
                    });
            }
        }
    }
    Ok((userTvarsThisComponent, userResidualsThisComponent))
}

// =============================================================================
//
// method: omc tearing
//
// =============================================================================
fn omcTearing(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut ojac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut jacType: BackendDAE::JacobianType,
    mut mixedSystem: bool,
) -> Result<(metamodelica::Ref<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool;
    let mut tvars: metamodelica::List<i32>;
    let mut residual: metamodelica::List<i32>;
    let mut unsolvables: metamodelica::List<i32>;
    let mut othercomps: metamodelica::List<metamodelica::List<i32>>;
    let mut subsyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut columark: metamodelica::Array<i32>;
    let mut size: i32;
    let mut tornsize: i32;
    let mut mark: i32;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut m1: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt1: metamodelica::Array<metamodelica::List<i32>>;
    let mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut tSel_always: metamodelica::List<i32>;
    let mut tSel_prefer: metamodelica::List<i32>;
    let mut tSel_avoid: metamodelica::List<i32>;
    let mut tSel_never: metamodelica::List<i32>;
    let mut DAEtypeStr: ArcStr;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of omcTearing\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    DAEtypeStr = BackendDump::printBackendDAEType2String(ishared.backendDAEType.clone())?;
    size = ((vindx).len() as i32);
    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst))?;
    eqns = BackendEquation::listEquation(&eqn_lst)?;
    var_lst = List::map1r(
        vindx.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        BackendVariable::daeVars(isyst),
    )?;
    vars = BackendVariable::listVar1(&var_lst)?;
    subsyst = BackendDAEUtil::createEqSystem(
        vars.clone(),
        eqns,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    funcs = BackendDAEUtil::getFunctions(ishared);
    (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        subsyst,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        Some(funcs),
        BackendDAEUtil::isInitializationDAE(ishared),
    )?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!(
            "\n\n###BEGIN print Strong Component#####################\n(Function:omcTearing)\n"
        ));
        BackendDump::printEqSystem(subsyst.clone())?;
        metamodelica::print(literal!(
            "\n###END print Strong Component#######################\n(Function:omcTearing)\n\n\n"
        ));
    }
    (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&subsyst, ishared, false)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\n\nAdjacencyMatrixEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
        metamodelica::print(literal!("\nAdjacencyMatrixTransposedEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
        metamodelica::print(literal!("\nmapEqnIncRow:"));
        BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone())?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(mapIncRowEqn.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    ass1 = arrayCreate(size, -1);
    ass2 = arrayCreate(size, -1);
    unsolvables = getUnsolvableVars(size, meT.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\n\nUnsolvable Vars:\n"));
        BackendDump::debuglst(
            &unsolvables,
            &fnptr!(intString, i32),
            &(literal!(", ")),
            &(literal!("\n")),
        )?;
    }
    columark = arrayCreate(size, -1);
    (tSel_always, tSel_prefer, tSel_avoid, tSel_never, _) = tearingSelect(&var_lst, metamodelica::nil(), &DAEtypeStr)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of omcTearing2\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (tvars, mark) = omcTearing2(
        unsolvables,
        tSel_always,
        &tSel_prefer,
        tSel_avoid,
        tSel_never,
        me.clone(),
        meT.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
        size,
        vars,
        ishared,
        ass1.clone(),
        ass2.clone(),
        columark.clone(),
        1,
        metamodelica::nil(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of omcTearing2\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    ass1 = List::fold(&tvars, &unassignTVars, ass1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n* BFS RESULTS:\n* ass1: "));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(ass1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("* ass2: "));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(ass2.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    residual = Matching::getUnassigned(size, ass2.clone(), metamodelica::nil())?;
    tornsize = ((tvars).len() as i32);
    let true = (intLt(tornsize, size)) else {
        return Err("pattern mismatch");
    };
    m1 = arrayCreate(size, metamodelica::nil());
    mt1 = arrayCreate(size, metamodelica::nil());
    m1 = AdjacencyMatrix::getOtherEqSysAdjacencyMatrix(m.clone(), size, 1, ass2.clone(), ass1.clone(), m1.clone())?;
    mt1 = AdjacencyMatrix::getOtherEqSysAdjacencyMatrix(mt.clone(), size, 1, ass1.clone(), ass2.clone(), mt1.clone())?;
    othercomps = Sorting::TarjanTransposed(mt1.clone(), ass2.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\nOtherEquationsOrder:\n"));
        BackendDump::dumpComponentsOLD(&othercomps)?;
        metamodelica::print(literal!("\n"));
    }
    mt1 = arrayCreate(size, metamodelica::nil());
    mark = getDependenciesOfVars(
        &othercomps,
        ass1.clone(),
        ass2.clone(),
        m.clone(),
        mt1.clone(),
        columark.clone(),
        mark,
    )?;
    (residual, mark) = sortResidualDepentOnTVars(
        residual,
        &tvars,
        ass1.clone(),
        m.clone(),
        mt1.clone(),
        columark.clone(),
        mark,
    )?;
    (ocomp, outRunMatching) = omcTearing4(
        jacType,
        isyst,
        ishared,
        &subsyst,
        tvars.clone(),
        residual.clone(),
        ass1.clone(),
        ass2.clone(),
        othercomps,
        eindex,
        vindx,
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
        columark.clone(),
        mark,
        mixedSystem,
    );
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(if (outRunMatching) {
            literal!("\nStatus:\nOk system torn\n\n")
        } else {
            literal!("\nStatus:\nSystem not torn\n\n")
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!(
                "\n* TEARING RESULTS:\n*\n* No of equations in strong component: "
            ));
            __mm_s.push_str(&*intString(size));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("* No of tVars: "));
            __mm_s.push_str(&*intString(tornsize));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("*\n* tVars: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(tvars, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("*\n* resEq: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(residual, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n*\n*"));
            ArcStr::from(__mm_s)
        });
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ocomp.clone()) {
            Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __pa0, residualequations: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        tvars = metamodelica::Own::own(__pa0);
        residual = metamodelica::Own::own(__pa1);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n* Related to entire Equationsystem:\n* =====\n* tVars: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(tvars, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n* =====\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("*\n* =====\n* resEq: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(residual, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n* =====\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\n\nStrongComponents:\n"));
        BackendDump::dumpComponent(&ocomp, None)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nEND of omcTearing\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((ocomp, outRunMatching))
}

fn getUnsolvableVars(
    mut size: i32,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<metamodelica::List<i32>> {
    let __ab_meT = meT.borrow();
    let mut unsolvables: metamodelica::List<i32> = metamodelica::nil();
    let mut isUnsolvable: bool;
    for mut index in 1..=size {
        isUnsolvable = unsolvable(&(*metamodelica::index_checked(&__ab_meT, index)?))?;
        if isUnsolvable {
            unsolvables = metamodelica::cons(index, unsolvables);
        }
    }
    Ok(unsolvables)
}

pub(crate) fn unsolvable(
    mut elem: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> Result<bool> {
    let mut isUnsolvable: bool = true;
    let mut e: i32;
    let mut s: BackendDAE::Solvability;
    for mut el in &**elem {
        (e, s, _) = el.clone();
        if solvable(s)? {
            if e > 0 {
                isUnsolvable = false;
                return Ok(isUnsolvable);
            }
        }
    }
    Ok(isUnsolvable)
}

fn unassignTVars(mut v: i32, mut inAss: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut outAss: metamodelica::Array<i32>;
    outAss = metamodelica::arrayUpdate(inAss.clone(), v, -1)?;
    Ok(outAss)
}

fn getDependenciesOfVars<'__b>(
    mut iComps: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut visited: metamodelica::Array<i32>,
    mut iMark: i32,
) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match iComps {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iMark)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: c, tail: Deref @ metamodelica::ListNode::Nil }, tail: comps } => {
                let mut v: i32;
                let mut tvars: metamodelica::List<i32>;
                let mut vars: metamodelica::List<i32>;
                v = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), c.clone())?).clone(); __elt});
                vars = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                tvars = tVarsofEqn(&vars, ass1.clone(), mT.clone(), visited.clone(), iMark, metamodelica::nil())?;
                metamodelica::arrayUpdate(mT.clone(), v, tvars)?;
                { (iComps, ass1, ass2, m, mT, visited, iMark) = (comps, ass1.clone(), ass2.clone(), m.clone(), mT.clone(), visited.clone(), iMark + 1); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: comp, tail: comps } => {
                let mut tvars: metamodelica::List<i32>;
                let mut vars: metamodelica::List<i32>;
                vars = List::map1r(comp.clone(), &arrayGet, ass2.clone())?;
                tvars = tVarsofEqns(metamodelica::AsArg::as_arg(&comp), m.clone(), ass1.clone(), mT.clone(), visited.clone(), iMark)?;
                List::fold1r(&vars, &*(Arc::new(arrayUpdate.clone())), tvars, mT.clone())?;
                { (iComps, ass1, ass2, m, mT, visited, iMark) = (comps, ass1.clone(), ass2.clone(), m.clone(), mT.clone(), visited.clone(), iMark + 1); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn tVarsofEqns(
    mut iEqns: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut visited: metamodelica::Array<i32>,
    mut iMark: i32,
) -> Result<metamodelica::List<i32>> {
    let __ab_m = m.borrow();
    let mut oAcc: metamodelica::List<i32> = metamodelica::nil();
    let mut vars: metamodelica::List<i32>;
    for mut e in &**iEqns {
        vars = List::select(
            (*metamodelica::index_checked(&__ab_m, e.clone())?).clone(),
            (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
        )?;
        oAcc = tVarsofEqn(&vars, ass1.clone(), mT.clone(), visited.clone(), iMark, oAcc)?;
    }
    Ok(oAcc)
}

fn tVarsofEqn(
    mut iVars: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut visited: metamodelica::Array<i32>,
    mut iMark: i32,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_ass1 = ass1.borrow();
    let __ab_mT = mT.borrow();
    let mut oAcc: metamodelica::List<i32> = iAcc;
    for mut v in &**iVars {
        if intLt((*metamodelica::index_checked(&__ab_ass1, v.clone())?).clone(), 0) {
            oAcc = uniqueIntLst(v.clone(), iMark, visited.clone(), oAcc)?;
        } else {
            oAcc = List::fold2(
                &(*metamodelica::index_checked(&__ab_mT, v.clone())?),
                &uniqueIntLst,
                iMark,
                visited.clone(),
                oAcc,
            )?;
        }
    }
    Ok(oAcc)
}

fn uniqueIntLst(
    mut c: i32,
    mut mark: i32,
    mut markarray: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oAcc: metamodelica::List<i32> = iAcc;
    if !(intEq(
        mark,
        ({
            let __elt = (*metamodelica::index_checked(&markarray.borrow(), c)?).clone();
            __elt
        }),
    )) {
        metamodelica::arrayUpdate(markarray.clone(), c, mark)?;
        oAcc = metamodelica::cons(c, oAcc);
    }
    Ok(oAcc)
}

fn sortResidualDepentOnTVars(
    mut iResiduals: metamodelica::List<i32>,
    mut iTVars: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut visited: metamodelica::Array<i32>,
    mut iMark: i32,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut oResiduals: metamodelica::List<i32>;
    let mut oMark: i32;
    let mut size: i32;
    let mut maplst: metamodelica::List<metamodelica::List<i32>>;
    let mut map: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqnLocalGlobal: metamodelica::Array<i32>;
    let mut varGlobalLocal: metamodelica::Array<i32>;
    let mut v1: metamodelica::Array<i32>;
    let mut v2: metamodelica::Array<i32>;
    eqnLocalGlobal = metamodelica::arrayFromVec(iResiduals.clone().into_iter().cloned().collect());
    varGlobalLocal = arrayCreate(metamodelica::arrayLength(m.clone()), -1);
    varGlobalLocal = getGlobalLocal(iTVars, 1, varGlobalLocal.clone())?;
    (oMark, maplst) = tVarsofResidualEqns(
        iResiduals,
        m.clone(),
        ass1.clone(),
        mT.clone(),
        varGlobalLocal.clone(),
        visited.clone(),
        iMark,
    )?;
    map = metamodelica::arrayFromVec(maplst.into_iter().cloned().collect());
    size = metamodelica::arrayLength(map.clone());
    Matching::matchingExternalsetAdjacencyMatrix(size, size, map.clone())?;
    BackendDAEEXT::matching(size, size, 5, -1, metamodelica::OrderedFloat(1.0_f64), 1);
    v1 = arrayCreate(size, -1);
    v2 = arrayCreate(size, -1);
    BackendDAEEXT::getAssignment(v2.clone(), v1.clone())?;
    oResiduals = getTVarResiduals(size, v1.clone(), eqnLocalGlobal.clone(), metamodelica::nil())?;
    Ok((oResiduals, oMark))
}

fn getGlobalLocal(
    mut iTVars: &metamodelica::List<i32>,
    mut index: i32,
    mut iVarGlobalLocal: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut oVarGlobalLocal: metamodelica::Array<i32> = iVarGlobalLocal;
    let mut idx: i32 = index;
    for mut i in &**iTVars {
        metamodelica::arrayUpdate(oVarGlobalLocal.clone(), i.clone(), idx)?;
        idx = idx + 1;
    }
    Ok(oVarGlobalLocal)
}

fn tVarsofResidualEqns(
    mut iEqns: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut varGlobalLocal: metamodelica::Array<i32>,
    mut visited: metamodelica::Array<i32>,
    mut iMark: i32,
) -> Result<(i32, metamodelica::List<metamodelica::List<i32>>)> {
    let __ab_m = m.borrow();
    let mut oMark: i32 = iMark;
    let mut oAcc: metamodelica::List<metamodelica::List<i32>>;
    oAcc = ({
        let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
        for mut eq in (iEqns).into_iter().cloned() {
            let __x = (match eq.clone() {
                mut e => {
                    let mut vars: metamodelica::List<i32>;
                    let mut tvars: metamodelica::List<i32>;
                    vars = List::select(
                        (*metamodelica::index_checked(&__ab_m, e)?).clone(),
                        (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                            as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
                    )?;
                    tvars = tVarsofEqn(
                        &vars,
                        ass1.clone(),
                        mT.clone(),
                        visited.clone(),
                        oMark,
                        metamodelica::nil(),
                    )?;
                    tvars = List::map1r(tvars.clone(), &arrayGet, varGlobalLocal.clone())?;
                    oMark = oMark + 1;
                    tvars.clone()
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((oMark, oAcc))
}

fn getTVarResiduals(
    mut index: i32,
    mut v1: metamodelica::Array<i32>,
    mut eqnLocalGlobal: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        match index {
            0 => return Ok(iAcc),
            _ => {
                let mut e: i32;
                e = ({
                    let __elt = (*metamodelica::index_checked(&v1.borrow(), index)?).clone();
                    __elt
                });
                e = ({
                    let __elt = (*metamodelica::index_checked(&eqnLocalGlobal.borrow(), e)?).clone();
                    __elt
                });
                {
                    (index, v1, eqnLocalGlobal, iAcc) = (
                        index - 1,
                        v1.clone(),
                        eqnLocalGlobal.clone(),
                        metamodelica::cons(e, iAcc),
                    );
                    continue '__tco;
                }
            }
        }
    }
}

fn omcTearing2(
    mut unsolvables: metamodelica::List<i32>,
    mut tSel_always: metamodelica::List<i32>,
    mut tSel_prefer: &metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut size: i32,
    mut vars: BackendDAE::Variables,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut columark: metamodelica::Array<i32>,
    mut mark: i32,
    mut inTVars: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut outTVars: metamodelica::List<i32> = metamodelica::nil();
    let mut oMark: i32 = 0;
    (outTVars, oMark) = 'mc: {
        let __mc_input = (&*unsolvables, &*tSel_always);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    let mut tvar: i32;
                    let mut unassigned: metamodelica::List<i32>;
                    let mut vareqns: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
                    let mut oMark: i32 = oMark.clone();
                    let mut outTVars: metamodelica::List<i32> = outTVars.clone();
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of omcTearingSelectTearingVar\n\n\n")); ArcStr::from(__mm_s) });
                    }
                    tvar = omcTearingSelectTearingVar(vars.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), tSel_prefer, tSel_avoid.clone(), tSel_never.clone())?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of omcTearingSelectTearingVar\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    metamodelica::arrayUpdate(ass1.clone(), tvar, size * 2)?;
                    vareqns = List::removeOnTrue(ass2.clone(), &move |__a0: metamodelica::Array<i32>, __a1: (i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)| isAssignedSaveEnhanced(__a0, &__a1), ({let __elt = (*metamodelica::index_checked(&mt.borrow(), tvar)?).clone(); __elt}))?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print(literal!("Assignable equations containing new tvar:\n"));
                        BackendDump::dumpAdjacencyRowEnhanced(&vareqns)?;
                        metamodelica::print(literal!("\n"));
                    }
                    tearingBFS(&vareqns, m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, ass1.clone(), ass2.clone(), &(metamodelica::nil()))?;
                    unassigned = Matching::getUnassigned(size, ass1.clone(), metamodelica::nil())?;
                    (outTVars, oMark) = omcTearing3(&unassigned, metamodelica::nil(), tSel_always.clone(), tSel_prefer, tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, vars.clone(), ishared, ass1.clone(), ass2.clone(), columark.clone(), mark + 1, metamodelica::cons(tvar, inTVars.clone()))?;
                    Ok(((outTVars.clone(), oMark), oMark.clone(), outTVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oMark = __wb0;
            outTVars = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: tvar, tail: rest }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut unassigned: metamodelica::List<i32>;
                    let mut vareqns: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
                    let mut oMark: i32 = oMark.clone();
                    let mut outTVars: metamodelica::List<i32> = outTVars.clone();
                    if listMember(tvar.clone(), tSel_never.clone()) {
                        Error::addCompilerWarning(literal!("There are tearing variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.never'. Use -d=tearingdump and -d=tearingdumpV for more information."))?;
                    }
                    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variable:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tVar: ")); __mm_s.push_str(&*intString(tvar.clone())); __mm_s.push_str(&*literal!(" (unsolvable in omcTearing2)\n\n\n")); ArcStr::from(__mm_s) });
                    }
                    metamodelica::arrayUpdate(ass1.clone(), tvar.clone(), size * 2)?;
                    vareqns = List::removeOnTrue(ass2.clone(), &move |__a0: metamodelica::Array<i32>, __a1: (i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)| isAssignedSaveEnhanced(__a0, &__a1), ({let __elt = (*metamodelica::index_checked(&mt.borrow(), tvar.clone())?).clone(); __elt}))?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print(literal!("Assignable equations containing new tvar:\n"));
                        BackendDump::dumpAdjacencyRowEnhanced(&vareqns)?;
                        metamodelica::print(literal!("\n"));
                    }
                    tearingBFS(&vareqns, m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, ass1.clone(), ass2.clone(), &(metamodelica::nil()))?;
                    unassigned = Matching::getUnassigned(size, ass1.clone(), metamodelica::nil())?;
                    (outTVars, oMark) = omcTearing3(&unassigned, rest.clone(), tSel_always.clone(), tSel_prefer, tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, vars.clone(), ishared, ass1.clone(), ass2.clone(), columark.clone(), mark + 1, metamodelica::cons(tvar.clone(), inTVars.clone()))?;
                    Ok(((outTVars.clone(), oMark), oMark.clone(), outTVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oMark = __wb0;
            outTVars = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut unassigned: metamodelica::List<i32>;
                    let mut unsolv: metamodelica::List<i32>;
                    let mut vareqns: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
                    let mut oMark: i32 = oMark.clone();
                    let mut outTVars: metamodelica::List<i32> = outTVars.clone();
                    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variables:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variables with annotation attribute 'always' as tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(tSel_always.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    markTVarsOrResiduals(&tSel_always, ass1.clone())?;
                    (_, unsolv, _) = List::intersection1OnTrue(unsolvables.clone(), tSel_always.clone(), &fnptr!(intEq, i32, i32))?;
                    vareqns = findVareqns(ass2.clone(), &move |__a0: metamodelica::Array<i32>, __a1: (i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)| isAssignedSaveEnhanced(__a0, &__a1), mt.clone(), &tSel_always)?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print(literal!("Assignable equations containing new tvars:\n"));
                        BackendDump::dumpAdjacencyRowEnhanced(&vareqns)?;
                        metamodelica::print(literal!("\n"));
                    }
                    tearingBFS(&vareqns, m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, ass1.clone(), ass2.clone(), &(metamodelica::nil()))?;
                    unassigned = Matching::getUnassigned(size, ass1.clone(), metamodelica::nil())?;
                    (outTVars, oMark) = omcTearing3(&unassigned, unsolv.clone(), metamodelica::nil(), tSel_prefer, tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, vars.clone(), ishared, ass1.clone(), ass2.clone(), columark.clone(), mark + 1, listAppend(tSel_always.clone(), inTVars.clone()))?;
                    Ok(((outTVars.clone(), oMark), oMark.clone(), outTVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oMark = __wb0;
            outTVars = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("Tearing.omcTearing2 failed!"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outTVars, oMark))
}

fn findVareqns(
    mut ass2In: metamodelica::Array<i32>,
    mut inCompFunc: &dyn ::std::ops::Fn(
        metamodelica::Array<i32>,
        (
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        ),
    ) -> Result<bool>,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut tSel_alwaysIn: &metamodelica::List<i32>,
) -> Result<
    metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
> {
    pub type CompFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<i32>,
                (
                    i32,
                    BackendDAE::Solvability,
                    metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
                ),
            ) -> Result<bool>
            + 'static,
    >;

    let __ab_mt = mt.borrow();
    let mut vareqnsOut: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )> = metamodelica::nil();
    for mut tvar in &**tSel_alwaysIn {
        vareqnsOut = List::append_reverse(
            &(List::removeOnTrue(
                ass2In.clone(),
                inCompFunc,
                (*metamodelica::index_checked(&__ab_mt, tvar.clone())?).clone(),
            )?),
            vareqnsOut,
        );
    }
    vareqnsOut = List::unique(&vareqnsOut);
    Ok(vareqnsOut)
}

fn omcTearingSelectTearingVar(
    mut vars: BackendDAE::Variables,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut tSel_prefer: &metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
) -> Result<i32> {
    let mut tearingVar: i32;
    tearingVar = 'mc: {
        let __mc_input = &*tSel_never;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut unsolvables: metamodelica::List<i32>;
                    let mut tvar: i32;
                    unsolvables = getUnsolvableVarsConsiderMatching(BackendVariable::varsSize(&vars), mt.clone(), ass1.clone(), ass2.clone())?;
                    let false = ((unsolvables).is_empty()) else { return Err("pattern mismatch") };
                    tvar = (unsolvables).head().cloned()?;
                    if listMember(tvar, tSel_never.clone()) {
                        Error::addCompilerWarning(literal!("There are tearing variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.never'. Use -d=tearingdump and -d=tearingdumpV for more information."))?;
                    }
                    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variable:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tVar: ")); __mm_s.push_str(&*intString(tvar)); __mm_s.push_str(&*literal!(" (unsolvable in omcTearingSelectTearingVar)\n\n")); ArcStr::from(__mm_s) });
                    }
                    Ok(tvar)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut freeVars: metamodelica::List<i32>;
                    let mut eqns: metamodelica::List<i32>;
                    let mut pointsLst: metamodelica::List<i32>;
                    let mut tvar: i32;
                    let mut varsize: i32;
                    let mut points: metamodelica::Array<i32>;
                    varsize = BackendVariable::varsSize(&vars);
                    freeVars = Matching::getUnassigned(varsize, ass1.clone(), metamodelica::nil())?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print(literal!("omcTearingSelectTearingVar Candidates(unassigned vars):\n"));
                        BackendDump::debuglst(&freeVars, &fnptr!(intString, i32), &(literal!(", ")), &(literal!("\n")))?;
                    }
                    (_, freeVars, _) = List::intersection1OnTrue(freeVars.clone(), tSel_never.clone(), &fnptr!(intEq, i32, i32))?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print(literal!("Candidates without variables with annotation attribute 'never':\n"));
                        BackendDump::debuglst(&freeVars, &fnptr!(intString, i32), &(literal!(", ")), &(literal!("\n")))?;
                    }
                    let false = ((freeVars).is_empty()) else { return Err("pattern mismatch") };
                    points = arrayCreate(varsize, 0);
                    points = List::fold2(&freeVars, &calcVarWeights, mt.clone(), ass2.clone(), points.clone())?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPoints after 'calcVarWeights':\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(points.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    eqns = Matching::getUnassigned(metamodelica::arrayLength(m.clone()), ass2.clone(), metamodelica::nil())?;
                    points = List::fold2(&eqns, &fnptr!(addEqnWeights, i32, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>), m.clone(), ass1.clone(), points.clone())?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after 'addEqnWeights':\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(points.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    points = List::fold1(&freeVars, &move |__a0: i32, __a1: BackendDAE::Variables, __a2: metamodelica::Array<i32>| discriminateDiscrete(__a0, &__a1, __a2), vars.clone(), points.clone())?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after 'discriminateDiscrete':\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(points.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    pointsLst = preferAvoidVariables(&freeVars, points.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>(), tSel_prefer, metamodelica::OrderedFloat(3.0_f64));
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after preferring variables with attribute 'prefer':\n")); __mm_s.push_str(&*stringDelimitList(List::map(pointsLst.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    pointsLst = preferAvoidVariables(&freeVars, pointsLst.clone(), &tSel_avoid, metamodelica::OrderedFloat(0.334_f64));
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after discrimination against variables with attribute 'avoid':\n")); __mm_s.push_str(&*stringDelimitList(List::map(pointsLst.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    tvar = selectVarWithMostPoints(&freeVars, &pointsLst)?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tVar: ")); __mm_s.push_str(&*intString(tvar)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((pointsLst).get(tvar)?)); __mm_s.push_str(&*literal!(" points)\n\n")); ArcStr::from(__mm_s) });
                    } else if listMember(tvar, tSel_avoid.clone()) {
                        Error::addCompilerWarning(literal!("The Tearing heuristic has chosen variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.avoid'. Use -d=tearingdump and -d=tearingdumpV for more information."))?;
                    }
                    Ok(tvar)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("omcTearingSelectTearingVar failed because no unmatched var!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(tearingVar)
}

fn getUnsolvableVarsConsiderMatching(
    mut size: i32,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_ass1 = ass1.borrow();
    let __ab_meT = meT.borrow();
    let mut unsolvables: metamodelica::List<i32> = metamodelica::nil();
    let mut elem: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    let mut isUnsolvable: bool;
    for mut index in 1..=size {
        if intLt((*metamodelica::index_checked(&__ab_ass1, index)?).clone(), 0) {
            elem = (*metamodelica::index_checked(&__ab_meT, index)?).clone();
            elem = removeMatched(&elem, ass2.clone())?;
            isUnsolvable = unsolvable(&elem)?;
            if isUnsolvable {
                unsolvables = metamodelica::cons(index, unsolvables);
            }
        }
    }
    Ok(unsolvables)
}

fn removeMatched(
    mut elem: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<
    metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
> {
    let __ab_ass2 = ass2.borrow();
    let mut oAcc: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )> = metamodelica::nil();
    let mut e: i32;
    for mut el in &**elem {
        (e, _, _) = el.clone();
        if intGt(e, 0) && intLt((*metamodelica::index_checked(&__ab_ass2, e)?).clone(), 0) {
            oAcc = metamodelica::cons(el.clone(), oAcc);
        }
    }
    Ok(oAcc)
}

fn calcVarWeights(
    mut v: i32,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass2: metamodelica::Array<i32>,
    mut iPoints: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let __ab_mt = mt.borrow();
    let mut oPoints: metamodelica::Array<i32>;
    let mut p: i32;
    p = calcSolvabilityWeight(&(*metamodelica::index_checked(&__ab_mt, v)?), ass2.clone())?;
    oPoints = metamodelica::arrayUpdate(iPoints.clone(), v, p)?;
    Ok(oPoints)
}

fn calcSolvabilityWeight(
    mut inRow: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<i32> {
    let mut w: i32;
    w = List::fold1(
        inRow,
        &move |__a0: (
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        ),
               __a1: metamodelica::Array<i32>,
               __a2: i32| solvabilityWeightsnoStates(&__a0, __a1, __a2),
        ass2.clone(),
        0,
    )?;
    Ok(w)
}

fn solvabilityWeightsnoStates(
    mut inTpl: &(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
    mut ass: metamodelica::Array<i32>,
    mut iW: i32,
) -> Result<i32> {
    let __ab_ass = ass.borrow();
    let mut oW: i32;
    oW = (::match_deref::match_deref! { match &(inTpl) {
        (eq, s, _) if (intGt(eq.clone(), 0) && !(intGt((*metamodelica::index_checked(&__ab_ass, eq.clone())?).clone(), 0))) => {
            let mut w: i32;
            w = solvabilityWeights(s.clone());
            intAdd(w, iW)
        },
        _ => {
            iW
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oW)
}

fn solvabilityWeights(mut solva: BackendDAE::Solvability) -> i32 {
    let mut i: i32;
    i = (match solva {
        BackendDAE::Solvability::SOLVABILITY_SOLVED { .. } => 0,
        BackendDAE::Solvability::SOLVABILITY_CONSTONE { .. } => 2,
        BackendDAE::Solvability::SOLVABILITY_CONST { .. } => 5,
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false } => 0,
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true } => 50,
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: false } => 0,
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: true } => 100,
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. } => 200,
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE { .. } => 300,
        _ => 0,
    });
    i
}

fn addEqnWeights(
    mut e: i32,
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1: metamodelica::Array<i32>,
    mut iPoints: metamodelica::Array<i32>,
) -> metamodelica::Array<i32> {
    let __ab_m = m.borrow();
    let mut oPoints: metamodelica::Array<i32>;
    oPoints = 'mc: {
        let __mc_input = iPoints.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v1: i32;
            let mut v2: i32;
            let mut points: metamodelica::Array<i32>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::removeOnTrue(ass1.clone(), &move |__a0: metamodelica::Array<i32>, __a1: (i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)| isAssignedSaveEnhanced(__a0, &__a1), (*metamodelica::index_checked(&__ab_m, e)?).clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: (__pa0, _, _), tail: Deref @ metamodelica::ListNode::Cons { head: (__pa1, _, _), tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            v1 = metamodelica::Own::own(__pa0);
            v2 = metamodelica::Own::own(__pa1);
            points = metamodelica::arrayUpdate(
                iPoints.clone(),
                v1,
                ({
                    let __elt = (*metamodelica::index_checked(&iPoints.borrow(), v1)?).clone();
                    __elt
                }) + 5,
            )?;
            points = metamodelica::arrayUpdate(
                iPoints.clone(),
                v2,
                ({
                    let __elt = (*metamodelica::index_checked(&points.borrow(), v2)?).clone();
                    __elt
                }) + 5,
            )?;
            Ok(points.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iPoints.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oPoints
}

fn isAssignedSaveEnhanced(
    mut ass: metamodelica::Array<i32>,
    mut inTpl: &(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inTpl) {
        (i, _, _) if (intGt(i.clone(), 0)) => {
            intGt((*metamodelica::index_checked(&__ab_ass, i.clone())?).clone(), 0)
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

fn discriminateDiscrete(
    mut v: i32,
    mut vars: &BackendDAE::Variables,
    mut iPoints: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut oPoints: metamodelica::Array<i32>;
    let mut p: i32;
    let mut b: bool;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    var = BackendVariable::getVarAt(vars, v)?;
    b = BackendVariable::isVarDiscrete(&var);
    p = ({
        let __elt = (*metamodelica::index_checked(&iPoints.borrow(), v)?).clone();
        __elt
    });
    p = if (b) { intDiv(p, 10) } else { p };
    oPoints = metamodelica::arrayUpdate(iPoints.clone(), v, p)?;
    Ok(oPoints)
}

fn selectVarWithMostPoints(mut vars: &metamodelica::List<i32>, mut points: &metamodelica::List<i32>) -> Result<i32> {
    let mut oVar: i32 = -1;
    let mut defp: i32 = -1;
    let mut p: i32;
    for mut v in &**vars {
        p = (points).get(v.clone())?;
        if p > defp {
            defp = p;
            oVar = v.clone();
        }
    }
    Ok(oVar)
}

fn tearingBFS(
    mut queue: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut size: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextQueue: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (queue, nextQueue) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            ()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            let mut newqueue: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
            newqueue = List::removeOnTrue(ass2.clone(), &move |__a0: metamodelica::Array<i32>, __a1: (i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)| isAssignedSaveEnhanced(__a0, &__a1), nextQueue.clone())?;
            newqueue = sortEqnsSolvable(&newqueue, m.clone())?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("Use next Queue!\n"));
            }
            tearingBFS(&newqueue, m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, ass1.clone(), ass2.clone(), &(metamodelica::nil()))?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: (c, _, _), tail: rest }, _) => {
            let mut eqnsize: i32;
            let mut cnonscalar: i32;
            let mut newqueue: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
            let mut rows: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("Queue:\n"));
                BackendDump::dumpAdjacencyRowEnhanced(queue)?;
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Process Eqn: ")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            rows = List::removeOnTrue(ass1.clone(), &move |__a0: metamodelica::Array<i32>, __a1: (i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)| isAssignedSaveEnhanced(__a0, &__a1), ({let __elt = (*metamodelica::index_checked(&m.borrow(), c.clone())?).clone(); __elt}))?;
            cnonscalar = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), c.clone())?).clone(); __elt});
            eqnsize = ((({let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), cnonscalar)?).clone(); __elt})).len() as i32);
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Eqn Size: ")); __mm_s.push_str(&*intString(eqnsize)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Rows (not assigned variables in eqn ")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!("):\n")); ArcStr::from(__mm_s) });
                BackendDump::dumpAdjacencyRowEnhanced(&rows)?;
                metamodelica::print(literal!("\n"));
            }
            newqueue = tearingBFS1(&rows, eqnsize, ({let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), cnonscalar)?).clone(); __elt}), mt.clone(), ass1.clone(), ass2.clone(), nextQueue.clone())?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("Next Queue:\n"));
                BackendDump::dumpAdjacencyRowEnhanced(&newqueue)?;
                metamodelica::print(literal!("\n\n"));
            }
            tearingBFS(rest, m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, ass1.clone(), ass2.clone(), &newqueue)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn sortEqnsSolvable(
    mut queue: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<
    metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
> {
    let mut nextQueue: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    let mut qnon: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    let mut qsolv: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    (qnon, qsolv) = List::split1OnTrue(
        queue,
        &move |__a0: (
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        ),
               __a1: metamodelica::Array<
            metamodelica::List<(
                i32,
                BackendDAE::Solvability,
                metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
            )>,
        >| hasnonlinearVars(&__a0, __a1),
        m.clone(),
    )?;
    nextQueue = listAppend(qsolv, qnon);
    Ok(nextQueue)
}

fn hasnonlinearVars(
    mut entry: &(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<bool> {
    let __ab_m = m.borrow();
    let mut hasnonlinear: bool;
    let mut r: i32;
    let mut row: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    (r, _, _) = entry.clone();
    row = (*metamodelica::index_checked(&__ab_m, r)?).clone();
    hasnonlinear = hasnonlinearVars1(&row);
    Ok(hasnonlinear)
}

fn hasnonlinearVars1<'__b>(
    mut row: &'__b metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match row {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: (_, BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. }, _), tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { row = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn tearingBFS1(
    mut rows: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut size: i32,
    mut c: metamodelica::List<i32>,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inNextQueue: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> Result<
    metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
> {
    let mut outNextQueue: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    outNextQueue = (::match_deref::match_deref! { match &(inNextQueue.clone()) {
        _ if (intEq(((rows).len() as i32), size) && solvableLst(rows)?) => {
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Assign Eqns: ")); __mm_s.push_str(&*stringDelimitList(List::map(c.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            tearingBFS2(rows, &c, mt.clone(), ass1.clone(), ass2.clone(), inNextQueue)?
        },
        _ => inNextQueue,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNextQueue)
}

fn solvableLst(
    mut rows: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> Result<bool> {
    let mut solvable: bool = true;
    let mut s: BackendDAE::Solvability;
    for mut r in &**rows {
        (_, s, _) = r.clone();
        if !(self::solvable(s)?) {
            solvable = false;
            return Ok(solvable);
        }
    }
    Ok(solvable)
}

fn solvable(mut s: BackendDAE::Solvability) -> Result<bool> {
    let mut b: bool;
    b = (match s {
        BackendDAE::Solvability::SOLVABILITY_SOLVED { .. } => true,
        BackendDAE::Solvability::SOLVABILITY_CONSTONE { .. } => true,
        BackendDAE::Solvability::SOLVABILITY_CONST { b: mut __esc_b } => {
            b = __esc_b.clone();
            b
        }
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: mut __esc_b } => {
            b = __esc_b.clone();
            b && !(stringEqual(
                &(Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?),
                &(literal!("veryStrict")),
            ))
        }
        BackendDAE::Solvability::SOLVABILITY_LINEAR { .. } => false,
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. } => false,
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE { .. } => false,
        BackendDAE::Solvability::SOLVABILITY_SOLVABLE { .. } => true,
        _ => false,
    });
    Ok(b)
}

fn isEntrySolved(
    mut entry: &(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(entry) {
        (_, BackendDAE::Solvability::SOLVABILITY_SOLVED { .. }, _) => true,
        (_, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: __esc_b }, _) => {
            b = (*__esc_b).clone();
            Error::addInternalError(literal!("SOLVABILITY_PARAMETER is not handled yet. Requires revision."), metamodelica::sourceInfo!("BackEnd/Tearing.mo"))?;
            b.clone() && !(stringEqual(&(Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?), &(literal!("veryStrict"))))
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn isEntrySolvable(
    mut entry: (
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
) -> Result<bool> {
    let mut b: bool;
    b = solvable(Util::tuple32(entry))?;
    Ok(b)
}

fn tearingBFS2<'__b>(
    mut rows: &'__b metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut clst: &'__b metamodelica::List<i32>,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inNextQueue: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> Result<
    metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
> {
    '__tco: loop {
        ::match_deref::match_deref! { match (rows, clst) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inNextQueue)
            },
            (Deref @ metamodelica::ListNode::Cons { head: (r, _, _), tail: rest }, Deref @ metamodelica::ListNode::Cons { head: c, tail: ilst }) => {
                let mut vareqns: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
                let mut newqueue: metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>;
                if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Assignment: Eq ")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!(" - Var ")); __mm_s.push_str(&*intString(r.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                }
                metamodelica::arrayUpdate(ass1.clone(), r.clone(), c.clone())?;
                metamodelica::arrayUpdate(ass2.clone(), c.clone(), r.clone())?;
                if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                }
                vareqns = List::removeOnTrue(ass2.clone(), &move |__a0: metamodelica::Array<i32>, __a1: (i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)| isAssignedSaveEnhanced(__a0, &__a1), ({let __elt = (*metamodelica::index_checked(&mt.borrow(), r.clone())?).clone(); __elt}))?;
                newqueue = listAppend(inNextQueue, vareqns);
                { (rows, clst, mt, ass1, ass2, inNextQueue) = (rest, ilst, mt.clone(), ass1.clone(), ass2.clone(), newqueue); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn omcTearing3(
    mut unassigned: &metamodelica::List<i32>,
    mut unsolvables: metamodelica::List<i32>,
    mut tSel_always: metamodelica::List<i32>,
    mut tSel_prefer: &metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mt: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut size: i32,
    mut vars: BackendDAE::Variables,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut columark: metamodelica::Array<i32>,
    mut mark: i32,
    mut inTVars: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut outTVars: metamodelica::List<i32>;
    let mut oMark: i32;
    (outTVars, oMark) = (::match_deref::match_deref! { match unassigned {
        Deref @ metamodelica::ListNode::Nil => (inTVars, mark),
        _ => {
            (outTVars, oMark) = omcTearing2(unsolvables, tSel_always, tSel_prefer, tSel_avoid, tSel_never, m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size, vars, ishared, ass1.clone(), ass2.clone(), columark.clone(), mark, inTVars)?;
            (outTVars, oMark)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outTVars, oMark))
}

fn omcTearing4(
    mut jacType: BackendDAE::JacobianType,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut subsyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut tvars: metamodelica::List<i32>,
    mut residual: metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut othercomps: metamodelica::List<metamodelica::List<i32>>,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut columark: metamodelica::Array<i32>,
    mut mark: i32,
    mut mixedSystem: bool,
) -> (metamodelica::Ref<BackendDAE::StrongComponent>, bool) {
    let mut ocomp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool;
    (ocomp, outRunMatching) = 'mc: {
        let __mc_input = mixedSystem;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ores: metamodelica::List<i32>;
            let mut residual1: metamodelica::List<i32>;
            let mut ovars: metamodelica::List<i32>;
            let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
            let mut eindxarr: metamodelica::Array<i32>;
            let mut varindxarr: metamodelica::Array<i32>;
            let mut linear: bool;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("handle torn System\n"));
            }
            residual1 = List::map1r(residual.clone(), &arrayGet, mapIncRowEqn.clone())?;
            residual1 = List::fold2(&residual1, &uniqueIntLst, mark, columark.clone(), metamodelica::nil())?;
            eindxarr = metamodelica::arrayFromVec(eindex.clone().into_iter().cloned().collect());
            ores = List::map1r(residual1.clone(), &arrayGet, eindxarr.clone())?;
            varindxarr = metamodelica::arrayFromVec(vindx.clone().into_iter().cloned().collect());
            ovars = List::map1r(tvars.clone(), &arrayGet, varindxarr.clone())?;
            innerEquations = omcTearing4_1(
                othercomps.clone(),
                ass2.clone(),
                mapIncRowEqn.clone(),
                eindxarr.clone(),
                varindxarr.clone(),
                columark.clone(),
                mark,
            )?;
            linear = BackendDAEUtil::getLinearfromJacType(jacType)?;
            Ok((
                metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
                    strictTearingSet: BackendDAE::TearingSet {
                        tearingvars: ovars.clone(),
                        residualequations: ores.clone(),
                        innerEquations: innerEquations.clone(),
                        jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
                    },
                    casualTearingSet: None,
                    linear: linear,
                    mixedSystem: mixedSystem,
                }),
                true,
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((
                metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
                    strictTearingSet: BackendDAE::TearingSet {
                        tearingvars: metamodelica::nil(),
                        residualequations: metamodelica::nil(),
                        innerEquations: metamodelica::nil(),
                        jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
                    },
                    casualTearingSet: None,
                    linear: false,
                    mixedSystem: mixedSystem,
                }),
                false,
            ))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (ocomp, outRunMatching)
}

fn omcTearing4_1(
    mut othercomps: metamodelica::List<metamodelica::List<i32>>,
    mut ass2: metamodelica::Array<i32>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut eindxarr: metamodelica::Array<i32>,
    mut varindxarr: metamodelica::Array<i32>,
    mut columark: metamodelica::Array<i32>,
    mut mark: i32,
) -> Result<metamodelica::List<BackendDAE::InnerEquation>> {
    let __ab_eindxarr = eindxarr.borrow();
    let mut outInnerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    outInnerEquations = ({
        let mut __acc: metamodelica::List<BackendDAE::InnerEquation> = metamodelica::nil();
        for mut x in (othercomps).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(x.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut e: i32;
                    let mut v: i32;
                    e = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), c.clone())?).clone(); __elt});
                    e = (*metamodelica::index_checked(&__ab_eindxarr, e)?).clone();
                    v = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), c.clone())?).clone(); __elt});
                    v = ({let __elt = (*metamodelica::index_checked(&varindxarr.borrow(), v)?).clone(); __elt});
                    BackendDAE::InnerEquation::INNEREQUATION { eqn: e, vars: list![v] }
                },
                clst => {
                    let mut vlst: metamodelica::List<i32>;
                    let mut elst: metamodelica::List<i32>;
                    let mut e: i32;
                    elst = List::map1r(clst.clone(), &arrayGet, mapIncRowEqn.clone())?;
                    elst = List::fold2(&elst, &uniqueIntLst, mark, columark.clone(), metamodelica::nil())?;
                    let __pa0 = ::match_deref::match_deref! { match &(elst.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    e = (*metamodelica::index_checked(&__ab_eindxarr, e)?).clone();
                    vlst = List::map1r(clst.clone(), &arrayGet, ass2.clone())?;
                    vlst = List::map1r(vlst.clone(), &arrayGet, varindxarr.clone())?;
                    BackendDAE::InnerEquation::INNEREQUATION { eqn: e, vars: vlst.clone() }
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outInnerEquations)
}

// ============================================================================
// Section for minimal tearing
//   Tear only the minimal amount of variables from strong components which are
//   all discrete variables and CSE variables.
// ============================================================================
fn minimalTearing(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut jacType: BackendDAE::JacobianType,
    mut mixedSystem: bool,
) -> Result<metamodelica::Ref<BackendDAE::StrongComponent>> {
    let mut ocomp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut size: i32;
    let mut qidx: i32;
    let mut vidx: i32;
    let mut nE: metamodelica::Array<i32>;
    let mut nV: metamodelica::Array<i32>;
    let mut varArray: metamodelica::Array<bool>;
    let mut eqArray: metamodelica::Array<bool>;
    let mut unsolvedDiscreteVars: metamodelica::List<i32>;
    let mut unsolvedCSEVars: metamodelica::List<i32>;
    let mut unsolvedCombined: metamodelica::List<i32>;
    let mut algSolvedVars: metamodelica::List<i32>;
    let mut iterationVars: metamodelica::List<i32> = metamodelica::nil();
    let mut residualequations: metamodelica::List<i32> = metamodelica::nil();
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut innerEquationsLocalIndex: metamodelica::List<BackendDAE::InnerEquation> = metamodelica::nil();
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let mut adjEnh: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut adjEnhT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut linear: bool;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut subsyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    linear = BackendDAEUtil::getLinearfromJacType(jacType)?;
    match '__try0: {
        eqn_lst = unwrap_break_err!(BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst)), '__try0);
        eqns = unwrap_break_err!(BackendEquation::listEquation(&eqn_lst), '__try0);
        var_lst = unwrap_break_err!(List::map1r(vindx.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), BackendVariable::daeVars(isyst)), '__try0);
        vars = unwrap_break_err!(BackendVariable::listVar1(&var_lst), '__try0);
        subsyst = BackendDAEUtil::createEqSystem(
            vars.clone(),
            eqns.clone(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        (adjEnh, adjEnhT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrixEnhanced(&subsyst, ishared, BackendDAEUtil::isInitializationDAE(ishared)), '__try0);
        size = ((vindx).len() as i32);
        varArray = arrayCreate(size, true);
        eqArray = arrayCreate(size, true);
        nE = arrayCreate(size, -1);
        nV = arrayCreate(size, -1);
        unsolvedDiscreteVars = unwrap_break_err!(findDiscreteWarnTearingSelect(&var_lst), '__try0);
        unsolvedCSEVars = findCSE(&var_lst);
        unsolvedCombined = unwrap_break_err!(List::uniqueIntN(&(listAppend(unsolvedDiscreteVars.clone(), unsolvedCSEVars.clone())), ((var_lst).len() as i32)), '__try0).reverse();
        qidx = 1;
        for mut eqn in &*eqn_lst {
            if BackendEquation::isAlgorithm(metamodelica::AsArg::as_arg(&eqn)) {
                {
                    let __cell1 = false;
                    let __idx1 = qidx;
                    *unwrap_break_err!(metamodelica::index_mut_checked(&mut eqArray.clone().borrow_mut(), __idx1), '__try0) =
                        __cell1;
                }
                algSolvedVars = metamodelica::nil();
                let __range2 = &*({
                    let __elt =
                        (*unwrap_break_err!(metamodelica::index_checked(&adjEnh.borrow(), qidx), '__try0)).clone();
                    __elt
                });
                for mut entr in __range2 {
                    if unwrap_break_err!(isEntrySolved(&(entr.clone())), '__try0) {
                        (vidx, _, _) = entr.clone();
                        algSolvedVars = metamodelica::cons(vidx, algSolvedVars.clone());
                        (unsolvedCombined, _) = unwrap_break_err!(List::deleteMemberOnTrue(vidx, unsolvedCombined.clone(), &fnptr!(intEq, i32, i32)), '__try0);
                        {
                            let __cell3 = false;
                            let __idx3 = vidx;
                            *unwrap_break_err!(metamodelica::index_mut_checked(&mut varArray.clone().borrow_mut(), __idx3), '__try0) =
                                __cell3;
                        }
                    }
                }
                innerEquationsLocalIndex = metamodelica::cons(
                    BackendDAE::InnerEquation::INNEREQUATION {
                        eqn: qidx,
                        vars: algSolvedVars.clone(),
                    },
                    innerEquationsLocalIndex.clone(),
                );
            }
            qidx = qidx + 1;
        }
        if !((unsolvedCombined).is_empty()) {
            unwrap_break_err!(matchDiscreteVars(&unsolvedCombined, adjEnhT.clone(), varArray.clone(), eqArray.clone(), nE.clone(), nV.clone()), '__try0);
            (varArray, eqArray, innerEquations) = unwrap_break_err!(getTearingSetfromAssign(&unsolvedCombined, nE.clone(), varArray.clone(), eqArray.clone()), '__try0);
            for mut iq in &*innerEquations {
                innerEquationsLocalIndex = metamodelica::cons(iq.clone(), innerEquationsLocalIndex.clone());
            }
        }
        for mut i in 1..=((eindex).len() as i32) {
            if ({
                let __elt = (*unwrap_break_err!(metamodelica::index_checked(&eqArray.borrow(), i), '__try0)).clone();
                __elt
            }) {
                residualequations = metamodelica::cons(i, residualequations.clone());
            }
        }
        for mut i in 1..=((vindx).len() as i32) {
            if ({
                let __elt = (*unwrap_break_err!(metamodelica::index_checked(&varArray.borrow(), i), '__try0)).clone();
                __elt
            }) {
                iterationVars = metamodelica::cons(i, iterationVars.clone());
            }
        }
        innerEquations = ({
            let mut __acc: metamodelica::List<BackendDAE::InnerEquation> = metamodelica::nil();
            for mut ieqn in (innerEquationsLocalIndex.clone()).into_iter().cloned() {
                let __x = (match ieqn.clone() {
                    BackendDAE::InnerEquation::INNEREQUATION { .. } => {
                        let __owned_variant_vars_0 = unwrap_break_err!(selectFromList_rev(&vindx, var_field!(ieqn.vars, BackendDAE::InnerEquation::INNEREQUATION).clone()), '__try0);
                        let __owned_variant_eqn_1 = unwrap_break_err!((eindex).get(var_field!(ieqn.eqn, BackendDAE::InnerEquation::INNEREQUATION).clone()), '__try0);
                        if let BackendDAE::InnerEquation::INNEREQUATION { vars, eqn, .. } = &mut ieqn {
                            *vars = __owned_variant_vars_0;
                            *eqn = __owned_variant_eqn_1;
                        } else {
                            panic!(
                                "owned-variant field-assign: value held a different variant than BackendDAE::InnerEquation::INNEREQUATION"
                            );
                        }
                        ieqn.clone()
                    }
                    _ => break '__try0 Err::<_, _>("fail"),
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        iterationVars = unwrap_break_err!(selectFromList_rev(&vindx, iterationVars.clone()), '__try0);
        residualequations = unwrap_break_err!(selectFromList_rev(&eindex, residualequations.clone()), '__try0);
        ocomp = metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet: BackendDAE::TearingSet {
                tearingvars: iterationVars.clone().reverse(),
                residualequations: residualequations.clone().reverse(),
                innerEquations: innerEquations.clone().reverse(),
                jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
            },
            casualTearingSet: None,
            linear: linear,
            mixedSystem: mixedSystem,
        });
        Ok::<_, &'static str>((
            adjEnh.clone(),
            adjEnhT.clone(),
            eqArray.clone(),
            eqn_lst.clone(),
            eqns.clone(),
            innerEquations.clone(),
            iterationVars.clone(),
            nE.clone(),
            nV.clone(),
            ocomp.clone(),
            qidx.clone(),
            residualequations.clone(),
            size.clone(),
            subsyst.clone(),
            unsolvedCSEVars.clone(),
            unsolvedCombined.clone(),
            unsolvedDiscreteVars.clone(),
            varArray.clone(),
            var_lst.clone(),
            vars.clone(),
        ))
    } {
        Ok((
            __try0_o0,
            __try0_o1,
            __try0_o2,
            __try0_o3,
            __try0_o4,
            __try0_o5,
            __try0_o6,
            __try0_o7,
            __try0_o8,
            __try0_o9,
            __try0_o10,
            __try0_o11,
            __try0_o12,
            __try0_o13,
            __try0_o14,
            __try0_o15,
            __try0_o16,
            __try0_o17,
            __try0_o18,
            __try0_o19,
        )) => {
            adjEnh = __try0_o0;
            adjEnhT = __try0_o1;
            eqArray = __try0_o2;
            eqn_lst = __try0_o3;
            eqns = __try0_o4;
            innerEquations = __try0_o5;
            iterationVars = __try0_o6;
            nE = __try0_o7;
            nV = __try0_o8;
            ocomp = __try0_o9;
            qidx = __try0_o10;
            residualequations = __try0_o11;
            size = __try0_o12;
            subsyst = __try0_o13;
            unsolvedCSEVars = __try0_o14;
            unsolvedCombined = __try0_o15;
            unsolvedDiscreteVars = __try0_o16;
            varArray = __try0_o17;
            var_lst = __try0_o18;
            vars = __try0_o19;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function minimalTearing failed"),
                metamodelica::sourceInfo!("BackEnd/Tearing.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok(ocomp)
}

fn matchDiscreteVars(
    mut inDiscreteVars: &metamodelica::List<i32>,
    mut adjEnhT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut varArray: metamodelica::Array<bool>,
    mut eqArray: metamodelica::Array<bool>,
    mut nE: metamodelica::Array<i32>,
    mut nV: metamodelica::Array<i32>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut nE: metamodelica::Array<i32> = nE;
    let mut nV: metamodelica::Array<i32> = nV;
    let mut eqMarker: metamodelica::Array<bool>;
    match '__try0: {
        for mut varIdx in &**inDiscreteVars {
            eqMarker = metamodelica::arrayFromVec(eqArray.clone().borrow().clone());
            let (__pa1, __pa2, __pa3, true) = (unwrap_break_err!(pathFound(varIdx.clone(), adjEnhT.clone(), varArray.clone(), eqArray.clone(), eqMarker.clone(), nE.clone(), nV.clone()), '__try0))
            else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
            eqMarker = metamodelica::Own::own(__pa1);
            nE = metamodelica::Own::own(__pa2);
            nV = metamodelica::Own::own(__pa3);
        }
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function matchDiscreteVars failed"),
                metamodelica::sourceInfo!("BackEnd/Tearing.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok((nE, nV))
}

fn pathFound(
    mut varIdx: i32,
    mut adjEnhT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut varArray: metamodelica::Array<bool>,
    mut eqArray: metamodelica::Array<bool>,
    mut eqMarker: metamodelica::Array<bool>,
    mut nE: metamodelica::Array<i32>,
    mut nV: metamodelica::Array<i32>,
) -> Result<(
    metamodelica::Array<bool>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    bool,
)> {
    let mut eqMarker: metamodelica::Array<bool> = eqMarker;
    let mut nE: metamodelica::Array<i32> = nE;
    let mut nV: metamodelica::Array<i32> = nV;
    let mut success: bool = false;
    let mut eqIdx: i32;
    match '__try0: {
        let __range1 = &*({
            let __elt = (*unwrap_break_err!(metamodelica::index_checked(&adjEnhT.borrow(), varIdx), '__try0)).clone();
            __elt
        });
        for mut entry in __range1 {
            (eqIdx, _, _) = entry.clone();
            if unwrap_break_err!(isEntrySolvable(entry.clone()), '__try0) && eqIdx > 0 {
                if ({
                    let __elt =
                        (*unwrap_break_err!(metamodelica::index_checked(&eqArray.borrow(), eqIdx), '__try0)).clone();
                    __elt
                }) && ({
                    let __elt = (*unwrap_break_err!(metamodelica::index_checked(&nV.borrow(), eqIdx), '__try0)).clone();
                    __elt
                }) == -1
                {
                    {
                        let __cell2 = varIdx;
                        let __idx2 = eqIdx;
                        *unwrap_break_err!(metamodelica::index_mut_checked(&mut nV.clone().borrow_mut(), __idx2), '__try0) =
                            __cell2;
                    }
                    {
                        let __cell3 = eqIdx;
                        let __idx3 = varIdx;
                        *unwrap_break_err!(metamodelica::index_mut_checked(&mut nE.clone().borrow_mut(), __idx3), '__try0) =
                            __cell3;
                    }
                    success = true;
                    return Ok((eqMarker, nE, nV, success));
                }
            }
        }
        let __range4 = &*({
            let __elt = (*unwrap_break_err!(metamodelica::index_checked(&adjEnhT.borrow(), varIdx), '__try0)).clone();
            __elt
        });
        for mut entry in __range4 {
            (eqIdx, _, _) = entry.clone();
            if unwrap_break_err!(isEntrySolvable(entry.clone()), '__try0) && eqIdx > 0 {
                if ({
                    let __elt =
                        (*unwrap_break_err!(metamodelica::index_checked(&eqMarker.borrow(), eqIdx), '__try0)).clone();
                    __elt
                }) {
                    {
                        let __cell5 = false;
                        let __idx5 = eqIdx;
                        *unwrap_break_err!(metamodelica::index_mut_checked(&mut eqMarker.clone().borrow_mut(), __idx5), '__try0) =
                            __cell5;
                    }
                    (eqMarker, nE, nV, success) = unwrap_break_err!(pathFound(({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&nV.borrow(), eqIdx), '__try0)).clone(); __elt}), adjEnhT.clone(), varArray.clone(), eqArray.clone(), eqMarker.clone(), nE.clone(), nV.clone()), '__try0);
                }
            }
            if success {
                {
                    let __cell6 = varIdx;
                    let __idx6 = eqIdx;
                    *unwrap_break_err!(metamodelica::index_mut_checked(&mut nV.clone().borrow_mut(), __idx6), '__try0) =
                        __cell6;
                }
                {
                    let __cell7 = eqIdx;
                    let __idx7 = varIdx;
                    *unwrap_break_err!(metamodelica::index_mut_checked(&mut nE.clone().borrow_mut(), __idx7), '__try0) =
                        __cell7;
                }
                return Ok((eqMarker, nE, nV, success));
            }
        }
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function pathFound failed"),
                metamodelica::sourceInfo!("BackEnd/Tearing.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok((eqMarker, nE, nV, success))
}

fn getTearingSetfromAssign(
    mut inDiscreteVars: &metamodelica::List<i32>,
    mut assign1: metamodelica::Array<i32>,
    mut varArray: metamodelica::Array<bool>,
    mut equationArray: metamodelica::Array<bool>,
) -> Result<(
    metamodelica::Array<bool>,
    metamodelica::Array<bool>,
    metamodelica::List<BackendDAE::InnerEquation>,
)> {
    let __ab_assign1 = assign1.borrow();
    let mut varArray: metamodelica::Array<bool> = varArray;
    let mut equationArray: metamodelica::Array<bool> = equationArray;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation> = metamodelica::nil();
    let mut eqIdx: i32;
    match '__try0: {
        for mut varIdx in &**inDiscreteVars {
            unwrap_break_err!(metamodelica::arrayUpdate(varArray.clone(), varIdx.clone(), false), '__try0);
            eqIdx = (*unwrap_break_err!(metamodelica::index_checked(&__ab_assign1, varIdx.clone()), '__try0)).clone();
            unwrap_break_err!(metamodelica::arrayUpdate(equationArray.clone(), eqIdx, false), '__try0);
            innerEquations = metamodelica::cons(
                BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: eqIdx,
                    vars: list![varIdx.clone()],
                },
                innerEquations.clone(),
            );
        }
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function getTearingSetfromAssign failed"),
                metamodelica::sourceInfo!("BackEnd/Tearing.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok((varArray, equationArray, innerEquations))
}

// =============================================================================
//
// Tearing from Book of Cellier
//
// =============================================================================
fn CellierTearing(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut tearingSelect_always: metamodelica::List<i32>,
    mut ojac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut jacType: BackendDAE::JacobianType,
    mut mixedSystem: bool,
    mut strongComponentIndex: i32,
) -> Result<(metamodelica::Ref<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool;
    let mut size: i32;
    let mut tornsize: i32;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut eqnNonlinPoints: metamodelica::Array<i32>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut OutTVars: metamodelica::List<i32>;
    let mut residual: metamodelica::List<i32>;
    let mut residual_coll: metamodelica::List<i32>;
    let mut order: metamodelica::List<i32>;
    let mut unsolvables: metamodelica::List<i32>;
    let mut discreteVars: metamodelica::List<i32>;
    let mut tSel_always: metamodelica::List<i32>;
    let mut tSel_alwaysByUser: metamodelica::List<i32>;
    let mut tSel_prefer: metamodelica::List<i32>;
    let mut tSel_avoid: metamodelica::List<i32>;
    let mut tSel_never: metamodelica::List<i32>;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let mut subsyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut DAEtype: BackendDAE::BackendDAEType;
    let mut DAEtypeStr: ArcStr;
    let mut strictTearingSet: BackendDAE::TearingSet;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut casualTearingSet: Option<BackendDAE::TearingSet>;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut linear: bool;
    let mut b: bool;
    let mut noDynamicStateSelection: bool;
    let mut dynamicTearing: bool;
    let mut dtTarget: bool;
    let mut forceDegree1: bool;
    let mut hasStartCycle: bool;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut startBaseCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut valueCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cyclicBaseCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut meTFull: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut s: ArcStr;
    let mut modelName: ArcStr;
    let debug: bool = false;
    linear = BackendDAEUtil::getLinearfromJacType(jacType)?;
    let __arc1 = &(*isyst);
    let BackendDAE::EQSYSTEM { stateSets: __pa0, .. } = &**__arc1;
    stateSets = metamodelica::Own::own(__pa0);
    noDynamicStateSelection = (stateSets).is_empty();
    let __arc4 = &(*ishared);
    let BackendDAE::SHARED {
        backendDAEType: __pa2,
        info: BackendDAE::EXTRA_INFO {
            fileNamePrefix: __pa3, ..
        },
        ..
    } = &**__arc4;
    DAEtype = metamodelica::Own::own(__pa2);
    modelName = metamodelica::Own::own(__pa3);
    DAEtypeStr = BackendDump::printBackendDAEType2String(DAEtype)?;
    forceDegree1 = BackendDAEUtil::isInitializationDAE(ishared);
    dtTarget = listMember(
        Config::simCodeTarget()?,
        list![literal!("C"), literal!("wasm-jit"), literal!("wasm")],
    );
    dynamicTearing = (::match_deref::match_deref! { match &((Config::dynamicTearing()?, linear, noDynamicStateSelection, DAEtypeStr.clone(), Flags::getConfigBool(Flags::DYNAMIC_TEARING_FOR_INITIALIZATION.clone())?, dtTarget)) {
        (Deref @ "true", _, true, Deref @ "simulation", _, true) => true,
        (Deref @ "true", _, true, Deref @ "initialization", true, true) => true,
        (Deref @ "linear", true, true, Deref @ "simulation", _, true) => true,
        (Deref @ "linear", true, true, Deref @ "initialization", true, true) => true,
        (Deref @ "nonlinear", false, true, Deref @ "simulation", _, true) => true,
        (Deref @ "nonlinear", false, true, Deref @ "initialization", true, true) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of CellierTearing\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    size = ((vindx).len() as i32);
    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst))?;
    eqns = BackendEquation::listEquation(&eqn_lst)?;
    var_lst = List::map1r(
        vindx.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        BackendVariable::daeVars(isyst),
    )?;
    vars = BackendVariable::listVar1(&var_lst)?;
    subsyst = BackendDAEUtil::createEqSystem(
        vars.clone(),
        eqns.clone(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        subsyst,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(ishared),
    )?;
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 1")))?;
    }
    m = Array::map(m.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
    mt = Array::map(mt.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!(
            "\n\n###BEGIN print Strong Component#####################\n(Function:CellierTearing)\n"
        ));
        BackendDump::printEqSystem(subsyst.clone())?;
        metamodelica::print(literal!(
            "\n###END print Strong Component#######################\n(Function:CellierTearing)\n\n\n"
        ));
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nDetermine STRICT TEARING SET\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&subsyst, ishared, false)?;
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 1.5")))?;
    }
    unsolvables = getUnsolvableVars(size, meT.clone())?;
    hasStartCycle = false;
    cyclicBaseCrefs = metamodelica::nil();
    if forceDegree1 {
        startBaseCrefs = metamodelica::nil();
        valueCrefs = metamodelica::nil();
        for mut i in 1..=size {
            cref = BackendVariable::varCref(&(BackendVariable::getVarAt(&vars, i)?));
            if ComponentReference::isStartCref(&cref) {
                startBaseCrefs = metamodelica::cons(ComponentReference::popCref(cref), startBaseCrefs);
            } else {
                valueCrefs = metamodelica::cons(cref, valueCrefs);
            }
        }
        for mut baseCref in &*startBaseCrefs {
            if List::isMemberOnTrue(baseCref.clone(), &valueCrefs, &move |__a0: metamodelica::Ref<
                DAE::ComponentRef,
            >,
                                                                          __a1: metamodelica::Ref<
                DAE::ComponentRef,
            >| {
                ComponentReferenceBasics::crefEqual(&__a0, &__a1)
            })? {
                hasStartCycle = true;
                cyclicBaseCrefs = metamodelica::cons(baseCref.clone(), cyclicBaseCrefs);
            }
        }
    }
    for mut baseCref in &*cyclicBaseCrefs {
        Error::addCompilerWarning({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Initialization start-value cycle: the start attribute of '"));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                metamodelica::AsArg::as_arg(&baseCref),
            )?);
            __mm_s.push_str(&*literal!(
                "' depends, directly or transitively, on the variable itself. The start attribute defines the "
            ));
            __mm_s.push_str(&*literal!("initial value of '"));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                metamodelica::AsArg::as_arg(&baseCref),
            )?);
            __mm_s.push_str(&*literal!("' and therefore "));
            __mm_s.push_str(&*literal!(
                "cannot be computed from it, so this start value cannot be used to initialize the variable (see "
            ));
            __mm_s.push_str(&*literal!(
                "Modelica Specification section 4.9.6). This is possibly a modeling error; if unintended, give '"
            ));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                metamodelica::AsArg::as_arg(&baseCref),
            )?);
            __mm_s.push_str(&*literal!("' a start value that does not depend on "));
            __mm_s.push_str(&*literal!(
                "itself. Use the transformational debugger to inspect the dependency chain."
            ));
            ArcStr::from(__mm_s)
        })?;
    }
    eqnNonlinPoints = arrayCreate(size, -1);
    getEquationNonlinearityPoints(eqnNonlinPoints.clone(), me.clone(), size)?;
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 2")))?;
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\nAdjacencyMatrixEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
        metamodelica::print(literal!("\nAdjacencyMatrixTransposedEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\neqLinPoints:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(eqnNonlinPoints.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("mapEqnIncRow:"));
        BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone())?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(mapIncRowEqn.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(unsolvables.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    discreteVars = findDiscrete(&var_lst);
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nDiscrete Vars:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(discreteVars.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (tSel_always, tSel_prefer, tSel_avoid, tSel_never, tSel_alwaysByUser) =
        tearingSelect(&var_lst, tearingSelect_always, &DAEtypeStr)?;
    if !((tSel_alwaysByUser).is_empty()) {
        Error::addMessage(
            Error::USER_TEARING_VARS.clone(),
            list![
                intString(strongComponentIndex),
                BackendDump::printBackendDAEType2String(DAEtype)?,
                BackendDump::dumpMarkedVarList(&var_lst, &tSel_alwaysByUser)?
            ],
        )?;
    }
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 3")))?;
    }
    ass1 = arrayCreate(size, -1);
    ass2 = arrayCreate(size, -1);
    order = metamodelica::nil();
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 3.1")))?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of CellierTearing2\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    meTFull = if (forceDegree1) {
        metamodelica::arrayFromVec(meT.clone().borrow().clone())
    } else {
        meT.clone()
    };
    (OutTVars, order) = CellierTearing2(
        false,
        m.clone(),
        mt.clone(),
        me.clone(),
        meT.clone(),
        ass1.clone(),
        ass2.clone(),
        unsolvables,
        metamodelica::nil(),
        &discreteVars,
        tSel_always.clone(),
        &tSel_prefer,
        &tSel_avoid,
        &tSel_never,
        order,
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
        eqnNonlinPoints.clone(),
        false,
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of CellierTearing2\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    if forceDegree1 && hasStartCycle && tornMatchingIsStructurallySingular(ass2.clone(), meTFull.clone(), size, &vars)?
    {
        subsyst = BackendDAEUtil::createEqSystem(
            vars.clone(),
            eqns.clone(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
            subsyst,
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            None,
            BackendDAEUtil::isInitializationDAE(ishared),
        )?;
        m = Array::map(m.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
        mt = Array::map(mt.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
        (me, meT, mapEqnIncRow, mapIncRowEqn) =
            BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&subsyst, ishared, false)?;
        unsolvables = getUnsolvableVars(size, meT.clone())?;
        eqnNonlinPoints = arrayCreate(size, -1);
        getEquationNonlinearityPoints(eqnNonlinPoints.clone(), me.clone(), size)?;
        ass1 = arrayCreate(size, -1);
        ass2 = arrayCreate(size, -1);
        order = metamodelica::nil();
        (OutTVars, order) = CellierTearing2(
            false,
            m.clone(),
            mt.clone(),
            me.clone(),
            meT.clone(),
            ass1.clone(),
            ass2.clone(),
            unsolvables,
            metamodelica::nil(),
            &discreteVars,
            tSel_always.clone(),
            &tSel_prefer,
            &tSel_avoid,
            &tSel_never,
            order,
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            eqnNonlinPoints.clone(),
            true,
        )?;
    }
    tornsize = ((OutTVars).len() as i32);
    b = intLt(tornsize, size);
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 3.2")))?;
    }
    residual = getUnassigned(ass2.clone());
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 3.3")))?;
    }
    residual_coll = List::map1r(residual, &arrayGet, mapIncRowEqn.clone())?;
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 3.4")))?;
    }
    residual_coll = List::unique(&residual_coll);
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 3.5")))?;
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        dumpTearingSetLocalIndexes(
            OutTVars.clone(),
            residual_coll.clone(),
            order.clone(),
            ass2.clone(),
            size,
            mapEqnIncRow.clone(),
            &vars,
            eqns.clone(),
            &(literal!(" - STRICT SET")),
        )?;
    }
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 4")))?;
    }
    OutTVars = selectFromList_rev(&vindx, OutTVars)?;
    residual = selectFromList_rev(&eindex, residual_coll)?;
    innerEquations = assignInnerEquations(order, &eindex, &vindx, ass2.clone(), mapEqnIncRow.clone(), None)?;
    if debug {
        execStat(&(literal!("Tearing.CellierTearing -> 5")))?;
    }
    strictTearingSet = BackendDAE::TearingSet {
        tearingvars: OutTVars,
        residualequations: residual,
        innerEquations: innerEquations,
        jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
    };
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        dumpTearingSetGlobalIndexes(strictTearingSet.clone(), size, &(literal!(" - STRICT SET")))?;
    }
    if dynamicTearing {
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\nDetermine CASUAL TEARING SET\n"));
                __mm_s.push_str(&*arcstr::literal!(BORDER));
                __mm_s.push_str(&*arcstr::literal!(BORDER));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        (_, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
            subsyst.clone(),
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            None,
            BackendDAEUtil::isInitializationDAE(ishared),
        )?;
        m = Array::map(m.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
        mt = Array::map(mt.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
        (me, meT, mapEqnIncRow, mapIncRowEqn) =
            BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&subsyst, ishared, true)?;
        unsolvables = getUnsolvableVars(size, meT.clone())?;
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("\nAdjacencyMatrixEnhanced:\n"));
            BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
            metamodelica::print(literal!("\nAdjacencyMatrixTransposedEnhanced:\n"));
            BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\neqLinPoints:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::mapArray(eqnNonlinPoints.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("mapEqnIncRow:"));
            BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone())?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::mapArray(mapIncRowEqn.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(unsolvables.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nDiscrete Vars:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(discreteVars.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        ass1 = arrayCreate(size, -1);
        ass2 = arrayCreate(size, -1);
        order = metamodelica::nil();
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*arcstr::literal!(BORDER));
                __mm_s.push_str(&*literal!("\nBEGINNING of CellierTearing2\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        (OutTVars, order) = CellierTearing2(
            false,
            m.clone(),
            mt.clone(),
            me.clone(),
            meT.clone(),
            ass1.clone(),
            ass2.clone(),
            unsolvables,
            metamodelica::nil(),
            &discreteVars,
            tSel_always,
            &tSel_prefer,
            &tSel_avoid,
            &tSel_never,
            order,
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            eqnNonlinPoints.clone(),
            false,
        )?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nEND of CellierTearing2\n"));
                __mm_s.push_str(&*arcstr::literal!(BORDER));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        if intLt(((OutTVars).len() as i32), tornsize) {
            residual = getUnassigned(ass2.clone());
            residual_coll = List::map1r(residual, &arrayGet, mapIncRowEqn.clone())?;
            residual_coll = List::unique(&residual_coll);
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                dumpTearingSetLocalIndexes(
                    OutTVars.clone(),
                    residual_coll.clone(),
                    order.clone(),
                    ass2.clone(),
                    size,
                    mapEqnIncRow.clone(),
                    &vars,
                    eqns,
                    &(literal!(" - CASUAL SET")),
                )?;
            }
            OutTVars = selectFromList_rev(&vindx, OutTVars)?;
            residual = selectFromList_rev(&eindex, residual_coll)?;
            innerEquations = assignInnerEquations(
                order,
                &eindex,
                &vindx,
                ass2.clone(),
                mapEqnIncRow.clone(),
                Some(me.clone()),
            )?;
            casualTearingSet = Some(BackendDAE::TearingSet {
                tearingvars: OutTVars.clone(),
                residualequations: residual.clone(),
                innerEquations: innerEquations.clone(),
                jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
            });
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                dumpTearingSetGlobalIndexes(
                    BackendDAE::TearingSet {
                        tearingvars: OutTVars,
                        residualequations: residual,
                        innerEquations: innerEquations,
                        jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
                    },
                    size,
                    &(literal!(" - CASUAL SET")),
                )?;
            }
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                if linear {
                    s = literal!("Linear");
                } else {
                    s = literal!("Nonlinear");
                }
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\nNote:\n=====\n"));
                    __mm_s.push_str(&*s);
                    __mm_s.push_str(&*literal!(" dynamic tearing for this strong component in model:\n"));
                    __mm_s.push_str(&*modelName);
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
        } else {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!(
                        "\n* TEARING RESULTS (CASUAL SET):\n*\n* No of equations in strong component: "
                    ));
                    __mm_s.push_str(&*intString(size));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("* No of tVars: "));
                    __mm_s.push_str(&*intString(((OutTVars).len() as i32)));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("*\n* tVars: "));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(OutTVars.reverse(), &fnptr!(intString, i32))?,
                        literal!(","),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("*\n* The casual tearing set is not smaller\n* than the strict tearing set and there-\n* fore it is discarded.\n*"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            if !(b) && !(Flags::getConfigBool(Flags::FORCE_TEARING.clone())?) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    metamodelica::print(literal!(
                        "\nNote:\n=====\nTearing set is discarded because it is not smaller than the original set. Use +forceTearing to prevent this.\n\n"
                    ));
                }
                return Err("fail");
            }
            casualTearingSet = None;
        }
        if debug {
            execStat(&(literal!("Tearing.CellierTearing -> 6")))?;
        }
    } else {
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!(
                "Note:\n=====\nNo dynamic Tearing for this strong component. Check if\n- flag 'dynamicTearing' is set proper\n- strong component does not contain statesets\n- system belongs to simulation\n- SimCode target is 'C'\n\n"
            ));
        }
        if !(b) && !(Flags::getConfigBool(Flags::FORCE_TEARING.clone())?) {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!(
                    "\nNote:\n=====\nTearing set is discarded because it is not smaller than the original set. Use +forceTearing to prevent this.\n\n"
                ));
            }
            return Err("fail");
        }
        casualTearingSet = None;
        if debug {
            execStat(&(literal!("Tearing.CellierTearing -> 7")))?;
        }
    }
    ocomp = metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
        strictTearingSet: strictTearingSet,
        casualTearingSet: casualTearingSet,
        linear: linear,
        mixedSystem: mixedSystem,
    });
    outRunMatching = true;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of CellierTearing\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((ocomp, outRunMatching))
}

fn tearingSelect(
    mut var_lstIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut always: metamodelica::List<i32>,
    mut DAEtypeStr: &ArcStr,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut always: metamodelica::List<i32> = always;
    let mut prefer: metamodelica::List<i32> = metamodelica::nil();
    let mut avoid: metamodelica::List<i32> = metamodelica::nil();
    let mut never: metamodelica::List<i32> = metamodelica::nil();
    let mut alwaysByUser: metamodelica::List<i32> = always.clone();
    let mut var: metamodelica::Ref<BackendDAE::Var> =
        <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
    let mut index: i32 = 1;
    let mut ts: Option<BackendDAE::TearingSelect>;
    let mut preferTVarsWithStartValue: bool;
    let mut inSimulation: bool = metamodelica::stringEq(&DAEtypeStr, &(literal!("simulation")));
    let mut decided: bool;
    preferTVarsWithStartValue = Flags::getConfigBool(Flags::PREFER_TVARS_WITH_START_VALUE.clone())?
        && metamodelica::stringEq(&DAEtypeStr, &(literal!("initialization")));
    for mut var in &**var_lstIn {
        let mut var = var.clone();
        let __arc1 = var.clone();
        let BackendDAE::VAR {
            tearingSelectOption: __pa0,
            ..
        } = &*__arc1;
        ts = metamodelica::Own::own(__pa0);
        decided = (match ts {
            None => false,
            Some(BackendDAE::TearingSelect::ALWAYS { .. }) => {
                if !(listMember(index, always.clone())) {
                    always = metamodelica::cons(index, always);
                    alwaysByUser = metamodelica::cons(index, alwaysByUser);
                }
                true
            }
            Some(BackendDAE::TearingSelect::PREFER { .. }) => {
                prefer = metamodelica::cons(index, prefer);
                true
            }
            Some(BackendDAE::TearingSelect::DEFAULT { .. }) => true,
            Some(BackendDAE::TearingSelect::AVOID { .. }) => {
                avoid = metamodelica::cons(index, avoid);
                true
            }
            Some(BackendDAE::TearingSelect::NEVER { .. }) => {
                never = metamodelica::cons(index, never);
                true
            }
            _ => return Err("match: no arm matched"),
        });
        if !(decided) {
            if Flags::getConfigBool(Flags::TEARING_ALWAYS_DERIVATIVES.clone())?
                && inSimulation
                && BackendVariable::isStateVar(&var)
                && !(listMember(index, always.clone()))
            {
                always = metamodelica::cons(index, always);
            } else if preferTVarsWithStartValue && BackendVariable::varHasStartValue(&var) {
                prefer = metamodelica::cons(index, prefer);
            }
        }
        index = index + 1;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nExternal influence on selection of iteration variables by variable annotations (__OpenModelica_tearingSelect)"));
            __mm_s.push_str(&*if (preferTVarsWithStartValue) {
                literal!(" and preference of variables with start attribute")
            } else {
                literal!("")
            });
            __mm_s.push_str(&*literal!(":\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Always: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(always.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Prefer: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(prefer.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Avoid: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(avoid.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Never: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(never.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((always, prefer, avoid, never, alwaysByUser))
}

pub(crate) fn deleteNegativeEntries(mut rowIn: metamodelica::List<i32>) -> metamodelica::List<i32> {
    let mut rowOut: metamodelica::List<i32>;
    rowOut = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut r in (rowIn).into_iter().cloned() {
            if !(r.clone() > 0) {
                continue;
            }
            let __x = r.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    rowOut
}

fn findDiscrete(mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> metamodelica::List<i32> {
    let mut discreteVarsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut var in &**inVars {
        if BackendVariable::isVarDiscrete(metamodelica::AsArg::as_arg(&var)) {
            discreteVarsOut = metamodelica::cons(index, discreteVarsOut);
        }
        index = index + 1;
    }
    discreteVarsOut
}

fn findDiscreteWarnTearingSelect(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<i32>> {
    let mut discreteVarsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut var in &**inVars {
        if BackendVariable::isVarDiscrete(metamodelica::AsArg::as_arg(&var)) {
            discreteVarsOut = metamodelica::cons(index, discreteVarsOut);
            let () = (match var.tearingSelectOption.clone() {
                Some(BackendDAE::TearingSelect::ALWAYS { .. }) => {
                    Error::addSourceMessage(
                        &(Error::COMPILER_WARNING.clone()),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Minimal Tearing is ignoring '__OpenModelica_tearingSelect = TearingSelect.always' annotation for discrete variable: "));
                            __mm_s.push_str(&*BackendDump::varString(metamodelica::AsArg::as_arg(&var))?);
                            ArcStr::from(__mm_s)
                        }],
                        &(ElementSource::getInfo(var.source.clone())),
                    )?;
                    ()
                }
                Some(BackendDAE::TearingSelect::PREFER { .. }) => {
                    Error::addSourceMessage(
                        &(Error::COMPILER_WARNING.clone()),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Minimal Tearing is ignoring '__OpenModelica_tearingSelect = TearingSelect.prefer' annotation for discrete variable: "));
                            __mm_s.push_str(&*BackendDump::varString(metamodelica::AsArg::as_arg(&var))?);
                            ArcStr::from(__mm_s)
                        }],
                        &(ElementSource::getInfo(var.source.clone())),
                    )?;
                    ()
                }
                _ => (),
            });
        }
        index = index + 1;
    }
    Ok(discreteVarsOut)
}

fn findCSE(mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> metamodelica::List<i32> {
    let mut cseVarsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut var in &**inVars {
        if BackendVariable::isCSEVar(metamodelica::AsArg::as_arg(&var)) {
            cseVarsOut = metamodelica::cons(index, cseVarsOut);
        }
        index = index + 1;
    }
    cseVarsOut
}

fn getEquationNonlinearityPoints(
    mut eqnNonlinPoints: metamodelica::Array<i32>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut size: i32,
) -> Result<metamodelica::Array<i32>> {
    let __ab_me = me.borrow();
    let mut eqnNonlinPoints: metamodelica::Array<i32> = eqnNonlinPoints;
    let mut row: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    let mut sum: i32;
    for mut i in 1..=size {
        row = (*metamodelica::index_checked(&__ab_me, i)?).clone();
        sum = 0;
        for mut entry in &*row {
            sum = sum + nonlinearityWeight(&(entry.clone()));
        }
        {
            let __cell0 = sum;
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut eqnNonlinPoints.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    Ok(eqnNonlinPoints)
}

fn nonlinearityWeight(
    mut entry: &(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
) -> i32 {
    let mut weight: i32;
    weight = (::match_deref::match_deref! { match &(entry) {
        (_, BackendDAE::Solvability::SOLVABILITY_SOLVED { .. }, _) => 0,
        (_, BackendDAE::Solvability::SOLVABILITY_CONSTONE { .. }, _) => 2,
        (_, BackendDAE::Solvability::SOLVABILITY_CONST { .. }, _) => 5,
        (_, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true }, _) => 10,
        (_, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false }, _) => 20,
        (_, BackendDAE::Solvability::SOLVABILITY_LINEAR { b: true }, _) => 20,
        (_, BackendDAE::Solvability::SOLVABILITY_LINEAR { b: false }, _) => 50,
        (_, BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. }, _) => 50,
        (_, BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE { .. }, _) => 100,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    weight
}

fn CellierTearing2<'__b>(
    mut inCausal: bool,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meTIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut Unsolvables: metamodelica::List<i32>,
    mut tvarsIn: metamodelica::List<i32>,
    mut discreteVars: &'__b metamodelica::List<i32>,
    mut tSel_always: metamodelica::List<i32>,
    mut tSel_prefer: &'__b metamodelica::List<i32>,
    mut tSel_avoid: &'__b metamodelica::List<i32>,
    mut tSel_never: &'__b metamodelica::List<i32>,
    mut orderIn: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut eqnNonlinPoints: metamodelica::Array<i32>,
    mut forceDegree1: bool,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut OutTVars: metamodelica::List<i32>;
    let mut orderOut: metamodelica::List<i32>;
    let debug: bool = false;
    if inCausal {
        OutTVars = tvarsIn;
        orderOut = orderIn;
        if debug {
            execStat(&(literal!("Tearing.CellierTearing2 - done")))?;
        }
        return Ok((OutTVars, orderOut));
    }
    (OutTVars, orderOut) = (::match_deref::match_deref! { match &((Unsolvables.clone(), tSel_always.clone())) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut tvar: i32;
            let mut tvars: metamodelica::List<i32>;
            let mut unsolvables: metamodelica::List<i32>;
            let mut order: metamodelica::List<i32>;
            let mut causal: bool;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of selectTearingVar\n\n")); ArcStr::from(__mm_s) });
            }
            tvar = selectTearingVar(meIn.clone(), meTIn.clone(), mIn.clone(), mtIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
            if debug {
                execStat(&(literal!("Tearing.CellierTearing2 - 1.0")))?;
            }
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of selectTearingVar\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            }
            metamodelica::arrayUpdate(ass1In.clone(), tvar, metamodelica::arrayLength(ass1In.clone()) * 2)?;
            deleteEntriesFromAdjacencyMatrix(mIn.clone(), mtIn.clone(), &(list![tvar]))?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\n\n###BEGIN print Adjacency Matrix w/o tvar############\n(Function: CellierTearing2)\n"));
                BackendDump::dumpAdjacencyMatrix(mIn.clone())?;
            }
            Array::replaceAtWithFill(tvar, metamodelica::nil(), metamodelica::nil(), mtIn.clone())?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                BackendDump::dumpAdjacencyMatrixT(mtIn.clone())?;
                metamodelica::print(literal!("\n###END print Adjacency Matrix w/o tvar##############\n(Function: CellierTearing2)\n\n\n"));
            }
            if debug {
                execStat(&(literal!("Tearing.CellierTearing2 - 1.1")))?;
            }
            tvars = metamodelica::cons(tvar, tvarsIn);
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of TarjanMatching\n\n")); ArcStr::from(__mm_s) });
            }
            (order, causal) = TarjanMatching(mIn.clone(), mtIn.clone(), meIn.clone(), meTIn.clone(), ass1In.clone(), ass2In.clone(), orderIn, mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone(), forceDegree1)?;
            if debug {
                execStat(&(literal!("Tearing.CellierTearing2 - 1.2")))?;
            }
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of TarjanMatching\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n* TARJAN RESULTS:\n* ass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1In.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2In.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* order: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            }
            if causal && (Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?) {
                metamodelica::print(literal!("\n"));
                BackendDump::dumpMatching(ass1In.clone())?;
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\norder: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            }
            unsolvables = getUnsolvableVarsConsiderMatching(metamodelica::arrayLength(meTIn.clone()), meTIn.clone(), ass1In.clone(), ass2In.clone())?;
            if debug {
                execStat(&(literal!("Tearing.CellierTearing2 - 1.3")))?;
            }
            (_, unsolvables, _) = List::intersection1OnTrue(unsolvables, tvars.clone(), &fnptr!(intEq, i32, i32))?;
            if debug {
                execStat(&(literal!("Tearing.CellierTearing2 - 1 done")))?;
            }
            (tvars, order) = CellierTearing2(causal, mIn.clone(), mtIn.clone(), meIn.clone(), meTIn.clone(), ass1In.clone(), ass2In.clone(), unsolvables, tvars, discreteVars, tSel_always, tSel_prefer, tSel_avoid, tSel_never, order, mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone(), forceDegree1)?;
            (tvars, order)
        },
        _ => {
            let mut tvars: metamodelica::List<i32>;
            let mut unsolvables: metamodelica::List<i32>;
            let mut tVar_never: metamodelica::List<i32>;
            let mut tVar_discrete: metamodelica::List<i32>;
            let mut order: metamodelica::List<i32>;
            let mut causal: bool;
            tvars = List::unique(&(listAppend(Unsolvables.clone(), tSel_always.clone())));
            tVar_never = List::intersectionOnTrue(tSel_never, &tvars, &fnptr!(intEq, i32, i32))?;
            tVar_discrete = List::intersectionOnTrue(discreteVars, &tvars, &fnptr!(intEq, i32, i32))?;
            if !((tVar_never).is_empty()) {
                Error::addCompilerWarning(literal!("There are tearing variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.never'. Use -d=tearingdump and -d=tearingdumpV for more information."))?;
            }
            if !((tVar_discrete).is_empty()) {
                Error::addCompilerWarning(literal!("There are discrete tearing variables because otherwise the system could not have been torn (unsolvables). This may lead to problems during simulation."))?;
            }
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variables:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\nUnsolvables as tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(Unsolvables, &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variables with annotation attribute 'always' as tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(tSel_always, &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            markTVarsOrResiduals(&tvars, ass1In.clone())?;
            deleteEntriesFromAdjacencyMatrix(mIn.clone(), mtIn.clone(), &tvars)?;
            deleteRowsFromAdjacencyMatrix(mtIn.clone(), &tvars)?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\n\n###BEGIN print Adjacency Matrix w/o tvars###########\n(Function: CellierTearing2)\n"));
                BackendDump::dumpAdjacencyMatrix(mIn.clone())?;
                BackendDump::dumpAdjacencyMatrixT(mtIn.clone())?;
                metamodelica::print(literal!("\n###END print Adjacency Matrix w/o tvars#############\n(Function: CellierTearing2)\n\n\n"));
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of TarjanMatching\n\n")); ArcStr::from(__mm_s) });
            }
            tvars = listAppend(tvars, tvarsIn);
            (order, causal) = TarjanMatching(mIn.clone(), mtIn.clone(), meIn.clone(), meTIn.clone(), ass1In.clone(), ass2In.clone(), orderIn, mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone(), forceDegree1)?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of TarjanMatching\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n* TARJAN RESULTS:\n* ass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1In.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2In.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* order: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            }
            if causal && (Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?) {
                metamodelica::print(literal!("\n"));
                BackendDump::dumpMatching(ass1In.clone())?;
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\norder: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            }
            unsolvables = getUnsolvableVarsConsiderMatching(metamodelica::arrayLength(meTIn.clone()), meTIn.clone(), ass1In.clone(), ass2In.clone())?;
            (_, unsolvables, _) = List::intersection1OnTrue(unsolvables, tvars.clone(), &fnptr!(intEq, i32, i32))?;
            if debug {
                execStat(&(literal!("Tearing.CellierTearing2 - 2")))?;
            }
            (tvars, order) = CellierTearing2(causal, mIn.clone(), mtIn.clone(), meIn.clone(), meTIn.clone(), ass1In.clone(), ass2In.clone(), unsolvables, tvars, discreteVars, metamodelica::nil(), tSel_prefer, tSel_avoid, tSel_never, order, mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone(), forceDegree1)?;
            (tvars, order)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((OutTVars, orderOut))
}

fn selectTearingVar(
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<i32> {
    let mut OutTVar: i32;
    let mut potentials: metamodelica::List<i32>;
    let mut heuristic: ArcStr;
    let mut tearingHeuristic: TearingHeuristic;
    heuristic = Config::getTearingHeuristic()?;
    tearingHeuristic = (::match_deref::match_deref! { match &(heuristic.clone()) {
        Deref @ "MC1" => (std::sync::Arc::new(ModifiedCellierHeuristic_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC2" => (std::sync::Arc::new(ModifiedCellierHeuristic_2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC11" => (std::sync::Arc::new(ModifiedCellierHeuristic_1_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC21" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC12" => (std::sync::Arc::new(ModifiedCellierHeuristic_1_2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC22" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC13" => (std::sync::Arc::new(ModifiedCellierHeuristic_1_3) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC23" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_3) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC231" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_3_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC3" => (std::sync::Arc::new(ModifiedCellierHeuristic_3) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        Deref @ "MC4" => (std::sync::Arc::new(ModifiedCellierHeuristic_4) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> + 'static>),
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown tearing heuristic: ")); __mm_s.push_str(&*heuristic); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Tearing.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of TearingHeuristic\n\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Chosen Heuristic: "));
            __mm_s.push_str(&*heuristic);
            __mm_s.push_str(&*literal!("\n\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    match '__try0: {
        potentials = unwrap_break_err!(tearingHeuristic(m.clone(), mt.clone(), me.clone(), meT.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(potentials.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        OutTVar = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((OutTVar.clone(), potentials.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            OutTVar = __try0_o0;
            potentials = __try0_o1;
        }
        Err(__try0_err) => {
            metamodelica::print(literal!("\nThe selection of a new tearing variable failed.\n"));
            Error::addCompilerWarning(literal!(
                "Function Tearing.selectTearingVar failed at least once. Use -d=tearingdump or -d=tearingdumpV for more information."
            ))?;
            return Err(__try0_err);
        }
    }
    if listMember(OutTVar, tSel_avoid) {
        Error::addCompilerWarning(literal!(
            "The Tearing heuristic has chosen variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.avoid'. Use -d=tearingdump and -d=tearingdumpV for more information."
        ))?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of TearingHeuristic\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(OutTVar)
}

type TearingHeuristic = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<
                metamodelica::List<(
                    i32,
                    BackendDAE::Solvability,
                    metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
                )>,
            >,
            metamodelica::Array<
                metamodelica::List<(
                    i32,
                    BackendDAE::Solvability,
                    metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
                )>,
            >,
            metamodelica::Array<i32>,
            metamodelica::Array<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
        ) -> Result<metamodelica::List<i32>>
        + 'static,
>;

fn ModifiedCellierHeuristic_1(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(&selectedcols1, mIn.clone(), mtIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1, discreteVars, &fnptr!(intEq, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Without Discrete: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Variables from (1st) with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, edges) = selectOneMostCausalizingVar(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n3rd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (2nd) causalizing most equations ["
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut varlst: metamodelica::List<i32>;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst, discreteVars, &fnptr!(intEq, i32, i32))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Non-discrete variables with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, edges) = selectOneMostCausalizingVar(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (1st) causalizing most equations ["
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_1_1(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(&selectedcols1, mIn.clone(), mtIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1, discreteVars, &fnptr!(intEq, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Without Discrete: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Variables from (1st) with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, _) = selectMostCausalizingVars(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n3rd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables from (2nd) causalizing most equations)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, edges) = getOneVarWithMostImpAss(&potentials, ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n4th: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from from (3rd) with most incident impossible assignments ["
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_1(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut varlst: metamodelica::List<i32>;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst, discreteVars, &fnptr!(intEq, i32, i32))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Non-discrete variables with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, _) = selectMostCausalizingVars(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables from (1st) causalizing most equations)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, edges) = getOneVarWithMostImpAss(&potentials, ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n3rd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (2nd) with most incident impossible assignments ["
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_1_2(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(&selectedcols1, mIn.clone(), mtIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1, discreteVars, &fnptr!(intEq, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Without Discrete: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Variables from (1st) with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (selectedcols1, _, _) = getAllVarsWithMostImpAss(&selectedcols1, ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n3rd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Variables from (2nd) with most incident impossible assignments)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, edges) = selectOneMostCausalizingVar(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n4th: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable.One from (3rd) causalizing most equations ["
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_2(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut varlst: metamodelica::List<i32>;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst, discreteVars, &fnptr!(intEq, i32, i32))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Non-discrete variables with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (selectedcols1, _, _) = getAllVarsWithMostImpAss(&selectedcols1, ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Variables from (1st) with most incident impossible assignments)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, edges) = selectOneMostCausalizingVar(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n3rd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (2nd) causalizing most equations ["
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_1_3(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut maxPoints: i32;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    let mut points: metamodelica::List<i32>;
    let mut counts1: metamodelica::List<i32>;
    let mut counts2: metamodelica::List<i32>;
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(&selectedcols1, mIn.clone(), mtIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1, discreteVars, &fnptr!(intEq, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Without Discrete: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Variables from (1st) with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (_, counts1) = selectMostCausalizingVars(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    counts1 = counts1.reverse();
    (_, counts2, _) = getAllVarsWithMostImpAss(&selectedcols1, ass2In.clone(), metIn.clone())?;
    points = List::threadMap(counts1, counts2, &fnptr!(intAdd, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nPoints: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(points.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Sum of impossible assignments and causalizable equations)\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, maxPoints) = getOneVarWithMostPoints(&selectedcols1, points)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n3rd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (2nd) with most points ["
            ));
            __mm_s.push_str(&*intString(maxPoints));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_3(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut maxPoints: i32;
    let mut varlst: metamodelica::List<i32>;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    let mut points: metamodelica::List<i32>;
    let mut counts1: metamodelica::List<i32>;
    let mut counts2: metamodelica::List<i32>;
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst, discreteVars, &fnptr!(intEq, i32, i32))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Non-discrete variables with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (_, counts1) = selectMostCausalizingVars(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    counts1 = counts1.reverse();
    (_, counts2, _) = getAllVarsWithMostImpAss(&selectedcols1, ass2In.clone(), metIn.clone())?;
    points = List::threadMap(counts1, counts2, &fnptr!(intAdd, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nPoints: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(points.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Sum of impossible assignments and causalizable equations)\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials, maxPoints) = getOneVarWithMostPoints(&selectedcols1, points)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (1st) with most points ["
            ));
            __mm_s.push_str(&*intString(maxPoints));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_3_1(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut potpoints1: i32;
    let mut potpoints2: i32;
    let mut varlst: metamodelica::List<i32>;
    let mut selectedcols0: metamodelica::List<i32>;
    let mut selectedcols1: metamodelica::List<i32>;
    let mut selectedrows: metamodelica::List<i32>;
    let mut potentials1: metamodelica::List<i32>;
    let mut potentials2: metamodelica::List<i32>;
    let mut counts1: metamodelica::List<i32>;
    let mut counts2: metamodelica::List<i32>;
    let mut points1: metamodelica::List<i32>;
    let mut points2: metamodelica::List<i32>;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("Start round 1:\n==============\n\n"));
    }
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols0, _) = List::intersection1OnTrue(varlst, discreteVars, &fnptr!(intEq, i32, i32))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), &selectedcols0)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Non-discrete variables with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedrows =
        traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (_, counts1) = selectMostCausalizingVars(
        mtIn.clone(),
        &selectedcols1,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    counts1 = counts1.reverse();
    (_, counts2, _) = getAllVarsWithMostImpAss(&selectedcols1, ass2In.clone(), metIn.clone())?;
    points1 = List::threadMap(counts1, counts2, &fnptr!(intAdd, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nPoints: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(points1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Sum of impossible assignments and causalizable equations)\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    (potentials1, potpoints1) = getOneVarWithMostPoints(&selectedcols1, points1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (1st) with most points ("
            ));
            __mm_s.push_str(&*intString(potpoints1));
            __mm_s.push_str(&*literal!(" points))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedcols1 = findNEntries(mtIn.clone(), &selectedcols0, edges - 1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nStart round 2:\n==============\n\n1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedcols1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables with occurrence in "));
            __mm_s.push_str(&*intString(edges - 1));
            __mm_s.push_str(&*literal!(" equations)\n\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedrows.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more Var)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    if (selectedcols1).is_empty() {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("Second set is empty."));
        }
        potentials = potentials1;
        potpoints2 = 0;
    } else {
        (_, counts1) = selectMostCausalizingVars(
            mtIn.clone(),
            &selectedcols1,
            meIn.clone(),
            ass1In.clone(),
            selectCausalVarsPrepareSelectionSet(&selectedrows, metamodelica::arrayLength(ass1In.clone()))?,
        )?;
        counts1 = counts1.reverse();
        (_, counts2, _) = getAllVarsWithMostImpAss(&selectedcols1, ass2In.clone(), metIn.clone())?;
        points2 = List::threadMap(counts1, counts2, &fnptr!(intAdd, i32, i32))?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nPoints: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(points2.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!(
                    "\n(Sum of impossible assignments and causalizable equations)\n"
                ));
                ArcStr::from(__mm_s)
            });
        }
        (potentials2, potpoints2) = getOneVarWithMostPoints(&selectedcols1, points2)?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n2nd: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(potentials2.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!(
                    "\n(Chosen tearing variable. One from (1st) with most points ("
                ));
                __mm_s.push_str(&*intString(potpoints2));
                __mm_s.push_str(&*literal!(" points))\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        potentials = if (intGe(potpoints1, potpoints2)) {
            potentials1
        } else {
            potentials2
        };
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n=====================\nChosen tearing variable: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n=====================\n(from round 1: "));
            __mm_s.push_str(&*boolString(intGe(potpoints1, potpoints2)));
            __mm_s.push_str(&*literal!(")\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_3(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut maxPoints: i32;
    let mut potentialTVars: metamodelica::List<i32>;
    let mut potentialTVars2: metamodelica::List<i32>;
    let mut bestPotentialTVars: metamodelica::List<i32>;
    let mut causEq: metamodelica::List<i32>;
    let mut points: metamodelica::List<i32>;
    let mut counts1: metamodelica::List<i32>;
    let mut counts2: metamodelica::List<i32>;
    let debug: bool = false;
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC0")))?;
    }
    causEq = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(causEq.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Equations which could be causalized by knowing one more variable)\n\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC1")))?;
    }
    potentialTVars = getUnassigned(ass1In.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentialTVars.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(All unassigned variables)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC2")))?;
    }
    (_, potentialTVars, _) = List::intersection1OnTrue(potentialTVars, tSel_never, &fnptr!(intEq, i32, i32))?;
    if (potentialTVars).is_empty() {
        Error::addCompilerError(literal!(
            "It is not possible to select a new tearing variable, because all remaining variables have the attribute '__OpenModelica_tearingSelect = TearingSelect.never'."
        ))?;
        return Ok(potentials);
    }
    (_, potentialTVars2, _) =
        List::intersection1OnTrue(potentialTVars.clone(), discreteVars, &fnptr!(intEq, i32, i32))?;
    if (potentialTVars2).is_empty() {
        potentialTVars2 = potentialTVars;
        Error::addCompilerWarning(literal!(
            "The tearing heuristic was not able to avoid discrete iteration variables because otherwise the system could not have been torn. This may lead to problems during simulation."
        ))?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("3rd: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(potentialTVars2.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!(
                    "\n(All unassigned variables without attribute 'never' (only discrete variables left))\n\n"
                ));
                ArcStr::from(__mm_s)
            });
        }
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("3rd: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(potentialTVars2.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!(
                    "\n(All non-discrete variables from (2nd) without attribute 'never')\n\n"
                ));
                ArcStr::from(__mm_s)
            });
        }
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC3")))?;
    }
    (potentialTVars, counts1) = selectCausalizingVars(
        mtIn.clone(),
        &potentialTVars2,
        meIn.clone(),
        ass1In.clone(),
        selectCausalVarsPrepareSelectionSet(&causEq, metamodelica::arrayLength(ass1In.clone()))?,
    )?;
    if (potentialTVars).is_empty() {
        potentialTVars = potentialTVars2.clone();
        counts1 = List::fill(0, ((potentialTVars2).len() as i32));
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC4_1")))?;
    }
    (_, counts2, _) = getAllVarsWithMostImpAss(&potentialTVars, ass2In.clone(), metIn.clone())?;
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC4_2")))?;
    }
    points = List::threadMap(counts1, counts2, &fnptr!(intAdd, i32, i32))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n4th (Points): "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(points.clone().reverse(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Sum of impossible assignments and causalizable equations)\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC4_3")))?;
    }
    if !((tSel_prefer).is_empty()) {
        points = preferAvoidVariables(
            &potentialTVars,
            points,
            &tSel_prefer,
            metamodelica::OrderedFloat(3.0_f64),
        );
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("    (Points): "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(points.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!(
                    "\n(Points after preferring variables with attribute 'prefer')\n"
                ));
                ArcStr::from(__mm_s)
            });
        }
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC4_4")))?;
    }
    if !((tSel_avoid).is_empty()) {
        points = preferAvoidVariables(
            &potentialTVars,
            points,
            &tSel_avoid,
            metamodelica::OrderedFloat(0.334_f64),
        );
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("    (Points): "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(points.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!(
                    "\n(Points after discrimination against variables with attribute 'avoid')\n"
                ));
                ArcStr::from(__mm_s)
            });
        }
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC4_5")))?;
    }
    (bestPotentialTVars, maxPoints) = getAllVarsWithMostPoints(&potentialTVars, &points, metamodelica::nil(), -1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n5th: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(bestPotentialTVars.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables from (3rd) with most points ["));
            __mm_s.push_str(&*intString(maxPoints));
            __mm_s.push_str(&*literal!("])\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC5")))?;
    }
    (edges, potentials) = getVarOccurringInMostEquations(mtIn.clone(), &bestPotentialTVars)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("6th: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from (5th) with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    if debug {
        execStat(&(literal!("TEARINGHEURISTIC6")))?;
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_4(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut metIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut discreteVars: metamodelica::List<i32>,
    mut tSel_prefer: metamodelica::List<i32>,
    mut tSel_avoid: metamodelica::List<i32>,
    mut tSel_never: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut potentials: metamodelica::List<i32> = metamodelica::nil();
    let mut edges: i32;
    let mut potentials1: metamodelica::List<i32>;
    let mut potentials2: metamodelica::List<i32>;
    let mut potentials3: metamodelica::List<i32>;
    let mut potentials4: metamodelica::List<i32>;
    let mut potentials5: metamodelica::List<i32>;
    let mut potentials6: metamodelica::List<i32>;
    let mut potentials7: metamodelica::List<i32>;
    let mut potentials8: metamodelica::List<i32>;
    let mut potentials9: metamodelica::List<i32>;
    let mut potentials10: metamodelica::List<i32>;
    let mut selectedvars: metamodelica::List<i32>;
    let mut count: metamodelica::List<i32>;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "Heuristic uses all modified Cellier-Heuristics\n\nHeuristic [MC1]\n"
            ));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials1 = ModifiedCellierHeuristic_1(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC2]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials2 = ModifiedCellierHeuristic_2(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC11]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials3 = ModifiedCellierHeuristic_1_1(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC21]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials4 = ModifiedCellierHeuristic_2_1(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC12]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials5 = ModifiedCellierHeuristic_1_2(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC22]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials6 = ModifiedCellierHeuristic_2_2(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC13]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials7 = ModifiedCellierHeuristic_1_3(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC23]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials8 = ModifiedCellierHeuristic_2_3(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC231]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials9 = ModifiedCellierHeuristic_2_3_1(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars.clone(),
        tSel_prefer.clone(),
        tSel_avoid.clone(),
        tSel_never.clone(),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nHeuristic [MC3]\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    potentials10 = ModifiedCellierHeuristic_3(
        mIn.clone(),
        mtIn.clone(),
        meIn.clone(),
        metIn.clone(),
        ass1In.clone(),
        ass2In.clone(),
        discreteVars,
        tSel_prefer,
        tSel_avoid,
        tSel_never,
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
    )?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\nSynopsis:\n=========\n[MC1]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC2]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials2.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC11]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials3.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC21]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials4.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC12]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials5.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC22]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials6.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC13]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials7.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC23]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials8.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC231]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials9.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[MC3]: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials10.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    selectedvars = listAppend(
        potentials1,
        listAppend(
            potentials2,
            listAppend(
                potentials3,
                listAppend(
                    potentials4,
                    listAppend(
                        potentials5,
                        listAppend(
                            potentials6,
                            listAppend(
                                potentials7,
                                listAppend(potentials8, listAppend(potentials9, potentials10)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1st: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedvars.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(All potentials)\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (count, selectedvars, _) = countMultiples(arrayCreate(1, selectedvars))?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("2nd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(selectedvars.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n(Variables from (1st) occurring in most potential-sets ("));
            __mm_s.push_str(&*stringDelimitList(
                List::map(count, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(" sets))\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    (edges, potentials) = getVarOccurringInMostEquations(mtIn.clone(), &selectedvars)?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("3rd: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(potentials.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(
                "\n(Chosen tearing variable. One from from (2nd) with most occurrence in equations ("
            ));
            __mm_s.push_str(&*intString(edges));
            __mm_s.push_str(&*literal!(" times))\n\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(potentials)
}

fn preferAvoidVariables(
    mut varsIn: &metamodelica::List<i32>,
    mut points: metamodelica::List<i32>,
    mut preferAvoidIn: &metamodelica::List<i32>,
    mut factor: metamodelica::Real,
) -> metamodelica::List<i32> {
    let mut points: metamodelica::List<i32> = points;
    let mut preferAvoidVar: i32 = 0;
    let mut pos: i32;
    for mut preferAvoidVar in &**preferAvoidIn {
        let mut preferAvoidVar = preferAvoidVar.clone();
        if '__try0: {
            pos = unwrap_break_err!(List::position(preferAvoidVar, varsIn), '__try0);
            points = unwrap_break_err!(List::set(points.clone(), pos, (((factor) * (intReal(unwrap_break_err!((points).get(pos), '__try0)))).0.floor() as i32)), '__try0);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    points
}

fn selectCausalVarsPrepareSelectionSet(
    mut selEqs: &metamodelica::List<i32>,
    mut ass1In_size: i32,
) -> Result<metamodelica::Array<bool>> {
    let mut selEqsSetArray: metamodelica::Array<bool>;
    selEqsSetArray = arrayCreate(ass1In_size, false);
    for mut e in &**selEqs {
        metamodelica::arrayUpdate(selEqsSetArray.clone(), e.clone(), true)?;
    }
    Ok(selEqsSetArray)
}

fn selectMostCausalizingVars(
    mut inMt: metamodelica::Array<metamodelica::List<i32>>,
    mut selVars: &metamodelica::List<i32>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut selEqsSetArray: metamodelica::Array<bool>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut cVars: metamodelica::List<i32> = metamodelica::nil();
    let mut counts: metamodelica::List<i32> = metamodelica::nil();
    let mut row: metamodelica::List<i32>;
    let mut size: i32;
    let mut num: i32 = 0;
    for mut var in &**selVars {
        row = metamodelica::arrayGet(inMt.clone(), var.clone())?;
        metamodelica::arrayUpdate(ass1In.clone(), var.clone(), 1)?;
        size = 0;
        for mut i in &*row {
            if metamodelica::arrayGet(selEqsSetArray.clone(), i.clone())? {
                size = sizeOfAssignable(i.clone(), me.clone(), ass1In.clone(), size)?;
            }
        }
        metamodelica::arrayUpdate(ass1In.clone(), var.clone(), -1)?;
        if size < num {
            counts = metamodelica::cons(size, counts);
        } else if size == num {
            cVars = metamodelica::cons(var.clone(), cVars);
            counts = metamodelica::cons(size, counts);
        } else {
            cVars = list![var.clone()];
            num = size;
            counts = metamodelica::cons(size, counts);
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Var "));
                __mm_s.push_str(&*intString(var.clone()));
                __mm_s.push_str(&*literal!(" would causalize "));
                __mm_s.push_str(&*intString(size));
                __mm_s.push_str(&*literal!(" Eqns\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok((cVars, counts))
}

fn selectCausalizingVars(
    mut inMt: metamodelica::Array<metamodelica::List<i32>>,
    mut selVars: &metamodelica::List<i32>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut selEqsSetArray: metamodelica::Array<bool>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut cVars: metamodelica::List<i32> = metamodelica::nil();
    let mut counts: metamodelica::List<i32> = metamodelica::nil();
    let mut row: metamodelica::List<i32>;
    let mut size: i32;
    for mut var in &**selVars {
        row = metamodelica::arrayGet(inMt.clone(), var.clone())?;
        metamodelica::arrayUpdate(ass1In.clone(), var.clone(), 1)?;
        size = 0;
        for mut i in &*row {
            if metamodelica::arrayGet(selEqsSetArray.clone(), i.clone())? {
                size = sizeOfAssignable(i.clone(), me.clone(), ass1In.clone(), size)?;
            }
        }
        metamodelica::arrayUpdate(ass1In.clone(), var.clone(), -1)?;
        if !(size == 0) {
            cVars = metamodelica::cons(var.clone(), cVars);
            counts = metamodelica::cons(size, counts);
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Var "));
                __mm_s.push_str(&*intString(var.clone()));
                __mm_s.push_str(&*literal!(" would causalize "));
                __mm_s.push_str(&*intString(size));
                __mm_s.push_str(&*literal!(" Eqns\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok((cVars, counts))
}

fn selectOneMostCausalizingVar(
    mut inMt: metamodelica::Array<metamodelica::List<i32>>,
    mut selVars: &metamodelica::List<i32>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut selEqsSetArray: metamodelica::Array<bool>,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut cVars: metamodelica::List<i32> = metamodelica::nil();
    let mut outMax: i32 = 0;
    let mut row: metamodelica::List<i32>;
    let mut size: i32;
    for mut var in &**selVars {
        row = metamodelica::arrayGet(inMt.clone(), var.clone())?;
        metamodelica::arrayUpdate(ass1In.clone(), var.clone(), 1)?;
        size = 0;
        for mut i in &*row {
            if metamodelica::arrayGet(selEqsSetArray.clone(), i.clone())? {
                size = sizeOfAssignable(i.clone(), me.clone(), ass1In.clone(), size)?;
            }
        }
        metamodelica::arrayUpdate(ass1In.clone(), var.clone(), -1)?;
        if intGe(size, outMax) {
            cVars = list![var.clone()];
            outMax = size;
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Var "));
                __mm_s.push_str(&*intString(var.clone()));
                __mm_s.push_str(&*literal!(" would causalize "));
                __mm_s.push_str(&*intString(size));
                __mm_s.push_str(&*literal!(" Eqns\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok((cVars, outMax))
}

fn getOneVarWithMostPoints(
    mut inVarList: &metamodelica::List<i32>,
    mut inPointsLst: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut outVarList: metamodelica::List<i32> = metamodelica::nil();
    let mut outMax: i32;
    let mut index: i32 = 1;
    outMax = ({
        let mut __acc: Option<i32> = None;
        for mut i in (inPointsLst.clone()).into_iter().cloned() {
            let __x = i.clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => {
                    if __x > __cur {
                        __x
                    } else {
                        __cur
                    }
                }
            });
        }
        __acc.unwrap_or((-i32::MAX))
    });
    for mut i in &*inPointsLst {
        if i.clone() == outMax {
            outVarList = list![(inVarList).get(index)?];
            return Ok((outVarList, outMax));
        }
        index = index + 1;
    }
    Ok((outVarList, outMax))
}

fn getAllVarsWithMostPoints(
    mut inVarList: &metamodelica::List<i32>,
    mut inPointsLst: &metamodelica::List<i32>,
    mut outVarList: metamodelica::List<i32>,
    mut outMax: i32,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut outVarList: metamodelica::List<i32> = outVarList;
    let mut outMax: i32 = outMax;
    let () = (::match_deref::match_deref! { match (inVarList, inPointsLst) {
        (Deref @ metamodelica::ListNode::Cons { head: v, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: p, tail: Deref @ metamodelica::ListNode::Nil }) => {
            if intGt(p.clone(), outMax) {
                outMax = p.clone();
                outVarList = list![v.clone()];
            } else if intEq(p.clone(), outMax) {
                outVarList = metamodelica::cons(v.clone(), outVarList);
            }
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: v, tail: vrest }, Deref @ metamodelica::ListNode::Cons { head: p, tail: prest }) => {
            if intGt(p.clone(), outMax) {
                outMax = p.clone();
                outVarList = list![v.clone()];
            } else if intEq(p.clone(), outMax) {
                outVarList = metamodelica::cons(v.clone(), outVarList);
            }
            (outVarList, outMax) = getAllVarsWithMostPoints(vrest, prest, outVarList, outMax)?;
            ()
        },
        _ => {
            Error::addCompilerError(literal!("Tearing.getAllVarsWithMostPoints: Finding variables with most points failed."))?;
            return Err("fail");
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVarList, outMax))
}

fn sizeOfAssignable(
    mut Eqn: i32,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1: metamodelica::Array<i32>,
    mut inSize: i32,
) -> Result<i32> {
    let __ab_me = me.borrow();
    let mut outSize: i32;
    let mut vars: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    let mut b: bool;
    vars = List::removeOnTrue(
        ass1.clone(),
        &move |__a0: metamodelica::Array<i32>,
               __a1: (
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )| isAssignedSaveEnhanced(__a0, &__a1),
        (*metamodelica::index_checked(&__ab_me, Eqn)?).clone(),
    )?;
    b = solvableLst(&vars)?;
    outSize = if (b) { inSize + 1 } else { inSize };
    Ok(outSize)
}

fn getAllVarsWithMostImpAss(
    mut inPotentials: &metamodelica::List<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>, i32)> {
    let __ab_meT = meT.borrow();
    let mut outPotentials: metamodelica::List<i32> = metamodelica::nil();
    let mut outCounts: metamodelica::List<i32> = metamodelica::nil();
    let mut outMax: i32 = 0;
    let mut count: i32;
    let mut elem: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    for mut v in &**inPotentials {
        elem = List::removeOnTrue(
            ass2.clone(),
            &move |__a0: metamodelica::Array<i32>,
                   __a1: (
                i32,
                BackendDAE::Solvability,
                metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
            )| isAssignedSaveEnhanced(__a0, &__a1),
            (*metamodelica::index_checked(&__ab_meT, v.clone())?).clone(),
        )?;
        count = countImpossibleAss(&elem)?;
        if count > outMax {
            outPotentials = list![v.clone()];
            outMax = count;
        } else if count == outMax {
            outPotentials = metamodelica::cons(v.clone(), outPotentials);
        }
        outCounts = metamodelica::cons(count, outCounts);
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Var "));
                __mm_s.push_str(&*intString(v.clone()));
                __mm_s.push_str(&*literal!(" has "));
                __mm_s.push_str(&*intString(count));
                __mm_s.push_str(&*literal!(" incident impossible assignments\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    outCounts = outCounts.reverse();
    Ok((outPotentials, outCounts, outMax))
}

fn getOneVarWithMostImpAss(
    mut inPotentials: &metamodelica::List<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<(metamodelica::List<i32>, i32)> {
    let __ab_meT = meT.borrow();
    let mut outPotentials: metamodelica::List<i32> = metamodelica::nil();
    let mut outMax: i32 = -1;
    let mut count: i32;
    let mut elem: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    for mut v in &**inPotentials {
        elem = List::removeOnTrue(
            ass2.clone(),
            &move |__a0: metamodelica::Array<i32>,
                   __a1: (
                i32,
                BackendDAE::Solvability,
                metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
            )| isAssignedSaveEnhanced(__a0, &__a1),
            (*metamodelica::index_checked(&__ab_meT, v.clone())?).clone(),
        )?;
        count = countImpossibleAss(&elem)?;
        if count > outMax {
            outPotentials = list![v.clone()];
            outMax = count;
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Var "));
                __mm_s.push_str(&*intString(v.clone()));
                __mm_s.push_str(&*literal!(" has "));
                __mm_s.push_str(&*intString(count));
                __mm_s.push_str(&*literal!(" incident impossible assignments\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok((outPotentials, outMax))
}

fn countImpossibleAss(
    mut elem: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> Result<i32> {
    let mut outCount: i32 = 0;
    let mut s: BackendDAE::Solvability;
    for mut e in &**elem {
        (_, s, _) = e.clone();
        if !(solvable(s)?) {
            outCount = outCount + 1;
        }
    }
    Ok(outCount)
}

fn TarjanMatching(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meTIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut orderIn: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut eqnNonlinPoints: metamodelica::Array<i32>,
    mut forceDegree1: bool,
) -> Result<(metamodelica::List<i32>, bool)> {
    let mut orderOut: metamodelica::List<i32>;
    let mut causal: bool;
    let mut unassigned: metamodelica::List<i32>;
    let mut order: metamodelica::List<i32> = orderIn;
    let mut assignable: bool = true;
    let debug: bool = false;
    while assignable {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("\nTarjanAssignment:\n"));
        }
        (order, assignable) = TarjanAssignment(
            mIn.clone(),
            mtIn.clone(),
            meIn.clone(),
            meTIn.clone(),
            ass1In.clone(),
            ass2In.clone(),
            order,
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            eqnNonlinPoints.clone(),
            forceDegree1,
        )?;
    }
    if debug {
        execStat(&(literal!("Tearing.TarjanMatching iters done")))?;
    }
    unassigned = getUnassigned(ass1In.clone());
    if (unassigned).is_empty() {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("\ncausal\n"));
        }
        orderOut = order.reverse();
        causal = true;
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("\nnoncausal\n"));
        }
        orderOut = order;
        causal = false;
    }
    if debug {
        execStat(&(literal!("Tearing.TarjanMatching done")))?;
    }
    Ok((orderOut, causal))
}

fn TarjanAssignment(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meTIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut orderIn: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut eqnNonlinPoints: metamodelica::Array<i32>,
    mut forceDegree1: bool,
) -> Result<(metamodelica::List<i32>, bool)> {
    let mut orderOut: metamodelica::List<i32> = orderIn;
    let mut assignable: bool = false;
    let mut eq_coll: i32;
    let mut assEq_coll: metamodelica::List<i32>;
    let mut eqns: metamodelica::List<i32> = metamodelica::nil();
    let mut vars: metamodelica::List<i32> = metamodelica::nil();
    if forceDegree1 {
        if '__try0: {
            (eq_coll, eqns, vars) = unwrap_break_err!(getNextDegree1Var(mtIn.clone(), meTIn.clone(), ass1In.clone(), ass2In.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()), '__try0);
            orderOut = metamodelica::cons(eq_coll, orderOut.clone());
            assignable = true;
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    if !(assignable) {
        assEq_coll = traverseCollectiveEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone())?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("New assEq_coll: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(assEq_coll.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if '__try1: {
            (eq_coll, eqns, vars) = unwrap_break_err!(getNextSolvableEqn(assEq_coll.clone(), mIn.clone(), meIn.clone(), ass1In.clone(), ass2In.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone()), '__try1);
            orderOut = metamodelica::cons(eq_coll, orderOut.clone());
            assignable = true;
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    if assignable {
        makeAssignment(&eqns, &vars, ass1In.clone(), ass2In.clone(), mIn.clone(), mtIn.clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("order: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(orderOut.clone().reverse(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((orderOut, assignable))
}

fn getNextDegree1Var(
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut meTIn: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<(i32, metamodelica::List<i32>, metamodelica::List<i32>)> {
    let __ab_mapEqnIncRow = mapEqnIncRow.borrow();
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut eqCollOut: i32;
    let mut eqnsOut: metamodelica::List<i32>;
    let mut varsOut: metamodelica::List<i32>;
    let mut e: i32;
    let mut eqColl: i32;
    let mut cnt: i32;
    let mut theEqn: i32;
    let mut s: BackendDAE::Solvability;
    for mut v in 1..=metamodelica::arrayLength(meTIn.clone()) {
        if metamodelica::arrayGet(ass1.clone(), v)? == -1
            && ((metamodelica::arrayGet(mtIn.clone(), v)?).len() as i32) > 1
        {
            cnt = 0;
            theEqn = -1;
            let __range0 = &*({
                let __elt = (*metamodelica::index_checked(&meTIn.borrow(), v)?).clone();
                __elt
            });
            for mut entry in __range0 {
                (e, s, _) = entry.clone();
                if metamodelica::arrayGet(ass2.clone(), e)? == -1 && BackendDAEUtil::isSolvable(s, false) {
                    cnt = cnt + 1;
                    theEqn = e;
                    if cnt > 1 {
                        break;
                    }
                }
            }
            if cnt == 1 {
                eqColl = (*metamodelica::index_checked(&__ab_mapIncRowEqn, theEqn)?).clone();
                if ((*metamodelica::index_checked(&__ab_mapEqnIncRow, eqColl)?).len() as i32) == 1 {
                    eqCollOut = eqColl;
                    eqnsOut = list![theEqn];
                    varsOut = list![v];
                    return Ok((eqCollOut, eqnsOut, varsOut));
                }
            }
        }
    }
    return Err("fail");
    Ok((eqCollOut, eqnsOut, varsOut))
}

fn tornMatchingIsStructurallySingular(
    mut ass2: metamodelica::Array<i32>,
    mut meTFull: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut size: i32,
    mut vars: &BackendDAE::Variables,
) -> Result<bool> {
    let mut singular: bool = false;
    let mut ev: i32;
    let mut cnt: i32;
    let mut theEqn: i32;
    let mut owner: i32;
    let mut s: BackendDAE::Solvability;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    for mut v in 1..=size {
        cref = BackendVariable::varCref(&(BackendVariable::getVarAt(vars, v)?));
        if ComponentReference::isStartCref(&cref) {
            cnt = 0;
            theEqn = -1;
            for mut entry in &*metamodelica::arrayGet(meTFull.clone(), v)? {
                (ev, s, _) = entry.clone();
                if BackendDAEUtil::isSolvable(s, false) {
                    cnt = cnt + 1;
                    theEqn = ev;
                    if cnt > 1 {
                        break;
                    }
                }
            }
            if cnt == 1 {
                owner = metamodelica::arrayGet(ass2.clone(), theEqn)?;
                if owner != v && owner != -2 {
                    singular = true;
                    return Ok(singular);
                }
            }
        }
    }
    Ok(singular)
}

fn traverseSingleEqnsforAssignable(
    mut inAss: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_m = m.borrow();
    let __ab_mapEqnIncRow = mapEqnIncRow.borrow();
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut selectedrows: metamodelica::List<i32>;
    let mut eqnColl: i32;
    let mut eqnSize: i32;
    let mut delst: DoubleEnded::MutableList<i32>;
    delst = DoubleEnded::empty(0);
    for mut e in 1..=metamodelica::arrayLength(inAss.clone()) {
        if metamodelica::arrayGet(inAss.clone(), e)? != -1 {
            continue;
        }
        eqnColl = (*metamodelica::index_checked(&__ab_mapIncRowEqn, e)?).clone();
        eqnSize = ((*metamodelica::index_checked(&__ab_mapEqnIncRow, eqnColl)?).len() as i32);
        if ((*metamodelica::index_checked(&__ab_m, e)?).len() as i32) == eqnSize + 1 {
            if eqnSize == 1 {
                DoubleEnded::push_back(delst.clone(), e)?;
            } else {
                DoubleEnded::push_front(delst.clone(), e);
            }
        }
    }
    selectedrows = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
    Ok(selectedrows)
}

fn traverseCollectiveEqnsforAssignable(
    mut inAss: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let __ab_m = m.borrow();
    let mut selectedrows: metamodelica::List<i32>;
    let mut eqnSize: i32;
    let mut e: i32;
    let mut eqnColl: i32 = 0;
    let mut delst: DoubleEnded::MutableList<i32>;
    delst = DoubleEnded::empty(0);
    let __range0 = mapEqnIncRow.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut eqnLst in __range0 {
        eqnColl = eqnColl + 1;
        e = (eqnLst).head().cloned()?;
        if metamodelica::arrayGet(inAss.clone(), e)? != -1 {
            continue;
        }
        eqnSize = ((eqnLst).len() as i32);
        if ((*metamodelica::index_checked(&__ab_m, e)?).len() as i32) == eqnSize {
            if eqnSize == 1 {
                DoubleEnded::push_back(delst.clone(), eqnColl)?;
            } else {
                DoubleEnded::push_front(delst.clone(), eqnColl);
            }
        }
    }
    selectedrows = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
    Ok(selectedrows)
}

fn makeAssignment(
    mut eqns: &metamodelica::List<i32>,
    mut vars: &metamodelica::List<i32>,
    mut ass1In: metamodelica::Array<i32>,
    mut ass2In: metamodelica::Array<i32>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut eq: i32;
    let mut var: i32;
    for mut index in 1..=((eqns).len() as i32) {
        eq = (eqns).get(index)?;
        var = (vars).get(index)?;
        metamodelica::arrayUpdate(ass1In.clone(), var, eq)?;
        metamodelica::arrayUpdate(ass2In.clone(), eq, var)?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("assignment: Eq "));
                __mm_s.push_str(&*intString(eq));
                __mm_s.push_str(&*literal!(" - Var "));
                __mm_s.push_str(&*intString(var));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Array::replaceAtWithFill(eq, metamodelica::nil(), metamodelica::nil(), mIn.clone())?;
        deleteEntriesFromAdjacencyMatrix(mIn.clone(), mtIn.clone(), &(list![var]))?;
        Array::replaceAtWithFill(var, metamodelica::nil(), metamodelica::nil(), mtIn.clone())?;
        deleteEntriesFromAdjacencyMatrix(mtIn.clone(), mIn.clone(), &(list![eq]))?;
    }
    Ok(())
}

fn getNextSolvableEqn(
    mut assEq_coll: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut eqnNonlinPoints: metamodelica::Array<i32>,
) -> Result<(i32, metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut eqOut: i32 = 0;
    let mut eqnsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut varsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut solvable: bool = false;
    let mut eqns: metamodelica::List<i32> = assEq_coll;
    while !((eqns).is_empty()) {
        eqOut = getMostNonlinearEquation(
            eqnNonlinPoints.clone(),
            eqns.clone(),
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
        )?;
        (solvable, eqnsOut, varsOut) =
            eqnSolvableCheck(eqOut, mapEqnIncRow.clone(), ass1.clone(), m.clone(), me.clone())?;
        (eqns, _) = List::deleteMemberOnTrue(eqOut, eqns, &fnptr!(intEq, i32, i32))?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Most nonlinear equation: "));
                __mm_s.push_str(&*intString(eqOut));
                __mm_s.push_str(&*literal!(" - solvable?: "));
                __mm_s.push_str(&*boolString(solvable));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if solvable {
            break;
        } else {
            let __range0 = &*({
                let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), eqOut)?).clone();
                __elt
            });
            for mut eq in __range0 {
                metamodelica::arrayUpdate(ass2.clone(), eq.clone(), -2)?;
            }
        }
    }
    if !(solvable) {
        return Err("fail");
    }
    Ok((eqOut, eqnsOut, varsOut))
}

fn eqnSolvableCheck(
    mut eqn_coll: i32,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<(bool, metamodelica::List<i32>, metamodelica::List<i32>)> {
    let __ab_mapEqnIncRow = mapEqnIncRow.borrow();
    let __ab_me = me.borrow();
    let mut solvable: bool;
    let mut eqns: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut eqn: i32;
    let mut vars_enh: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    eqns = (*metamodelica::index_checked(&__ab_mapEqnIncRow, eqn_coll)?).clone();
    eqn = (eqns).head().cloned()?;
    vars = metamodelica::arrayGet(m.clone(), eqn)?;
    vars_enh = List::removeOnTrue(
        ass1.clone(),
        &move |__a0: metamodelica::Array<i32>,
               __a1: (
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )| isAssignedSaveEnhanced(__a0, &__a1),
        (*metamodelica::index_checked(&__ab_me, eqn)?).clone(),
    )?;
    solvable = solvableLst(&vars_enh)?;
    Ok((solvable, eqns, vars))
}

fn assignInnerEquations(
    mut inEqns: metamodelica::List<i32>,
    mut eindex: &metamodelica::List<i32>,
    mut vindex: &metamodelica::List<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut meOpt: Option<
        metamodelica::Array<
            metamodelica::List<(
                i32,
                BackendDAE::Solvability,
                metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
            )>,
        >,
    >,
) -> Result<metamodelica::List<BackendDAE::InnerEquation>> {
    let __ab_mapEqnIncRow = mapEqnIncRow.borrow();
    let mut outInnerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    outInnerEquations = ({
        let mut __acc: metamodelica::List<BackendDAE::InnerEquation> = metamodelica::nil();
        for mut eqn in (inEqns).into_iter().cloned() {
            let __x = (match (eqn.clone(), meOpt.clone()) {
                (mut eq, None) => {
                    let mut otherEqn: i32;
                    let mut vars: metamodelica::List<i32>;
                    let mut otherVars: metamodelica::List<i32>;
                    vars = List::map1r(
                        (*metamodelica::index_checked(&__ab_mapEqnIncRow, eq)?).clone(),
                        &arrayGet,
                        ass2.clone(),
                    )?;
                    otherEqn = (eindex).get(eq)?;
                    otherVars = selectFromList_rev(vindex, vars.clone())?;
                    BackendDAE::InnerEquation::INNEREQUATION {
                        eqn: otherEqn,
                        vars: otherVars.clone(),
                    }
                }
                (mut eq, Some(mut me)) => {
                    let mut otherEqn: i32;
                    let mut eqns: metamodelica::List<i32>;
                    let mut vars: metamodelica::List<i32>;
                    let mut otherVars: metamodelica::List<i32>;
                    let mut innerEquation: BackendDAE::InnerEquation;
                    let mut constraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>>;
                    eqns = (*metamodelica::index_checked(&__ab_mapEqnIncRow, eq)?).clone();
                    vars = List::map1r(eqns.clone(), &arrayGet, ass2.clone())?;
                    otherEqn = (eindex).get(eq)?;
                    otherVars = selectFromList_rev(vindex, vars.clone())?;
                    constraints = findConstraintForInnerEquation(
                        &({
                            let __elt = (*metamodelica::index_checked(&me.borrow(), (eqns).head().cloned()?)?).clone();
                            __elt
                        }),
                        (vars).head().cloned()?,
                    );
                    if (constraints).is_empty() {
                        innerEquation = BackendDAE::InnerEquation::INNEREQUATION {
                            eqn: otherEqn,
                            vars: otherVars.clone(),
                        };
                    } else {
                        innerEquation = BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS {
                            eqn: otherEqn,
                            vars: otherVars.clone(),
                            cons: constraints.clone(),
                        };
                    }
                    innerEquation.clone()
                }
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outInnerEquations)
}

fn findConstraintForInnerEquation(
    mut meRow: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut searchIndex: i32,
) -> metamodelica::List<metamodelica::Ref<DAE::Constraint>> {
    let mut constraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>> = metamodelica::nil();
    let mut index: i32;
    let mut meElem: (
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ) = (0, BackendDAE::Solvability::SOLVABILITY_CONSTONE, metamodelica::nil());
    let mut cons: metamodelica::List<metamodelica::Ref<DAE::Constraint>>;
    for mut meElem in &**meRow {
        let mut meElem = meElem.clone();
        (index, _, cons) = meElem;
        if intEq(index, searchIndex) {
            constraints = cons;
            break;
        }
    }
    constraints
}

fn markTVarsOrResiduals(
    mut markList: &metamodelica::List<i32>,
    mut assIn: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut assOut: metamodelica::Array<i32> = assIn.clone();
    let mut len: i32;
    len = metamodelica::arrayLength(assIn.clone());
    for mut i in &**markList {
        metamodelica::arrayUpdate(assOut.clone(), i.clone(), len * 2)?;
    }
    Ok(assOut)
}

fn countMultiples(
    mut inArr: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut counter: metamodelica::List<i32>;
    let mut numbers: metamodelica::List<i32>;
    let mut values: metamodelica::List<i32>;
    (counter, numbers, values, _) = Array::fold(
        inArr.clone(),
        &move |__a0: metamodelica::List<i32>,
               __a1: (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            i32,
        )| countMultiples2(__a0, &__a1),
        (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), 1),
    )?;
    Ok((counter, numbers, values))
}

fn countMultiples2(
    mut rowIn: metamodelica::List<i32>,
    mut valIn: &(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    i32,
)> {
    let mut valOut: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        i32,
    );
    let mut counter: metamodelica::List<i32>;
    let mut values: metamodelica::List<i32>;
    let mut row: metamodelica::List<i32>;
    let mut set: metamodelica::List<i32>;
    let mut num: metamodelica::List<i32>;
    let mut val: metamodelica::List<i32>;
    let mut positions: metamodelica::List<i32>;
    let mut numbers: metamodelica::List<i32>;
    let mut indx: i32;
    let mut value: i32;
    let mut number: i32;
    let mut position: i32;
    (counter, _, values, indx) = valIn.clone();
    row = List::removeOnTrue(0, &fnptr!(intEq, i32, i32), rowIn)?;
    set = List::unique(&row);
    if (set).is_empty() {
        val = list![0];
        num = list![0];
    } else {
        (val, num) = countMultiples3(&row, &set, metamodelica::nil(), metamodelica::nil())?;
    }
    positions = maxListInt(num.clone());
    position = (positions).head().cloned()?;
    number = (num).get(position)?;
    numbers = selectFromList(&val, &positions)?;
    value = (val).get(position)?;
    counter = List::set(counter, indx, number)?;
    values = List::set(values, indx, value)?;
    valOut = (counter, numbers, values, indx + 1);
    Ok(valOut)
}

fn countMultiples3<'__b>(
    mut lstIn: &'__b metamodelica::List<i32>,
    mut set: &'__b metamodelica::List<i32>,
    mut valIn: metamodelica::List<i32>,
    mut numIn: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match set {
            Deref @ metamodelica::ListNode::Cons { head: value, tail: rest } => {
                let mut number: i32;
                let mut val: metamodelica::List<i32>;
                let mut num: metamodelica::List<i32>;
                number = ((lstIn).len() as i32) - (((List::removeOnTrue(value.clone(), &fnptr!(intEq, i32, i32), lstIn.clone())?)).len() as i32);
                { (lstIn, set, valIn, numIn) = (lstIn, rest, metamodelica::cons(value.clone(), valIn), metamodelica::cons(number, numIn)); continue '__tco; }
            },
            _ => {
                return Ok((valIn, numIn))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn maxListInt(mut inList: metamodelica::List<i32>) -> metamodelica::List<i32> {
    let mut outList: metamodelica::List<i32> = metamodelica::nil();
    let mut maxi: i32;
    let mut index: i32 = 1;
    maxi = ({
        let mut __acc: Option<i32> = None;
        for mut i in (inList.clone()).into_iter().cloned() {
            let __x = i.clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => {
                    if __x > __cur {
                        __x
                    } else {
                        __cur
                    }
                }
            });
        }
        __acc.unwrap_or((-i32::MAX))
    });
    for mut i in &*inList {
        if i.clone() == maxi {
            outList = metamodelica::cons(index, outList);
        }
        index = index + 1;
    }
    outList
}

fn getMostNonlinearEquation(
    mut inArray: metamodelica::Array<i32>,
    mut inList: metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<i32> {
    let __ab_inArray = inArray.borrow();
    let __ab_mapEqnIncRow = mapEqnIncRow.borrow();
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut index: i32 = 1;
    let mut maxi: i32;
    maxi = ({
        let mut __acc: Option<i32> = None;
        for mut i in (inList.clone()).into_iter().cloned() {
            let __x = (*metamodelica::index_checked(
                &__ab_inArray,
                (*metamodelica::index_checked(&__ab_mapEqnIncRow, i.clone())?)
                    .head()
                    .cloned()?,
            )?)
            .clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => {
                    if __x > __cur {
                        __x
                    } else {
                        __cur
                    }
                }
            });
        }
        __acc.unwrap_or((-i32::MAX))
    });
    for mut i in &*inList {
        index = (*metamodelica::index_checked(&__ab_mapEqnIncRow, i.clone())?)
            .head()
            .cloned()?;
        if (*metamodelica::index_checked(&__ab_inArray, index)?).clone() == maxi {
            index = (*metamodelica::index_checked(&__ab_mapIncRowEqn, index)?).clone();
            return Ok(index);
        }
    }
    Ok(index)
}

fn selectFromList_rev(
    mut inList: &metamodelica::List<i32>,
    mut selList: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outList: metamodelica::List<i32>;
    let mut len: i32;
    len = ((inList).len() as i32);
    outList = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut num in (selList).into_iter().cloned() {
            if !(num.clone() > 0 && num.clone() <= len) {
                continue;
            }
            let __x = (inList).get(num.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

fn selectFromList(
    mut inList: &metamodelica::List<i32>,
    mut selList: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outList: metamodelica::List<i32> = metamodelica::nil();
    let mut num: i32 = 0;
    let mut actual: i32;
    let mut len: i32;
    len = ((inList).len() as i32);
    for mut num in &**selList {
        let mut num = num.clone();
        if num > 0 && num <= len {
            actual = (inList).get(num)?;
            outList = metamodelica::cons(actual, outList);
        }
    }
    Ok(outList)
}

fn deleteEntriesFromAdjacencyMatrix(
    mut mUpdate: metamodelica::Array<metamodelica::List<i32>>,
    mut mHelp: metamodelica::Array<metamodelica::List<i32>>,
    mut entries: &metamodelica::List<i32>,
) -> Result<()> {
    let mut rowIndx: i32 = 0;
    let mut rowsIndx: metamodelica::List<i32>;
    let mut row: metamodelica::List<i32>;
    for mut entry in &**entries {
        rowsIndx = metamodelica::arrayGet(mHelp.clone(), entry.clone())?;
        for mut rowIndx in &*rowsIndx {
            let mut rowIndx = rowIndx.clone();
            row = metamodelica::arrayGet(mUpdate.clone(), rowIndx)?;
            (row, _) = List::deleteMemberOnTrue(entry.clone(), row, &fnptr!(intEq, i32, i32))?;
            Array::replaceAtWithFill(rowIndx, row.clone(), row, mUpdate.clone())?;
        }
    }
    Ok(())
}

fn deleteRowsFromAdjacencyMatrix(
    mut mUpdate: metamodelica::Array<metamodelica::List<i32>>,
    mut rows: &metamodelica::List<i32>,
) -> Result<()> {
    for mut row in &**rows {
        Array::replaceAtWithFill(row.clone(), metamodelica::nil(), metamodelica::nil(), mUpdate.clone())?;
    }
    Ok(())
}

fn getVarsOfEqnsWithMostVars(
    mut inVars: &metamodelica::List<i32>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let __ab_mtIn = mtIn.borrow();
    let mut outVars: metamodelica::List<i32> = metamodelica::nil();
    let mut size: i32;
    let mut maxSize: i32 = 0;
    let mut eqns: metamodelica::List<i32>;
    let mut eqn_size_arr: metamodelica::Array<i32>;
    eqn_size_arr = arrayCreate(metamodelica::arrayLength(mIn.clone()), -1);
    for mut i in 1..=metamodelica::arrayLength(mIn.clone()) {
        size = (({
            let __elt = (*metamodelica::index_checked(&mIn.borrow(), i)?).clone();
            __elt
        })
        .len() as i32);
        {
            let __cell0 = size;
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut eqn_size_arr.clone().borrow_mut(), __idx0)? = __cell0;
        }
        if size > maxSize {
            maxSize = size;
        }
    }
    for mut var in &**inVars {
        eqns = (*metamodelica::index_checked(&__ab_mtIn, var.clone())?).clone();
        for mut e in &*eqns {
            if ({
                let __elt = (*metamodelica::index_checked(&eqn_size_arr.borrow(), e.clone())?).clone();
                __elt
            }) == maxSize
            {
                outVars = metamodelica::cons(var.clone(), outVars);
                break;
            }
        }
    }
    GCExt::free(eqn_size_arr.clone());
    Ok(outVars)
}

fn getVarsOccurringInMostEquations(
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut inSelect: &metamodelica::List<i32>,
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut length: i32 = 0;
    let mut outLst: metamodelica::List<i32> = metamodelica::nil();
    let mut length1: i32;
    let mut row: metamodelica::List<i32>;
    for mut sel in &**inSelect {
        row = metamodelica::arrayGet(mtIn.clone(), sel.clone())?;
        length1 = ((row).len() as i32);
        if intGt(length1, length) {
            length = length1;
            outLst = list![sel.clone()];
        } else if intEq(length1, length) {
            outLst = metamodelica::cons(sel.clone(), outLst);
        }
    }
    Ok((length, outLst))
}

fn getVarOccurringInMostEquations(
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut inSelect: &metamodelica::List<i32>,
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut length: i32 = 0;
    let mut outLst: metamodelica::List<i32> = metamodelica::nil();
    let mut length1: i32;
    let mut row: metamodelica::List<i32>;
    for mut sel in &**inSelect {
        row = metamodelica::arrayGet(mtIn.clone(), sel.clone())?;
        length1 = ((row).len() as i32);
        if intGt(length1, length) {
            length = length1;
            outLst = list![sel.clone()];
        }
    }
    Ok((length, outLst))
}

fn findNEntries(
    mut mtIn: metamodelica::Array<metamodelica::List<i32>>,
    mut inSelect: &metamodelica::List<i32>,
    mut num: i32,
) -> Result<metamodelica::List<i32>> {
    let mut outList: metamodelica::List<i32> = metamodelica::nil();
    let mut length: i32;
    let mut row: metamodelica::List<i32>;
    for mut sel in &**inSelect {
        row = metamodelica::arrayGet(mtIn.clone(), sel.clone())?;
        length = ((row).len() as i32);
        if intEq(num, length) {
            outList = metamodelica::cons(sel.clone(), outList);
        }
    }
    Ok(outList)
}

// =============================================================================
// section for preOptModule >>recursiveTearing<<
//
// inline and repeat tearing
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn recursiveTearing(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut con: bool;
    if Flags::getConfigInt(Flags::RTEARING.clone())? > 0 {
        (outDAE, con) = recursiveTearingMain(inDAE)?;
        while con {
            outDAE = tearingSystem(&outDAE)?;
            (outDAE, con) = recursiveTearingMain(outDAE)?;
        }
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn recursiveTearingMain(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(metamodelica::Ref<BackendDAE::BackendDAE>, bool)> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut update: bool = false;
    let mut systlst_new: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut vars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let mut innerEquation: BackendDAE::InnerEquation =
        <BackendDAE::InnerEquation as ::std::default::Default>::default();
    let mut eqindex: i32;
    let mut vindex: i32;
    let mut residualequations: metamodelica::List<i32>;
    let mut tearingvars: metamodelica::List<i32>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut tear_cr: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tear_cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut all_vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut tear_exp: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqn1: metamodelica::Ref<BackendDAE::Equation>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs1: metamodelica::Ref<DAE::Exp>;
    let mut sumRhs: metamodelica::Ref<DAE::Exp>;
    let mut sumLhs: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut res: metamodelica::Ref<DAE::Exp>;
    let mut n: i32;
    let mut i: i32;
    let mut j: i32 = 0;
    let mut m: i32;
    let mut index: i32 = 1;
    let mut optarr: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut optarr_res: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut indx_res: metamodelica::Array<i32>;
    let mut indx_eq: metamodelica::Array<i32>;
    let mut indx_var: metamodelica::Array<i32>;
    let mut tmp_update: bool;
    let mut isDer: bool;
    let mut mm: metamodelica::Array<metamodelica::List<i32>>;
    let mut maxSizeOne: bool = Flags::getConfigInt(Flags::RTEARING.clone())? == 1;
    let mut loopT: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut noLoopT: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    shared = inDAE.shared.clone();
    let __arc2 = shared.clone();
    let BackendDAE::SHARED {
        functionTree: __pa0,
        globalKnownVars: __pa1,
        ..
    } = &*__arc2;
    funcs = metamodelica::Own::own(__pa0);
    globalKnownVars = metamodelica::Own::own(__pa1);
    for mut syst in &*inDAE.eqs.clone() {
        let (__pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa3, orderedEqs: __pa4, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa5, .. }, stateSets: __pa6, partitionKind: __pa7, .. } => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
            _ => return Err("pattern mismatch"),
        } };
        vars = metamodelica::Own::own(__pa3);
        eqns = metamodelica::Own::own(__pa4);
        comps = metamodelica::Own::own(__pa5);
        stateSets = metamodelica::Own::own(__pa6);
        partitionKind = metamodelica::Own::own(__pa7);
        (_, mm, _) = BackendDAEUtil::getAdjacencyMatrix(
            syst.clone(),
            openmodelica_backend_types::BackendDAE::IndexType::SPARSE,
            Some(funcs.clone()),
            BackendDAEUtil::isInitializationDAE(&shared),
        )?;
        tmp_update = false;
        for mut comp in &*comps {
            if isTornsystem(metamodelica::AsArg::as_arg(&comp), true, false) {
                let (__pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations: __pa9, residualequations: __pa10, tearingvars: __pa11, .. }, .. } => (__pa9.clone(), __pa10.clone(), __pa11.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                innerEquations = metamodelica::Own::own(__pa9);
                residualequations = metamodelica::Own::own(__pa10);
                tearingvars = metamodelica::Own::own(__pa11);
                n = ((innerEquations).len() as i32);
                m = ((residualequations).len() as i32);
                if maxSizeOne && m > 1 {
                    continue;
                }
                indx_res = arrayCreate(m, 0);
                indx_var = arrayCreate(n, 0);
                indx_eq = arrayCreate(n, 0);
                i = 1;
                optarr = arrayCreate(n, None);
                update = true;
                tmp_update = true;
                for mut innerEquation in &*innerEquations {
                    let mut innerEquation = innerEquation.clone();
                    let (__pa12, __pa13) = ::match_deref::match_deref! { match &(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&innerEquation)) {
                        (__pa12, Deref @ metamodelica::ListNode::Cons { head: __pa13, tail: Deref @ metamodelica::ListNode::Nil }, _) => (__pa12.clone(), __pa13.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqindex = metamodelica::Own::own(__pa12);
                    vindex = metamodelica::Own::own(__pa13);
                    let __arc17 = BackendVariable::getVarAt(&vars, vindex)?;
                    let __pa16 = (__arc17).clone();
                    let BackendDAE::VAR { varName: __pa15, .. } = &*__arc17;
                    cr = metamodelica::Own::own(__pa15);
                    var = metamodelica::Own::own(__pa16);
                    all_vars = metamodelica::cons(cr.clone(), all_vars);
                    metamodelica::arrayUpdate(indx_var.clone(), i, vindex)?;
                    eqn = BackendEquation::get(eqns.clone(), eqindex)?;
                    if BackendVariable::isStateVar(&var) {
                        eqn = BackendEquation::solveEquation(
                            eqn,
                            Expression::expDer(Expression::crefExp(cr)?),
                            Some(funcs.clone()),
                        )?;
                    } else {
                        eqn = BackendEquation::solveEquation(eqn, Expression::crefExp(cr)?, Some(funcs.clone()))?;
                    }
                    metamodelica::arrayUpdate(optarr.clone(), i, Some(eqn.clone()))?;
                    eqns = BackendEquation::setAtIndex(eqns, eqindex, eqn.clone())?;
                    metamodelica::arrayUpdate(indx_eq.clone(), i, eqindex)?;
                    i = i + 1;
                    if Flags::isSet(Flags::DUMP_RTEARING.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("INeqn => "));
                            __mm_s.push_str(&*BackendDump::equationString(&eqn)?);
                            __mm_s.push_str(&*literal!("["));
                            __mm_s.push_str(&*intString(i - 1));
                            __mm_s.push_str(&*literal!("]\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                }
                var_lst = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
                    for mut i in (tearingvars).into_iter().cloned() {
                        let __x = BackendVariable::getVarAt(&vars, i)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                tear_cr_lst = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                    for mut vv in (var_lst).into_iter().cloned() {
                        let __x = BackendVariable::varCref(&(vv.clone()));
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                tear_cr = metamodelica::arrayFromVec(tear_cr_lst.clone().into_iter().cloned().collect());
                all_vars = listAppend(tear_cr_lst, all_vars);
                tear_exp = arrayCreate(
                    m,
                    metamodelica::Ref::new(DAE::Exp::RCONST {
                        real: metamodelica::OrderedFloat(0.0_f64),
                    }),
                );
                i = 1;
                let __range18 = tear_cr.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut tcr in __range18 {
                    metamodelica::arrayUpdate(tear_exp.clone(), i, Expression::crefExp(tcr)?)?;
                    i = i + 1;
                }
                optarr_res = arrayCreate(m, None);
                for mut i in 1..=m {
                    let (__pa19, __pa20) = ::match_deref::match_deref! { match &(residualequations) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa19, tail: __pa20 } => (__pa19.clone(), __pa20.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqindex = metamodelica::Own::own(__pa19);
                    residualequations = metamodelica::Own::own(__pa20);
                    metamodelica::arrayUpdate(indx_res.clone(), i, eqindex)?;
                    eqn = BackendEquation::get(eqns.clone(), eqindex)?;
                    if Flags::isSet(Flags::DUMP_RTEARING.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("INres => "));
                            __mm_s.push_str(&*BackendDump::equationString(&eqn)?);
                            __mm_s.push_str(&*literal!("["));
                            __mm_s.push_str(&*intString(i));
                            __mm_s.push_str(&*literal!("]\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    metamodelica::arrayUpdate(optarr_res.clone(), i, Some(eqn))?;
                }
                for mut i in 1..=n {
                    let __pa21 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(optarr.clone(), i)?) {
                        Some(__pa21) => __pa21.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn = metamodelica::Own::own(__pa21);
                    rhs = BackendEquation::getEquationRHS(&eqn)?;
                    lhs = BackendEquation::getEquationLHS(&eqn)?;
                    (cr, isDer) = Expression::expOrDerCref(&lhs)?;
                    for mut j in i + 1..=n {
                        if listMember(
                            metamodelica::arrayGet(indx_var.clone(), i)?,
                            metamodelica::arrayGet(mm.clone(), metamodelica::arrayGet(indx_eq.clone(), j)?)?,
                        ) {
                            let __pa22 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(optarr.clone(), j)?) {
                                Some(__pa22) => __pa22.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            eqn1 = metamodelica::Own::own(__pa22);
                            rhs1 = BackendEquation::getEquationRHS(&eqn1)?;
                            rhs1 = recursiveTearingReplace(rhs1, cr.clone(), rhs.clone(), isDer)?;
                            rhs1 = recursiveTearingCollect(tear_exp.clone(), rhs1)?;
                            (index, vars, eqns, shared, _, e, _, _, _) = BackendDAEOptimize::simplifyLoopExp(
                                index,
                                vars,
                                eqns,
                                shared,
                                &all_vars,
                                rhs1,
                                metamodelica::nil(),
                                metamodelica::nil(),
                                true,
                                true,
                                -1,
                                metamodelica::nil(),
                                &(literal!("RTEARING")),
                                false,
                            )?;
                            eqn1 = BackendEquation::setEquationRHS(eqn1, e)?;
                            metamodelica::arrayUpdate(optarr.clone(), j, Some(eqn1))?;
                        }
                    }
                    for mut j in 1..=m {
                        if listMember(
                            metamodelica::arrayGet(indx_var.clone(), i)?,
                            metamodelica::arrayGet(mm.clone(), metamodelica::arrayGet(indx_res.clone(), j)?)?,
                        ) {
                            let __pa23 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(optarr_res.clone(), j)?) {
                                Some(__pa23) => __pa23.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            eqn1 = metamodelica::Own::own(__pa23);
                            res = BackendDAEOptimize::makeEquationToResidualExp(&eqn1)?;
                            res = recursiveTearingCollect(tear_exp.clone(), res)?;
                            (loopT, noLoopT) = BackendDAEOptimize::simplifyLoops_SplitTerms(&all_vars, res)?;
                            sumRhs = Expression::makeSum1(noLoopT, true)?;
                            sumLhs = Expression::makeSum1(loopT, true)?;
                            sumRhs = recursiveTearingReplace(sumRhs, cr.clone(), rhs.clone(), isDer)?;
                            sumLhs = recursiveTearingReplace(sumLhs, cr.clone(), rhs.clone(), isDer)?;
                            sumRhs = recursiveTearingCollect(tear_exp.clone(), sumRhs)?;
                            sumLhs = recursiveTearingCollect(tear_exp.clone(), sumLhs)?;
                            (sumRhs, _) = ExpressionSimplify::simplify(sumRhs)?;
                            (index, vars, eqns, shared, _, sumRhs, _, _, _) = BackendDAEOptimize::simplifyLoopExp(
                                index,
                                vars,
                                eqns,
                                shared,
                                &all_vars,
                                sumRhs,
                                metamodelica::nil(),
                                metamodelica::nil(),
                                true,
                                true,
                                -1,
                                metamodelica::nil(),
                                &(literal!("RTEARING")),
                                false,
                            )?;
                            eqn1 = BackendEquation::setEquationRHS(eqn1, Expression::negate(sumRhs)?)?;
                            (sumLhs, _) = ExpressionSimplify::simplify(sumLhs)?;
                            (index, vars, eqns, shared, _, sumLhs, _, _, _) = BackendDAEOptimize::simplifyLoopExp(
                                index,
                                vars,
                                eqns,
                                shared,
                                &all_vars,
                                sumLhs,
                                metamodelica::nil(),
                                metamodelica::nil(),
                                true,
                                true,
                                -1,
                                metamodelica::nil(),
                                &(literal!("RTEARING")),
                                false,
                            )?;
                            eqn1 = BackendEquation::setEquationLHS(eqn1, sumLhs)?;
                            metamodelica::arrayUpdate(optarr_res.clone(), j, Some(eqn1))?;
                        }
                    }
                }
                for mut i in 1..=n {
                    eqindex = metamodelica::arrayGet(indx_eq.clone(), i)?;
                    let __pa24 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(optarr.clone(), i)?) {
                        Some(__pa24) => __pa24.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn = metamodelica::Own::own(__pa24);
                    eqns = BackendEquation::setAtIndex(eqns, eqindex, eqn.clone())?;
                    if Flags::isSet(Flags::DUMP_RTEARING.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("OUTeqn => "));
                            __mm_s.push_str(&*BackendDump::equationString(&eqn)?);
                            __mm_s.push_str(&*literal!("["));
                            __mm_s.push_str(&*intString(i - 1));
                            __mm_s.push_str(&*literal!("]\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                }
                for mut i in 1..=m {
                    eqindex = metamodelica::arrayGet(indx_res.clone(), i)?;
                    let __pa25 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(optarr_res.clone(), i)?) {
                        Some(__pa25) => __pa25.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn = metamodelica::Own::own(__pa25);
                    eqns = BackendEquation::setAtIndex(eqns, eqindex, eqn.clone())?;
                    if Flags::isSet(Flags::DUMP_RTEARING.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("OUTres => "));
                            __mm_s.push_str(&*BackendDump::equationString(&eqn)?);
                            __mm_s.push_str(&*literal!("["));
                            __mm_s.push_str(&*intString(i - 1));
                            __mm_s.push_str(&*literal!("]\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                }
                if Flags::isSet(Flags::DUMP_RTEARING.clone())? {
                    metamodelica::print(literal!("****************\n"));
                    for mut i in 1..=m {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("TearVar: "));
                            __mm_s.push_str(&*ExpressionBasics::printExpStr(metamodelica::arrayGet(
                                tear_exp.clone(),
                                i,
                            )?)?);
                            __mm_s.push_str(&*literal!("["));
                            __mm_s.push_str(&*intString(i - 1));
                            __mm_s.push_str(&*literal!("]\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    metamodelica::print(literal!("****************\n"));
                }
            }
        }
        if tmp_update {
            systlst_new = metamodelica::cons(
                BackendDAEUtil::createEqSystem(vars, eqns, stateSets, partitionKind, BackendEquation::emptyEqns()),
                systlst_new,
            );
        } else {
            systlst_new = metamodelica::cons(syst.clone(), systlst_new);
        }
    }
    if update {
        outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: systlst_new,
            shared: shared,
        });
        if '__try26: {
            outDAE = unwrap_break_err!(BackendDAEUtil::transformBackendDAE(&outDAE, Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT)), None, None), '__try26);
            Ok::<(), &'static str>(())
        }.is_err() {
            update = false;
        }
    } else {
        outDAE = inDAE;
    }
    Ok((outDAE, update))
}

fn recursiveTearingCollect(
    mut tear_exp: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut k: i32 = 0;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    (e1, e2) = ExpressionSolve::collectX(inExp, metamodelica::arrayGet(tear_exp.clone(), 1)?, true)?;
    for mut k in 2..=metamodelica::arrayLength(tear_exp.clone()) {
        (lhs, e2) = ExpressionSolve::collectX(e2, metamodelica::arrayGet(tear_exp.clone(), k)?, true)?;
        e1 = Expression::expAdd(e1, lhs)?;
    }
    outExp = Expression::expAdd(e2, e1)?;
    Ok(outExp)
}

fn isTornsystem(
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut getLin: bool,
    mut getNoLin: bool,
) -> bool {
    let mut res: bool;
    res = (match &**comp {
        BackendDAE::StrongComponent::TORNSYSTEM { linear, .. }
            if (linear.clone() == getLin || getNoLin == !(linear.clone())) =>
        {
            true
        }
        _ => false,
    });
    res
}

fn recursiveTearingHelper(
    mut rhs1: metamodelica::Ref<DAE::Exp>,
    mut tear_exp: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut m: i32,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sumRhs: metamodelica::Ref<DAE::Exp> = Expression::makeConstZeroE(rhs1.clone())?;
    let mut k: i32 = 0;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp> = rhs1.clone();
    for mut k in 1..=m {
        (e, rhs) = ExpressionSolve::collectX(rhs, metamodelica::arrayGet(tear_exp.clone(), k)?, true)?;
        sumRhs = Expression::expAdd(e, sumRhs)?;
    }
    sumRhs = Expression::expAdd(rhs, sumRhs)?;
    (sumRhs, _) = ExpressionSimplify::simplify(sumRhs)?;
    Ok(sumRhs)
}

fn recursiveTearingReplace(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSourceExp: metamodelica::Ref<DAE::ComponentRef>,
    mut inTargetExp: metamodelica::Ref<DAE::Exp>,
    mut isDer: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    if isDer {
        res = Expression::crefExp(inSourceExp)?;
        res = Expression::expDer(res);
        (res, _) = Expression::replaceExp(inExp, res, inTargetExp)?;
    } else {
        res = Expression::replaceCrefBottomUp(inExp, inSourceExp, inTargetExp)?;
    }
    Ok(res)
}

fn getUnassigned(mut ass: metamodelica::Array<i32>) -> metamodelica::List<i32> {
    let mut unassigned: metamodelica::List<i32> = metamodelica::nil();
    for mut i in 1..=metamodelica::arrayLength(ass.clone()) {
        if metamodelica::Dangerous::arrayGetNoBoundsChecking(ass.clone(), i) < 0 {
            unassigned = metamodelica::cons(i, unassigned);
        }
    }
    unassigned
}

fn dumpTearingSetLocalIndexes(
    mut tVars: metamodelica::List<i32>,
    mut residuals: metamodelica::List<i32>,
    mut order: metamodelica::List<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut size: i32,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut vars: &BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut setString: &ArcStr,
) -> Result<()> {
    let __ab_mapEqnIncRow = mapEqnIncRow.borrow();
    let mut s: metamodelica::List<ArcStr>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n* TEARING RESULTS"));
        __mm_s.push_str(&*setString);
        __mm_s.push_str(&*literal!(
            ":\n* (Local Indexes)\n*\n* No of equations in strong component: "
        ));
        __mm_s.push_str(&*intString(size));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("* No of tVars: "));
        __mm_s.push_str(&*intString(((tVars).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("*\n* tVars: "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(tVars.clone().reverse(), &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if Flags::isSet(Flags::ITERATION_VARS.clone())? {
        s = ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut tVar in (tVars).into_iter().cloned() {
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("* "));
                    __mm_s.push_str(&*intString(tVar.clone()));
                    __mm_s.push_str(&*literal!(": "));
                    __mm_s.push_str(&*BackendDump::varString(
                        &(BackendVariable::getVarAt(vars, tVar.clone())?),
                    )?);
                    ArcStr::from(__mm_s)
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(s, literal!("\n")));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("*\n* resEq: "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(residuals.clone(), &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if Flags::isSet(Flags::ITERATION_VARS.clone())? && Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        s = ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut eqn in (residuals).into_iter().cloned() {
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("* "));
                    __mm_s.push_str(&*intString(eqn.clone()));
                    __mm_s.push_str(&*literal!(": "));
                    __mm_s.push_str(&*BackendDump::equationString(
                        &(BackendEquation::get(eqns.clone(), eqn.clone())?),
                    )?);
                    ArcStr::from(__mm_s)
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(s, literal!("\n")));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    s = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut e in (order).into_iter().cloned() {
            let __x = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*intString(e.clone()));
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(
                        List::map1r(
                            (*metamodelica::index_checked(&__ab_mapEqnIncRow, e.clone())?).clone(),
                            &arrayGet,
                            ass2.clone(),
                        )?,
                        &fnptr!(intString, i32),
                    )?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("*\n* innerEquations ({eqn,vars}):\n* "));
        __mm_s.push_str(&*stringDelimitList(s, literal!(", ")));
        __mm_s.push_str(&*literal!("\n*\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn dumpTearingSetGlobalIndexes(
    mut tearingSet: BackendDAE::TearingSet,
    mut size: i32,
    mut setString: &ArcStr,
) -> Result<()> {
    let mut tVars: metamodelica::List<i32>;
    let mut residuals: metamodelica::List<i32>;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let BackendDAE::TEARINGSET {
        tearingvars: __pa0,
        residualequations: __pa1,
        innerEquations: __pa2,
        ..
    } = tearingSet;
    tVars = metamodelica::Own::own(__pa0);
    residuals = metamodelica::Own::own(__pa1);
    innerEquations = metamodelica::Own::own(__pa2);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n* TEARING RESULTS"));
        __mm_s.push_str(&*setString);
        __mm_s.push_str(&*literal!(
            ":\n* (Global Indexes)\n*\n* No of equations in strong component: "
        ));
        __mm_s.push_str(&*intString(size));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("* No of tVars: "));
        __mm_s.push_str(&*intString(((tVars).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("*\n* tVars: "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(tVars.reverse(), &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("*\n* resEq: "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(residuals, &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("*\n* innerEquations ({eqn,vars}):\n* "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(innerEquations, &move |__a0: BackendDAE::InnerEquation| {
                BackendDump::innerEquationString(&__a0)
            })?,
            literal!(", "),
        ));
        __mm_s.push_str(&*literal!("\n*\n*"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn dumpTearingSetsGlobalIndexes(
    mut tearingSets: &metamodelica::List<BackendDAE::TearingSet>,
    mut size: i32,
) -> Result<()> {
    for mut tearingSet in &**tearingSets {
        dumpTearingSetGlobalIndexes(tearingSet.clone(), size, &(literal!("")))?;
    }
    Ok(())
}

// =============================================================================
//
// Total Tearing - Determination of All Possible Tearing Sets
// author: ptaeuber FHB 2016
//
// =============================================================================
fn totalTearing(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut ojac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut jacType: BackendDAE::JacobianType,
    mut mixedSystem: bool,
) -> Result<(metamodelica::Ref<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool;
    let mut size: i32;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut tVars: metamodelica::List<i32> = metamodelica::nil();
    let mut order: metamodelica::List<i32>;
    let mut causEq: metamodelica::List<i32>;
    let mut unsolvables: metamodelica::List<i32>;
    let mut discreteVars: metamodelica::List<i32>;
    let mut subsyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mLoop: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut mtLoop: metamodelica::Array<metamodelica::List<i32>>;
    let mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut DAEtype: BackendDAE::BackendDAEType;
    let mut tearingSets: metamodelica::List<BackendDAE::TearingSet>;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut linear: bool;
    let mut modelName: ArcStr;
    let mut powerSet: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchingList: metamodelica::List<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<i32>,
    )>;
    let mut visited: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Array<i32>>>;
    linear = BackendDAEUtil::getLinearfromJacType(jacType)?;
    let __arc2 = &(*ishared);
    let BackendDAE::SHARED {
        backendDAEType: __pa0,
        info: BackendDAE::EXTRA_INFO {
            fileNamePrefix: __pa1, ..
        },
        ..
    } = &**__arc2;
    DAEtype = metamodelica::Own::own(__pa0);
    modelName = metamodelica::Own::own(__pa1);
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of totalTearing\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    size = ((vindx).len() as i32);
    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst))?;
    eqns = BackendEquation::listEquation(&eqn_lst)?;
    var_lst = List::map1r(
        vindx.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        BackendVariable::daeVars(isyst),
    )?;
    vars = BackendVariable::listVar1(&var_lst)?;
    subsyst = BackendDAEUtil::createEqSystem(
        vars,
        eqns,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        subsyst,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(ishared),
    )?;
    m = Array::map(m.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
    mt = Array::map(mt.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!(
            "\n\n###BEGIN print Strong Component#####################\n(Function:totalTearing)\n"
        ));
        BackendDump::printEqSystem(subsyst.clone())?;
        metamodelica::print(literal!(
            "\n###END print Strong Component#######################\n(Function:totalTearing)\n\n\n"
        ));
    }
    (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&subsyst, ishared, false)?;
    unsolvables = getUnsolvableVars(size, meT.clone())?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\nAdjacencyMatrixEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
        metamodelica::print(literal!("\nAdjacencyMatrixTransposedEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\n\nmapEqnIncRow:"));
        BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone())?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(mapIncRowEqn.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(unsolvables, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    discreteVars = findDiscrete(&var_lst);
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nDiscrete Vars:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(discreteVars, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    for mut i in ({
        let __s = Util::intPow(2, size)? - 1;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        powerSet = metamodelica::cons(getPowerSetElement(i), powerSet);
    }
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        BackendDump::dumpListList(powerSet.clone(), &(literal!("Power Set")))?;
    }
    tearingSets = metamodelica::nil();
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\n###BEGIN TO LOOP#####################\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    for mut tVars in &*powerSet {
        let mut tVars = tVars.clone();
        if Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\ntVars:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(tVars.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        ass1 = arrayCreate(size, -1);
        ass2 = arrayCreate(size, -1);
        order = metamodelica::nil();
        mLoop = metamodelica::arrayFromVec(m.clone().borrow().clone());
        mtLoop = metamodelica::arrayFromVec(mt.clone().borrow().clone());
        markTVarsOrResiduals(&tVars, ass1.clone())?;
        deleteEntriesFromAdjacencyMatrix(mLoop.clone(), mtLoop.clone(), &tVars)?;
        deleteRowsFromAdjacencyMatrix(mtLoop.clone(), &tVars)?;
        causEq = traverseCollectiveEqnsforAssignable(ass2.clone(), mLoop.clone(), mapEqnIncRow.clone())?;
        visited = UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(Array::hashIntArray, metamodelica::Array<i32>))
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(Array::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>),
            13,
        );
        matchingList = totalMatching(
            ass1.clone(),
            ass2.clone(),
            order,
            &causEq,
            mLoop.clone(),
            mtLoop.clone(),
            me.clone(),
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            visited,
            metamodelica::nil(),
        )?;
        if Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
            dumpMatchingList(&matchingList)?;
        }
        tearingSets = createTearingSets(
            tVars,
            &matchingList,
            &vindx,
            &eindex,
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            tearingSets,
        )?;
    }
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        dumpTearingSetsGlobalIndexes(&tearingSets, size)?;
    }
    ocomp = metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
        strictTearingSet: (tearingSets).head().cloned()?,
        casualTearingSet: None,
        linear: linear,
        mixedSystem: mixedSystem,
    });
    outRunMatching = true;
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nTotal number of different tearing sets: "));
            __mm_s.push_str(&*intString(((tearingSets).len() as i32)));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of totalTearing\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((ocomp, outRunMatching))
}

fn getPowerSetElement(mut i: i32) -> metamodelica::List<i32> {
    let mut powerSetElement: metamodelica::List<i32> = metamodelica::nil();
    let mut c: i32 = 0;
    let mut e: i32 = i;
    let mut r: i32;
    while !(intEq(e, 0)) {
        c = c + 1;
        r = intMod(e, 2);
        e = intDiv(e, 2);
        if intEq(r, 1) {
            powerSetElement = metamodelica::cons(c, powerSetElement);
        }
    }
    powerSetElement
}

fn totalMatching(
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut orderIn: metamodelica::List<i32>,
    mut causEqIn: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut visited: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Array<i32>>>,
    mut matchingListIn: metamodelica::List<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<i32>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<i32>,
    )>,
> {
    let mut matchingListOut: metamodelica::List<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<i32>,
    )> = matchingListIn;
    let mut order: metamodelica::List<i32>;
    let mut causEq: metamodelica::List<i32>;
    let mut e_exp: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut unassigned: metamodelica::List<i32>;
    let mut ass1Copy: metamodelica::Array<i32>;
    let mut ass2Copy: metamodelica::Array<i32>;
    let mut mCopy: metamodelica::Array<metamodelica::List<i32>>;
    let mut mtCopy: metamodelica::Array<metamodelica::List<i32>>;
    let mut solvable: bool;
    for mut e in &**causEqIn {
        (solvable, e_exp, vars) =
            eqnSolvableCheck(e.clone(), mapEqnIncRow.clone(), ass1.clone(), m.clone(), me.clone())?;
        if !(solvable) {
            continue;
        } else {
            ass1Copy = metamodelica::arrayFromVec(ass1.clone().borrow().clone());
            ass2Copy = metamodelica::arrayFromVec(ass2.clone().borrow().clone());
            mCopy = metamodelica::arrayFromVec(m.clone().borrow().clone());
            mtCopy = metamodelica::arrayFromVec(mt.clone().borrow().clone());
            makeAssignment(
                &e_exp,
                &vars,
                ass1Copy.clone(),
                ass2Copy.clone(),
                mCopy.clone(),
                mtCopy.clone(),
            )?;
            order = metamodelica::cons(e.clone(), orderIn.clone());
            if UnorderedSet::contains(ass1Copy.clone(), visited.clone())? {
                continue;
            }
            UnorderedSet::addNew(ass1Copy.clone(), visited.clone())?;
            causEq = traverseCollectiveEqnsforAssignable(ass2Copy.clone(), mCopy.clone(), mapEqnIncRow.clone())?;
            if (causEq).is_empty() {
                unassigned = getUnassigned(ass1Copy.clone());
                if (unassigned).is_empty() {
                    matchingListOut =
                        metamodelica::cons((ass1Copy.clone(), ass2Copy.clone(), order.reverse()), matchingListOut);
                }
            } else {
                matchingListOut = totalMatching(
                    ass1Copy.clone(),
                    ass2Copy.clone(),
                    order,
                    &causEq,
                    mCopy.clone(),
                    mtCopy.clone(),
                    me.clone(),
                    mapEqnIncRow.clone(),
                    mapIncRowEqn.clone(),
                    visited.clone(),
                    matchingListOut,
                )?;
            }
        }
    }
    Ok(matchingListOut)
}

fn createTearingSets(
    mut tVarsIn: metamodelica::List<i32>,
    mut matchingList: &metamodelica::List<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<i32>,
    )>,
    mut vindx: &metamodelica::List<i32>,
    mut eindex: &metamodelica::List<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut tearingSetsIn: metamodelica::List<BackendDAE::TearingSet>,
) -> Result<metamodelica::List<BackendDAE::TearingSet>> {
    let mut tearingSetsOut: metamodelica::List<BackendDAE::TearingSet> = tearingSetsIn;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut tVars: metamodelica::List<i32>;
    let mut residual: metamodelica::List<i32>;
    let mut residual_coll: metamodelica::List<i32>;
    let mut order: metamodelica::List<i32>;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    for mut matching in &**matchingList {
        (ass1, ass2, order) = matching.clone();
        residual = getUnassigned(ass2.clone());
        residual_coll = List::map1r(residual, &arrayGet, mapIncRowEqn.clone())?;
        residual_coll = List::unique(&residual_coll);
        tVars = selectFromList_rev(vindx, tVarsIn.clone())?;
        residual = selectFromList_rev(eindex, residual_coll.clone())?;
        innerEquations = assignInnerEquations(order, eindex, vindx, ass2.clone(), mapEqnIncRow.clone(), None)?;
        tearingSetsOut = metamodelica::cons(
            BackendDAE::TearingSet {
                tearingvars: tVars,
                residualequations: residual,
                innerEquations: innerEquations,
                jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
            },
            tearingSetsOut,
        );
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nTearing Variables:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(tVarsIn.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Residual Equations:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(residual_coll, &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok(tearingSetsOut)
}

fn dumpMatchingList(
    mut matchingList: &metamodelica::List<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::List<i32>,
    )>,
) -> Result<()> {
    let mut c: i32 = 0;
    let mut order: metamodelica::List<i32>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    metamodelica::print(literal!("\n"));
    for mut matching in &**matchingList {
        c = c + 1;
        (ass1, ass2, order) = matching.clone();
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Matching "));
            __mm_s.push_str(&*intString(c));
            __mm_s.push_str(&*literal!(":\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ass1: "));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(ass1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ass2: "));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(ass2.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("order: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(order, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

// =============================================================================
//
// User-Defined Tearing - Determine the tearing set defined by the user
// author: ptaeuber FHB 2016
//
// =============================================================================
fn userDefinedTearing(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut ojac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut jacType: BackendDAE::JacobianType,
    mut mixedSystem: bool,
    mut userTVars: metamodelica::List<i32>,
    mut userResiduals: metamodelica::List<i32>,
) -> Result<(metamodelica::Ref<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool;
    let mut size: i32;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut tVars: metamodelica::List<i32>;
    let mut residuals: metamodelica::List<i32>;
    let mut order: metamodelica::List<i32>;
    let mut causEq: metamodelica::List<i32>;
    let mut unsolvables: metamodelica::List<i32>;
    let mut discreteVars: metamodelica::List<i32>;
    let mut userResiduals_exp: metamodelica::List<i32>;
    let mut subsyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut DAEtype: BackendDAE::BackendDAEType;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let mut tearingSet: BackendDAE::TearingSet;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut linear: bool;
    let mut modelName: ArcStr;
    linear = BackendDAEUtil::getLinearfromJacType(jacType)?;
    let __arc2 = &(*ishared);
    let BackendDAE::SHARED {
        backendDAEType: __pa0,
        info: BackendDAE::EXTRA_INFO {
            fileNamePrefix: __pa1, ..
        },
        ..
    } = &**__arc2;
    DAEtype = metamodelica::Own::own(__pa0);
    modelName = metamodelica::Own::own(__pa1);
    size = ((vindx).len() as i32);
    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst))?;
    eqns = BackendEquation::listEquation(&eqn_lst)?;
    var_lst = List::map1r(
        vindx.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        BackendVariable::daeVars(isyst),
    )?;
    vars = BackendVariable::listVar1(&var_lst)?;
    subsyst = BackendDAEUtil::createEqSystem(
        vars.clone(),
        eqns.clone(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        subsyst,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(ishared),
    )?;
    m = Array::map(m.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
    mt = Array::map(mt.clone(), &fnptr!(deleteNegativeEntries, metamodelica::List<i32>))?;
    (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&subsyst, ishared, false)?;
    if let Ok(__iflet3) = List::flatten(
        ({
            let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
            for mut i in (userResiduals.clone()).into_iter().cloned() {
                let __x = metamodelica::arrayGet(mapEqnIncRow.clone(), i.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    ) {
        userResiduals_exp = __iflet3;
    } else {
        Error::addMessage(
            Error::USER_DEFINED_TEARING_ERROR.clone(),
            list![literal!("Index out of bounds.")],
        )?;
        return Err("fail");
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\nBEGINNING of userDefinedTearing\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUsers tearing vars: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(userTVars.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUsers residual equations: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(userResiduals.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUsers residual equations expanded: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(userResiduals_exp.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!(
            "\n\n###BEGIN print Strong Component#####################\n(Function:userDefinedTearing)\n"
        ));
        BackendDump::printEqSystem(subsyst)?;
        metamodelica::print(literal!(
            "\n###END print Strong Component#######################\n(Function:userDefinedTearing)\n\n\n"
        ));
    }
    if !(intEq(((userTVars).len() as i32), ((userResiduals_exp).len() as i32))) {
        Error::addMessage(
            Error::USER_DEFINED_TEARING_ERROR.clone(),
            list![literal!(
                "The number of tearing variables and residual equations is not identical."
            )],
        )?;
        return Err("fail");
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\nAdjacencyMatrixEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
        metamodelica::print(literal!("\nAdjacencyMatrixTransposedEnhanced:\n"));
        BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
    }
    unsolvables = getUnsolvableVars(size, meT.clone())?;
    discreteVars = findDiscrete(&var_lst);
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\n\nmapEqnIncRow:"));
        BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone())?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(mapIncRowEqn.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(unsolvables, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nDiscrete Vars:\n"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(discreteVars, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    ass1 = arrayCreate(size, -1);
    ass2 = arrayCreate(size, -1);
    order = metamodelica::nil();
    markTVarsOrResiduals(&userTVars, ass1.clone())?;
    markTVarsOrResiduals(&userResiduals_exp, ass2.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nass1: "));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(ass1.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ass2: "));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(ass2.clone(), &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    deleteEntriesFromAdjacencyMatrix(m.clone(), mt.clone(), &userTVars)?;
    deleteRowsFromAdjacencyMatrix(mt.clone(), &userTVars)?;
    deleteEntriesFromAdjacencyMatrix(mt.clone(), m.clone(), &userResiduals_exp)?;
    deleteRowsFromAdjacencyMatrix(m.clone(), &userResiduals_exp)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print(literal!("\nAdjacency Matrix without tvars and residuals:\n"));
        BackendDump::dumpAdjacencyMatrix(m.clone())?;
        BackendDump::dumpAdjacencyMatrix(mt.clone())?;
    }
    if intEq(((userTVars).len() as i32), countEmptyRows(m.clone()))
        && intEq(((userResiduals_exp).len() as i32), countEmptyRows(mt.clone()))
    {
        causEq = traverseCollectiveEqnsforAssignable(ass2.clone(), m.clone(), mapEqnIncRow.clone())?;
        order = simpleMatching(
            ass1.clone(),
            ass2.clone(),
            order,
            causEq,
            m.clone(),
            mt.clone(),
            me.clone(),
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
        )?;
        tVars = selectFromList_rev(&vindx, userTVars.clone())?;
        residuals = selectFromList_rev(&eindex, userResiduals.clone())?;
        innerEquations =
            assignInnerEquations(order.clone(), &eindex, &vindx, ass2.clone(), mapEqnIncRow.clone(), None)?;
        tearingSet = BackendDAE::TearingSet {
            tearingvars: tVars,
            residualequations: residuals,
            innerEquations: innerEquations,
            jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
        };
        ocomp = metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet: tearingSet.clone(),
            casualTearingSet: None,
            linear: linear,
            mixedSystem: mixedSystem,
        });
        outRunMatching = true;
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            dumpTearingSetLocalIndexes(
                userTVars,
                userResiduals,
                order,
                ass2.clone(),
                size,
                mapEqnIncRow.clone(),
                &vars,
                eqns,
                &(literal!("")),
            )?;
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            dumpTearingSetGlobalIndexes(tearingSet, size, &(literal!("")))?;
        }
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("\nMatching failed, choose different tearing set!\n\n\n"));
        }
        Error::addCompilerError(literal!(
            "There is no possible matching for a user-defined tearing set."
        ))?;
        return Err("fail");
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of userDefinedTearing\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((ocomp, outRunMatching))
}

fn countEmptyRows(mut m: metamodelica::Array<metamodelica::List<i32>>) -> i32 {
    let mut count: i32 = 0;
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        if (row).is_empty() {
            count = count + 1;
        }
    }
    count
}

fn simpleMatching(
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut orderIn: metamodelica::List<i32>,
    mut causEqIn: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut orderOut: metamodelica::List<i32> = orderIn;
    let mut e: i32;
    let mut causEq: metamodelica::List<i32> = causEqIn;
    let mut e_exp: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nStart Matching:\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    while !((causEq).is_empty()) {
        if let Ok((__pa0, __pa1, __pa2)) = getNextSolvableEqn(
            causEq.clone(),
            m.clone(),
            me.clone(),
            ass1.clone(),
            ass2.clone(),
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            ass1.clone(),
        ) {
            e = metamodelica::Own::own(__pa0);
            e_exp = metamodelica::Own::own(__pa1);
            vars = metamodelica::Own::own(__pa2);
        } else {
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                metamodelica::print(literal!("\nMatching failed, choose different tearing set!\n\n\n"));
            }
            Error::addCompilerError(literal!(
                "There is no possible matching for a user-defined tearing set."
            ))?;
            return Err("fail");
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("causEq: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(causEq, &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\nProcess "));
                __mm_s.push_str(&*intString(e));
                __mm_s.push_str(&*literal!(":\ne_exp: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(e_exp.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        makeAssignment(&e_exp, &vars, ass1.clone(), ass2.clone(), m.clone(), mt.clone())?;
        orderOut = metamodelica::cons(e, orderOut);
        causEq = traverseCollectiveEqnsforAssignable(ass2.clone(), m.clone(), mapEqnIncRow.clone())?;
    }
    if (getUnassigned(ass1.clone())).is_empty() {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("\nMatching succeeded!\n"));
        }
        orderOut = orderOut.reverse();
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            metamodelica::print(literal!("\nMatching failed, choose different tearing set!\n\n\n"));
        }
        Error::addCompilerError(literal!(
            "There is no possible matching for a user-defined tearing set."
        ))?;
        return Err("fail");
    }
    Ok(orderOut)
}
