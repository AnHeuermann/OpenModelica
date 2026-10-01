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

use crate::BackendDAEOptimize;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Differentiate;
use crate::DynamicOptimization;
use crate::IndexReduction;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::Coloring;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::Graph;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for postOptModule >>symbolicJacobian<<
//
// Detects the sparse pattern of the ODE system and calculates also the symbolic
// Jacobian if flag "--generateDynamicJacobian=symbolic".
// =============================================================================
// From User Documentation for ida v5.4.0 equation (2.5) aka Alpha
// is the scalar in the system Jacobian, proportional to the inverse of the step
// size used for DAE_Mode symbolic jacobians
pub(crate) const DAE_CJ: &'static str = "$DAE_CJ";

pub(crate) fn symbolicJacobian(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?) {
        Deref @ "none" => inDAE,
        Deref @ "numeric" => detectSparsePatternODE(inDAE)?,
        Deref @ "symbolic" => generateSymbolicJacobianPast(&inDAE)?,
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>calculateStateSetsJacobians<<
//
// =============================================================================
pub(crate) fn calculateStateSetsJacobians(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, &calculateEqSystemStateSetsJacobians)?;
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>calculateStrongComponentJacobians<<
//
// Module for to calculate strong component Jacobian matrices
// =============================================================================
pub(crate) fn calculateStrongComponentJacobians(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> metamodelica::Ref<BackendDAE::BackendDAE> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    match '__try0: {
        outDAE = unwrap_break_err!(BackendDAEUtil::mapEqSystem(&inDAE, &calculateEqSystemJacobians), '__try0);
        Ok::<_, &'static str>((outDAE.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDAE = __try0_o0;
        }
        Err(_) => {
            outDAE = inDAE.clone();
        }
    }
    outDAE
}

// =============================================================================
// section for postOptModule >>constantLinearSystem<<
//
// constant Jacobian matrices. Linear system of equations (A x = b) where
// A and b are constant.
// =============================================================================
pub(crate) fn constantLinearSystem(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, &constantLinearSystem0, (false, 1))?;
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>detectSparsePatternODE<<
//
// Generate sparse pattern
// =============================================================================
fn detectSparsePatternODE(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut DAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut coloredCols: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut sparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut v: BackendDAE::Variables;
    let debug: bool = false;
    match '__try0: {
        if debug {
            unwrap_break_err!(execStat(&(literal!("detectSparsePatternODE -> start "))), '__try0);
        }
        let __arc2 = inBackendDAE.clone();
        let BackendDAE::DAE { eqs: __pa1, .. } = &*__arc2;
        eqs = metamodelica::Own::own(__pa1);
        DAE = unwrap_break_err!(BackendDAEUtil::copyBackendDAE(&inBackendDAE), '__try0);
        if debug {
            unwrap_break_err!(execStat(&(literal!("detectSparsePatternODE -> copy dae "))), '__try0);
        }
        DAE = unwrap_break_err!(BackendDAEOptimize::collapseIndependentContinuousBlocks(&DAE), '__try0);
        if debug {
            unwrap_break_err!(execStat(&(literal!("detectSparsePatternODE -> collapse blocks "))), '__try0);
        }
        DAE = unwrap_break_err!(BackendDAEUtil::transformBackendDAE(&DAE, Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT)), None, None), '__try0);
        if debug {
            unwrap_break_err!(execStat(&(literal!("detectSparsePatternODE -> transform backend dae "))), '__try0);
        }
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(DAE.clone()) {
            Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: __pa3, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        v = metamodelica::Own::own(__pa3);
        shared = metamodelica::Own::own(__pa4);
        states = unwrap_break_err!(BackendVariable::getAllStateVarFromVariables(v.clone()), '__try0);
        if debug {
            unwrap_break_err!(execStat(&(literal!("detectSparsePatternODE -> get all vars "))), '__try0);
        }
        (sparsePattern, coloredCols) =
            unwrap_break_err!(generateSparsePattern(&DAE, states.clone(), states.clone(), false, true), '__try0);
        if debug {
            unwrap_break_err!(execStat(&(literal!("detectSparsePatternODE -> generateSparsePattern "))), '__try0);
        }
        shared = unwrap_break_err!(addBackendDAESharedJacobianSparsePattern(sparsePattern.clone(), coloredCols.clone(), BackendDAE::SymbolicJacobianAIndex.clone(), shared.clone()), '__try0);
        if debug {
            unwrap_break_err!(execStat(&(literal!("detectSparsePatternODE -> addBackendDAESharedJacobianSparsePattern "))), '__try0);
        }
        outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: eqs.clone(),
            shared: shared.clone(),
        });
        Ok::<_, &'static str>((outBackendDAE.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outBackendDAE = __try0_o0;
        }
        Err(_) => {
            Error::addCompilerWarning(literal!(
                "The optimization module detectJacobianSparsePattern failed. This module will be skipped and the transformation process continued."
            ))?;
            outBackendDAE = inBackendDAE.clone();
        }
    }
    Ok(outBackendDAE)
}

// =============================================================================
// section for postOptModule >>symbolicJacobianDAE<<
//
// Generate symbolic jacobian for DAEMode
// =============================================================================
pub(crate) fn symbolicJacobianDAE(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut DAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut coloredCols: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut sparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut nonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut inDepVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut depVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut v: BackendDAE::Variables;
    let mut resVars: BackendDAE::Variables;
    let mut emptyVars: BackendDAE::Variables = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    let mut symjac: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let debug: bool = false;
    match '__try0: {
        if debug {
            unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> start ")); ArcStr::from(__mm_s) })), '__try0);
        }
        let __arc2 = inBackendDAE.clone();
        let BackendDAE::DAE { eqs: __pa1, .. } = &*__arc2;
        eqs = metamodelica::Own::own(__pa1);
        DAE = unwrap_break_err!(BackendDAEUtil::copyBackendDAE(&inBackendDAE), '__try0);
        if debug {
            unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> copy dae ")); ArcStr::from(__mm_s) })), '__try0);
        }
        DAE = unwrap_break_err!(BackendDAEOptimize::collapseIndependentBlocks(&DAE), '__try0);
        if debug {
            unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> collapse blocks ")); ArcStr::from(__mm_s) })), '__try0);
        }
        DAE = unwrap_break_err!(BackendDAEUtil::transformBackendDAE(&DAE, Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT)), None, None), '__try0);
        if debug {
            unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> transform backend dae ")); ArcStr::from(__mm_s) })), '__try0);
        }
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(DAE.clone()) {
            Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: __pa3, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        v = metamodelica::Own::own(__pa3);
        shared = metamodelica::Own::own(__pa4);
        (_, resVars) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(v.clone(), (std::sync::Arc::new(BackendVariable::collectVarKindVarinVariables) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>, BackendDAE::Variables)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>, BackendDAE::Variables))> + 'static>), ((std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isDAEmodeResVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>), emptyVars.clone())), '__try0);
        depVars = unwrap_break_err!(BackendVariable::varList(&resVars), '__try0);
        inDepVars = listAppend(
            shared.daeModeData.stateVars.clone(),
            shared.daeModeData.algStateVars.clone(),
        );
        if debug {
            unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> get all vars ")); ArcStr::from(__mm_s) })), '__try0);
        }
        if metamodelica::stringEq(
            &(unwrap_break_err!(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone()), '__try0)),
            &(literal!("symbolic")),
        ) {
            (symjac, funcs, sparsePattern, coloredCols, nonlinearPattern) = unwrap_break_err!(generateGenericJacobian(&DAE, inDepVars.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &shared.globalKnownVars, resVars.clone(), unwrap_break_err!(BackendVariable::varList(&v), '__try0), literal!("A"), false, true), '__try0);
            if debug {
                unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> generateGenericJacobian ")); ArcStr::from(__mm_s) })), '__try0);
            }
            assign_field!(
                shared.symjacs = unwrap_break_err!(List::set(shared.symjacs.clone(), BackendDAE::SymbolicJacobianAIndex.clone(), (symjac.clone(), sparsePattern.clone(), coloredCols.clone(), nonlinearPattern.clone())), '__try0),
                shared.functionTree = funcs.clone()
            );
            if debug {
                unwrap_break_err!(BackendDump::dumpJacobianString(&(metamodelica::Ref::new(BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: symjac.clone(), sparsePattern: sparsePattern.clone(), coloring: coloredCols.clone(), nonlinearPattern: nonlinearPattern.clone() }))), '__try0);
            }
        } else {
            (sparsePattern, coloredCols) = unwrap_break_err!(generateSparsePattern(&DAE, inDepVars.clone(), depVars.clone(), false, true), '__try0);
            if debug {
                unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> generateSparsePattern ")); ArcStr::from(__mm_s) })), '__try0);
            }
            shared = unwrap_break_err!(addBackendDAESharedJacobianSparsePattern(sparsePattern.clone(), coloredCols.clone(), BackendDAE::SymbolicJacobianAIndex.clone(), shared.clone()), '__try0);
            if debug {
                unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE")); __mm_s.push_str(&*literal!("-> addBackendDAESharedJacobianSparsePattern ")); ArcStr::from(__mm_s) })), '__try0);
            }
        }
        outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: eqs.clone(),
            shared: shared.clone(),
        });
        Ok::<_, &'static str>((outBackendDAE.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outBackendDAE = __try0_o0;
        }
        Err(_) => {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("The optimization module "));
                __mm_s.push_str(&*literal!("SymbolicJacobian.symbolicJacobianDAE"));
                __mm_s.push_str(&*literal!(
                    " failed. This module will be skipped and the transformation process continued."
                ));
                ArcStr::from(__mm_s)
            })?;
            outBackendDAE = inBackendDAE.clone();
        }
    }
    Ok(outBackendDAE)
}

// =============================================================================
// section for postOptModule >>generateSymbolicJacobianPast<<
//
// Symbolic Jacobian subsection
// =============================================================================
fn generateSymbolicJacobianPast(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut symJacA: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut sparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut sparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut nonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT_JACOBIANS.clone())?;
    let __arc2 = &(*inBackendDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    (symJacA, funcs, sparsePattern, sparseColoring, nonlinearPattern) = createSymbolicJacobianforStates(inBackendDAE)?;
    shared = addBackendDAESharedJacobian(symJacA, sparsePattern, sparseColoring, nonlinearPattern, shared);
    functionTree = BackendDAEUtil::getFunctions(&shared);
    functionTree = AvlTreePathFunction::join(
        functionTree,
        &funcs,
        &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
            as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
    )?;
    shared = BackendDAEUtil::setSharedFunctionTree(shared, functionTree);
    outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqs,
        shared: shared,
    });
    System::realtimeTock(ClockIndexes::RT_CLOCK_EXECSTAT_JACOBIANS.clone())?;
    Ok(outBackendDAE)
}

fn createSymbolicJacobianforStates(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
)> {
    let mut outJacobian: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outSparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut outSparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut outNonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut backendDAE2: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut inputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut paramvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut v: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "analytical Jacobians -> start generate system for matrix A time : "
            ));
            __mm_s.push_str(&*realString(clock()));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    backendDAE2 = BackendDAEUtil::copyBackendDAE(inBackendDAE)?;
    backendDAE2 = BackendDAEOptimize::collapseIndependentContinuousBlocks(&backendDAE2)?;
    backendDAE2 = BackendDAEUtil::transformBackendDAE(
        &backendDAE2,
        Some((
            openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION,
            openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT,
        )),
        None,
        None,
    )?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backendDAE2.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: Deref @ BackendDAE::Shared { globalKnownVars: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    v = metamodelica::Own::own(__pa0);
    globalKnownVars = metamodelica::Own::own(__pa1);
    varlst = BackendVariable::varList(&v)?;
    knvarlst = BackendVariable::varList(&globalKnownVars)?;
    states = BackendVariable::getAllStateVarFromVariables(v)?;
    inputvars = List::select(
        knvarlst.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isInput(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    paramvars = List::select(
        knvarlst,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isParam(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "analytical Jacobians -> prepared vars for symbolic matrix A time: "
            ));
            __mm_s.push_str(&*realString(clock()));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
        BackendDump::bltdump(literal!("System to create symbolic jacobian of: "), &backendDAE2)?;
    }
    (
        outJacobian,
        outFunctionTree,
        outSparsePattern,
        outSparseColoring,
        outNonlinearPattern,
    ) = generateGenericJacobian(
        &backendDAE2,
        states.clone(),
        &(BackendVariable::listVar1(&states)?),
        &(BackendVariable::listVar1(&inputvars)?),
        &(BackendVariable::listVar1(&paramvars)?),
        BackendVariable::listVar1(&states)?,
        varlst,
        literal!("A"),
        false,
        false,
    )?;
    Ok((
        outJacobian,
        outFunctionTree,
        outSparsePattern,
        outSparseColoring,
        outNonlinearPattern,
    ))
}

// =============================================================================
// section for postOptModule >>generateSymbolicSensitivities<<
//
// That function generates symbolic sentivities for parameters
// by differentiatiating the states with respect to the parameters
// =============================================================================
pub(crate) fn generateSymbolicSensitivities(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut symJacS: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut sparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut sparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut nonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT_JACOBIANS.clone())?;
    let __arc2 = &(*inBackendDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    (symJacS, funcs, sparsePattern, sparseColoring, nonlinearPattern) =
        createSymbolicJacobianforParameters(inBackendDAE)?;
    shared = addBackendDAESharedJacobian(symJacS, sparsePattern, sparseColoring, nonlinearPattern, shared);
    functionTree = BackendDAEUtil::getFunctions(&shared);
    functionTree = AvlTreePathFunction::join(
        functionTree,
        &funcs,
        &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
            as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
    )?;
    shared = BackendDAEUtil::setSharedFunctionTree(shared, functionTree);
    outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqs,
        shared: shared,
    });
    System::realtimeTock(ClockIndexes::RT_CLOCK_EXECSTAT_JACOBIANS.clone())?;
    Ok(outBackendDAE)
}

fn createSymbolicJacobianforParameters(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
)> {
    let mut outJacobian: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outSparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut outSparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut outNonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut backendDAE2: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut inputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut paramvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut v: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "analytical Jacobians -> start generate system for matrix S time : "
            ));
            __mm_s.push_str(&*realString(clock()));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    backendDAE2 = BackendDAEUtil::copyBackendDAE(inBackendDAE)?;
    backendDAE2 = BackendDAEOptimize::collapseIndependentContinuousBlocks(&backendDAE2)?;
    backendDAE2 = BackendDAEUtil::transformBackendDAE(
        &backendDAE2,
        Some((
            openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION,
            openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT,
        )),
        None,
        None,
    )?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backendDAE2.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: Deref @ BackendDAE::Shared { globalKnownVars: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    v = metamodelica::Own::own(__pa0);
    globalKnownVars = metamodelica::Own::own(__pa1);
    varlst = BackendVariable::varList(&v)?;
    knvarlst = BackendVariable::varList(&globalKnownVars)?;
    states = BackendVariable::getAllStateVarFromVariables(v)?;
    inputvars = List::select(
        knvarlst.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isInput(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    paramvars = List::select(
        knvarlst,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isParam(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "analytical Jacobians -> prepared vars for symbolic matrix S time: "
            ));
            __mm_s.push_str(&*realString(clock()));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
        BackendDump::bltdump(literal!("System to create symbolic jacobian of: "), &backendDAE2)?;
    }
    (
        outJacobian,
        outFunctionTree,
        outSparsePattern,
        outSparseColoring,
        outNonlinearPattern,
    ) = generateGenericJacobian(
        &backendDAE2,
        paramvars,
        &(BackendVariable::listVar1(&states)?),
        &(BackendVariable::listVar1(&inputvars)?),
        &(BackendVariable::listVar1(&states)?),
        BackendVariable::listVar1(&states)?,
        varlst,
        literal!("S"),
        false,
        false,
    )?;
    Ok((
        outJacobian,
        outFunctionTree,
        outSparsePattern,
        outSparseColoring,
        outNonlinearPattern,
    ))
}

// =============================================================================
// section for postOptModule >>generateSymbolicLinearizationPast<<
//
// =============================================================================
pub(crate) fn generateSymbolicLinearizationPast(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> metamodelica::Ref<BackendDAE::BackendDAE> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    outBackendDAE = 'mc: {
        let __mc_input = &*inBackendDAE;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut linearModelMatrices: metamodelica::List<(Option<(metamodelica::Ref<BackendDAE::BackendDAE>, ArcStr, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>), i32), metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>), i32))>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE> = outBackendDAE.clone();
                    let true = (Flags::getConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone())?) else { return Err("pattern mismatch") };
                    let __arc2 = inBackendDAE.clone();
                    let BackendDAE::DAE { eqs: __pa0, shared: __pa1 } = &*__arc2;
                    eqs = metamodelica::Own::own(__pa0);
                    shared = metamodelica::Own::own(__pa1);
                    (linearModelMatrices, funcs) = createLinearModelMatrices(inBackendDAE.clone(), Config::acceptOptimicaGrammar()?)?;
                    shared = BackendDAEUtil::setSharedSymJacs(shared.clone(), linearModelMatrices.clone());
                    functionTree = BackendDAEUtil::getFunctions(&shared);
                    functionTree = AvlTreePathFunction::join(functionTree.clone(), &funcs, &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?;
                    shared = BackendDAEUtil::setSharedFunctionTree(shared.clone(), functionTree.clone());
                    outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: eqs.clone(), shared: shared.clone() });
                    Ok((outBackendDAE.clone(), outBackendDAE.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outBackendDAE = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inBackendDAE.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBackendDAE
}

// =============================================================================
// section for postOptModule >>inputDerivativesUsed<<
//
// check for derivatives of inputs
// =============================================================================
pub(crate) fn inputDerivativesUsed(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, &inputDerivativesUsedWork, false)?;
    Ok(outDAE)
}

fn inputDerivativesUsedWork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool;
    let mut hasFailed: bool = false;
    (osyst, outChanged) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut s: ArcStr;
                    let mut hasFailed: bool = hasFailed.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendDAEUtil::traverseBackendDAEExpsEqns(orderedEqs.clone(), (std::sync::Arc::new(traverserinputDerivativesUsed) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>)) -> Result<(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>))> + 'static>), (BackendVariable::daeGlobalKnownVars(&inShared), metamodelica::nil()))?) {
                        (_, __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    explst = metamodelica::Own::own(__pa0);
                    s = stringDelimitList(List::map(explst.clone(), &ExpressionBasics::printExpStr)?, literal!("\n"));
                    Error::addMessage(Error::DERIVATIVE_INPUT.clone(), list![s.clone()])?;
                    hasFailed = true;
                    Ok(((BackendDAEUtil::setEqSystEqs(isyst.clone(), orderedEqs.clone()), true), hasFailed.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            hasFailed = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inChanged))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    if hasFailed {
        return Err("fail");
    }
    Ok((osyst, outShared, outChanged))
}

fn traverserinputDerivativesUsed(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut itpl: (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>),
)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>);
    (e, tpl) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(
            traverserExpinputDerivativesUsed,
            metamodelica::Ref<DAE::Exp>,
            (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>)
        ),
        itpl,
    )?;
    Ok((e, tpl))
}

fn traverserExpinputDerivativesUsed(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::Exp>>);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), &tpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (vars, explst)) => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?;
                    let true = (BackendVariable::isVarOnTopLevelAndInput(&var)) else { return Err("pattern mismatch") };
                    Ok((e.clone(), false, (vars.clone(), metamodelica::cons(e.clone(), explst.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (vars, explst)) => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?;
                    let true = (BackendVariable::isVarOnTopLevelAndInput(&var)) else { return Err("pattern mismatch") };
                    Ok((e.clone(), false, (vars.clone(), metamodelica::cons(e.clone(), explst.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

// =============================================================================
// solve linear systems with constant jacobian and variable b-Vector
//
// =============================================================================
fn jacobianIsConstant(
    mut jac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<bool> {
    let mut isConst: bool;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    eqs = List::map(jac, &fnptr!(Util::tuple33, _))?;
    isConst = !(List::any(
        &eqs,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(variableResidual(&__a0))
        },
    )?);
    Ok(isConst)
}

fn variableResidual(mut eq: &metamodelica::Ref<BackendDAE::Equation>) -> bool {
    let mut isNotConst: bool;
    isNotConst = (::match_deref::match_deref! { match eq {
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: Deref @ DAE::Exp::RCONST { real: _ }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNotConst
}

fn replaceStrongComponent(
    mut systIn: metamodelica::Ref<BackendDAE::EqSystem>,
    mut idx: i32,
    mut compsNew: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut compsAdd: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut systOut: metamodelica::Ref<BackendDAE::EqSystem> = systIn.clone();
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut assAdd: metamodelica::Array<i32>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(systIn) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ass1 = metamodelica::Own::own(__pa0);
    ass2 = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    if !((compsAdd).is_empty()) {
        assAdd = arrayCreate(((compsAdd).len() as i32), 0);
        ass1 = metamodelica::arrayAppend(ass1.clone(), assAdd.clone());
        ass2 = metamodelica::arrayAppend(ass2.clone(), assAdd.clone());
        List::map2_0(
            &compsAdd,
            &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
                   __a1: metamodelica::Array<i32>,
                   __a2: metamodelica::Array<i32>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(updateAssignment(&__a0, __a1, __a2))
            },
            ass1.clone(),
            ass2.clone(),
        )?;
    }
    List::map2_0(
        &compsNew,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
               __a1: metamodelica::Array<i32>,
               __a2: metamodelica::Array<i32>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(updateAssignment(&__a0, __a1, __a2)) },
        ass1.clone(),
        ass2.clone(),
    )?;
    comps = List::replaceAtWithList(compsNew, idx, comps)?;
    assign_field!(
        systOut.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
            ass1: ass1.clone(),
            ass2: ass2.clone(),
            comps: listAppend(comps, compsAdd)
        })
    );
    systOut = BackendDAEUtil::setEqSystMatrices(systOut, None, None, None);
    Ok(systOut)
}

fn updateAssignment(
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> () {
    let () = 'mc: {
        let __mc_input = &**comp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eq, var } => {
                    metamodelica::arrayUpdate(ass2.clone(), eq.clone(), var.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), var.clone(), eq.clone())?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn solveConstJacLinearSystem(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut eqn_lst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut eqn_indxs: &metamodelica::List<i32>,
    mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut var_indxs: &metamodelica::List<i32>,
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut sysIdxIn: i32,
    mut compIdxIn: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::Array<i32>,
    i32,
)> {
    let mut sysEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut bEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut bVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut orderOut: metamodelica::Array<i32>;
    let mut sysIdxOut: i32;
    let mut vars: BackendDAE::Variables;
    let mut v: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqns1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut beqs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut sources: metamodelica::List<metamodelica::Ref<DAE::ElementSource>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut A: metamodelica::Array<metamodelica::Array<metamodelica::Real>>;
    let mut b: metamodelica::Array<metamodelica::Real>;
    let mut row: i32 = 0;
    let mut n: i32;
    let mut order: metamodelica::Array<i32>;
    let __arc5 = &(*syst);
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        matching: __pa2,
        stateSets: __pa3,
        partitionKind: __pa4,
        ..
    } = &**__arc5;
    vars = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    matching = metamodelica::Own::own(__pa2);
    stateSets = metamodelica::Own::own(__pa3);
    partitionKind = metamodelica::Own::own(__pa4);
    let __arc7 = &(*ishared);
    let BackendDAE::SHARED {
        functionTree: __pa6, ..
    } = &**__arc7;
    funcs = metamodelica::Own::own(__pa6);
    eqns1 = BackendEquation::listEquation(eqn_lst)?;
    v = BackendVariable::listVar1(&var_lst)?;
    n = ((var_lst).len() as i32);
    (beqs, sources) = BackendDAEUtil::getEqnSysRhs(eqns1, v, Some(funcs))?;
    beqs = beqs.reverse();
    A = evaluateConstantJacobianArray(((var_lst).len() as i32), jac)?;
    b = arrayCreate(n * n, metamodelica::OrderedFloat(0.0_f64));
    order = arrayCreate(n, 0);
    for mut row in 1..=n {
        metamodelica::arrayUpdate(b.clone(), (row - 1) * n + row, metamodelica::OrderedFloat(1.0_f64))?;
    }
    gauss(A.clone(), b.clone(), 1, n, &(List::intRange(n)), order.clone());
    (bVarsOut, bEqsOut) = createBVecVars(sysIdxIn, compIdxIn, n, DAE::T_REAL_DEFAULT().clone(), &beqs)?;
    sysEqsOut = createSysEquations(A.clone(), b.clone(), n, order.clone(), var_lst, bVarsOut.clone())?;
    let __range8 = A.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut a in __range8 {
        GCExt::free(a.clone());
    }
    GCExt::free(A.clone());
    GCExt::free(b.clone());
    sysIdxOut = sysIdxIn + 1;
    orderOut = order.clone();
    Ok((sysEqsOut, bEqsOut, bVarsOut, orderOut, sysIdxOut))
}

fn createSysEquations(
    mut A: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
    mut b: metamodelica::Array<metamodelica::Real>,
    mut n: i32,
    mut order: metamodelica::Array<i32>,
    mut xVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut bVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let __ab_A = A.borrow();
    let mut sysEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut row: i32;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut coeffExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut xExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut bExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut xProds: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut bProds: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut coeffs: metamodelica::List<metamodelica::Real>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    xExps = List::map(xVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>| {
        BackendVariable::varExp2(&__a0)
    })?;
    bExps = List::map(bVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>| {
        BackendVariable::varExp2(&__a0)
    })?;
    for mut i in 1..=n {
        row = metamodelica::arrayGet(order.clone(), i)?;
        coeffs = (*metamodelica::index_checked(&__ab_A, row)?)
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>();
        coeffExps = List::map(coeffs, &fnptr!(Expression::makeRealExp, metamodelica::Real))?;
        xProds = List::threadMap1(
            coeffExps,
            xExps.clone(),
            &fnptr!(
                makeBinaryExp,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<DAE::Exp>,
                DAE::Operator
            ),
            DAE::Operator::MUL {
                ty: DAE::T_REAL_DEFAULT().clone(),
            },
        )?;
        lhs = List::fold1(
            &xProds,
            &fnptr!(
                Expression::makeBinaryExp,
                metamodelica::Ref<DAE::Exp>,
                DAE::Operator,
                metamodelica::Ref<DAE::Exp>
            ),
            DAE::Operator::ADD {
                ty: DAE::T_REAL_DEFAULT().clone(),
            },
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            }),
        )?;
        (lhs, _) = ExpressionSimplify::simplify(lhs)?;
        coeffs = Array::getRange((row - 1) * n + 1, row * n, b.clone())?;
        coeffExps = List::map(coeffs, &fnptr!(Expression::makeRealExp, metamodelica::Real))?;
        bProds = List::threadMap1(
            coeffExps,
            bExps.clone(),
            &fnptr!(
                makeBinaryExp,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<DAE::Exp>,
                DAE::Operator
            ),
            DAE::Operator::MUL {
                ty: DAE::T_REAL_DEFAULT().clone(),
            },
        )?;
        rhs = List::fold1(
            &bProds,
            &fnptr!(
                Expression::makeBinaryExp,
                metamodelica::Ref<DAE::Exp>,
                DAE::Operator,
                metamodelica::Ref<DAE::Exp>
            ),
            DAE::Operator::ADD {
                ty: DAE::T_REAL_DEFAULT().clone(),
            },
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            }),
        )?;
        (rhs, _) = ExpressionSimplify::simplify(rhs)?;
        eq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: lhs,
            scalar: rhs,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
        });
        sysEqs = metamodelica::cons(eq, sysEqs);
    }
    Ok(sysEqs)
}

