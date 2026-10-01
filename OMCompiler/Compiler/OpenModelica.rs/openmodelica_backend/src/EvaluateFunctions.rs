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
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::RemoveSimpleEquations;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::HashSetExp;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// TODO:
// - evaluation of for-loops
// - evaluation of while-loops
// - evaluation of xOut := funcCall1(funcCall2(xIn[1]));  with funcCall2(xIn[1]) = xIn[1,2] for example have a look at Media.Examples.ReferenceAir.MoistAir
// - evaluation of BackendDAE.ARRAY_EQUATION
// =============================================================================
// =============================================================================
// type definitions
//
// =============================================================================
/// store informations when traversing the statements and evaluate the function calls
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FuncInfo {
    pub repl: BackendVarTransform::VariableReplacements,
    pub funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    pub idx: i32,
}

impl metamodelica::gc::MMTrace for FuncInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.repl, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.funcTree, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.idx, __mmv)?;
        Ok(())
    }
}
impl Default for FuncInfo {
    fn default() -> Self {
        Self {
            repl: Default::default(),
            funcTree: Default::default(),
            idx: Default::default(),
        }
    }
}

pub type FUNCINFO = FuncInfo;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Variability {
    CONST,
    VARIABLE,
}
impl metamodelica::gc::MMTrace for Variability {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Variability::CONST => Ok(()),
            Variability::VARIABLE => Ok(()),
        }
    }
}
impl Default for Variability {
    fn default() -> Self {
        Self::CONST
    }
}
pub(crate) use self::Variability::{CONST, VARIABLE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CallSignature {
    pub path: metamodelica::Ref<Absyn::Path>,
    pub inputsVari: metamodelica::List<Variability>,
    pub canBeEvaluated: bool,
}

impl metamodelica::gc::MMTrace for CallSignature {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.path, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inputsVari, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.canBeEvaluated, __mmv)?;
        Ok(())
    }
}
impl Default for CallSignature {
    fn default() -> Self {
        Self {
            path: Default::default(),
            inputsVari: Default::default(),
            canBeEvaluated: Default::default(),
        }
    }
}

pub type SIGNATURE = CallSignature;

// =============================================================================
// caching of already evaluated functions
//
// =============================================================================
fn checkCallSignatureForExp(
    mut expIn: metamodelica::Ref<DAE::Exp>,
    mut signLst: &metamodelica::List<CallSignature>,
) -> Result<bool> {
    let mut continueEval: bool;
    let mut signature: CallSignature;
    continueEval = true;
    signature = getCallSignatureForCall(expIn)?;
    if List::isMemberOnTrue(signature.clone(), signLst, &callSignatureIsEqual)? {
        let CallSignature {
            canBeEvaluated: __pa0, ..
        } = List::getMemberOnTrue(signature, signLst, &callSignatureIsEqual)?;
        continueEval = metamodelica::Own::own(__pa0);
    }
    Ok(continueEval)
}

fn callSignatureStr(mut signat: CallSignature) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut varis: metamodelica::List<Variability>;
    let mut b: bool;
    let CallSignature {
        path: __pa0,
        inputsVari: __pa1,
        canBeEvaluated: __pa2,
    } = signat;
    path = metamodelica::Own::own(__pa0);
    varis = metamodelica::Own::own(__pa1);
    b = metamodelica::Own::own(__pa2);
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*AbsynUtil::pathString(path, literal!("."), true, false)?);
        __mm_s.push_str(&*literal!("[ "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(varis, &fnptr!(VariabilityString, Variability))?,
            literal!(" | "),
        ));
        __mm_s.push_str(&*literal!(" ] "));
        __mm_s.push_str(&*boolString(b));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn VariabilityString(mut var: Variability) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match var {
        Variability::CONST { .. } => literal!("CONST"),
        _ => literal!("VARIABLE"),
    });
    r#str
}

fn callSignatureIsEqual(mut signat1: CallSignature, mut signat2: CallSignature) -> Result<bool> {
    let mut isEqual: bool;
    let mut path1: metamodelica::Ref<Absyn::Path>;
    let mut path2: metamodelica::Ref<Absyn::Path>;
    let mut vari1: metamodelica::List<Variability>;
    let mut vari2: metamodelica::List<Variability>;
    let CallSignature {
        path: __pa0,
        inputsVari: __pa1,
        ..
    } = signat1;
    path1 = metamodelica::Own::own(__pa0);
    vari1 = metamodelica::Own::own(__pa1);
    let CallSignature {
        path: __pa2,
        inputsVari: __pa3,
        ..
    } = signat2;
    path2 = metamodelica::Own::own(__pa2);
    vari2 = metamodelica::Own::own(__pa3);
    isEqual = false;
    if AbsynUtil::pathEqual(&path1, &path2) {
        if List::isEqualOnTrue(vari1, vari2, &fnptr!(VariabilityIsEqual, Variability, Variability))? {
            isEqual = true;
        }
    }
    Ok(isEqual)
}

fn VariabilityIsEqual(mut vari1: Variability, mut vari2: Variability) -> bool {
    let mut isEqual: bool;
    isEqual = (match (vari1, vari2) {
        (Variability::CONST { .. }, Variability::CONST { .. }) => true,
        (Variability::VARIABLE { .. }, Variability::VARIABLE { .. }) => true,
        _ => false,
    });
    isEqual
}

fn getCallSignatureForCall(mut callExpIn: metamodelica::Ref<DAE::Exp>) -> Result<CallSignature> {
    let mut signatureOut: CallSignature;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut vari: metamodelica::List<Variability>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(callExpIn.clone()) {
            Deref @ DAE::Exp::CALL { path: __pa1, expLst: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        path = metamodelica::Own::own(__pa1);
        expLst = metamodelica::Own::own(__pa2);
        vari = unwrap_break_err!(List::map(expLst.clone(), &getVariabilityForExp), '__try0);
        signatureOut = CallSignature {
            path: path.clone(),
            inputsVari: vari.clone(),
            canBeEvaluated: true,
        };
        Ok::<_, &'static str>((expLst.clone(), path.clone(), signatureOut.clone(), vari.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            expLst = __try0_o0;
            path = __try0_o1;
            signatureOut = __try0_o2;
            vari = __try0_o3;
        }
        Err(__try0_err) => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("evalFunc.getCallSignatureForCall failed for :\n"));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(callExpIn.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            return Err(__try0_err);
        }
    }
    Ok(signatureOut)
}

