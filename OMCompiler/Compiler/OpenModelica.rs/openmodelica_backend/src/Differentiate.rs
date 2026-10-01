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

use crate::BackendDAECreate;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendUtil;
use crate::BackendVariable;
use crate::SymbolicJacobian::DAE_CJ;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::Ceval;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTpl;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_tpl::Tpl;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub(crate) const defaultMaxIter: i32 = 20;

// =============================================================================
// differentiation interfaces:
//  - createDifferentiatedCrefName
//  - createSeedCrefName
//  - differentiateEquation
//  - differentiateEquationTime
//  - differentiateExpCrefFullJacobian
//  - differentiateExpSolve
//  - differentiateExpTime
// =============================================================================
pub(crate) fn differentiateEquationTime(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inVariables: BackendDAE::Variables,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    Option<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outEquation: Option<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut diffData: BackendDAE::DifferentiateInputData;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut knvars: BackendDAE::Variables;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrEqnStr(&(literal!("### differentiateEquationTime\n")), inEquation, &(literal!(" w.r.t. time\n"))), '__try0);
        }
        funcs = BackendDAEUtil::getFunctions(&inShared);
        knvars = BackendDAEUtil::getGlobalKnownVarsFromShared(&inShared);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        diffData.allVars = Some(inVariables.clone());
        (eqn, funcs) = unwrap_break_err!(differentiateEquation(inEquation, DAE::crefTime().clone(), &diffData, openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, funcs.clone()), '__try0);
        outEquation = Some(eqn.clone());
        outShared = BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrEqnStr(&(literal!("### Result of differentiateEquationTime\n --> ")), &eqn, &(literal!("\n"))), '__try0);
        }
        Ok::<_, &'static str>((outEquation.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outEquation = __try0_o0;
        }
        Err(_) => {
            source = BackendEquation::equationSource(inEquation)?;
            Error::addSourceMessage(
                &(Error::INTERNAL_ERROR.clone()),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\nDifferentiate.differentiateEquationTime failed for "));
                    __mm_s.push_str(&*BackendDump::equationString(inEquation)?);
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                }],
                &(ElementSource::getElementSourceFileInfo(source.clone())),
            )?;
            outEquation = None;
        }
    }
    Ok((outEquation, outShared))
}

pub(crate) fn differentiateExpTime(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVariables: BackendDAE::Variables,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<BackendDAE::Shared>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut dexp: metamodelica::Ref<DAE::Exp>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut diffData: BackendDAE::DifferentiateInputData;
    let mut knvars: BackendDAE::Variables;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr(&(literal!("### differentiateExpTime\n ")), inExp.clone(), &(literal!(" w.r.t. time\n"))), '__try0);
        }
        funcs = BackendDAEUtil::getFunctions(&inShared);
        knvars = BackendDAEUtil::getGlobalKnownVarsFromShared(&inShared);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        (dexp, funcs) = unwrap_break_err!(differentiateExp(inExp.clone(), &(DAE::crefTime().clone()), &diffData, openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, &funcs, defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        outShared = BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr(&(literal!("### Result of differentiateExpTime\n --> ")), outExp.clone(), &(literal!("n"))), '__try0);
        }
        Ok::<_, &'static str>((
            dexp.clone(),
            diffData.clone(),
            funcs.clone(),
            knvars.clone(),
            outExp.clone(),
            outShared.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            dexp = __try0_o0;
            diffData = __try0_o1;
            funcs = __try0_o2;
            knvars = __try0_o3;
            outExp = __try0_o4;
            outShared = __try0_o5;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(
                    &(Error::NON_EXISTING_DERIVATIVE.clone()),
                    list![ExpressionBasics::printExpStr(inExp.clone())?, literal!("time")],
                    &(metamodelica::sourceInfo!("BackEnd/Differentiate.mo")),
                )?;
            }
            return Err(__try0_err);
        }
    }
    Ok((outExp, outShared))
}

pub(crate) fn differentiateExpSolve(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut functions: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut fac: metamodelica::List<metamodelica::Ref<DAE::Exp>> = Expression::factors(&inExp)?;
    let mut dexp: metamodelica::Ref<DAE::Exp>;
    let mut fun: metamodelica::Ref<AvlTreePathFunction::Tree>;
    ::match_deref::match_deref! { match &(List::split1OnTrue(&fac, &Expression::expHasCrefInIf, inCref.clone())?) {
        (Deref @ metamodelica::ListNode::Nil, _) => (),
        _ => return Err("pattern mismatch"),
    } };
    match '__try0: {
        fun = (::match_deref::match_deref! { match &(&functions) {
            Some(fun_) => {
                fun_.clone()
            },
            _ => {
                openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStrCrefStr(&(literal!("### differentiateExpSolve\n ")), inExp.clone(), &(literal!(" w.r.t. ")), &inCref, &(literal!("\n"))), '__try0);
        }
        (dexp, _) = unwrap_break_err!(differentiateExp(inExp.clone(), &inCref, &(BackendDAE::emptyInputData().clone()), openmodelica_backend_types::BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION, &fun, defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr(&(literal!("### Result of differentiateExpSolve\n --> ")), outExp.clone(), &(literal!("\n"))), '__try0);
        }
        Ok::<_, &'static str>((dexp.clone(), fun.clone(), outExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            dexp = __try0_o0;
            fun = __try0_o1;
            outExp = __try0_o2;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(
                    &(Error::NON_EXISTING_DERIVATIVE.clone()),
                    list![
                        ExpressionBasics::printExpStr(inExp.clone())?,
                        ComponentReference::crefStr(&inCref)?
                    ],
                    &(metamodelica::sourceInfo!("BackEnd/Differentiate.mo")),
                )?;
            }
            return Err(__try0_err);
        }
    }
    Ok(outExp)
}

pub(crate) fn differentiateExpCrefFullJacobian(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: BackendDAE::Variables,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<BackendDAE::Shared>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut dexp: metamodelica::Ref<DAE::Exp>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut diffData: BackendDAE::DifferentiateInputData;
    let mut knvars: BackendDAE::Variables;
    match '__try0: {
        funcs = BackendDAEUtil::getFunctions(&inShared);
        knvars = BackendDAEUtil::getGlobalKnownVarsFromShared(&inShared);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        (dexp, funcs) = unwrap_break_err!(differentiateExp(inExp.clone(), inCref, &diffData, openmodelica_backend_types::BackendDAE::DifferentiationType::DIFF_FULL_JACOBIAN, &funcs, defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        outShared = BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone());
        Ok::<_, &'static str>((
            dexp.clone(),
            diffData.clone(),
            funcs.clone(),
            knvars.clone(),
            outExp.clone(),
            outShared.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            dexp = __try0_o0;
            diffData = __try0_o1;
            funcs = __try0_o2;
            knvars = __try0_o3;
            outExp = __try0_o4;
            outShared = __try0_o5;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(
                    &(Error::NON_EXISTING_DERIVATIVE.clone()),
                    list![
                        ExpressionBasics::printExpStr(inExp.clone())?,
                        ComponentReference::crefStr(inCref)?
                    ],
                    &(metamodelica::sourceInfo!("BackEnd/Differentiate.mo")),
                )?;
            }
            return Err(__try0_err);
        }
    }
    Ok((outExp, outShared))
}

// =============================================================================
// further interface functions to differentiation
//  - differentiateEquation
//  - differentiateBackendDAE
//
// =============================================================================
pub(crate) fn differentiateEquation(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inDiffwrtCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    if let Ok((__pa0, __pa1)) = differentiateEquationFragile(
        inEquation,
        inDiffwrtCref.clone(),
        inInputData,
        inDiffType,
        inFunctionTree.clone(),
    ) {
        outEquation = metamodelica::Own::own(__pa0);
        outFunctionTree = metamodelica::Own::own(__pa1);
    } else {
        Error::addSourceMessage(
            &(Error::NON_EXISTING_DERIVATIVE.clone()),
            list![
                BackendDump::equationString(inEquation)?,
                ComponentReference::crefStr(&inDiffwrtCref)?
            ],
            &(metamodelica::sourceInfo!("BackEnd/Differentiate.mo")),
        )?;
        return Err("fail");
    }
    Ok((outEquation, outFunctionTree))
}