pub(crate) fn makeBinaryExp(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
    mut inOp: DAE::Operator,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: inLhs,
        operator: inOp,
        exp2: inRhs,
    });
    outExp
}

fn createBVecVars(
    mut sysIdx: i32,
    mut compIdx: i32,
    mut size: i32,
    mut typ: metamodelica::Ref<DAE::Type>,
    mut bExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut ident: ArcStr;
    let mut i: i32 = 0;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut beq: metamodelica::Ref<BackendDAE::Equation>;
    for mut i in 1..=size {
        ident = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$sys"));
            __mm_s.push_str(&*intString(sysIdx));
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(compIdx));
            __mm_s.push_str(&*literal!("_b"));
            __mm_s.push_str(&*intString(i));
            ArcStr::from(__mm_s)
        };
        cref = ComponentReferenceBasics::makeCrefIdent(ident, typ.clone(), metamodelica::nil());
        var = BackendVariable::makeVar(cref.clone())?;
        varLst = metamodelica::cons(var, varLst);
        beq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: (bExps).get(i)?,
            scalar: Expression::crefExp(cref)?,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
        });
        eqLst = metamodelica::cons(beq, eqLst);
    }
    Ok((varLst, eqLst))
}

fn gauss(
    mut A: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
    mut b: metamodelica::Array<metamodelica::Real>,
    mut indxIn: i32,
    mut n: i32,
    mut rangeIn: &metamodelica::List<i32>,
    mut permutation: metamodelica::Array<i32>,
) -> () {
    let mut pivotIdx: i32 = 0;
    let mut pos: i32 = 0;
    let mut ir: i32 = 0;
    let mut ic: i32 = 0;
    let mut pivot: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut entry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut b_entry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut first: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut range: metamodelica::List<i32> = metamodelica::nil();
    let () = 'mc: {
        let __mc_input = permutation.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut b_entry: metamodelica::Real = b_entry.clone();
            let mut entry: metamodelica::Real = entry.clone();
            let mut first: metamodelica::Real = first.clone();
            let mut pivot: metamodelica::Real = pivot.clone();
            let mut pivotIdx: i32 = pivotIdx.clone();
            let mut pos: i32 = pos.clone();
            let mut range: metamodelica::List<i32> = range.clone();
            let true = (intLe(indxIn, n)) else {
                return Err("pattern mismatch");
            };
            (pivotIdx, pivot) = getPivotElement(A.clone(), rangeIn, indxIn, n)?;
            metamodelica::arrayUpdate(permutation.clone(), indxIn, pivotIdx)?;
            (range, _) = List::deleteMemberOnTrue(pivotIdx, rangeIn.clone(), &fnptr!(intEq, i32, i32))?;
            for mut ic in indxIn..=n {
                entry = metamodelica::arrayGet(
                    ({
                        let __elt = (*metamodelica::index_checked(&A.borrow(), pivotIdx)?).clone();
                        __elt
                    }),
                    ic,
                )?;
                entry = realDiv(entry, pivot);
                metamodelica::arrayUpdate(
                    ({
                        let __elt = (*metamodelica::index_checked(&A.borrow(), pivotIdx)?).clone();
                        __elt
                    }),
                    ic,
                    entry,
                )?;
            }
            for mut ic in 1..=n {
                pos = (pivotIdx - 1) * n + ic;
                b_entry = metamodelica::arrayGet(b.clone(), pos)?;
                b_entry = realDiv(b_entry, pivot);
                metamodelica::arrayUpdate(b.clone(), pos, b_entry)?;
            }
            for mut ir in &*range {
                let mut ir = ir.clone();
                first = metamodelica::arrayGet(
                    ({
                        let __elt = (*metamodelica::index_checked(&A.borrow(), ir)?).clone();
                        __elt
                    }),
                    indxIn,
                )?;
                for mut ic in indxIn..=n {
                    pos = (ir - 1) * n + ic;
                    entry = metamodelica::arrayGet(
                        ({
                            let __elt = (*metamodelica::index_checked(&A.borrow(), ir)?).clone();
                            __elt
                        }),
                        ic,
                    )?;
                    pivot = metamodelica::arrayGet(
                        ({
                            let __elt = (*metamodelica::index_checked(&A.borrow(), pivotIdx)?).clone();
                            __elt
                        }),
                        ic,
                    )?;
                    entry = (entry) - ((first) * (pivot));
                    metamodelica::arrayUpdate(
                        ({
                            let __elt = (*metamodelica::index_checked(&A.borrow(), ir)?).clone();
                            __elt
                        }),
                        ic,
                        entry,
                    )?;
                    b_entry = metamodelica::arrayGet(b.clone(), pos)?;
                    pivot = metamodelica::arrayGet(b.clone(), (pivotIdx - 1) * n + ic)?;
                    b_entry = b_entry - (first) * (pivot);
                    metamodelica::arrayUpdate(b.clone(), pos, b_entry)?;
                }
            }
            gauss(A.clone(), b.clone(), indxIn + 1, n, &range, permutation.clone());
            Ok((
                (),
                b_entry.clone(),
                entry.clone(),
                first.clone(),
                pivot.clone(),
                pivotIdx.clone(),
                pos.clone(),
                range.clone(),
            ))
        })() {
            b_entry = __wb0;
            entry = __wb1;
            first = __wb2;
            pivot = __wb3;
            pivotIdx = __wb4;
            pos = __wb5;
            range = __wb6;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn getPivotElement(
    mut A: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
    mut rangeIn: &metamodelica::List<i32>,
    mut startIdx: i32,
    mut n: i32,
) -> Result<(i32, metamodelica::Real)> {
    let __ab_A = A.borrow();
    let mut pos: i32 = 0;
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i: i32 = 0;
    let mut entry: metamodelica::Real;
    for mut i in &**rangeIn {
        let mut i = i.clone();
        entry = metamodelica::arrayGet((*metamodelica::index_checked(&__ab_A, i)?).clone(), startIdx)?;
        if realAbs(entry) > value {
            value = entry;
            pos = i;
        }
    }
    Ok((pos, value))
}

fn rListStr(mut l: metamodelica::List<metamodelica::Real>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(List::map(l, &fnptr!(realString, metamodelica::Real))?, literal!(" , "));
    Ok(s)
}

// =============================================================================
// unsorted section
//
// =============================================================================
fn constantLinearSystem0(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut iTpl: (bool, i32),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (bool, i32),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut oTpl: (bool, i32);
    let mut changed: bool;
    let mut sysIdx: i32;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    (changed, sysIdx) = iTpl;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    (osyst, outShared, changed, sysIdx) = constantLinearSystem1(isyst, inShared, &comps, changed, sysIdx, 1);
    osyst = constantLinearSystem2(changed, osyst)?;
    oTpl = (changed, sysIdx + 1);
    Ok((osyst, outShared, oTpl))
}

fn constantLinearSystem2(
    mut b: bool,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &((b, isyst.clone())) {
        (false, _) => {
            isyst
        },
        (true, Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, stateSets, partitionKind, .. }) => {
            let mut vars = (*vars).clone();
            let mut eqns = (*eqns).clone();
            vars = BackendVariable::listVar1(&(BackendVariable::varList(metamodelica::AsArg::as_arg(&vars))?))?;
            eqns = BackendEquation::listEquation(&(BackendEquation::equationList(eqns.clone())?))?;
            BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), stateSets.clone(), partitionKind.clone(), BackendEquation::emptyEqns())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(osyst)
}

fn constantLinearSystem1(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inRunMatching: bool,
    mut sysIdxIn: i32,
    mut compIdxIn: i32,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
    i32,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut runMatching: bool = inRunMatching;
    let mut sysIdxOut: i32 = sysIdxIn;
    let mut compIdx: i32 = compIdxIn;
    let mut b: bool;
    for mut comp in &**inComps {
        (osyst, oshared, b, sysIdxOut, compIdx) =
            constantLinearSystemWork(osyst, oshared, metamodelica::AsArg::as_arg(&comp), sysIdxOut, compIdx);
        runMatching = b || runMatching;
    }
    (osyst, oshared, runMatching, sysIdxOut)
}

fn constantLinearSystemWork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut sysIdxIn: i32,
    mut compIdxIn: i32,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
    i32,
    i32,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outRunMatching: bool;
    let mut sysIdxOut: i32;
    let mut compIdxOut: i32;
    (osyst, oshared, outRunMatching, sysIdxOut, compIdxOut) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), &**comp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_CONSTANT { .. }, .. }) => {
                    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqn_lst = BackendEquation::getList(eindex.clone(), syst.orderedEqs.clone())?;
                    var_lst = List::map1r(vindx.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), syst.orderedVars.clone())?;
                    (syst, shared) = solveLinearSystem(syst.clone(), shared.clone(), eqn_lst.clone(), metamodelica::AsArg::as_arg(&eindex), var_lst.clone(), vindx.clone(), metamodelica::AsArg::as_arg(&jac))?;
                    Ok((syst.clone(), shared.clone(), true, sysIdxIn, compIdxIn + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }) => {
                    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut sysIdx: i32;
                    let mut order: metamodelica::Array<i32>;
                    let mut bVarIdcs: metamodelica::List<i32>;
                    let mut bEqIdcs: metamodelica::List<i32>;
                    let mut bVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut bEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut sysEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut bComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut sysComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (BackendDAEUtil::isSimulationDAE(&ishared)) else { return Err("pattern mismatch") };
                    eqn_lst = BackendEquation::getList(eindex.clone(), eqns.clone())?;
                    var_lst = List::map1r(vindx.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    let true = (jacobianIsConstant(jac.clone())?) else { return Err("pattern mismatch") };
                    let true = (Flags::isSet(Flags::CONSTJAC.clone())?) else { return Err("pattern mismatch") };
                    eqn_lst = BackendEquation::getList(eindex.clone(), eqns.clone())?;
                    var_lst = List::map1r(vindx.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    (sysEqs, bEqs, bVars, order, sysIdx) = solveConstJacLinearSystem(metamodelica::AsArg::as_arg(&syst), metamodelica::AsArg::as_arg(&shared), &eqn_lst, metamodelica::AsArg::as_arg(&eindex), var_lst.clone().reverse(), metamodelica::AsArg::as_arg(&vindx), metamodelica::AsArg::as_arg(&jac), sysIdxIn, compIdxIn)?;
                    bVarIdcs = List::intRange2(BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars)) + 1, BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars)) + ((bVars).len() as i32));
                    bEqIdcs = List::intRange2(BackendEquation::getNumberOfEquations(eqns.clone()) + 1, BackendEquation::getNumberOfEquations(eqns.clone()) + ((bEqs).len() as i32));
                    bComps = List::threadMap(bEqIdcs.clone(), bVarIdcs.clone(), &fnptr!(BackendDAEUtil::makeSingleEquationComp, i32, i32))?;
                    sysComps = List::threadMap(List::map1(order.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), eindex.clone())?, vindx.clone().reverse(), &fnptr!(BackendDAEUtil::makeSingleEquationComp, i32, i32))?;
                    assign_field!(syst.orderedVars = List::fold(&bVars, &BackendVariable::addVar, vars.clone())?);
                    eqns = BackendEquation::addList(&bEqs, eqns.clone())?;
                    assign_field!(syst.orderedEqs = List::threadFold(metamodelica::AsArg::as_arg(&eindex), sysEqs.clone(), &BackendEquation::setAtIndexFirst, eqns.clone())?);
                    syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None);
                    syst = replaceStrongComponent(syst.clone(), compIdxIn, sysComps.clone(), bComps.clone())?;
                    Ok((syst.clone(), ishared.clone(), false, sysIdx, compIdxIn + ((sysComps).len() as i32)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone(), false, sysIdxIn, compIdxIn + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared, outRunMatching, sysIdxOut, compIdxOut)
}