fn getVariabilityForExp(mut expIn: metamodelica::Ref<DAE::Exp>) -> Result<Variability> {
    let mut variOut: Variability;
    variOut = (match &*expIn {
        DAE::Exp::ICONST { .. } => crate::EvaluateFunctions::Variability::CONST,
        DAE::Exp::RCONST { .. } => crate::EvaluateFunctions::Variability::CONST,
        DAE::Exp::SCONST { .. } => crate::EvaluateFunctions::Variability::CONST,
        DAE::Exp::BCONST { .. } => crate::EvaluateFunctions::Variability::CONST,
        DAE::Exp::CLKCONST { .. } => crate::EvaluateFunctions::Variability::CONST,
        DAE::Exp::ENUM_LITERAL { .. } => crate::EvaluateFunctions::Variability::CONST,
        DAE::Exp::CREF { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::BINARY { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::UNARY { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::LBINARY { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::LUNARY { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::RELATION { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::IFEXP { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::CALL { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::RECORD { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::PARTEVALFUNCTION { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::ARRAY { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::MATRIX { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::RANGE { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::TUPLE { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::CAST { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::ASUB { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::TSUB { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::RSUB { .. } => {
            let mut vari: Variability;
            if Expression::isConst(expIn)? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari
        }
        DAE::Exp::SIZE { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::CODE { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::EMPTY { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        DAE::Exp::REDUCTION { .. } => crate::EvaluateFunctions::Variability::VARIABLE,
        _ => crate::EvaluateFunctions::Variability::VARIABLE,
    });
    Ok(variOut)
}

// =============================================================================
// evaluate functions
//
// =============================================================================
pub(crate) fn evalFunctions(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> metamodelica::Ref<BackendDAE::BackendDAE> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut changed: bool;
    let mut eqSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    match '__try0: {
        let __arc3 = inDAE.clone();
        let BackendDAE::DAE {
            eqs: __pa1,
            shared: __pa2,
        } = &*__arc3;
        eqSysts = metamodelica::Own::own(__pa1);
        shared = metamodelica::Own::own(__pa2);
        let (__pa4, (__pa5, _, __pa6, _)) = unwrap_break_err!(List::mapFold(&eqSysts, &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: (metamodelica::Ref<BackendDAE::Shared>, i32, bool, metamodelica::List<CallSignature>)| evalFunctions_main(__a0, &__a1), (shared.clone(), 1, false, metamodelica::nil())), '__try0);
        eqSysts = metamodelica::Own::own(__pa4);
        shared = metamodelica::Own::own(__pa5);
        changed = metamodelica::Own::own(__pa6);
        if changed {
            outDAE = unwrap_break_err!(updateVarKinds(&(unwrap_break_err!(RemoveSimpleEquations::fastAcausal(&(metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: eqSysts.clone(), shared: shared.clone() }))), '__try0))), '__try0);
        } else {
            outDAE = inDAE.clone();
        }
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

fn evalFunctions_main(
    mut eqSysIn: metamodelica::Ref<BackendDAE::EqSystem>,
    mut tplIn: &(
        metamodelica::Ref<BackendDAE::Shared>,
        i32,
        bool,
        metamodelica::List<CallSignature>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    (
        metamodelica::Ref<BackendDAE::Shared>,
        i32,
        bool,
        metamodelica::List<CallSignature>,
    ),
)> {
    let mut eqSysOut: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut tplOut: (
        metamodelica::Ref<BackendDAE::Shared>,
        i32,
        bool,
        metamodelica::List<CallSignature>,
    );
    let mut changed: bool;
    let mut sysIdx: i32;
    let mut sharedIn: metamodelica::Ref<BackendDAE::Shared>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut callSign: metamodelica::List<CallSignature>;
    let mut recursion_limit: i32;
    (sharedIn, sysIdx, changed, callSign) = tplIn.clone();
    let __arc1 = eqSysIn.clone();
    let BackendDAE::EQSYSTEM { orderedEqs: __pa0, .. } = &*__arc1;
    eqs = metamodelica::Own::own(__pa0);
    eqLst = BackendEquation::equationList(eqs)?;
    recursion_limit = Flags::getConfigInt(Flags::EVAL_RECURSION_LIMIT.clone())?;
    (eqLst, shared, addEqs, _, changed, callSign) = List::mapFold5(
        &eqLst,
        &({
            let __pe_b6 = recursion_limit;
            move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5| {
                Ok(evalFunctions_findFuncs(
                    __pe_a0,
                    __pe_a1,
                    __pe_a2,
                    __pe_a3,
                    __pe_a4,
                    __pe_a5,
                    __pe_b6.clone(),
                ))
            }
        }),
        sharedIn,
        metamodelica::nil(),
        1,
        changed,
        callSign,
    )?;
    eqs = BackendEquation::listEquation(&(listAppend(eqLst, addEqs)))?;
    eqSysOut = BackendDAEUtil::setEqSystEqs(eqSysIn, eqs);
    tplOut = (shared, sysIdx + 1, changed, callSign);
    Ok((eqSysOut, tplOut))
}

fn evalFunctions_findFuncs(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut idx: i32,
    mut changed: bool,
    mut callSign: metamodelica::List<CallSignature>,
    mut recursionLimit: i32,
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::Ref<BackendDAE::Shared>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    i32,
    bool,
    metamodelica::List<CallSignature>,
) {
    let mut eqIn: metamodelica::Ref<BackendDAE::Equation> = eqIn;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = addEqs;
    let mut idx: i32 = idx;
    let mut changed: bool = changed;
    let mut callSign: metamodelica::List<CallSignature> = callSign;
    eqIn = ({
        let mut lhsExp: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
        let mut rhsExp: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
        'mc: {
            let __mc_input = eqIn.clone();
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ BackendDAE::Equation::EQUATION { exp: exp1, scalar: exp2, source, attr } => {
                        let mut b1: bool;
                        let mut b2: bool;
                        let mut changed1: bool;
                        let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                        let mut addEqs1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut addEqs2: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = addEqs.clone();
                        let mut callSign: metamodelica::List<CallSignature> = callSign.clone();
                        let mut changed: bool = changed.clone();
                        let mut idx: i32 = idx.clone();
                        b1 = Expression::containFunctioncall(exp1.clone())?;
                        b2 = Expression::containFunctioncall(exp2.clone())?;
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        funcs = BackendDAEUtil::getFunctions(&shared);
                        if b1 {
                            (rhsExp, lhsExp, addEqs1, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp1.clone(), exp2.clone(), funcs.clone(), idx, callSign.clone(), recursionLimit)?;
                            changed = changed || changed1;
                            addEqs = listAppend(addEqs1.clone(), addEqs.clone());
                        }
                        if b2 {
                            (rhsExp, lhsExp, addEqs2, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp2.clone(), exp1.clone(), funcs.clone(), idx, callSign.clone(), recursionLimit)?;
                            changed = changed || changed1;
                            addEqs = listAppend(addEqs2.clone(), addEqs.clone());
                        }
                        eq = BackendEquation::generateEquation(lhsExp.clone(), rhsExp.clone(), source.clone(), attr.clone())?;
                        idx = idx + 1;
                        Ok((eq.clone(), addEqs.clone(), callSign.clone(), changed.clone(), idx.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                addEqs = __wb0;
                callSign = __wb1;
                changed = __wb2;
                idx = __wb3;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
                        if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                            metamodelica::print(literal!("this is an array equation. update evalFunctions_findFuncs\n"));
                        }
                        Ok(eqIn.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: exp1, right: exp2, source, attr, .. } => {
                        let mut b1: bool;
                        let mut b2: bool;
                        let mut changed1: bool;
                        let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                        let mut addEqs1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = addEqs.clone();
                        let mut callSign: metamodelica::List<CallSignature> = callSign.clone();
                        let mut changed: bool = changed.clone();
                        let mut idx: i32 = idx.clone();
                        let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared.clone();
                        b1 = Expression::containFunctioncall(exp1.clone())?;
                        b2 = Expression::containFunctioncall(exp2.clone())?;
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        funcs = BackendDAEUtil::getFunctions(&shared);
                        if b1 {
                            (rhsExp, lhsExp, addEqs1, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp1.clone(), exp2.clone(), funcs.clone(), idx, callSign.clone(), recursionLimit)?;
                            changed = changed || changed1;
                            addEqs = listAppend(addEqs1.clone(), addEqs.clone());
                        }
                        if b2 {
                            (rhsExp, lhsExp, addEqs1, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp2.clone(), exp1.clone(), funcs.clone(), idx, callSign.clone(), recursionLimit)?;
                            changed = changed || changed1;
                            addEqs = listAppend(addEqs1.clone(), addEqs.clone());
                        }
                        shared = BackendDAEUtil::setSharedFunctionTree(shared.clone(), funcs.clone());
                        eq = BackendEquation::generateEquation(lhsExp.clone(), rhsExp.clone(), source.clone(), attr.clone())?;
                        (eq, addEqs) = convertTupleEquations(eq.clone(), addEqs.clone())?;
                        idx = idx + 1;
                        Ok((eq.clone(), addEqs.clone(), callSign.clone(), changed.clone(), idx.clone(), shared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                addEqs = __wb0;
                callSign = __wb1;
                changed = __wb2;
                idx = __wb3;
                shared = __wb4;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(eqIn.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        }
    });
    (eqIn, shared, addEqs, idx, changed, callSign)
}

pub(crate) fn evaluateConstantFunctionCallExp(
    mut expIn: &metamodelica::Ref<DAE::Exp>,
    mut funcsIn: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut evalConstArgsOnly: bool,
    mut recursionLimit: i32,
) -> metamodelica::Ref<DAE::Exp> {
    let mut expOut: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    expOut = 'mc: {
        let __mc_input = &**expIn;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { path, expLst: exps0, .. } => {
                            let mut repl: BackendVarTransform::VariableReplacements;
                            let mut func: DAE::Function;
                            let mut allInputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut allOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut constInputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut constCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut constComplexCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut varScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut constScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut scalarInputs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
                            let mut scalarOutputs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
                            let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut protectVars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut algs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut allInputs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut allOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut allInputExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut constInputExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut constExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut constComplexExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut constScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut scalarExp: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                            let mut expOut: metamodelica::Ref<DAE::Exp> = expOut.clone();
                            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart constant evaluation of expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(expIn.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                            }
                            if evalConstArgsOnly {
                                let true = (Expression::isConstWorkList(exps0.clone())?) else { return Err("pattern mismatch") };
                            }
                            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcsIn, path.clone())?) {
                                Some(__pa0) => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            func = metamodelica::Own::own(__pa0);
                            let false = (DAEUtil::isExtFunction(&func)) else { return Err("pattern mismatch") };
                            elements = DAEUtil::getFunctionElements(&func)?;
                            exps = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut e in (exps0.clone()).into_iter().cloned() {
                            let __x = evaluateConstantFunctionCallExp(&(e.clone()), funcsIn, evalConstArgsOnly, recursionLimit);
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            scalarExp = List::map1(exps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<AvlTreePathFunction::Tree>| -> metamodelica::Result<_> { ::std::result::Result::Ok(expandComplexExpressions(__a0, &__a1)) }, funcsIn.clone())?;
                            allInputExps = List::flatten(scalarExp.clone())?;
                            if (elements).is_empty() && DAEUtil::funcIsRecord(&func) {
                                expOut = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: allInputExps.clone() });
                                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                    metamodelica::print(literal!("\nIts a record.\n"));
                                }
                            } else {
                                allInputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isInputVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                                scalarInputs = List::map(allInputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| expandComplexElementsToCrefs(&__a0))?;
                                allInputCrefs = List::flatten(scalarInputs.clone())?;
                                protectVars = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isProtectedVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                                algs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isAlgorithm(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                                algs = listAppend(protectVars.clone(), algs.clone());
                                allOutputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isOutputVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                                outputCrefs = List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::varCref(&__a0))?;
                                scalarOutputs = List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getScalarsForComplexVar(&__a0)) })?;
                                allOutputCrefs = listAppend(outputCrefs.clone(), List::flatten(scalarOutputs.clone())?);
                                (constInputExps, constInputCrefs) = List::filterOnTrueSync(&allInputExps, &Expression::isConst, allInputCrefs.clone())?;
                                repl = BackendVarTransform::emptyReplacements();
                                repl = BackendVarTransform::addReplacements(repl.clone(), &constInputCrefs, &constInputExps, None)?;
                                (algs, _, repl, _) = List::mapFold3(&algs, &({ let __pe_b4 = recursionLimit; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| evaluateFunctions_updateAlgElements(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), funcsIn.clone(), repl.clone(), 1)?;
                                (constCrefs, constExps) = BackendVarTransform::getAllReplacements(&repl)?;
                                (constCrefs, constExps) = List::filter1OnTrueSync(&constCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefInLst(__a0, &__a1), allOutputCrefs.clone(), constExps.clone())?;
                                (constExps, constCrefs) = List::filterOnTrueSync(&constExps, &Expression::isConst, constCrefs.clone())?;
                                (constComplexCrefs, _, constScalarCrefs, varScalarCrefs) = checkIfOutputIsEvaluatedConstant(&allOutputs, &constCrefs, &(metamodelica::nil()), &(metamodelica::nil()), &(metamodelica::nil()), &(metamodelica::nil()))?;
                                constScalarExps = List::map1r(constScalarCrefs.clone(), &move |__a0: BackendVarTransform::VariableReplacements, __a1: metamodelica::Ref<DAE::ComponentRef>| BackendVarTransform::getReplacement(&__a0, __a1), repl.clone())?;
                                constComplexExps = List::map1r(constComplexCrefs.clone(), &move |__a0: BackendVarTransform::VariableReplacements, __a1: metamodelica::Ref<DAE::ComponentRef>| BackendVarTransform::getReplacement(&__a0, __a1), repl.clone())?;
                                (constScalarCrefs, constScalarExps) = List::filter1OnTrueSync(&constCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefInLst(__a0, &__a1), constScalarCrefs.clone(), constExps.clone())?;
                                (constComplexCrefs, constComplexExps) = List::filter1OnTrueSync(&constCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefInLst(__a0, &__a1), constComplexCrefs.clone(), constExps.clone())?;
                                if (varScalarCrefs).is_empty() && (varScalarCrefs).is_empty() && (constComplexCrefs).is_empty() && !((constScalarExps).is_empty()) {
                                    if ((constScalarCrefs).len() as i32) == 1 {
                                                expOut = (constScalarExps).head().cloned()?;
                                    } else {
                                                expOut = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: constScalarExps.clone() });
                                    }
                                } else if (varScalarCrefs).is_empty() && (varScalarCrefs).is_empty() && (constScalarCrefs).is_empty() && !((constComplexExps).is_empty()) {
                                    if ((constComplexCrefs).len() as i32) == 1 {
                                                expOut = (constComplexExps).head().cloned()?;
                                    } else {
                                                expOut = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: constComplexExps.clone() });
                                    }
                                } else {
                                    expOut = expIn.clone();
                                }
                            }
                            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nevaluated to: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(expOut.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                            }
                            Ok((expOut.clone(), expOut.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            expOut = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { path, expLst: exps, attr: attr1 }, sub } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = evaluateConstantFunctionCallExp(&(metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: exps.clone(), attr: attr1.clone() })), funcsIn, evalConstArgsOnly, recursionLimit);
                    (exp, _) = ExpressionSimplify::simplify(metamodelica::Ref::new(DAE::Exp::ASUB { exp: exp.clone(), sub: sub.clone() }))?;
                    if !(Expression::isConst(exp.clone())?) {
                        exp = expIn.clone();
                    }
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(expIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    expOut
}

fn hasUnknownType(mut eIn: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut bOut: bool;
    bOut = (::match_deref::match_deref! { match eIn {
        Deref @ DAE::Exp::TUPLE { PR: eLst } => {
            List::any(eLst, &move |__a0: metamodelica::Ref<DAE::Exp>| hasUnknownType(&__a0))?
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_UNKNOWN { .. }, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bOut)
}

fn hasMultipleArrayDimensions(mut eIn: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut bOut: bool;
    bOut = (match &**eIn {
        DAE::Exp::TUPLE { PR: eLst } => List::any(eLst, &move |__a0: metamodelica::Ref<DAE::Exp>| {
            hasMultipleArrayDimensions(&__a0)
        })?,
        DAE::Exp::CREF { ty, .. } => {
            let mut b: bool;
            if Types::isArray(ty) {
                b = intNe(1, ((Types::getDimensionSizes(ty)?).len() as i32));
            } else {
                b = false;
            }
            b
        }
        _ => false,
    });
    Ok(bOut)
}

fn doNotInline(mut func: &DAE::Function) -> bool {
    let mut dontInline: bool;
    dontInline = (match func.clone() {
        DAE::Function::FUNCTION {
            inlineType: DAE::InlineType::NO_INLINE { .. },
            ..
        } => true,
        _ => false,
    });
    dontInline
}

pub(crate) fn evaluateConstantFunction(
    mut rhsExpIn: metamodelica::Ref<DAE::Exp>,
    mut lhsExpIn: metamodelica::Ref<DAE::Exp>,
    mut funcsIn: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut eqIdx: i32,
    mut callSignLstIn: metamodelica::List<CallSignature>,
    mut recursionLimit: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    i32,
    bool,
    metamodelica::List<CallSignature>,
)> {
    let mut rhsExpOut: metamodelica::Ref<DAE::Exp>;
    let mut lhsExpOut: metamodelica::Ref<DAE::Exp>;
    let mut addedEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut funcsOut: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut eqIdxOut: i32;
    let mut changed: bool = false;
    let mut callSignLstOut: metamodelica::List<CallSignature>;
    let mut funcIsConst: bool = false;
    let mut funcIsPartConst: bool = false;
    let mut isConstRec: bool = false;
    let mut hasAssert: bool = false;
    let mut hasReturn: bool = false;
    let mut hasTerminate: bool = false;
    let mut hasReinit: bool = false;
    let mut abort: bool = false;
    let mut isUnknownType: bool = false;
    let mut isNDimArray: bool = false;
    let mut idx: i32 = 0;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut repl: BackendVarTransform::VariableReplacements =
        <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut attr1: metamodelica::Ref<DAE::CallAttributes>;
    let mut attr2: metamodelica::Ref<DAE::CallAttributes> =
        <metamodelica::Ref<DAE::CallAttributes> as ::std::default::Default>::default();
    let mut exp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut outputExp: metamodelica::Ref<DAE::Exp> =
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut func: DAE::Function = <DAE::Function as ::std::default::Default>::default();
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree> =
        metamodelica::Ref::new(AvlTreePathFunction::Tree::EMPTY);
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut singleOutputType: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut constEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut allInputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut allOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut constInputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut constCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut varScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut constScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut constScalarCrefsLhs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut constComplexCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut varComplexCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut varScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut constScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut copyOutStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut algs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut allInputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut protectVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut allOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut updatedVarOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut newOutputVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut expsIn: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut allInputExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut constInputExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut constExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut constComplexExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut constScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut lhsExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut sub: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut scalarExp: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
    let mut outputVarTypes: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    let mut outputVarNames: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut scalarInputs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> =
        metamodelica::nil();
    let mut scalarOutputs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> =
        metamodelica::nil();
    let mut signature: CallSignature = <CallSignature as ::std::default::Default>::default();
    let mut callSignLst: metamodelica::List<CallSignature> = metamodelica::nil();
    let mut continueEval: bool = false;
    let true = (recursionLimit > 0) else {
        return Err("pattern mismatch");
    };
    (
        rhsExpOut,
        lhsExpOut,
        addedEquations,
        funcsOut,
        eqIdxOut,
        changed,
        callSignLstOut,
    ) = 'mc: {
        let __mc_input = (&*rhsExpIn, callSignLstIn.clone());
        if let Ok((
            __v,
            __wb0,
            __wb1,
            __wb2,
            __wb3,
            __wb4,
            __wb5,
            __wb6,
            __wb7,
            __wb8,
            __wb9,
            __wb10,
            __wb11,
            __wb12,
            __wb13,
            __wb14,
            __wb15,
            __wb16,
            __wb17,
            __wb18,
            __wb19,
            __wb20,
            __wb21,
            __wb22,
            __wb23,
            __wb24,
            __wb25,
            __wb26,
            __wb27,
            __wb28,
            __wb29,
            __wb30,
            __wb31,
            __wb32,
            __wb33,
            __wb34,
            __wb35,
            __wb36,
            __wb37,
            __wb38,
            __wb39,
            __wb40,
            __wb41,
            __wb42,
            __wb43,
            __wb44,
            __wb45,
            __wb46,
            __wb47,
            __wb48,
            __wb49,
            __wb50,
            __wb51,
            __wb52,
            __wb53,
        )) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Exp::CALL { path, expLst: expsIn, attr: attr1 }, callSignLst) => {
                            let mut path = (*path).clone();
                            let mut attr1 = (*attr1).clone();
                            let mut callSignLst = (*callSignLst).clone();
                            let mut abort: bool = abort.clone();
                            let mut algs: metamodelica::List<metamodelica::Ref<DAE::Element>> = algs.clone();
                            let mut allInputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = allInputCrefs.clone();
                            let mut allInputExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = allInputExps.clone();
                            let mut allInputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = allInputs.clone();
                            let mut allOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = allOutputCrefs.clone();
                            let mut allOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = allOutputs.clone();
                            let mut attr2: metamodelica::Ref<DAE::CallAttributes> = attr2.clone();
                            let mut changed: bool = changed.clone();
                            let mut constComplexCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = constComplexCrefs.clone();
                            let mut constComplexExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = constComplexExps.clone();
                            let mut constCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = constCrefs.clone();
                            let mut constEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = constEqs.clone();
                            let mut constExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = constExps.clone();
                            let mut constInputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = constInputCrefs.clone();
                            let mut constInputExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = constInputExps.clone();
                            let mut constScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = constScalarCrefs.clone();
                            let mut constScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = constScalarCrefsInFunc.clone();
                            let mut constScalarCrefsLhs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = constScalarCrefsLhs.clone();
                            let mut constScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = constScalarExps.clone();
                            let mut continueEval: bool = continueEval.clone();
                            let mut copyOutStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = copyOutStmts.clone();
                            let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements.clone();
                            let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = exps.clone();
                            let mut func: DAE::Function = func.clone();
                            let mut funcIsConst: bool = funcIsConst.clone();
                            let mut funcIsPartConst: bool = funcIsPartConst.clone();
                            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree> = funcs.clone();
                            let mut hasAssert: bool = hasAssert.clone();
                            let mut hasReinit: bool = hasReinit.clone();
                            let mut hasReturn: bool = hasReturn.clone();
                            let mut hasTerminate: bool = hasTerminate.clone();
                            let mut idx: i32 = idx.clone();
                            let mut isConstRec: bool = isConstRec.clone();
                            let mut isNDimArray: bool = isNDimArray.clone();
                            let mut isUnknownType: bool = isUnknownType.clone();
                            let mut lhsExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = lhsExps.clone();
                            let mut newOutputVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = newOutputVars.clone();
                            let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = outputCrefs.clone();
                            let mut outputExp: metamodelica::Ref<DAE::Exp> = outputExp.clone();
                            let mut outputVarNames: metamodelica::List<ArcStr> = outputVarNames.clone();
                            let mut outputVarTypes: metamodelica::List<metamodelica::Ref<DAE::Type>> = outputVarTypes.clone();
                            let mut protectVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = protectVars.clone();
                            let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                            let mut scalarExp: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = scalarExp.clone();
                            let mut scalarInputs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> = scalarInputs.clone();
                            let mut scalarOutputs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> = scalarOutputs.clone();
                            let mut signature: CallSignature = signature.clone();
                            let mut singleOutputType: metamodelica::Ref<DAE::Type> = singleOutputType.clone();
                            let mut updatedVarOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = updatedVarOutputs.clone();
                            let mut varComplexCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = varComplexCrefs.clone();
                            let mut varScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = varScalarCrefs.clone();
                            let mut varScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = varScalarCrefsInFunc.clone();
                            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart function evaluation of:\n")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhsExpIn.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhsExpIn.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                            }
                            continueEval = checkCallSignatureForExp(rhsExpIn.clone(), metamodelica::AsArg::as_arg(&callSignLst))?;
                            isUnknownType = hasUnknownType(&lhsExpIn)?;
                            isNDimArray = hasMultipleArrayDimensions(&lhsExpIn)?;
                            if !(continueEval) && Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                metamodelica::print(literal!("THIS FUNCTION CALL WITH THIS SPECIFIC SIGNATURE CANNOT BE EVALUTED\n"));
                            }
                            if !(continueEval) || isUnknownType || isNDimArray {
                                return Err("fail");
                            }
                            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&funcsIn, path.clone())?) {
                                Some(__pa0) => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            func = metamodelica::Own::own(__pa0);
                            let false = (doNotInline(&func)) else { return Err("pattern mismatch") };
                            elements = DAEUtil::getFunctionElements(&func)?;
                            protectVars = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isProtectedVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            algs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isAlgorithm(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && (elements).is_empty() {
                                metamodelica::print(literal!("Its a Record!\n"));
                                let false = (true) else { return Err("pattern mismatch") };
                            } else if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && ((protectVars).is_empty() && (algs).is_empty()) {
                                metamodelica::print(literal!("Its a Built-In!\n"));
                                let false = (true) else { return Err("pattern mismatch") };
                            }
                            let false = ((elements).is_empty()) else { return Err("pattern mismatch") };
                            let false = ((algs).is_empty()) else { return Err("pattern mismatch") };
                            exps = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut e in (expsIn.clone()).into_iter().cloned() {
                            let __x = evaluateConstantFunctionCallExp(&(e.clone()), &funcsIn, false, recursionLimit - 1);
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            scalarExp = List::map1(exps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<AvlTreePathFunction::Tree>| -> metamodelica::Result<_> { ::std::result::Result::Ok(expandComplexExpressions(__a0, &__a1)) }, funcsIn.clone())?;
                            allInputExps = List::flatten(scalarExp.clone())?;
                            allInputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isInputVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            scalarInputs = List::map(allInputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| expandComplexElementsToCrefs(&__a0))?;
                            allInputCrefs = List::flatten(scalarInputs.clone())?;
                            allOutputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isOutputVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            outputCrefs = List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::varCref(&__a0))?;
                            scalarOutputs = List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getScalarsForComplexVar(&__a0)) })?;
                            allOutputCrefs = listAppend(outputCrefs.clone(), List::flatten(scalarOutputs.clone())?);
                            (constInputExps, constInputCrefs) = List::filterOnTrueSync(&allInputExps, &Expression::isConst, allInputCrefs.clone())?;
                            repl = BackendVarTransform::emptyReplacements();
                            repl = BackendVarTransform::addReplacements(repl.clone(), &constInputCrefs, &constInputExps, None)?;
                            hasAssert = List::fold(&algs, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(hasAssertFold(&__a0, __a1)) }, false)?;
                            hasReturn = List::fold(&algs, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(hasReturnFold(&__a0, __a1)) }, false)?;
                            hasTerminate = List::fold(&algs, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(hasReturnFold(&__a0, __a1)) }, false)?;
                            hasReinit = List::fold(&algs, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(hasReinitFold(&__a0, __a1)) }, false)?;
                            abort = hasReturn || hasTerminate || hasReinit;
                            (algs, funcs, repl, idx) = List::mapFold3(&algs, &({ let __pe_b4 = recursionLimit - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| evaluateFunctions_updateAlgElements(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), funcsIn.clone(), repl.clone(), eqIdx)?;
                            (constCrefs, constExps) = BackendVarTransform::getAllReplacements(&repl)?;
                            (constCrefs, constExps) = List::filter1OnTrueSync(&constCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefInLst(__a0, &__a1), allOutputCrefs.clone(), constExps.clone())?;
                            (constExps, constCrefs) = List::filterOnTrueSync(&constExps, &Expression::isConst, constCrefs.clone())?;
                            (constComplexCrefs, varComplexCrefs, constScalarCrefs, varScalarCrefs) = checkIfOutputIsEvaluatedConstant(&allOutputs, &constCrefs, &(metamodelica::nil()), &(metamodelica::nil()), &(metamodelica::nil()), &(metamodelica::nil()))?;
                            (constScalarCrefs, constScalarExps) = List::filter1OnTrueSync(&constCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefInLst(__a0, &__a1), constScalarCrefs.clone(), constExps.clone())?;
                            (constComplexCrefs, constComplexExps) = List::filter1OnTrueSync(&constCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefInLst(__a0, &__a1), constComplexCrefs.clone(), constExps.clone())?;
                            funcIsConst = (varScalarCrefs).is_empty() && (varComplexCrefs).is_empty() && (!((constScalarCrefs).is_empty()) || !((constComplexCrefs).is_empty()));
                            funcIsPartConst = (!((varScalarCrefs).is_empty()) || !((varComplexCrefs).is_empty())) && (!((constScalarCrefs).is_empty()) || !((constComplexCrefs).is_empty())) && !(funcIsConst);
                            isConstRec = intEq(((constScalarCrefs).len() as i32), (((List::flatten(scalarOutputs.clone())?)).len() as i32)) && (varScalarCrefs).is_empty() && (varComplexCrefs).is_empty() && (constComplexCrefs).is_empty();
                            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                if funcIsConst {
                                    if hasAssert {
                                                metamodelica::print(literal!("the function output is completely constant but there is an assertion\n"));
                                    } else {
                                                metamodelica::print(literal!("the function output is completely constant\n"));
                                    }
                                } else if !(funcIsPartConst) {
                                    metamodelica::print(literal!("the function output is not constant in any case\n"));
                                }
                                if abort {
                                    metamodelica::print(literal!("the evaluated function is not used because there is a return or a terminate or a reinit statement\n"));
                                }
                            }
                            funcIsConst = if (hasAssert && funcIsConst || abort) {false} else {funcIsConst};
                            funcIsPartConst = if (hasAssert && funcIsConst) {true} else {funcIsPartConst};
                            funcIsPartConst = if (abort) {false} else {funcIsPartConst};
                            let true = (funcIsPartConst || funcIsConst) else { return Err("pattern mismatch") };
                            signature = getCallSignatureForCall(rhsExpIn.clone())?;
                            signature.canBeEvaluated = true;
                            callSignLst = metamodelica::cons(signature.clone(), callSignLst.clone());
                            changed = funcIsPartConst || funcIsConst;
                            (updatedVarOutputs, outputExp, varScalarCrefsInFunc, constScalarCrefsInFunc, copyOutStmts) = buildVariableFunctionParts(scalarOutputs.clone(), constComplexCrefs.clone(), varComplexCrefs.clone(), constScalarCrefs.clone(), varScalarCrefs.clone(), allOutputs.clone(), lhsExpIn.clone())?;
                            (constScalarCrefsLhs, constComplexCrefs) = buildConstFunctionCrefs(constScalarCrefs.clone(), constComplexCrefs.clone(), &allOutputCrefs, &lhsExpIn);
                            if !(funcIsConst) {
                                (algs, constEqs) = buildPartialFunction(&((varScalarCrefsInFunc.clone(), algs.clone())), &((constScalarCrefsInFunc.clone(), constScalarExps.clone(), constComplexCrefs.clone(), constComplexExps.clone(), constScalarCrefsLhs.clone())), copyOutStmts.clone(), &repl)?;
                            } else {
                                constEqs = metamodelica::nil();
                            }
                            elements = listAppend(protectVars.clone(), algs.clone());
                            elements = listAppend(updatedVarOutputs.clone(), elements.clone());
                            elements = listAppend(allInputs.clone(), elements.clone());
                            elements = List::unique(&elements);
                            newOutputVars = List::filterOnTrue(updatedVarOutputs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isOutputVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            (func, path) = updateFunctionBody(func.clone(), elements.clone(), idx, newOutputVars.clone(), &allOutputs)?;
                            funcs = if (funcIsPartConst) {AvlTreePathFunction::addDaeFunction(&(list![func.clone()]), funcs.clone())?} else {funcs.clone()};
                            idx = if (funcIsPartConst || funcIsConst) {idx + 1} else {idx};
                            outputExp = if (funcIsPartConst) {outputExp.clone()} else {lhsExpIn.clone()};
                            lhsExps = getCrefsForRecord(&lhsExpIn)?;
                            outputExp = if (isConstRec) {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: lhsExps.clone() })} else {outputExp.clone()};
                            outputVarTypes = List::map(newOutputVars.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::getVariableType(&__a0))?;
                            outputVarNames = List::map(newOutputVars.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::varName(&__a0))?;
                            attr2 = DAEUtil::replaceCallAttrType(attr1.clone(), metamodelica::Ref::new(DAE::Type::T_TUPLE { types: outputVarTypes.clone(), names: Some(outputVarNames.clone()) }));
                            let __arc2 = attr1.clone();
                            let DAE::CALL_ATTR { ty: __pa1, .. } = &*__arc2;
                            singleOutputType = metamodelica::Own::own(__pa1);
                            singleOutputType = if (!((newOutputVars).is_empty())) {(outputVarTypes).head().cloned()?} else {singleOutputType.clone()};
                            attr1 = DAEUtil::replaceCallAttrType(attr1.clone(), singleOutputType.clone());
                            attr2 = if (intEq(((newOutputVars).len() as i32), 1)) {attr1.clone()} else {attr2.clone()};
                            if List::hasOneElement(&(listAppend(constComplexExps.clone(), constScalarExps.clone()))) && funcIsConst {
                                exp = ((listAppend(constComplexExps.clone(), constScalarExps.clone()))).head().cloned()?;
                            } else if funcIsConst && !(List::hasOneElement(&(listAppend(constComplexExps.clone(), constScalarExps.clone())))) {
                                exp = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: listAppend(constComplexExps.clone(), constScalarExps.clone()) });
                            } else {
                                exp = rhsExpIn.clone();
                            }
                            exp = if (funcIsPartConst) {metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expsIn.clone(), attr: attr2.clone() })} else {exp.clone()};
                            exp = if (isConstRec) {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: constScalarExps.clone() })} else {exp.clone()};
                            outputExp = setRecordTypes(outputExp.clone());
                            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Finish evaluation of:\n")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhsExpIn.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhsExpIn.clone())?); __mm_s.push_str(&*literal!("\nto:\n")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outputExp.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                if !((constEqs).is_empty()) {
                                    BackendDump::dumpEquationList(&constEqs, &(literal!("including the additional equations:\n")))?;
                                }
                            }
                            Ok(((exp.clone(), outputExp.clone(), constEqs.clone(), funcs.clone(), idx, changed, callSignLst.clone()), abort.clone(), algs.clone(), allInputCrefs.clone(), allInputExps.clone(), allInputs.clone(), allOutputCrefs.clone(), allOutputs.clone(), attr2.clone(), changed.clone(), constComplexCrefs.clone(), constComplexExps.clone(), constCrefs.clone(), constEqs.clone(), constExps.clone(), constInputCrefs.clone(), constInputExps.clone(), constScalarCrefs.clone(), constScalarCrefsInFunc.clone(), constScalarCrefsLhs.clone(), constScalarExps.clone(), continueEval.clone(), copyOutStmts.clone(), elements.clone(), exp.clone(), exps.clone(), func.clone(), funcIsConst.clone(), funcIsPartConst.clone(), funcs.clone(), hasAssert.clone(), hasReinit.clone(), hasReturn.clone(), hasTerminate.clone(), idx.clone(), isConstRec.clone(), isNDimArray.clone(), isUnknownType.clone(), lhsExps.clone(), newOutputVars.clone(), outputCrefs.clone(), outputExp.clone(), outputVarNames.clone(), outputVarTypes.clone(), protectVars.clone(), repl.clone(), scalarExp.clone(), scalarInputs.clone(), scalarOutputs.clone(), signature.clone(), singleOutputType.clone(), updatedVarOutputs.clone(), varComplexCrefs.clone(), varScalarCrefs.clone(), varScalarCrefsInFunc.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            abort = __wb0;
            algs = __wb1;
            allInputCrefs = __wb2;
            allInputExps = __wb3;
            allInputs = __wb4;
            allOutputCrefs = __wb5;
            allOutputs = __wb6;
            attr2 = __wb7;
            changed = __wb8;
            constComplexCrefs = __wb9;
            constComplexExps = __wb10;
            constCrefs = __wb11;
            constEqs = __wb12;
            constExps = __wb13;
            constInputCrefs = __wb14;
            constInputExps = __wb15;
            constScalarCrefs = __wb16;
            constScalarCrefsInFunc = __wb17;
            constScalarCrefsLhs = __wb18;
            constScalarExps = __wb19;
            continueEval = __wb20;
            copyOutStmts = __wb21;
            elements = __wb22;
            exp = __wb23;
            exps = __wb24;
            func = __wb25;
            funcIsConst = __wb26;
            funcIsPartConst = __wb27;
            funcs = __wb28;
            hasAssert = __wb29;
            hasReinit = __wb30;
            hasReturn = __wb31;
            hasTerminate = __wb32;
            idx = __wb33;
            isConstRec = __wb34;
            isNDimArray = __wb35;
            isUnknownType = __wb36;
            lhsExps = __wb37;
            newOutputVars = __wb38;
            outputCrefs = __wb39;
            outputExp = __wb40;
            outputVarNames = __wb41;
            outputVarTypes = __wb42;
            protectVars = __wb43;
            repl = __wb44;
            scalarExp = __wb45;
            scalarInputs = __wb46;
            scalarOutputs = __wb47;
            signature = __wb48;
            singleOutputType = __wb49;
            updatedVarOutputs = __wb50;
            varComplexCrefs = __wb51;
            varScalarCrefs = __wb52;
            varScalarCrefsInFunc = __wb53;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { path, expLst: exps, attr: attr1 }, sub }, callSignLst) => {
                    let mut changed: bool = changed.clone();
                    let mut continueEval: bool = continueEval.clone();
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    exp = metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: exps.clone(), attr: attr1.clone() });
                    continueEval = checkCallSignatureForExp(exp.clone(), metamodelica::AsArg::as_arg(&callSignLst))?;
                    if !(continueEval) && Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print(literal!("THIS FUNCTION CALL WITH THIS SPECIFIC SIGNATURE CANNOT BE EVALUTED\n"));
                    }
                    if !(continueEval) {
                        return Err("fail");
                    }
                    exp = evaluateConstantFunctionCallExp(&exp, &funcsIn, false, recursionLimit);
                    (exp, _) = ExpressionSimplify::simplify(metamodelica::Ref::new(DAE::Exp::ASUB { exp: exp.clone(), sub: sub.clone() }))?;
                    changed = true;
                    if !(Expression::isConst(exp.clone())?) {
                        exp = rhsExpIn.clone();
                        changed = false;
                    }
                    Ok(((exp.clone(), lhsExpIn.clone(), metamodelica::nil(), funcsIn.clone(), eqIdx, changed, callSignLst.clone()), changed.clone(), continueEval.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            changed = __wb0;
            continueEval = __wb1;
            exp = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut callSignLst: metamodelica::List<CallSignature> = callSignLst.clone();
                    let mut signature: CallSignature = signature.clone();
                    callSignLst = callSignLstIn.clone();
                    if Expression::isCall(&rhsExpIn) {
                        signature = getCallSignatureForCall(rhsExpIn.clone())?;
                        signature.canBeEvaluated = false;
                        if !(List::isMemberOnTrue(signature.clone(), &callSignLstIn, &callSignatureIsEqual)?) {
                            callSignLst = metamodelica::cons(signature.clone(), callSignLst.clone());
                        }
                    }
                    Ok(((rhsExpIn.clone(), lhsExpIn.clone(), metamodelica::nil(), funcsIn.clone(), eqIdx, false, callSignLst.clone()), callSignLst.clone(), signature.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            callSignLst = __wb0;
            signature = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        rhsExpOut,
        lhsExpOut,
        addedEquations,
        funcsOut,
        eqIdxOut,
        changed,
        callSignLstOut,
    ))
}

fn expandComplexExpressions(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut funcs: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut eLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    eLst = 'mc: {
        let __mc_input = &*e;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, expLst: lst, .. } => {
                    let mut func: DAE::Function;
                    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut allOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut lst = (*lst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcs, path.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    func = metamodelica::Own::own(__pa0);
                    elements = DAEUtil::getFunctionElements(&func)?;
                    if (elements).is_empty() {
                    } else {
                        let __pa1 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcs, path.clone())?) {
                            Some(__pa1) => __pa1.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        func = metamodelica::Own::own(__pa1);
                        elements = DAEUtil::getFunctionElements(&func)?;
                        allOutputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isOutputVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                        lst = List::map(List::flatten(List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getScalarsForComplexVar(&__a0)) })?)?, &Expression::crefExp)?;
                    }
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    lst = Expression::getComplexContents(&e);
                    let false = ((lst).is_empty()) else { return Err("pattern mismatch") };
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![e.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    eLst
}

fn expandComplexElementsToCrefs(
    mut e: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut eLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    eLst = if (isNotComplexVar(e)) {
        list![DAEUtil::varCref(e)?]
    } else {
        getScalarsForComplexVar(e)
    };
    Ok(eLst)
}

fn hasAssertFold(mut stmt: &metamodelica::Ref<DAE::Element>, mut bIn: bool) -> bool {
    let mut bOut: bool;
    let mut bLst: metamodelica::List<bool>;
    let mut stmtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    match '__try0: {
        stmtLst = unwrap_break_err!(DAEUtil::getStatement(stmt), '__try0);
        bLst = unwrap_break_err!(List::map(stmtLst.clone(), &move |__a0: metamodelica::Ref<DAE::Statement>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isStmtAssert(&__a0)) }), '__try0);
        bOut = unwrap_break_err!(List::fold(&bLst, &fnptr!(boolOr, bool, bool), bIn), '__try0);
        Ok::<_, &'static str>((bOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bOut = __try0_o0;
        }
        Err(_) => {
            bOut = false;
        }
    }
    bOut
}

fn hasReturnFold(mut stmt: &metamodelica::Ref<DAE::Element>, mut bIn: bool) -> bool {
    let mut bOut: bool;
    let mut bLst: metamodelica::List<bool>;
    let mut stmtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    match '__try0: {
        stmtLst = unwrap_break_err!(DAEUtil::getStatement(stmt), '__try0);
        bLst = unwrap_break_err!(List::map(stmtLst.clone(), &move |__a0: metamodelica::Ref<DAE::Statement>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isStmtReturn(&__a0)) }), '__try0);
        bOut = unwrap_break_err!(List::fold(&bLst, &fnptr!(boolOr, bool, bool), bIn), '__try0);
        Ok::<_, &'static str>((bOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bOut = __try0_o0;
        }
        Err(_) => {
            bOut = false;
        }
    }
    bOut
}

fn hasReinitFold(mut stmt: &metamodelica::Ref<DAE::Element>, mut bIn: bool) -> bool {
    let mut bOut: bool;
    let mut bLst: metamodelica::List<bool>;
    let mut stmtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    match '__try0: {
        stmtLst = unwrap_break_err!(DAEUtil::getStatement(stmt), '__try0);
        bLst = unwrap_break_err!(List::map(stmtLst.clone(), &move |__a0: metamodelica::Ref<DAE::Statement>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isStmtReturn(&__a0)) }), '__try0);
        bOut = unwrap_break_err!(List::fold(&bLst, &fnptr!(boolOr, bool, bool), bIn), '__try0);
        Ok::<_, &'static str>((bOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bOut = __try0_o0;
        }
        Err(_) => {
            bOut = false;
        }
    }
    bOut
}

fn setRecordTypes(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst, attr: Deref @ DAE::CallAttributes { ty, .. }, .. } => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::isCall(&inExp)) else { return Err("pattern mismatch") };
                    let true = (((expLst).len() as i32) == 1) else { return Err("pattern mismatch") };
                    exp1 = (expLst).head().cloned()?;
                    cref = Expression::expCref(&exp1)?;
                    exp1 = Expression::makeCrefExp(cref.clone(), ty.clone())?;
                    Ok(exp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expLst } => {
                    let mut expLst = (*expLst).clone();
                    expLst = List::map(expLst.clone(), &fnptr!(setRecordTypes, metamodelica::Ref<DAE::Exp>))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst.clone() }))
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

pub(crate) fn getCrefsForRecord(
    mut e: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    es = (match &**e {
        DAE::Exp::CREF { componentRef: cref, .. } => {
            let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crefs = ComponentReference::expandCref(cref, true)?;
            expLst = List::map(crefs, &Expression::crefExp)?;
            expLst
        }
        _ => metamodelica::nil(),
    });
    Ok(es)
}

fn scalarRecExpForOneDimRec(mut expIn: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    expOut = 'mc: {
        let __mc_input = &*expIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, varLst, .. } } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut cref = (*cref).clone();
                    let true = (((varLst).len() as i32) == 1) else { return Err("pattern mismatch") };
                    crefs = getRecordScalars(metamodelica::AsArg::as_arg(&cref));
                    let true = (((crefs).len() as i32) == 1) else { return Err("pattern mismatch") };
                    cref = (crefs).head().cloned()?;
                    exp = Expression::crefExp(cref.clone())?;
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(expIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    expOut
}