pub(crate) fn differentiateEquationFragile(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inDiffwrtCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        BackendDump::debugStrEqnStr(
            &(literal!("### differentiateEquation\n ")),
            inEquation,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" w.r.t. "));
                __mm_s.push_str(&*ComponentReference::crefStr(&inDiffwrtCref)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    (outEquation, outFunctionTree) = (::match_deref::match_deref! { match inEquation {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut op1: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut op2: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), &inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), &inDiffwrtCref, inInputData, inDiffType, &funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(&(list![op1, op2]), &ElementSource::addSymbolicTransformation, source.clone())?;
            (metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e2, source, attr: eqAttr } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut op1: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut op2: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            e1 = Expression::crefExp(cref.clone())?;
            (e1_1, funcs) = differentiateExp(e1.clone(), &inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), &inDiffwrtCref, inInputData, inDiffType, &funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1, after: e1_1.clone() });
            op2 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(&(list![op1, op2]), &ElementSource::addSymbolicTransformation, source.clone())?;
            (metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source, attr: eqAttr } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut op1: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), &inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            op1 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e1.clone(), after: e1_1.clone() });
            source = List::foldr(&(list![op1]), &ElementSource::addSymbolicTransformation, source.clone())?;
            (metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, source, attr: eqAttr } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut op1: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut op2: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), &inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), &inDiffwrtCref, inInputData, inDiffType, &funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(&(list![op1, op2]), &ElementSource::addSymbolicTransformation, source.clone())?;
            (metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_1, right: e2_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr: eqAttr, recordSize } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut op1: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut op2: metamodelica::Ref<DAE::SymbolicOperation>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), &inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), &inDiffwrtCref, inInputData, inDiffType, &funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(&(list![op1, op2]), &ElementSource::addSymbolicTransformation, source.clone())?;
            (metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_1, right: e2_1, source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst }, source, expand, attr: eqAttr } => {
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut alg: metamodelica::Ref<DAE::Algorithm>;
            let mut statementLst = (*statementLst).clone();
            (statementLst, funcs) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), &inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), &inFunctionTree, defaultMaxIter.clone())?;
            alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: statementLst.clone() });
            (metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg, source: source.clone(), expand: expand.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: expExpLst, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr: eqAttr } => {
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut eqnslst = (*eqnslst).clone();
            let mut eqns = (*eqns).clone();
            (eqnslst, funcs) = differentiateEquationsLst(metamodelica::AsArg::as_arg(&eqnslst), &inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), &inFunctionTree)?;
            (eqns, funcs) = differentiateEquations(metamodelica::AsArg::as_arg(&eqns), &inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), &funcs)?;
            (metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: expExpLst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: whenEqn, source, attr: eqAttr } => {
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut whenEqn = (*whenEqn).clone();
            (whenEqn, funcs) = differentiateWhenEquations(metamodelica::AsArg::as_arg(&whenEqn), &inDiffwrtCref, inInputData, inDiffType, inFunctionTree)?;
            (metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqn.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        _ => {
            Error::addSourceMessage(&(Error::NON_EXISTING_DERIVATIVE.clone()), list![BackendDump::equationString(inEquation)?, ComponentReference::crefStr(&inDiffwrtCref)?], &(metamodelica::sourceInfo!("BackEnd/Differentiate.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        BackendDump::debugStrEqnStr(
            &(literal!("### Result of differentiateEquation\n --> ")),
            &outEquation,
            &(literal!("\n")),
        )?;
    }
    Ok((outEquation, outFunctionTree))
}

fn differentiateEquations(
    mut inEquations: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inEquationsAccum: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outEquations, outFunctionTree) = 'mc: {
        let __mc_input = &**inEquations;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inEquationsAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eqn, tail: rest } => {
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eqn = (*eqn).clone();
                    (eqn, funcs) = differentiateEquation(metamodelica::AsArg::as_arg(&eqn), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree.clone())?;
                    eqns = metamodelica::cons(eqn.clone(), inEquationsAccum.clone());
                    (eqns, funcs) = differentiateEquations(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, &eqns, &funcs)?;
                    Ok((eqns.clone(), funcs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eqn, tail: _ } => {
                    Error::addSourceMessage(&(Error::NON_EXISTING_DERIVATIVE.clone()), list![BackendDump::equationString(metamodelica::AsArg::as_arg(&eqn))?, ComponentReference::crefStr(inDiffwrtCref)?], &(metamodelica::sourceInfo!("BackEnd/Differentiate.mo")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquations, outFunctionTree))
}

fn differentiateEquationsLst(
    mut inEquationsLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inEquationsLstAccum: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outEquationsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outEquationsLst, outFunctionTree) = 'mc: {
        let __mc_input = &**inEquationsLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inEquationsLstAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eqns, tail: rest } => {
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut eqnsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut eqns = (*eqns).clone();
                    (eqns, funcs) = differentiateEquations(metamodelica::AsArg::as_arg(&eqns), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), inFunctionTree)?;
                    eqnsLst = metamodelica::cons(eqns.clone(), inEquationsLstAccum.clone());
                    (eqnsLst, funcs) = differentiateEquationsLst(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, &eqnsLst, &funcs)?;
                    Ok((eqnsLst.clone(), funcs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eqns, tail: _ } => {
                    Error::addSourceMessage(&(Error::NON_EXISTING_DERIVATIVE.clone()), list![BackendDump::equationListString(metamodelica::AsArg::as_arg(&eqns), &(literal!("equation list")))?, ComponentReference::crefStr(inDiffwrtCref)?], &(metamodelica::sourceInfo!("BackEnd/Differentiate.mo")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquationsLst, outFunctionTree))
}

fn differentiateWhenEquations(
    mut inWhenEquations: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Ref<BackendDAE::WhenEquation>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outWhenEquations: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut elsewhenPart: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut delsewhenPart: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut oelsepart: Option<metamodelica::Ref<BackendDAE::WhenEquation>>;
    let mut whenStmtLst: metamodelica::List<BackendDAE::WhenOperator>;
    let mut stmtLst: metamodelica::List<BackendDAE::WhenOperator>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let __arc3 = &(*inWhenEquations);
    let BackendDAE::WHEN_STMTS {
        condition: __pa0,
        whenStmtLst: __pa1,
        elsewhenPart: __pa2,
    } = &**__arc3;
    condition = metamodelica::Own::own(__pa0);
    whenStmtLst = metamodelica::Own::own(__pa1);
    oelsepart = metamodelica::Own::own(__pa2);
    funcs = inFunctionTree;
    stmtLst = metamodelica::nil();
    for mut rs in &*whenStmtLst {
        let mut rs = rs.clone();
        rs = (match rs.clone() {
            BackendDAE::WhenOperator::ASSIGN {
                left: ref eleft,
                right: mut right,
                source: ref src,
            } => {
                let mut dright: metamodelica::Ref<DAE::Exp>;
                let mut dleft: metamodelica::Ref<DAE::Exp>;
                (dleft, funcs) = differentiateExp(
                    eleft.clone(),
                    inDiffwrtCref,
                    inInputData,
                    inDiffType,
                    &funcs,
                    defaultMaxIter.clone(),
                )?;
                (dright, funcs) = differentiateExp(
                    right.clone(),
                    inDiffwrtCref,
                    inInputData,
                    inDiffType,
                    &funcs,
                    defaultMaxIter.clone(),
                )?;
                BackendDAE::WhenOperator::ASSIGN {
                    left: dleft,
                    right: dright,
                    source: src.clone(),
                }
            }
            _ => rs,
        });
        stmtLst = metamodelica::cons(rs, stmtLst);
    }
    if (oelsepart).is_some() {
        let __pa4 = ::match_deref::match_deref! { match &(oelsepart) {
            Some(__pa4) => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        elsewhenPart = metamodelica::Own::own(__pa4);
        (delsewhenPart, funcs) =
            differentiateWhenEquations(&elsewhenPart, inDiffwrtCref, inInputData, inDiffType, funcs)?;
        oelsepart = Some(delsewhenPart);
    } else {
        oelsepart = None;
    }
    outWhenEquations = metamodelica::Ref::new(BackendDAE::WhenEquation {
        condition: condition,
        whenStmtLst: stmtLst,
        elsewhenPart: oelsepart,
    });
    outFunctionTree = funcs;
    Ok((outWhenEquations, outFunctionTree))
}

// =============================================================================
// main differentiation functions
//  - differentiateExp
//  - differentiateStatements
//
// =============================================================================
fn differentiateExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let debug: bool = false;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nDifferentiate Exp: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?);
            __mm_s.push_str(&*literal!(" w.r.t. "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::SCONST { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::CLKCONST { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            (metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, ty: tp } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            if ComponentReference::isStartCref(metamodelica::AsArg::as_arg(&cref)) {
                res = Expression::makeConstZero(metamodelica::AsArg::as_arg(&tp));
                functionTree = inFunctionTree.clone();
            } else {
                (res, functionTree) = differentiateCrefs(inExp, inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            }
            (res, functionTree)
        },
        Deref @ DAE::Exp::BINARY { .. } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateBinary(inExp, inDiffwrtCref, inInputData, inDiffType, inFunctionTree.clone(), maxIter - 1)?;
            res = ExpressionSimplify::simplifyBinaryExp(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            res = metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: res });
            res = ExpressionSimplify::simplifyUnaryExp(res);
            (res, functionTree)
        },
        Deref @ DAE::Exp::LBINARY { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::LUNARY { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::RELATION { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            (res2, functionTree) = differentiateExp(e3.clone(), inDiffwrtCref, inInputData, inDiffType, &functionTree, maxIter - 1)?;
            res = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: res1, expElse: res2 });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::ListNode::Cons { head: actual, tail: Deref @ metamodelica::ListNode::Cons { head: simplified, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            let mut lambda: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            lambda = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent(arcstr::literal!(BackendDAE::homotopyLambda), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            (e1, functionTree) = differentiateExp(actual.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
            (e2, functionTree) = differentiateExp(simplified.clone(), inDiffwrtCref, inInputData, inDiffType, &functionTree, maxIter)?;
            e3 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: lambda.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e1 }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lambda }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }) });
            (e3.clone(), functionTree)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Nil } } }, .. } if (Expression::expHasCref(e2.clone(), inDiffwrtCref.clone())? || Expression::expHasCref(e3.clone(), inDiffwrtCref.clone())?) => {
            return Err("fail")
        },
        Deref @ DAE::Exp::CALL { .. } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateCalls(inExp, inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone(), maxIter - 1)?;
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::RECORD { path: p, exps: expl, comp: strLst, ty: tp } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut sub: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            sub = metamodelica::nil();
            functionTree = inFunctionTree.clone();
            for mut e in &*expl.clone() {
                (e1, functionTree) = differentiateExp(e.clone(), inDiffwrtCref, inInputData, inDiffType, &functionTree, maxIter)?;
                sub = metamodelica::cons(e1, sub);
            }
            (metamodelica::Ref::new(DAE::Exp::RECORD { path: p.clone(), exps: sub.reverse(), comp: strLst.clone(), ty: tp.clone() }), functionTree)
        },
        Deref @ DAE::Exp::ARRAY { ty: tp, scalar: b, array: expl } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut expl = (*expl).clone();
            (expl, functionTree) = List::map3Fold(metamodelica::AsArg::as_arg(&expl), &({ let __pe_b5 = maxIter - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())?;
            res = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: b.clone(), array: expl.clone() });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::MATRIX { ty: tp, integer: i, matrix } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut dmatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            (dmatrix, functionTree) = List::mapFoldList(metamodelica::AsArg::as_arg(&matrix), &({ let __pe_b1 = inDiffwrtCref.clone(); let __pe_b2 = inInputData.clone(); let __pe_b3 = inDiffType; let __pe_b5 = maxIter - 1; move |__pe_a0, __pe_a4| differentiateExp(__pe_a0, &__pe_b1, &__pe_b2, __pe_b3.clone(), &__pe_a4, __pe_b5.clone()) }), inFunctionTree.clone())?;
            res = metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: i.clone(), matrix: dmatrix });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::TUPLE { PR: expl } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut expl = (*expl).clone();
            (expl, functionTree) = List::map3Fold(metamodelica::AsArg::as_arg(&expl), &({ let __pe_b5 = maxIter - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())?;
            res = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl.clone() });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::CAST { ty: tp, exp: e1 } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: res }), functionTree)
        },
        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            res = Expression::makeASUB(res1, ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(&(s.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: tp } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            if !(referenceEq(&*(e1.clone()),&*(&*res1))) {
                res = metamodelica::Ref::new(DAE::Exp::TSUB { exp: res1, ix: i.clone(), ty: tp.clone() });
                (res, _) = ExpressionSimplify::simplify1(res)?;
            } else {
                res = inExp;
            }
            (res, functionTree)
        },
        e1 @ Deref @ DAE::Exp::RSUB { .. } => {
            let mut p1: metamodelica::Ref<Absyn::Path>;
            let mut p2: metamodelica::Ref<Absyn::Path>;
            let mut b: bool;
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut strLst: metamodelica::List<ArcStr>;
            let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e1 = (*e1).clone();
            (res, b) = ExpressionSimplify::simplify(e1.clone())?;
            if b {
                (res, functionTree) = differentiateExp(res, inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            } else {
                (res1, functionTree) = differentiateExp(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
                if !(referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(&*res1))) {
                    match '__try0: {
                        (expl, strLst) = (::match_deref::match_deref! { match &(&*res1) {
        Deref @ DAE::Exp::RECORD { exps: __esc_expl, comp: __esc_strLst, .. } => {
            expl = (*__esc_expl).clone();
            strLst = (*__esc_strLst).clone();
            (expl.clone(), strLst.clone())
        },
        Deref @ DAE::Exp::CALL { path: p1, expLst: __esc_expl, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, varLst: __esc_varLst, .. }, .. } } if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) => {
            expl = (*__esc_expl).clone();
            varLst = (*__esc_varLst).clone();
            (expl.clone(), ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))
        },
        _ => break '__try0 Err::<_, _>("match: no arm matched"),
    } });
                        res = unwrap_break_err!((expl).get(unwrap_break_err!(List::position1OnTrue(&strLst, &fnptr!(stringEq, ArcStr, ArcStr), var_field!((*e1).fieldName, DAE::Exp::RSUB).clone()), '__try0)), '__try0);
                        Ok::<_, &'static str>((res.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            res = __try0_o0;
                        }
                        Err(_) => {
                            assign_variant_field!(e1 => DAE::Exp::RSUB; exp = res1.clone());
                            (res, _) = ExpressionSimplify::simplify1(e1.clone())?;
                        }
                    }
                }
            }
            (res, functionTree)
        },
        Deref @ DAE::Exp::SIZE { .. } => {
            (inExp, inFunctionTree.clone())
        },
        Deref @ DAE::Exp::REDUCTION { expr: __inExp_expr, iterators: __inExp_iterators, reductionInfo: __inExp_reductionInfo } => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(__inExp_expr.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            if !(referenceEq(&*(__inExp_expr.clone()),&*(&*res1))) {
                res = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: __inExp_reductionInfo.clone(), expr: res1, iterators: __inExp_iterators.clone() });
                (res, _) = ExpressionSimplify::simplify1(res)?;
            } else {
                res = inExp;
            }
            (res, functionTree)
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut stp: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            s1 = ExpressionBasics::printExpStr(inExp.clone())?;
            s2 = ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?;
            stp = TypesDump::printTypeStr(Expression::r#typeof(inExp)?);
            Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- differentiateExp ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*stp); __mm_s.push_str(&*literal!(" w.r.t ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Differentiate-Exp-result: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateStatements(
    mut inStmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inStmtsAccum: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outDiffedStmts, outFunctionTree) = 'mc: {
        let __mc_input = &**inStmts;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inStmtsAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_ASSIGN { type_, exp1: lhs, exp: rhs, source }, tail: restStatements } => {
                    let mut derivedLHS: metamodelica::Ref<DAE::Exp>;
                    let mut derivedRHS: metamodelica::Ref<DAE::Exp>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedLHS, functions) = differentiateExp(lhs.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
                    (derivedRHS, functions) = differentiateExp(rhs.clone(), inDiffwrtCref, inInputData, inDiffType, &functions, maxIter)?;
                    (derivedRHS, _) = ExpressionSimplify::simplify(derivedRHS.clone())?;
                    if Expression::isZero(&derivedLHS)? {
                        derivedStatements1 = list![currStatement.clone()];
                    } else {
                        derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: derivedLHS.clone(), exp: derivedRHS.clone(), source: source.clone() }), currStatement.clone()];
                    }
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, exp: rhs, source, .. }, tail: restStatements } => {
                            let mut derivedRHS: metamodelica::Ref<DAE::Exp>;
                            let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                            let mut dexpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut expLstRHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut exptl: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>;
                            let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut optDerivedStatements1: metamodelica::List<Option<metamodelica::Ref<DAE::Statement>>>;
                            (dexpLst, functions) = List::map3Fold(metamodelica::AsArg::as_arg(&expLst), &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())?;
                            let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(differentiateExp(rhs.clone(), inDiffwrtCref, inInputData, inDiffType, &functions, maxIter)?) {
                                (__pa1 @ Deref @ DAE::Exp::TUPLE { PR: __pa0 }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            expLstRHS = metamodelica::Own::own(__pa0);
                            derivedRHS = metamodelica::Own::own(__pa1);
                            functions = metamodelica::Own::own(__pa2);
                            let __pa4 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(derivedRHS.clone())?) {
                                (Deref @ DAE::Exp::TUPLE { PR: __pa4 }, _) => __pa4.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            expLstRHS = metamodelica::Own::own(__pa4);
                            exptl = List::zip(dexpLst.clone(), expLstRHS.clone());
                            optDerivedStatements1 = List::map2(exptl.clone(), &move |__a0: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), __a1: metamodelica::Ref<DAE::ElementSource>, __a2: metamodelica::Ref<AvlTreePathFunction::Tree>| makeAssignmentfromTuple(&__a0, __a1, &__a2), source.clone(), inFunctionTree.clone())?;
                            derivedStatements1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
                for mut s in (optDerivedStatements1.clone()).into_iter().cloned() {
                            if !((s).is_some()) { continue; }
                            let __x = Util::getOption(s.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            derivedStatements2 = listAppend(derivedStatements1.clone(), list![currStatement.clone()]);
                            derivedStatements1 = listAppend(derivedStatements2.clone(), inStmtsAccum.clone());
                            (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements1, &functions, maxIter)?;
                            Ok((derivedStatements2.clone(), functions.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, exp: rhs @ Deref @ DAE::Exp::CALL { .. }, type_, source }, tail: restStatements } => {
                            let mut derivedRHS: metamodelica::Ref<DAE::Exp>;
                            let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                            let mut dexpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut optDerivedStatements1: metamodelica::List<Option<metamodelica::Ref<DAE::Statement>>>;
                            let mut type_ = (*type_).clone();
                            (dexpLst, functions) = List::map3Fold(metamodelica::AsArg::as_arg(&expLst), &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())?;
                            let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(differentiateExp(rhs.clone(), inDiffwrtCref, inInputData, inDiffType, &functions, maxIter)?) {
                                (__pa1 @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: __pa0, .. }, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            type_ = metamodelica::Own::own(__pa0);
                            derivedRHS = metamodelica::Own::own(__pa1);
                            functions = metamodelica::Own::own(__pa2);
                            optDerivedStatements1 = list![Some(metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: type_.clone(), expExpLst: dexpLst.clone(), exp: derivedRHS.clone(), source: source.clone() }))];
                            derivedStatements1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
                for mut s in (optDerivedStatements1.clone()).into_iter().cloned() {
                            if !((s).is_some()) { continue; }
                            let __x = Util::getOption(s.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            derivedStatements2 = listAppend(derivedStatements1.clone(), list![currStatement.clone()]);
                            derivedStatements1 = listAppend(derivedStatements2.clone(), inStmtsAccum.clone());
                            (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements1, &functions, maxIter)?;
                            Ok((derivedStatements2.clone(), functions.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs, exp: rhs, type_, source }, tail: restStatements } => {
                    let mut derivedLHS: metamodelica::Ref<DAE::Exp>;
                    let mut derivedRHS: metamodelica::Ref<DAE::Exp>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedLHS, functions) = differentiateExp(lhs.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
                    (derivedRHS, functions) = differentiateExp(rhs.clone(), inDiffwrtCref, inInputData, inDiffType, &functions, maxIter)?;
                    (derivedRHS, _) = ExpressionSimplify::simplify(derivedRHS.clone())?;
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: derivedLHS.clone(), exp: derivedRHS.clone(), source: source.clone() }), currStatement.clone()];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_, iterIsArray, iter: ident, range: exp, statementLst, source, sub_iters }, tail: restStatements } => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut inputData: BackendDAE::DifferentiateInputData;
                    let mut controlVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    cref = ComponentReferenceBasics::makeCrefIdent(ident.clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
                    controlVar = metamodelica::Ref::new(BackendDAE::Var { varName: cref.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
                    inputData = addGlobalVars(list![controlVar.clone()], inInputData.clone())?;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), inDiffwrtCref, &inputData, inDiffType, &(metamodelica::nil()), inFunctionTree, maxIter)?;
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: type_.clone(), iterIsArray: iterIsArray.clone(), iter: ident.clone(), range: exp.clone(), statementLst: derivedStatements1.clone(), source: source.clone(), sub_iters: sub_iters.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst, else_: Deref @ DAE::Else::NOELSE { .. }, source }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), inFunctionTree, maxIter)?;
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst, else_: Deref @ DAE::Else::ELSEIF { exp: elseif_exp, statementLst: elseif_statementLst, else_: elseif_else_ }, source }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), inFunctionTree, maxIter)?;
                    (derivedStatements2, functions) = differentiateStatements(&(list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: elseif_exp.clone(), statementLst: elseif_statementLst.clone(), else_: elseif_else_.clone(), source: source.clone() })]), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), &functions, maxIter)?;
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: metamodelica::Ref::new(DAE::Else::ELSE { statementLst: derivedStatements2.clone() }), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst, else_: Deref @ DAE::Else::ELSE { statementLst: else_statementLst }, source }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), inFunctionTree, maxIter)?;
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&else_statementLst), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), &functions, maxIter)?;
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: metamodelica::Ref::new(DAE::Else::ELSE { statementLst: derivedStatements2.clone() }), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHILE { exp, statementLst, source }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), inFunctionTree, maxIter)?;
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: exp.clone(), statementLst: derivedStatements1.clone(), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp, initialCall, statementLst, elseWhen: None, source, .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), inFunctionTree, maxIter)?;
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: exp.clone(), conditions: metamodelica::nil(), initialCall: initialCall.clone(), statementLst: derivedStatements1.clone(), elseWhen: None, source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp, initialCall, statementLst, elseWhen: Some(stmt), source, .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut dstmt: metamodelica::Ref<DAE::Statement>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&statementLst), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), inFunctionTree, maxIter)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateStatements(&(list![stmt.clone()]), inDiffwrtCref, inInputData, inDiffType, &(metamodelica::nil()), &functions, maxIter)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    dstmt = metamodelica::Own::own(__pa0);
                    functions = metamodelica::Own::own(__pa1);
                    derivedStatements1 = list![metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: exp.clone(), conditions: metamodelica::nil(), initialCall: initialCall.clone(), statementLst: derivedStatements1.clone(), elseWhen: Some(dstmt.clone()), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements2, &functions, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, inStmtsAccum, inFunctionTree, maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TERMINATE { .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut derivedStatements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements1, inFunctionTree, maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_REINIT { .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements1, inFunctionTree, maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_NORETCALL { .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements1, inFunctionTree, maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_RETURN { .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements1, inFunctionTree, maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_BREAK { .. }, tail: restStatements } => {
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(metamodelica::AsArg::as_arg(&restStatements), inDiffwrtCref, inInputData, inDiffType, &derivedStatements1, inFunctionTree, maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut currStatement: metamodelica::Ref<DAE::Statement>;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        let __pa0 = ::match_deref::match_deref! { match &((*inStmts)) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        currStatement = metamodelica::Own::own(__pa0);
                        s1 = DAEDump::ppStatementStr(currStatement.clone());
                        s2 = ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?;
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- differentiateStatements ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" w.r.t: ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) })?;
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
    Ok((outDiffedStmts, outFunctionTree))
}

fn isDiscreteAssignStatment(mut inStmt: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut out: bool;
    out = (match &**inStmt {
        DAE::Statement::STMT_ASSIGN { type_: tp, .. } => Types::isDiscreteType(tp),
        DAE::Statement::STMT_ASSIGN_ARR { type_: tp, .. } => Types::isDiscreteType(tp),
        DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, .. } => Types::isDiscreteType(tp),
        _ => false,
    });
    out
}

fn makeAssignmentfromTuple(
    mut inTpl: &(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>),
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<Option<metamodelica::Ref<DAE::Statement>>> {
    let mut outStmt: Option<metamodelica::Ref<DAE::Statement>>;
    outStmt = (::match_deref::match_deref! { match &(inTpl) {
        (e1 @ Deref @ DAE::Exp::CREF { ty: tp, .. }, e2) => {
            Some(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e1.clone(), exp: e2.clone(), source: source }))
        },
        (e1 @ Deref @ DAE::Exp::CALL { .. }, e2) if (Expression::isRecordCall(metamodelica::AsArg::as_arg(&e1), inFunctionTree)?) => {
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(e1.clone())?;
            Some(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: tp, exp1: e1.clone(), exp: e2.clone(), source: source }))
        },
        (e1, e2) if (Expression::isZero(metamodelica::AsArg::as_arg(&e1))?) => {
            None
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStmt)
}

// =============================================================================
// help functions for differentiation
//  - differentiateCrefs
//  - differentiateCalls
//  - differentiateBinary (e.g.: ADD, SUB, MUL, DIV, POW, ...
//
// =============================================================================
fn differentiateCrefs(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> =
        metamodelica::Ref::new(AvlTreePathFunction::Tree::EMPTY);
    let debug: bool = false;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nDifferentiate Exp-Cref: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?);
            __mm_s.push_str(&*literal!(" w.r.t. "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (outDiffedExp, outFunctionTree) = ({
        let mut diffed_exps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        'mc: {
            let __mc_input = (inExp.clone(), &**inDiffwrtCref, inInputData, inDiffType);
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut cr = (*cr).clone();
                        cr = ComponentReference::prependStringCref(arcstr::literal!(BackendDAE::functionDerivativeNamePrefix), metamodelica::AsArg::as_arg(&cr))?;
                        cr = ComponentReference::prependStringCref(matrixName.clone(), metamodelica::AsArg::as_arg(&cr))?;
                        res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp @ Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path }, .. } }, _, _, _) => {
                        let mut compType: metamodelica::Ref<DAE::Type>;
                        let mut e1: metamodelica::Ref<DAE::Exp>;
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                        let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                        let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                        expl = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| Expression::generateCrefsExpFromExpVar(&__a0, &__a1), cr.clone())?;
                        expl_1 = metamodelica::nil();
                        outFunctionTree = inFunctionTree.clone();
                        for mut comp in &*expl {
                            (e1, outFunctionTree) = differentiateExp(comp.clone(), inDiffwrtCref, inInputData, inDiffType, &outFunctionTree, maxIter)?;
                            compType = Expression::r#typeof(comp.clone())?;
                            if Expression::isZero(&e1)? && (Types::isString(&compType) || Types::isBoolean(&compType) || Types::isEnumeration(&compType)) {
                                e1 = zeroOfType(&compType);
                            }
                            expl_1 = metamodelica::cons(e1.clone(), expl_1.clone());
                        }
                        res = metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expl_1.clone().reverse(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) });
                        Ok(((res.clone(), outFunctionTree.clone()), outFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                outFunctionTree = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp @ Deref @ DAE::Type::T_ARRAY { .. } }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut cr = (*cr).clone();
                        cr = ComponentReference::prependStringCref(arcstr::literal!(BackendDAE::functionDerivativeNamePrefix), metamodelica::AsArg::as_arg(&cr))?;
                        cr = ComponentReference::prependStringCref(matrixName.clone(), metamodelica::AsArg::as_arg(&cr))?;
                        res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, _, _, diffType) => {
                                if !(((match diffType.clone() {
                    BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. } => false,
                    _ => true,
                }))) { return Err("guard") }
                                let mut e1: metamodelica::Ref<DAE::Exp>;
                                let mut res: metamodelica::Ref<DAE::Exp>;
                                let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                                let true = (Flags::isSet(Flags::NF_SCALARIZE.clone())?) else { return Err("pattern mismatch") };
                                let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)) {
                                    (__pa0, true) => __pa0.clone(),
                                    _ => return Err("pattern mismatch"),
                                } };
                                e1 = metamodelica::Own::own(__pa0);
                                (res, outFunctionTree) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
                                Ok(((res.clone(), outFunctionTree.clone()), outFunctionTree.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                outFunctionTree = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _, _, _) => {
                        Ok((e.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, _, _) => {
                        let mut one: metamodelica::Ref<DAE::Exp>;
                        let true = (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), inDiffwrtCref)?) else { return Err("pattern mismatch") };
                        (one, _) = Expression::makeOneExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((one.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION { .. }) => {
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::DIFF_FULL_JACOBIAN { .. }) => {
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { knownVars: Some(knvars), .. }, _) => {
                        let mut var: metamodelica::Ref<BackendDAE::Var>;
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&knvars))?;
                        let false = (BackendVariable::isVarOnTopLevelAndInput(&var)) else { return Err("pattern mismatch") };
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { allVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                        let mut kind: BackendDAE::VarKind;
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        let (__t1, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&timevars))?;
                        let __arc2 = __t1.clone();
                        let BackendDAE::VAR { varKind: __pa0, .. } = &*__arc2;
                        kind = metamodelica::Own::own(__pa0);
                        let true = (listMember(kind.clone(), list![openmodelica_backend_types::BackendDAE::VarKind::DISCRETE]) || !(Types::isReal(metamodelica::AsArg::as_arg(&tp)))) else { return Err("pattern mismatch") };
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                        let mut var: metamodelica::Ref<BackendDAE::Var>;
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut cr = (*cr).clone();
                        (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&timevars))?;
                        let true = (BackendVariable::isDummyStateVar(&var)) else { return Err("pattern mismatch") };
                        cr = ComponentReference::crefPrefixDer(cr.clone());
                        res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&timevars))?;
                        res = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: list![e.clone()], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) });
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
                        cr1 = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                        BackendVariable::getVar(cr1.clone(), metamodelica::AsArg::as_arg(&timevars))?;
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&timevars))?;
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut cr = (*cr).clone();
                        cr = ComponentReference::prependStringCref(arcstr::literal!(BackendDAE::functionDerivativeNamePrefix), metamodelica::AsArg::as_arg(&cr))?;
                        cr = ComponentReference::prependStringCref(matrixName.clone(), metamodelica::AsArg::as_arg(&cr))?;
                        res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { ty: tp, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        (res, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { independenentVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                        let mut scalarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                        let mut arrayType: metamodelica::Ref<DAE::Type>;
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut res1: metamodelica::Ref<DAE::Exp>;
                        let mut scalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                        let mut cr = (*cr).clone();
                        let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                        (scalarLst, _) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&timevars))?;
                        arrayType = ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cr))?;
                        if !((scalarLst).is_empty()) && ((scalarLst).len() as i32) != Types::getDimensionProduct(&arrayType)? {
                            scalarCrefs = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
                            outFunctionTree = inFunctionTree.clone();
                            for mut cref in &*scalarCrefs {
                                (res1, outFunctionTree) = differentiateCrefs(Expression::crefExp(cref.clone())?, inDiffwrtCref, inInputData, inDiffType.clone(), &outFunctionTree, maxIter)?;
                                diffed_exps = metamodelica::cons(res1.clone(), diffed_exps.clone());
                            }
                            res = Expression::listToArray(&(diffed_exps.clone().reverse()), &(TypesDump::getDimensions(&arrayType)))?;
                        } else {
                            cr = createSeedCrefName(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&matrixName))?;
                            res = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                        }
                        Ok(((res.clone(), inFunctionTree.clone()), outFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                outFunctionTree = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { allVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                        let mut var: metamodelica::Ref<BackendDAE::Var>;
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut cr = (*cr).clone();
                        let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&timevars))?) {
                            (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        var = metamodelica::Own::own(__pa0);
                        let false = (BackendVariable::isStateVar(&var)) else { return Err("pattern mismatch") };
                        cr = ComponentReference::createDifferentiatedCrefName(metamodelica::AsArg::as_arg(&cr), inDiffwrtCref.clone(), metamodelica::AsArg::as_arg(&matrixName))?;
                        res = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                        let mut var: metamodelica::Ref<BackendDAE::Var>;
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut cr = (*cr).clone();
                        let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&timevars))?) {
                            (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        var = metamodelica::Own::own(__pa0);
                        let false = (BackendVariable::isStateVar(&var)) else { return Err("pattern mismatch") };
                        cr = ComponentReference::createDifferentiatedCrefName(metamodelica::AsArg::as_arg(&cr), inDiffwrtCref.clone(), metamodelica::AsArg::as_arg(&matrixName))?;
                        res = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                        Ok((res.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                        let mut zero: metamodelica::Ref<DAE::Exp>;
                        (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                        Ok((zero.clone(), inFunctionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut s1: ArcStr;
                        let mut s2: ArcStr;
                        let mut serr: ArcStr;
                        let mut se1: ArcStr;
                        let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                        s1 = ExpressionBasics::printExpStr(inExp.clone())?;
                        se1 = TypesDump::printTypeStr(Expression::r#typeof(inExp.clone())?);
                        s2 = ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?;
                        serr = stringAppendList(list![literal!("\n- differentiateCrefs "), s1.clone(), literal!(" type:"), se1.clone(), literal!(" w.r.t: "), s2.clone(), literal!(" failed\n")]);
                        Debug::trace(serr.clone())?;
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
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Differentiate-ExpCref-result: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((outDiffedExp, outFunctionTree))
}

pub(crate) fn createDiffedCrefName(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inMatrixName: ArcStr,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    subs = ComponentReference::crefLastSubs(inCref)?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref)?;
    outCref =
        ComponentReference::prependStringCref(arcstr::literal!(BackendDAE::functionDerivativeNamePrefix), &outCref)?;
    outCref = ComponentReference::prependStringCref(inMatrixName, &outCref)?;
    outCref = ComponentReference::crefSetLastSubs(&outCref, &subs)?;
    outCref = ComponentReference::crefSetLastType(&outCref, &(ComponentReference::crefLastType(inCref)?))?;
    Ok(outCref)
}

pub(crate) fn createSeedCrefName(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inMatrixName: &ArcStr,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let debug: bool = false;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("inCref: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("after full type  "));
            __mm_s.push_str(&*TypesDump::printTypeStr(ComponentReference::crefTypeConsiderSubs(
                inCref,
            )?));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    subs = ComponentReference::crefLastSubs(inCref)?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref)?;
    outCref = ComponentReference::crefSetLastType(&outCref, &(DAE::T_UNKNOWN_DEFAULT().clone()))?;
    outCref = ComponentReference::joinCrefs(
        &outCref,
        ComponentReferenceBasics::makeCrefIdent(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Seed"));
                __mm_s.push_str(&*inMatrixName);
                ArcStr::from(__mm_s)
            },
            DAE::T_UNKNOWN_DEFAULT().clone(),
            metamodelica::nil(),
        ),
    )?;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("after join: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(
                ComponentReference::expandCref(&outCref, true)?,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    outCref = ComponentReference::crefSetLastSubs(&outCref, &subs)?;
    outCref = ComponentReference::crefSetLastType(&outCref, &(ComponentReference::crefLastType(inCref)?))?;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("outCref: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&outCref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(outCref)
}

pub(crate) fn isSeedCref<'__b>(mut cr: &'__b metamodelica::Ref<DAE::ComponentRef>) -> bool {
    '__tco: loop {
        match &**cr {
            DAE::ComponentRef::CREF_IDENT { .. } => {
                return StringUtil::startsWith(
                    var_field!((**cr).ident, DAE::ComponentRef::CREF_IDENT).clone(),
                    literal!("Seed"),
                );
            }
            DAE::ComponentRef::CREF_QUAL { .. } => {
                cr = var_field!((**cr).componentRef, DAE::ComponentRef::CREF_QUAL);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

fn differentiateCalls(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDiffwrtCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let debug: bool = false;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nDifferentiate Exp-Call: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?);
            __mm_s.push_str(&*literal!(" w.r.t. "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inDiffwrtCref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::ListNode::Cons { head: actual, tail: Deref @ metamodelica::ListNode::Cons { head: simplified, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, _, _, _) => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (e1, funcs) = differentiateExp(actual.clone(), &inDiffwrtCref, &inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (_, funcs) = differentiateExp(simplified.clone(), &inDiffwrtCref, &inInputData, inDiffType, &funcs, maxIter)?;
            (e1, funcs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, BackendDAE::DifferentiateInputData { independenentVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::makeCrefQual(arcstr::literal!(DAE::previousNamePrefix), tp.clone(), metamodelica::nil(), cr.clone());
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&timevars))?) {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => (),
                _ => return Err("pattern mismatch"),
            } };
            cr = createSeedCrefName(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&matrixName))?;
            res = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
            (res, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, attr }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
            (metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: 2 })], attr: attr.clone() }), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
            let mut i = (*i).clone();
            i = i.clone() + 1;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() })], attr: attr.clone() }), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: true }) => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut cj: metamodelica::Ref<DAE::ComponentRef>;
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            cj = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: arcstr::literal!(DAE_CJ), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            cr = Expression::expCref(metamodelica::AsArg::as_arg(&e))?;
            tp = Expression::r#typeof(e.clone())?;
            cr = createSeedCrefName(&cr, metamodelica::AsArg::as_arg(&matrixName))?;
            res = Expression::makeCrefExp(cr, tp)?;
            res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: Expression::makeCrefExp(cj, DAE::T_REAL_DEFAULT().clone())?, operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: res });
            (res, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, _) => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            cr = Expression::expCref(metamodelica::AsArg::as_arg(&e))?;
            tp = Expression::r#typeof(e.clone())?;
            cr = ComponentReference::crefPrefixDer(cr);
            cr = ComponentReference::createDifferentiatedCrefName(&cr, inDiffwrtCref.clone(), metamodelica::AsArg::as_arg(&matrixName))?;
            res = Expression::makeCrefExp(cr, tp.clone())?;
            if ComponentReferenceBasics::crefEqual(&(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("$"), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() })), &inDiffwrtCref)? {
                (res, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            }
            (res, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, _) => {
            let mut zero: metamodelica::Ref<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&(Expression::r#typeof(e.clone())?))))?;
            (zero, inFunctionTree.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, builtin: false, .. }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, _) if (!(Expression::isRecordCall(metamodelica::AsArg::as_arg(&e), &inFunctionTree)?)) => {
            let mut zero: metamodelica::Ref<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            (zero, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil } }, _, _, _) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res, funcs) = differentiateCallExp1Arg(metamodelica::AsArg::as_arg(&name), e.clone(), &inDiffwrtCref, &inInputData, inDiffType, inFunctionTree.clone(), maxIter)?;
            (res, funcs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e1 @ Deref @ DAE::Exp::RCONST { real: __rlit_0 }, tail: Deref @ metamodelica::ListNode::Nil } } }, _, _, _) if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, attr: attr @ Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: expl @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } }, _, _, _) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res, funcs) = differentiateCallExpNArg(name.clone(), metamodelica::AsArg::as_arg(&expl), attr.clone(), &inDiffwrtCref, &inInputData, inDiffType, inFunctionTree.clone(), maxIter)?;
            (res, funcs)
        },
        (e @ Deref @ DAE::Exp::CALL { .. }, _, _, _) => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (e1, funcs) = differentiateFunctionCall(e.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree.clone(), maxIter)?;
            (e1, _, _, _) = Inline::inlineExp(e1, (Some(funcs.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE]), DAE::emptyElementSource().clone());
            (e1, funcs)
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut serr: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            s1 = ExpressionBasics::printExpStr(inExp)?;
            s2 = ComponentReferenceBasics::printComponentRefStr(&inDiffwrtCref)?;
            serr = stringAppendList(list![literal!("\n- Function differentiateCalls failed. differentiateExp "), s1, literal!(" w.r.t: "), s2, literal!(" failed\n")]);
            Debug::trace(serr)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Differentiate-ExpCall-result: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateCallExp1Arg(
    mut name: &ArcStr,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFuncs: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((name.clone(), exp.clone())) {
        (Deref @ "pre", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            (exp_1, inFuncs)
        },
        (Deref @ "previous", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            (exp_1, inFuncs)
        },
        (Deref @ "$getPart", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (exp_1, funcs) = differentiateExp(exp, inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            (exp_1, funcs)
        },
        (Deref @ "firstTick", _) => {
            (exp, inFuncs)
        },
        (Deref @ "interval", _) => {
            (exp, inFuncs)
        },
        (Deref @ "sin", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("cos"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::MUL { ty: tp }, exp2: exp_1 }), funcs)
        },
        (Deref @ "cos", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sin"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: exp_2 }), operator: DAE::Operator::MUL { ty: tp }, exp2: exp_1 }), funcs)
        },
        (Deref @ "tan", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("cos"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp })], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1 }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::ADD { ty: tp }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }) }), funcs)
        },
        (Deref @ "asin", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sqrt"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp }) })], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp }, exp2: exp_2 }), funcs)
        },
        (Deref @ "acos", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sqrt"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp }) })], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp }, exp2: exp_2 }) }), funcs)
        },
        (Deref @ "atan", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp }, exp2: exp }) }) }), funcs)
        },
        (Deref @ "sinh", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("cosh"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::MUL { ty: tp }, exp2: exp_2 }), funcs)
        },
        (Deref @ "cosh", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sinh"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::MUL { ty: tp }, exp2: exp_2 }), funcs)
        },
        (Deref @ "tanh", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("cosh"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::POW { ty: tp }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) }) }), funcs)
        },
        (Deref @ "exp", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("exp"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::MUL { ty: tp }, exp2: exp_1 }), funcs)
        },
        (Deref @ "log", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp }, exp2: exp }), funcs)
        },
        (Deref @ "log10", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("log"), list![metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) })], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp, operator: DAE::Operator::MUL { ty: tp }, exp2: exp_2 }) }), funcs)
        },
        (Deref @ "sqrt", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sqrt"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp }, exp2: exp_2 }) }), funcs)
        },
        (Deref @ "abs", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sign"), list![exp], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::MUL { ty: tp }, exp2: exp_1 }), funcs)
        },
        (Deref @ "sign", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            (exp_1, inFuncs)
        },
        (Deref @ "transpose", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp, inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("transpose"), list![exp_1], tp);
            (exp_2, funcs)
        },
        (Deref @ "sum", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp, inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sum"), list![exp_1], tp);
            (exp_2, funcs)
        },
        (Deref @ "max", Deref @ DAE::Exp::ARRAY { array: expl, ty: tp, .. }) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp = (*tp).clone();
            tp = Types::arrayElementType(metamodelica::AsArg::as_arg(&tp));
            exp_1 = createFromNCall2ArgsCall(literal!("max"), metamodelica::AsArg::as_arg(&expl), tp.clone())?;
            (exp_2, funcs) = differentiateExp(exp_1, inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            (exp_2, funcs)
        },
        (Deref @ "min", Deref @ DAE::Exp::ARRAY { array: expl, ty: tp, .. }) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp = (*tp).clone();
            tp = Types::arrayElementType(metamodelica::AsArg::as_arg(&tp));
            exp_1 = createFromNCall2ArgsCall(literal!("min"), metamodelica::AsArg::as_arg(&expl), tp.clone())?;
            (exp_2, funcs) = differentiateExp(exp_1, inDiffwrtCref, inInputData, inDiffType, &inFuncs, maxIter)?;
            (exp_2, funcs)
        },
        (Deref @ "floor", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            (exp_1, inFuncs)
        },
        (Deref @ "ceil", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            (exp_1, inFuncs)
        },
        (Deref @ "integer", _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&tp)))?;
            (exp_1, inFuncs)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