fn solveLinearSystem(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut eqn_indxs: &metamodelica::List<i32>,
    mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut var_indxs: metamodelica::List<i32>,
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    (osyst, oshared) = (::match_deref::match_deref! { match &((inSyst, ishared.clone())) {
        (syst @ Deref @ BackendDAE::EqSystem { .. }, Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
            let mut v: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut eqns1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut beqs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut sources: metamodelica::List<metamodelica::Ref<DAE::ElementSource>>;
            let mut rhsVals: metamodelica::List<metamodelica::Real>;
            let mut solvedVals: metamodelica::List<metamodelica::Real>;
            let mut jacVals: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut linInfo: i32;
            let mut names: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut syst = (*syst).clone();
            eqns1 = BackendEquation::listEquation(&eqn_lst)?;
            v = BackendVariable::listVar1(&var_lst)?;
            (beqs, sources) = BackendDAEUtil::getEqnSysRhs(eqns1, v, Some(funcs.clone()))?;
            beqs = beqs.reverse();
            rhsVals = ValuesUtil::valueReals(&(List::map(beqs, &Ceval::cevalSimple)?));
            jacVals = evaluateConstantJacobian(((var_lst).len() as i32), jac)?;
            (solvedVals, linInfo) = System::dgesv(jacVals.clone(), rhsVals.clone())?;
            names = List::map(var_lst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?;
            checkLinearSystem(linInfo, names.clone(), jacVals.clone(), rhsVals.clone(), eqn_lst)?;
            sources = List::map1(sources, &ElementSource::addSymbolicTransformation, metamodelica::Ref::new(DAE::SymbolicOperation::LINEAR_SOLVED { vars: names, jac: jacVals, rhs: rhsVals, result: solvedVals.clone() }))?;
            (v, eqns, shared) = changeConstantLinearSystemVars(var_lst, solvedVals, sources, var_indxs, syst.orderedVars.clone(), syst.orderedEqs.clone(), ishared)?;
            assign_field!(
                syst.orderedVars = v,
                syst.orderedEqs = List::fold(eqn_indxs, &BackendEquation::delete, eqns)?
            );
            (BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None), shared)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared))
}

fn changeConstantLinearSystemVars(
    mut inVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inSolvedVals: metamodelica::List<metamodelica::Real>,
    mut inSources: metamodelica::List<metamodelica::Ref<DAE::ElementSource>>,
    mut var_indxs: metamodelica::List<i32>,
    mut inVars: BackendDAE::Variables,
    mut ieqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inVarLst, inSolvedVals, inSources, var_indxs, inVars, ieqns)) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, vars, eqns) => {
                return Ok((vars.clone(), eqns.clone(), ishared))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: cref, varKind: BackendDAE::VarKind::STATE { .. }, varType: tp, .. }, tail: varlst }, Deref @ metamodelica::ListNode::Cons { head: r, tail: rlst }, Deref @ metamodelica::ListNode::Cons { head: _, tail: slst }, Deref @ metamodelica::ListNode::Cons { head: _, tail: vindxs }, vars, eqns) => {
                let mut vars2: BackendDAE::Variables;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut eqns = (*eqns).clone();
                e = Expression::makeCrefExp(cref.clone(), tp.clone())?;
                e = Expression::expDer(e);
                eqns = BackendEquation::add(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e, scalar: metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), eqns.clone())?;
                { (inVarLst, inSolvedVals, inSources, var_indxs, inVars, ieqns, ishared) = (varlst.clone(), rlst.clone(), slst.clone(), vindxs.clone(), vars.clone(), eqns.clone(), ishared); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: v, tail: varlst }, Deref @ metamodelica::ListNode::Cons { head: r, tail: rlst }, Deref @ metamodelica::ListNode::Cons { head: _, tail: slst }, Deref @ metamodelica::ListNode::Cons { head: indx, tail: vindxs }, vars, eqns) => {
                let mut v1: metamodelica::Ref<BackendDAE::Var>;
                let mut vars1: BackendDAE::Variables;
                let mut vars2: BackendDAE::Variables;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut eqns = (*eqns).clone();
                v1 = BackendVariable::setBindExp(v.clone(), Some(metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() })));
                v1 = BackendVariable::setVarStartValue(v1, metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }))?;
                (vars1, _) = BackendVariable::removeVar(indx.clone(), vars.clone())?;
                shared = BackendVariable::addGlobalKnownVarDAE(v1, ishared)?;
                { (inVarLst, inSolvedVals, inSources, var_indxs, inVars, ieqns, ishared) = (varlst.clone(), rlst.clone(), slst.clone(), vindxs.clone(), vars1, eqns.clone(), shared); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn evaluateConstantJacobian(
    mut size: i32,
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Real>>> {
    let mut vals: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut valarr: metamodelica::Array<metamodelica::Array<metamodelica::Real>>;
    let mut tmp2: metamodelica::List<metamodelica::Array<metamodelica::Real>>;
    valarr = evaluateConstantJacobianArray(size, jac)?;
    tmp2 = valarr
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    vals = List::map(tmp2, &fnptr!(arrayList, metamodelica::Array<metamodelica::Real>))?;
    Ok(vals)
}

fn evaluateConstantJacobianArray(
    mut size: i32,
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<metamodelica::Array<metamodelica::Array<metamodelica::Real>>> {
    let mut valarr: metamodelica::Array<metamodelica::Array<metamodelica::Real>>;
    let mut tmp: metamodelica::Array<metamodelica::Real>;
    let mut tmp2: metamodelica::List<metamodelica::Array<metamodelica::Real>>;
    tmp = arrayCreate(size, metamodelica::OrderedFloat(0.0_f64));
    tmp2 = List::map(
        List::fill(tmp.clone(), size),
        &fnptr!(arrayCopy, metamodelica::Array<metamodelica::Real>),
    )?;
    valarr = metamodelica::arrayFromVec(tmp2.into_iter().cloned().collect());
    List::map1_0(
        jac,
        &move |__a0: (i32, i32, metamodelica::Ref<BackendDAE::Equation>),
               __a1: metamodelica::Array<metamodelica::Array<metamodelica::Real>>| {
            evaluateConstantJacobian2(&__a0, __a1)
        },
        valarr.clone(),
    )?;
    Ok(valarr)
}

fn evaluateConstantJacobian2(
    mut jac: &(i32, i32, metamodelica::Ref<BackendDAE::Equation>),
    mut vals: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(jac) {
        (i1, i2, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp, .. }) => {
            let mut r: metamodelica::Real;
            let __pa0 = ::match_deref::match_deref! { match &(Ceval::cevalSimple(exp.clone())?) {
                Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            metamodelica::arrayUpdate(metamodelica::arrayGet(vals.clone(), i1.clone())?, i2.clone(), r)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn checkLinearSystem(
    mut info: i32,
    mut vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut jac: metamodelica::List<metamodelica::List<metamodelica::Real>>,
    mut rhs: metamodelica::List<metamodelica::Real>,
    mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = info;
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { return Err("nomatch") };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut infoStr: ArcStr;
            let mut syst: ArcStr;
            let mut varnames: ArcStr;
            let mut varname: ArcStr;
            let mut rhsStr: ArcStr;
            let mut jacStr: ArcStr;
            let mut eqnstr: ArcStr;
            let true = (info > 0) else {
                return Err("pattern mismatch");
            };
            varname = ComponentReferenceBasics::printComponentRefStr(&((vars).get(info)?))?;
            infoStr = intString(info);
            varnames = stringDelimitList(
                List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                })?,
                literal!(" ;\n  "),
            );
            rhsStr = stringDelimitList(
                List::map(rhs.clone(), &fnptr!(realString, metamodelica::Real))?,
                literal!(" ;\n  "),
            );
            jacStr = stringDelimitList(
                List::map1(
                    List::mapList(jac.clone(), &fnptr!(realString, metamodelica::Real))?,
                    &fnptr!(stringDelimitList, metamodelica::List<ArcStr>, ArcStr),
                    literal!(" , "),
                )?,
                literal!(" ;\n  "),
            );
            eqnstr = BackendDump::dumpEqnsStr(eqnlst.clone())?;
            syst = stringAppendList(list![
                literal!("\n"),
                eqnstr.clone(),
                literal!("\n[\n  "),
                jacStr.clone(),
                literal!("\n]\n  *\n[\n  "),
                varnames.clone(),
                literal!("\n]\n  =\n[\n  "),
                rhsStr.clone(),
                literal!("\n]")
            ]);
            Error::addMessage(
                Error::LINEAR_SYSTEM_SINGULAR.clone(),
                list![syst.clone(), infoStr.clone(), varname.clone()],
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut syst: ArcStr;
            let mut varnames: ArcStr;
            let mut rhsStr: ArcStr;
            let mut jacStr: ArcStr;
            let mut eqnstr: ArcStr;
            let true = (info < 0) else {
                return Err("pattern mismatch");
            };
            varnames = stringDelimitList(
                List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                })?,
                literal!(" ;\n  "),
            );
            rhsStr = stringDelimitList(
                List::map(rhs.clone(), &fnptr!(realString, metamodelica::Real))?,
                literal!(" ; "),
            );
            jacStr = stringDelimitList(
                List::map1(
                    List::mapList(jac.clone(), &fnptr!(realString, metamodelica::Real))?,
                    &fnptr!(stringDelimitList, metamodelica::List<ArcStr>, ArcStr),
                    literal!(" , "),
                )?,
                literal!(" ; "),
            );
            eqnstr = BackendDump::dumpEqnsStr(eqnlst.clone())?;
            syst = stringAppendList(list![
                eqnstr.clone(),
                literal!("\n["),
                jacStr.clone(),
                literal!("] * ["),
                varnames.clone(),
                literal!("] = ["),
                rhsStr.clone(),
                literal!("]")
            ]);
            Error::addMessage(
                Error::LINEAR_SYSTEM_INVALID.clone(),
                list![literal!("LAPACK/dgesv"), syst.clone()],
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub(crate) fn generateSparsePattern(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inIndependentVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inDependentVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut nonlinearPattern: bool,
    mut withColoring: bool,
) -> Result<(
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
)> {
    let mut outSparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ) = (
        metamodelica::nil(),
        metamodelica::nil(),
        (metamodelica::nil(), metamodelica::nil()),
        0,
    );
    let mut outColoredCols: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let debug: bool = false;
    let mut patternName: ArcStr = if (nonlinearPattern) {
        literal!("Nonlinear")
    } else {
        literal!("Sparsity")
    };
    (outSparsePattern, outColoredCols) = 'mc: {
        let __mc_input = (&**inBackendDAE, inIndependentVars, inDependentVars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(((metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst @ Deref @ BackendDAE::EqSystem { matching: bdaeMatching @ Deref @ BackendDAE::Matching::MATCHING { comps, ass1, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, independentVars, dependentVars) => {
                            let mut syst1: metamodelica::Ref<BackendDAE::EqSystem>;
                            let mut adjMatrix: metamodelica::Array<metamodelica::List<i32>>;
                            let mut adjMatrixT: metamodelica::Array<metamodelica::List<i32>>;
                            let mut sizeN: i32;
                            let mut sizeM: i32;
                            let mut adjSize: i32;
                            let mut adjSizeT: i32;
                            let mut nonZeroElements: i32;
                            let mut nodesEqnsIndex: metamodelica::List<i32>;
                            let mut sparsepattern: metamodelica::List<metamodelica::List<i32>>;
                            let mut sparsepatternT: metamodelica::List<metamodelica::List<i32>>;
                            let mut jacDiffVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                            let mut varswithDiffs: BackendDAE::Variables;
                            let mut orderedEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                            let mut coloredArray: metamodelica::Array<metamodelica::List<i32>>;
                            let mut depCompRefsLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut inDepCompRefsLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut depCompRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut inDepCompRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut eqnSparse: metamodelica::Array<metamodelica::List<i32>>;
                            let mut varSparse: metamodelica::Array<metamodelica::List<i32>>;
                            let mut sparseArray: metamodelica::Array<metamodelica::List<i32>>;
                            let mut sparseArrayT: metamodelica::Array<metamodelica::List<i32>>;
                            let mut mark: metamodelica::Array<i32>;
                            let mut usedvar: metamodelica::Array<i32>;
                            let mut coloring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
                            let mut translated: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
                            let mut sparsetuple: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>;
                            let mut sparsetupleT: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>;
                            let mut outSparsePattern: (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>), i32) = outSparsePattern.clone();
                            if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" start getting ")); __mm_s.push_str(&*patternName); __mm_s.push_str(&*literal!(" pattern for variables : ")); __mm_s.push_str(&*intString(((dependentVars).len() as i32))); __mm_s.push_str(&*literal!(" and the independent vars: ")); __mm_s.push_str(&*intString(((independentVars).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> do start ")))?;
                            }
                            depCompRefsLst = List::map(dependentVars.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?;
                            depCompRefs = metamodelica::arrayFromVec(depCompRefsLst.clone().into_iter().cloned().collect());
                            sizeM = metamodelica::arrayLength(depCompRefs.clone());
                            (jacDiffVars, inDepCompRefsLst) = createInDepVars(metamodelica::AsArg::as_arg(&independentVars), true)?;
                            inDepCompRefs = metamodelica::arrayFromVec(inDepCompRefsLst.clone().into_iter().cloned().collect());
                            sizeN = metamodelica::arrayLength(inDepCompRefs.clone());
                            let __arc3 = BackendDAEUtil::addVarsToEqSystem(syst.clone(), &jacDiffVars)?;
                            let __pa2 = (__arc3).clone();
                            let BackendDAE::EQSYSTEM { orderedVars: __pa0, orderedEqs: __pa1, .. } = &*__arc3;
                            varswithDiffs = metamodelica::Own::own(__pa0);
                            orderedEqns = metamodelica::Own::own(__pa1);
                            syst1 = metamodelica::Own::own(__pa2);
                            (adjMatrix, adjMatrixT) = BackendDAEUtil::adjacencyMatrix(&syst1, openmodelica_backend_types::BackendDAE::IndexType::SPARSE, None, BackendDAEUtil::isInitializationDAE(&inBackendDAE.shared))?;
                            adjSize = metamodelica::arrayLength(adjMatrix.clone());
                            adjSizeT = metamodelica::arrayLength(adjMatrixT.clone());
                            if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                                BackendDump::printVarList(&(BackendVariable::varList(&varswithDiffs)?))?;
                                BackendDump::printEquationList(&(BackendEquation::equationList(orderedEqns.clone())?))?;
                                BackendDump::dumpAdjacencyMatrix(adjMatrix.clone())?;
                                BackendDump::dumpAdjacencyMatrixT(adjMatrixT.clone())?;
                                BackendDump::dumpFullMatching(metamodelica::AsArg::as_arg(&bdaeMatching), None)?;
                            }
                            nodesEqnsIndex = BackendVariable::getVarIndexFromVars(metamodelica::AsArg::as_arg(&dependentVars), &varswithDiffs);
                            nodesEqnsIndex = List::map1(nodesEqnsIndex.clone(), &Array::getIndexFirst, ass1.clone())?;
                            if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                                metamodelica::print(literal!("nodesEqnsIndexs: "));
                                BackendDump::dumpAdjacencyRow(&nodesEqnsIndex)?;
                                metamodelica::print(literal!("\n"));
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians[")); __mm_s.push_str(&*patternName); __mm_s.push_str(&*literal!("] -> build sparse graph: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                            eqnSparse = arrayCreate(adjSize, metamodelica::nil());
                            varSparse = arrayCreate(adjSizeT, metamodelica::nil());
                            mark = arrayCreate(adjSizeT, 0);
                            usedvar = arrayCreate(adjSizeT, 0);
                            if sizeN > 0 {
                                usedvar = Array::setRange(adjSizeT - (sizeN - 1), adjSizeT, usedvar.clone(), 1)?;
                            }
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> start ")))?;
                            }
                            eqnSparse = getSparsePattern(comps.clone(), eqnSparse.clone(), varSparse.clone(), mark.clone(), usedvar.clone(), 1, adjMatrix.clone(), adjMatrixT.clone())?;
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> end ")))?;
                            }
                            if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                                BackendDump::dumpSparsePatternArray(eqnSparse.clone())?;
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians[")); __mm_s.push_str(&*patternName); __mm_s.push_str(&*literal!("] -> prepared arrayList for transpose list: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                            sparseArray = Array::select(eqnSparse.clone(), &nodesEqnsIndex)?;
                            sparsepattern = sparseArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>();
                            sparsepattern = List::map1List(sparsepattern.clone(), &fnptr!(intSub, i32, i32), adjSizeT - sizeN)?;
                            sparseArray = metamodelica::arrayFromVec(sparsepattern.clone().into_iter().cloned().collect());
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> postProcess ")))?;
                            }
                            sparseArrayT = arrayCreate(sizeN, metamodelica::nil());
                            sparseArrayT = transposeSparsePattern(&sparsepattern, sparseArrayT.clone(), 1)?;
                            sparsepatternT = sparseArrayT.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>();
                            nonZeroElements = List::lengthListElements(sparsepattern.clone());
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> transpose done ")))?;
                            }
                            if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                                dumpSparsePatternStatistics(nonZeroElements, &sparsepatternT)?;
                                BackendDump::dumpSparsePattern(&sparsepattern)?;
                                BackendDump::dumpSparsePattern(&sparsepatternT)?;
                            }
                            if (sparsepattern).is_empty() {
                                sparsetuple = metamodelica::nil();
                                sparsetupleT = metamodelica::nil();
                            } else {
                                translated = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> = metamodelica::nil();
                for mut lst in (sparsepattern.clone()).into_iter().cloned() {
                            let __x = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut i in (lst.clone()).into_iter().cloned() {
                            let __x = metamodelica::arrayGet(inDepCompRefs.clone(), i.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                sparsetuple = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)> = metamodelica::nil();
                let __thr_src0 = depCompRefs.clone();
                let __thr_borrow0 = __thr_src0.borrow();
                let mut __thr_it0 = __thr_borrow0.iter().cloned();
                let __thr_src1 = translated.clone();
                let mut __thr_it1 = (&__thr_src1).into_iter();
                loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(cr), Some(t)) => {
                                    let __x = (cr.clone(), t.clone());
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                }
                __acc.reverse()
            });
                                translated = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> = metamodelica::nil();
                for mut lst in (sparsepatternT.clone()).into_iter().cloned() {
                            let __x = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut i in (lst.clone()).into_iter().cloned() {
                            let __x = metamodelica::arrayGet(depCompRefs.clone(), i.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                sparsetupleT = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)> = metamodelica::nil();
                let __thr_src0 = inDepCompRefs.clone();
                let __thr_borrow0 = __thr_src0.borrow();
                let mut __thr_it0 = __thr_borrow0.iter().cloned();
                let __thr_src1 = translated.clone();
                let mut __thr_it1 = (&__thr_src1).into_iter();
                loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(cr), Some(t)) => {
                                    let __x = (cr.clone(), t.clone());
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                }
                __acc.reverse()
            });
                            }
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> coloring start ")))?;
                            }
                            if nonlinearPattern || !(withColoring) || Flags::isSet(Flags::DISABLE_COLORING.clone())? {
                                coloring = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> = metamodelica::nil();
                for mut i in (1..=sizeN).into_iter() {
                            let __x = list![metamodelica::arrayGet(inDepCompRefs.clone(), i.clone())?];
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            } else {
                                coloredArray = Coloring::createColoring(sparseArray.clone(), sparseArrayT.clone(), sizeN, sizeM)?;
                                coloring = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> = metamodelica::nil();
                for mut lst in (coloredArray.clone()).borrow().iter() {
                            let __x = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut i in (lst.clone()).into_iter().cloned() {
                            let __x = metamodelica::arrayGet(inDepCompRefs.clone(), i.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            }
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> coloring done ")))?;
                            }
                            if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians[")); __mm_s.push_str(&*patternName); __mm_s.push_str(&*literal!("] -> ready! ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                            outSparsePattern = (sparsetupleT.clone(), sparsetuple.clone(), (inDepCompRefsLst.clone(), depCompRefsLst.clone()), nonZeroElements);
                            if Flags::isSet(Flags::DUMP_SPARSE.clone())? {
                                BackendDump::dumpSparsityPattern(&outSparsePattern, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" --- ")); __mm_s.push_str(&*patternName); __mm_s.push_str(&*literal!(" Pattern ---")); ArcStr::from(__mm_s) }))?;
                                BackendDump::dumpSparseColoring(&coloring, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" --- ")); __mm_s.push_str(&*patternName); __mm_s.push_str(&*literal!(" Coloring ---")); ArcStr::from(__mm_s) }))?;
                            }
                            if debug {
                                execStat(&(literal!("generateSparsePattern -> final end ")))?;
                            }
                            Ok(((outSparsePattern.clone(), coloring.clone()), outSparsePattern.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outSparsePattern = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function generateSparsePattern failed"), metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outSparsePattern, outColoredCols))
}

fn dumpSparsePatternStatistics(
    mut nonZeroElements: i32,
    mut sparsepatternT: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<()> {
    let mut maxDegree: i32;
    (_, maxDegree) = List::mapFold(
        sparsepatternT,
        &move |__a0: _, __a1: i32| -> metamodelica::Result<_> { ::std::result::Result::Ok(findDegrees(&__a0, __a1)) },
        0,
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "analytical Jacobians[SPARSE] -> got sparse pattern nonZeroElements: "
        ));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", nonZeroElements)));
        __mm_s.push_str(&*literal!(" maxNodeDegree: "));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", maxDegree)));
        __mm_s.push_str(&*literal!(" time : "));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", clock())));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn findDegrees<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inValue: i32,
) -> (i32, i32) {
    let mut outDegree: i32;
    let mut outMaxDegree: i32;
    outDegree = ((inList).len() as i32);
    outMaxDegree = intMax(inValue, outDegree);
    (outDegree, outMaxDegree)
}

fn getSparsePattern(
    mut inComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut ineqnSparse: metamodelica::Array<metamodelica::List<i32>>,
    mut invarSparse: metamodelica::Array<metamodelica::List<i32>>,
    mut inMark: metamodelica::Array<i32>,
    mut inUsed: metamodelica::Array<i32>,
    mut inmarkValue: i32,
    mut inMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut inMatrixT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inComponents.clone(), ineqnSparse.clone())) {
            (Deref @ metamodelica::ListNode::Nil, result) => {
                return Ok(result.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn, var }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                inputVars = metamodelica::arrayGet(inMatrix.clone(), eqn.clone())?;
                inputVars = List::removeOnTrue(var.clone(), &fnptr!(intEq, i32, i32), inputVars)?;
                getSparsePattern2(&inputVars, &(list![var.clone()]), &(list![eqn.clone()]), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn, vars: solvedVars }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                inputVars = metamodelica::arrayGet(inMatrix.clone(), eqn.clone())?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, metamodelica::AsArg::as_arg(&solvedVars), &(list![eqn.clone()]), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn, vars: solvedVars }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                inputVars = metamodelica::arrayGet(inMatrixT.clone(), eqn.clone())?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, metamodelica::AsArg::as_arg(&solvedVars), &(list![eqn.clone()]), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn, vars: solvedVars }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                inputVars = metamodelica::arrayGet(inMatrix.clone(), eqn.clone())?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, metamodelica::AsArg::as_arg(&solvedVars), &(list![eqn.clone()]), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn, vars: solvedVars }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                inputVars = metamodelica::arrayGet(inMatrix.clone(), eqn.clone())?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, metamodelica::AsArg::as_arg(&solvedVars), &(list![eqn.clone()]), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn, vars: solvedVars }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                inputVars = metamodelica::arrayGet(inMatrix.clone(), eqn.clone())?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, metamodelica::AsArg::as_arg(&solvedVars), &(list![eqn.clone()]), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn, vars: solvedVars }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                inputVars = metamodelica::arrayGet(inMatrix.clone(), eqn.clone())?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, metamodelica::AsArg::as_arg(&solvedVars), &(list![eqn.clone()]), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns, vars: solvedVars, .. }, tail: rest }, result) => {
                let mut inputVars: metamodelica::List<i32>;
                let mut inputVarsLst: metamodelica::List<metamodelica::List<i32>>;
                let mut result = (*result).clone();
                inputVarsLst = List::map1(eqns.clone(), &Array::getIndexFirst, inMatrix.clone())?;
                inputVars = List::flatten(inputVarsLst)?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, metamodelica::AsArg::as_arg(&solvedVars), metamodelica::AsArg::as_arg(&eqns), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: eqns, tearingvars: vars, innerEquations, .. }, .. }, tail: rest }, result) => {
                let mut vars1: metamodelica::List<i32>;
                let mut eqns1: metamodelica::List<i32>;
                let mut inputVars: metamodelica::List<i32>;
                let mut inputVarsLst: metamodelica::List<metamodelica::List<i32>>;
                let mut solvedVars: metamodelica::List<i32>;
                let mut result = (*result).clone();
                (eqns1, inputVarsLst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                vars1 = List::flatten(inputVarsLst)?;
                eqns1 = listAppend(eqns.clone(), eqns1);
                solvedVars = listAppend(vars.clone(), vars1);
                inputVarsLst = List::map1(eqns1.clone(), &Array::getIndexFirst, inMatrix.clone())?;
                inputVars = List::flatten(inputVarsLst)?;
                inputVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut v in (inputVars).into_iter().cloned() {
                if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                getSparsePattern2(&inputVars, &solvedVars, &eqns1, ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue)?;
                { (inComponents, ineqnSparse, invarSparse, inMark, inUsed, inmarkValue, inMatrix, inMatrixT) = (rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue + 1, inMatrix.clone(), inMatrixT.clone()); continue '__tco; }
            },
            _ => {
                let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                let __pa0 = ::match_deref::match_deref! { match &(inComponents) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                comp = metamodelica::Own::own(__pa0);
                BackendDump::dumpComponent(&comp, None)?;
                Error::addInternalError(literal!("function getSparsePattern failed"), metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getSparsePattern2(
    mut inInputVars: &metamodelica::List<i32>,
    mut inSolvedVars: &metamodelica::List<i32>,
    mut inEqns: &metamodelica::List<i32>,
    mut ineqnSparse: metamodelica::Array<metamodelica::List<i32>>,
    mut invarSparse: metamodelica::Array<metamodelica::List<i32>>,
    mut inMark: metamodelica::Array<i32>,
    mut inUsed: metamodelica::Array<i32>,
    mut inmarkValue: i32,
) -> Result<()> {
    let mut localList: metamodelica::List<i32>;
    localList = getSparsePatternHelp(
        inInputVars,
        invarSparse.clone(),
        inMark.clone(),
        inUsed.clone(),
        inmarkValue,
    )?;
    List::map2_0(
        inSolvedVars,
        &Array::updateIndexFirst,
        localList.clone(),
        invarSparse.clone(),
    )?;
    List::map2_0(inEqns, &Array::updateIndexFirst, localList, ineqnSparse.clone())?;
    Ok(())
}

fn getSparsePatternHelp(
    mut inInputVars: &metamodelica::List<i32>,
    mut invarSparse: metamodelica::Array<metamodelica::List<i32>>,
    mut inMark: metamodelica::Array<i32>,
    mut inUsed: metamodelica::Array<i32>,
    mut inmarkValue: i32,
) -> Result<metamodelica::List<i32>> {
    let mut outLocalList: metamodelica::List<i32> = metamodelica::nil();
    let mut arrayElement: i32;
    let mut varSparse: metamodelica::List<i32>;
    for mut var in &**inInputVars {
        arrayElement = metamodelica::arrayGet(inUsed.clone(), var.clone())?;
        if intEq(1, arrayElement) {
            arrayElement = metamodelica::arrayGet(inMark.clone(), var.clone())?;
            if !(intEq(inmarkValue, arrayElement)) {
                metamodelica::arrayUpdate(inMark.clone(), var.clone(), inmarkValue)?;
                outLocalList = metamodelica::cons(var.clone(), outLocalList);
            }
        }
        varSparse = metamodelica::arrayGet(invarSparse.clone(), var.clone())?;
        for mut v in &*varSparse {
            arrayElement = metamodelica::arrayGet(inMark.clone(), v.clone())?;
            if !(intEq(inmarkValue, arrayElement)) {
                metamodelica::arrayUpdate(inMark.clone(), v.clone(), inmarkValue)?;
                outLocalList = metamodelica::cons(v.clone(), outLocalList);
            }
        }
    }
    Ok(outLocalList)
}

pub(crate) fn transposeSparsePattern(
    mut inSparsePattern: &metamodelica::List<metamodelica::List<i32>>,
    mut inAccumList: metamodelica::Array<metamodelica::List<i32>>,
    mut inValue: i32,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut outSparsePattern: metamodelica::Array<metamodelica::List<i32>> = inAccumList;
    let mut value: i32 = inValue;
    let mut tmplist: metamodelica::List<i32>;
    for mut oneList in &**inSparsePattern {
        for mut oneElem in &*oneList.clone() {
            tmplist = metamodelica::arrayGet(outSparsePattern.clone(), oneElem.clone())?;
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
                outSparsePattern.clone(),
                oneElem.clone(),
                metamodelica::cons(value, tmplist),
            );
        }
        value = value + 1;
    }
    Ok(outSparsePattern)
}

pub(crate) fn transposeSparsePatternTuple(
    mut inSparsePattern: &metamodelica::List<(i32, metamodelica::List<i32>)>,
    mut inAccumList: metamodelica::Array<(i32, metamodelica::List<i32>)>,
) -> Result<metamodelica::Array<(i32, metamodelica::List<i32>)>> {
    let mut outSparsePattern: metamodelica::Array<(i32, metamodelica::List<i32>)> = inAccumList;
    let mut value: i32;
    let mut tmplist: metamodelica::List<i32>;
    let mut oneList: metamodelica::List<i32>;
    let mut tmpTuple: (i32, metamodelica::List<i32>);
    let mut i: i32 = 0;
    for mut oneListTuple in &**inSparsePattern {
        (value, oneList) = oneListTuple.clone();
        for mut oneElem in &*oneList {
            tmpTuple = metamodelica::arrayGet(outSparsePattern.clone(), oneElem.clone() + 1)?;
            (_, tmplist) = tmpTuple;
            tmplist = metamodelica::cons(value, tmplist);
            tmpTuple = (oneElem.clone(), tmplist);
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
                outSparsePattern.clone(),
                oneElem.clone() + 1,
                tmpTuple,
            );
        }
    }
    for mut i in 1..=((inSparsePattern).len() as i32) {
        tmpTuple = metamodelica::arrayGet(outSparsePattern.clone(), i)?;
        (value, tmplist) = tmpTuple;
        tmplist = List::heapSortIntList(tmplist)?;
        tmpTuple = (value, tmplist);
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(outSparsePattern.clone(), i, tmpTuple);
    }
    Ok(outSparsePattern)
}

fn createInDepVars(
    mut independentVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut createpDerStates: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    for mut v in &**independentVars {
        if BackendVariable::isClockedStateVar(metamodelica::AsArg::as_arg(&v)) {
            var = BackendVariable::createClockedState(v.clone())?;
            outVars = metamodelica::cons(var.clone(), outVars);
            outCrefs = metamodelica::cons(var.varName.clone(), outCrefs);
        } else if createpDerStates {
            outVars = metamodelica::cons(BackendVariable::createpDerVar(v.clone())?, outVars);
            outCrefs = metamodelica::cons(v.varName.clone(), outCrefs);
        } else {
            outVars = metamodelica::cons(v.clone(), outVars);
            outCrefs = metamodelica::cons(v.varName.clone(), outCrefs);
        }
    }
    outVars = outVars.reverse();
    outCrefs = outCrefs.reverse();
    Ok((outVars, outCrefs))
}

pub fn createFMIModelDerivatives(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outJacobianMatrices: metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )> = metamodelica::nil();
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut backendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut emptyBDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqSyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outJacobian: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut inputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut paramvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut indepVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut depVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut v: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut statesarr: BackendDAE::Variables;
    let mut inputvarsarr: BackendDAE::Variables;
    let mut paramvarsarr: BackendDAE::Variables;
    let mut depVarsArr: BackendDAE::Variables;
    let mut sparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut sparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut nonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut ei: BackendDAE::ExtraInfo;
    let mut cache: FCore::Cache;
    let mut graph: FCore::Graph;
    if Flags::isSet(Flags::DIS_SYMJAC_FMI20.clone())? && ((inBackendDAE.eqs).len() as i32) == 1 {
        (sparsePattern, sparseColoring) = fmiDerSparsePattern(inBackendDAE)?;
        outJacobianMatrices = list![(
            Some((
                metamodelica::Ref::new(BackendDAE::BackendDAE {
                    eqs: list![BackendDAEUtil::createEqSystem(
                        BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
                        BackendEquation::emptyEqns(),
                        metamodelica::nil(),
                        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                        BackendEquation::emptyEqns()
                    )],
                    shared: BackendDAEUtil::createEmptyShared(
                        openmodelica_backend_types::BackendDAE::BackendDAEType::JACOBIAN,
                        inBackendDAE.shared.info.clone(),
                        inBackendDAE.shared.cache.clone(),
                        inBackendDAE.shared.graph.clone()
                    )?
                }),
                literal!("FMIDER"),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil()
            )),
            sparsePattern,
            sparseColoring,
            BackendDAE::emptyNonlinearPattern().clone()
        )];
        outFunctionTree = inBackendDAE.shared.functionTree.clone();
        return Ok((outJacobianMatrices, outFunctionTree));
    }
    match '__try0: {
        backendDAE = unwrap_break_err!(BackendDAEUtil::copyBackendDAE(inBackendDAE), '__try0);
        backendDAE = unwrap_break_err!(BackendDAEOptimize::collapseIndependentBlocks(&backendDAE), '__try0);
        backendDAE = unwrap_break_err!(BackendDAEUtil::transformBackendDAE(&backendDAE, Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT)), None, None), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(backendDAE.eqs.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        eqSyst = metamodelica::Own::own(__pa1);
        v = eqSyst.orderedVars.clone();
        globalKnownVars = backendDAE.shared.globalKnownVars.clone();
        varlst = unwrap_break_err!(BackendVariable::varList(&v), '__try0);
        knvarlst = unwrap_break_err!(BackendVariable::varList(&globalKnownVars), '__try0);
        states = if (unwrap_break_err!(Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone()), '__try0))
        {
            unwrap_break_err!(BackendVariable::getAllClockedStatesFromVariables(v.clone()), '__try0)
        } else {
            metamodelica::nil()
        };
        states = listAppend(
            unwrap_break_err!(BackendVariable::getAllStateVarFromVariables(v.clone()), '__try0),
            states.clone(),
        );
        inputvars = unwrap_break_err!(List::select(knvarlst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndInput(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)), '__try0);
        outputvars = unwrap_break_err!(List::select(varlst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndOutput(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)), '__try0);
        indepVars = listAppend(states.clone(), inputvars.clone());
        depVars = listAppend(states.clone(), outputvars.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DIS_SYMJAC_FMI20.clone()), '__try0) {
            cache = backendDAE.shared.cache.clone();
            graph = backendDAE.shared.graph.clone();
            ei = backendDAE.shared.info.clone();
            emptyBDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
                eqs: list![BackendDAEUtil::createEqSystem(
                    BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
                    BackendEquation::emptyEqns(),
                    metamodelica::nil(),
                    openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                    BackendEquation::emptyEqns()
                )],
                shared: unwrap_break_err!(BackendDAEUtil::createEmptyShared(openmodelica_backend_types::BackendDAE::BackendDAEType::JACOBIAN, ei.clone(), cache.clone(), graph.clone()), '__try0),
            });
            (sparsePattern, sparseColoring) = unwrap_break_err!(generateSparsePattern(&backendDAE, indepVars.clone(), depVars.clone(), false, false), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
                unwrap_break_err!(BackendDump::dumpSparsityPattern(&sparsePattern, &(literal!("FMI sparsity"))), '__try0);
            }
            outJacobianMatrices = metamodelica::cons(
                (
                    Some((
                        emptyBDAE.clone(),
                        literal!("FMIDER"),
                        metamodelica::nil(),
                        metamodelica::nil(),
                        metamodelica::nil(),
                        metamodelica::nil(),
                    )),
                    sparsePattern.clone(),
                    sparseColoring.clone(),
                    BackendDAE::emptyNonlinearPattern().clone(),
                ),
                outJacobianMatrices.clone(),
            );
            outFunctionTree = inBackendDAE.shared.functionTree.clone();
        } else {
            paramvars = unwrap_break_err!(List::select(knvarlst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isParam(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)), '__try0);
            statesarr = unwrap_break_err!(BackendVariable::listVar1(&states), '__try0);
            inputvarsarr = unwrap_break_err!(BackendVariable::listVar1(&inputvars), '__try0);
            paramvarsarr = unwrap_break_err!(BackendVariable::listVar1(&paramvars), '__try0);
            depVarsArr = unwrap_break_err!(BackendVariable::listVar1(&depVars), '__try0);
            (
                outJacobian,
                outFunctionTree,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = unwrap_break_err!(generateGenericJacobian(&backendDAE, indepVars.clone(), &statesarr, &inputvarsarr, &paramvarsarr, depVarsArr.clone(), varlst.clone(), literal!("FMIDER"), unwrap_break_err!(Flags::isSet(Flags::DIS_SYMJAC_FMI20.clone()), '__try0), false), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
                unwrap_break_err!(BackendDump::dumpSparsityPattern(&sparsePattern, &(literal!("FMI sparsity"))), '__try0);
            }
            outJacobianMatrices = metamodelica::cons(
                (
                    outJacobian.clone(),
                    sparsePattern.clone(),
                    sparseColoring.clone(),
                    nonlinearPattern.clone(),
                ),
                outJacobianMatrices.clone(),
            );
            outFunctionTree = unwrap_break_err!(AvlTreePathFunction::join(inBackendDAE.shared.functionTree.clone(), &outFunctionTree, &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))), '__try0);
        }
        Ok::<_, &'static str>((outFunctionTree.clone(), outJacobianMatrices.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outFunctionTree = __try0_o0;
            outJacobianMatrices = __try0_o1;
        }
        Err(_) => {
            Error::addInternalError(
                literal!("function createFMIModelDerivatives failed"),
                metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"),
            )?;
            outJacobianMatrices = metamodelica::nil();
            outFunctionTree = inBackendDAE.shared.functionTree.clone();
        }
    }
    Ok((outJacobianMatrices, outFunctionTree))
}

fn fmiDerSparsePattern(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
)> {
    let mut outSparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut outColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = BackendDAEUtil::copyBackendDAE(inDAE)?;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = (dae.eqs).head().cloned()?;
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut inputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    states = if (Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())?) {
        BackendVariable::getAllClockedStatesFromVariables(syst.orderedVars.clone())?
    } else {
        metamodelica::nil()
    };
    states = listAppend(
        BackendVariable::getAllStateVarFromVariables(syst.orderedVars.clone())?,
        states,
    );
    outputvars = List::select(
        BackendVariable::varList(&syst.orderedVars)?,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndOutput(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    inputvars = List::select(
        BackendVariable::varList(&dae.shared.globalKnownVars)?,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndInput(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    (outSparsePattern, outColoring) = generateSparsePattern(
        &dae,
        listAppend(states.clone(), inputvars),
        listAppend(states, outputvars),
        false,
        false,
    )?;
    Ok((outSparsePattern, outColoring))
}

pub(crate) fn createFMIModelDerivativesForInitialization(
    mut initDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut simDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut depVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut indepVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut orderedVars: &BackendDAE::Variables,
    mut sparsePattern_: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    mut sparseColoring_: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
) -> Result<(
    metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outJacobianMatrices: metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )> = metamodelica::nil();
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut backendDAE_1: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut emptyBDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outJacobian: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut clockedStates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut inputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut paramvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut statesarr: BackendDAE::Variables;
    let mut inputvarsarr: BackendDAE::Variables;
    let mut paramvarsarr: BackendDAE::Variables;
    let mut depVarsArr: BackendDAE::Variables;
    let mut ei: BackendDAE::ExtraInfo;
    let mut cache: FCore::Cache;
    let mut graph: FCore::Graph;
    let mut newOrderedEquationArray: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut rhsCr: metamodelica::Ref<DAE::ComponentRef>;
    let mut crefsVarsToRemove: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut protectedCrefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut newVars: BackendDAE::Variables;
    if Flags::isSet(Flags::DIS_SYMJAC_FMI20.clone())? {
        cache = initDAE.shared.cache.clone();
        graph = initDAE.shared.graph.clone();
        ei = initDAE.shared.info.clone();
        emptyBDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: list![BackendDAEUtil::createEqSystem(
                BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
                BackendEquation::emptyEqns(),
                metamodelica::nil(),
                openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                BackendEquation::emptyEqns()
            )],
            shared: BackendDAEUtil::createEmptyShared(
                openmodelica_backend_types::BackendDAE::BackendDAEType::JACOBIAN,
                ei,
                cache,
                graph,
            )?,
        });
        outJacobianMatrices = metamodelica::cons(
            (
                Some((
                    emptyBDAE,
                    literal!("FMIDERINIT"),
                    metamodelica::nil(),
                    metamodelica::nil(),
                    metamodelica::nil(),
                    metamodelica::nil(),
                )),
                BackendDAE::emptySparsePattern().clone(),
                metamodelica::nil(),
                BackendDAE::emptyNonlinearPattern().clone(),
            ),
            outJacobianMatrices,
        );
        outFunctionTree = initDAE.shared.functionTree.clone();
        return Ok((outJacobianMatrices, outFunctionTree));
    }
    match '__try0: {
        backendDAE_1 = unwrap_break_err!(BackendDAEUtil::copyBackendDAE(initDAE), '__try0);
        backendDAE_1 = unwrap_break_err!(BackendDAEOptimize::collapseIndependentBlocks(&backendDAE_1), '__try0);
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(backendDAE_1.clone()) {
            Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        currentSystem = metamodelica::Own::own(__pa1);
        shared = metamodelica::Own::own(__pa2);
        protectedCrefs = UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::hashComponentRef(&__a0)
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::crefEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        );
        for mut var in &**depVars {
            unwrap_break_err!(UnorderedSet::add(var.varName.clone(), protectedCrefs.clone()), '__try0);
            if BackendVariable::isParam(metamodelica::AsArg::as_arg(&var))
                && !(unwrap_break_err!(BackendVariable::varHasConstantBindExp(metamodelica::AsArg::as_arg(&var)), '__try0))
            {
                lhs = unwrap_break_err!(BackendVariable::varExp(metamodelica::AsArg::as_arg(&var)), '__try0);
                rhs = unwrap_break_err!(BackendVariable::varBindExpStartValueNoFail(metamodelica::AsArg::as_arg(&var)), '__try0);
                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                    exp: lhs.clone(),
                    scalar: rhs.clone(),
                    source: DAE::emptyElementSource().clone(),
                    attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
                });
                unwrap_break_err!(BackendEquation::add(eqn.clone(), currentSystem.orderedEqs.clone()), '__try0);
                if !(BackendVariable::containsCref(var.varName.clone(), &currentSystem.orderedVars)) {
                    currentSystem = unwrap_break_err!(BackendVariable::addVarDAE(unwrap_break_err!(BackendVariable::makeVar(var.varName.clone()), '__try0), currentSystem.clone()), '__try0);
                }
            }
        }
        newOrderedEquationArray = BackendEquation::emptyEqns();
        crefsVarsToRemove = UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::hashComponentRef(&__a0)
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::crefEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        );
        for mut eq in &*unwrap_break_err!(BackendEquation::equationList(currentSystem.orderedEqs.clone()), '__try0) {
            if !(BackendEquation::isAlgorithm(metamodelica::AsArg::as_arg(&eq))) {
                lhs = unwrap_break_err!(BackendEquation::getEquationLHS(metamodelica::AsArg::as_arg(&eq)), '__try0);
                rhs = unwrap_break_err!(BackendEquation::getEquationRHS(metamodelica::AsArg::as_arg(&eq)), '__try0);
                if Expression::isExpCref(&lhs) {
                    cr = unwrap_break_err!(Expression::expCref(&lhs), '__try0);
                    if ComponentReference::isStartCref(&cr) {
                        unwrap_break_err!(UnorderedSet::add(cr.clone(), crefsVarsToRemove.clone()), '__try0);
                    } else if Expression::isExpCref(&rhs)
                        && !(unwrap_break_err!(UnorderedSet::contains(cr.clone(), protectedCrefs.clone()), '__try0))
                    {
                        rhsCr = unwrap_break_err!(Expression::expCref(&rhs), '__try0);
                        if ComponentReference::isStartCref(&rhsCr)
                            && unwrap_break_err!(ComponentReferenceBasics::crefEqual(&(ComponentReference::popCref(rhsCr.clone())), &cr), '__try0)
                        {
                            unwrap_break_err!(UnorderedSet::add(cr.clone(), crefsVarsToRemove.clone()), '__try0);
                        } else {
                            unwrap_break_err!(BackendEquation::add(eq.clone(), newOrderedEquationArray.clone()), '__try0);
                        }
                    } else {
                        unwrap_break_err!(BackendEquation::add(eq.clone(), newOrderedEquationArray.clone()), '__try0);
                    }
                } else {
                    unwrap_break_err!(BackendEquation::add(eq.clone(), newOrderedEquationArray.clone()), '__try0);
                }
            } else {
                unwrap_break_err!(BackendEquation::add(eq.clone(), newOrderedEquationArray.clone()), '__try0);
            }
        }
        newVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
        for mut var in &*unwrap_break_err!(BackendVariable::varList(&currentSystem.orderedVars), '__try0) {
            let mut var = var.clone();
            if !(unwrap_break_err!(UnorderedSet::contains(var.varName.clone(), crefsVarsToRemove.clone()), '__try0)) {
                if unwrap_break_err!(UnorderedSet::contains(var.varName.clone(), protectedCrefs.clone()), '__try0) {
                    var = BackendVariable::setVarUnreplaceable(var.clone(), true);
                }
                newVars = unwrap_break_err!(BackendVariable::addVar(var.clone(), newVars.clone()), '__try0);
            }
        }
        currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem.clone(), newOrderedEquationArray.clone());
        currentSystem = BackendDAEUtil::setEqSystVars(currentSystem.clone(), newVars.clone());
        backendDAE_1 = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: list![currentSystem.clone()],
            shared: shared.clone(),
        });
        backendDAE_1 = unwrap_break_err!(BackendDAEOptimize::collapseIndependentBlocks(&backendDAE_1), '__try0);
        backendDAE_1 = unwrap_break_err!(BackendDAEUtil::transformBackendDAE(&backendDAE_1, Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT)), None, None), '__try0);
        states = metamodelica::nil();
        clockedStates = metamodelica::nil();
        for mut syst in &*simDAE.eqs.clone() {
            states = List::append_reverse(
                &(unwrap_break_err!(BackendVariable::getAllStateVarFromVariables(syst.orderedVars.clone()), '__try0)),
                states.clone(),
            );
            if unwrap_break_err!(Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone()), '__try0) {
                clockedStates = List::append_reverse(
                    &(unwrap_break_err!(BackendVariable::getAllClockedStatesFromVariables(syst.orderedVars.clone()), '__try0)),
                    clockedStates.clone(),
                );
            }
        }
        states = listAppend(states.clone().reverse(), clockedStates.clone().reverse());
        varlst = unwrap_break_err!(BackendVariable::varList(&currentSystem.orderedVars), '__try0);
        knvarlst = unwrap_break_err!(BackendVariable::varList(&simDAE.shared.globalKnownVars), '__try0);
        inputvars = unwrap_break_err!(List::select(knvarlst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndInput(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)), '__try0);
        paramvars = unwrap_break_err!(List::select(knvarlst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isParam(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)), '__try0);
        statesarr = unwrap_break_err!(BackendVariable::listVar1(&states), '__try0);
        inputvarsarr = unwrap_break_err!(BackendVariable::listVar1(&inputvars), '__try0);
        paramvarsarr = unwrap_break_err!(BackendVariable::listVar1(&paramvars), '__try0);
        depVarsArr = unwrap_break_err!(BackendVariable::listVar1(depVars), '__try0);
        (outJacobian, outFunctionTree, _, _, _) = unwrap_break_err!(generateGenericJacobian(&backendDAE_1, indepVars.clone(), &statesarr, &inputvarsarr, &paramvarsarr, depVarsArr.clone(), varlst.clone(), literal!("FMIDERINIT"), false, false), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpSparsityPattern(&sparsePattern_, &(literal!("FMI sparsity"))), '__try0);
        }
        outJacobianMatrices = metamodelica::cons(
            (
                outJacobian.clone(),
                sparsePattern_.clone(),
                sparseColoring_.clone(),
                BackendDAE::emptyNonlinearPattern().clone(),
            ),
            outJacobianMatrices.clone(),
        );
        outFunctionTree = unwrap_break_err!(AvlTreePathFunction::join(initDAE.shared.functionTree.clone(), &outFunctionTree, &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))), '__try0);
        Ok::<_, &'static str>((outFunctionTree.clone(), outJacobianMatrices.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outFunctionTree = __try0_o0;
            outJacobianMatrices = __try0_o1;
        }
        Err(_) => {
            Error::addInternalError(
                literal!("function createFMIModelDerivativesForInitialization failed"),
                metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"),
            )?;
            outJacobianMatrices = metamodelica::nil();
            outFunctionTree = initDAE.shared.functionTree.clone();
        }
    }
    Ok((outJacobianMatrices, outFunctionTree))
}

fn createLinearModelMatrices(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut useOptimica: bool,
) -> Result<(
    metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outJacobianMatrices: metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outJacobianMatrices, outFunctionTree) = (match useOptimica {
        false => {
            let mut backendDAE = inBackendDAE;
            let mut backendDAE2: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut knvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut inputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut inputvars2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut outputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut paramvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut v: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut statesarr: BackendDAE::Variables;
            let mut inputvarsarr: BackendDAE::Variables;
            let mut paramvarsarr: BackendDAE::Variables;
            let mut outputvarsarr: BackendDAE::Variables;
            let mut linearModelMatrices: metamodelica::List<(
                Option<(
                    metamodelica::Ref<BackendDAE::BackendDAE>,
                    ArcStr,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                (
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    (
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    ),
                    i32,
                ),
                metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
                (
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    (
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    ),
                    i32,
                ),
            )>;
            let mut linearModelMatrix: Option<(
                metamodelica::Ref<BackendDAE::BackendDAE>,
                ArcStr,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>;
            let mut sparsePattern: (
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                (
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                ),
                i32,
            );
            let mut sparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
            let mut nonlinearPattern: (
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                (
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                ),
                i32,
            );
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            backendDAE2 = BackendDAEUtil::copyBackendDAE(&backendDAE)?;
            backendDAE2 = BackendDAEOptimize::collapseIndependentBlocks(&backendDAE2)?;
            backendDAE2 = BackendDAEUtil::transformBackendDAE(
                &backendDAE2,
                Some((
                    openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION,
                    openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT,
                )),
                None,
                None,
            )?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backendDAE2.clone()) {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: Deref @ BackendDAE::Shared { globalKnownVars: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            v = metamodelica::Own::own(__pa0);
            globalKnownVars = metamodelica::Own::own(__pa1);
            varlst = BackendVariable::varList(&v)?;
            knvarlst = BackendVariable::varList(&globalKnownVars)?;
            states = BackendVariable::getAllStateVarFromVariables(v)?;
            inputvars = List::select(
                knvarlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isInput(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            paramvars = List::select(
                knvarlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isParam(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            inputvars2 = List::select(
                knvarlst,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndInput(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            outputvars = List::select(
                varlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndOutput(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            statesarr = BackendVariable::listVar1(&states)?;
            inputvarsarr = BackendVariable::listVar1(&inputvars)?;
            paramvarsarr = BackendVariable::listVar1(&paramvars)?;
            outputvarsarr = BackendVariable::listVar1(&outputvars)?;
            (
                linearModelMatrix,
                functionTree,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                states.clone(),
                &(statesarr.clone()),
                &inputvarsarr,
                &paramvarsarr,
                statesarr.clone(),
                varlst.clone(),
                literal!("A"),
                false,
                false,
            )?;
            backendDAE2 = BackendDAEUtil::setFunctionTree(&backendDAE2, functionTree.clone());
            linearModelMatrices = list![(linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern)];
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix A time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (
                linearModelMatrix,
                funcs,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                inputvars2.clone(),
                &(statesarr.clone()),
                &inputvarsarr,
                &paramvarsarr,
                statesarr.clone(),
                varlst.clone(),
                literal!("B"),
                false,
                false,
            )?;
            functionTree = AvlTreePathFunction::join(
                functionTree,
                &funcs,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            backendDAE2 = BackendDAEUtil::setFunctionTree(&backendDAE2, functionTree.clone());
            linearModelMatrices = metamodelica::cons(
                (linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern),
                linearModelMatrices,
            );
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix B time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (
                linearModelMatrix,
                funcs,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                states,
                &statesarr,
                &inputvarsarr,
                &paramvarsarr,
                outputvarsarr.clone(),
                varlst.clone(),
                literal!("C"),
                false,
                false,
            )?;
            functionTree = AvlTreePathFunction::join(
                functionTree,
                &funcs,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            backendDAE2 = BackendDAEUtil::setFunctionTree(&backendDAE2, functionTree.clone());
            linearModelMatrices = metamodelica::cons(
                (linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern),
                linearModelMatrices,
            );
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix C time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (
                linearModelMatrix,
                funcs,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                inputvars2,
                &statesarr,
                &inputvarsarr,
                &paramvarsarr,
                outputvarsarr,
                varlst,
                literal!("D"),
                false,
                false,
            )?;
            functionTree = AvlTreePathFunction::join(
                functionTree,
                &funcs,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            linearModelMatrices = metamodelica::cons(
                (linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern),
                linearModelMatrices,
            );
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix D time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (linearModelMatrices.reverse(), functionTree)
        }
        true => {
            let mut backendDAE = inBackendDAE;
            let mut backendDAE2: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut knvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut inputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut inputvars2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut outputvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut paramvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut states_inputs: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut conVarsList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut fconVarsList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut object: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut v: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut statesarr: BackendDAE::Variables;
            let mut inputvarsarr: BackendDAE::Variables;
            let mut paramvarsarr: BackendDAE::Variables;
            let mut outputvarsarr: BackendDAE::Variables;
            let mut optimizer_vars: BackendDAE::Variables;
            let mut conVars: BackendDAE::Variables;
            let mut linearModelMatrices: metamodelica::List<(
                Option<(
                    metamodelica::Ref<BackendDAE::BackendDAE>,
                    ArcStr,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                (
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    (
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    ),
                    i32,
                ),
                metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
                (
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                    (
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    ),
                    i32,
                ),
            )>;
            let mut linearModelMatrix: Option<(
                metamodelica::Ref<BackendDAE::BackendDAE>,
                ArcStr,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>;
            let mut sparsePattern: (
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                (
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                ),
                i32,
            );
            let mut sparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
            let mut nonlinearPattern: (
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
                (
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                ),
                i32,
            );
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            backendDAE2 = BackendDAEUtil::copyBackendDAE(&backendDAE)?;
            backendDAE2 = BackendDAEOptimize::collapseIndependentBlocks(&backendDAE2)?;
            backendDAE2 = BackendDAEUtil::transformBackendDAE(
                &backendDAE2,
                Some((
                    openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION,
                    openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT,
                )),
                None,
                None,
            )?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backendDAE2.clone()) {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: Deref @ BackendDAE::Shared { globalKnownVars: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            v = metamodelica::Own::own(__pa0);
            globalKnownVars = metamodelica::Own::own(__pa1);
            varlst = BackendVariable::varList(&v)?;
            knvarlst = BackendVariable::varList(&globalKnownVars)?;
            states = BackendVariable::getAllStateVarFromVariables(v)?;
            inputvars = List::select(
                knvarlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isInput(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            paramvars = List::select(
                knvarlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isParam(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            inputvars2 = List::select(
                knvarlst,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndInputNoDerInput(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            outputvars = List::select(
                varlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndOutput(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            conVarsList = List::select(
                varlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isRealOptimizeConstraintsVars(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            fconVarsList = List::select(
                varlst.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isRealOptimizeFinalConstraintsVars(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
            )?;
            states_inputs = listAppend(states.clone(), inputvars2);
            statesarr = BackendVariable::listVar1(&states)?;
            inputvarsarr = BackendVariable::listVar1(&inputvars)?;
            paramvarsarr = BackendVariable::listVar1(&paramvars)?;
            outputvarsarr = BackendVariable::listVar1(&outputvars)?;
            conVars = BackendVariable::listVar1(&conVarsList)?;
            (
                linearModelMatrix,
                functionTree,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                states,
                &(statesarr.clone()),
                &inputvarsarr,
                &paramvarsarr,
                statesarr.clone(),
                varlst.clone(),
                literal!("A"),
                false,
                false,
            )?;
            backendDAE2 = BackendDAEUtil::setFunctionTree(&backendDAE2, functionTree.clone());
            linearModelMatrices = list![(linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern)];
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix A time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            optimizer_vars = BackendVariable::addVariables(statesarr.clone(), BackendVariable::copyVariables(conVars))?;
            object = DynamicOptimization::checkObjectIsSet(
                &outputvarsarr,
                arcstr::literal!(BackendDAE::optimizationLagrangeTermName),
            );
            optimizer_vars = BackendVariable::addVars(&object, optimizer_vars)?;
            (
                linearModelMatrix,
                funcs,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                states_inputs.clone(),
                &statesarr,
                &inputvarsarr,
                &paramvarsarr,
                optimizer_vars.clone(),
                varlst.clone(),
                literal!("B"),
                false,
                false,
            )?;
            functionTree = AvlTreePathFunction::join(
                functionTree,
                &funcs,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            backendDAE2 = BackendDAEUtil::setFunctionTree(&backendDAE2, functionTree.clone());
            linearModelMatrices = metamodelica::cons(
                (linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern),
                linearModelMatrices,
            );
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix B time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            object = DynamicOptimization::checkObjectIsSet(
                &outputvarsarr,
                arcstr::literal!(BackendDAE::optimizationMayerTermName),
            );
            optimizer_vars = BackendVariable::addVars(&object, optimizer_vars)?;
            (
                linearModelMatrix,
                funcs,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                states_inputs.clone(),
                &statesarr,
                &inputvarsarr,
                &paramvarsarr,
                optimizer_vars,
                varlst.clone(),
                literal!("C"),
                false,
                false,
            )?;
            functionTree = AvlTreePathFunction::join(
                functionTree,
                &funcs,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            backendDAE2 = BackendDAEUtil::setFunctionTree(&backendDAE2, functionTree.clone());
            linearModelMatrices = metamodelica::cons(
                (linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern),
                linearModelMatrices,
            );
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix C time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            optimizer_vars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
            optimizer_vars = BackendVariable::listVar1(&fconVarsList)?;
            (
                linearModelMatrix,
                funcs,
                sparsePattern,
                sparseColoring,
                nonlinearPattern,
            ) = generateGenericJacobian(
                &backendDAE2,
                states_inputs,
                &statesarr,
                &inputvarsarr,
                &paramvarsarr,
                optimizer_vars,
                varlst,
                literal!("D"),
                false,
                false,
            )?;
            functionTree = AvlTreePathFunction::join(
                functionTree,
                &funcs,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            linearModelMatrices = metamodelica::cons(
                (linearModelMatrix, sparsePattern, sparseColoring, nonlinearPattern),
                linearModelMatrices,
            );
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "analytical Jacobians -> generated system for matrix D time: "
                    ));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (linearModelMatrices.reverse(), functionTree)
        }
        _ => {
            Error::addInternalError(
                literal!("Generation of LinearModel Matrices failed."),
                metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok((outJacobianMatrices, outFunctionTree))
}

fn generateGenericJacobian(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inDiffVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inStateVars: &BackendDAE::Variables,
    mut inInputVars: &BackendDAE::Variables,
    mut inParameterVars: &BackendDAE::Variables,
    mut inDifferentiatedVars: BackendDAE::Variables,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inName: ArcStr,
    mut onlySparsePattern: bool,
    mut daeMode: bool,
) -> Result<(
    Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
)> {
    let mut outJacobian: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outSparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ) = BackendDAE::emptySparsePattern().clone();
    let mut outSparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> =
        metamodelica::nil();
    let mut nonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut symbolicJacobian: (
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    );
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = inBackendDAE.shared.clone();
    let mut jacDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut jacDiffedVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    match '__try0: {
        outFunctionTree = shared.functionTree.clone();
        if !(onlySparsePattern) {
            (symbolicJacobian, outFunctionTree) = unwrap_break_err!(createJacobian(inBackendDAE, inDiffVars.clone(), inStateVars, inInputVars, inParameterVars, inDifferentiatedVars.clone(), inVars.clone(), inName.clone(), daeMode), '__try0);
            let true = (unwrap_break_err!(checkForNonLinearStrongComponents(&symbolicJacobian), '__try0)) else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
            outJacobian = Some(symbolicJacobian.clone());
            (jacDAE, _, _, _, _, _) = symbolicJacobian.clone();
            jacDiffedVars = unwrap_break_err!(getJacobianResiduals(&jacDAE), '__try0);
            (nonlinearPattern, _) = unwrap_break_err!(generateSparsePattern(&(unwrap_break_err!(BackendDAEUtil::copyBackendDAE(&jacDAE), '__try0)), inDiffVars.clone(), jacDiffedVars.clone(), true, true), '__try0);
            nonlinearPattern = stripPartialDerNonlinearPattern(nonlinearPattern.clone());
        } else {
            outJacobian = None;
            nonlinearPattern = BackendDAE::emptyNonlinearPattern().clone();
        }
        if !(stringEq(&inName, &(literal!("FMIDERINIT")))) {
            (outSparsePattern, outSparseColoring) = unwrap_break_err!(generateSparsePattern(inBackendDAE, inDiffVars.clone(), unwrap_break_err!(BackendVariable::varList(&inDifferentiatedVars), '__try0), false, true), '__try0);
        }
        Ok::<_, &'static str>((nonlinearPattern.clone(), outFunctionTree.clone(), outJacobian.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            nonlinearPattern = __try0_o0;
            outFunctionTree = __try0_o1;
            outJacobian = __try0_o2;
        }
        Err(__try0_err) => {
            return Err(__try0_err);
        }
    }
    Ok((
        outJacobian,
        outFunctionTree,
        outSparsePattern,
        outSparseColoring,
        nonlinearPattern,
    ))
}

fn createJacobian(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inDiffVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inStateVars: &BackendDAE::Variables,
    mut inInputVars: &BackendDAE::Variables,
    mut inParameterVars: &BackendDAE::Variables,
    mut inDifferentiatedVars: BackendDAE::Variables,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inName: ArcStr,
    mut daeMode: bool,
) -> Result<(
    (
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outJacobian: (
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    );
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outJacobian, outFunctionTree) = 'mc: {
        let __mc_input = inName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut backendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut reducedDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut comref_vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut comref_differentiatedVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut dependencies: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut diffedVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut seedlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut indepVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            diffedVars = BackendVariable::varList(&inDifferentiatedVars)?;
            comref_differentiatedVars = List::map(diffedVars.clone(), &move |__a0: metamodelica::Ref<
                BackendDAE::Var,
            >|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
            })?;
            reducedDAE = BackendDAEUtil::reduceEqSystemsInDAE(
                inBackendDAE,
                diffedVars.clone(),
                true,
                !(Flags::getConfigBool(Flags::CAUSALIZE_DAE_MODE.clone())?),
            )?;
            (indepVars, _) = createInDepVars(&inDiffVars, false)?;
            comref_vars = List::map(
                inDiffVars.clone(),
                &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
                },
            )?;
            seedlst = List::map1(
                comref_vars.clone(),
                &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: ArcStr| createSeedVars(&__a0, &__a1),
                inName.clone(),
            )?;
            if Flags::isSet(Flags::JAC_DUMP.clone())? {
                metamodelica::print(literal!("Create symbolic Jacobians from:\n"));
                metamodelica::print(BackendDump::varListString(
                    &indepVars,
                    &(literal!("Independent Variables")),
                )?);
                metamodelica::print(BackendDump::varListString(
                    &diffedVars,
                    &(literal!("Dependent Variables")),
                )?);
                metamodelica::print(literal!("Basic equation system:\n"));
                metamodelica::print(BackendDump::equationListString(
                    &(BackendEquation::equationSystemsEqnsLst(&reducedDAE.eqs)?),
                    &(literal!("differentiated equations")),
                )?);
                metamodelica::print(BackendDump::varListString(
                    &(BackendVariable::equationSystemsVarsLst(&reducedDAE.eqs)?),
                    &(literal!("related variables")),
                )?);
                metamodelica::print(BackendDump::varListString(
                    &(BackendVariable::varList(&reducedDAE.shared.globalKnownVars)?),
                    &(literal!("known variables")),
                )?);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(generateSymbolicJacobian(&reducedDAE, indepVars.clone(), inDifferentiatedVars.clone(), BackendVariable::listVar1(&seedlst)?, inStateVars, inInputVars, inParameterVars, inName.clone(), daeMode)?) {
                (__pa0 @ Deref @ BackendDAE::BackendDAE { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => unreachable!(),
            } };
            backendDAE = metamodelica::Own::own(__pa0);
            funcs = metamodelica::Own::own(__pa1);
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("analytical Jacobians -> generated equations for Jacobian "));
                    __mm_s.push_str(&*inName);
                    __mm_s.push_str(&*literal!(" time: "));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            backendDAE = BackendDAEUtil::setFunctionTree(&backendDAE, funcs.clone());
            backendDAE = optimizeJacobianMatrix(backendDAE.clone(), &comref_differentiatedVars, &comref_vars)?;
            if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("analytical Jacobians -> generated Jacobian DAE time: "));
                    __mm_s.push_str(&*realString(clock()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            dependencies = calcJacobianDependencies(
                &((
                    backendDAE.clone(),
                    literal!(""),
                    metamodelica::nil(),
                    metamodelica::nil(),
                    metamodelica::nil(),
                    metamodelica::nil(),
                )),
            )?;
            Ok((
                (
                    backendDAE.clone(),
                    inName.clone(),
                    inDiffVars.clone(),
                    diffedVars.clone(),
                    inVars.clone(),
                    dependencies.clone(),
                ),
                funcs.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("function createJacobian failed"),
                metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outJacobian, outFunctionTree))
}

fn optimizeJacobianMatrix(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inComRef1: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inComRef2: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outJacobian: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut ea: metamodelica::Array<i32> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut eMatching: metamodelica::Ref<BackendDAE::Matching> =
        metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
            ass1: ea.clone(),
            ass2: ea.clone(),
            comps: metamodelica::nil(),
        });
    outJacobian = ({
        let mut b: bool = false;
        'mc: {
            let __mc_input = (inBackendDAE, &**inComRef1, &**inComRef2);
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst, tail: Deref @ metamodelica::ListNode::Nil }, shared }, Deref @ metamodelica::ListNode::Nil, _) => {
                        let mut syst = (*syst).clone();
                        assign_field!(
                            syst.orderedVars = BackendVariable::listVar(metamodelica::nil())?,
                            syst.matching = eMatching.clone()
                        );
                        Ok(metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(syst.clone(), metamodelica::nil()), shared: shared.clone() }))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst, tail: Deref @ metamodelica::ListNode::Nil }, shared }, _, Deref @ metamodelica::ListNode::Nil) => {
                        let mut syst = (*syst).clone();
                        assign_field!(
                            syst.orderedVars = BackendVariable::listVar(metamodelica::nil())?,
                            syst.matching = eMatching.clone()
                        );
                        Ok(metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(syst.clone(), metamodelica::nil()), shared: shared.clone() }))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (backendDAE, _, _) => {
                        let mut backendDAE2: metamodelica::Ref<BackendDAE::BackendDAE>;
                        let mut strPostOptModules: metamodelica::List<ArcStr>;
                        if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians -> optimize jacobians time: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        if Flags::isSet(Flags::JAC_DUMP.clone())? {
                            BackendDump::bltdump(literal!("Symbolic Jacobian"), metamodelica::AsArg::as_arg(&backendDAE))?;
                        } else {
                            b = FlagsUtil::disableDebug(Flags::EXEC_STAT.clone())?;
                        }
                        strPostOptModules = list![literal!("wrapFunctionCalls"), literal!("inlineArrayEqn"), literal!("constantLinearSystem"), literal!("solveSimpleEquations"), literal!("tearingSystem"), literal!("calculateStrongComponentJacobians"), literal!("removeConstants"), literal!("simplifyTimeIndepFuncCalls")];
                        if Flags::isSet(Flags::SPLIT_CONSTANT_PARTS_SYMJAC.clone())? {
                            strPostOptModules = List::insert(strPostOptModules.clone(), 4, literal!("removeSimpleEquations"))?;
                        }
                        backendDAE2 = BackendDAEUtil::getSolvedSystemforJacobians(backendDAE.clone(), &(list![literal!("removeEqualRHS"), literal!("removeSimpleEquations"), literal!("evalFunc")]), None, None, &strPostOptModules)?;
                        if Flags::isSet(Flags::JAC_DUMP.clone())? {
                            BackendDump::bltdump(literal!("Symbolic Jacobian"), &backendDAE2)?;
                        } else {
                            FlagsUtil::set(Flags::EXEC_STAT.clone(), b)?;
                        }
                        Ok(backendDAE2.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Error::addInternalError(literal!("function optimizeJacobianMatrix failed"), metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"))?;
                        Ok(return Err("fail"))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    });
    Ok(outJacobian)
}

fn generateSymbolicJacobian(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inDiffedVars: BackendDAE::Variables,
    mut inSeedVars: BackendDAE::Variables,
    mut inStateVars: &BackendDAE::Variables,
    mut inInputVars: &BackendDAE::Variables,
    mut inParamVars: &BackendDAE::Variables,
    mut inMatrixName: ArcStr,
    mut daeMode: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::BackendDAE>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outJacobian: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut outFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outJacobian, outFunctions) = 'mc: {
        let __mc_input = (&**inBackendDAE, inVars, inDiffedVars, inMatrixName);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::BackendDAE { shared: Deref @ BackendDAE::Shared { cache, graph, info: ei, functionTree: functions, .. }, .. }, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    let mut jacobian: metamodelica::Ref<BackendDAE::BackendDAE>;
                    jacobian = metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: list![BackendDAEUtil::createEqSystem(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendEquation::emptyEqns(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns())], shared: BackendDAEUtil::createEmptyShared(openmodelica_backend_types::BackendDAE::BackendDAEType::JACOBIAN, ei.clone(), cache.clone(), graph.clone())? });
                    Ok((jacobian.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars, orderedEqs, matching: Deref @ BackendDAE::Matching::MATCHING { ass2, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: Deref @ BackendDAE::Shared { globalKnownVars, cache, graph, functionTree: functions, info: ei, .. } }, diffVars, diffedVars, matrixName) => {
                    let mut comref_diffvars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut x: metamodelica::Ref<DAE::ComponentRef>;
                    let mut dummyVarName: ArcStr;
                    let mut diffVarsArr: BackendDAE::Variables;
                    let mut jacobian: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut jacOrderedVars: BackendDAE::Variables;
                    let mut jacKnownVars: BackendDAE::Variables;
                    let mut jacOrderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut derivedVariables: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut derivedEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut diffData: BackendDAE::DifferentiateInputData;
                    let mut size: i32;
                    let mut functions = (*functions).clone();
                    let mut diffVars = (*diffVars).clone();
                    dummyVarName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("dummyVar")); __mm_s.push_str(&*matrixName); ArcStr::from(__mm_s) };
                    x = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: dummyVarName.clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
                    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** analytical Jacobians -> derived all algorithms time: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    diffVarsArr = BackendVariable::listVar1(metamodelica::AsArg::as_arg(&diffVars))?;
                    comref_diffvars = List::map(diffVars.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?;
                    diffData = BackendDAE::emptyInputData().clone();
                    diffData.independenentVars = Some(diffVarsArr.clone());
                    diffData.dependenentVars = Some(diffedVars.clone());
                    diffData.knownVars = Some(globalKnownVars.clone());
                    diffData.allVars = Some(orderedVars.clone());
                    diffData.diffCrefs = comref_diffvars.clone();
                    diffData.matrixName = Some(matrixName.clone());
                    eqns = BackendEquation::equationList(orderedEqs.clone())?;
                    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** analytical Jacobians -> before derive all equation: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    (derivedEquations, functions) = deriveAll(&eqns, &(ass2.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), x.clone(), diffData.clone(), functions.clone(), daeMode)?;
                    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** analytical Jacobians -> after derive all equation: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    derivedEquations = BackendEquation::replaceDerOpInEquationList(&derivedEquations)?;
                    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** analytical Jacobians -> created all derived equation time: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    diffVars = BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?;
                    derivedVariables = createAllDiffedVars(diffVars.clone(), x.clone(), metamodelica::AsArg::as_arg(&diffedVars), metamodelica::AsArg::as_arg(&matrixName))?;
                    jacOrderedVars = BackendVariable::listVar1(&derivedVariables)?;
                    size = BackendVariable::varsSize(metamodelica::AsArg::as_arg(&orderedVars)) + BackendVariable::varsSize(metamodelica::AsArg::as_arg(&globalKnownVars)) + BackendVariable::varsSize(&inSeedVars);
                    jacKnownVars = BackendVariable::emptyVarsSized(size);
                    jacKnownVars = BackendVariable::addVariables(inSeedVars.clone(), jacKnownVars.clone())?;
                    (jacKnownVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(jacKnownVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::setVarDirectionTpl, metamodelica::Ref<BackendDAE::Var>, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, DAE::VarDirection) -> Result<(metamodelica::Ref<BackendDAE::Var>, DAE::VarDirection)> + 'static>), openmodelica_frontend_types::DAE::VarDirection::INPUT)?;
                    jacKnownVars = BackendVariable::addVariables(orderedVars.clone(), jacKnownVars.clone())?;
                    jacKnownVars = BackendVariable::addVariables(globalKnownVars.clone(), jacKnownVars.clone())?;
                    jacOrderedEqs = BackendEquation::listEquation(&derivedEquations)?;
                    shared = BackendDAEUtil::createEmptyShared(openmodelica_backend_types::BackendDAE::BackendDAEType::JACOBIAN, ei.clone(), cache.clone(), graph.clone())?;
                    jacobian = metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(BackendDAEUtil::createEqSystem(jacOrderedVars.clone(), jacOrderedEqs.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns()), metamodelica::nil()), shared: BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), jacKnownVars.clone()) });
                    Ok((jacobian.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.generateSymbolicJacobian")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outJacobian, outFunctions))
}

pub(crate) fn createSeedVars(
    mut indiffVar: &metamodelica::Ref<DAE::ComponentRef>,
    mut inMatrixName: &ArcStr,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outSeedVar: metamodelica::Ref<BackendDAE::Var>;
    let mut derivedCref: metamodelica::Ref<DAE::ComponentRef>;
    derivedCref = Differentiate::createSeedCrefName(indiffVar, inMatrixName)?;
    outSeedVar = metamodelica::Ref::new(BackendDAE::Var {
        varName: derivedCref.clone(),
        varKind: openmodelica_backend_types::BackendDAE::VarKind::STATE_DER,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::INPUT,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: ComponentReference::crefLastType(&derivedCref)?,
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
        unreplaceable: true,
        initNonlinear: false,
        encrypted: false,
    });
    Ok(outSeedVar)
}

fn createAllDiffedVars(
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inAllVars: &BackendDAE::Variables,
    mut inMatrixName: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    if let Ok(__iflet0) = createAllDiffedVarsWork(
        inVars.clone(),
        inCref.clone(),
        inAllVars,
        0,
        inMatrixName,
        metamodelica::nil(),
    ) {
        outVars = __iflet0;
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!("SymbolicJacobian.createAllDiffedVars failed")],
        )?;
        return Err("fail");
    }
    Ok(outVars)
}

fn createAllDiffedVarsWork<'__b>(
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inAllVars: &'__b BackendDAE::Variables,
    mut inIndex: i32,
    mut inMatrixName: &'__b ArcStr,
    mut iVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inVars, inCref, inIndex)) {
            (Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok(iVars.reverse())
            },
            (Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { varName: currVar, varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: restVar }, cref, index) => {
                let mut r1: metamodelica::Ref<BackendDAE::Var>;
                let mut derivedCref: metamodelica::Ref<DAE::ComponentRef>;
                let mut currVar = (*currVar).clone();
                let mut index = (*index).clone();
                match '__try0: {
                    unwrap_break_err!(BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&currVar), inAllVars), '__try0);
                    currVar = ComponentReference::crefPrefixDer(currVar.clone());
                    derivedCref = unwrap_break_err!(ComponentReference::createDifferentiatedCrefName(metamodelica::AsArg::as_arg(&currVar), cref.clone(), inMatrixName), '__try0);
                    r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                    r1 = unwrap_break_err!(BackendVariable::setVarKind(r1.clone(), openmodelica_backend_types::BackendDAE::VarKind::STATE_DER), '__try0);
                    assign_field!(r1.unreplaceable = true);
                    index = index.clone() + 1;
                    Ok::<_, &'static str>((currVar.clone(), derivedCref.clone(), r1.clone()))
                } {
                    Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                        currVar = __try0_o0;
                        derivedCref = __try0_o1;
                        r1 = __try0_o2;
                    }
                    Err(_) => {
                        currVar = ComponentReference::crefPrefixDer(currVar.clone());
                        derivedCref = ComponentReference::createDifferentiatedCrefName(metamodelica::AsArg::as_arg(&currVar), cref.clone(), inMatrixName)?;
                        r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                        r1 = BackendVariable::setVarKind(r1.clone(), openmodelica_backend_types::BackendDAE::VarKind::STATE_DER)?;
                    }
                }
                { (inVars, inCref, inAllVars, inIndex, inMatrixName, iVars) = (restVar.clone(), cref.clone(), inAllVars, index.clone(), inMatrixName, metamodelica::cons(r1, iVars)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { varName: currVar, .. }, tail: restVar }, cref, index) => {
                let mut r1: metamodelica::Ref<BackendDAE::Var>;
                let mut derivedCref: metamodelica::Ref<DAE::ComponentRef>;
                let mut index = (*index).clone();
                match '__try0: {
                    unwrap_break_err!(BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&currVar), inAllVars), '__try0);
                    derivedCref = unwrap_break_err!(ComponentReference::createDifferentiatedCrefName(metamodelica::AsArg::as_arg(&currVar), cref.clone(), inMatrixName), '__try0);
                    r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                    r1 = unwrap_break_err!(BackendVariable::setVarKind(r1.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE), '__try0);
                    assign_field!(r1.unreplaceable = true);
                    index = index.clone() + 1;
                    Ok::<_, &'static str>((derivedCref.clone(), r1.clone()))
                } {
                    Ok((__try0_o0, __try0_o1)) => {
                        derivedCref = __try0_o0;
                        r1 = __try0_o1;
                    }
                    Err(_) => {
                        derivedCref = ComponentReference::createDifferentiatedCrefName(metamodelica::AsArg::as_arg(&currVar), cref.clone(), inMatrixName)?;
                        r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                        r1 = BackendVariable::setVarKind(r1.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    }
                }
                { (inVars, inCref, inAllVars, inIndex, inMatrixName, iVars) = (restVar.clone(), cref.clone(), inAllVars, index.clone(), inMatrixName, metamodelica::cons(r1, iVars)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn deriveAll(
    mut inEquations: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut ass2: &metamodelica::List<i32>,
    mut inDiffCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inDiffData: BackendDAE::DifferentiateInputData,
    mut inFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut daeMode: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDerivedEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut outFunctions: metamodelica::Ref<AvlTreePathFunction::Tree> = inFunctions;
    let mut allVars: BackendDAE::Variables;
    let mut currDerivedEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut tmpEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(inDiffData.clone()) {
            BackendDAE::DifferentiateInputData { allVars: Some(__pa1), .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        allVars = metamodelica::Own::own(__pa1);
        for mut currEquation in &**inEquations {
            (currDerivedEquation, outFunctions) = unwrap_break_err!(Differentiate::differentiateEquation(metamodelica::AsArg::as_arg(&currEquation), inDiffCref.clone(), &inDiffData, BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: daeMode }, outFunctions.clone()), '__try0);
            tmpEquations = unwrap_break_err!(BackendEquation::scalarComplexEquations(currDerivedEquation.clone(), &outFunctions), '__try0);
            outDerivedEquations = listAppend(tmpEquations.clone(), outDerivedEquations.clone());
        }
        outDerivedEquations = outDerivedEquations.clone().reverse();
        Ok::<_, &'static str>((allVars.clone(), outDerivedEquations.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            allVars = __try0_o0;
            outDerivedEquations = __try0_o1;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("SymbolicJacobian.deriveAll failed")],
            )?;
            return Err(__try0_err);
        }
    }
    Ok((outDerivedEquations, outFunctions))
}

pub(crate) fn getJacobianMatrixbyName<'__b>(
    mut injacobianMatrices: &'__b metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )>,
    mut inJacobianName: &'__b ArcStr,
) -> Option<(
    Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match injacobianMatrices {
            Deref @ metamodelica::ListNode::Cons { head: matrix @ (Some((_, name, _, _, _, _)), _, _, _), tail: _ } if (stringEq(&name, &inJacobianName)) => {
                return Some(matrix.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (injacobianMatrices, inJacobianName) = (rest, inJacobianName); continue '__tco; }
            },
            _ => {
                return None
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn updateJacobianDependencies(
    mut jacobian: metamodelica::Ref<BackendDAE::Jacobian>,
) -> Result<metamodelica::Ref<BackendDAE::Jacobian>> {
    let mut jacobian: metamodelica::Ref<BackendDAE::Jacobian> = jacobian;
    jacobian = (::match_deref::match_deref! { match &(jacobian.clone()) {
        jac @ Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { .. } => {
            let mut symJac: (metamodelica::Ref<BackendDAE::BackendDAE>, ArcStr, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>);
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut name: ArcStr;
            let mut diffVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut diffedVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut allDiffedVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut dependencies: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut jac = (*jac).clone();
            let (__pa7, __pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(var_field!((*jac).jacobian, BackendDAE::Jacobian::GENERIC_JACOBIAN).clone()) {
                Some(__pa7 @ (Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa1 }, __pa2, __pa3, __pa4, __pa5, __pa6)) => (__pa7.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => return Err("pattern mismatch"),
            } };
            syst = metamodelica::Own::own(__pa0);
            shared = metamodelica::Own::own(__pa1);
            name = metamodelica::Own::own(__pa2);
            diffVars = metamodelica::Own::own(__pa3);
            diffedVars = metamodelica::Own::own(__pa4);
            allDiffedVars = metamodelica::Own::own(__pa5);
            dependencies = metamodelica::Own::own(__pa6);
            symJac = metamodelica::Own::own(__pa7);
            dependencies = calcJacobianDependencies(&symJac)?;
            assign_variant_field!(jac => BackendDAE::Jacobian::GENERIC_JACOBIAN; jacobian = Some((metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: list![syst], shared: shared }), name, diffVars, diffedVars, allDiffedVars, dependencies)));
            jac.clone()
        },
        _ => {
            jacobian
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(jacobian)
}

pub(crate) fn calcJacobianDependencies(
    mut jacobian: &(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut dependencies: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut systems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let (__t2, _, _, _, _, _) = jacobian.clone();
    let __arc3 = __t2.clone();
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &*__arc3;
    systems = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    syst = (systems).head().cloned()?;
    dependencies = BackendEquation::getCrefsFromEquations(
        syst.orderedEqs.clone(),
        syst.orderedVars.clone(),
        shared.globalKnownVars.clone(),
    )?;
    Ok(dependencies)
}

pub(crate) fn getJacobianDependencies(
    mut jacobian: &metamodelica::Ref<BackendDAE::Jacobian>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut dependencies: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    dependencies = (::match_deref::match_deref! { match jacobian {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, _, _, _, _, __esc_dependencies)), .. } => {
            dependencies = (*__esc_dependencies).clone();
            dependencies.clone()
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: None, .. } => metamodelica::nil(),
        _ => {
            Error::addInternalError(literal!("function getJacobianDependencies failed"), metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dependencies)
}

// =============================================================================
// Module for to calculate strong component Jacobains
//
// =============================================================================
fn calculateEqSystemJacobians(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    (outSyst, outShared) = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, matching: Deref @ BackendDAE::Matching::MATCHING { ass1, ass2, comps }, .. } => {
            let mut shared = inShared;
            let mut syst = (*syst).clone();
            let mut comps = (*comps).clone();
            (comps, shared) = calculateJacobiansComponents(comps.clone(), vars.clone(), eqns.clone(), shared)?;
            assign_field!(syst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }));
            (syst.clone(), shared)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outSyst, outShared))
}

fn calculateJacobiansComponents(
    mut inComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    outComps = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = metamodelica::nil();
        for mut component in (inComps).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(component.clone()) {
                comp => {
                    let mut comp = (*comp).clone();
                    (comp, outShared) = calculateJacobianComponent(comp.clone(), inVars.clone(), inEqns.clone(), outShared.clone())?;
                    comp.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((outComps, outShared))
}

pub(crate) fn prepareTornStrongComponentData(
    mut inVars: &BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inIterationvarsInts: metamodelica::List<i32>,
    mut inResidualequations: metamodelica::List<i32>,
    mut innerEquations: &metamodelica::List<BackendDAE::InnerEquation>,
    mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut name: &ArcStr,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut outDiffVars: BackendDAE::Variables;
    let mut outResidualVars: BackendDAE::Variables;
    let mut outOtherVars: BackendDAE::Variables;
    let mut outResidualEqns: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut outOtherEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut iterationvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut resVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut ovarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut otherEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut otherVarsIntsLst: metamodelica::List<metamodelica::List<i32>>;
    let mut otherEqnsInts: metamodelica::List<i32>;
    let mut otherVarsInts: metamodelica::List<i32>;
    match '__try0: {
        iterationvars = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut e in (inIterationvarsInts.clone()).into_iter().cloned() {
                let __x = BackendVariable::transformXToXd(
                    unwrap_break_err!(BackendVariable::getVarAt(inVars, e.clone()), '__try0),
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outDiffVars = unwrap_break_err!(BackendVariable::listVar1(&iterationvars), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALGLOOP_JACOBIAN.clone()), '__try0) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("*** got iteration variables at time: "));
                __mm_s.push_str(&*realString(clock()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            unwrap_break_err!(BackendDump::printVarList(&iterationvars), '__try0);
        }
        reqns = unwrap_break_err!(BackendEquation::getList(inResidualequations.clone(), inEqns.clone()), '__try0);
        reqns = unwrap_break_err!(BackendEquation::replaceDerOpInEquationList(&reqns), '__try0);
        outResidualEqns = unwrap_break_err!(BackendEquation::listEquation(&reqns), '__try0);
        (_, reqns) = unwrap_break_err!(BackendEquation::traverseEquationArray(outResidualEqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (metamodelica::Ref<AvlTreePathFunction::Tree>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>)| BackendEquation::traverseEquationToScalarResidualForm(__a0, &__a1), (funcTree.clone(), metamodelica::nil())), '__try0);
        reqns = reqns.clone().reverse();
        (reqns, resVarsLst, _) = unwrap_break_err!(BackendEquation::convertResidualsIntoSolvedEquations(&reqns, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$res_")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("_")); ArcStr::from(__mm_s) }), 1, false), '__try0);
        outResidualVars = unwrap_break_err!(BackendVariable::listVar1(&resVarsLst), '__try0);
        outResidualEqns = unwrap_break_err!(BackendEquation::listEquation(&reqns), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALGLOOP_JACOBIAN.clone()), '__try0) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "*** got residual equation and created corresponding variables at time: "
                ));
                __mm_s.push_str(&*realString(clock()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("Equations:\n"));
            unwrap_break_err!(BackendDump::printEquationList(&reqns), '__try0);
        }
        (otherEqnsInts, otherVarsIntsLst, _) = unwrap_break_err!(List::map_3(innerEquations, &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) }), '__try0);
        otherEqnsLst = unwrap_break_err!(BackendEquation::getList(otherEqnsInts.clone(), inEqns.clone()), '__try0);
        otherEqnsLst = unwrap_break_err!(BackendEquation::replaceDerOpInEquationList(&otherEqnsLst), '__try0);
        outOtherEqns = unwrap_break_err!(BackendEquation::listEquation(&otherEqnsLst), '__try0);
        otherVarsInts = unwrap_break_err!(List::flatten(otherVarsIntsLst.clone()), '__try0);
        ovarsLst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut e in (otherVarsInts.clone()).into_iter().cloned() {
                let __x = BackendVariable::transformXToXd(
                    unwrap_break_err!(BackendVariable::getVarAt(inVars, e.clone()), '__try0),
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outOtherVars = unwrap_break_err!(BackendVariable::listVar1(&ovarsLst), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALGLOOP_JACOBIAN.clone()), '__try0) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "*** got residual equation and created corresponding variables at time: "
                ));
                __mm_s.push_str(&*realString(clock()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("other Equations:\n"));
            unwrap_break_err!(BackendDump::printEquationList(&otherEqnsLst), '__try0);
            metamodelica::print(literal!("other Variables:\n"));
            unwrap_break_err!(BackendDump::printVarList(&ovarsLst), '__try0);
        }
        Ok::<_, &'static str>((
            iterationvars.clone(),
            otherEqnsInts.clone(),
            otherEqnsLst.clone(),
            otherVarsInts.clone(),
            otherVarsIntsLst.clone(),
            outDiffVars.clone(),
            outOtherEqns.clone(),
            outOtherVars.clone(),
            outResidualEqns.clone(),
            outResidualVars.clone(),
            ovarsLst.clone(),
            reqns.clone(),
            resVarsLst.clone(),
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
        )) => {
            iterationvars = __try0_o0;
            otherEqnsInts = __try0_o1;
            otherEqnsLst = __try0_o2;
            otherVarsInts = __try0_o3;
            otherVarsIntsLst = __try0_o4;
            outDiffVars = __try0_o5;
            outOtherEqns = __try0_o6;
            outOtherVars = __try0_o7;
            outResidualEqns = __try0_o8;
            outResidualVars = __try0_o9;
            ovarsLst = __try0_o10;
            reqns = __try0_o11;
            resVarsLst = __try0_o12;
        }
        Err(__try0_err) => {
            return Err(__try0_err);
        }
    }
    Ok((
        outDiffVars,
        outResidualVars,
        outOtherVars,
        outResidualEqns,
        outOtherEqns,
    ))
}

fn checkForSymbolicJacobian(
    mut inResidualEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inOtherEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut name: &ArcStr,
) -> Result<bool> {
    let mut out: bool;
    let mut b1: bool;
    let mut b2: bool;
    if !(Flags::isSet(Flags::FORCE_NLS_ANALYTIC_JACOBIAN.clone())?) {
        match '__try0: {
            (b1, _) = unwrap_break_err!(BackendEquation::traverseExpsOfEquationList_WithStop(inResidualEqns, &traverserhasEqnNonDiffParts, (metamodelica::nil(), true, false)), '__try0);
            (b2, _) = unwrap_break_err!(BackendEquation::traverseExpsOfEquationList_WithStop(inOtherEqns, &traverserhasEqnNonDiffParts, (metamodelica::nil(), true, false)), '__try0);
            if !(b1 && b2) {
                if unwrap_break_err!(Flags::isSet(Flags::FAILTRACE.clone()), '__try0) {
                    unwrap_break_err!(Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Skip symbolic jacobian for non-linear system ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }), '__try0);
                }
                out = false;
            } else {
                out = true;
            }
            Ok::<_, &'static str>((out.clone(),))
        } {
            Ok((__try0_o0,)) => {
                out = __try0_o0;
            }
            Err(_) => {
                out = false;
            }
        }
    } else {
        out = true;
    }
    Ok(out)
}

fn calculateTearingSetJacobian(
    mut inVars: &BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inTearingSet: &BackendDAE::TearingSet,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut isLinear: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::Jacobian>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outJacobian: metamodelica::Ref<BackendDAE::Jacobian>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut name: ArcStr;
    let mut prename: ArcStr;
    let mut debug: bool = false;
    let mut onlySparsePattern: bool = false;
    let mut diffVars: BackendDAE::Variables;
    let mut oVars: BackendDAE::Variables;
    let mut resVars: BackendDAE::Variables;
    let mut resEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut oEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    match '__try0: {
        if !(isLinear) && !(unwrap_break_err!(Flags::isSet(Flags::NLS_ANALYTIC_JACOBIAN.clone()), '__try0)) {
            onlySparsePattern = true;
        }
        if isLinear {
            prename = literal!("LS");
        } else {
            prename = literal!("NLS");
        }
        name = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prename);
            __mm_s.push_str(&*literal!("Jac"));
            __mm_s.push_str(&*intString(System::tmpTickIndex(
                Global::backendDAE_jacobianSeq.clone(),
            )));
            ArcStr::from(__mm_s)
        };
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("*** "));
                __mm_s.push_str(&*prename);
                __mm_s.push_str(&*literal!("-JAC *** start creating Jacobian for a torn system "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" of size "));
                __mm_s.push_str(&*intString(((inTearingSet.tearingvars).len() as i32)));
                __mm_s.push_str(&*literal!(" time: "));
                __mm_s.push_str(&*realString(clock()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        (diffVars, resVars, oVars, resEqns, oEqns) = unwrap_break_err!(prepareTornStrongComponentData(inVars, inEqns.clone(), inTearingSet.tearingvars.clone(), inTearingSet.residualequations.clone(), &inTearingSet.innerEquations, inShared.functionTree.clone(), &name), '__try0);
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("*** "));
                __mm_s.push_str(&*prename);
                __mm_s.push_str(&*literal!("-JAC *** prepared all data for differentiation at time: "));
                __mm_s.push_str(&*realString(clock()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if !(isLinear
            || unwrap_break_err!(checkForSymbolicJacobian(&(unwrap_break_err!(BackendEquation::equationList(resEqns.clone()), '__try0)), &(unwrap_break_err!(BackendEquation::equationList(oEqns.clone()), '__try0)), &name), '__try0))
        {
            onlySparsePattern = true;
        }
        (outJacobian, outShared) = unwrap_break_err!(getSymbolicJacobian(&diffVars, resEqns.clone(), resVars.clone(), oEqns.clone(), oVars.clone(), inShared.clone(), inVars, name.clone(), onlySparsePattern), '__try0);
        Ok::<_, &'static str>((
            diffVars.clone(),
            name.clone(),
            oEqns.clone(),
            oVars.clone(),
            outJacobian.clone(),
            outShared.clone(),
            prename.clone(),
            resEqns.clone(),
            resVars.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            diffVars = __try0_o0;
            name = __try0_o1;
            oEqns = __try0_o2;
            oVars = __try0_o3;
            outJacobian = __try0_o4;
            outShared = __try0_o5;
            prename = __try0_o6;
            resEqns = __try0_o7;
            resVars = __try0_o8;
        }
        Err(__try0_err) => {
            return Err(__try0_err);
        }
    }
    Ok((outJacobian, outShared))
}

fn calculateJacobianComponent(
    mut inComp: metamodelica::Ref<BackendDAE::StrongComponent>,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::StrongComponent>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outComp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    (outComp, outShared) = ({
        let mut onlySparsePattern: bool = true;
        'mc: {
            let __mc_input = inComp;
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: strictTearingset, casualTearingSet: optCasualTearingSet, linear, mixedSystem } => {
                        let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                        let mut jacobian: metamodelica::Ref<BackendDAE::Jacobian>;
                        let mut jacobianCausal: metamodelica::Ref<BackendDAE::Jacobian>;
                        let mut casualTearingSet: BackendDAE::TearingSet;
                        let mut strictTearingset = (*strictTearingset).clone();
                        let mut optCasualTearingSet = (*optCasualTearingSet).clone();
                        (jacobian, shared) = calculateTearingSetJacobian(&inVars, inEqns.clone(), metamodelica::AsArg::as_arg(&strictTearingset), inShared.clone(), linear.clone())?;
                        strictTearingset.jac = jacobian.clone();
                        if (optCasualTearingSet).is_some() {
                            casualTearingSet = Util::getOption(optCasualTearingSet.clone())?;
                            (jacobianCausal, shared) = calculateTearingSetJacobian(&inVars, inEqns.clone(), &casualTearingSet, shared.clone(), linear.clone())?;
                            casualTearingSet.jac = jacobianCausal.clone();
                            optCasualTearingSet = Some(casualTearingSet.clone());
                        }
                        Ok((metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: strictTearingset.clone(), casualTearingSet: optCasualTearingSet.clone(), linear: linear.clone(), mixedSystem: mixedSystem.clone() }), shared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_CONSTANT { .. }, .. } => {
                        Ok((comp.clone(), inShared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, eqns: residualequations, vars: iterationvarsInts, mixedSystem, .. } => {
                        if !((Flags::isSet(Flags::LS_ANALYTIC_JACOBIAN.clone())?)) { return Err("guard") }
                        let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                        let mut jacobian: metamodelica::Ref<BackendDAE::Jacobian>;
                        let mut strictTearingset: BackendDAE::TearingSet;
                        strictTearingset = BackendDAE::TearingSet { tearingvars: iterationvarsInts.clone(), residualequations: residualequations.clone(), innerEquations: metamodelica::nil(), jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN() };
                        (jacobian, shared) = calculateTearingSetJacobian(&inVars, inEqns.clone(), &strictTearingset, inShared.clone(), true)?;
                        strictTearingset.jac = jacobian.clone();
                        Ok((metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: strictTearingset.clone(), casualTearingSet: None, linear: true, mixedSystem: mixedSystem.clone() }), shared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. } => {
                        Ok((comp.clone(), inShared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: residualequations, vars: iterationvarsInts, mixedSystem, .. } => {
                        let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                        let mut iterationvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                        let mut resVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                        let mut diffVars: BackendDAE::Variables;
                        let mut ovars: BackendDAE::Variables;
                        let mut resVars: BackendDAE::Variables;
                        let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                        let mut oeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                        let mut jacobian: metamodelica::Ref<BackendDAE::Jacobian>;
                        let mut name: ArcStr;
                        name = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NLSJac")); __mm_s.push_str(&*intString(System::tmpTickIndex(Global::backendDAE_jacobianSeq.clone()))); ArcStr::from(__mm_s) };
                        iterationvars = List::map1r(iterationvarsInts.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), inVars.clone())?;
                        iterationvars = List::map(iterationvars.clone(), &fnptr!(BackendVariable::transformXToXd, metamodelica::Ref<BackendDAE::Var>))?;
                        iterationvars = iterationvars.clone().reverse();
                        diffVars = BackendVariable::listVar1(&iterationvars)?;
                        reqns = BackendEquation::getList(residualequations.clone(), inEqns.clone())?;
                        reqns = BackendEquation::replaceDerOpInEquationList(&reqns)?;
                        if checkForSymbolicJacobian(&reqns, &(metamodelica::nil()), &name)? && Flags::isSet(Flags::NLS_ANALYTIC_JACOBIAN.clone())? {
                            onlySparsePattern = false;
                        }
                        eqns = BackendEquation::listEquation(&reqns)?;
                        (_, reqns) = BackendEquation::traverseEquationArray(eqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (metamodelica::Ref<AvlTreePathFunction::Tree>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>)| BackendEquation::traverseEquationToScalarResidualForm(__a0, &__a1), (inShared.functionTree.clone(), metamodelica::nil()))?;
                        reqns = reqns.clone().reverse();
                        (reqns, resVarsLst, _) = BackendEquation::convertResidualsIntoSolvedEquations(&reqns, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$res_")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("_")); ArcStr::from(__mm_s) }), 1, false)?;
                        resVars = BackendVariable::listVar1(&resVarsLst)?;
                        eqns = BackendEquation::listEquation(&reqns)?;
                        oeqns = BackendEquation::listEquation(&(metamodelica::nil()))?;
                        ovars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
                        (jacobian, shared) = getSymbolicJacobian(&diffVars, eqns.clone(), resVars.clone(), oeqns.clone(), ovars.clone(), inShared.clone(), &inVars, name.clone(), onlySparsePattern)?;
                        Ok((metamodelica::Ref::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: residualequations.clone(), vars: iterationvarsInts.clone(), jac: jacobian.clone(), jacType: openmodelica_backend_types::BackendDAE::JacobianType::JAC_GENERIC, mixedSystem: mixedSystem.clone() }), shared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    comp => {
                        Ok((comp.clone(), inShared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    });
    if BackendDAEUtil::isInitializationDAE(&inShared) {
        if '__try0: {
            unwrap_break_err!(checkNonLinDependecies(&outComp, inEqns.clone()), '__try0);
            Ok::<(), &'static str>(())
        }
        .is_err()
        {
            Error::addInternalError(
                literal!(
                    "function calculateJacobianComponent failed to check all non-linear iteration variables for start values."
                ),
                metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"),
            )?;
        }
    }
    Ok((outComp, outShared))
}

fn checkNonLinDependecies(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<()> {
    let mut name: ArcStr;
    let mut msg: ArcStr;
    let mut existNonLin: bool;
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        let () = ({
            let mut eqnIndices: metamodelica::List<i32> = metamodelica::nil();
            (match &**inComp {
                BackendDAE::StrongComponent::TORNSYSTEM {
                    strictTearingSet:
                        BackendDAE::TearingSet {
                            jac,
                            residualequations: resIndices,
                            innerEquations,
                            ..
                        },
                    linear: false,
                    ..
                } => {
                    for mut eq in &*innerEquations.clone() {
                        eqnIndices = (match eq.clone() {
                            BackendDAE::InnerEquation::INNEREQUATION { eqn: mut idx, .. } => {
                                metamodelica::cons(idx.clone(), eqnIndices)
                            }
                            BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS { eqn: mut idx, .. } => {
                                metamodelica::cons(idx.clone(), eqnIndices)
                            }
                            _ => eqnIndices,
                        });
                    }
                    eqnIndices = listAppend(resIndices.clone(), eqnIndices);
                    printNonLinIterVarsAndEqs(metamodelica::AsArg::as_arg(&jac), &eqnIndices, inEqns)?;
                    ()
                }
                BackendDAE::StrongComponent::EQUATIONSYSTEM {
                    eqns: eqnIndices,
                    jac,
                    jacType: BackendDAE::JacobianType::JAC_NONLINEAR { .. },
                    ..
                } => {
                    printNonLinIterVarsAndEqs(jac, metamodelica::AsArg::as_arg(&eqnIndices), inEqns)?;
                    ()
                }
                _ => (),
            })
        });
    } else {
        (existNonLin, name) = (match &**inComp {
            BackendDAE::StrongComponent::TORNSYSTEM {
                strictTearingSet: BackendDAE::TearingSet { jac, .. },
                linear: false,
                ..
            } => existNonLinIterVars(metamodelica::AsArg::as_arg(&jac))?,
            BackendDAE::StrongComponent::EQUATIONSYSTEM {
                jac,
                jacType: BackendDAE::JacobianType::JAC_NONLINEAR { .. },
                ..
            } => existNonLinIterVars(jac)?,
            _ => (false, literal!("")),
        });
        if existNonLin {
            msg = literal!(
                "For more information set -d=initialization. In OMEdit Tools->Options->Simulation->Show additional information from the initialization process, in OMNotebook call setCommandLineOptions(\"-d=initialization\")"
            );
            Error::addMessage(Error::INITIALIZATION_ITERATION_VARIABLES.clone(), list![name, msg])?;
        }
    }
    Ok(())
}

fn existNonLinIterVars(mut jacobian_in: &metamodelica::Ref<BackendDAE::Jacobian>) -> Result<(bool, ArcStr)> {
    let mut existNonLin: bool;
    let mut jacName: ArcStr;
    (existNonLin, jacName) = ({
        let mut exist: bool = false;
        (::match_deref::match_deref! { match jacobian_in {
            Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, name, diffVars, _, _, dependentVarsCref)), .. } => {
                let mut varCref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
                let mut var: metamodelica::Ref<BackendDAE::Var> = <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
                for mut varCref in &*dependentVarsCref.clone() {
                    let mut varCref = varCref.clone();
                    for mut var in &*diffVars.clone() {
                        let mut var = var.clone();
                        if ComponentReferenceBasics::crefEqual(&varCref, &var.varName)? {
                            if !(BackendVariable::varHasStartValue(&var)) {
                                exist = true;
                                break;
                            }
                        }
                    }
                    if exist {
                        break;
                    }
                }
                (exist, name.clone())
            },
            _ => {
                (false, literal!(""))
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok((existNonLin, jacName))
}

fn printNonLinIterVarsAndEqs(
    mut jacobian: &metamodelica::Ref<BackendDAE::Jacobian>,
    mut eqnIndices: &metamodelica::List<i32>,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<()> {
    let () = ({
        let mut nonLin: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        let mut nonLinStart: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        let mut lin: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        (::match_deref::match_deref! { match jacobian {
            Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, shared: _ }, name, diffVars, _, allDiffedVars, dependentVarsCref)), .. } => {
                let mut varCref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
                let mut var: metamodelica::Ref<BackendDAE::Var> = <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
                for mut varCref in &*dependentVarsCref.clone() {
                    let mut varCref = varCref.clone();
                    for mut var in &*diffVars.clone() {
                        let mut var = var.clone();
                        if ComponentReferenceBasics::crefEqual(&varCref, &var.varName)? {
                            if !(BackendVariable::varHasStartValue(&var)) {
                                nonLin = metamodelica::cons(var, nonLin);
                            } else {
                                nonLinStart = metamodelica::cons(var, nonLinStart);
                            }
                        }
                    }
                }
                if !((nonLin).is_empty()) {
                    BackendDump::dumpVarList(&nonLin, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Nonlinear iteration variables with default zero start attribute in ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }))?;
                }
                if !((nonLinStart).is_empty()) {
                    BackendDump::dumpVarList(&nonLinStart, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Nonlinear iteration variables with predefined start attribute in ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }))?;
                }
                for mut var in &*allDiffedVars.clone() {
                    let mut var = var.clone();
                    if BackendVariable::varHasStartValue(&var) && !(BackendVariable::isVarDiscrete(&var)) {
                        lin = metamodelica::cons(var, lin);
                    }
                }
                if !((lin).is_empty()) {
                    BackendDump::dumpVarList(&lin, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Linear iteration variables with predefined start attributes that are unrelevant in ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }))?;
                }
                if !((nonLin).is_empty() && (nonLinStart).is_empty() && (lin).is_empty()) {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Info: Only non-linear iteration variables in non-linear eqation systems require start values.")); __mm_s.push_str(&*literal!(" All other start values have no influence on convergence and are ignored.")); __mm_s.push_str(&*if (Flags::isSet(Flags::DUMP_LOOPS.clone())?) {literal!("\n\n")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" Use \"-d=dumpLoops\" to show all loops. In OMEdit Tools->Options->Simulation->Additional Translation Flags,")); __mm_s.push_str(&*literal!(" in OMNotebook call setCommandLineOptions(\"-d=dumpLoops\")\n\n")); ArcStr::from(__mm_s) }}); ArcStr::from(__mm_s) });
                }
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(())
}

pub(crate) fn getNonLinearVariables(
    mut jacobian: &metamodelica::Ref<BackendDAE::Jacobian>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut nonLin: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    nonLin = (::match_deref::match_deref! { match jacobian {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, _, diffVars, _, _, dependentVarsCref)), .. } => {
            for mut varCref in &*dependentVarsCref.clone() {
                for mut var in &*diffVars.clone() {
                    let mut var = var.clone();
                    if ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&varCref), &var.varName)? {
                        assign_field!(var.initNonlinear = true);
                        nonLin = metamodelica::cons(var, nonLin);
                        break;
                    }
                }
            }
            nonLin
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(nonLin)
}

fn traverserhasEqnNonDiffParts(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, bool) = inTpl.clone();
    let mut expList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (__pa0, (__pa1, __pa2, _)) = Expression::traverseExpTopDown(inExp, &hasEqnNonDiffParts, inTpl)?;
    outExp = metamodelica::Own::own(__pa0);
    expList = metamodelica::Own::own(__pa1);
    cont = metamodelica::Own::own(__pa2);
    if Flags::isSet(Flags::DUMP_EXCLUDED_EXP.clone())? && !(cont) {
        metamodelica::print(literal!(
            "Traverser for catching functions, that should not be differentiated\n"
        ));
        metamodelica::print(stringDelimitList(
            List::map(expList, &ExpressionBasics::printExpStr)?,
            literal!("\n"),
        ));
        metamodelica::print(literal!("\n\n"));
    }
    Ok((outExp, cont, outTpl))
}

fn hasEqnNonDiffParts(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, (expLst, _, insideCall)) => {
            (inExp.clone(), false, (metamodelica::cons(inExp, expLst.clone()), false, insideCall.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, (expLst, _, insideCall)) => {
            (inExp.clone(), false, (metamodelica::cons(inExp, expLst.clone()), false, insideCall.clone()))
        },
        (__esc_outExp, (_, b, _)) => {
            outExp = (*__esc_outExp).clone();
            (outExp.clone(), b.clone(), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn isRecordInvoled<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> Result<bool> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_COMPLEX { .. } => return Ok(true),
            DAE::Type::T_ARRAY { ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            DAE::Type::T_TUPLE { types, .. } => {
                return Ok(List::any(types, &move |__a0: metamodelica::Ref<DAE::Type>| {
                    isRecordInvoled(&__a0)
                })?);
            }
            _ => return Ok(false),
        }
    }
}

pub fn getSymbolicJacobian(
    mut inDiffVars: &BackendDAE::Variables,
    mut inResEquations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inResVars: BackendDAE::Variables,
    mut inotherEquations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inotherVars: BackendDAE::Variables,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAllVars: &BackendDAE::Variables,
    mut inName: ArcStr,
    mut inOnlySparsePattern: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::Jacobian>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outJacobian: metamodelica::Ref<BackendDAE::Jacobian>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut backendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut einfo: BackendDAE::ExtraInfo;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut sparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut sparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut nonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    );
    let mut dependentVars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut cache: FCore::Cache;
    let mut graph: FCore::Graph;
    let mut knvarLst1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knvarLst2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut independentVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut dependentVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut otherVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut independentComRefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut otherVarsLstComRefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut symJacBDAE: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    match '__try0: {
        globalKnownVars = BackendDAEUtil::getGlobalKnownVarsFromShared(&inShared);
        funcs = BackendDAEUtil::getFunctions(&inShared);
        einfo = BackendDAEUtil::getExtraInfo(&inShared);
        if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
            metamodelica::print(literal!("---+++ create analytical jacobian +++---"));
            metamodelica::print(literal!("\n---+++ independent variables +++---\n"));
            unwrap_break_err!(BackendDump::printVariables(inDiffVars), '__try0);
            metamodelica::print(literal!("\n---+++ equation system +++---\n"));
            unwrap_break_err!(BackendDump::printEquationArray(inResEquations.clone()), '__try0);
        }
        independentVarsLst = unwrap_break_err!(BackendVariable::varList(inDiffVars), '__try0);
        independentComRefs = unwrap_break_err!(List::map(independentVarsLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) }), '__try0);
        otherVarsLst = unwrap_break_err!(BackendVariable::varList(&inotherVars), '__try0);
        otherVarsLstComRefs = unwrap_break_err!(List::map(otherVarsLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) }), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
            metamodelica::print(literal!("\n---+++ known variables +++---\n"));
            unwrap_break_err!(BackendDump::printVariables(&globalKnownVars), '__try0);
        }
        dependentVars =
            unwrap_break_err!(BackendVariable::mergeVariables(inResVars.clone(), inotherVars.clone(), true), '__try0);
        eqns = unwrap_break_err!(BackendEquation::merge(inResEquations.clone(), inotherEquations.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
            metamodelica::print(literal!("\n---+++ created backend system +++---\n"));
            metamodelica::print(literal!("\n---+++ vars +++---\n"));
            unwrap_break_err!(BackendDump::printVariables(&dependentVars), '__try0);
            metamodelica::print(literal!("\n---+++ equations +++---\n"));
            unwrap_break_err!(BackendDump::printEquationArray(eqns.clone()), '__try0);
        }
        knvarLst1 = unwrap_break_err!(BackendEquation::equationsVars(eqns.clone(), globalKnownVars.clone()), '__try0);
        knvarLst2 = metamodelica::nil();
        globalKnownVars = unwrap_break_err!(BackendVariable::listVar2(&knvarLst1, &knvarLst2), '__try0);
        globalKnownVars =
            unwrap_break_err!(BackendVariable::removeCrefs(&independentComRefs, globalKnownVars.clone()), '__try0);
        globalKnownVars =
            unwrap_break_err!(BackendVariable::removeCrefs(&otherVarsLstComRefs, globalKnownVars.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
            metamodelica::print(literal!("\n---+++ known variables +++---\n"));
            unwrap_break_err!(BackendDump::printVariables(&globalKnownVars), '__try0);
        }
        cache = FCore::emptyCache();
        graph = FGraph::empty();
        shared = unwrap_break_err!(BackendDAEUtil::createEmptyShared(openmodelica_backend_types::BackendDAE::BackendDAEType::ALGEQSYSTEM, einfo.clone(), cache.clone(), graph.clone()), '__try0);
        shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), globalKnownVars.clone());
        shared = BackendDAEUtil::setSharedFunctionTree(shared.clone(), funcs.clone());
        backendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: list![BackendDAEUtil::createEqSystem(
                dependentVars.clone(),
                eqns.clone(),
                metamodelica::nil(),
                openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                BackendEquation::emptyEqns()
            )],
            shared: shared.clone(),
        });
        if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP2.clone()), '__try0) {
            unwrap_break_err!(BackendDump::bltdump(literal!("System"), &backendDAE), '__try0);
        }
        backendDAE = unwrap_break_err!(BackendDAEUtil::transformBackendDAE(&backendDAE, Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT)), None, None), '__try0);
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(backendDAE.clone()) {
            Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, shared: Deref @ BackendDAE::Shared { globalKnownVars: __pa2, .. } } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        dependentVars = metamodelica::Own::own(__pa1);
        globalKnownVars = metamodelica::Own::own(__pa2);
        dependentVarsLst = unwrap_break_err!(BackendVariable::varList(&dependentVars), '__try0);
        (symJacBDAE, funcs, sparsePattern, sparseColoring, nonlinearPattern) = unwrap_break_err!(generateGenericJacobian(&backendDAE, independentVarsLst.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &globalKnownVars, inResVars.clone(), dependentVarsLst.clone(), inName.clone(), inOnlySparsePattern, false), '__try0);
        outJacobian = metamodelica::Ref::new(BackendDAE::Jacobian::GENERIC_JACOBIAN {
            jacobian: symJacBDAE.clone(),
            sparsePattern: sparsePattern.clone(),
            coloring: sparseColoring.clone(),
            nonlinearPattern: nonlinearPattern.clone(),
        });
        outShared = BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone());
        Ok::<_, &'static str>((outJacobian.clone(), outShared.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outJacobian = __try0_o0;
            outShared = __try0_o1;
        }
        Err(_) => {
            if Flags::isSet(Flags::JAC_DUMP.clone())? {
                Error::addInternalError(
                    literal!("function getSymbolicJacobian failed"),
                    metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"),
                )?;
            }
            outJacobian = openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN();
            outShared = inShared.clone();
        }
    }
    Ok((outJacobian, outShared))
}

pub(crate) fn hasGenericSymbolicJacobian(mut inJacobian: &metamodelica::Ref<BackendDAE::Jacobian>) -> bool {
    let mut out: bool;
    out = (::match_deref::match_deref! { match inJacobian {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(_), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn calculateEqSystemStateSetsJacobians(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    (outSyst, outShared) = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, stateSets, .. } => {
            let mut shared = inShared;
            let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
            let mut syst = (*syst).clone();
            let mut stateSets = (*stateSets).clone();
            comps = BackendDAEUtil::getStrongComponents(metamodelica::AsArg::as_arg(&syst));
            (stateSets, shared) = calculateStateSetsJacobian(stateSets.clone(), vars.clone(), eqns.clone(), &comps, shared)?;
            assign_field!(syst.stateSets = stateSets.clone());
            (syst.clone(), shared)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSyst, outShared))
}

fn calculateStateSetsJacobian(
    mut inStateSets: metamodelica::List<BackendDAE::StateSet>,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<BackendDAE::StateSet>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outStateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    outStateSets = ({
        let mut __acc: metamodelica::List<BackendDAE::StateSet> = metamodelica::nil();
        for mut s in (inStateSets).into_iter().cloned() {
            let __x = (match s.clone() {
                mut stateSet => {
                    (stateSet, outShared) = calculateStateSetJacobian(
                        &stateSet,
                        inVars.clone(),
                        inEqns.clone(),
                        inComps,
                        outShared.clone(),
                    )?;
                    stateSet.clone()
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((outStateSets, outShared))
}

fn calculateStateSetJacobian(
    mut inStateSet: &BackendDAE::StateSet,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(BackendDAE::StateSet, metamodelica::Ref<BackendDAE::Shared>)> {
    let mut outStateSet: BackendDAE::StateSet;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    (outStateSet, outShared) = (match inStateSet.clone() {
        BackendDAE::StateSet {
            index: mut index,
            rang: mut rang,
            state: mut state,
            crA: mut crA,
            varA: mut varA,
            statescandidates: mut statescandidates,
            ovars: mut ovars,
            eqns: mut eqns,
            oeqns: mut oeqns,
            crJ: mut crJ,
            varJ: mut varJ,
            ..
        } => {
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut crstates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut marked: metamodelica::Array<bool>;
            let mut hs: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
                i32,
                (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
            );
            let mut statevars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut compvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut diffVars: BackendDAE::Variables;
            let mut allvars: BackendDAE::Variables;
            let mut oVars: BackendDAE::Variables;
            let mut resVars: BackendDAE::Variables;
            let mut compeqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ceqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut cEqns: metamodelica::Ref<
                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
            >;
            let mut oEqns: metamodelica::Ref<
                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
            >;
            let mut jacobian: metamodelica::Ref<BackendDAE::Jacobian>;
            let mut name: ArcStr;
            let mut oeqns = oeqns.clone();
            crstates = List::map(statescandidates.clone(), &move |__a0: metamodelica::Ref<
                BackendDAE::Var,
            >|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
            })?;
            marked = arrayCreate(BackendVariable::varsSize(&inVars), false);
            marked = List::fold1(
                &crstates,
                &move |__a0: metamodelica::Ref<DAE::ComponentRef>,
                       __a1: BackendDAE::Variables,
                       __a2: metamodelica::Array<bool>| markSetStates(&__a0, &__a1, __a2),
                inVars.clone(),
                marked.clone(),
            )?;
            (compeqns, compvars) = getStateSetCompVarEqns(inComps, marked.clone(), inEqns, inVars.clone())?;
            compeqns = List::select(
                compeqns,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(removeStateSetEqn(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Equation>) -> Result<bool> + 'static,
                    >),
            )?;
            hs = List::fold(
                &crstates,
                &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1),
                HashSet::emptyHashSet(),
            )?;
            compvars = List::select1(
                compvars,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>,
                          __a1: (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                        ),
                        i32,
                        i32,
                        (
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::ComponentRef>,
                                    ) -> Result<bool>
                                    + 'static,
                            >,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                        ),
                    )| removeStateSetStates(&__a0, &__a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::Var>,
                                (
                                    metamodelica::Array<
                                        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
                                    >,
                                    (
                                        i32,
                                        i32,
                                        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                    ),
                                    i32,
                                    i32,
                                    (
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(
                                                    metamodelica::Ref<DAE::ComponentRef>,
                                                    metamodelica::Ref<DAE::ComponentRef>,
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                                + 'static,
                                        >,
                                    ),
                                ),
                            ) -> Result<bool>
                            + 'static,
                    >),
                hs,
            )?;
            (ceqns, oeqns) = IndexReduction::splitEqnsinConstraintAndOther(&compvars, &compeqns, &inShared)?;
            compvars = List::map(
                compvars,
                &fnptr!(BackendVariable::transformXToXd, metamodelica::Ref<BackendDAE::Var>),
            )?;
            ceqns = BackendEquation::replaceDerOpInEquationList(&ceqns)?;
            oeqns = BackendEquation::replaceDerOpInEquationList(metamodelica::AsArg::as_arg(&oeqns))?;
            ceqns = createResidualSetEquations(ceqns.clone(), crJ.clone(), 1, intGt(((ceqns).len() as i32), 1))?;
            allvars = BackendVariable::copyVariables(inVars);
            statevars = BackendVariable::getAllStateVarFromVariables(allvars.clone())?;
            statevars = List::map(
                statevars,
                &fnptr!(BackendVariable::transformXToXd, metamodelica::Ref<BackendDAE::Var>),
            )?;
            allvars = BackendVariable::addVars(&statevars, allvars)?;
            resVars = BackendVariable::listVar1(metamodelica::AsArg::as_arg(&varJ))?;
            diffVars = BackendVariable::listVar1(metamodelica::AsArg::as_arg(&statescandidates))?;
            oVars = BackendVariable::listVar1(&compvars)?;
            cEqns = BackendEquation::listEquation(&ceqns)?;
            oEqns = BackendEquation::listEquation(metamodelica::AsArg::as_arg(&oeqns))?;
            name = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("StateSetJac"));
                __mm_s.push_str(&*intString(System::tmpTickIndex(
                    Global::backendDAE_jacobianSeq.clone(),
                )));
                ArcStr::from(__mm_s)
            };
            (jacobian, shared) =
                getSymbolicJacobian(&diffVars, cEqns, resVars, oEqns, oVars, inShared, &allvars, name, false)?;
            (
                BackendDAE::StateSet {
                    index: index.clone(),
                    rang: rang.clone(),
                    state: state.clone(),
                    crA: crA.clone(),
                    varA: varA.clone(),
                    statescandidates: statescandidates.clone(),
                    ovars: ovars.clone(),
                    eqns: eqns.clone(),
                    oeqns: oeqns.clone(),
                    crJ: crJ.clone(),
                    varJ: varJ.clone(),
                    jacobian: jacobian,
                },
                shared,
            )
        }
    });
    Ok((outStateSet, outShared))
}

fn markSetStates(
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
    mut iVars: &BackendDAE::Variables,
    mut iMark: metamodelica::Array<bool>,
) -> Result<metamodelica::Array<bool>> {
    let mut oMark: metamodelica::Array<bool>;
    let mut index: i32;
    (_, index) = BackendVariable::getVarSingle(inCr, iVars)?;
    oMark = metamodelica::arrayUpdate(iMark.clone(), index, true)?;
    Ok(oMark)
}

fn removeStateSetStates(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut hs: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<bool> {
    let mut b: bool;
    b = !(BaseHashSet::has(BackendVariable::varCref(inVar), hs)?);
    Ok(b)
}

fn removeStateSetEqn(mut inEqn: &metamodelica::Ref<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inEqn {
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { source: Deref @ DAE::ElementSource { info: SourceInfo { fileName: Deref @ "stateselection", .. }, .. }, .. } => false,
        Deref @ BackendDAE::Equation::EQUATION { source: Deref @ DAE::ElementSource { info: SourceInfo { fileName: Deref @ "stateselection", .. }, .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn foundMarked<'__b>(mut ilst: &'__b metamodelica::List<i32>, mut marked: metamodelica::Array<bool>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match ilst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(false)
            },
            Deref @ metamodelica::ListNode::Cons { head: i, tail: rest } => {
                let mut b: bool;
                b = ({let __elt = (*metamodelica::index_checked(&marked.borrow(), i.clone())?).clone(); __elt});
                if (!(b)) {{ (ilst, marked) = (rest, marked.clone()); continue '__tco; }} else {return Ok(b)}
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getStateSetCompVarEqns(
    mut inComp: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut marked: metamodelica::Array<bool>,
    mut inEquationArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inVariables: BackendDAE::Variables,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut elst: metamodelica::List<i32>;
    let mut vlst: metamodelica::List<i32>;
    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    for mut comp in &**inComp {
        (elst, vlst) = BackendDAETransform::getEquationAndSolvedVarIndxes(metamodelica::AsArg::as_arg(&comp))?;
        if foundMarked(&vlst, marked.clone())? {
            eqnlst = BackendEquation::getList(elst, inEquationArray.clone())?;
            varlst = List::map1r(
                vlst,
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                inVariables.clone(),
            )?;
            outEquations = listAppend(eqnlst, outEquations);
            outVars = listAppend(varlst, outVars);
        }
    }
    Ok((outEquations, outVars))
}

fn createResidualSetEquations(
    mut iEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut crJ: metamodelica::Ref<DAE::ComponentRef>,
    mut index: i32,
    mut applySubs: bool,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut oEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut idx: i32 = index;
    oEqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
        for mut eq in (iEqs).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(eq.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr } => {
                    let mut crj: metamodelica::Ref<DAE::ComponentRef>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut expJ: metamodelica::Ref<DAE::Exp>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    crj = if (applySubs) {ComponentReference::subscriptCrefWithInt(&crJ, idx)?} else {crJ.clone()};
                    expJ = Expression::crefExp(crj.clone())?;
                    res = Expression::expSub(e1.clone(), e2.clone())?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: expJ.clone(), scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() });
                    idx = idx + 1;
                    eqn.clone()
                },
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source, attr: eqAttr } => {
                    let mut expJ: metamodelica::Ref<DAE::Exp>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    expJ = Expression::crefExp(ComponentReference::subscriptCrefWithInt(&crJ, idx)?)?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: expJ.clone(), scalar: e1.clone(), source: source.clone(), attr: eqAttr.clone() });
                    idx = idx + 1;
                    eqn.clone()
                },
                eqn => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function createResidualSetEquations failed for equation: ")); __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eqn))?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo"))?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(oEqs)
}

pub fn calculateJacobian(
    mut inVariables: BackendDAE::Variables,
    mut inEquationArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inAdjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut differentiateIfExp: bool,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
) -> (
    Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut outTplIntegerIntegerEquationLstOption: Option<
        metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    >;
    let mut oShared: metamodelica::Ref<BackendDAE::Shared>;
    (outTplIntegerIntegerEquationLstOption, oShared) = 'mc: {
        let __mc_input = (inVariables, inEquationArray, inAdjacencyMatrix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, eqns, m) => {
                    let mut jac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    (jac, shared) = calculateJacobianRows(eqns.clone(), vars.clone(), m.clone(), 1, 1, differentiateIfExp, iShared.clone(), &BackendDAEUtil::varsInEqn)?;
                    Ok((Some(jac.clone()), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, iShared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outTplIntegerIntegerEquationLstOption, oShared)
}

fn calculateJacobianRows<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEquationArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: BackendDAE::Variables,
    mut m: Type_a,
    mut eqn_indx: i32,
    mut scalar_eqn_indx: i32,
    mut differentiateIfExp: bool,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
    mut varsInEqn: &dyn ::std::ops::Fn(Type_a, i32) -> Result<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    pub type varsInEqnFunc<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a, i32) -> Result<metamodelica::List<i32>> + 'static>;

    let mut outLst: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)> = metamodelica::nil();
    let mut oShared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    let mut size: i32;
    let mut i: i32;
    let mut j: i32;
    let mut n: i32;
    let mut k: i32 = 0;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    i = eqn_indx;
    j = scalar_eqn_indx;
    size = 0;
    n = ExpandableArray::getLastUsedIndex(inEquationArray.clone());
    for mut k in 1..=n {
        if ExpandableArray::occupied(k, inEquationArray.clone()) {
            eqn = ExpandableArray::get(k, inEquationArray.clone())?;
            (outLst, size, oShared) = calculateJacobianRow(
                eqn,
                vars.clone(),
                m.clone(),
                i,
                j,
                differentiateIfExp,
                oShared,
                varsInEqn,
                outLst,
            )?;
            i = i + 1;
            j = j + size;
        }
    }
    outLst = metamodelica::Dangerous::listReverseInPlace(outLst);
    Ok((outLst, oShared))
}

fn calculateJacobianRow<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEquation: metamodelica::Ref<BackendDAE::Equation>,
    mut vars: BackendDAE::Variables,
    mut m: Type_a,
    mut eqn_indx: i32,
    mut scalar_eqn_indx: i32,
    mut differentiateIfExp: bool,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
    mut fvarsInEqn: &dyn ::std::ops::Fn(Type_a, i32) -> Result<metamodelica::List<i32>>,
    mut iAcc: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<(
    metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    i32,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    pub type varsInEqnFunc<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a, i32) -> Result<metamodelica::List<i32>> + 'static>;

    let mut outLst: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
    let mut size: i32;
    let mut oShared: metamodelica::Ref<BackendDAE::Shared>;
    (outLst, size, oShared) = (match &*inEquation {
        BackendDAE::Equation::EQUATION {
            exp: e1,
            scalar: e2,
            source,
            ..
        } => {
            let mut var_indxs: metamodelica::List<i32>;
            let mut var_indxs_1: metamodelica::List<i32>;
            let mut eqns: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            var_indxs = fvarsInEqn(m, eqn_indx)?;
            var_indxs_1 = List::sort(
                var_indxs,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            var_indxs_1 = List::sortedUnique(var_indxs_1, &fnptr!(intEq, i32, i32))?;
            (eqns, shared) = calculateJacobianRow2(
                Expression::expSub(e1.clone(), e2.clone())?,
                vars,
                scalar_eqn_indx,
                &var_indxs_1,
                differentiateIfExp,
                iShared,
                source.clone(),
                iAcc,
            )?;
            (eqns, 1, shared)
        }
        BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source, .. } => {
            let mut var_indxs: metamodelica::List<i32>;
            let mut var_indxs_1: metamodelica::List<i32>;
            let mut eqns: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            var_indxs = fvarsInEqn(m, eqn_indx)?;
            var_indxs_1 = List::sort(
                var_indxs,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            var_indxs_1 = List::sortedUnique(var_indxs_1, &fnptr!(intEq, i32, i32))?;
            (eqns, shared) = calculateJacobianRow2(
                e.clone(),
                vars,
                scalar_eqn_indx,
                &var_indxs_1,
                differentiateIfExp,
                iShared,
                source.clone(),
                iAcc,
            )?;
            (eqns, 1, shared)
        }
        BackendDAE::Equation::SOLVED_EQUATION {
            componentRef: cr,
            exp: e2,
            source,
            ..
        } => {
            let mut var_indxs: metamodelica::List<i32>;
            let mut var_indxs_1: metamodelica::List<i32>;
            let mut eqns: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            e1 = Expression::crefExp(cr.clone())?;
            var_indxs = fvarsInEqn(m, eqn_indx)?;
            var_indxs_1 = List::sort(
                var_indxs,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            var_indxs_1 = List::sortedUnique(var_indxs_1, &fnptr!(intEq, i32, i32))?;
            (eqns, shared) = calculateJacobianRow2(
                Expression::expSub(e1, e2.clone())?,
                vars,
                scalar_eqn_indx,
                &var_indxs_1,
                differentiateIfExp,
                iShared,
                source.clone(),
                iAcc,
            )?;
            (eqns, 1, shared)
        }
        BackendDAE::Equation::ARRAY_EQUATION {
            dimSize: ds,
            left: e1,
            right: e2,
            source,
            ..
        } => {
            let mut var_indxs: metamodelica::List<i32>;
            let mut var_indxs_1: metamodelica::List<i32>;
            let mut eqns: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut subslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            e = Expression::expSub(e1.clone(), e2.clone())?;
            (e, _) = Expression::extendArrExp(e, false);
            subslst = Expression::dimensionSizesSubscripts(ds.clone())?;
            subslst = Expression::rangesToSubscripts(&subslst)?;
            expl = List::map1r(subslst, &Expression::applyExpSubscripts, e)?;
            var_indxs = fvarsInEqn(m, eqn_indx)?;
            var_indxs_1 = List::sort(
                var_indxs,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            var_indxs_1 = List::sortedUnique(var_indxs_1, &fnptr!(intEq, i32, i32))?;
            (eqns, shared) = calculateJacobianRowLst(
                &expl,
                vars,
                scalar_eqn_indx,
                &var_indxs_1,
                differentiateIfExp,
                iShared,
                source.clone(),
                iAcc,
            )?;
            size = List::fold(metamodelica::AsArg::as_arg(&ds), &fnptr!(intMul, i32, i32), 1)?;
            (eqns, size, shared)
        }
        _ => {
            let mut r#str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            r#str = BackendDump::dumpEqnsStr(list![inEquation])?;
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- BackendDAE.calculateJacobianRow failed on "));
                __mm_s.push_str(&*r#str);
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok((outLst, size, oShared))
}

fn calculateJacobianRowLst(
    mut inExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut vars: BackendDAE::Variables,
    mut eqn_indx: i32,
    mut inIntegerLst: &metamodelica::List<i32>,
    mut differentiateIfExp: bool,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut iAcc: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<(
    metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outLst: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)> = iAcc;
    let mut oShared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    let mut eqn_indx_arr: i32 = eqn_indx;
    for mut e in &**inExps {
        (outLst, oShared) = calculateJacobianRow2(
            e.clone(),
            vars.clone(),
            eqn_indx_arr,
            inIntegerLst,
            differentiateIfExp,
            oShared,
            source.clone(),
            outLst,
        )?;
        eqn_indx_arr = eqn_indx_arr + 1;
    }
    Ok((outLst, oShared))
}

fn calculateJacobianRow2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut vars: BackendDAE::Variables,
    mut eqn_indx: i32,
    mut inIntegerLst: &metamodelica::List<i32>,
    mut differentiateIfExp: bool,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut iAcc: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<(
    metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outLst: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)> = iAcc;
    let mut oShared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e_1: metamodelica::Ref<DAE::Exp>;
    let mut dcrexp: metamodelica::Ref<DAE::Exp>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
    let mut vindx: i32 = 0;
    let mut r#str: ArcStr;
    match '__try0: {
        for mut vindx in &**inIntegerLst {
            let mut vindx = vindx.clone();
            v = unwrap_break_err!(BackendVariable::getVarAt(&vars, vindx), '__try0);
            cr = BackendVariable::varCref(&v);
            if BackendVariable::isStateVar(&v) {
                dcr = ComponentReference::crefPrefixDer(cr.clone());
                dcrexp = unwrap_break_err!(Expression::crefExp(cr.clone()), '__try0);
                dcrexp = metamodelica::Ref::new(DAE::Exp::CALL {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }),
                    expLst: list![dcrexp.clone()],
                    attr: DAE::callAttrBuiltinReal().clone(),
                });
                (e, _) = unwrap_break_err!(Expression::replaceExp(inExp.clone(), dcrexp.clone(), unwrap_break_err!(Expression::crefExp(dcr.clone()), '__try0)), '__try0);
            }
            (e_1, oShared) = unwrap_break_err!(Differentiate::differentiateExpCrefFullJacobian(inExp.clone(), &cr, vars.clone(), oShared.clone()), '__try0);
            if !(unwrap_break_err!(Expression::isZero(&e_1), '__try0)) {
                outLst = metamodelica::cons(
                    (
                        eqn_indx,
                        vindx,
                        metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION {
                            exp: e_1.clone(),
                            source: source.clone(),
                            attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
                        }),
                    ),
                    outLst.clone(),
                );
            }
        }
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                r#str = ExpressionBasics::printExpStr(inExp.clone())?;
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- BackendDAE.calculateJacobianRow2 failed on "));
                    __mm_s.push_str(&*r#str);
                    ArcStr::from(__mm_s)
                })?;
            }
            return Err(__try0_err);
        }
    }
    Ok((outLst, oShared))
}

fn addBackendDAESharedJacobian(
    mut inSymJac: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    mut inSparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    mut inSparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    mut inNonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> metamodelica::Ref<BackendDAE::Shared> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut symjacs: metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )>;
    symjacs = list![
        (inSymJac, inSparsePattern, inSparseColoring, inNonlinearPattern),
        (
            None,
            (
                metamodelica::nil(),
                metamodelica::nil(),
                (metamodelica::nil(), metamodelica::nil()),
                -1
            ),
            metamodelica::nil(),
            (
                metamodelica::nil(),
                metamodelica::nil(),
                (metamodelica::nil(), metamodelica::nil()),
                -1
            )
        ),
        (
            None,
            (
                metamodelica::nil(),
                metamodelica::nil(),
                (metamodelica::nil(), metamodelica::nil()),
                -1
            ),
            metamodelica::nil(),
            (
                metamodelica::nil(),
                metamodelica::nil(),
                (metamodelica::nil(), metamodelica::nil()),
                -1
            )
        ),
        (
            None,
            (
                metamodelica::nil(),
                metamodelica::nil(),
                (metamodelica::nil(), metamodelica::nil()),
                -1
            ),
            metamodelica::nil(),
            (
                metamodelica::nil(),
                metamodelica::nil(),
                (metamodelica::nil(), metamodelica::nil()),
                -1
            )
        )
    ];
    outShared = BackendDAEUtil::setSharedSymJacs(inShared, symjacs);
    outShared
}

fn addBackendDAESharedJacobianSparsePattern(
    mut inSparsePattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    mut inSparseColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    mut inIndex: i32,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::Shared>> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut symjacs: metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )>;
    let mut symJac: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut nonlinearPattern: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ) = BackendDAE::emptyNonlinearPattern().clone();
    let __arc1 = inShared.clone();
    let BackendDAE::SHARED { symjacs: __pa0, .. } = &*__arc1;
    symjacs = metamodelica::Own::own(__pa0);
    (symJac, _, _, _) = (symjacs).get(inIndex)?;
    symjacs = List::set(
        symjacs,
        inIndex,
        (symJac, inSparsePattern, inSparseColoring, nonlinearPattern),
    )?;
    outShared = BackendDAEUtil::setSharedSymJacs(inShared, symjacs);
    Ok(outShared)
}

pub(crate) fn analyzeJacobian(
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inTplIntegerIntegerEquationLstOption: Option<
        metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    >,
) -> (BackendDAE::JacobianType, bool) {
    let mut outJacobianType: BackendDAE::JacobianType;
    let mut jacConstant: bool;
    (outJacobianType, jacConstant) = 'mc: {
        let __mc_input = inTplIntegerIntegerEquationLstOption;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(jac) => {
                    let mut b: bool;
                    b = jacobianNonlinear(vars.clone(), metamodelica::AsArg::as_arg(&jac))?;
                    let (_, false) = (if (!(b)) {BackendDAEUtil::traverseBackendDAEExpsEqnsWithStop(eqns.clone(), &varsNotInRelations, (vars.clone(), true))?} else {(vars.clone(), false)}) else { return Err("pattern mismatch") };
                    Ok((openmodelica_backend_types::BackendDAE::JacobianType::JAC_NONLINEAR, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(jac) => {
                    let mut b: bool;
                    let mut jactype: BackendDAE::JacobianType;
                    let true = (jacobianConstant(metamodelica::AsArg::as_arg(&jac))?) else { return Err("pattern mismatch") };
                    b = rhsConstant(vars.clone(), eqns.clone())?;
                    jactype = if (b) {openmodelica_backend_types::BackendDAE::JacobianType::JAC_CONSTANT} else {openmodelica_backend_types::BackendDAE::JacobianType::JAC_LINEAR};
                    Ok((jactype, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(_) => {
                    Ok((openmodelica_backend_types::BackendDAE::JacobianType::JAC_LINEAR, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok((openmodelica_backend_types::BackendDAE::JacobianType::JAC_NO_ANALYTIC, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outJacobianType, jacConstant)
}

fn jacobianNonlinear(
    mut vars: BackendDAE::Variables,
    mut inTplIntegerIntegerEquationLst: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<bool> {
    let mut isNonLinear: bool = false;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut tpl: (i32, i32, metamodelica::Ref<BackendDAE::Equation>) =
        (0, 0, metamodelica::Ref::new(BackendDAE::Equation::DUMMY_EQUATION));
    for mut tpl in &**inTplIntegerIntegerEquationLst {
        let mut tpl = tpl.clone();
        (_, _, eq) = tpl;
        isNonLinear = (match &*eq {
            BackendDAE::Equation::EQUATION {
                exp: __esc_e1,
                scalar: __esc_e2,
                ..
            } => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                jacobianNonlinearExp(vars.clone(), e1.clone())? || jacobianNonlinearExp(vars.clone(), e2.clone())?
            }
            BackendDAE::Equation::RESIDUAL_EQUATION { exp: __esc_e, .. } => {
                e = (*__esc_e).clone();
                jacobianNonlinearExp(vars.clone(), e.clone())?
            }
            _ => return Err("match: no arm matched"),
        });
        if isNonLinear {
            return Ok(isNonLinear);
        }
    }
    Ok(isNonLinear)
}

fn jacobianNonlinearExp(mut vars: BackendDAE::Variables, mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    let (_, (_, __pa0)) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(
            traverserjacobianNonlinearExp,
            metamodelica::Ref<DAE::Exp>,
            (BackendDAE::Variables, bool)
        ),
        (vars, false),
    )?;
    outBoolean = metamodelica::Own::own(__pa0);
    Ok(outBoolean)
}

fn traverserjacobianNonlinearExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (BackendDAE::Variables, bool),
) -> (metamodelica::Ref<DAE::Exp>, bool, (BackendDAE::Variables, bool)) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (BackendDAE::Variables, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp, &tpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _)) => {
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok((e.clone(), false, (vars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (vars, _)) => {
                    BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    Ok((e.clone(), false, (vars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((e.clone(), false, tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok((e.clone(), false, tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, (_, b)) => {
                    Ok((e.clone(), !(b.clone()), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

fn jacobianConstant(
    mut inTplIntegerIntegerEquationLst: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<bool> {
    let mut outBoolean: bool = true;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (i32, i32, metamodelica::Ref<BackendDAE::Equation>) =
        (0, 0, metamodelica::Ref::new(BackendDAE::Equation::DUMMY_EQUATION));
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    for mut tpl in &**inTplIntegerIntegerEquationLst {
        let mut tpl = tpl.clone();
        eqn = Util::tuple33(tpl);
        outBoolean = (match &*eqn {
            BackendDAE::Equation::EQUATION {
                exp: __esc_e1,
                scalar: __esc_e2,
                ..
            } => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                Expression::isConst(e1.clone())? && Expression::isConst(e2.clone())?
            }
            BackendDAE::Equation::RESIDUAL_EQUATION { exp: __esc_e, .. } => {
                e = (*__esc_e).clone();
                Expression::isConst(e.clone())?
            }
            BackendDAE::Equation::SOLVED_EQUATION { exp: __esc_e, .. } => {
                e = (*__esc_e).clone();
                Expression::isConst(e.clone())?
            }
            BackendDAE::Equation::ARRAY_EQUATION {
                left: __esc_e1,
                right: __esc_e2,
                ..
            } => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                Expression::isConst(e1.clone())? && Expression::isConst(e2.clone())?
            }
            BackendDAE::Equation::COMPLEX_EQUATION {
                left: __esc_e1,
                right: __esc_e2,
                ..
            } => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                Expression::isConst(e1.clone())? && Expression::isConst(e2.clone())?
            }
            _ => false,
        });
        if !(outBoolean) {
            break;
        }
    }
    Ok(outBoolean)
}

pub(crate) fn isJacobianGeneric(mut inJac: &metamodelica::Ref<BackendDAE::Jacobian>) -> bool {
    let mut result: bool;
    result = (match &**inJac {
        BackendDAE::Jacobian::GENERIC_JACOBIAN { .. } => true,
        _ => false,
    });
    result
}

fn varsNotInRelations(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (BackendDAE::Variables, bool),
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, (BackendDAE::Variables, bool))> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut cont: bool;
    let mut tpl: (BackendDAE::Variables, bool) = tpl;
    (exp, cont, tpl) = (::match_deref::match_deref! { match &((exp.clone(), tpl.clone())) {
        (Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: t, expElse: f }, (vars, b)) => {
            let mut t = (*t).clone();
            let mut f = (*f).clone();
            let mut b = (*b).clone();
            let (_, (_, __pa0)) = Expression::traverseExpTopDown(cond.clone(), &fnptr!(BackendDAEUtil::getEqnsysRhsExp2, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), (vars.clone(), b.clone()))?;
            b = metamodelica::Own::own(__pa0);
            let (__pa1, (_, __pa2)) = Expression::traverseExpTopDown(t.clone(), &varsNotInRelations, (vars.clone(), b.clone()))?;
            t = metamodelica::Own::own(__pa1);
            b = metamodelica::Own::own(__pa2);
            let (__pa3, (_, __pa4)) = Expression::traverseExpTopDown(f.clone(), &varsNotInRelations, (vars.clone(), b.clone()))?;
            f = metamodelica::Own::own(__pa3);
            b = metamodelica::Own::own(__pa4);
            (metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: t.clone(), expElse: f.clone() }), false, (vars.clone(), b.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
            (exp, true, tpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (exp, false, tpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (exp, false, tpl)
        },
        (Deref @ DAE::Exp::CALL { expLst, .. }, _) => {
            (_, tpl) = Expression::traverseExpListTopDown(expLst.clone(), &fnptr!(BackendDAEUtil::getEqnsysRhsExp2, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), tpl)?;
            (exp, false, tpl)
        },
        (Deref @ DAE::Exp::LBINARY { .. }, _) => {
            (_, tpl) = Expression::traverseExpTopDown(exp.clone(), &fnptr!(BackendDAEUtil::getEqnsysRhsExp2, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), tpl)?;
            (exp, false, tpl)
        },
        (Deref @ DAE::Exp::LUNARY { .. }, __esc_tpl) => {
            tpl = (*__esc_tpl).clone();
            (_, tpl) = Expression::traverseExpTopDown(exp.clone(), &fnptr!(BackendDAEUtil::getEqnsysRhsExp2, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), tpl.clone())?;
            (exp, false, tpl.clone())
        },
        (Deref @ DAE::Exp::RELATION { .. }, __esc_tpl) => {
            tpl = (*__esc_tpl).clone();
            (_, tpl) = Expression::traverseExpTopDown(exp.clone(), &fnptr!(BackendDAEUtil::getEqnsysRhsExp2, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), tpl.clone())?;
            (exp, false, tpl.clone())
        },
        (Deref @ DAE::Exp::ASUB { exp: e1, sub: subs }, _) => {
            let mut b: bool;
            let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            let (_, ref __pa1 @ (_, ref __pa0)) = Expression::traverseExpTopDown(e1.clone(), &varsNotInRelations, tpl)?;
            b = metamodelica::Own::own(__pa0);
            tpl = metamodelica::Own::own(__pa1);
            if b {
                (_, tpl) = Expression::traverseExpListTopDown(expLst, &fnptr!(BackendDAEUtil::getEqnsysRhsExp2, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), tpl)?;
            }
            (exp, false, tpl)
        },
        (_, (_, b)) => {
            (exp, b.clone(), tpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, cont, tpl))
}

fn rhsConstant(
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<bool> {
    let mut outBoolean: bool;
    let mut repl: BackendVarTransform::VariableReplacements;
    if BackendEquation::equationArraySize(eqns.clone())? == 0 {
        outBoolean = true;
    } else {
        repl = BackendDAEUtil::makeZeroReplacements(vars.clone())?;
        (_, outBoolean, _) = BackendEquation::traverseEquationArray_WithStop(
            eqns,
            &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                   __a1: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)|
                  -> metamodelica::Result<_> { ::std::result::Result::Ok(rhsConstant2(__a0, &__a1)) },
            (vars, true, repl),
        )?;
    }
    Ok(outBoolean)
}

fn rhsConstant2(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    bool,
    (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
) {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut cont: bool;
    let mut outTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements);
    (outEq, cont, outTpl) = 'mc: {
        let __mc_input = (inEq, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, (vars, b, repl)) => {
                    let mut new_exp: metamodelica::Ref<DAE::Exp>;
                    let mut rhs_exp: metamodelica::Ref<DAE::Exp>;
                    let mut res: bool;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(new_exp.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res, (vars.clone(), b.clone() && res, repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }, (vars, b, repl)) => {
                    let mut new_exp: metamodelica::Ref<DAE::Exp>;
                    let mut rhs_exp: metamodelica::Ref<DAE::Exp>;
                    let mut res: bool;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(new_exp.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res, (vars.clone(), b.clone() && res, repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }, (vars, b, repl)) => {
                    let mut new_exp: metamodelica::Ref<DAE::Exp>;
                    let mut rhs_exp: metamodelica::Ref<DAE::Exp>;
                    let mut res: bool;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(new_exp.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res, (vars.clone(), b.clone() && res, repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, (vars, b, repl)) => {
                    let mut rhs_exp: metamodelica::Ref<DAE::Exp>;
                    let mut res: bool;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(e.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res, (vars.clone(), b.clone() && res, repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, (vars, _, repl)) => {
                    Ok((eqn.clone(), false, (vars.clone(), false, repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, cont, outTpl)
}

fn getJacobianResiduals(
    mut jacDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut diffedRes: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let __pa0 = ::match_deref::match_deref! { match &(jacDAE.eqs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    syst = metamodelica::Own::own(__pa0);
    diffedRes = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (BackendVariable::varList(&syst.orderedVars)?).into_iter().cloned() {
            if !(BackendVariable::isRESVar(&(var.clone()))) {
                continue;
            }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(diffedRes)
}

// =============================================================================
// Function detects non-linear strong component in symbolic jacobians
//  - non-linear components should never appear in symbolic jacobian and
//    indicate an singular or wrong system
//  - this modules stops compiling and outputs an error, otherwise we
//    would get error at runtime compiling
// =============================================================================
fn checkForNonLinearStrongComponents(
    mut symbolicJacobian: &(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> Result<bool> {
    let mut result: bool;
    let mut jacBDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut name: ArcStr;
    (jacBDAE, name, _, _, _, _) = symbolicJacobian.clone();
    match '__try0: {
        unwrap_break_err!(BackendDAEUtil::mapEqSystem(&jacBDAE, &checkForNonLinearStrongComponents_work), '__try0);
        result = true;
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(Error::INVALID_NONLINEAR_JACOBIAN_COMPONENT.clone(), list![name.clone()])?;
            result = false;
        }
    }
    Ok(result)
}

fn checkForNonLinearStrongComponents_work(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        comps = metamodelica::Own::own(__pa1);
        for mut comp in &*comps {
            let () = (match &*comp.clone() {
                BackendDAE::StrongComponent::EQUATIONSYSTEM {
                    jacType: BackendDAE::JacobianType::JAC_NONLINEAR { .. },
                    ..
                } => {
                    if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!(
                                "[symjacdump] Following strong component represents a nonlinear symbolic jacobian:\n"
                            ));
                            __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(metamodelica::AsArg::as_arg(&comp), Some(syst.clone())), '__try0));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    break '__try0 Err::<_, _>("fail");
                }
                BackendDAE::StrongComponent::EQUATIONSYSTEM {
                    jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC { .. },
                    ..
                } => {
                    if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!(
                                "[symjacdump] Following strong component represents a no symbolic jacobian:\n"
                            ));
                            __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(metamodelica::AsArg::as_arg(&comp), Some(syst.clone())), '__try0));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    break '__try0 Err::<_, _>("fail");
                }
                BackendDAE::StrongComponent::EQUATIONSYSTEM {
                    jacType: BackendDAE::JacobianType::JAC_GENERIC { .. },
                    ..
                } => {
                    if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!(
                                "[symjacdump] Following strong component represents a generic jacobian:\n"
                            ));
                            __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(metamodelica::AsArg::as_arg(&comp), Some(syst.clone())), '__try0));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    break '__try0 Err::<_, _>("fail");
                }
                BackendDAE::StrongComponent::TORNSYSTEM { linear: false, .. } => {
                    if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("[symjacdump] Following (torn) strong component represents a nonlinear symbolic jacobian:\n"));
                            __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(metamodelica::AsArg::as_arg(&comp), Some(syst.clone())), '__try0));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    break '__try0 Err::<_, _>("fail");
                }
                _ => (),
            });
        }
        Ok::<_, &'static str>((comps.clone(),))
    } {
        Ok((__try0_o0,)) => {
            comps = __try0_o0;
        }
        Err(__try0_err) => {
            return Err(__try0_err);
        }
    }
    Ok((syst, shared))
}

pub(crate) fn getFixedStatesForSelfdependentSets(
    mut stateSet: &BackendDAE::StateSet,
    mut unfixedStates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut toFix: i32,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut statesToFix: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut nonlinearCountLst: metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Var>)> = metamodelica::nil();
    let _ = (::match_deref::match_deref! { match &(stateSet.jacobian.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(sJac), .. } => {
            let mut dae: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut matrixName: ArcStr;
            (dae, matrixName, _, _, _, _) = sJac.clone();
            for mut var in &*unfixedStates {
                nonlinearCountLst = metamodelica::cons(getNonlinearStateCount(var.clone(), unfixedStates.clone(), &dae, matrixName.clone())?, nonlinearCountLst);
            }
            0
        },
        _ => return Err("match: no arm matched"),
    } });
    statesToFix = fixedVarsFromNonlinearCount(nonlinearCountLst, toFix)?;
    Ok(statesToFix)
}

fn getNonlinearStateCount(
    mut state: metamodelica::Ref<BackendDAE::Var>,
    mut diffVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut matrixName: ArcStr,
) -> Result<(i32, metamodelica::Ref<BackendDAE::Var>)> {
    let mut outTpl: (i32, metamodelica::Ref<BackendDAE::Var>);
    outTpl = ({
        let mut nonlinearCount: i32 = 0;
        (match &**dae {
            BackendDAE::BackendDAE { eqs: systs, .. } => {
                let mut tpl: (
                    metamodelica::Ref<BackendDAE::Var>,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    i32,
                    ArcStr,
                );
                let mut outState: metamodelica::Ref<BackendDAE::Var>;
                tpl = (state, diffVars, nonlinearCount, matrixName);
                for mut syst in &*systs.clone() {
                    let _ = (match &*syst.clone() {
                        BackendDAE::EqSystem {
                            orderedVars: _,
                            orderedEqs: eqnarray,
                            m: _,
                            mT: _,
                            mapping: _,
                            matching: _,
                            stateSets: _,
                            partitionKind: _,
                            ..
                        } => {
                            tpl = BackendEquation::traverseEquationArray(
                                eqnarray.clone(),
                                &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                                       __a1: (
                                    metamodelica::Ref<BackendDAE::Var>,
                                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                                    i32,
                                    ArcStr,
                                )| getNonlinearStateCount0(__a0, &__a1),
                                tpl,
                            )?;
                            0
                        }
                    });
                }
                (outState, _, nonlinearCount, _) = tpl;
                (nonlinearCount, outState)
            }
        })
    });
    Ok(outTpl)
}

fn getNonlinearStateCount0(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        ArcStr,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        ArcStr,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTpl: (
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        ArcStr,
    );
    outEq = inEq.clone();
    outTpl = (match &*inEq {
        BackendDAE::Equation::EQUATION { scalar: exp, .. } => {
            let mut diffExp: metamodelica::Ref<DAE::Exp>;
            let mut state: metamodelica::Ref<BackendDAE::Var>;
            let mut diffVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut nonlinearCount: i32;
            let mut matrixName: ArcStr;
            let mut seedVar: metamodelica::Ref<DAE::ComponentRef>;
            (state, diffVars, nonlinearCount, matrixName) = inTpl.clone();
            seedVar = Differentiate::createSeedCrefName(&(BackendVariable::varCref(&state)), &matrixName)?;
            diffExp = Differentiate::differentiateExpSolve(exp.clone(), seedVar, None)?;
            for mut var in &*diffVars {
                if !(ComponentReferenceBasics::crefEqual(&var.varName, &state.varName)?)
                    && Expression::expContains(&diffExp, &(Expression::crefExp(var.varName.clone())?))?
                {
                    if Expression::isZero(&(BackendVariable::varStartValue(metamodelica::AsArg::as_arg(&var))?))? {
                        nonlinearCount = nonlinearCount + 2;
                    } else {
                        nonlinearCount = nonlinearCount + 1;
                    }
                }
            }
            (state, diffVars, nonlinearCount, matrixName)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outEq, outTpl))
}

fn fixedVarsFromNonlinearCount(
    mut tplLst: metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Var>)>,
    mut toFix: i32,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut fixedVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut sortedTplLst: metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Var>)>;
    let mut strippedTplLst: metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Var>)>;
    let mut fixVar: metamodelica::Ref<BackendDAE::Var>;
    let mut fixInt: i32;
    for mut tpl in &*tplLst {
        (fixInt, fixVar) = tpl.clone();
    }
    sortedTplLst = List::sort(tplLst, std::sync::Arc::new(fnptr!(Util::compareTupleIntGt, _, _)))?;
    strippedTplLst = List::firstN(sortedTplLst, toFix)?;
    for mut tpl in &*strippedTplLst {
        (_, fixVar) = tpl.clone();
        assign_field!(
            fixVar.values = DAEUtil::setFixedAttr(
                fixVar.values.clone(),
                Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))
            )?
        );
        fixedVars = metamodelica::cons(fixVar, fixedVars);
    }
    Ok(fixedVars)
}

fn stripPartialDerNonlinearPattern(
    mut pat: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
) -> (
    metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
    i32,
) {
    let mut pat: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ) = pat;
    let mut pat_cref: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut pat_crefT: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut v1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut v2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut index: i32;
    let (__pa0, __pa1, (__pa2, __pa3), __pa4) = pat;
    pat_cref = metamodelica::Own::own(__pa0);
    pat_crefT = metamodelica::Own::own(__pa1);
    v1 = metamodelica::Own::own(__pa2);
    v2 = metamodelica::Own::own(__pa3);
    index = metamodelica::Own::own(__pa4);
    pat_cref = ({
        let mut __acc: metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )> = metamodelica::nil();
        for mut cref_tpl in (pat_cref).into_iter().cloned() {
            let __x = stripPartialDer(cref_tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    pat_crefT = ({
        let mut __acc: metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )> = metamodelica::nil();
        for mut cref_tpl in (pat_crefT).into_iter().cloned() {
            let __x = stripPartialDer(cref_tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    v1 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (v1).into_iter().cloned() {
            let __x = (stripPartialDerWork(v.clone())).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    v2 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (v2).into_iter().cloned() {
            let __x = (stripPartialDerWork(v.clone())).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    pat = (pat_cref, pat_crefT, (v1, v2), index);
    pat
}

fn stripPartialDer(
    mut cref_tpl: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> (
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut cref_tpl: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ) = cref_tpl;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut dependencies: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (cref, dependencies) = cref_tpl;
    (cref, _) = stripPartialDerWork(cref);
    dependencies = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut dep in (dependencies).into_iter().cloned() {
            let __x = (stripPartialDerWork(dep.clone())).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    cref_tpl = (cref, dependencies);
    cref_tpl
}

fn stripPartialDerWork(mut cref: metamodelica::Ref<DAE::ComponentRef>) -> (metamodelica::Ref<DAE::ComponentRef>, bool) {
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = cref;
    let mut strip: bool;
    (cref, strip) = (match &*cref {
        DAE::ComponentRef::CREF_IDENT {
            ident: __cref_ident, ..
        } if (StringUtil::startsWith(__cref_ident.clone(), literal!("$pDER"))) => (cref, true),
        DAE::ComponentRef::CREF_QUAL {
            ident: __cref_ident, ..
        } if (StringUtil::startsWith(__cref_ident.clone(), literal!("$pDER"))) => (cref, true),
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cref_componentRef,
            ident: __cref_ident,
            identType: __cref_identType,
            subscriptLst: __cref_subscriptLst,
        } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            (cr, strip) = stripPartialDerWork(__cref_componentRef.clone());
            if strip {
                cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: __cref_ident.clone(),
                    identType: __cref_identType.clone(),
                    subscriptLst: __cref_subscriptLst.clone(),
                });
            } else {
                cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: __cref_ident.clone(),
                    identType: __cref_identType.clone(),
                    subscriptLst: __cref_subscriptLst.clone(),
                    componentRef: cr,
                });
            }
            (cr, false)
        }
        _ => (cref, false),
    });
    (cref, strip)
}

// =============================================================================
// [ASSC] section for analytical to symbolical singularity transformation
//
// Generates linear jacobian
// =============================================================================
pub type LinearJacobianRow = metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>;

pub type LinearJacobianRhs = metamodelica::Array<metamodelica::Ref<DAE::Exp>>;

pub type LinearJacobianInd = metamodelica::Array<(i32, i32)>;

pub mod LinearJacobian {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct LinearJacobian {
        /// all loop variables entries
        pub rows: metamodelica::Array<metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>>,
        /// the expression containing all non loop variable entries
        pub rhs: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        /// equation indices  <array, scalar>
        pub ind: metamodelica::Array<(i32, i32)>,
        /// changed equations
        pub eq_marks: metamodelica::Array<bool>,
    }

    impl metamodelica::gc::MMTrace for LinearJacobian {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.rows, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.rhs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.ind, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.eq_marks, __mmv)?;
            Ok(())
        }
    }
    impl Default for LinearJacobian {
        fn default() -> Self {
            Self {
                rows: Default::default(),
                rhs: Default::default(),
                ind: Default::default(),
                eq_marks: Default::default(),
            }
        }
    }

    pub type LINEAR_REAL_JACOBIAN = LinearJacobian;

    pub(crate) fn toString(mut linJac: &metamodelica::Ref<LinearJacobian>, mut heading: &ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("######################################################\n"));
            __mm_s.push_str(&*literal!(" LinearJacobian sparsity pattern: "));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("######################################################\n"));
            __mm_s.push_str(&*literal!(
                "(scal_idx|arr_idx|changed) [var_index, value] || RHS_EXPRESSION\n"
            ));
            ArcStr::from(__mm_s)
        };
        for mut idx in 1..=metamodelica::arrayLength(linJac.rows.clone()) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*rowToString(
                    ({
                        let __elt = (*metamodelica::index_checked(&linJac.rows.borrow(), idx)?).clone();
                        __elt
                    }),
                    ({
                        let __elt = (*metamodelica::index_checked(&linJac.rhs.borrow(), idx)?).clone();
                        __elt
                    }),
                    ({
                        let __elt = (*metamodelica::index_checked(&linJac.ind.borrow(), idx)?).clone();
                        __elt
                    }),
                    ({
                        let __elt = (*metamodelica::index_checked(&linJac.eq_marks.borrow(), idx)?).clone();
                        __elt
                    }),
                )?);
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    fn rowToString(
        mut row: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>,
        mut rhs: metamodelica::Ref<DAE::Exp>,
        mut indices: (i32, i32),
        mut changed: bool,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut i_arr: i32;
        let mut i_scal: i32;
        let mut index: i32;
        let mut value: metamodelica::Real;
        let mut row_lst: metamodelica::List<(i32, metamodelica::Real)> = UnorderedMap::toList(row.clone());
        (i_arr, i_scal) = indices;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(i_arr));
            __mm_s.push_str(&*literal!("|"));
            __mm_s.push_str(&*intString(i_scal));
            __mm_s.push_str(&*literal!("|"));
            __mm_s.push_str(&*boolString(changed));
            __mm_s.push_str(&*literal!("):    "));
            ArcStr::from(__mm_s)
        };
        if (row_lst).is_empty() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("EMPTY ROW     "));
                ArcStr::from(__mm_s)
            };
        } else {
            for mut element in &*row_lst {
                (index, value) = element.clone();
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("["));
                    __mm_s.push_str(&*intString(index));
                    __mm_s.push_str(&*literal!("|"));
                    __mm_s.push_str(&*realString(value));
                    __mm_s.push_str(&*literal!("] "));
                    ArcStr::from(__mm_s)
                };
            }
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("    || RHS: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr((ExpressionSimplify::simplify(rhs)?).0)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn generate(
        mut loopEqs: &metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, (i32, i32))>,
        mut loopVars: &metamodelica::List<(metamodelica::Ref<BackendDAE::Var>, i32)>,
        mut ass1: metamodelica::Array<i32>,
    ) -> Result<metamodelica::Ref<LinearJacobian>> {
        type evaluateFunc =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Real> + 'static>;

        fn intWrapperFunc(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Real> {
            let mut v: metamodelica::Real;
            v = intReal(Expression::getEvaluatedConstInteger(&e)?);
            Ok(v)
        }

        let mut linJac: metamodelica::Ref<LinearJacobian>;
        let mut eqn_index: i32 = 1;
        let mut var_index: i32;
        let mut constReal: metamodelica::Real;
        let mut row: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>;
        let mut tmp_mat: metamodelica::List<metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>> =
            metamodelica::nil();
        let mut tmp_rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        let mut tmp_idx: metamodelica::List<(i32, i32)> = metamodelica::nil();
        let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
        let mut index: (i32, i32);
        let mut scal_idx: i32;
        let mut var: metamodelica::Ref<BackendDAE::Var>;
        let mut res: metamodelica::Ref<DAE::Exp>;
        let mut pDer: metamodelica::Ref<DAE::Exp>;
        let mut varRep: BackendVarTransform::VariableReplacements;
        let mut eFunc: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Real> + 'static> =
            if (Flags::getConfigBool(Flags::REAL_ASSC.clone())?) {
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| Expression::getEvaluatedConstReal(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Real> + 'static,
                    >)
            } else {
                (std::sync::Arc::new(intWrapperFunc)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Real> + 'static,
                    >)
            };
        varRep = BackendVarTransform::emptyReplacements();
        for mut loopVar in &**loopVars {
            (var, _) = loopVar.clone();
            varRep = BackendVarTransform::addReplacement(
                varRep,
                BackendVariable::varCref(&var),
                metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
                None,
            )?;
        }
        for mut loopEq in &**loopEqs {
            row = UnorderedMap::new(
                std::sync::Arc::new(fnptr!(Util::id, _)),
                (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                1,
            );
            (eqn, index) = loopEq.clone();
            res = BackendEquation::createResidualExp(&eqn)?;
            if '__try0: {
                for mut loopVar in &**loopVars {
                    (var, var_index) = loopVar.clone();
                    pDer = unwrap_break_err!(Differentiate::differentiateExpSolve(res.clone(), BackendVariable::varCref(&var), None), '__try0);
                    (pDer, _) = unwrap_break_err!(ExpressionSimplify::simplify(pDer.clone()), '__try0);
                    constReal = unwrap_break_err!(eFunc(pDer.clone()), '__try0);
                    if !(realEq(constReal, metamodelica::OrderedFloat(0.0_f64))) {
                        unwrap_break_err!(UnorderedMap::add(var_index, constReal, row.clone()), '__try0);
                    }
                }
                (res, _) = BackendVarTransform::replaceExp(&res, &varRep, None);
                tmp_mat = metamodelica::cons(row.clone(), tmp_mat.clone());
                tmp_rhs = metamodelica::cons((unwrap_break_err!(ExpressionSimplify::simplify(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }), operator: DAE::Operator::MUL { ty: DAE::T_UNKNOWN_DEFAULT().clone() }, exp2: res.clone() })), '__try0)).0, tmp_rhs.clone());
                tmp_idx = metamodelica::cons(index, tmp_idx.clone());
                (_, scal_idx) = index;
                eqn_index = eqn_index + 1;
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
        linJac = metamodelica::Ref::new(LinearJacobian {
            rows: metamodelica::arrayFromVec(tmp_mat.clone().into_iter().cloned().collect()),
            rhs: metamodelica::arrayFromVec(tmp_rhs.into_iter().cloned().collect()),
            ind: metamodelica::arrayFromVec(tmp_idx.into_iter().cloned().collect()),
            eq_marks: arrayCreate(((tmp_mat).len() as i32), false),
        });
        Ok(linJac)
    }

    pub(crate) fn emptyOrSingle(mut linJac: &metamodelica::Ref<LinearJacobian>) -> bool {
        let mut empty: bool = metamodelica::arrayLength(linJac.rows.clone()) < 2
            && metamodelica::arrayLength(linJac.rhs.clone()) < 2
            && metamodelica::arrayLength(linJac.ind.clone()) < 2
            && metamodelica::arrayLength(linJac.eq_marks.clone()) < 2;
        empty
    }

    pub(crate) fn solve(mut linJac: metamodelica::Ref<LinearJacobian>) -> metamodelica::Ref<LinearJacobian> {
        let mut linJac: metamodelica::Ref<LinearJacobian> = linJac;
        let mut col_index: i32;
        let mut piv_value: metamodelica::Real;
        let mut row_value: metamodelica::Real;
        for mut i in 1..=metamodelica::arrayLength(linJac.rows.clone()) {
            if '__try0: {
                (col_index, piv_value) = unwrap_break_err!(getPivot(({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&linJac.rows.borrow(), i), '__try0)).clone(); __elt})), '__try0);
                for mut j in i + 1..=metamodelica::arrayLength(linJac.rows.clone()) {
                    row_value = unwrap_break_err!(UnorderedMap::getOrDefault(col_index, ({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&linJac.rows.borrow(), j), '__try0)).clone(); __elt}), metamodelica::OrderedFloat(0.0_f64)), '__try0);
                    if !(realEq(row_value, metamodelica::OrderedFloat(0.0_f64))) {
                        {
                            let __cell1 = true;
                            let __idx1 = j;
                            *unwrap_break_err!(metamodelica::index_mut_checked(&mut linJac.eq_marks.clone().borrow_mut(), __idx1), '__try0) = __cell1;
                        }
                        unwrap_break_err!(solveRow(({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&linJac.rows.borrow(), i), '__try0)).clone(); __elt}), ({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&linJac.rows.borrow(), j), '__try0)).clone(); __elt}), piv_value, row_value), '__try0);
                        {
                            let __cell2 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: ({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&linJac.rhs.borrow(), j), '__try0)).clone(); __elt}), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: piv_value }) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: ({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&linJac.rhs.borrow(), i), '__try0)).clone(); __elt}), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: row_value }) }) });
                            let __idx2 = j;
                            *unwrap_break_err!(metamodelica::index_mut_checked(&mut linJac.rhs.clone().borrow_mut(), __idx2), '__try0) = __cell2;
                        }
                    }
                }
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
        linJac
    }

    pub(crate) fn solveRow(
        mut pivot_row: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>,
        mut row: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>,
        mut piv_value: metamodelica::Real,
        mut row_value: metamodelica::Real,
    ) -> Result<()> {
        let mut idx: i32 = 0;
        let mut val: metamodelica::Real;
        let mut diag_val: metamodelica::Real;
        for mut idx in &*UnorderedMap::keyList(pivot_row.clone()) {
            let mut idx = idx.clone();
            let () = (match (
                UnorderedMap::get(idx, row.clone())?,
                UnorderedMap::get(idx, pivot_row.clone())?,
            ) {
                (Some(mut __esc_val), Some(mut __esc_diag_val)) => {
                    val = __esc_val.clone();
                    diag_val = __esc_diag_val.clone();
                    val = val * piv_value - diag_val * row_value;
                    if realAbs(val) < metamodelica::OrderedFloat(1e-12_f64) {
                        UnorderedMap::remove(idx, row.clone())?;
                    } else {
                        UnorderedMap::add(idx, val, row.clone())?;
                    }
                    ()
                }
                (None, Some(mut __esc_diag_val)) => {
                    diag_val = __esc_diag_val.clone();
                    UnorderedMap::add(idx, -(diag_val * row_value), row.clone())?;
                    ()
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("SymbolicJacobian.LinearJacobian.solveRow"));
                            __mm_s.push_str(&*literal!(" key does not have an element in pivot row."));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("BackEnd/SymbolicJacobian.mo")),
                    )?;
                    ()
                }
            });
        }
        for mut idx in &*UnorderedMap::keyList(row.clone()) {
            let mut idx = idx.clone();
            let () = (match (
                UnorderedMap::get(idx, row.clone())?,
                UnorderedMap::get(idx, pivot_row.clone())?,
            ) {
                (Some(mut __esc_val), None) => {
                    val = __esc_val.clone();
                    val = val * piv_value;
                    UnorderedMap::add(idx, val, row.clone())?;
                    ()
                }
                _ => (),
            });
        }
        Ok(())
    }

    pub(crate) fn updatePivotRow(
        mut pivot_row: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>,
        mut piv_value: metamodelica::Real,
    ) -> Result<()> {
        let mut value: metamodelica::Real;
        if !(realEq(piv_value, metamodelica::OrderedFloat(1.0_f64))) {
            for mut idx in &*UnorderedMap::keyList(pivot_row.clone()) {
                value = UnorderedMap::getOrFail(idx.clone(), pivot_row.clone())?;
                UnorderedMap::add(
                    idx.clone(),
                    metamodelica::real_div_checked(value, piv_value)?,
                    pivot_row.clone(),
                )?;
            }
        }
        Ok(())
    }

    fn getPivot(
        mut pivot_row: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>,
    ) -> Result<(i32, metamodelica::Real)> {
        let mut idx: i32;
        let mut value: metamodelica::Real;
        if Vector::isEmpty(pivot_row.keys.clone()) {
            return Err("fail");
        } else {
            idx = UnorderedMap::firstKey(pivot_row.clone())?;
            value = UnorderedMap::getOrFail(idx, pivot_row)?;
        }
        Ok((idx, value))
    }

    pub(crate) fn resolveASSC(
        mut linJac: &metamodelica::Ref<LinearJacobian>,
        mut ass1: metamodelica::Array<i32>,
        mut ass2: metamodelica::Array<i32>,
        mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
        mut init: bool,
    ) -> Result<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::Ref<BackendDAE::EqSystem>,
    )> {
        let mut ass1: metamodelica::Array<i32> = ass1;
        let mut ass2: metamodelica::Array<i32> = ass2;
        let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
        let mut i_arr: i32;
        let mut i_scal: i32;
        let mut lhs: metamodelica::Ref<DAE::Exp>;
        let mut rhs: metamodelica::Ref<DAE::Exp>;
        let mut newEqn: metamodelica::Ref<BackendDAE::Equation>;
        let mut updateList_arr: metamodelica::List<i32> = metamodelica::nil();
        let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
        let mut mapIncRowEqn: metamodelica::Array<i32>;
        let mut indexType: BackendDAE::IndexType;
        let mut fullASSC: bool = Flags::getConfigBool(Flags::FULL_ASSC.clone())?;
        for mut r in 1..=metamodelica::arrayLength(linJac.rows.clone()) {
            if ({
                let __elt = (*metamodelica::index_checked(&linJac.eq_marks.borrow(), r)?).clone();
                __elt
            }) && (UnorderedMap::isEmpty(
                ({
                    let __elt = (*metamodelica::index_checked(&linJac.rows.borrow(), r)?).clone();
                    __elt
                }),
            ) || fullASSC)
            {
                (i_arr, i_scal) = ({
                    let __elt = (*metamodelica::index_checked(&linJac.ind.borrow(), r)?).clone();
                    __elt
                });
                {
                    let __cell0 = -1;
                    let __idx0 = ({
                        let __elt = (*metamodelica::index_checked(&ass1.borrow(), i_scal)?).clone();
                        __elt
                    });
                    *metamodelica::index_mut_checked(&mut ass2.clone().borrow_mut(), __idx0)? = __cell0;
                }
                {
                    let __cell1 = -1;
                    let __idx1 = i_scal;
                    *metamodelica::index_mut_checked(&mut ass1.clone().borrow_mut(), __idx1)? = __cell1;
                }
                (rhs, _) = ExpressionSimplify::simplify(
                    ({
                        let __elt = (*metamodelica::index_checked(&linJac.rhs.borrow(), r)?).clone();
                        __elt
                    }),
                )?;
                lhs = generateLHSfromList(
                    UnorderedMap::keyArray(
                        ({
                            let __elt = (*metamodelica::index_checked(&linJac.rows.borrow(), r)?).clone();
                            __elt
                        }),
                    ),
                    UnorderedMap::valueArray(
                        ({
                            let __elt = (*metamodelica::index_checked(&linJac.rows.borrow(), r)?).clone();
                            __elt
                        }),
                    ),
                    &syst.orderedVars,
                )?;
                newEqn = BackendEquation::generateEquation(
                    lhs,
                    rhs,
                    DAE::emptyElementSource().clone(),
                    BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
                )?;
                if Flags::isSet(Flags::DUMP_ASSC.clone())?
                    || Flags::isSet(Flags::BLT_DUMP.clone())?
                        && UnorderedMap::isEmpty(
                            ({
                                let __elt = (*metamodelica::index_checked(&linJac.rows.borrow(), r)?).clone();
                                __elt
                            }),
                        )
                {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("[ASSC] The equation: "));
                        __mm_s.push_str(&*BackendDump::equationString(
                            &(BackendEquation::get(syst.orderedEqs.clone(), i_arr)?),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("[ASSC] Gets replaced by equation: "));
                        __mm_s.push_str(&*BackendDump::equationString(&newEqn)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                assign_field!(syst.orderedEqs = BackendEquation::setAtIndex(syst.orderedEqs.clone(), i_arr, newEqn)?);
                updateList_arr = metamodelica::cons(i_arr, updateList_arr);
            }
        }
        if !((updateList_arr).is_empty()) {
            match '__try2: {
                let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.mapping.clone()) {
                    Some((__pa3, __pa4, __pa5, true, _)) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                    _ => break '__try2 Err::<_, _>("pattern mismatch"),
                } };
                mapEqnIncRow = metamodelica::Own::own(__pa3);
                mapIncRowEqn = metamodelica::Own::own(__pa4);
                indexType = metamodelica::Own::own(__pa5);
                (syst, _, _) = unwrap_break_err!(BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), indexType, None, updateList_arr.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), false), '__try2);
                Ok::<_, &'static str>((syst.clone(),))
            } {
                Ok((__try2_o0,)) => {
                    syst = __try2_o0;
                }
                Err(_) => {
                    syst = BackendDAEUtil::updateAdjacencyMatrix(
                        syst.clone(),
                        openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
                        None,
                        &updateList_arr,
                        false,
                    )?;
                }
            }
        }
        if !((updateList_arr).is_empty())
            && !(Flags::isSet(Flags::DUMP_ASSC.clone())?)
            && Flags::isSet(Flags::BLT_DUMP.clone())?
        {
            metamodelica::print(literal!(
                "--- Some equations have been changed, for more information please use -d=dumpASSC.---\n\n"
            ));
        }
        Ok((ass1, ass2, syst))
    }

    fn generateLHSfromList(
        mut row_indices: metamodelica::Array<i32>,
        mut row_values: metamodelica::Array<metamodelica::Real>,
        mut vars: &BackendDAE::Variables,
    ) -> Result<metamodelica::Ref<DAE::Exp>> {
        let __ab_row_values = row_values.borrow();
        let mut lhs: metamodelica::Ref<DAE::Exp>;
        let mut length: i32 = metamodelica::arrayLength(row_indices.clone());
        if length == 0 {
            lhs = metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            });
        } else {
            lhs = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: (*metamodelica::index_checked(&__ab_row_values, 1)?).clone(),
                }),
                operator: DAE::Operator::MUL {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: BackendVariable::varExp(
                    &(BackendVariable::getVarAt(
                        vars,
                        ({
                            let __elt = (*metamodelica::index_checked(&row_indices.borrow(), 1)?).clone();
                            __elt
                        }),
                    )?),
                )?,
            });
        }
        for mut i in 2..=metamodelica::arrayLength(row_indices.clone()) {
            lhs = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: lhs,
                operator: DAE::Operator::ADD {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: metamodelica::Ref::new(DAE::Exp::RCONST {
                        real: (*metamodelica::index_checked(&__ab_row_values, i)?).clone(),
                    }),
                    operator: DAE::Operator::MUL {
                        ty: DAE::T_REAL_DEFAULT().clone(),
                    },
                    exp2: BackendVariable::varExp(
                        &(BackendVariable::getVarAt(
                            vars,
                            ({
                                let __elt = (*metamodelica::index_checked(&row_indices.borrow(), i)?).clone();
                                __elt
                            }),
                        )?),
                    )?,
                }),
            });
        }
        Ok(lhs)
    }

    pub(crate) fn anyChanges(mut linJac: &metamodelica::Ref<LinearJacobian>) -> Result<bool> {
        let mut changed: bool = false;
        for mut i in 1..=metamodelica::arrayLength(linJac.eq_marks.clone()) {
            if ({
                let __elt = (*metamodelica::index_checked(&linJac.eq_marks.borrow(), i)?).clone();
                __elt
            }) {
                changed = true;
                return Ok(changed);
            }
        }
        Ok(changed)
    }
}