fn scalarRecCrefsForOneDimRec(
    mut crefIn: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut crefOut: metamodelica::Ref<DAE::ComponentRef>;
    crefOut = 'mc: {
        let __mc_input = &*crefIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    crefs = getRecordScalars(&crefIn);
                    let true = (((crefs).len() as i32) == 1) else { return Err("pattern mismatch") };
                    cref = (crefs).head().cloned()?;
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(crefIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    crefOut
}

fn partiallyConstantArrayNeedsExpansion(
    mut allOutputCrefsIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut constScalarCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<bool> {
    let mut bOut: bool = false;
    for mut cref in &**allOutputCrefsIn {
        if Types::isArray(&(ComponentReference::crefType(metamodelica::AsArg::as_arg(&cref))?)) {
            if List::isMemberOnTrue(
                cref.clone(),
                constScalarCrefs,
                &fnptr!(
                    ComponentReferenceBasics::crefEqualWithoutSubs,
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>
                ),
            )? {
                bOut = true;
            }
        }
    }
    Ok(bOut)
}

fn buildVariableFunctionParts(
    mut scalarOutputs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    mut constComplexCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut varComplexCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut constScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut varScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut allOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut lhsExpIn: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
)> {
    let mut varOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut outputExpOut: metamodelica::Ref<DAE::Exp>;
    let mut varScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut constScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut copyOutStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut pos: metamodelica::List<i32> = metamodelica::nil();
    let mut lhsCref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    let mut outputExp: metamodelica::Ref<DAE::Exp> =
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut newOutput: metamodelica::Ref<DAE::Element> =
        <metamodelica::Ref<DAE::Element> as ::std::default::Default>::default();
    let mut protVar: metamodelica::Ref<DAE::Element> =
        <metamodelica::Ref<DAE::Element> as ::std::default::Default>::default();
    let mut copyStmt: metamodelica::Ref<DAE::Statement> =
        <metamodelica::Ref<DAE::Statement> as ::std::default::Default>::default();
    let mut varScalarCrefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut allOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut allOutputCrefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut protCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut scalarizedCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut funcOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut funcProts: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut funcSOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut funcSProts: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut varScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    (
        varOutputs,
        outputExpOut,
        varScalarCrefsInFunc,
        constScalarCrefsInFunc,
        copyOutStmts,
    ) = 'mc: {
        let __mc_input = (
            &*constComplexCrefs,
            &*varComplexCrefs,
            &*constScalarCrefs,
            &*varScalarCrefs,
            &*lhsExpIn,
        );
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut allOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = allOutputCrefs.clone();
                    let mut funcOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcOutputs.clone();
                    let mut funcProts: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcProts.clone();
                    let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = outputCrefs.clone();
                    let mut outputExp: metamodelica::Ref<DAE::Exp> = outputExp.clone();
                    let mut pos: metamodelica::List<i32> = pos.clone();
                    let mut protCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = protCrefs.clone();
                    let mut varOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = varOutputs.clone();
                    let mut varScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = varScalarCrefsInFunc.clone();
                    let mut varScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = varScalarExps.clone();
                    varScalarCrefsInFunc = metamodelica::nil();
                    allOutputCrefs = List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::varCref(&__a0))?;
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(constComplexCrefs.clone(), allOutputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                    pos = List::map1(outputCrefs.clone(), &move |__a0: _, __a1: _| List::position(__a0, &__a1), allOutputCrefs.clone())?;
                    varScalarExps = List::map1(pos.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), expLst.clone())?;
                    outputExp = if (List::hasOneElement(&varScalarExps)) {(varScalarExps).head().cloned()?} else {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    funcOutputs = List::map2(outputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateOutputElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                    funcProts = List::map2(protCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateProtectedElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                    varOutputs = listAppend(funcOutputs.clone(), funcProts.clone());
                    Ok(((varOutputs.clone(), outputExp.clone(), varScalarCrefsInFunc.clone(), constScalarCrefs.clone(), metamodelica::nil()), allOutputCrefs.clone(), funcOutputs.clone(), funcProts.clone(), outputCrefs.clone(), outputExp.clone(), pos.clone(), protCrefs.clone(), varOutputs.clone(), varScalarCrefsInFunc.clone(), varScalarExps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            allOutputCrefs = __wb0;
            funcOutputs = __wb1;
            funcProts = __wb2;
            outputCrefs = __wb3;
            outputExp = __wb4;
            pos = __wb5;
            protCrefs = __wb6;
            varOutputs = __wb7;
            varScalarCrefsInFunc = __wb8;
            varScalarExps = __wb9;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ DAE::Exp::LBINARY { .. }) => {
                    Ok((metamodelica::nil(), lhsExpIn.clone(), metamodelica::nil(), constScalarCrefs.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10)) =
            (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (_, _, _, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                        let mut allOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = allOutputCrefs.clone();
                        let mut allOutputCrefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = allOutputCrefs2.clone();
                        let mut funcOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcOutputs.clone();
                        let mut funcProts: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcProts.clone();
                        let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = outputCrefs.clone();
                        let mut outputExp: metamodelica::Ref<DAE::Exp> = outputExp.clone();
                        let mut pos: metamodelica::List<i32> = pos.clone();
                        let mut protCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = protCrefs.clone();
                        let mut varOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = varOutputs.clone();
                        let mut varScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = varScalarCrefsInFunc.clone();
                        let mut varScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = varScalarExps.clone();
                        allOutputCrefs = List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::varCref(&__a0))?;
                        allOutputCrefs2 = List::map(allOutputCrefs.clone(), &fnptr!(scalarRecCrefsForOneDimRec, metamodelica::Ref<DAE::ComponentRef>))?;
                        (_, _, varScalarCrefsInFunc) = List::intersection1OnTrue(allOutputCrefs.clone(), allOutputCrefs2.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                        allOutputCrefs = allOutputCrefs2.clone();
                        if partiallyConstantArrayNeedsExpansion(&allOutputCrefs, &constScalarCrefs)? {
                            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                                metamodelica::print(literal!("A partially constant array needs expansion. Thats not supported.\n"));
                            }
                            return Err("fail");
                        }
                        (protCrefs, _, outputCrefs) = List::intersection1OnTrue(listAppend(constComplexCrefs.clone(), constScalarCrefs.clone()), allOutputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                        funcOutputs = List::map2(outputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateOutputElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                        funcProts = List::map2(protCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateProtectedElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                        varOutputs = listAppend(funcOutputs.clone(), funcProts.clone());
                        pos = List::map1(outputCrefs.clone(), &move |__a0: _, __a1: _| List::position(__a0, &__a1), allOutputCrefs.clone())?;
                        varScalarExps = List::map1(pos.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), expLst.clone())?;
                        varScalarExps = List::map(varScalarExps.clone(), &fnptr!(scalarRecExpForOneDimRec, metamodelica::Ref<DAE::Exp>))?;
                        outputExp = if (List::hasOneElement(&varScalarExps)) {(varScalarExps).head().cloned()?} else {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                        Ok(((varOutputs.clone(), outputExp.clone(), varScalarCrefsInFunc.clone(), constScalarCrefs.clone(), metamodelica::nil()), allOutputCrefs.clone(), allOutputCrefs2.clone(), funcOutputs.clone(), funcProts.clone(), outputCrefs.clone(), outputExp.clone(), pos.clone(), protCrefs.clone(), varOutputs.clone(), varScalarCrefsInFunc.clone(), varScalarExps.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })()
        {
            allOutputCrefs = __wb0;
            allOutputCrefs2 = __wb1;
            funcOutputs = __wb2;
            funcProts = __wb3;
            outputCrefs = __wb4;
            outputExp = __wb5;
            pos = __wb6;
            protCrefs = __wb7;
            varOutputs = __wb8;
            varScalarCrefsInFunc = __wb9;
            varScalarExps = __wb10;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut allOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = allOutputCrefs.clone();
                    let mut funcOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcOutputs.clone();
                    let mut funcProts: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcProts.clone();
                    let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = outputCrefs.clone();
                    let mut outputExp: metamodelica::Ref<DAE::Exp> = outputExp.clone();
                    let mut pos: metamodelica::List<i32> = pos.clone();
                    let mut protCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = protCrefs.clone();
                    let mut varOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = varOutputs.clone();
                    let mut varScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = varScalarCrefsInFunc.clone();
                    let mut varScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = varScalarExps.clone();
                    let true = (((List::flatten(scalarOutputs.clone())?)).is_empty()) else { return Err("pattern mismatch") };
                    let true = (!((constScalarCrefs).is_empty())) else { return Err("pattern mismatch") };
                    varScalarCrefsInFunc = metamodelica::nil();
                    allOutputCrefs = List::map(allOutputs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::varCref(&__a0))?;
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(constScalarCrefs.clone(), allOutputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                    pos = List::map1(outputCrefs.clone(), &move |__a0: _, __a1: _| List::position(__a0, &__a1), allOutputCrefs.clone())?;
                    varScalarExps = List::map1(pos.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), expLst.clone())?;
                    outputExp = if (List::hasOneElement(&varScalarExps)) {(varScalarExps).head().cloned()?} else {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    funcOutputs = List::map2(outputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateOutputElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                    funcProts = List::map2(protCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateProtectedElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                    varOutputs = listAppend(funcOutputs.clone(), funcProts.clone());
                    Ok(((varOutputs.clone(), outputExp.clone(), varScalarCrefsInFunc.clone(), constScalarCrefs.clone(), metamodelica::nil()), allOutputCrefs.clone(), funcOutputs.clone(), funcProts.clone(), outputCrefs.clone(), outputExp.clone(), pos.clone(), protCrefs.clone(), varOutputs.clone(), varScalarCrefsInFunc.clone(), varScalarExps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            allOutputCrefs = __wb0;
            funcOutputs = __wb1;
            funcProts = __wb2;
            outputCrefs = __wb3;
            outputExp = __wb4;
            pos = __wb5;
            protCrefs = __wb6;
            varOutputs = __wb7;
            varScalarCrefsInFunc = __wb8;
            varScalarExps = __wb9;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expLst.clone();
                    let mut lhsCref: metamodelica::Ref<DAE::ComponentRef> = lhsCref.clone();
                    let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = outputCrefs.clone();
                    let mut outputExp: metamodelica::Ref<DAE::Exp> = outputExp.clone();
                    lhsCref = Expression::expCref(&lhsExpIn)?;
                    outputCrefs = List::map(constScalarCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::crefStripFirstIdent(&__a0))?;
                    outputCrefs = List::map1(outputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::joinCrefsR(__a0, &__a1), lhsCref.clone())?;
                    expLst = List::map(outputCrefs.clone(), &Expression::crefExp)?;
                    outputExp = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst.clone() });
                    Ok(((metamodelica::nil(), outputExp.clone(), metamodelica::nil(), constScalarCrefs.clone(), metamodelica::nil()), expLst.clone(), lhsCref.clone(), outputCrefs.clone(), outputExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expLst = __wb0;
            lhsCref = __wb1;
            outputCrefs = __wb2;
            outputExp = __wb3;
            break 'mc __v;
        }
        if let Ok((
            __v,
            __wb0,
            __wb1,
            __wb2,
            __wb3,
            __wb4,
            __wb5,
            __wb6,
            __wb7,
            __wb8,
            __wb9,
            __wb10,
            __wb11,
            __wb12,
            __wb13,
        )) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut copyOutStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = copyOutStmts.clone();
                    let mut copyStmt: metamodelica::Ref<DAE::Statement> = copyStmt.clone();
                    let mut funcOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcOutputs.clone();
                    let mut funcProts: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcProts.clone();
                    let mut funcSOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcSOutputs.clone();
                    let mut funcSProts: metamodelica::List<metamodelica::Ref<DAE::Element>> = funcSProts.clone();
                    let mut lhsCref: metamodelica::Ref<DAE::ComponentRef> = lhsCref.clone();
                    let mut newOutput: metamodelica::Ref<DAE::Element> = newOutput.clone();
                    let mut outputExp: metamodelica::Ref<DAE::Exp> = outputExp.clone();
                    let mut protVar: metamodelica::Ref<DAE::Element> = protVar.clone();
                    let mut scalarizedCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = scalarizedCrefs.clone();
                    let mut varOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>> = varOutputs.clone();
                    let mut varScalarCrefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = varScalarCrefs1.clone();
                    let mut varScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = varScalarExps.clone();
                    lhsCref = Expression::expCref(&lhsExpIn)?;
                    funcOutputs = List::map2(varComplexCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateOutputElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                    funcProts = List::map2(constComplexCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a2: metamodelica::Ref<DAE::Exp>| generateProtectedElements(__a0, &__a1, &__a2), allOutputs.clone(), lhsExpIn.clone())?;
                    scalarizedCrefs = listAppend(varScalarCrefs.clone(), constScalarCrefs.clone());
                    funcSProts = metamodelica::nil();
                    for mut outVar in &*allOutputs {
                        if List::exist1(&scalarizedCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefFirstIdentEqual(&__a0, &__a1), DAEUtil::varCref(metamodelica::AsArg::as_arg(&outVar))?)? {
                            protVar = DAEUtil::setElementVarVisibility(outVar.clone(), openmodelica_frontend_types::DAE::VarVisibility::PROTECTED);
                            funcSProts = metamodelica::cons(DAEUtil::setElementVarDirection(protVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR), funcSProts.clone());
                        }
                    }
                    funcSOutputs = metamodelica::nil();
                    copyOutStmts = metamodelica::nil();
                    for mut cref in &*varScalarCrefs {
                        (newOutput, copyStmt) = generateScalarOutputElement(cref.clone(), &allOutputs)?;
                        funcSOutputs = metamodelica::cons(newOutput.clone(), funcSOutputs.clone());
                        copyOutStmts = metamodelica::cons(copyStmt.clone(), copyOutStmts.clone());
                    }
                    funcSOutputs = funcSOutputs.clone().reverse();
                    copyOutStmts = copyOutStmts.clone().reverse();
                    varOutputs = List::flatten(list![funcOutputs.clone(), funcSOutputs.clone(), funcProts.clone(), funcSProts.clone()])?;
                    varScalarCrefs1 = List::map(varScalarCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::crefStripFirstIdent(&__a0))?;
                    varScalarCrefs1 = List::map1(varScalarCrefs1.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::joinCrefsR(__a0, &__a1), lhsCref.clone())?;
                    varScalarExps = List::map(varScalarCrefs1.clone(), &Expression::crefExp)?;
                    outputExp = if (List::hasOneElement(&varScalarExps)) {(varScalarExps).head().cloned()?} else {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    Ok(((varOutputs.clone(), outputExp.clone(), metamodelica::nil(), metamodelica::nil(), copyOutStmts.clone()), copyOutStmts.clone(), copyStmt.clone(), funcOutputs.clone(), funcProts.clone(), funcSOutputs.clone(), funcSProts.clone(), lhsCref.clone(), newOutput.clone(), outputExp.clone(), protVar.clone(), scalarizedCrefs.clone(), varOutputs.clone(), varScalarCrefs1.clone(), varScalarExps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            copyOutStmts = __wb0;
            copyStmt = __wb1;
            funcOutputs = __wb2;
            funcProts = __wb3;
            funcSOutputs = __wb4;
            funcSProts = __wb5;
            lhsCref = __wb6;
            newOutput = __wb7;
            outputExp = __wb8;
            protVar = __wb9;
            scalarizedCrefs = __wb10;
            varOutputs = __wb11;
            varScalarCrefs1 = __wb12;
            varScalarExps = __wb13;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print(literal!("buildVariableFunctionParts failed!\n"));
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n scalarOutputs \n")); __mm_s.push_str(&*stringDelimitList(List::map(List::flatten(scalarOutputs.clone())?, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!("\n"))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n constScalarCrefs \n")); __mm_s.push_str(&*stringDelimitList(List::map(constScalarCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!("\n"))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n allOutputs ")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&allOutputs)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n lhsExpIn ")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(lhsExpIn.clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
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
    Ok((
        varOutputs,
        outputExpOut,
        varScalarCrefsInFunc,
        constScalarCrefsInFunc,
        copyOutStmts,
    ))
}

fn buildConstFunctionCrefs(
    mut constScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut constComplCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut allOutputCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut lhsExpIn: &metamodelica::Ref<DAE::Exp>,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut constScalarCrefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut constComplCrefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (constScalarCrefsOut, constComplCrefsOut) = 'mc: {
        let __mc_input = (&*constScalarCrefs, &*constComplCrefs, &**lhsExpIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, _) => {
                    let mut lhsCref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut constCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    lhsCref = Expression::expCref(lhsExpIn)?;
                    constCrefs = List::map(constScalarCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::crefStripFirstIdent(&__a0))?;
                    constCrefs = List::map1(constCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::joinCrefsR(__a0, &__a1), lhsCref.clone())?;
                    Ok((constCrefs.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut pos: metamodelica::List<i32>;
                    let mut lhsCref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
                    let mut constExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut constCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    pos = metamodelica::nil();
                    for mut lhsCref in &*constComplCrefs {
                        let mut lhsCref = lhsCref.clone();
                        pos = metamodelica::cons(List::position1OnTrue(allOutputCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1), lhsCref.clone())?, pos.clone());
                    }
                    pos = pos.clone().reverse();
                    constExps = List::map1(pos.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), expLst.clone())?;
                    constCrefs = List::map(constExps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    Ok((metamodelica::nil(), constCrefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((constScalarCrefs.clone(), constComplCrefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (constScalarCrefsOut, constComplCrefsOut)
}

fn checkIfOutputIsEvaluatedConstant(
    mut elements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut constCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut constComplexLstIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut varComplexLstIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut constScalarLstIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut varScalarLstIn: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut constComplexLstOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut varComplexLstOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut constScalarLstOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut varScalarLstOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (constComplexLstOut, varComplexLstOut, constScalarLstOut, varScalarLstOut) = 'mc: {
        let __mc_input = &**elements;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((constComplexLstIn.clone(), varComplexLstIn.clone(), constScalarLstIn.clone(), varScalarLstIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut scalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constCrefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constCompl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varCompl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varScalar: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constScalar: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    cref = DAEUtil::varCref(metamodelica::AsArg::as_arg(&elem))?;
                    (constVars, varVars, constCrefs1) = List::intersection1OnTrue(list![cref.clone()], constCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                    if (constVars).is_empty() {
                        scalars = getScalarsForComplexVar(metamodelica::AsArg::as_arg(&elem));
                        if (scalars).is_empty() {
                            (constCompl, varCompl, constScalar, varScalar) = (constComplexLstIn.clone(), listAppend(varVars.clone(), varComplexLstIn.clone()), constScalarLstIn.clone(), varScalarLstIn.clone());
                        } else {
                            (constVars, varVars, constCrefs1) = List::intersection1OnTrue(scalars.clone(), constCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                            (constCompl, varCompl, constScalar, varScalar) = (constComplexLstIn.clone(), varComplexLstIn.clone(), listAppend(constVars.clone(), constScalarLstIn.clone()), listAppend(varVars.clone(), varScalarLstIn.clone()));
                        }
                    } else {
                        (constCompl, varCompl, constScalar, varScalar) = (listAppend(constVars.clone(), constComplexLstIn.clone()), varComplexLstIn.clone(), constScalarLstIn.clone(), varScalarLstIn.clone());
                    }
                    (constCompl, varCompl, constScalar, varScalar) = checkIfOutputIsEvaluatedConstant(metamodelica::AsArg::as_arg(&rest), &constCrefs1, &constCompl, &varCompl, &constScalar, &varScalar)?;
                    Ok((constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                    let mut r#const: bool;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
                    let mut scalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constCompl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varCompl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varScalar: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constScalar: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    scalars = getScalarsForComplexVar(metamodelica::AsArg::as_arg(&elem));
                    let false = ((scalars).is_empty()) else { return Err("pattern mismatch") };
                    constVars = List::intersectionOnTrue(&scalars, constCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                    r#const = intEq(((scalars).len() as i32), ((constVars).len() as i32));
                    constScalarCrefs = List::filter1OnTrue(constCrefs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefInLst(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<bool> + 'static>), constVars.clone())?;
                    (_, varCrefs, _) = List::intersection1OnTrue(scalars.clone(), constScalarCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                    constCompl = if (false) {metamodelica::cons(cref.clone(), constComplexLstIn.clone())} else {constComplexLstIn.clone()};
                    varCompl = varComplexLstIn.clone();
                    constScalar = if (true) {listAppend(constScalarCrefs.clone(), constScalarLstIn.clone())} else {constScalarLstIn.clone()};
                    varScalar = if (!(r#const)) {listAppend(varCrefs.clone(), varScalarLstIn.clone())} else {varScalarLstIn.clone()};
                    (constCompl, varCompl, constScalar, varScalar) = checkIfOutputIsEvaluatedConstant(metamodelica::AsArg::as_arg(&rest), constCrefs, &constCompl, &varCompl, &constScalar, &varScalar)?;
                    Ok((constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                    let mut r#const: bool;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut scalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constCompl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varCompl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varScalar: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constScalar: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    cref = DAEUtil::varCref(metamodelica::AsArg::as_arg(&elem))?;
                    scalars = getScalarsForComplexVar(metamodelica::AsArg::as_arg(&elem));
                    let true = ((scalars).is_empty()) else { return Err("pattern mismatch") };
                    r#const = listMember(cref.clone(), constCrefs.clone());
                    constCompl = if (r#const) {metamodelica::cons(cref.clone(), constComplexLstIn.clone())} else {constComplexLstIn.clone()};
                    varCompl = if (!(r#const)) {metamodelica::cons(cref.clone(), varComplexLstIn.clone())} else {varComplexLstIn.clone()};
                    (constCompl, varCompl, constScalar, varScalar) = checkIfOutputIsEvaluatedConstant(metamodelica::AsArg::as_arg(&rest), constCrefs, &constCompl, &varCompl, constScalarLstIn, varScalarLstIn)?;
                    Ok((constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("checkIfOutputIsEvaluatedConstant failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((constComplexLstOut, varComplexLstOut, constScalarLstOut, varScalarLstOut))
}

fn generateScalarOutputElement(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut inFuncOutputs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::Statement>)> {
    let mut newOutput: metamodelica::Ref<DAE::Element>;
    let mut copyStmt: metamodelica::Ref<DAE::Statement>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut newCref: metamodelica::Ref<DAE::ComponentRef>;
    ty = ComponentReference::crefTypeFull(&cref)?;
    newCref = ComponentReferenceBasics::makeCrefIdent(flattenCrefIdent(&cref)?, ty.clone(), metamodelica::nil());
    newOutput = (::match_deref::match_deref! { match &((inFuncOutputs).head().cloned()?) {
        var @ Deref @ DAE::Element::VAR { .. } => {
            let mut var = (*var).clone();
            assign_variant_field!(var => DAE::Element::VAR;
                componentRef = newCref.clone(),
                ty = ty.clone(),
                dims = metamodelica::nil(),
                binding = None,
                direction = openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
                protection = openmodelica_frontend_types::DAE::VarVisibility::PUBLIC
            );
            var.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    copyStmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
        type_: ty,
        exp1: Expression::crefToExp(newCref)?,
        exp: Expression::crefToExp(cref)?,
        source: DAE::emptyElementSource().clone(),
    });
    Ok((newOutput, copyStmt))
}

fn flattenCrefIdent(mut cref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut ident: ArcStr;
    ident = (match &**cref {
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            subscriptLst: subs,
            componentRef: rest,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*id);
            __mm_s.push_str(&*flattenSubscripts(subs)?);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*flattenCrefIdent(rest)?);
            ArcStr::from(__mm_s)
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            subscriptLst: subs,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*id);
            __mm_s.push_str(&*flattenSubscripts(subs)?);
            ArcStr::from(__mm_s)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(ident)
}

fn flattenSubscripts(mut subs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    for mut sub in &**subs {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*ExpressionDump::subscriptString(metamodelica::AsArg::as_arg(&sub))?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

fn generateOutputElements(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut inFuncOutputs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut recId: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut newOutputs: metamodelica::Ref<DAE::Element>;
    newOutputs = (match &*cref.clone() {
        DAE::ComponentRef::CREF_QUAL { subscriptLst: sl, .. } => {
            let mut i1: ArcStr;
            let mut i2: ArcStr;
            let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
            let mut var: metamodelica::Ref<DAE::Element>;
            let mut typ: metamodelica::Ref<DAE::Type>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            typ = ComponentReference::crefLastType(&cref)?;
            cref1 = ComponentReference::crefStripLastIdent(&cref)?;
            crefs = getRecordScalars(&cref);
            cref1 = if (intEq(((crefs).len() as i32), 1)) {
                (crefs).head().cloned()?
            } else {
                cref1
            };
            i1 = ComponentReferenceBasics::crefFirstIdent(&cref)?;
            i2 = ComponentReferenceBasics::crefLastIdent(&cref)?;
            i1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*i1);
                __mm_s.push_str(&*literal!("_"));
                __mm_s.push_str(&*i2);
                ArcStr::from(__mm_s)
            };
            cref1 = ComponentReferenceBasics::makeCrefIdent(i1, typ.clone(), sl.clone());
            var = (inFuncOutputs).head().cloned()?;
            var = DAEUtil::replaceCrefandTypeInVar(cref1, typ, &var)?;
            var
        }
        DAE::ComponentRef::CREF_IDENT { identType: typ, .. } => {
            let mut var: metamodelica::Ref<DAE::Element>;
            var = (inFuncOutputs).head().cloned()?;
            var = DAEUtil::replaceCrefandTypeInVar(cref, typ.clone(), &var)?;
            var
        }
        _ => {
            metamodelica::print(literal!("generateOutputElements failed!\n"));
            return Err("fail");
        }
    });
    Ok(newOutputs)
}

fn generateProtectedElements(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut inFuncOutputs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut recId: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut newProts: metamodelica::Ref<DAE::Element>;
    newProts = (match &*cref.clone() {
        DAE::ComponentRef::CREF_QUAL { subscriptLst: sl, .. } => {
            let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
            let mut i1: ArcStr;
            let mut i2: ArcStr;
            let mut var: metamodelica::Ref<DAE::Element>;
            let mut typ: metamodelica::Ref<DAE::Type>;
            typ = ComponentReference::crefLastType(&cref)?;
            i1 = ComponentReferenceBasics::crefFirstIdent(&cref)?;
            i2 = ComponentReferenceBasics::crefLastIdent(&cref)?;
            i1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*i1);
                __mm_s.push_str(&*literal!("_"));
                __mm_s.push_str(&*i2);
                ArcStr::from(__mm_s)
            };
            cref1 = ComponentReferenceBasics::makeCrefIdent(i1, typ.clone(), sl.clone());
            var = (inFuncOutputs).head().cloned()?;
            var = DAEUtil::replaceCrefandTypeInVar(cref1, typ, &var)?;
            var = DAEUtil::setElementVarVisibility(var, openmodelica_frontend_types::DAE::VarVisibility::PROTECTED);
            var = DAEUtil::setElementVarDirection(var, openmodelica_frontend_types::DAE::VarDirection::BIDIR);
            var
        }
        DAE::ComponentRef::CREF_IDENT { identType: typ, .. } => {
            let mut var: metamodelica::Ref<DAE::Element>;
            var = (inFuncOutputs).head().cloned()?;
            var = DAEUtil::replaceCrefandTypeInVar(cref, typ.clone(), &var)?;
            var = DAEUtil::setElementVarVisibility(var, openmodelica_frontend_types::DAE::VarVisibility::PROTECTED);
            var = DAEUtil::setElementVarDirection(var, openmodelica_frontend_types::DAE::VarDirection::BIDIR);
            var
        }
        _ => {
            metamodelica::print(literal!("generateProtectedElements failed!\n"));
            return Err("fail");
        }
    });
    Ok(newProts)
}

fn updateFunctionBody(
    mut funcIn: DAE::Function,
    mut body: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut idx: i32,
    mut outputs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut origOutputs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(DAE::Function, metamodelica::Ref<Absyn::Path>)> {
    let mut funcOut: DAE::Function = funcIn;
    let mut pathOut: metamodelica::Ref<Absyn::Path>;
    (funcOut, pathOut) = (match funcOut.clone() {
        DAE::Function::FUNCTION { .. } => {
            let mut s: ArcStr;
            s = AbsynUtil::pathLastIdent(var_field!(funcOut.path, DAE::Function::FUNCTION));
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("_eval"));
                __mm_s.push_str(&*intString(idx));
                ArcStr::from(__mm_s)
            };
            let __owned_variant_path_0 = AbsynUtil::pathSetLastIdent(
                &(AbsynUtil::makeNotFullyQualified(var_field!(funcOut.path, DAE::Function::FUNCTION).clone())),
                &s,
            );
            let __owned_variant_type__1 = updateFunctionType(
                var_field!(funcOut.type_, DAE::Function::FUNCTION).clone(),
                outputs,
                origOutputs,
            );
            let __owned_variant_functions_2 = list![DAE::FunctionDefinition::FUNCTION_DEF { body: body }];
            if let DAE::Function::FUNCTION {
                path, type_, functions, ..
            } = &mut funcOut
            {
                *path = __owned_variant_path_0;
                *type_ = __owned_variant_type__1;
                *functions = __owned_variant_functions_2;
            } else {
                panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION");
            }
            (
                funcOut.clone(),
                var_field!(funcOut.path, DAE::Function::FUNCTION).clone(),
            )
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("EvaluateFunctions.updateFunctionBody"));
                    __mm_s.push_str(&*literal!(" failed"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("BackEnd/EvaluateFunctions.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((funcOut, pathOut))
}

fn updateFunctionType(
    mut typIn: metamodelica::Ref<DAE::Type>,
    mut outputs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut originOutputs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> metamodelica::Ref<DAE::Type> {
    let mut typOut: metamodelica::Ref<DAE::Type>;
    typOut = 'mc: {
        let __mc_input = typIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        ty @ Deref @ DAE::Type::T_FUNCTION { .. } => {
                            let mut outTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut outNames: metamodelica::List<ArcStr>;
                            let mut ty = (*ty).clone();
                            outTypeLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut o in (outputs.clone()).into_iter().cloned() {
                            let __x = DAEUtil::getVariableType(&(o.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            outNames = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut o in (outputs.clone()).into_iter().cloned() {
                            let __x = DAEUtil::varName(&(o.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            assign_variant_field!(ty => DAE::Type::T_FUNCTION; funcResultType = if (intEq(((outTypeLst).len() as i32), 1)) {(outTypeLst).head().cloned()?} else {metamodelica::Ref::new(DAE::Type::T_TUPLE { types: outTypeLst.clone(), names: Some(outNames.clone()) })});
                            Ok(ty.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(typIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    typOut
}

fn buildPartialFunction(
    mut varPart: &(
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    ),
    mut constPart: &(
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
    mut copyOutStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut replIn: &BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut algsOut: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut eqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut constScalarCrefsInFunc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut varScalarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut constComplCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut constScalarCrefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut funcAlgs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut constComplExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut constScalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut lhsExps1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut lhsExps2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    (varScalarCrefs, funcAlgs) = varPart.clone();
    (
        constScalarCrefsInFunc,
        constScalarExps,
        constComplCrefs,
        constComplExps,
        constScalarCrefsOut,
    ) = constPart.clone();
    funcAlgs = List::filterOnTrue(
        funcAlgs,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(DAEUtil::isAlgorithm(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    lhsExps1 = List::map(constScalarCrefsOut, &Expression::crefExp)?;
    lhsExps2 = List::map(constComplCrefs, &Expression::crefExp)?;
    eqsOut = generateConstEqs(&lhsExps1, &constScalarExps, metamodelica::nil())?;
    eqsOut = generateConstEqs(&lhsExps2, &constComplExps, eqsOut)?;
    stmts1 = List::mapFlatReverse(&funcAlgs, &move |__a0: metamodelica::Ref<DAE::Element>| {
        DAEUtil::getStatement(&__a0)
    })?;
    (stmts1, _) = DAEUtil::traverseDAEEquationsStmts(
        stmts1,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                makeIdentCref,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )> + 'static,
                >),
            varScalarCrefs,
        ),
    )?;
    (stmts1, _) = DAEUtil::traverseDAEEquationsStmts(
        stmts1,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                makeIdentCref,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )> + 'static,
                >),
            constScalarCrefsInFunc,
        ),
    )?;
    stmts1 = listAppend(stmts1, copyOutStmts);
    algsOut = list![metamodelica::Ref::new(DAE::Element::ALGORITHM {
        algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts1 }),
        source: DAE::emptyElementSource().clone()
    })];
    Ok((algsOut, eqsOut))
}

fn stmtCanBeRemoved(
    mut stmtIn: metamodelica::Ref<DAE::Statement>,
    mut repl: BackendVarTransform::VariableReplacements,
) -> (metamodelica::Ref<DAE::Statement>, bool) {
    let mut tplOut: (metamodelica::Ref<DAE::Statement>, bool);
    tplOut = 'mc: {
        let __mc_input = &*stmtIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { .. } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceStatementLst(&(list![stmtIn.clone()]), repl.clone(), None, &(metamodelica::nil()), false)) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    stmt = metamodelica::Own::own(__pa0);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(stmt.clone()) {
                        Deref @ DAE::Statement::STMT_ASSIGN { exp1: __pa2, exp: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa2);
                    e2 = metamodelica::Own::own(__pa3);
                    b1 = Expression::isConst(e1.clone())?;
                    b2 = Expression::isConst(e2.clone())?;
                    stmt = stmtIn.clone();
                    Ok((stmt.clone(), b1 && b2))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((stmtIn.clone(), false))
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

fn traverseStmtsAndUpdate<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut stmtsIn: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Statement>,
                Type_a,
            ) -> Result<(metamodelica::Ref<DAE::Statement>, bool)>
            + 'static,
    >,
    mut argIn: Type_a,
    mut stmtsFold: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Statement>,
                Type_a,
            ) -> Result<(metamodelica::Ref<DAE::Statement>, bool)>
            + 'static,
    >;

    let mut stmtsOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    stmtsOut = 'mc: {
        let __mc_input = &**stmtsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    stmtsFold.clone().reverse();
                    Ok(stmtsFold.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { statementLst: stmtLst, else_, .. }, tail: rest } => {
                    let mut xs: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut stmtLst = (*stmtLst).clone();
                    x = (stmtsIn).head().cloned()?;
                    stmtLstLst = getDAEelseStatemntLsts(metamodelica::AsArg::as_arg(&else_), metamodelica::nil());
                    stmtLstLst = stmtLstLst.clone().reverse();
                    stmtLstLst = List::map3(stmtLstLst.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Statement>>, __a1: _, __a2: _, __a3: metamodelica::List<metamodelica::Ref<DAE::Statement>>| traverseStmtsAndUpdate(&__a0, __a1, __a2, &__a3), func.clone(), argIn.clone(), metamodelica::nil())?;
                    stmtLst = traverseStmtsAndUpdate(metamodelica::AsArg::as_arg(&stmtLst), func.clone(), argIn.clone(), &(metamodelica::nil()))?;
                    stmtLstLst = metamodelica::cons(stmtLst.clone(), stmtLstLst.clone());
                    x = updateStatementsInIfStmt(&stmtLstLst, &x)?;
                    xs = traverseStmtsAndUpdate(metamodelica::AsArg::as_arg(&rest), func.clone(), argIn.clone(), &(metamodelica::cons(x.clone(), stmtsFold.clone())))?;
                    Ok(xs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: x, tail: rest } => {
                    let mut b: bool;
                    let mut xs: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut x = (*x).clone();
                    (x, b) = func(x.clone(), argIn.clone())?;
                    xs = if (b) {stmtsFold.clone()} else {metamodelica::cons(x.clone(), stmtsFold.clone())};
                    xs = traverseStmtsAndUpdate(metamodelica::AsArg::as_arg(&rest), func.clone(), argIn.clone(), &xs)?;
                    Ok(xs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(stmtsOut)
}

fn makeIdentCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outCrefs) = (match &*inExp {
        DAE::Exp::CREF { componentRef: cref, ty } => {
            let mut crefs = inCrefs.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cref = (*cref).clone();
            cref = makeIdentCref2(cref.clone(), &crefs);
            exp = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cref.clone(),
                ty: ty.clone(),
            });
            (exp, crefs)
        }
        _ => (inExp, inCrefs),
    });
    (outExp, outCrefs)
}

fn makeIdentCref2(
    mut crefIn: metamodelica::Ref<DAE::ComponentRef>,
    mut changeTheseCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut crefOut: metamodelica::Ref<DAE::ComponentRef>;
    crefOut = 'mc: {
        let __mc_input = crefIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cref1 @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: i1, componentRef: cref2, .. } => {
                    let mut i2: ArcStr;
                    let mut i1 = (*i1).clone();
                    let mut cref2 = (*cref2).clone();
                    let true = (List::isMemberOnTrue(cref1.clone(), changeTheseCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?) else { return Err("pattern mismatch") };
                    i2 = ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cref2))?;
                    i1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*i1); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*i2); ArcStr::from(__mm_s) };
                    cref2 = replaceCrefIdent(cref2.clone(), i1.clone());
                    cref2 = makeIdentCref2(cref2.clone(), changeTheseCrefs);
                    Ok(cref2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cref1 @ Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
                    Ok(cref1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(crefIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    crefOut
}

fn replaceCrefIdent(
    mut crefIn: metamodelica::Ref<DAE::ComponentRef>,
    mut ident: ArcStr,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut crefOut: metamodelica::Ref<DAE::ComponentRef>;
    crefOut = (match &*crefIn {
        DAE::ComponentRef::CREF_QUAL {
            identType: typ,
            subscriptLst: sl,
            componentRef: cref2,
            ..
        } => {
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            cref = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: ident,
                identType: typ.clone(),
                subscriptLst: sl.clone(),
                componentRef: cref2.clone(),
            });
            cref
        }
        DAE::ComponentRef::CREF_IDENT {
            identType: typ,
            subscriptLst: sl,
            ..
        } => {
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            cref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: ident,
                identType: typ.clone(),
                subscriptLst: sl.clone(),
            });
            cref
        }
        _ => crefIn,
    });
    crefOut
}

fn statementRHSIsNotConst(mut stmt: &metamodelica::Ref<DAE::Statement>) -> Result<bool> {
    let mut notConst: bool;
    notConst = (match &**stmt {
        DAE::Statement::STMT_ASSIGN { exp: rhs, .. } => {
            let mut b: bool;
            b = Expression::isConst(rhs.clone())?;
            !(b)
        }
        _ => true,
    });
    Ok(notConst)
}

fn generateConstEqs<'__b>(
    mut lhsLst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rhsLst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut eqsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (lhsLst, rhsLst) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(eqsIn)
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, tail: lrest }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rrest }) => {
                let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                { (lhsLst, rhsLst, eqsIn) = (lrest, rrest, eqsIn); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: lhs, tail: lrest }, Deref @ metamodelica::ListNode::Cons { head: rhs, tail: rrest }) => {
                let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                eq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                { (lhsLst, rhsLst, eqsIn) = (lrest, rrest, metamodelica::cons(eq, eqsIn)); continue '__tco; }
            },
            _ => {
                metamodelica::print(literal!("generateConstEqs failed!\n"));
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn addReplacementRuleForAssignment(
    mut stmt: &metamodelica::Ref<DAE::Statement>,
    mut replIn: BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = (match &**stmt {
        DAE::Statement::STMT_ASSIGN {
            exp1: lhs, exp: rhs, ..
        } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            cref = Expression::expCref(lhs)?;
            repl = BackendVarTransform::addReplacement(replIn, cref, rhs.clone(), None)?;
            repl
        }
        _ => replIn,
    });
    Ok(replOut)
}

fn evaluateFunctions_updateAlgElements(
    mut element: metamodelica::Ref<DAE::Element>,
    mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut repl: BackendVarTransform::VariableReplacements,
    mut idx: i32,
    mut recursionLimit: i32,
) -> Result<(
    metamodelica::Ref<DAE::Element>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    BackendVarTransform::VariableReplacements,
    i32,
)> {
    let mut element: metamodelica::Ref<DAE::Element> = element;
    let mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree> = funcTree;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut idx: i32 = idx;
    element = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut alg = (*alg).clone();
            stmts = DAEUtil::getStatement(&element)?;
            (stmts, funcTree, repl, idx) = evaluateFunctions_updateStatement(stmts, funcTree, repl, idx, &(metamodelica::nil()), recursionLimit)?;
            alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts });
            metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::VAR { componentRef: cref, binding: Some(exp), .. } => {
            let mut scalarExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut scalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut exp = (*exp).clone();
            (exp, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), &repl, None);
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            if Expression::isConst(exp.clone())? {
                repl = BackendVarTransform::addReplacement(repl, cref.clone(), exp.clone(), None)?;
                scalars = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cref), false)?;
                scalarExps = Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp));
                if ((scalars).len() as i32) == ((scalarExps).len() as i32) {
                    repl = BackendVarTransform::addReplacements(repl, &scalars, &scalarExps, None)?;
                }
            }
            DAEUtil::replaceBindungInVar(exp.clone(), &element)?
        },
        _ => {
            element
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((element, funcTree, repl, idx))
}

fn unboxExp<'__b>(mut ie: &'__b metamodelica::Ref<DAE::Exp>, mut bIn: bool) -> (metamodelica::Ref<DAE::Exp>, bool) {
    '__tco: loop {
        match &**ie {
            DAE::Exp::BOX { exp: e } => {
                (ie, bIn) = (e, true);
                continue '__tco;
            }
            _ => return (ie.clone(), bIn),
        }
    }
}

fn evaluateFunctions_updateStatement(
    mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut repl: BackendVarTransform::VariableReplacements,
    mut idx: i32,
    mut lstIn: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut recursionLimit: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    BackendVarTransform::VariableReplacements,
    i32,
)> {
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts;
    let mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree> = funcTree;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut idx: i32 = idx;
    let mut stmtsList: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
    stmtsList = ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>> = metamodelica::nil();
        for mut stmt in (stmts).into_iter().cloned() {
            let __x = (match &*stmt.clone() {
                DAE::Statement::STMT_ASSIGN {
                    type_: typ,
                    exp1,
                    exp: exp2,
                    source,
                } => {
                    let mut isCon: bool;
                    let mut isRec: bool;
                    let mut isTpl: bool;
                    let mut eqDim: bool;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut stmt1: metamodelica::Ref<DAE::Statement>;
                    let mut scalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varScalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constScalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut outputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut addStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut tplStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tplExpsLHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tplExpsRHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut lhsExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp1 = (*exp1).clone();
                    let mut exp2 = (*exp2).clone();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("assignment:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    cref = Expression::expCref(metamodelica::AsArg::as_arg(&exp1))?;
                    scalars = getRecordScalars(&cref);
                    (exp2, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp2), &repl, None);
                    (exp2, _) = ExpressionSimplify::simplify(exp2.clone())?;
                    (exp2, exp1, funcTree, idx, addStmts) = evaluateConstantFunctionCall(
                        exp2.clone(),
                        exp1.clone(),
                        funcTree.clone(),
                        idx,
                        recursionLimit,
                    )?;
                    (exp2, _) = ExpressionSimplify::simplify(exp2.clone())?;
                    (exp2, _) = Expression::traverseExpBottomUp(
                        exp2.clone(),
                        &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: bool| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(unboxExp(&__a0, __a1))
                        },
                        false,
                    )?;
                    expLst = Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp2));
                    repl = List::fold(
                        &addStmts,
                        &move |__a0: metamodelica::Ref<DAE::Statement>,
                               __a1: BackendVarTransform::VariableReplacements| {
                            addReplacementRuleForAssignment(&__a0, __a1)
                        },
                        repl.clone(),
                    )?;
                    lhsExps = Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp1));
                    outputs = List::map(lhsExps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| {
                        Expression::expCref(&__a0)
                    })?;
                    BackendVarTransform::removeReplacements(&repl, &outputs)?;
                    isCon =
                        Expression::isConst(exp2.clone())? && !(Expression::isCall(metamodelica::AsArg::as_arg(&exp2)));
                    eqDim = ((scalars).len() as i32) == ((expLst).len() as i32);
                    isRec = ComponentReference::isRecord(&cref)
                        || Expression::isRecordCall(metamodelica::AsArg::as_arg(&exp2), &funcTree)?;
                    isTpl = Expression::isTuple(metamodelica::AsArg::as_arg(&exp1))
                        && Expression::isTuple(metamodelica::AsArg::as_arg(&exp2));
                    scalars = if (isRec && eqDim) {
                        scalars.clone()
                    } else {
                        metamodelica::nil()
                    };
                    expLst = if (isRec && eqDim) {
                        expLst.clone()
                    } else {
                        metamodelica::nil()
                    };
                    (_, varScalars) = List::filterOnTrueSync(&expLst, &Expression::isNotConst, scalars.clone())?;
                    (expLst, constScalars) = List::filterOnTrueSync(&expLst, &Expression::isConst, scalars.clone())?;
                    repl = if (isCon && !(isRec)) {
                        BackendVarTransform::addReplacement(repl.clone(), cref.clone(), exp2.clone(), None)?
                    } else {
                        repl.clone()
                    };
                    repl = if (isCon && isRec) {
                        BackendVarTransform::addReplacements(repl.clone(), &scalars, &expLst, None)?
                    } else {
                        repl.clone()
                    };
                    if !(isCon) {
                        if !(isRec) {
                            BackendVarTransform::removeReplacement(&repl, cref.clone())?;
                        } else {
                            BackendVarTransform::removeReplacements(&repl, &varScalars)?;
                            repl = BackendVarTransform::addReplacements(repl.clone(), &constScalars, &expLst, None)?;
                        }
                    }
                    stmt1 = if (isCon) {
                        metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                            type_: typ.clone(),
                            exp1: exp1.clone(),
                            exp: exp2.clone(),
                            source: source.clone(),
                        })
                    } else {
                        stmt.clone()
                    };
                    tplExpsLHS = if (isTpl) {
                        Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp1))
                    } else {
                        metamodelica::nil()
                    };
                    tplExpsRHS = if (isTpl) {
                        Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp2))
                    } else {
                        metamodelica::nil()
                    };
                    tplStmts = makeAssignmentMap(tplExpsLHS.clone(), tplExpsRHS.clone())?;
                    stmts1 = if (isTpl) {
                        tplStmts.clone()
                    } else {
                        list![stmt1.clone()]
                    };
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("evaluated assignment to:\n"));
                            __mm_s.push_str(&*stringDelimitList(
                                List::map(
                                    stmts1.clone(),
                                    &fnptr!(DAEDump::ppStatementStr, metamodelica::Ref<DAE::Statement>),
                                )?,
                                literal!("\n"),
                            ));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    stmts1.clone()
                }
                DAE::Statement::STMT_ASSIGN_ARR {
                    type_: typ,
                    lhs: exp1,
                    exp: exp2,
                    source,
                } => {
                    let mut isCon: bool;
                    let mut isRec: bool;
                    let mut isTpl: bool;
                    let mut eqDim: bool;
                    let mut isArr: bool;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut stmt1: metamodelica::Ref<DAE::Statement>;
                    let mut scalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varScalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constScalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut outputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut addStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut tplStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tplExpsLHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tplExpsRHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut lhsExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp1 = (*exp1).clone();
                    let mut exp2 = (*exp2).clone();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Array assignment:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    cref = Expression::expCref(metamodelica::AsArg::as_arg(&exp1))?;
                    scalars = getRecordScalars(&cref);
                    (exp2, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp2), &repl, None);
                    (exp2, exp1, funcTree, idx, addStmts) = evaluateConstantFunctionCall(
                        exp2.clone(),
                        exp1.clone(),
                        funcTree.clone(),
                        idx,
                        recursionLimit,
                    )?;
                    (exp2, _) = ExpressionSimplify::simplify(exp2.clone())?;
                    expLst = Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp2));
                    repl = List::fold(
                        &addStmts,
                        &move |__a0: metamodelica::Ref<DAE::Statement>,
                               __a1: BackendVarTransform::VariableReplacements| {
                            addReplacementRuleForAssignment(&__a0, __a1)
                        },
                        repl.clone(),
                    )?;
                    lhsExps = Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp1));
                    outputs = List::map(lhsExps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| {
                        Expression::expCref(&__a0)
                    })?;
                    BackendVarTransform::removeReplacements(&repl, &outputs)?;
                    isCon =
                        Expression::isConst(exp2.clone())? && !(Expression::isCall(metamodelica::AsArg::as_arg(&exp2)));
                    eqDim = ((scalars).len() as i32) == ((expLst).len() as i32);
                    isRec = ComponentReference::isRecord(&cref);
                    isArr = ComponentReference::isArrayElement(&cref);
                    isTpl = Expression::isTuple(metamodelica::AsArg::as_arg(&exp1))
                        && Expression::isTuple(metamodelica::AsArg::as_arg(&exp2));
                    scalars = if ((isRec || isArr) && eqDim) {
                        scalars.clone()
                    } else {
                        metamodelica::nil()
                    };
                    expLst = if ((isRec || isArr) && eqDim) {
                        expLst.clone()
                    } else {
                        metamodelica::nil()
                    };
                    (_, varScalars) = List::filterOnTrueSync(&expLst, &Expression::isNotConst, scalars.clone())?;
                    (expLst, constScalars) = List::filterOnTrueSync(&expLst, &Expression::isConst, scalars.clone())?;
                    repl = if (isCon && !(isRec)) {
                        BackendVarTransform::addReplacement(repl.clone(), cref.clone(), exp2.clone(), None)?
                    } else {
                        repl.clone()
                    };
                    repl = if (isCon && isRec) {
                        BackendVarTransform::addReplacements(repl.clone(), &scalars, &expLst, None)?
                    } else {
                        repl.clone()
                    };
                    repl = if (isCon && isArr) {
                        BackendVarTransform::addReplacements(repl.clone(), &scalars, &expLst, None)?
                    } else {
                        repl.clone()
                    };
                    if !(isCon) {
                        if !(isRec) {
                            BackendVarTransform::removeReplacement(&repl, cref.clone())?;
                        } else {
                            BackendVarTransform::removeReplacements(&repl, &varScalars)?;
                            repl = BackendVarTransform::addReplacements(repl.clone(), &constScalars, &expLst, None)?;
                        }
                    }
                    stmt1 = if (isCon) {
                        metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                            type_: typ.clone(),
                            exp1: exp1.clone(),
                            exp: exp2.clone(),
                            source: source.clone(),
                        })
                    } else {
                        stmt.clone()
                    };
                    tplExpsLHS = if (isTpl) {
                        Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp1))
                    } else {
                        metamodelica::nil()
                    };
                    tplExpsRHS = if (isTpl) {
                        Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp2))
                    } else {
                        metamodelica::nil()
                    };
                    tplStmts = makeAssignmentMap(tplExpsLHS.clone(), tplExpsRHS.clone())?;
                    stmts1 = if (isTpl) {
                        tplStmts.clone()
                    } else {
                        list![stmt1.clone()]
                    };
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("evaluated array assignment to:\n"));
                            __mm_s.push_str(&*stringDelimitList(
                                List::map(
                                    stmts1.clone(),
                                    &fnptr!(DAEDump::ppStatementStr, metamodelica::Ref<DAE::Statement>),
                                )?,
                                literal!("\n"),
                            ));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    stmts1.clone()
                }
                DAE::Statement::STMT_IF {
                    statementLst: stmtsIf,
                    else_,
                    ..
                } => {
                    let mut predicted: bool;
                    let mut isEval: bool;
                    let mut outputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut addStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtsNew: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut allStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("IF-statement:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    stmtsList = getDAEelseStatemntLsts(metamodelica::AsArg::as_arg(&else_), metamodelica::nil());
                    stmtsList = stmtsList.clone().reverse();
                    stmtsList = metamodelica::cons(stmtsIf.clone(), stmtsList.clone());
                    allStmts = List::flatten(stmtsList.clone())?;
                    outputs = getStatementsOutputs(&allStmts, funcTree.clone())?;
                    (isEval, stmts1, repl) = evaluateIfStatement(
                        stmt.clone(),
                        &(FuncInfo {
                            repl: repl.clone(),
                            funcTree: funcTree.clone(),
                            idx: idx,
                        }),
                        recursionLimit,
                    )?;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isEval) {
                        metamodelica::print(literal!("-->try to predict the outputs \n"));
                    }
                    if !(isEval) {
                        let (
                            (__pa0, __pa1),
                            FuncInfo {
                                repl: __pa2,
                                funcTree: __pa3,
                                idx: __pa4,
                            },
                        ) = predictIfOutput(
                            stmt.clone(),
                            FuncInfo {
                                repl: repl.clone(),
                                funcTree: funcTree.clone(),
                                idx: idx,
                            },
                            recursionLimit,
                        );
                        stmtsNew = metamodelica::Own::own(__pa0);
                        addStmts = metamodelica::Own::own(__pa1);
                        repl = metamodelica::Own::own(__pa2);
                        funcTree = metamodelica::Own::own(__pa3);
                        idx = metamodelica::Own::own(__pa4);
                    } else {
                        stmtsNew = stmts1.clone();
                        addStmts = metamodelica::nil();
                    }
                    predicted = !((addStmts).is_empty()) || (stmtsNew).is_empty() && !(isEval);
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isEval) {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("could it be predicted? "));
                            __mm_s.push_str(&*boolString(predicted));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    if !(predicted) && !(isEval) {
                        BackendVarTransform::removeReplacements(&repl, &outputs)?;
                    }
                    stmts1 = if (predicted) { stmtsNew.clone() } else { stmts1.clone() };
                    (addStmts, funcTree, repl, idx) = evaluateFunctions_updateStatement(
                        addStmts.clone(),
                        funcTree.clone(),
                        repl.clone(),
                        idx,
                        &(metamodelica::nil()),
                        recursionLimit,
                    )?;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("evaluated IF-statements to:\n"));
                            __mm_s.push_str(&*stringDelimitList(
                                List::map(
                                    listAppend(stmts1.clone(), addStmts.clone()),
                                    &fnptr!(DAEDump::ppStatementStr, metamodelica::Ref<DAE::Statement>),
                                )?,
                                literal!("\n"),
                            ));
                            __mm_s.push_str(&*literal!("\n\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    listAppend(stmts1.clone(), addStmts.clone())
                }
                DAE::Statement::STMT_TUPLE_ASSIGN {
                    expExpLst: expLst,
                    exp: exp0,
                    ..
                } => {
                    let mut isCon: bool;
                    let mut size: i32;
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    let mut typ: metamodelica::Ref<DAE::Type>;
                    let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut varScalars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtsNew: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut tplExpsLHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tplExpsRHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Tuple-statement:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    (exp1, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp0), &repl, None);
                    exp2 = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst.clone() });
                    (exp1, exp2, addEqs, funcTree, idx, _, _) = evaluateConstantFunction(
                        exp1.clone(),
                        exp2.clone(),
                        funcTree.clone(),
                        idx,
                        metamodelica::nil(),
                        recursionLimit,
                    )?;
                    isCon = Expression::isConst(exp1.clone())?;
                    exp1 = if (isCon) { exp1.clone() } else { exp0.clone() };
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("--> is the tuple const? "));
                            __mm_s.push_str(&*boolString(isCon));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    varScalars = List::map(expLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| {
                        Expression::expCref(&__a0)
                    })?;
                    if !(isCon) {
                        BackendVarTransform::removeReplacements(&repl, &varScalars)?;
                    } else {
                        repl = addTplReplacements(repl.clone(), exp1.clone(), exp2.clone());
                    }
                    size = DAEUtil::getTupleSize(&exp2);
                    typ = Expression::r#typeof(exp2.clone())?;
                    tplExpsLHS = DAEUtil::getTupleExps(exp2.clone());
                    tplExpsLHS = if (isCon) {
                        tplExpsLHS.clone()
                    } else {
                        metamodelica::nil()
                    };
                    tplExpsRHS = DAEUtil::getTupleExps(exp1.clone());
                    tplExpsRHS = if (isCon) {
                        tplExpsRHS.clone()
                    } else {
                        metamodelica::nil()
                    };
                    stmtsNew = makeAssignmentMap(tplExpsLHS.clone(), tplExpsRHS.clone())?;
                    stmtsNew = if (isCon) { stmtsNew.clone() } else { list![stmt.clone()] };
                    stmts2 = if (intEq(size, 0)) {
                        list![metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                            type_: typ.clone(),
                            exp1: exp2.clone(),
                            exp: exp1.clone(),
                            source: DAE::emptyElementSource().clone()
                        })]
                    } else {
                        stmtsNew.clone()
                    };
                    stmts1 = List::map(addEqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| {
                        equationToStatement(&__a0)
                    })?;
                    stmts1 = listAppend(stmts2.clone(), stmts1.clone());
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("evaluated Tuple-statements to (incl. addEqs):\n"));
                            __mm_s.push_str(&*stringDelimitList(
                                List::map(
                                    stmts1.clone(),
                                    &fnptr!(DAEDump::ppStatementStr, metamodelica::Ref<DAE::Statement>),
                                )?,
                                literal!("\n"),
                            ));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    stmts1.clone().reverse()
                }
                DAE::Statement::STMT_FOR {
                    statementLst: stmts1, ..
                } => {
                    let mut stmts1 = (*stmts1).clone();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("For-statement:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    (stmts1, funcTree, repl, idx) =
                        evaluateForStatement(stmt.clone(), funcTree.clone(), repl.clone(), idx, recursionLimit)?;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("evaluated for-statements to:\n"));
                            __mm_s.push_str(&*stringDelimitList(
                                List::map(
                                    stmts1.clone(),
                                    &fnptr!(DAEDump::ppStatementStr, metamodelica::Ref<DAE::Statement>),
                                )?,
                                literal!("\n"),
                            ));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    stmts1.clone().reverse()
                }
                DAE::Statement::STMT_WHILE {
                    statementLst: stmts1, ..
                } => {
                    let mut outputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("While-statement (not evaluated):\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    outputs = getStatementsOutputs(metamodelica::AsArg::as_arg(&stmts1), funcTree.clone())?;
                    BackendVarTransform::removeReplacements(&repl, &outputs)?;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("evaluated While-statement to:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    list![stmt.clone()]
                }
                DAE::Statement::STMT_ASSERT {
                    cond, msg, level: lvl, ..
                } => {
                    let mut cond = (*cond).clone();
                    let mut msg = (*msg).clone();
                    (cond, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&cond), &repl, None);
                    cond = evaluateConstantFunctionCallExp(
                        metamodelica::AsArg::as_arg(&cond),
                        &funcTree,
                        false,
                        recursionLimit,
                    );
                    (cond, _) = ExpressionSimplify::simplify(cond.clone())?;
                    (msg, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&msg), &repl, None);
                    msg = evaluateConstantFunctionCallExp(
                        metamodelica::AsArg::as_arg(&msg),
                        &funcTree,
                        false,
                        recursionLimit,
                    );
                    (msg, _) = ExpressionSimplify::simplify(msg.clone())?;
                    if ExpressionBasics::expEqual(
                        metamodelica::AsArg::as_arg(&cond),
                        metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                    )? && metamodelica::stringEq(
                        &(Expression::sconstEnumNameString(metamodelica::AsArg::as_arg(&lvl))?),
                        &(literal!("AssertionLevel.error")),
                    ) {
                        if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                            metamodelica::print({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("ERROR: "));
                                __mm_s.push_str(&*ExpressionBasics::printExpStr(msg.clone())?);
                                __mm_s.push_str(&*literal!("\n"));
                                ArcStr::from(__mm_s)
                            });
                        }
                        return Err("fail");
                    } else if ExpressionBasics::expEqual(
                        metamodelica::AsArg::as_arg(&cond),
                        metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                    )? && metamodelica::stringEq(
                        &(Expression::sconstEnumNameString(metamodelica::AsArg::as_arg(&lvl))?),
                        &(literal!("AssertionLevel.warning")),
                    ) {
                        if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                            metamodelica::print({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("WARNING: "));
                                __mm_s.push_str(&*ExpressionBasics::printExpStr(msg.clone())?);
                                __mm_s.push_str(&*literal!("\n"));
                                ArcStr::from(__mm_s)
                            });
                        }
                        return Err("fail");
                    }
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("assert-statement:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    list![stmt.clone()]
                }
                DAE::Statement::STMT_TERMINATE { .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("terminate-statement:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    list![stmt.clone()]
                }
                DAE::Statement::STMT_REINIT { .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("reinit-statement:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    list![stmt.clone()]
                }
                DAE::Statement::STMT_NORETCALL { .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("noretcall-statement (not evaluated):\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    list![stmt.clone()]
                }
                DAE::Statement::STMT_RETURN { .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("return-statement:\n"));
                            __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone()));
                            ArcStr::from(__mm_s)
                        });
                    }
                    list![stmt.clone()]
                }
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    stmts = List::flatten(stmtsList)?;
    Ok((stmts, funcTree, repl, idx))
}

fn evaluateForStatement(
    mut stmtIn: metamodelica::Ref<DAE::Statement>,
    mut funcTreeIn: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut replIn: BackendVarTransform::VariableReplacements,
    mut idxIn: i32,
    mut recursionLimit: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    BackendVarTransform::VariableReplacements,
    i32,
)> {
    let mut stmtsOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut funcTreeOut: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut idxOut: i32;
    let mut hasNoRepl: bool;
    let mut i: i32 = 0;
    let mut start: i32;
    let mut stop: i32;
    let mut step: i32;
    let mut iter: ArcStr;
    let mut range: metamodelica::Ref<DAE::Exp>;
    let mut outputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut lhsExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut lhsExpLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut stmtsIn: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(stmtIn.clone()) {
        Deref @ DAE::Statement::STMT_FOR { iter: __pa0, range: __pa1, statementLst: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iter = metamodelica::Own::own(__pa0);
    range = metamodelica::Own::own(__pa1);
    stmtsIn = metamodelica::Own::own(__pa2);
    match '__try3: {
        (range, _) = BackendVarTransform::replaceExp(&range, &replIn, None);
        (start, stop, step) = unwrap_break_err!(getRangeBounds(&range), '__try3);
        let true = (intEq(step, 1)) else {
            break '__try3 Err::<_, _>("pattern mismatch");
        };
        let true = (intGe(stop, start)) else {
            break '__try3 Err::<_, _>("pattern mismatch");
        };
        repl = replIn.clone();
        for mut i in start..=stop {
            repl = unwrap_break_err!(BackendVarTransform::addReplacement(repl.clone(), ComponentReferenceBasics::makeCrefIdent(iter.clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil()), metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }), None), '__try3);
            (stmts, _, repl, _) = unwrap_break_err!(evaluateFunctions_updateStatement(stmtsIn.clone(), funcTreeIn.clone(), repl.clone(), i, &(metamodelica::nil()), recursionLimit), '__try3);
            outputs = unwrap_break_err!(getStatementsOutputs(&stmts, funcTreeIn.clone()), '__try3);
            hasNoRepl = unwrap_break_err!(List::applyAndFold1(&outputs, &fnptr!(boolAnd, bool, bool), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: BackendVarTransform::VariableReplacements| BackendVarTransform::hasNoReplacement(__a0, &__a1), repl.clone(), true), '__try3);
            if hasNoRepl {
                if unwrap_break_err!(Flags::isSet(Flags::EVAL_FUNC_DUMP.clone()), '__try3) {
                    metamodelica::print(literal!(
                        "For-loop evaluation is skipped, since the first loop evaluated nothing.\n"
                    ));
                }
                break '__try3 Err::<_, _>("fail");
            }
        }
        unwrap_break_err!(BackendVarTransform::removeReplacement(&repl, ComponentReferenceBasics::makeCrefIdent(iter.clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil())), '__try3);
        funcTreeOut = funcTreeIn.clone();
        idxOut = idxIn;
        stmtsOut = stmts.clone();
        Ok::<_, &'static str>((funcTreeOut.clone(), idxOut.clone(), repl.clone(), stmtsOut.clone()))
    } {
        Ok((__try3_o0, __try3_o1, __try3_o2, __try3_o3)) => {
            funcTreeOut = __try3_o0;
            idxOut = __try3_o1;
            repl = __try3_o2;
            stmtsOut = __try3_o3;
        }
        Err(_) => {
            lhsExps = List::fold(&stmtsIn, &getStatementLHS, metamodelica::nil())?;
            lhsExps = List::unique(&lhsExps);
            lhsExpLst = List::map(
                lhsExps.clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Expression::getComplexContents(&__a0))
                },
            )?;
            lhsExps = listAppend(List::flatten(lhsExpLst.clone())?, lhsExps.clone());
            outputs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut e in (lhsExps.clone()).into_iter().cloned() {
                    if !(Expression::isCref(&(e.clone()))) {
                        continue;
                    }
                    let __x = Expression::expCref(&(e.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            repl = replIn.clone();
            BackendVarTransform::removeReplacements(&repl, &outputs)?;
            stmtsOut = list![stmtIn.clone()];
            funcTreeOut = funcTreeIn.clone();
            idxOut = idxIn;
        }
    }
    Ok((stmtsOut, funcTreeOut, repl, idxOut))
}

fn getRangeBounds(mut range: &metamodelica::Ref<DAE::Exp>) -> Result<(i32, i32, i32)> {
    let mut start: i32;
    let mut stop: i32;
    let mut step: i32;
    (start, stop, step) = (::match_deref::match_deref! { match range {
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: i1 }, step: None, stop: Deref @ DAE::Exp::ICONST { integer: i2 }, .. } => {
            (i1.clone(), i2.clone(), 1)
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: i1 }, step: Some(Deref @ DAE::Exp::ICONST { integer: i3 }), stop: Deref @ DAE::Exp::ICONST { integer: i2 }, .. } => {
            (i1.clone(), i2.clone(), i3.clone())
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start, stop, step))
}

fn evaluateIfStatement(
    mut stmtIn: metamodelica::Ref<DAE::Statement>,
    mut info: &FuncInfo,
    mut recursionLimit: i32,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut isEval: bool;
    let mut stmtsOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut replOut: BackendVarTransform::VariableReplacements;
    (isEval, stmtsOut, replOut) = 'mc: {
        let __mc_input = (&*stmtIn, info);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { exp: expIf, statementLst: stmtsIf, else_, .. }, FuncInfo { repl: replIn, funcTree, idx }) => {
                    let mut isIf: bool;
                    let mut isCon: bool;
                    let mut isElse: bool;
                    let mut eval: bool;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtsElse: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut funcTree = (*funcTree).clone();
                    let mut idx = (*idx).clone();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print(literal!("-->try to check if its the if case\n"));
                    }
                    (exp1, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&expIf), metamodelica::AsArg::as_arg(&replIn), None);
                    (exp1, _, _, _, _) = evaluateConstantFunctionCall(exp1.clone(), exp1.clone(), funcTree.clone(), idx.clone(), recursionLimit)?;
                    (exp1, _) = BackendVarTransform::replaceExp(&exp1, metamodelica::AsArg::as_arg(&replIn), None);
                    (exp1, _) = ExpressionSimplify::simplify(exp1.clone())?;
                    isCon = Expression::isConst(exp1.clone())?;
                    isIf = if (isCon) {Expression::toBool(&exp1)?} else {false};
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-->is the if const? ")); __mm_s.push_str(&*boolString(isCon)); __mm_s.push_str(&*literal!(" and is it the if case ? ")); __mm_s.push_str(&*boolString(isIf)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    if isIf && isCon {
                        (stmts1, funcTree, repl, idx) = evaluateFunctions_updateStatement(stmtsIf.clone(), funcTree.clone(), replIn.clone(), idx.clone(), &(metamodelica::nil()), recursionLimit)?;
                    } else {
                        stmts1 = list![stmtIn.clone()];
                        repl = replIn.clone();
                    }
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isIf) {
                        metamodelica::print(literal!("-->try to check if its another case\n"));
                    }
                    if isCon && !(isIf) {
                        (stmtsElse, isElse) = evaluateElse(metamodelica::AsArg::as_arg(&else_), info, recursionLimit)?;
                    } else {
                        stmtsElse = list![stmtIn.clone()];
                        isElse = false;
                    }
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isIf) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-->is it an other case? ")); __mm_s.push_str(&*boolString(isElse)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    if isCon && isElse {
                        (stmts1, funcTree, repl, idx) = evaluateFunctions_updateStatement(stmtsElse.clone(), funcTree.clone(), replIn.clone(), idx.clone(), &(metamodelica::nil()), recursionLimit)?;
                    }
                    eval = isCon && (isIf || isElse);
                    Ok((eval, stmts1.clone(), repl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print(literal!("evaluateIfStatement failed \n"));
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
    Ok((isEval, stmtsOut, replOut))
}

fn evaluateElse(
    mut elseIn: &metamodelica::Ref<DAE::Else>,
    mut info: &FuncInfo,
    mut recursionLimit: i32,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, bool)> {
    let mut stmtsOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut isElse: bool;
    (stmtsOut, isElse) = (::match_deref::match_deref! { match &((&**elseIn, info)) {
        (Deref @ DAE::Else::ELSEIF { exp: expIf, statementLst: stmts, else_ }, FuncInfo { repl: replIn, funcTree, idx }) => {
            let mut isCon: bool;
            let mut isElseIf: bool;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut stmts = (*stmts).clone();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                metamodelica::print(literal!("-->try to check if its the elseif case\n"));
            }
            (exp1, _, _, _, _) = evaluateConstantFunctionCall(expIf.clone(), expIf.clone(), funcTree.clone(), idx.clone(), recursionLimit)?;
            (exp1, _) = BackendVarTransform::replaceExp(&exp1, metamodelica::AsArg::as_arg(&replIn), None);
            (exp1, _) = ExpressionSimplify::simplify(exp1)?;
            isCon = Expression::isConst(exp1.clone())?;
            isElseIf = if (isCon) {Expression::toBool(&exp1)?} else {false};
            if isCon && !(isElseIf) {
                (stmts, isElseIf) = evaluateElse(metamodelica::AsArg::as_arg(&else_), info, recursionLimit)?;
            }
            (stmts.clone(), isElseIf)
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, FuncInfo { .. }) => {
            (stmts.clone(), true)
        },
        (Deref @ DAE::Else::NOELSE { .. }, FuncInfo { .. }) => {
            (metamodelica::nil(), true)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((stmtsOut, isElse))
}

fn addTplReplacements(
    mut replIn: BackendVarTransform::VariableReplacements,
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> BackendVarTransform::VariableReplacements {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = 'mc: {
        let __mc_input = &*e2;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tplLHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tplRHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    tplRHS = DAEUtil::getTupleExps(e1.clone());
                    tplLHS = DAEUtil::getTupleExps(e2.clone());
                    crefs = List::map(tplLHS.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    repl = BackendVarTransform::addReplacements(replIn.clone(), &crefs, &tplRHS, None)?;
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(replIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    replOut
}

fn equationToStatement(
    mut eqIn: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut stmtOut: metamodelica::Ref<DAE::Statement>;
    stmtOut = (match &**eqIn {
        BackendDAE::Equation::EQUATION {
            exp: lhs,
            scalar: rhs,
            source,
            ..
        } => {
            let mut typ: metamodelica::Ref<DAE::Type>;
            typ = Expression::r#typeof(lhs.clone())?;
            metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                type_: typ,
                exp1: lhs.clone(),
                exp: rhs.clone(),
                source: source.clone(),
            })
        }
        _ => {
            metamodelica::print(literal!("equationToStatement failed!\n"));
            return Err("fail");
        }
    });
    Ok(stmtOut)
}

fn replaceExps(
    mut replIn: BackendVarTransform::VariableReplacements,
    mut expsIn: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut expsOut: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (expsOut, _) = List::map2_2(
        expsIn,
        &move |__a0: metamodelica::Ref<DAE::Exp>,
               __a1: BackendVarTransform::VariableReplacements,
               __a2: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVarTransform::replaceExp(&__a0, &__a1, __a2))
        },
        replIn,
        None,
    )?;
    Ok(expsOut)
}

fn getStatementLHS(
    mut stmt: metamodelica::Ref<DAE::Statement>,
    mut expsIn: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(stmt.clone()) {
            Deref @ DAE::Statement::STMT_ASSIGN { exp1: exp, .. } => {
                return Ok(metamodelica::cons(exp.clone(), expsIn))
            },
            Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, .. } => {
                return Ok(listAppend(expLst.clone(), expsIn))
            },
            Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp, .. } => {
                return Ok(metamodelica::cons(exp.clone(), expsIn))
            },
            Deref @ DAE::Statement::STMT_IF { statementLst: stmtLst1, else_, .. } => {
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut stmtLst2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut stmtLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
                stmtLstLst = getDAEelseStatemntLsts(metamodelica::AsArg::as_arg(&else_), metamodelica::nil());
                stmtLst2 = List::flatten(stmtLstLst)?;
                stmtLst2 = listAppend(stmtLst1.clone(), stmtLst2);
                return Ok(List::fold(&stmtLst2, &getStatementLHS, expsIn)?)
            },
            Deref @ DAE::Statement::STMT_FOR { statementLst: stmtLst1, .. } => {
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                return Ok(List::fold(metamodelica::AsArg::as_arg(&stmtLst1), &getStatementLHS, expsIn)?)
            },
            Deref @ DAE::Statement::STMT_PARFOR { statementLst: stmtLst1, .. } => {
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                return Ok(List::fold(metamodelica::AsArg::as_arg(&stmtLst1), &getStatementLHS, expsIn)?)
            },
            Deref @ DAE::Statement::STMT_WHILE { statementLst: stmtLst1, .. } => {
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                return Ok(List::fold(metamodelica::AsArg::as_arg(&stmtLst1), &getStatementLHS, expsIn)?)
            },
            Deref @ DAE::Statement::STMT_WHEN { statementLst: stmtLst1, elseWhen: Some(stmt1), .. } => {
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" check getStatementLHS for WHEN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                }
                expLst = List::fold(metamodelica::AsArg::as_arg(&stmtLst1), &getStatementLHS, expsIn)?;
                { (stmt, expsIn) = (stmt1.clone(), expLst); continue '__tco; }
            },
            Deref @ DAE::Statement::STMT_WHEN { statementLst: stmtLst1, elseWhen: None, .. } => {
                let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" check getStatementLHS for WHEN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                }
                return Ok(List::fold(metamodelica::AsArg::as_arg(&stmtLst1), &getStatementLHS, expsIn)?)
            },
            Deref @ DAE::Statement::STMT_ASSERT { .. } => {
                return Ok(expsIn)
            },
            Deref @ DAE::Statement::STMT_TERMINATE { .. } => {
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for TERMINATE!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                }
                return Ok(return Err("fail"))
            },
            Deref @ DAE::Statement::STMT_REINIT { .. } => {
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for REINIT!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                }
                return Ok(return Err("fail"))
            },
            Deref @ DAE::Statement::STMT_NORETCALL { .. } => {
                return Ok(expsIn)
            },
            Deref @ DAE::Statement::STMT_RETURN { .. } => {
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for RETURN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                }
                return Ok(return Err("fail"))
            },
            Deref @ DAE::Statement::STMT_BREAK { .. } => {
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for BREAK!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                }
                return Ok(return Err("fail"))
            },
            Deref @ DAE::Statement::STMT_ARRAY_INIT { .. } => {
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for ARRAY_INIT!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                }
                return Ok(return Err("fail"))
            },
            _ => {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for !\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) });
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getStatementLHSScalar(
    mut stmt: metamodelica::Ref<DAE::Statement>,
    mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut expsIn: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    lhs = 'mc: {
        let __mc_input = &*stmt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { exp1: exp, exp: Deref @ DAE::Exp::CALL { path, .. }, .. } => {
                    let mut lhsCref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut func: DAE::Function;
                    let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut algs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut stmtLst1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&funcTree, path.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    func = metamodelica::Own::own(__pa0);
                    elements = DAEUtil::getFunctionElements(&func)?;
                    algs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isAlgorithm(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                    stmtLstLst = List::map(algs.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::getStatement(&__a0))?;
                    stmtLst1 = List::flatten(stmtLstLst.clone())?;
                    expLst = List::fold1(&stmtLst1, &getStatementLHSScalar, funcTree.clone(), metamodelica::nil())?;
                    outputCrefs = List::map(expLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    lhsCref = Expression::expCref(metamodelica::AsArg::as_arg(&exp))?;
                    outputCrefs = List::filterOnTrue(outputCrefs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(ComponentReference::crefIsNotIdent(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    outputCrefs = List::map(outputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::crefStripFirstIdent(&__a0))?;
                    outputCrefs = List::map1(outputCrefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::joinCrefsR(__a0, &__a1), lhsCref.clone())?;
                    expLst = List::map(outputCrefs.clone(), &Expression::crefExp)?;
                    Ok(listAppend(expLst.clone(), expsIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp, .. } => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = Expression::getComplexContents(metamodelica::AsArg::as_arg(&exp));
                    Ok(listAppend(expLst.clone(), expsIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = getStatementLHS(stmt.clone(), metamodelica::nil())?;
                    Ok(listAppend(expLst.clone(), expsIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(lhs)
}

fn getStatementsOutputs(
    mut statements: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut lhs_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut lhs_set: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::Exp>>>),
        i32,
        i32,
        (
            HashSetExp::FuncHashCref,
            HashSetExp::FuncCrefEqual,
            HashSetExp::FuncCrefStr,
        ),
    );
    lhs_expl = List::fold1(statements, &getStatementLHSScalar, funcTree, metamodelica::nil())?;
    lhs_set = HashSetExp::emptyHashSetSized(Util::nextPrime(((lhs_expl).len() as i32)));
    lhs_set = List::fold(
        &lhs_expl,
        &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1),
        lhs_set,
    )?;
    outputs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut e in (BaseHashSet::hashSetList(&lhs_set)?).into_iter().cloned() {
            let __x = Expression::expCref(&(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outputs)
}

fn getDAEelseStatemntLsts<'__b>(
    mut elseIn: &'__b metamodelica::Ref<DAE::Else>,
    mut stmtLstsIn: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>,
) -> metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    '__tco: loop {
        match &**elseIn {
            DAE::Else::ELSEIF {
                statementLst: stmts,
                else_: else1,
                ..
            } => {
                let mut stmtsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
                stmtsLst = metamodelica::cons(stmts.clone(), stmtLstsIn);
                {
                    (elseIn, stmtLstsIn) = (else1, stmtsLst);
                    continue '__tco;
                }
            }
            DAE::Else::ELSE { statementLst: stmts } => {
                let mut stmtsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
                return metamodelica::cons(stmts.clone(), stmtLstsIn);
            }
            _ => return stmtLstsIn,
        }
    }
}

fn evaluateConstantFunctionCall(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut eqIdx: i32,
    mut recursionLimit: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    i32,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outLhs: metamodelica::Ref<DAE::Exp>;
    let mut outFuncs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outEqIdx: i32;
    let mut addedStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let (__pa0, (__pa1, __pa2, __pa3, __pa4)) = Expression::traverseExpTopDown(
        exp,
        &({
            let __pe_b2 = recursionLimit;
            move |__pe_a0, __pe_a1| Ok(evaluateConstantFunction_traverser(&__pe_a0, &__pe_a1, __pe_b2.clone()))
        }),
        (lhs, funcs, eqIdx, metamodelica::nil()),
    )?;
    outExp = metamodelica::Own::own(__pa0);
    outLhs = metamodelica::Own::own(__pa1);
    outFuncs = metamodelica::Own::own(__pa2);
    outEqIdx = metamodelica::Own::own(__pa3);
    addedStmts = metamodelica::Own::own(__pa4);
    Ok((outExp, outLhs, outFuncs, outEqIdx, addedStmts))
}

fn evaluateConstantFunction_traverser(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inTpl: &(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    ),
    mut recursionLimit: i32,
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    );
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (&**inExp, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { .. }, (lhs, funcs, idx, stmtsIn)) => {
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut addEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut lhs = (*lhs).clone();
                    let mut funcs = (*funcs).clone();
                    let mut idx = (*idx).clone();
                    (rhs, lhs, addEqs, funcs, idx, _, _) = evaluateConstantFunction(inExp.clone(), lhs.clone(), funcs.clone(), idx.clone(), metamodelica::nil(), recursionLimit)?;
                    stmts = List::map(addEqs.clone(), &equationToStmt)?;
                    Ok((rhs.clone(), true, (lhs.clone(), funcs.clone(), idx.clone(), listAppend(stmts.clone(), stmtsIn.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: rhs, .. }, _) => {
                    let mut tpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<AvlTreePathFunction::Tree>, i32, metamodelica::List<metamodelica::Ref<DAE::Statement>>);
                    let mut rhs = (*rhs).clone();
                    (rhs, _, tpl) = evaluateConstantFunction_traverser(metamodelica::AsArg::as_arg(&rhs), inTpl, recursionLimit);
                    Ok((rhs.clone(), true, tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), false, inTpl.clone()))
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

fn equationToStmt(mut eqIn: metamodelica::Ref<BackendDAE::Equation>) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut stmtOut: metamodelica::Ref<DAE::Statement>;
    stmtOut = 'mc: {
        let __mc_input = &*eqIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, source, .. } => {
                    let mut typ: metamodelica::Ref<DAE::Type>;
                    typ = expType(lhs.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: typ.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("equationToStmt failed for: ")); __mm_s.push_str(&*BackendDump::dumpEqnsStr(list![eqIn.clone()])?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(stmtOut)
}

fn expType(mut eIn: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut tOut: metamodelica::Ref<DAE::Type>;
    tOut = (match &*eIn {
        DAE::Exp::CREF { ty: t, .. } => t.clone(),
        _ => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("expType failed for: "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(eIn)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            return Err("fail");
        }
    });
    Ok(tOut)
}

fn getScalarsForComplexVar(
    mut inElem: &metamodelica::Ref<DAE::Element>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut crefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crefsOut = 'mc: {
        let __mc_input = &**inElem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, .. } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
                    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut names: metamodelica::List<ArcStr>;
                    names = List::map(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::typeVarIdent(&__a0)) })?;
                    types = List::map(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::varType(&__a0)) })?;
                    crefs = List::map1(names.clone(), &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::appendStringCref(__a0, &__a1), cref.clone())?;
                    crefs = setTypesForScalarCrefs(crefs.clone(), types.clone())?;
                    crefLst = List::map1(crefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| ComponentReference::expandCref(&__a0, __a1), true)?;
                    crefs = List::flatten(crefLst.clone())?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_REAL { .. }, dims, .. } => {
                    let mut subslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    subslst = expandDimension(metamodelica::AsArg::as_arg(&dims), metamodelica::nil())?;
                    crefs = List::map1r(subslst.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>| ComponentReference::subscriptCref(&__a0, __a1), cref.clone())?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_INTEGER { .. }, dims, .. } => {
                    let mut subslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    subslst = expandDimension(metamodelica::AsArg::as_arg(&dims), metamodelica::nil())?;
                    crefs = List::map1r(subslst.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>| ComponentReference::subscriptCref(&__a0, __a1), cref.clone())?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: dimensions2 }, dims: dimensions }, .. } => {
                    let mut subslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    let mut subslst1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    let mut subslst2: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    subslst1 = expandDimension(metamodelica::AsArg::as_arg(&dimensions), metamodelica::nil())?;
                    subslst2 = expandDimension(metamodelica::AsArg::as_arg(&dimensions2), metamodelica::nil())?;
                    subslst = metamodelica::nil();
                    for mut subs in &*subslst1 {
                        for mut subs2 in &*subslst2 {
                            subslst = metamodelica::cons(listAppend(subs.clone(), subs2.clone()), subslst.clone());
                        }
                    }
                    crefs = List::map1r(subslst.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>| ComponentReference::subscriptCref(&__a0, __a1), cref.clone())?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_ARRAY { dims: dimensions, .. }, .. } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the array cref before\n")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!("\n"))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    crefs = ComponentReference::expandArrayCref(metamodelica::AsArg::as_arg(&cref), dimensions.clone())?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update getScalarsForComplexVar for enumerations: the enum cref is :")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!("\n"))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_TUPLE { .. }, .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update getScalarsForComplexVar for tuple types: the tupl cref is :\n")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!("\n"))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    crefsOut
}