fn createFromNCall2ArgsCall(
    mut funcName: ArcStr,
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut tp: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut result: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*expl)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    rest = metamodelica::Own::own(__pa2);
    result = Expression::makePureBuiltinCall(funcName.clone(), list![e1, e2], tp.clone());
    for mut elem in &*rest {
        result = Expression::makePureBuiltinCall(funcName.clone(), list![result, elem.clone()], tp.clone());
    }
    Ok(result)
}

fn differentiateCallExpNArg(
    mut name: ArcStr,
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAttr: metamodelica::Ref<DAE::CallAttributes>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((name.clone(), inExpl.clone(), inAttr.clone())) {
        (Deref @ "smooth", Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            e1 = metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() - 1 });
            res2 = if (intGe(i.clone(), 1)) {Expression::makePureBuiltinCall(literal!("smooth"), list![e1, res1], tp.clone())} else {res1};
            (res2, funcs)
        },
        (Deref @ "noEvent", Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            res1 = Expression::makePureBuiltinCall(literal!("noEvent"), list![res1], tp.clone());
            (res1, funcs)
        },
        (Deref @ "atan2", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            e2 = Expression::makeDiv(e.clone(), e1.clone())?;
            (res1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            res2 = Expression::addNoEventToRelations(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: e1.clone(), expElse: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2 }) }) }) }))?;
            (res2, funcs)
        },
        (Deref @ "semiLinear", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res, funcs) = differentiateExp(e.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            res1 = Expression::expAdd(Expression::expMul(res1, e.clone())?, Expression::expMul(e1.clone(), res.clone())?)?;
            res2 = Expression::expAdd(Expression::expMul(res2, e.clone())?, Expression::expMul(e2.clone(), res)?)?;
            (res, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            res = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: res, index: -1, optionExpisASUB: None });
            (metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: res, expThen: res1, expElse: res2 }), funcs)
        },
        (Deref @ "cross", Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            res2 = Expression::makePureBuiltinCall(literal!("cross"), list![e1.clone(), res2], tp.clone());
            res1 = Expression::makePureBuiltinCall(literal!("cross"), list![res1, e2.clone()], tp.clone());
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res2, operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: res1 }), funcs)
        },
        (Deref @ "max", Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("noEvent") }), expLst: list![metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::GREATER { ty: tp.clone() }, exp2: e2.clone(), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: res1, expElse: res2 }), funcs)
        },
        (Deref @ "min", Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("noEvent") }), expLst: list![metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: e2.clone(), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: res1, expElse: res2 }), funcs)
        },
        (Deref @ "div", Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            (res1, inFunctionTree)
        },
        (Deref @ "mod", Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut etmp: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            etmp = Expression::makePureBuiltinCall(literal!("floor"), list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e2.clone() })], tp.clone());
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp }) });
            (res1, funcs) = differentiateExp(e, inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (res1, funcs)
        },
        (Deref @ "rem", Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (res1, funcs)
        },
        (Deref @ "delay", Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Cons { head: e4, tail: Deref @ metamodelica::ListNode::Nil } } } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            res1 = (match inDiffType {
        BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. } => metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }),
        _ => metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
    });
            (res2, funcs) = differentiateExp(e3.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            res2 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res1, operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: res2 });
            (res2, _) = ExpressionSimplify::simplify(res2)?;
            if Expression::isZero(&res2)? {
                (res, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            } else {
                (e, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, &funcs, maxIter)?;
                e = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: name }), expLst: list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }), e, e3.clone(), e4.clone()], attr: inAttr });
                res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: res2, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e });
                (res, _) = ExpressionSimplify::simplify(res)?;
            }
            (res, funcs)
        },
        (Deref @ "sample", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            (res1, inFunctionTree)
        },
        (Deref @ "floor", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            (res1, inFunctionTree)
        },
        (Deref @ "ceil", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            (res1, inFunctionTree)
        },
        (Deref @ "integer", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            (res1, inFunctionTree)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateBinary(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV_ARR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut tp1: metamodelica::Ref<DAE::Type>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            tp1 = Expression::r#typeof(e2.clone())?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp1 }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: tp }, exp2: e2 } => {
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &funcs, maxIter)?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV_ARR { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::RCONST { real: r } } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut r = (*r).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            r = r.clone() - metamodelica::OrderedFloat(1.0_f64);
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::ICONST { integer: i } } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut i = (*i).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            i = i.clone() - 1;
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, operator: DAE::Operator::POW { ty: tp }, .. } if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            let mut zero: metamodelica::Ref<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
            (zero, inFunctionTree)
        },
        e0 @ Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r }, operator: DAE::Operator::POW { ty: tp }, exp2: e1 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut r = (*r).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            r = (r.clone()).ln();
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e0.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } } if (isParamOrConstant(cr.clone(), inInputData)?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut etmp: metamodelica::Ref<DAE::Exp>;
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            etmp = (match &*tp.clone() {
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }) }),
        _ => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }),
    });
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: etmp }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        e0 @ Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } if (isParamOrConstant(cr.clone(), inInputData)?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut etmp: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            etmp = Expression::makePureBuiltinCall(literal!("log"), list![e1.clone()], tp.clone());
            e = Expression::addNoEventToRelations(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e0.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }) }))?;
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut etmp: metamodelica::Ref<DAE::Exp>;
            let mut de1: metamodelica::Ref<DAE::Exp>;
            let mut de2: metamodelica::Ref<DAE::Exp>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, &inFunctionTree, maxIter)?;
            etmp = Expression::makePureBuiltinCall(literal!("log"), list![e1.clone()], tp.clone());
            e = Expression::addNoEventToRelations(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 }) }) }) }))?;
            (e, funcs)
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut serr: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            s1 = ExpressionBasics::printExpStr(inExp)?;
            s2 = ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?;
            serr = stringAppendList(list![literal!("\n- Function differentiateBinary failed. differentiateExp "), s1, literal!(" w.r.t: "), s2, literal!(" failed\n")]);
            Debug::trace(serr)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

// =============================================================================
// functions to generate derivative of a function
// =============================================================================
fn differentiateFunctionCall(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDiffwrtCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> =
        metamodelica::Ref::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedExp, outFunctionTree) = 'mc: {
        let __mc_input = (inExp.clone(), inDiffType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION { .. }) => {
                    if !((!(Expression::expHasCref(inExp.clone(), inDiffwrtCref.clone())?))) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    (e, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(&(ComponentReference::crefTypeFull(&inDiffwrtCref)?))))?;
                    Ok((e.clone(), inFunctionTree.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst: expl, attr: Deref @ DAE::CallAttributes { tuple_: b, builtin: c, isImpure, ty, tailCall: tc, .. } }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dpath: metamodelica::Ref<Absyn::Path>;
                    let mut dinl: DAE::InlineType;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut dtp: metamodelica::Ref<DAE::Type>;
                    let mut blst: metamodelica::List<bool>;
                    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    (mapper, tp) = getFunctionMapper(path.clone(), &inFunctionTree)?;
                    (dpath, blst) = differentiateFunction1(metamodelica::AsArg::as_arg(&path), &mapper, tp.clone(), metamodelica::AsArg::as_arg(&expl), &((inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&inFunctionTree, dpath.clone())?) {
                        Some(DAE::Function::FUNCTION { type_: __pa0, inlineType: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    dtp = metamodelica::Own::own(__pa0);
                    dinl = metamodelica::Own::own(__pa1);
                    ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), &tp, &dtp)?) {
                        (true, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (expl1, _) = List::splitOnBoolList(expl.clone(), blst.clone())?;
                    (dexpl, outFunctionTree) = List::map3Fold(&expl1, &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    expl1 = listAppend(expl.clone(), dexpl.clone());
                    Ok(((metamodelica::Ref::new(DAE::Exp::CALL { path: dpath.clone(), expLst: expl1.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: b.clone(), builtin: c.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: dinl, tailCall: tc.clone(), noReturn: DAE::NoReturn::RETURNS.clone() }) }), outFunctionTree.clone()), outFunctionTree.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outFunctionTree = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst: expl, .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut dpath: metamodelica::Ref<Absyn::Path>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut dtp: metamodelica::Ref<DAE::Type>;
                    let mut blst: metamodelica::List<bool>;
                    let mut tlst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut typstring: ArcStr;
                    let mut dastring: ArcStr;
                    let mut typlststring: metamodelica::List<ArcStr>;
                    (mapper, tp) = getFunctionMapper(path.clone(), &inFunctionTree)?;
                    (dpath, blst) = differentiateFunction1(metamodelica::AsArg::as_arg(&path), &mapper, tp.clone(), metamodelica::AsArg::as_arg(&expl), &((inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())))?;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&inFunctionTree, dpath.clone())?) {
                        Some(DAE::Function::FUNCTION { type_: __pa0, .. }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dtp = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), &tp, &dtp)?) {
                        (false, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    tlst = metamodelica::Own::own(__pa1);
                    typlststring = List::map(tlst.clone(), &TypesDump::unparseType)?;
                    typstring = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(typlststring.clone(), literal!(";\n"))); ArcStr::from(__mm_s) };
                    dastring = AbsynUtil::pathString(dpath.clone(), literal!("."), true, false)?;
                    metamodelica::print(literal!("Input warnings for function mapper2\n"));
                    Error::addMessage(Error::UNEXPECTED_FUNCTION_INPUTS_WARNING.clone(), list![dastring.clone(), typstring.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    if '__try0: {
                        let BackendDAE::DIFF_FULL_JACOBIAN { .. } = (inDiffType) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let __pa1 = ::match_deref::match_deref! { match &(Inline::forceInlineExp(inExp.clone(), (Some(inFunctionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone(), &Ceval::cevalSimpleWithFunctionTreeReturnExp)?) {
                        (__pa1, _, true) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa1);
                    (e, functions) = differentiateExp(e.clone(), &inDiffwrtCref, &inInputData, inDiffType, &inFunctionTree, maxIter)?;
                    Ok((e.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, expLst: expl, attr }, _) => {
                    if !((Expression::isRecordCall(metamodelica::AsArg::as_arg(&e), &inFunctionTree)?)) { return Err("guard") }
                    let mut dexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    (dexpl, functions) = List::map3Fold(metamodelica::AsArg::as_arg(&expl), &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: dexpl.clone(), attr: attr.clone() }), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let mut de: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut e = (*e).clone();
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        BackendDump::debugStrExpStr(&(literal!("### Differentiate call\n ")), e.clone(), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(&inDiffwrtCref)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }))?;
                    }
                    (de, functions) = differentiateFunctionCallPartial(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone(), maxIter)?;
                    (e, _, b) = Inline::forceInlineExp(de.clone(), (Some(functions.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone(), &Ceval::cevalSimpleWithFunctionTreeReturnExp)?;
                    if b {
                        de = e.clone();
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        BackendDump::debugStrExpStr(&(literal!("### result output :\n")), de.clone(), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(&inDiffwrtCref)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }))?;
                    }
                    Ok((de.clone(), functions.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut zero: metamodelica::Ref<DAE::Exp>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let false = (Expression::expContains(&inExp, &(Expression::crefExp(inDiffwrtCref.clone())?))?) else { return Err("pattern mismatch") };
                    tp = Expression::r#typeof(inExp.clone())?;
                    zero = Expression::createZeroExpression(tp.clone())?;
                    Ok((zero.clone(), inFunctionTree.clone()))
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
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate.differentiateFunctionCall")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateFunctionCallPartial(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDiffwrtCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outDiffedExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { path, expLst: expl, attr: Deref @ DAE::CallAttributes { tuple_: b, builtin: c, isImpure, ty, tailCall: tc, .. } } => {
                            let mut diffFuncData: BackendDAE::DifferentiateInputData;
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut dexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut dexplZero: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut dpath: metamodelica::Ref<Absyn::Path>;
                            let mut dinl: DAE::InlineType;
                            let mut dtp: metamodelica::Ref<DAE::Type>;
                            let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                            let mut mapper: DAE::FunctionDefinition;
                            let mut tp: metamodelica::Ref<DAE::Type>;
                            let mut dtp: metamodelica::Ref<DAE::Type>;
                            let mut blst: metamodelica::List<bool>;
                            let mut funcname: ArcStr;
                            (mapper, tp) = getFunctionMapper(path.clone(), &inFunctionTree)?;
                            (dpath, blst) = differentiateFunction1(metamodelica::AsArg::as_arg(&path), &mapper, tp.clone(), metamodelica::AsArg::as_arg(&expl), &((inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())))?;
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&inFunctionTree, dpath.clone())?) {
                                Some(DAE::Function::FUNCTION { type_: __pa0, inlineType: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            dtp = metamodelica::Own::own(__pa0);
                            dinl = metamodelica::Own::own(__pa1);
                            ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), &tp, &dtp)?) {
                                (true, _) => (),
                                _ => return Err("pattern mismatch"),
                            } };
                            (expl1, _) = List::splitOnBoolList(expl.clone(), blst.clone())?;
                            (dexpl, functions) = List::map3Fold(&expl1, &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())?;
                            funcname = BackendUtil::modelicaStringToCStr(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?, false)?;
                            diffFuncData = BackendDAE::emptyInputData().clone();
                            diffFuncData.matrixName = Some(funcname.clone());
                            (dexplZero, functions) = List::map3Fold(&expl1, &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("$"), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), diffFuncData.clone(), BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: false }, functions.clone())?;
                            dexplZero = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                let __thr_src0 = expl1.clone();
                let mut __thr_it0 = (&__thr_src0).into_iter();
                let __thr_src1 = dexplZero.clone();
                let mut __thr_it1 = (&__thr_src1).into_iter();
                loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(a), Some(z)) => {
                                    let __x = typedZeroSeed(a.clone(), z.clone())?;
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                }
                __acc.reverse()
            });
                            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                                metamodelica::print(literal!("### differentiated argument list:\n"));
                                metamodelica::print(literal!("Diffed ExpList: \n"));
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(dexpl.clone(), &ExpressionBasics::printExpStr)?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                            e = metamodelica::Ref::new(DAE::Exp::CALL { path: dpath.clone(), expLst: expl1.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: b.clone(), builtin: c.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: dinl, tailCall: tc.clone(), noReturn: DAE::NoReturn::RETURNS.clone() }) });
                            e = createPartialArguments(ty.clone(), dexpl.clone(), dexplZero.clone(), expl.clone(), e.clone(), inDiffType)?;
                            Ok((e.clone(), functions.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, expLst: expl, .. } => {
                    let mut dpath: metamodelica::Ref<Absyn::Path>;
                    let mut dtp: metamodelica::Ref<DAE::Type>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut dtp: metamodelica::Ref<DAE::Type>;
                    let mut blst: metamodelica::List<bool>;
                    let mut tlst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut typstring: ArcStr;
                    let mut dastring: ArcStr;
                    let mut typlststring: metamodelica::List<ArcStr>;
                    (mapper, tp) = getFunctionMapper(path.clone(), &inFunctionTree)?;
                    (dpath, blst) = differentiateFunction1(metamodelica::AsArg::as_arg(&path), &mapper, tp.clone(), metamodelica::AsArg::as_arg(&expl), &((inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree.clone())))?;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&inFunctionTree, dpath.clone())?) {
                        Some(DAE::Function::FUNCTION { type_: __pa0, .. }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dtp = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), &tp, &dtp)?) {
                        (false, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    tlst = metamodelica::Own::own(__pa1);
                    typlststring = List::map(tlst.clone(), &TypesDump::unparseType)?;
                    typstring = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(typlststring.clone(), literal!(";\n"))); ArcStr::from(__mm_s) };
                    dastring = AbsynUtil::pathString(dpath.clone(), literal!("."), true, false)?;
                    metamodelica::print(literal!("Input warnings for function mapper2\n"));
                    Error::addMessage(Error::UNEXPECTED_FUNCTION_INPUTS_WARNING.clone(), list![dastring.clone(), typstring.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { path, expLst: expl, attr: Deref @ DAE::CallAttributes { tuple_: b, builtin: false, isImpure, ty, tailCall: tc, .. } } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dexplZero: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dpath: metamodelica::Ref<Absyn::Path>;
                    let mut dtp: metamodelica::Ref<DAE::Type>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut dtp: metamodelica::Ref<DAE::Type>;
                    let mut blst: metamodelica::List<bool>;
                    let mut expBoolLst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, bool)>;
                    let mut funstring: ArcStr;
                    let mut inputVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut outputVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut func: DAE::Function;
                    let mut dfunc: DAE::Function;
                    let mut success: bool;
                    let mut e = (*e).clone();
                    let mut inInputData: BackendDAE::DifferentiateInputData = inInputData.clone();
                    if '__try0: {
                        let BackendDAE::SIMPLE_DIFFERENTIATION { .. } = (inDiffType) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    if '__try1: {
                        let BackendDAE::DIFF_FULL_JACOBIAN { .. } = (inDiffType) else { break '__try1 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let __pa2 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&inFunctionTree, path.clone())?) {
                        Some(__pa2) => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    func = metamodelica::Own::own(__pa2);
                    if !(AvlSetPath::hasKey(inInputData.diffedFunctions.clone(), path.clone())?) {
                        inInputData.diffedFunctions = AvlSetPath::add(inInputData.diffedFunctions.clone(), metamodelica::AsArg::as_arg(&path))?;
                        (dfunc, functions, blst) = differentiatePartialFunction(func.clone(), &inDiffwrtCref, &inInputData, inDiffType, inFunctionTree.clone(), maxIter)?;
                        dpath = DAEUtil::functionName(&dfunc);
                        let __pa3 = ::match_deref::match_deref! { match &(DAEUtil::getFunctionType(&dfunc)) {
                            Deref @ DAE::Type::T_FUNCTION { funcResultType: __pa3, .. } => __pa3.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        dtp = metamodelica::Own::own(__pa3);
                        if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                            funstring = Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: DAE::Function| DAEDumpTpl::dumpFunction(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, DAE::Function) -> Result<Tpl::Text> + 'static>), dfunc.clone())?;
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Differentiate function: \n")); __mm_s.push_str(&*funstring); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                        }
                        functions = AvlTreePathFunction::addDaeFunction(&(list![dfunc.clone()]), functions.clone())?;
                        func = DAEUtil::addFunctionDefinition(func.clone(), DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: path.clone(), derivativeFunction: dpath.clone(), derivativeOrder: 1, conditionRefs: metamodelica::nil(), defaultDerivative: None, lowerOrderDerivatives: metamodelica::nil() });
                        functions = AvlTreePathFunction::add(functions.clone(), metamodelica::AsArg::as_arg(&path), Some(func.clone()), &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?;
                    } else {
                        (functions, inputVarsDer, _, outputVarsDer, _, blst) = getFunctionInOutVars(&func, inFunctionTree.clone(), &inDiffwrtCref, maxIter)?;
                        (dpath, dtp) = getDiffedTypeandName(&func, inputVarsDer.clone(), &outputVarsDer, blst.clone())?;
                        let __pa4 = ::match_deref::match_deref! { match &(dtp.clone()) {
                            Deref @ DAE::Type::T_FUNCTION { funcResultType: __pa4, .. } => __pa4.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        dtp = metamodelica::Own::own(__pa4);
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print(literal!("### Detailed arguments list: \n"));
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(expl.clone(), &ExpressionBasics::printExpStr)?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print(literal!("### and argument types: \n"));
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::mapMap(expl.clone(), &Expression::r#typeof, &fnptr!(TypesDump::printTypeStr, metamodelica::Ref<DAE::Type>))?, literal!(" | "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### and output type: \n")); __mm_s.push_str(&*TypesDump::printTypeStr(dtp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    expBoolLst = List::zip(expl.clone(), blst.clone());
                    expBoolLst = List::filterOnTrue(expBoolLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)))?;
                    expl1 = List::map(expBoolLst.clone(), &fnptr!(Util::tuple21, _))?;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print(literal!("### Selected Arguments: \n"));
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(expl1.clone(), &ExpressionBasics::printExpStr)?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    (dexpl, functions) = List::map3Fold(&expl1, &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, functions.clone())?;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print(literal!("### Diffed ExpList: \n"));
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(dexpl.clone(), &ExpressionBasics::printExpStr)?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    (dexplZero, functions, success) = tryZeroDiff(expl1.clone(), functions.clone(), maxIter);
                    if success {
                        e = metamodelica::Ref::new(DAE::Exp::CALL { path: dpath.clone(), expLst: dexpl.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: dtp.clone(), tuple_: b.clone(), builtin: false, isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: tc.clone(), noReturn: DAE::NoReturn::RETURNS.clone() }) });
                        exp = createPartialArguments(ty.clone(), dexpl.clone(), dexplZero.clone(), expl.clone(), e.clone(), inDiffType)?;
                    } else {
                        exp = metamodelica::Ref::new(DAE::Exp::CALL { path: dpath.clone(), expLst: listAppend(expl.clone(), dexpl.clone()), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: dtp.clone(), tuple_: b.clone(), builtin: false, isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: tc.clone(), noReturn: DAE::NoReturn::RETURNS.clone() }) });
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print(literal!("### differentiated result CALL :\n"));
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    Ok(((exp.clone(), functions.clone()), inInputData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inInputData = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate.differentiateFunctionCallPartial failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    Debug::trace(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outDiffedExp, outFunctionTree))
}

fn addFunctionConstantsAndParameters(
    mut knownVars_opt: Option<BackendDAE::Variables>,
    mut func: &DAE::Function,
) -> Result<Option<BackendDAE::Variables>> {
    let mut knownVars_opt: Option<BackendDAE::Variables> = knownVars_opt;
    knownVars_opt = ({
        let mut body_knowns: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(func) {
            DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body }, tail: _ }, .. } => {
                let mut var_opt: Option<metamodelica::Ref<BackendDAE::Var>>;
                for mut element in &*body.clone() {
                    var_opt = BackendDAECreate::lowerKnownVarSingle(element.clone())?;
                    if (var_opt).is_some() {
                        body_knowns = metamodelica::cons(Util::getOption(var_opt)?, body_knowns);
                    }
                }
                if (body_knowns).is_empty() {
                    knownVars_opt = knownVars_opt;
                } else if (knownVars_opt).is_some() {
                    knownVars_opt = Some(BackendVariable::addVars(&body_knowns, Util::getOption(knownVars_opt)?)?);
                } else {
                    knownVars_opt = Some(BackendVariable::listVar(body_knowns)?);
                }
                knownVars_opt
            },
            _ => {
                knownVars_opt
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(knownVars_opt)
}

fn tryZeroDiff(
    mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    bool,
) {
    let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>> = explist;
    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree> = functions;
    let mut success: bool;
    let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>> = explist.clone();
    match '__try0: {
        (explist, functions) = unwrap_break_err!(List::map3Fold(&explist, &({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, &__pe_a1, &__pe_a2, __pe_a3, &__pe_a4, __pe_b5.clone()) }), metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("$"), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), BackendDAE::emptyInputData().clone(), BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: false }, functions.clone()), '__try0);
        explist = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            let __thr_src0 = args.clone();
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = explist.clone();
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(a), Some(z)) => {
                        let __x = unwrap_break_err!(typedZeroSeed(a.clone(), z.clone()), '__try0);
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => break '__try0 Err::<_, _>("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        });
        success = true;
        Ok::<_, &'static str>((explist.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            explist = __try0_o0;
            success = __try0_o1;
        }
        Err(_) => {
            explist = metamodelica::nil();
            success = false;
        }
    }
    (explist, functions, success)
}

fn createPartialArguments(
    mut outputType: metamodelica::Ref<DAE::Type>,
    mut inArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inDiffedArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inOrginalExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCall: metamodelica::Ref<DAE::Exp>,
    mut inDiffType: BackendDAE::DifferentiationType,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (&*outputType, &*inCall);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: rPath }, varLst, .. }, Deref @ DAE::Exp::CALL { path, attr, .. }) => {
                            let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut varNames: metamodelica::List<ArcStr>;
                            if boolAnd(!(List::all(&inArgs, &move |__a0: metamodelica::Ref<DAE::Exp>| isZeroDerivative(&__a0))?), inDiffType == openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION) {
                                e = metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() });
                            } else {
                                tys = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut v in (varLst.clone()).into_iter().cloned() {
                            let __x = DAEUtil::varType(&(v.clone()));
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                varNames = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut v in (varLst.clone()).into_iter().cloned() {
                            let __x = DAEUtil::typeVarIdent(&(v.clone()));
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                expLst = createPartialArgumentsRecord(tys.clone(), varNames.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone(), inDiffType)?;
                                e = metamodelica::Ref::new(DAE::Exp::RECORD { path: rPath.clone(), exps: expLst.clone(), comp: varNames.clone(), ty: outputType.clone() });
                            }
                            Ok(e.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CALL { path, attr, .. }, .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, _) => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = createPartialArgumentsTuple(tys.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone(), inDiffType)?;
                    Ok(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut ezero: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    dims = Expression::arrayDimension(&outputType);
                    (ezero, _) = Expression::makeZeroExpression(&dims)?;
                    e = createPartialDifferentiatedExp(&inArgs, inDiffedArgs.clone(), inOrginalExpl.clone(), &inCall, 1, ezero.clone())?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CALL { path, attr, .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
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

fn createPartialArgumentsTuple(
    mut inTypesLst: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inDiffedArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inOrginalExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCall: metamodelica::Ref<DAE::Exp>,
    mut inDiffType: BackendDAE::DifferentiationType,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        let __thr_src0 = inTypesLst.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let mut __thr_it1 = (1..=((inTypesLst).len() as i32)).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(tp), Some(number)) => {
                    let __x = createPartialArguments(
                        tp.clone(),
                        inArgs.clone(),
                        inDiffedArgs.clone(),
                        inOrginalExpl.clone(),
                        metamodelica::Ref::new(DAE::Exp::TSUB {
                            exp: inCall.clone(),
                            ix: number.clone(),
                            ty: tp.clone(),
                        }),
                        inDiffType,
                    )?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outExpLst)
}

fn createPartialArgumentsRecord(
    mut inTypesLst: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inVarNames: metamodelica::List<ArcStr>,
    mut inArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inDiffedArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inOrginalExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCall: metamodelica::Ref<DAE::Exp>,
    mut inDiffType: BackendDAE::DifferentiationType,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        let __thr_src0 = inTypesLst;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inVarNames;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(tp), Some(name)) => {
                    let __x = createPartialArguments(
                        tp.clone(),
                        inArgs.clone(),
                        inDiffedArgs.clone(),
                        inOrginalExpl.clone(),
                        metamodelica::Ref::new(DAE::Exp::RSUB {
                            exp: inCall.clone(),
                            ix: -1,
                            fieldName: name.clone(),
                            ty: tp.clone(),
                        }),
                        inDiffType,
                    )?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outExpLst)
}

fn createPartialDifferentiatedExp(
    mut inDiffExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inDiffExplZero: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inOrginalExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCall: &metamodelica::Ref<DAE::Exp>,
    mut currentLstElement: i32,
    mut inAccum: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inAccum;
    let mut i: i32 = currentLstElement;
    for mut de in &**inDiffExpl {
        outExp = (::match_deref::match_deref! { match &((de.clone(), &**inCall)) {
            (_, Deref @ DAE::Exp::CALL { path, attr, .. }) if (Types::isRecord(&(Expression::r#typeof(de.clone())?))) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut dexpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                if isZeroDerivative(metamodelica::AsArg::as_arg(&de))? {
                    e = outExp;
                } else {
                    dexpLst = List::set(inDiffExplZero.clone(), i, de.clone())?;
                    expLst = listAppend(inOrginalExpl.clone(), dexpLst);
                    e = Expression::expAdd(outExp, metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst, attr: attr.clone() }))?;
                }
                e
            },
            (Deref @ DAE::Exp::ARRAY { ty: tp, scalar: b, array: expl }, _) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut eArray: metamodelica::Ref<DAE::Exp>;
                let mut arrayArgs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut dexpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                eArray = (inDiffExplZero).get(i)?;
                dexpLst = Expression::arrayElements(eArray)?;
                arrayArgs = prepareArgumentsExplArray(metamodelica::AsArg::as_arg(&expl), &dexpLst, 1, metamodelica::nil())?;
                expLst = List::map2(arrayArgs, &fnptr!(Expression::makeArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::Type>, bool), tp.clone(), b.clone())?;
                arrayArgs = ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
            for mut exp in (expLst).into_iter().cloned() {
                let __x = List::set(inDiffExplZero.clone(), i, exp.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                arrayArgs = List::map1r(arrayArgs, &fnptr!(listAppend, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::List<metamodelica::Ref<DAE::Exp>>), inOrginalExpl.clone())?;
                e = createPartialSum(&arrayArgs, expl.clone(), inCall, outExp)?;
                e
            },
            _ => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut eone: metamodelica::Ref<DAE::Exp>;
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut dexpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut tp: metamodelica::Ref<DAE::Type>;
                let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                tp = Expression::r#typeof(de.clone())?;
                dims = Expression::arrayDimension(&tp);
                (eone, _) = Expression::makeOneExpression(&dims)?;
                dexpLst = List::set(inDiffExplZero.clone(), i, eone)?;
                expLst = listAppend(inOrginalExpl.clone(), dexpLst);
                e = createPartialSum(&(list![expLst]), list![de.clone()], inCall, outExp)?;
                e
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        i = i + 1;
    }
    Ok(outExp)
}

fn createPartialSum(
    mut inArgsLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inDiff: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCall: &metamodelica::Ref<DAE::Exp>,
    mut inAccum: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inAccum;
    let mut restDiff: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inDiff;
    let mut de: metamodelica::Ref<DAE::Exp>;
    let mut res: metamodelica::Ref<DAE::Exp>;
    for mut expLst in &**inArgsLst {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(restDiff) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        de = metamodelica::Own::own(__pa0);
        restDiff = metamodelica::Own::own(__pa1);
        if !(Expression::isZero(&de)?) {
            res = (::match_deref::match_deref! { match inCall {
                Deref @ DAE::Exp::RSUB { exp: Deref @ DAE::Exp::CALL { path, attr, .. }, ix, fieldName: name, ty } => {
                    metamodelica::Ref::new(DAE::Exp::RSUB { exp: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), fieldName: name.clone(), ty: ty.clone() })
                },
                Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CALL { path, attr, .. }, ix, ty } => {
                    metamodelica::Ref::new(DAE::Exp::TSUB { exp: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), ty: ty.clone() })
                },
                Deref @ DAE::Exp::CALL { path, attr, .. } => {
                    metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() })
                },
                _ => return Err("match: no arm matched"),
            } });
            res = Expression::expMul(de, res)?;
            outExp = Expression::expAdd(outExp, res)?;
        }
    }
    Ok(outExp)
}