fn expandDimension<'__b>(
    mut dims: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut subsIn: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>> {
    '__tco: loop {
        ({
            let mut subFold: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> =
                metamodelica::nil();
            ::match_deref::match_deref! { match dims {
                Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest } => {
                    let mut size: i32;
                    let mut range: metamodelica::List<i32>;
                    let mut sub: metamodelica::Ref<DAE::Subscript> = metamodelica::Ref::new(DAE::Subscript::WHOLEDIM);
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut subsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    let mut subsLst1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    size = Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
                    range = List::intRange(size);
                    subs = List::map(range, &fnptr!(Expression::intSubscript, i32))?;
                    subsLst = List::map(subs, &fnptr!(List::create, _))?;
                    for mut sub in &*subsIn {
                        let mut sub = sub.clone();
                        subsLst1 = List::map1r(subsLst.clone(), &fnptr!(listAppend, metamodelica::List<metamodelica::Ref<DAE::Subscript>>, metamodelica::List<metamodelica::Ref<DAE::Subscript>>), sub)?;
                        subFold = listAppend(subFold, subsLst1);
                    }
                    if (subsIn).is_empty() {
                        subFold = subsLst;
                    }
                    { (dims, subsIn) = (rest, subFold); continue '__tco; }
                },
                Deref @ metamodelica::ListNode::Nil => {
                    return Ok(subsIn)
                },
                _ => return Err("match: no arm matched"),
            } }
        })
    }
}