fn prepareArgumentsExplArray<'__b>(
    mut inWorkLst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inArgs: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCurrentArg: i32,
    mut inAccum: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inWorkLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inAccum.reverse())
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
                let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut eone: metamodelica::Ref<DAE::Exp>;
                let mut tp: metamodelica::Ref<DAE::Type>;
                let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                tp = Expression::r#typeof(e.clone())?;
                dims = Expression::arrayDimension(&tp);
                (eone, _) = Expression::makeOneExpression(&dims)?;
                args = List::set(inArgs.clone(), inCurrentArg, eone)?;
                { (inWorkLst, inArgs, inCurrentArg, inAccum) = (rest, inArgs, inCurrentArg + 1, metamodelica::cons(args, inAccum)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn differentiatePartialFunction(
    mut inFunction: DAE::Function,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut maxIter: i32,
) -> Result<(
    DAE::Function,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    metamodelica::List<bool>,
)> {
    let mut outDerFunction: DAE::Function;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outBooleanlst: metamodelica::List<bool>;
    (outDerFunction, outFunctionTree, outBooleanlst) = 'mc: {
        let __mc_input = inFunction.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut func = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut inputData: BackendDAE::DifferentiateInputData;
            let mut diffFuncData: BackendDAE::DifferentiateInputData;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut dpath: metamodelica::Ref<Absyn::Path>;
            let mut isImpure: bool;
            let mut dinl: DAE::InlineType;
            let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut dtp: metamodelica::Ref<DAE::Type>;
            let mut funcbodyDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut inputVars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut inputVarsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut inputVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut outputVars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut outputVarsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut outputVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut protectedVars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut protectedVarsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut protectedVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut newProtectedVars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut bodyStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut derbodyStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut dfunc: DAE::Function;
            let mut funcname: ArcStr;
            let mut funstring: ArcStr;
            let mut blst: metamodelica::List<bool>;
            let mut visibility: SCode::Visibility;
            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                funstring = Tpl::tplString(
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: DAE::Function| {
                        DAEDumpTpl::dumpFunction(__a0, &__a1)
                    })
                        as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, DAE::Function) -> Result<Tpl::Text> + 'static>),
                    func.clone(),
                )?;
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("### Differentiate differentiateFunctionCallPartial: \n"));
                    __mm_s.push_str(&*funstring);
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
            inputVars = DAEUtil::getFunctionInputVars(&(func.clone()))?;
            outputVars = DAEUtil::getFunctionOutputVars(&(func.clone()))?;
            protectedVars = DAEUtil::getFunctionProtectedVars(&(func.clone()))?;
            bodyStmts = DAEUtil::getFunctionAlgorithmStmts(&(func.clone()))?;
            visibility = DAEUtil::getFunctionVisibility(&(func.clone()));
            (
                functions,
                inputVarsDer,
                inputVarsNoDer,
                outputVarsDer,
                outputVarsNoDer,
                blst,
            ) = getFunctionInOutVars(&(func.clone()), inFunctionTree.clone(), inDiffwrtCref, maxIter)?;
            path = DAEUtil::functionName(&(func.clone()));
            funcname = BackendUtil::modelicaStringToCStr(
                AbsynUtil::pathString(path.clone(), literal!("."), true, false)?,
                false,
            )?;
            diffFuncData = BackendDAE::emptyInputData().clone();
            diffFuncData.matrixName = Some(funcname.clone());
            diffFuncData.diffedFunctions = inInputData.diffedFunctions.clone();
            (inputData, _) = addElementVars2Dep(&inputVarsNoDer, &functions, diffFuncData.clone())?;
            (inputData, _) = addElementVars2Dep(&outputVarsNoDer, &functions, inputData.clone())?;
            (protectedVarsDer, functions, protectedVarsNoDer, _) = differentiateElementVars(
                &protectedVars,
                inDiffwrtCref,
                &inputData,
                openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION,
                &functions,
                &(metamodelica::nil()),
                &(metamodelica::nil()),
                &(metamodelica::nil()),
                maxIter,
                false,
            )?;
            (inputData, _) = addElementVars2Dep(&protectedVarsNoDer, &functions, inputData.clone())?;
            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                dumpInputData(&inputData)?;
            }
            inputData.knownVars = addFunctionConstantsAndParameters(inputData.knownVars.clone(), &(func.clone()))?;
            (derbodyStmts, functions) = differentiateStatements(
                &(bodyStmts.clone().reverse()),
                inDiffwrtCref,
                &inputData,
                openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION,
                &(metamodelica::nil()),
                &functions,
                maxIter,
            )?;
            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                funstring = DAEDump::ppStmtListStr(&derbodyStmts, 0)?;
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "### Differentiate differentiateFunctionCallPartial stmts: \n"
                    ));
                    __mm_s.push_str(&*funstring);
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (dpath, dtp) = getDiffedTypeandName(&(func.clone()), inputVarsDer.clone(), &outputVarsDer, blst.clone())?;
            newProtectedVars = List::map1(
                outputVars.clone(),
                &fnptr!(
                    DAEUtil::setElementVarVisibility,
                    metamodelica::Ref<DAE::Element>,
                    DAE::VarVisibility
                ),
                openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
            )?;
            newProtectedVars = List::map1(
                newProtectedVars.clone(),
                &fnptr!(
                    DAEUtil::setElementVarDirection,
                    metamodelica::Ref<DAE::Element>,
                    DAE::VarDirection
                ),
                openmodelica_frontend_types::DAE::VarDirection::BIDIR,
            )?;
            funcbodyDer = listAppend(
                newProtectedVars.clone(),
                list![metamodelica::Ref::new(DAE::Element::ALGORITHM {
                    algorithm_: metamodelica::Ref::new(DAE::Algorithm {
                        statementLst: derbodyStmts.clone()
                    }),
                    source: DAE::emptyElementSource().clone()
                })],
            );
            funcbodyDer = listAppend(protectedVarsDer.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(protectedVars.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(outputVarsDer.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(inputVarsDer.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(inputVars.clone(), funcbodyDer.clone());
            isImpure = DAEUtil::getFunctionImpureAttribute(&(func.clone()))?;
            dinl = DAEUtil::getFunctionInlineType(&(func.clone()))?;
            dfunc = DAE::Function::FUNCTION {
                path: dpath.clone(),
                functions: list![DAE::FunctionDefinition::FUNCTION_DEF {
                    body: funcbodyDer.clone()
                }],
                type_: dtp.clone(),
                visibility: visibility,
                partialPrefix: false,
                isImpure: isImpure,
                inlineType: dinl,
                unusedInputs: metamodelica::nil(),
                source: DAE::emptyElementSource().clone(),
                comment: None,
            };
            Ok((dfunc.clone(), functions.clone(), blst.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut r#str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            path = DAEUtil::functionName(&inFunction);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "\nDifferentiate.differentiatePartialFunction failed for function: "
                ));
                __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            Debug::trace(r#str.clone())?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outDerFunction, outFunctionTree, outBooleanlst))
}

fn getDiffedTypeandName(
    mut inFunction: &DAE::Function,
    mut inputVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut outputVarsDer: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut blst: metamodelica::List<bool>,
) -> Result<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<DAE::Type>)> {
    let mut diffedName: metamodelica::Ref<Absyn::Path>;
    let mut diffedType: metamodelica::Ref<DAE::Type>;
    diffedType = Types::extendsFunctionTypeArgs(
        &(DAEUtil::getFunctionType(inFunction)),
        inputVarsDer,
        outputVarsDer,
        blst,
    )?;
    diffedName = AbsynUtil::stringPath({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$DER"));
        __mm_s.push_str(&*BackendUtil::modelicaStringToCStr(
            AbsynUtil::pathString(DAEUtil::functionName(inFunction), literal!("."), true, false)?,
            false,
        )?);
        ArcStr::from(__mm_s)
    })?;
    Ok((diffedName, diffedType))
}

fn getFunctionInOutVars(
    mut inFunction: &DAE::Function,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut maxIter: i32,
) -> Result<(
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<bool>,
)> {
    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree> = inFunctionTree;
    let mut inputVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut inputVarsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outputVarsDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outputVarsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut blst: metamodelica::List<bool>;
    let mut inputVars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outputVars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut diffData: BackendDAE::DifferentiateInputData;
    inputVars = DAEUtil::getFunctionInputVars(inFunction)?;
    outputVars = DAEUtil::getFunctionOutputVars(inFunction)?;
    diffData = BackendDAE::emptyInputData().clone();
    diffData.matrixName = Some(BackendUtil::modelicaStringToCStr(
        AbsynUtil::pathString(DAEUtil::functionName(inFunction), literal!("."), true, false)?,
        false,
    )?);
    (inputVarsDer, functions, inputVarsNoDer, blst) = differentiateElementVars(
        &inputVars,
        inDiffwrtCref,
        &diffData,
        openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION,
        &functions,
        &(metamodelica::nil()),
        &(metamodelica::nil()),
        &(metamodelica::nil()),
        maxIter,
        true,
    )?;
    (outputVarsDer, functions, outputVarsNoDer, _) = differentiateElementVars(
        &outputVars,
        inDiffwrtCref,
        &diffData,
        openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION,
        &functions,
        &(metamodelica::nil()),
        &(metamodelica::nil()),
        &(metamodelica::nil()),
        maxIter,
        false,
    )?;
    Ok((
        functions,
        inputVarsDer,
        inputVarsNoDer,
        outputVarsDer,
        outputVarsNoDer,
        blst,
    ))
}

fn differentiateElementVars(
    mut inElements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inDiffwrtCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInputData: &BackendDAE::DifferentiateInputData,
    mut inDiffType: BackendDAE::DifferentiationType,
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inElementsDer: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inElementsNoDer: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inBooleanLst: &metamodelica::List<bool>,
    mut maxIter: i32,
    mut elementListInputs: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<bool>,
)> {
    let mut outElements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outElementsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outBooleanLst: metamodelica::List<bool>;
    (outElements, outFunctionTree, outElementsNoDer, outBooleanLst) = 'mc: {
        let __mc_input = (&**inElements, inInputData);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((metamodelica::Dangerous::listReverseInPlace(inElementsDer.clone()), inFunctionTree.clone(), metamodelica::Dangerous::listReverseInPlace(inElementsNoDer.clone()), metamodelica::Dangerous::listReverseInPlace(inBooleanLst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, binding: Some(binding), .. }, tail: rest }, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elementsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut var: metamodelica::Ref<DAE::Element>;
                    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut dbinding: metamodelica::Ref<DAE::Exp>;
                    let mut blst: metamodelica::List<bool>;
                    dcref = createDiffedCrefName(metamodelica::AsArg::as_arg(&cref), matrixName.clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), metamodelica::AsArg::as_arg(&var1))?;
                    (dbinding, _) = differentiateExp(binding.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
                    var = DAEUtil::replaceBindungInVar(dbinding.clone(), &var)?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, &vars, inElementsNoDer, &blst, maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, .. }, tail: rest }, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elementsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut var: metamodelica::Ref<DAE::Element>;
                    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut blst: metamodelica::List<bool>;
                    dcref = createDiffedCrefName(metamodelica::AsArg::as_arg(&cref), matrixName.clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), metamodelica::AsArg::as_arg(&var1))?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, &vars, inElementsNoDer, &blst, maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: var @ Deref @ DAE::Element::VAR { binding: Some(binding), .. }, tail: rest }, BackendDAE::DifferentiateInputData { independenentVars: Some(timevars), .. }) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elementsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut blst: metamodelica::List<bool>;
                    crefLst = Expression::extractCrefsFromExp(binding.clone())?;
                    ::match_deref::match_deref! { match &(BackendVariable::getVarLst(&crefLst, metamodelica::AsArg::as_arg(&timevars))) {
                        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::cons(var.clone(), inElementsNoDer.clone());
                    blst = metamodelica::cons(false, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, inElementsDer, &vars, &blst, maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: tp, binding: Some(binding), .. }, tail: rest }, _) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elementsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut var: metamodelica::Ref<DAE::Element>;
                    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut dbinding: metamodelica::Ref<DAE::Exp>;
                    let mut blst: metamodelica::List<bool>;
                    if elementListInputs {
                        let true = (Types::isRealOrSubTypeReal(tp.clone())) else { return Err("pattern mismatch") };
                    }
                    e = Expression::crefExp(cref.clone())?;
                    (e, functions) = differentiateCrefs(e.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
                    dcref = Expression::expCref(&e)?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), metamodelica::AsArg::as_arg(&var1))?;
                    (dbinding, functions) = differentiateExp(binding.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
                    var = DAEUtil::replaceBindungInVar(dbinding.clone(), &var)?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, &functions, &vars, inElementsNoDer, &blst, maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: tp, .. }, tail: rest }, _) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elementsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut var: metamodelica::Ref<DAE::Element>;
                    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut blst: metamodelica::List<bool>;
                    if elementListInputs {
                        let true = (Types::isRealOrSubTypeReal(tp.clone())) else { return Err("pattern mismatch") };
                    }
                    e = Expression::crefExp(cref.clone())?;
                    (e, functions) = differentiateCrefs(e.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
                    dcref = Expression::expCref(&e)?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), metamodelica::AsArg::as_arg(&var1))?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, &functions, &vars, inElementsNoDer, &blst, maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: var @ Deref @ DAE::Element::VAR { .. }, tail: rest }, _) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elementsNoDer: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut blst: metamodelica::List<bool>;
                    elementsNoDer = metamodelica::cons(var.clone(), inElementsNoDer.clone());
                    blst = metamodelica::cons(false, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(metamodelica::AsArg::as_arg(&rest), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, inElementsDer, &elementsNoDer, &blst, maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outElements, outFunctionTree, outElementsNoDer, outBooleanLst))
}

fn differentiateFunction1(
    mut inFuncName: &metamodelica::Ref<Absyn::Path>,
    mut inMapper: &DAE::FunctionDefinition,
    mut inTp: metamodelica::Ref<DAE::Type>,
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inDiffArgs: &(
        metamodelica::Ref<DAE::ComponentRef>,
        BackendDAE::DifferentiateInputData,
        BackendDAE::DifferentiationType,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
) -> Result<(metamodelica::Ref<Absyn::Path>, metamodelica::List<bool>)> {
    let mut outFuncName: metamodelica::Ref<Absyn::Path>;
    let mut blst: metamodelica::List<bool> = metamodelica::nil();
    (outFuncName, blst) = 'mc: {
        let __mc_input = (inMapper, inTp, inDiffArgs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivativeFunction: inDFuncName, derivativeOrder, conditionRefs: cr, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg, .. }, _) => {
                    if !((intEq(1, derivativeOrder.clone()))) { return Err("guard") }
                    let mut tplst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut bl: metamodelica::List<bool>;
                    let mut ba: metamodelica::Array<bool>;
                    tplst = List::map(funcArg.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgType(&__a0)) })?;
                    ba = Array::mapList(&tplst, &fnptr!(diffableTypes, metamodelica::Ref<DAE::Type>))?;
                    bl = checkDerFunctionConds(ba.clone(), metamodelica::AsArg::as_arg(&cr), expl, inDiffArgs)?;
                    Ok((inDFuncName.clone(), bl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivativeFunction: inDFuncName, derivativeOrder, conditionRefs: cr, .. }, tp, (_, _, _, functions)) => {
                    if !((!(intEq(1, derivativeOrder.clone())))) { return Err("guard") }
                    let mut fname: metamodelica::Ref<Absyn::Path>;
                    let mut bl: metamodelica::List<bool>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut ba: metamodelica::Array<bool>;
                    let mut tp = (*tp).clone();
                    let mut blst: metamodelica::List<bool> = blst.clone();
                    fname = getlowerOrderDerivative(inFuncName.clone(), metamodelica::AsArg::as_arg(&functions))?;
                    (mapper, tp) = getFunctionMapper(fname.clone(), metamodelica::AsArg::as_arg(&functions))?;
                    (_, blst) = differentiateFunction1(&fname, &mapper, tp.clone(), expl, inDiffArgs)?;
                    (bl, _) = List::split1OnTrue(&blst, &fnptr!(valueEq, _, _), true)?;
                    ba = metamodelica::arrayAppend(arrayCreate(((blst).len() as i32), false), metamodelica::arrayFromVec(bl.clone().into_iter().cloned().collect()));
                    bl = checkDerFunctionConds(ba.clone(), metamodelica::AsArg::as_arg(&cr), expl, inDiffArgs)?;
                    Ok(((inDFuncName.clone(), bl.clone()), blst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            blst = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: fname, derivativeOrder, defaultDerivative: Some(default), lowerOrderDerivatives, .. }, tp, _) => {
                    let mut da: metamodelica::Ref<Absyn::Path>;
                    let mut bl: metamodelica::List<bool>;
                    (da, bl) = differentiateFunction1(inFuncName, &(DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: fname.clone(), derivativeFunction: default.clone(), derivativeOrder: derivativeOrder.clone(), conditionRefs: metamodelica::nil(), defaultDerivative: Some(default.clone()), lowerOrderDerivatives: lowerOrderDerivatives.clone() }), tp.clone(), expl, inDiffArgs)?;
                    Ok((da.clone(), bl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outFuncName, blst))
}