fn subsLstString(mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(subs, &move |__a0: metamodelica::Ref<DAE::Subscript>| {
                ExpressionDump::subscriptString(&__a0)
            })?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

fn isNotComplexVar(mut inElem: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = &**inElem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, .. } => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, dims, .. } => {
                    let mut dimints: metamodelica::List<i32>;
                    dimints = List::map(dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionSize(&__a0))?;
                    let true = ((dimints).head().cloned()? != 0) else { return Err("pattern mismatch") };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, dims, .. } => {
                    let mut dimints: metamodelica::List<i32>;
                    dimints = List::map(dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionSize(&__a0))?;
                    let true = ((dimints).head().cloned()? != 0) else { return Err("pattern mismatch") };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. } => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    b
}

fn setTypesForScalarCrefs(
    mut allCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crefsOut = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        let __thr_src0 = allCrefs;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = types;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(cr), Some(ty)) => {
                    let __x = ComponentReference::crefSetLastType(&(cr.clone()), &(ty.clone()))?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(crefsOut)
}

pub(crate) fn getRecordScalars(
    mut crefIn: &metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut crefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    match '__try0: {
        crefsOut = unwrap_break_err!(ComponentReference::expandCref(crefIn, true), '__try0);
        Ok::<_, &'static str>((crefsOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            crefsOut = __try0_o0;
        }
        Err(_) => {
            crefsOut = metamodelica::nil();
        }
    }
    crefsOut
}

fn getScalarExpSize(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut size: i32;
    size = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::TUPLE { PR: exps @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
            let mut exps_len: i32;
            exps_len = ({
        let mut __acc: i32 = 0;
        for mut exp in (exps.clone()).into_iter().cloned() {
            if !(Expression::isNotWild(&(exp.clone()))) { continue; }
            let __x = 1;
            __acc += __x;
        }
        __acc
    });
            size = ({
        let mut __acc: i32 = 0;
        for mut exp in (exps.clone()).into_iter().cloned() {
            let __x = getScalarExpSize(&(exp.clone()))?;
            __acc += __x;
        }
        __acc
    });
            std::cmp::max(size, exps_len)
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { varLst: vl @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. } => {
            ({
        let mut __acc: i32 = 0;
        for mut v in (vl.clone()).into_iter().cloned() {
            let __x = getScalarVarSize(&(v.clone()))?;
            __acc += __x;
        }
        __acc
    })
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            size = if (ComponentReference::isArrayElement(cref)) {(((ComponentReference::expandCref(cref, true)?)).len() as i32)} else {1};
            size
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst: vl @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. }, .. } => {
            ({
        let mut __acc: i32 = 0;
        for mut v in (vl.clone()).into_iter().cloned() {
            let __x = getScalarVarSize(&(v.clone()))?;
            __acc += __x;
        }
        __acc
    })
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_TUPLE { types: tyl @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. }, .. } => {
            let mut vl: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            size = 0;
            for mut ty in &*tyl.clone() {
                vl = getVarLstFromType(metamodelica::AsArg::as_arg(&ty));
                if !((vl).is_empty()) {
                    size = size + ({
        let mut __acc: i32 = 0;
        for mut v in (vl).into_iter().cloned() {
            let __x = getScalarVarSize(&(v.clone()))?;
            __acc += __x;
        }
        __acc
    });
                }
            }
            size
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

fn getVarLstFromType(mut tyIn: &metamodelica::Ref<DAE::Type>) -> metamodelica::List<metamodelica::Ref<DAE::Var>> {
    let mut varsOut: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    varsOut = (::match_deref::match_deref! { match tyIn {
        Deref @ DAE::Type::T_TUPLE { types: tyLst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => {
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
        for mut ty in (tyLst.clone()).into_iter().cloned() {
            let __x = getVarLstFromType(&(ty.clone()));
            __acc = __x.append(&__acc);
        }
        __acc
    })
        },
        Deref @ DAE::Type::T_COMPLEX { varLst, .. } => {
            varLst.clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst, .. } => {
            varLst.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    varsOut
}

fn getScalarVarSize(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<i32> {
    let mut size: i32;
    size = (::match_deref::match_deref! { match inVar {
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_COMPLEX { varLst: vl @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. } => {
            ({
        let mut __acc: i32 = 0;
        for mut v in (vl.clone()).into_iter().cloned() {
            let __x = getScalarVarSize(&(v.clone()))?;
            __acc += __x;
        }
        __acc
    })
        },
        Deref @ DAE::Var { ty: ty @ Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. } => {
            ({
        let mut __acc: i32 = 1;
        for mut sz in (DAEUtil::expTypeArrayDimensions(metamodelica::AsArg::as_arg(&ty))?).into_iter().cloned() {
            let __x = sz.clone();
            __acc *= __x;
        }
        __acc
    })
        },
        _ => {
            1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

// =============================================================================
// predict if statements
//
// =============================================================================
fn evaluateFunctions_updateStatementEmptyRepl(
    mut algsIn: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inFuncTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inIndex: i32,
    mut recursionLimit: i32,
) -> Result<(
    (
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
        BackendVarTransform::VariableReplacements,
    ),
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    i32,
)> {
    let mut mapTplOut: (
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
        BackendVarTransform::VariableReplacements,
    );
    let mut outFuncTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outIndex: i32;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut algsOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    repl = BackendVarTransform::emptyReplacements();
    (algsOut, outFuncTree, repl, outIndex) = evaluateFunctions_updateStatement(
        algsIn,
        inFuncTree,
        repl,
        inIndex,
        &(metamodelica::nil()),
        recursionLimit,
    )?;
    mapTplOut = (algsOut, repl);
    Ok((mapTplOut, outFuncTree, outIndex))
}

fn evaluateFunctions_updateAllStatements(
    mut stmtsIn: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut elseStmtsLstIn: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>,
    mut replIn: &BackendVarTransform::VariableReplacements,
    mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut idx: i32,
    mut recursionLimit: i32,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    i32,
)> {
    let mut stmtsLstOut: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
    let mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree> = funcTree;
    let mut idx: i32 = idx;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    repl = getOnlyConstantReplacements(replIn)?;
    (stmts, funcTree, _, idx) =
        evaluateFunctions_updateStatement(stmtsIn, funcTree, repl, idx, &(metamodelica::nil()), recursionLimit)?;
    stmtsLstOut = list![stmts];
    for mut elseStmts in &**elseStmtsLstIn {
        repl = getOnlyConstantReplacements(replIn)?;
        (stmts, funcTree, _, idx) = evaluateFunctions_updateStatement(
            elseStmts.clone(),
            funcTree,
            repl,
            idx,
            &(metamodelica::nil()),
            recursionLimit,
        )?;
        stmtsLstOut = metamodelica::cons(stmts, stmtsLstOut);
    }
    stmtsLstOut = stmtsLstOut.reverse();
    Ok((stmtsLstOut, funcTree, idx))
}

fn predictIfOutput(
    mut stmtIn: metamodelica::Ref<DAE::Statement>,
    mut infoIn: FuncInfo,
    mut recursionLimit: i32,
) -> (
    (
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    ),
    FuncInfo,
) {
    let mut stmtsOut: (
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    );
    let mut infoOut: FuncInfo;
    (stmtsOut, infoOut) = 'mc: {
        let __mc_input = (&*stmtIn, &infoIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { statementLst: stmts1, else_, .. }, FuncInfo { repl: replIn, funcTree, idx }) => {
                    let mut constantOutputs: metamodelica::List<i32>;
                    let mut replLst: metamodelica::List<BackendVarTransform::VariableReplacements>;
                    let mut stmtNew: metamodelica::Ref<DAE::Statement>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut allLHS: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut addStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
                    let mut elseStmtsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
                    let mut funcTree = (*funcTree).clone();
                    let mut idx = (*idx).clone();
                    elseStmtsLst = getDAEelseStatemntLsts(metamodelica::AsArg::as_arg(&else_), metamodelica::nil());
                    elseStmtsLst = elseStmtsLst.clone().reverse();
                    (stmtsLst, funcTree, idx) = evaluateFunctions_updateAllStatements(stmts1.clone(), &elseStmtsLst, metamodelica::AsArg::as_arg(&replIn), funcTree.clone(), idx.clone(), recursionLimit)?;
                    replLst = List::map(stmtsLst.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Statement>>| collectReplacements(&__a0))?;
                    expLst = List::fold(&(List::flatten(stmtsLst.clone())?), &getStatementLHS, metamodelica::nil())?;
                    expLst = List::unique(&expLst);
                    allLHS = expLst.clone().reverse();
                    expLstLst = List::map1(replLst.clone(), &move |__a0: BackendVarTransform::VariableReplacements, __a1: metamodelica::List<metamodelica::Ref<DAE::Exp>>| replaceExps(__a0, &__a1), allLHS.clone())?;
                    constantOutputs = compareConstantExps(expLstLst.clone())?;
                    outExps = List::map1(constantOutputs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), allLHS.clone())?;
                    let true = (List::all(&outExps, &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isCref(&__a0)) })?) else { return Err("pattern mismatch") };
                    expLst = List::map1(constantOutputs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), (expLstLst).head().cloned()?)?;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--> the predicted const outputs:\n")); __mm_s.push_str(&*stringDelimitList(List::map(outExps.clone(), &ExpressionBasics::printExpStr)?, literal!("\n"))); ArcStr::from(__mm_s) });
                    }
                    addStmts = makeAssignmentMap(outExps.clone(), expLst.clone())?;
                    stmtNew = updateStatementsInIfStmt(&stmtsLst, &stmtIn)?;
                    Ok(((list![stmtNew.clone()], addStmts.clone()), FuncInfo { repl: replIn.clone(), funcTree: funcTree.clone(), idx: idx.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(((list![stmtIn.clone()], metamodelica::nil()), infoIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (stmtsOut, infoOut)
}

fn collectReplacements(
    mut stmtsIn: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut repl: BackendVarTransform::VariableReplacements;
    repl = BackendVarTransform::emptyReplacements();
    replOut = collectReplacements1(stmtsIn, &repl)?;
    Ok(replOut)
}

fn collectReplacements1(
    mut stmtsIn: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut replIn: &BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = 'mc: {
        let __mc_input = &**stmtsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(replIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: lhs, exp: rhs, .. }, tail: rest } => {
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut rhs = (*rhs).clone();
                    (rhs, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&rhs), replIn, None);
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    let true = (Expression::isConst(rhs.clone())?) else { return Err("pattern mismatch") };
                    cref = Expression::expCref(metamodelica::AsArg::as_arg(&lhs))?;
                    repl = BackendVarTransform::addReplacement(replIn.clone(), cref.clone(), rhs.clone(), None)?;
                    repl = collectReplacements1(metamodelica::AsArg::as_arg(&rest), &repl)?;
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: lhsLst, exp: rhs, .. }, tail: rest } => {
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut constCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut rhsLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut constExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut rhs = (*rhs).clone();
                    (rhs, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&rhs), replIn, None);
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    rhsLst = Expression::getComplexContents(metamodelica::AsArg::as_arg(&rhs));
                    crefs = List::map(lhsLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    (constExps, constCrefs) = List::filterOnTrueSync(&rhsLst, &Expression::isConst, crefs.clone())?;
                    (_, varCrefs) = List::filterOnTrueSync(&rhsLst, &Expression::isNotConst, crefs.clone())?;
                    repl = BackendVarTransform::addReplacements(replIn.clone(), &constCrefs, &constExps, None)?;
                    BackendVarTransform::removeReplacements(&repl, &varCrefs)?;
                    repl = collectReplacements1(metamodelica::AsArg::as_arg(&rest), &repl)?;
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: stmt, tail: rest } => {
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut lhsLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    lhsLst = getStatementLHS(stmt.clone(), metamodelica::nil())?;
                    crefs = List::map(lhsLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    BackendVarTransform::removeReplacements(replIn, &crefs)?;
                    repl = collectReplacements1(metamodelica::AsArg::as_arg(&rest), replIn)?;
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("collectReplacements failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(replOut)
}

fn getOnlyConstantReplacements(
    mut replIn: &BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    (crefs, exps) = BackendVarTransform::getAllReplacements(replIn)?;
    (exps, crefs) = List::filterOnTrueSync(&exps, &Expression::isConst, crefs)?;
    repl = BackendVarTransform::emptyReplacements();
    replOut = BackendVarTransform::addReplacements(repl, &crefs, &exps, None)?;
    Ok(replOut)
}

fn updateStatementsInIfStmt(
    mut stmtLstIn: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>,
    mut origIf: &metamodelica::Ref<DAE::Statement>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut ifStmtOut: metamodelica::Ref<DAE::Statement>;
    ifStmtOut = (::match_deref::match_deref! { match (stmtLstIn, origIf) {
        (Deref @ metamodelica::ListNode::Cons { head: stmts, tail: rest }, Deref @ DAE::Statement::STMT_IF { exp, else_: els, source, .. }) => {
            let mut els = (*els).clone();
            els = updateStatementsInElse(rest, metamodelica::AsArg::as_arg(&els))?;
            metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: stmts.clone(), else_: els.clone(), source: source.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(ifStmtOut)
}

fn updateStatementsInElse(
    mut stmtLstIn: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>,
    mut origElse: &metamodelica::Ref<DAE::Else>,
) -> Result<metamodelica::Ref<DAE::Else>> {
    let mut elseOut: metamodelica::Ref<DAE::Else>;
    elseOut = (::match_deref::match_deref! { match (stmtLstIn, origElse) {
        (Deref @ metamodelica::ListNode::Cons { head: stmts, tail: rest }, Deref @ DAE::Else::ELSEIF { exp, else_: els, .. }) => {
            let mut els = (*els).clone();
            els = updateStatementsInElse(rest, metamodelica::AsArg::as_arg(&els))?;
            metamodelica::Ref::new(DAE::Else::ELSEIF { exp: exp.clone(), statementLst: stmts.clone(), else_: els.clone() })
        },
        (Deref @ metamodelica::ListNode::Cons { head: stmts, tail: _ }, Deref @ DAE::Else::ELSE { .. }) => {
            metamodelica::Ref::new(DAE::Else::ELSE { statementLst: stmts.clone() })
        },
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ DAE::Else::NOELSE { .. }) => {
            openmodelica_frontend_types::DAE::Else::interned_NOELSE()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(elseOut)
}

fn compareConstantExps(
    mut expLstLstIn: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<metamodelica::List<i32>> {
    let mut posLstOut: metamodelica::List<i32> = metamodelica::nil();
    for mut i in 1..=(((expLstLstIn).head().cloned()?).len() as i32) {
        posLstOut = compareConstantExps2(i, expLstLstIn.clone(), posLstOut)?;
    }
    Ok(posLstOut)
}

fn compareConstantExps2(
    mut idx: i32,
    mut expLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut pos: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut pos: metamodelica::List<i32> = pos;
    let mut b1: bool;
    let mut b2: bool;
    let mut firstExp: metamodelica::Ref<DAE::Exp>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    expLst = List::map1(expLstLst, &listGet, idx)?;
    b1 = List::all(&expLst, &Expression::isConst)?;
    if b1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expLst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        firstExp = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        b2 = List::all(
            &rest,
            &({
                let __pe_b1 = firstExp;
                move |__pe_a0| ExpressionBasics::expEqual(&__pe_a0, __pe_b1.clone())
            }),
        )?;
        if b2 {
            pos = metamodelica::cons(idx, pos);
        }
    }
    Ok(pos)
}

fn makeAssignmentMap(
    mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    stmts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
        let __thr_src0 = lhs;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = rhs;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = makeAssignment(e1.clone(), e2.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(stmts)
}

fn makeAssignment(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut stmtOut: metamodelica::Ref<DAE::Statement>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Expression::r#typeof(rhs.clone())?;
    stmtOut = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
        type_: ty,
        exp1: lhs,
        exp: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    Ok(stmtOut)
}

// =============================================================================
// redeclare the varKinds (maybe some state candidates are vanished)
//
// =============================================================================
fn updateVarKinds(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    systs = List::map1(
        systs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: metamodelica::Ref<BackendDAE::Shared>| {
            updateVarKinds_eqSys(__a0, &__a1)
        },
        shared.clone(),
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: systs,
        shared: shared,
    });
    Ok(outDAE)
}

fn updateVarKinds_eqSys(
    mut sysIn: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut sysOut: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut ssVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut initEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut derVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut ssVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut derVarsInit: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let __arc2 = sysIn.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        ..
    } = &*__arc2;
    vars = metamodelica::Own::own(__pa0);
    eqs = metamodelica::Own::own(__pa1);
    varLst = BackendVariable::varList(&vars)?;
    initEqs = BackendEquation::getInitialEqnsFromShared(shared);
    states = List::filterOnTrue(
        varLst.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isStateorStateDerVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    (_, derVarsInit) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        initEqs,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                findDerVarCrefs,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )> + 'static,
                >),
            metamodelica::nil(),
        ),
    )?;
    (_, derVars) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        eqs,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                findDerVarCrefs,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )> + 'static,
                >),
            derVarsInit,
        ),
    )?;
    ssVarLst = List::filterOnTrue(
        varLst,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(varSSisPreferOrHigher(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    ssVars = List::map(
        ssVarLst,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        },
    )?;
    derVars = List::unique(&(listAppend(derVars, ssVars)));
    (vars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        vars,
        (std::sync::Arc::new(fnptr!(
            setVarKindForStates,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )> + 'static,
            >),
        derVars,
    )?;
    sysOut = BackendDAEUtil::setEqSystVars(sysIn, vars);
    Ok(sysOut)
}

fn varSSisPreferOrHigher(mut varIn: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut ssOut: bool;
    let mut i: i32;
    let mut ss: DAE::StateSelect;
    ss = BackendVariable::varStateSelect(varIn);
    i = BackendVariable::stateSelectToInteger(ss);
    ssOut = intGe(i, 2);
    ssOut
}

fn setVarKindForStates(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outVar, outCrefs) = 'mc: {
        let __mc_input = (inVar.clone(), inCrefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (varOld @ Deref @ BackendDAE::Var { varName: cr1, varKind: BackendDAE::VarKind::STATE { .. }, .. }, derVars) => {
                    let mut isState: bool;
                    let mut varNew: metamodelica::Ref<BackendDAE::Var>;
                    isState = List::isMemberOnTrue(cr1.clone(), metamodelica::AsArg::as_arg(&derVars), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                    varNew = if (!(isState)) {BackendVariable::setVarKind(varOld.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?} else {varOld.clone()};
                    Ok((varNew.clone(), derVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inCrefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outCrefs)
}

fn findDerVarCrefs(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outCrefs = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            metamodelica::cons(cr.clone(), inCrefs)
        },
        _ => {
            inCrefs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, outCrefs)
}

// =============================================================================
// convert tuple equations to several single equations
//
// =============================================================================
fn convertTupleEquations(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut addEqsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut eqOut: metamodelica::Ref<BackendDAE::Equation>;
    let mut addEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (eqOut, addEqsOut) = (::match_deref::match_deref! { match &(eqIn.clone()) {
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::TUPLE { PR: lhs }, right: Deref @ DAE::Exp::TUPLE { PR: rhs }, .. } => {
            let mut eq: metamodelica::Ref<BackendDAE::Equation>;
            let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut lhs = (*lhs).clone();
            let mut rhs = (*rhs).clone();
            lhs = List::mapFlat(metamodelica::AsArg::as_arg(&lhs), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::getComplexContents(&__a0)) })?;
            rhs = List::mapFlat(metamodelica::AsArg::as_arg(&rhs), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::getComplexContents(&__a0)) })?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
        let __thr_src0 = lhs.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = rhs.clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(lh), Some(rh)) => {
                    let __x = makeBackendEquation(lh.clone(), rh.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    })) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eq = metamodelica::Own::own(__pa0);
            eqs = metamodelica::Own::own(__pa1);
            (eq, listAppend(eqs, addEqsIn))
        },
        _ => {
            (eqIn, addEqsIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqOut, addEqsOut))
}

fn makeBackendEquation(
    mut ls: metamodelica::Ref<DAE::Exp>,
    mut rs: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<BackendDAE::Equation> {
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    eq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
        exp: rs,
        scalar: ls,
        source: DAE::emptyElementSource().clone(),
        attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
    });
    eq
}