fn checkDerivativeFunctionInputs(
    mut blst: metamodelica::List<bool>,
    mut tp: &metamodelica::Ref<DAE::Type>,
    mut dtp: &metamodelica::Ref<DAE::Type>,
) -> Result<(bool, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut outBoolean: bool;
    let mut outExpectedTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outBoolean, outExpectedTypeLst) = 'mc: {
        let __mc_input = (&**tp, &**dtp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: falst, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg: dfalst, .. }) => {
                    let mut falst1: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
                    let mut falst2: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
                    let mut tlst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut dtlst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut ret: bool;
                    (falst1, _) = List::splitOnBoolList(falst.clone(), blst.clone())?;
                    falst2 = listAppend(falst.clone(), falst1.clone());
                    tlst = List::map(falst2.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgType(&__a0)) })?;
                    dtlst = List::map(dfalst.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgType(&__a0)) })?;
                    ret = List::isEqualOnTrue(tlst.clone(), dtlst.clone(), &fnptr!(Types::equivtypes, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>))?;
                    Ok((ret, tlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("-Differentiate.checkDerivativeFunctionInputs failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outBoolean, outExpectedTypeLst))
}

fn checkDerFunctionConds(
    mut inbarr: metamodelica::Array<bool>,
    mut icrlst: &metamodelica::List<(i32, DAE::derivativeCond)>,
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inDiffArgs: &(
        metamodelica::Ref<DAE::ComponentRef>,
        BackendDAE::DifferentiateInputData,
        BackendDAE::DifferentiationType,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
) -> Result<metamodelica::List<bool>> {
    let mut outblst: metamodelica::List<bool>;
    let mut i: i32;
    let mut dc: DAE::derivativeCond;
    let mut e: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut p1: metamodelica::Ref<Absyn::Path>;
    let mut p2: metamodelica::Ref<Absyn::Path> = <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut ba: metamodelica::Array<bool> = inbarr;
    let mut diffwrtCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut inputData: BackendDAE::DifferentiateInputData;
    let mut diffType: BackendDAE::DifferentiationType;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (diffwrtCref, inputData, diffType, functionTree) = inDiffArgs.clone();
    for mut tpl in &**icrlst {
        (i, dc) = tpl.clone();
        let () = 'mc: {
            let __mc_input = dc;
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    DAE::derivativeCond::ZERO_DERIVATIVE { .. } => {
                        let mut e: metamodelica::Ref<DAE::Exp> = e.clone();
                        let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree> = functionTree.clone();
                        e = (expl).get(i)?;
                        (e, functionTree) = differentiateExp(e.clone(), &diffwrtCref, &inputData, diffType, &functionTree, defaultMaxIter.clone())?;
                        let true = (isZeroDerivative(&e)?) else { return Err("pattern mismatch") };
                        Ok(((), e.clone(), functionTree.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e = __wb0;
                functionTree = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    DAE::derivativeCond::NO_DERIVATIVE { binding: Deref @ DAE::Exp::CALL { path: p1, .. } } => {
                        let mut p2: metamodelica::Ref<Absyn::Path> = p2.clone();
                        let __pa0 = ::match_deref::match_deref! { match &((expl).get(i)?) {
                            Deref @ DAE::Exp::CALL { path: __pa0, .. } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        p2 = metamodelica::Own::own(__pa0);
                        let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), &p2)) else { return Err("pattern mismatch") };
                        Ok(((), p2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                p2 = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    DAE::derivativeCond::NO_DERIVATIVE { binding: Deref @ DAE::Exp::ICONST { .. } } => {
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
                        let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                        Debug::traceln(literal!("-Differentiate.checkDerFunctionConds failed"))?;
                        Ok(return Err("fail"))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        metamodelica::arrayUpdate(ba.clone(), i, false)?;
    }
    outblst = ba.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>();
    Ok(outblst)
}

fn zeroOfType(mut inType: &metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Exp> {
    let mut outZero: metamodelica::Ref<DAE::Exp>;
    outZero = (::match_deref::match_deref! { match inType {
        Deref @ DAE::Type::T_STRING { .. } => {
            metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
        },
        Deref @ DAE::Type::T_ENUMERATION { path, names: Deref @ metamodelica::ListNode::Cons { head: name, tail: _ }, .. } => {
            metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: AbsynUtil::suffixPath(path, metamodelica::AsArg::as_arg(&name)), index: 1 })
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst, .. } => {
            metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
            let __x = zeroOfType(&(DAEUtil::varType(&(v.clone()))));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: inType.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) })
        },
        _ => {
            Expression::makeConstZero(inType)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outZero
}

fn typedZeroSeed(
    mut inArg: metamodelica::Ref<DAE::Exp>,
    mut inZero: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outZero: metamodelica::Ref<DAE::Exp> = inZero.clone();
    let mut ty: metamodelica::Ref<DAE::Type>;
    if Expression::isZero(&inZero)? {
        ty = Expression::r#typeof(inArg)?;
        if Types::isRecord(&ty) || Types::isString(&ty) || Types::isBoolean(&ty) || Types::isEnumeration(&ty) {
            outZero = zeroOfType(&ty);
        }
    }
    Ok(outZero)
}

fn isZeroDerivative(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outZero: bool;
    outZero = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::RECORD { exps: expl, .. } => {
            List::all(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isZeroDerivative(&__a0))?
        },
        Deref @ DAE::Exp::CALL { path, expLst: expl, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: rpath }, .. }, .. } } if (AbsynUtil::pathEqual(path, metamodelica::AsArg::as_arg(&rpath))) => {
            List::all(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isZeroDerivative(&__a0))?
        },
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            List::all(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isZeroDerivative(&__a0))?
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            true
        },
        _ => {
            Expression::isZero(inExp)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outZero)
}

fn getlowerOrderDerivative(
    mut fname: metamodelica::Ref<Absyn::Path>,
    mut functions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outFName: metamodelica::Ref<Absyn::Path>;
    outFName = (match &**functions {
        _ => {
            let mut flst: metamodelica::List<DAE::FunctionDefinition>;
            let mut lowerOrderDerivatives: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut name: metamodelica::Ref<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(functions, fname)?) {
                Some(DAE::Function::FUNCTION { functions: __pa0, .. }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            flst = metamodelica::Own::own(__pa0);
            let DAE::FUNCTION_DER_MAPPER {
                lowerOrderDerivatives: __pa1,
                ..
            } = (getFunctionMapper1(&flst)?)
            else {
                return Err("pattern mismatch");
            };
            lowerOrderDerivatives = metamodelica::Own::own(__pa1);
            name = List::last(&lowerOrderDerivatives)?;
            name
        }
    });
    Ok(outFName)
}

pub(crate) fn getFunctionMapper(
    mut fname: metamodelica::Ref<Absyn::Path>,
    mut functions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(DAE::FunctionDefinition, metamodelica::Ref<DAE::Type>)> {
    let mut mapper: DAE::FunctionDefinition;
    let mut tp: metamodelica::Ref<DAE::Type>;
    (mapper, tp) = 'mc: {
        let __mc_input = &**functions;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut flst: metamodelica::List<DAE::FunctionDefinition>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut m: DAE::FunctionDefinition;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(functions, fname.clone())?) {
                        Some(DAE::Function::FUNCTION { functions: __pa0, type_: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    flst = metamodelica::Own::own(__pa0);
                    t = metamodelica::Own::own(__pa1);
                    m = getFunctionMapper1(&flst)?;
                    Ok((m.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    s = AbsynUtil::pathString(fname.clone(), literal!("."), true, false)?;
                    s = stringAppend(literal!("-Differentiate.getFunctionMapper failed for function "), s.clone());
                    Debug::traceln(s.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((mapper, tp))
}

fn getFunctionMapper1(mut inFuncDefs: &metamodelica::List<DAE::FunctionDefinition>) -> Result<DAE::FunctionDefinition> {
    let mut mapper: DAE::FunctionDefinition;
    mapper = 'mc: {
        let __mc_input = &**inFuncDefs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: m @ DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. }, tail: _ } => {
                    Ok(m.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: funcDefs } => {
                    let mut m: DAE::FunctionDefinition;
                    m = getFunctionMapper1(metamodelica::AsArg::as_arg(&funcDefs))?;
                    Ok(m.clone())
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
                    Debug::trace(literal!("-Differentiate.getFunctionMapper1 failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(mapper)
}

fn diffableTypes(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut out: bool = Types::isRealOrSubTypeReal(inType.clone()) || Types::isRecord(&inType);
    out
}

//
// util functions for Types: DifferentiateInputData, DifferentiateInputArguments, DifferentiationType
//
fn addDependentVars(
    mut inVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inDiffData: BackendDAE::DifferentiateInputData,
) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData;
    let mut depVars: BackendDAE::Variables;
    if (outDiffData.dependenentVars).is_some() {
        depVars = BackendVariable::addVars(&inVarsLst, Util::getOption(outDiffData.dependenentVars.clone())?)?;
    } else {
        depVars = BackendVariable::listVar(inVarsLst)?;
    }
    outDiffData.dependenentVars = Some(depVars);
    Ok(outDiffData)
}

fn addAllVars(
    mut inVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inDiffData: BackendDAE::DifferentiateInputData,
) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData;
    let mut allVars: BackendDAE::Variables;
    if (outDiffData.allVars).is_some() {
        allVars = BackendVariable::addVars(&inVarsLst, Util::getOption(outDiffData.allVars.clone())?)?;
    } else {
        allVars = BackendVariable::listVar(inVarsLst)?;
    }
    outDiffData.allVars = Some(allVars);
    Ok(outDiffData)
}

fn addGlobalVars(
    mut inVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inDiffData: BackendDAE::DifferentiateInputData,
) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData;
    let mut glVars: BackendDAE::Variables;
    if (outDiffData.knownVars).is_some() {
        glVars = BackendVariable::addVars(&inVarsLst, Util::getOption(outDiffData.knownVars.clone())?)?;
    } else {
        glVars = BackendVariable::listVar(inVarsLst)?;
    }
    outDiffData.knownVars = Some(glVars);
    Ok(outDiffData)
}

fn lowerVarsElementVars(
    mut inElementLstVars: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut functions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut varsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut reqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut exvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    match '__try0: {
        (vars, knvars, exvars, eqnsLst, reqnsLst) = unwrap_break_err!(BackendDAECreate::lowerVars(inElementLstVars, functions, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), '__try0);
        varsLst = listAppend(exvars.clone(), listAppend(vars.clone(), knvars.clone()));
        Ok::<_, &'static str>((
            eqnsLst.clone(),
            exvars.clone(),
            knvars.clone(),
            reqnsLst.clone(),
            vars.clone(),
            varsLst.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            eqnsLst = __try0_o0;
            exvars = __try0_o1;
            knvars = __try0_o2;
            reqnsLst = __try0_o3;
            vars = __try0_o4;
            varsLst = __try0_o5;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln(literal!("- Differentiate.lowerVarsElementVars failed."))?;
            return Err(__try0_err);
        }
    }
    Ok((varsLst, eqnsLst, reqnsLst))
}

fn addElementVars2Dep(
    mut inElementLstVars: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inFunctions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inDiffData: BackendDAE::DifferentiateInputData,
) -> Result<(
    BackendDAE::DifferentiateInputData,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outDiffData: BackendDAE::DifferentiateInputData;
    let mut outEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    match '__try0: {
        (varsLst, outEqnsLst, _) = unwrap_break_err!(lowerVarsElementVars(inElementLstVars, inFunctions), '__try0);
        outDiffData = unwrap_break_err!(addDependentVars(varsLst.clone(), inDiffData.clone()), '__try0);
        Ok::<_, &'static str>((outDiffData.clone(), outEqnsLst.clone(), varsLst.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outDiffData = __try0_o0;
            outEqnsLst = __try0_o1;
            varsLst = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("Differentiate.addElementVars2Dep failed")],
            )?;
            return Err(__try0_err);
        }
    }
    Ok((outDiffData, outEqnsLst))
}

fn dumpInputData(mut inDiffData: &BackendDAE::DifferentiateInputData) -> Result<()> {
    let mut independenentVars: Option<BackendDAE::Variables>;
    let mut dependenentVars: Option<BackendDAE::Variables>;
    let mut knownVars: Option<BackendDAE::Variables>;
    let mut allVars: Option<BackendDAE::Variables>;
    let mut controlVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut diffCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut matrixName: Option<ArcStr>;
    metamodelica::print(literal!("### dumpInputData ###\n"));
    if (inDiffData.matrixName).is_some() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("### for "));
            __mm_s.push_str(&*Util::getOption(inDiffData.matrixName.clone())?);
            __mm_s.push_str(&*literal!(" ###\n"));
            ArcStr::from(__mm_s)
        });
    }
    if (inDiffData.independenentVars).is_some() {
        metamodelica::print(literal!("independentVars:\n"));
        BackendDump::printVariables(&(Util::getOption(inDiffData.independenentVars.clone())?))?;
    }
    if (inDiffData.dependenentVars).is_some() {
        metamodelica::print(literal!("dependenentVars:\n"));
        BackendDump::printVariables(&(Util::getOption(inDiffData.dependenentVars.clone())?))?;
    }
    if (inDiffData.knownVars).is_some() {
        metamodelica::print(literal!("knownVars:\n"));
        BackendDump::printVariables(&(Util::getOption(inDiffData.knownVars.clone())?))?;
    }
    if (inDiffData.allVars).is_some() {
        metamodelica::print(literal!("allVars:\n"));
        BackendDump::printVariables(&(Util::getOption(inDiffData.allVars.clone())?))?;
    }
    if !((inDiffData.controlVars).is_empty()) {
        metamodelica::print(literal!("controlVars:\n"));
        BackendDump::printVarList(&inDiffData.controlVars)?;
    }
    if !((inDiffData.diffCrefs).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("diffCrefs:\n"));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(
                inDiffData.diffCrefs.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn isParamOrConstant(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut diffData: &BackendDAE::DifferentiateInputData,
) -> Result<bool> {
    let mut b: bool;
    b = (match diffData.clone() {
        BackendDAE::DifferentiateInputData {
            knownVars: Some(mut knownVars),
            ..
        } => {
            let mut var_lst: Option<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            var_lst = BackendVariable::getVarTryHard(cref, metamodelica::AsArg::as_arg(&knownVars));
            if (var_lst).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(Util::getOption(var_lst)?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                var = metamodelica::Own::own(__pa0);
                b = BackendVariable::isParamOrConstant(&var);
            } else {
                b = false;
            }
            b
        }
        _ => false,
    });
    Ok(b)
}
