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
// two functions "addLabelToExpList" and "addLabelToExpListForSubstitution", were commented in old version
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

use crate::BackendVarTransform;
use crate::Differentiate;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) const LABELNAME: &'static str = "label";

pub(crate) fn buildLabels(
    mut inEquationLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut inModelInfo: SimCode::ModelInfo,
    mut reduceList: &metamodelica::List<i32>,
    mut inArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    SimCode::ModelInfo,
)> {
    let mut outEquationLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut outModelInfo: SimCode::ModelInfo;
    (outEquationLst, outModelInfo) = 'mc: {
        let __mc_input = (inEquationLst, inModelInfo, &**inArgs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqns, modelInfo @ SimCode::ModelInfo { varInfo: varInfo @ SimCode::VarInfo { .. }, .. }, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: exp_list }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }) => {
                    let mut eqns_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels_1: metamodelica::List<ArcStr>;
                    let mut labels_2: metamodelica::List<ArcStr>;
                    let mut p: i32;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut modelInfo = (*modelInfo).clone();
                    let mut varInfo = (*varInfo).clone();
                    repl = meanValueReplacements(&modelInfo.vars, metamodelica::AsArg::as_arg(&exp_list))?;
                    let (__pa0, __pa1, (_, __pa2), __pa3) = addLabelToEquations(metamodelica::AsArg::as_arg(&eqns), modelInfo.vars.clone(), (0, varInfo.numParams.clone()), reduceList, &repl)?;
                    eqns_1 = metamodelica::Own::own(__pa0);
                    vars_1 = metamodelica::Own::own(__pa1);
                    p = metamodelica::Own::own(__pa2);
                    labels_1 = metamodelica::Own::own(__pa3);
                    labels_2 = listAppend(modelInfo.labels.clone(), labels_1.clone());
                    if varInfo.numParams.clone() != p {
                        varInfo.numParams = p;
                        modelInfo.varInfo = varInfo.clone();
                    }
                    modelInfo.labels = labels_2.clone();
                    if !({ let __refeq_sl = &(modelInfo.vars.clone()); let __refeq_sr = &(vars_1.clone()); metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stateVars), &(__refeq_sr.stateVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.derivativeVars), &(__refeq_sr.derivativeVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.algVars), &(__refeq_sr.algVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.discreteAlgVars), &(__refeq_sr.discreteAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intAlgVars), &(__refeq_sr.intAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolAlgVars), &(__refeq_sr.boolAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.inputVars), &(__refeq_sr.inputVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.outputVars), &(__refeq_sr.outputVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.aliasVars), &(__refeq_sr.aliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intAliasVars), &(__refeq_sr.intAliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolAliasVars), &(__refeq_sr.boolAliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.paramVars), &(__refeq_sr.paramVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intParamVars), &(__refeq_sr.intParamVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolParamVars), &(__refeq_sr.boolParamVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringAlgVars), &(__refeq_sr.stringAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringParamVars), &(__refeq_sr.stringParamVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringAliasVars), &(__refeq_sr.stringAliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.extObjVars), &(__refeq_sr.extObjVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.constVars), &(__refeq_sr.constVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intConstVars), &(__refeq_sr.intConstVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolConstVars), &(__refeq_sr.boolConstVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringConstVars), &(__refeq_sr.stringConstVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.jacobianVars), &(__refeq_sr.jacobianVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.seedVars), &(__refeq_sr.seedVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.realOptimizeConstraintsVars), &(__refeq_sr.realOptimizeConstraintsVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.realOptimizeFinalConstraintsVars), &(__refeq_sr.realOptimizeFinalConstraintsVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.sensitivityVars), &(__refeq_sr.sensitivityVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.dataReconSetcVars), &(__refeq_sr.dataReconSetcVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.dataReconinputVars), &(__refeq_sr.dataReconinputVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.dataReconSetBVars), &(__refeq_sr.dataReconSetBVars)) }) {
                        modelInfo.vars = vars_1.clone();
                    }
                    Ok((eqns_1.clone(), modelInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqns, modelInfo @ SimCode::ModelInfo { varInfo: varInfo @ SimCode::VarInfo { .. }, .. }, _) => {
                    let mut eqns_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels_1: metamodelica::List<ArcStr>;
                    let mut labels_2: metamodelica::List<ArcStr>;
                    let mut p: i32;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut modelInfo = (*modelInfo).clone();
                    let mut varInfo = (*varInfo).clone();
                    repl = BackendVarTransform::emptyReplacements();
                    let (__pa0, __pa1, (_, __pa2), __pa3) = addLabelToEquations(metamodelica::AsArg::as_arg(&eqns), modelInfo.vars.clone(), (0, varInfo.numParams.clone()), reduceList, &repl)?;
                    eqns_1 = metamodelica::Own::own(__pa0);
                    vars_1 = metamodelica::Own::own(__pa1);
                    p = metamodelica::Own::own(__pa2);
                    labels_1 = metamodelica::Own::own(__pa3);
                    labels_2 = listAppend(modelInfo.labels.clone(), labels_1.clone());
                    if varInfo.numParams.clone() != p {
                        varInfo.numParams = p;
                        modelInfo.varInfo = varInfo.clone();
                    }
                    modelInfo.labels = labels_2.clone();
                    if !({ let __refeq_sl = &(modelInfo.vars.clone()); let __refeq_sr = &(vars_1.clone()); metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stateVars), &(__refeq_sr.stateVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.derivativeVars), &(__refeq_sr.derivativeVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.algVars), &(__refeq_sr.algVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.discreteAlgVars), &(__refeq_sr.discreteAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intAlgVars), &(__refeq_sr.intAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolAlgVars), &(__refeq_sr.boolAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.inputVars), &(__refeq_sr.inputVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.outputVars), &(__refeq_sr.outputVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.aliasVars), &(__refeq_sr.aliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intAliasVars), &(__refeq_sr.intAliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolAliasVars), &(__refeq_sr.boolAliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.paramVars), &(__refeq_sr.paramVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intParamVars), &(__refeq_sr.intParamVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolParamVars), &(__refeq_sr.boolParamVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringAlgVars), &(__refeq_sr.stringAlgVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringParamVars), &(__refeq_sr.stringParamVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringAliasVars), &(__refeq_sr.stringAliasVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.extObjVars), &(__refeq_sr.extObjVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.constVars), &(__refeq_sr.constVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.intConstVars), &(__refeq_sr.intConstVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.boolConstVars), &(__refeq_sr.boolConstVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.stringConstVars), &(__refeq_sr.stringConstVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.jacobianVars), &(__refeq_sr.jacobianVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.seedVars), &(__refeq_sr.seedVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.realOptimizeConstraintsVars), &(__refeq_sr.realOptimizeConstraintsVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.realOptimizeFinalConstraintsVars), &(__refeq_sr.realOptimizeFinalConstraintsVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.sensitivityVars), &(__refeq_sr.sensitivityVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.dataReconSetcVars), &(__refeq_sr.dataReconSetcVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.dataReconinputVars), &(__refeq_sr.dataReconinputVars)) && metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.dataReconSetBVars), &(__refeq_sr.dataReconSetBVars)) }) {
                        modelInfo.vars = vars_1.clone();
                    }
                    Ok((eqns_1.clone(), modelInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquationLst, outModelInfo))
}

pub(crate) fn reduceTerms(
    mut inEquationLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut inModelInfo: SimCode::ModelInfo,
    mut inArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    SimCode::ModelInfo,
)> {
    let mut outEquationLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut outModelInfo: SimCode::ModelInfo;
    (outEquationLst, outModelInfo) = ({
        let mut reduceListStr: ArcStr = literal!("");
        (match &**inArgs {
            Absyn::FunctionArgs::FUNCTIONARGS {
                args: inExpArgList,
                argNames: inNamedArgList,
            } => {
                let mut eqns = inEquationLst;
                let mut modelInfo = inModelInfo;
                let mut reduceList: metamodelica::List<i32>;
                let mut outExpList: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                let mut modelInfo_1: SimCode::ModelInfo;
                (_, outExpList) = AbsynUtil::getNamedFuncArgNamesAndValues(inNamedArgList.clone());
                reduceListStr = System::stringReplace(
                    ExpressionBasics::printExpStr(Expression::fromAbsynExp((outExpList).get(1)?)?)?,
                    literal!("\""),
                    literal!(""),
                )?;
                reduceList = StringDelimit2Int(reduceListStr, literal!(","));
                (eqns, modelInfo_1) = buildLabels(
                    eqns,
                    modelInfo,
                    &reduceList,
                    &(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
                        args: inExpArgList.clone(),
                        argNames: inNamedArgList.clone(),
                    })),
                )?;
                (eqns, modelInfo_1)
            }
            _ => return Err("match: no arm matched"),
        })
    });
    Ok((outEquationLst, outModelInfo))
}

fn meanValueReplacements(
    mut inVarLst: &SimCodeVar::SimVars,
    mut exp_list: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outVarRepl: BackendVarTransform::VariableReplacements;
    outVarRepl = (match inVarLst.clone() {
        SimCodeVar::SimVars {
            algVars: ref alg,
            intAlgVars: ref intAlg,
            boolAlgVars: ref boolAlg,
            stateVars: ref states,
            ..
        } => {
            let mut listVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
            let mut listVars1: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
            let mut listVars2: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
            let mut repl: BackendVarTransform::VariableReplacements;
            repl = BackendVarTransform::emptyReplacements();
            listVars1 = listAppend(alg.clone(), intAlg.clone());
            listVars2 = listAppend(listVars1, boolAlg.clone());
            listVars = listAppend(listVars2, states.clone());
            repl = meanValueReplacements2(repl, &listVars, exp_list)?;
            repl
        }
    });
    Ok(outVarRepl)
}

fn meanValueReplacements2(
    mut inVarRepl: BackendVarTransform::VariableReplacements,
    mut inVarList: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut inValuesList: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outVarRepl: BackendVarTransform::VariableReplacements;
    outVarRepl = 'mc: {
        let __mc_input = (inVarRepl, &**inVarList, &**inValuesList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::REAL { value }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut repl = (*repl).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), DAE::crefTime().clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: stringReal(value.clone())? }), None)?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("Add replacement for time\n"))?;
                    }
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeVar::SimVar { name, type_: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, tail: restVar }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::REAL { value }, tail: restVal }) => {
                    let mut repl = (*repl).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: stringReal(value.clone())? }), None)?;
                    repl = meanValueReplacements2(repl.clone(), metamodelica::AsArg::as_arg(&restVar), metamodelica::AsArg::as_arg(&restVal))?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&name))?); __mm_s.push_str(&*literal!(" by ")); __mm_s.push_str(&*value); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeVar::SimVar { name, type_: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, tail: restVar }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::INTEGER { value: value2 }, tail: restVal }) => {
                    let mut value: ArcStr;
                    let mut repl = (*repl).clone();
                    value = intString(value2.clone());
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: stringReal(value.clone())? }), None)?;
                    repl = meanValueReplacements2(repl.clone(), metamodelica::AsArg::as_arg(&restVar), metamodelica::AsArg::as_arg(&restVal))?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&name))?); __mm_s.push_str(&*literal!(" by ")); __mm_s.push_str(&*value); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeVar::SimVar { name, type_: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, tail: restVar }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp: Deref @ Absyn::Exp::REAL { value } }, tail: restVal }) => {
                    let mut repl = (*repl).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: stringReal(value.clone())? }) }), None)?;
                    repl = meanValueReplacements2(repl.clone(), metamodelica::AsArg::as_arg(&restVar), metamodelica::AsArg::as_arg(&restVal))?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&name))?); __mm_s.push_str(&*literal!(" by -")); __mm_s.push_str(&*value); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeVar::SimVar { name, type_: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, tail: restVar }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp: Deref @ Absyn::Exp::INTEGER { value: value2 } }, tail: restVal }) => {
                    let mut value: ArcStr;
                    let mut repl = (*repl).clone();
                    value = intString(value2.clone());
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: stringReal(value.clone())? }) }), None)?;
                    repl = meanValueReplacements2(repl.clone(), metamodelica::AsArg::as_arg(&restVar), metamodelica::AsArg::as_arg(&restVal))?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&name))?); __mm_s.push_str(&*literal!(" by -")); __mm_s.push_str(&*value); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("Add no replacement\n"))?;
                    }
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::ListNode::Cons { head: _, tail: restVar }, Deref @ metamodelica::ListNode::Cons { head: _, tail: restVal }) => {
                    let mut repl = (*repl).clone();
                    repl = meanValueReplacements2(repl.clone(), metamodelica::AsArg::as_arg(&restVar), metamodelica::AsArg::as_arg(&restVal))?;
                    Ok(repl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarRepl)
}

fn addLabelToEquations(
    mut inEquationLst1: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIndex: (i32, i32),
    mut reduceList: &metamodelica::List<i32>,
    mut inVarRepl: &BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
)> {
    let mut outEquationLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outEquationLst, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (&**inEquationLst1, inVarLst, inIndex);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, vars, idx) => {
                    Ok((metamodelica::nil(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: i, res_index: res_i, exp: e, source, eqAttr }, tail: es }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace residuals\n"))?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExp(e.clone(), vars.clone(), idx.clone(), true, reduceList, inVarRepl)?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_RESIDUAL { index: i.clone(), res_index: res_i.clone(), exp: e2.clone(), source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i, cref: cr, exp: e, source, eqAttr }, tail: es }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace simple assignments\n"))?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExp(e.clone(), vars.clone(), idx.clone(), true, reduceList, inVarRepl)?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i.clone(), cref: cr.clone(), exp: e2.clone(), source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: i, statements, eqAttr }, tail: es }, vars, idx) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut statements2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace algorithms\n"))?;
                    }
                    (statements2, vars_1, idx2, labels) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&statements), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_ALGORITHM { index: i.clone(), statements: statements2.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: i, partOfMixed: partOfLinear, tornSystem, vars: varsLin, beqs: b, simJac: A, residual, jacobianMatrix, sources: sourcelist, indexLinearSystem: idxLS, nUnknowns: nUnknownsLS, partOfJac }, alternativeTearing: None, eqAttr }, tail: es }, vars, idx) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut A2: metamodelica::List<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace linear equation systems\n"))?;
                    }
                    (A2, vars_1, idx2, labels) = addLabelToLinearEquationSystems(metamodelica::AsArg::as_arg(&A), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_LINEAR { lSystem: metamodelica::Ref::new(SimCode::LinearSystem { index: i.clone(), partOfMixed: partOfLinear.clone(), tornSystem: tornSystem.clone(), vars: varsLin.clone(), beqs: b.clone(), simJac: A2.clone(), residual: residual.clone(), jacobianMatrix: jacobianMatrix.clone(), sources: sourcelist.clone(), indexLinearSystem: idxLS.clone(), nUnknowns: nUnknownsLS.clone(), partOfJac: partOfJac.clone() }), alternativeTearing: None, eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: i, eqs: nl, crefs, indexNonLinearSystem: idxNLS, nUnknowns: nUnknownsNLS, jacobianMatrix, clockIndex, .. }, alternativeTearing: None, eqAttr }, tail: es }, vars, idx) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut nl_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace non-linear equation systems\n"))?;
                    }
                    (nl_1, vars_1, idx2, labels) = addLabelToEquations(metamodelica::AsArg::as_arg(&nl), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: metamodelica::Ref::new(SimCode::NonlinearSystem { index: i.clone(), eqs: nl_1.clone(), crefs: crefs.clone(), indexNonLinearSystem: idxNLS.clone(), nUnknowns: nUnknownsNLS.clone(), jacobianMatrix: jacobianMatrix.clone(), homotopySupport: false, mixedSystem: false, tornSystem: false, clockIndex: clockIndex.clone() }), alternativeTearing: None, eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_MIXED { index: i, cont, discVars, discEqs: disc, indexMixedSystem: indexSys, eqAttr }, tail: es }, vars, idx) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut cont_1: metamodelica::Ref<SimCode::SimEqSystem>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace mixed equation systems\n"))?;
                    }
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToEquations(&(list![cont.clone()]), vars.clone(), idx.clone(), reduceList, inVarRepl)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cont_1 = metamodelica::Own::own(__pa0);
                    vars_1 = metamodelica::Own::own(__pa1);
                    idx2 = metamodelica::Own::own(__pa2);
                    labels = metamodelica::Own::own(__pa3);
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_MIXED { index: i.clone(), cont: cont_1.clone(), discVars: discVars.clone(), discEqs: disc.clone(), indexMixedSystem: indexSys.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_WHEN { index: i, conditions, initialCall, whenStmtLst, elseWhen: None, source, eqAttr }, tail: es }, vars, idx) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace when equations without else statement\n"))?;
                    }
                    (es_1, vars_1, idx2, labels) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_WHEN { index: i.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), whenStmtLst: whenStmtLst.clone(), elseWhen: None, source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_WHEN { index: i, conditions, initialCall, whenStmtLst, elseWhen: Some(elsePart), source, eqAttr }, tail: es }, vars, idx) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut elsePart = (*elsePart).clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace when equations with else statement\n"))?;
                    }
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToEquations(&(list![elsePart.clone()]), vars.clone(), idx.clone(), reduceList, inVarRepl)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    elsePart = metamodelica::Own::own(__pa0);
                    vars_1 = metamodelica::Own::own(__pa1);
                    idx2 = metamodelica::Own::own(__pa2);
                    labels = metamodelica::Own::own(__pa3);
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(SimCode::SimEqSystem::SES_WHEN { index: i.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), whenStmtLst: whenStmtLst.clone(), elseWhen: Some(elsePart.clone()), source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: eq, tail: es }, vars, idx) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut idx2: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace unknown equations\n"))?;
                    }
                    (es_1, vars_1, idx2, labels) = addLabelToEquations(metamodelica::AsArg::as_arg(&es), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((metamodelica::cons(eq.clone(), es_1.clone()), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquationLst, outVarLst, outIndex, outStringList))
}

fn addLabelToAlgorithms(
    mut inStatements: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIndex: (i32, i32),
    mut reduceList: &metamodelica::List<i32>,
    mut inVarRepl: &BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
)> {
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outStatements, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (&**inStatements, inVarLst, inIndex);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace empty algorithm\n"))?;
                    }
                    Ok((metamodelica::nil(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { type_: ty, exp1: e1, exp: e, source }, tail: rest }, vars, idx) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace assignment algorithm\n"))?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExp(e.clone(), vars.clone(), idx.clone(), true, reduceList, inVarRepl)?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&rest), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: ty.clone(), exp1: e1.clone(), exp: e2.clone(), source: source.clone() }), rest2.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: stmtLst, else_, source }, tail: rest }, vars, idx) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtLst2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace if algorithm\n"))?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&stmtLst), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&rest), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: stmtLst2.clone(), else_: else_.clone(), source: source.clone() }), rest2.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_: ty, iterIsArray, iter, range: e, statementLst: stmtLst, source, sub_iters }, tail: rest }, vars, idx) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtLst2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace for algorithm\n"))?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&stmtLst), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&rest), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: ty.clone(), iterIsArray: iterIsArray.clone(), iter: iter.clone(), range: e.clone(), statementLst: stmtLst2.clone(), source: source.clone(), sub_iters: sub_iters.clone() }), rest2.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmtLst, source }, tail: rest }, vars, idx) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtLst2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace while algorithm\n"))?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&stmtLst), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&rest), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e.clone(), statementLst: stmtLst2.clone(), source: source.clone() }), rest2.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmtLst, elseWhen: None, source }, tail: rest }, vars, idx) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtLst2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace when algorithm without else statement\n"))?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&stmtLst), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&rest), vars_1.clone(), idx2, reduceList, inVarRepl)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmtLst2.clone(), elseWhen: None, source: source.clone() }), rest2.clone()), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmtLst, elseWhen: Some(elseWhen), source }, tail: rest }, vars, idx) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut rest2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmtLst2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut labels5: metamodelica::List<ArcStr>;
                    let mut elseWhen2: metamodelica::Ref<DAE::Statement>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace when algorithm with else statement\n"))?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&stmtLst), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToAlgorithms(&(list![elseWhen.clone()]), vars_1.clone(), idx2, reduceList, inVarRepl)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    elseWhen2 = metamodelica::Own::own(__pa0);
                    vars_2 = metamodelica::Own::own(__pa1);
                    idx3 = metamodelica::Own::own(__pa2);
                    labels2 = metamodelica::Own::own(__pa3);
                    (rest2, vars_3, idx4, labels3) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&rest), vars_2.clone(), idx3, reduceList, inVarRepl)?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmtLst2.clone(), elseWhen: Some(elseWhen2.clone()), source: source.clone() }), rest2.clone()), vars_3.clone(), idx4, labels5.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: stmt, tail: rest }, vars, idx) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut rest2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("---Replace other algorithm\n"))?;
                    }
                    (rest2, vars_1, idx2, labels) = addLabelToAlgorithms(metamodelica::AsArg::as_arg(&rest), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((metamodelica::cons(stmt.clone(), rest2.clone()), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStatements, outVarLst, outIndex, outStringList))
}

/* Fatima
protected function addLabelToElse
"helper function for labeling else part"
  input  DAE.Else inElse;
  input  SimCodeVar.SimVars inVarLst;
  input  tuple<Integer,Integer> inIndex;
  input  list <Integer> reduceList;
  input  BackendVarTransform.VariableReplacements inVarRepl;
  output DAE.Else outElse;
  output SimCodeVar.SimVars outVarLst;
  output tuple<Integer,Integer> outIndex;
  output list<String> outStringList;
  algorithm
  (outElse,outVarLst,outIndex,outStringList) := matchcontinue (inElse,inVarLst,inIndex,reduceList,inVarRepl)
    local
      SimCodeVar.SimVars vars,vars_1,vars_2,vars_3;
      tuple <Integer,Integer> idx,idx2,idx3,idx4;
      SimCode.SimEqSystem el,el2;
      list<DAE.Statement> rest,rest2,stmtLst,stmtLst2;
      list<String> labels,labels2,labels3,labels4,labels5;
      DAE.Else else_,else2;
      DAE.Exp e;
    case(DAE.NOELSE(),vars,idx,reduceList,inVarRepl) then (DAE.NOELSE(),vars,idx,{});
    case(DAE.ELSEIF(e,stmtLst,else_),vars,idx,reduceList,inVarRepl)
      algorithm
        ////Debug.fcall(Flags.CPP,print,"---Replace elseif with else\n" );
        (stmtLst2,vars_1,idx2,labels) = addLabelToAlgorithms(stmtLst,vars,idx,reduceList,inVarRepl);
        (else2,vars_2,idx3,labels2) = addLabelToElse(else_,vars_1,idx2,reduceList,inVarRepl);
        labels3=listAppend(labels,labels2);
      then
        (DAE.ELSEIF(e,stmtLst2,else2),vars_2,idx3,labels3);
    case(DAE.ELSE(stmtLst),vars,idx,reduceList,inVarRepl)
      algorithm
        //Debug.fcall(Flags.CPP,print,"---Replace else\n" );
        (stmtLst2,vars_1,idx2,labels) = addLabelToAlgorithms(stmtLst,vars,idx,reduceList,inVarRepl);
      then
        (DAE.ELSE(stmtLst2),vars_1,idx2,labels) ;
  end matchcontinue;
end addLabelToElse;
*/
fn addLabelToLinearEquationSystems(
    mut inLinear: &metamodelica::List<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)>,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIndex: (i32, i32),
    mut reduceList: &metamodelica::List<i32>,
    mut inVarRepl: &BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
)> {
    let mut outLinear: metamodelica::List<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outLinear, outVarLst, outIndex, outStringList) = (::match_deref::match_deref! { match inLinear {
        Deref @ metamodelica::ListNode::Nil => {
            let mut vars = inVarLst;
            let mut idx = inIndex;
            (metamodelica::nil(), vars, idx, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: (i, j, el), tail: rest } => {
            let mut vars = inVarLst;
            let mut idx = inIndex;
            let mut vars_1: SimCodeVar::SimVars;
            let mut vars_2: SimCodeVar::SimVars;
            let mut idx2: (i32, i32);
            let mut idx3: (i32, i32);
            let mut el2: metamodelica::Ref<SimCode::SimEqSystem>;
            let mut rest2: metamodelica::List<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)>;
            let mut labels: metamodelica::List<ArcStr>;
            let mut labels2: metamodelica::List<ArcStr>;
            let mut labels3: metamodelica::List<ArcStr>;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToEquations(&(list![el.clone()]), vars, idx, reduceList, inVarRepl)?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            el2 = metamodelica::Own::own(__pa0);
            vars_1 = metamodelica::Own::own(__pa1);
            idx2 = metamodelica::Own::own(__pa2);
            labels = metamodelica::Own::own(__pa3);
            (rest2, vars_2, idx3, labels2) = addLabelToLinearEquationSystems(rest, vars_1, idx2, reduceList, inVarRepl)?;
            labels3 = listAppend(labels, labels2);
            (metamodelica::cons((i.clone(), j.clone(), el2), rest2), vars_2, idx3, labels3)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outLinear, outVarLst, outIndex, outStringList))
}

fn addLabelToExp(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIntdex: (i32, i32),
    mut add: bool,
    mut reduceList: &metamodelica::List<i32>,
    mut inVarRepl: &BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIntdex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outExp, outVarLst, outIntdex, outStringList) = 'mc: {
        let __mc_input = inVarRepl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut vars: SimCodeVar::SimVars;
            let mut idx: (i32, i32);
            let mut labels: metamodelica::List<ArcStr>;
            ::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REDUCTION_METHOD.clone())?) {
                Deref @ "deletion" => (),
                _ => return Err("pattern mismatch"),
            } };
            (e, vars, idx, labels) =
                addLabelToExpForDeletion(inExp1.clone(), inVarLst.clone(), inIntdex, add, reduceList)?;
            Ok((e.clone(), vars.clone(), idx, labels.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut vars: SimCodeVar::SimVars;
            let mut idx: (i32, i32);
            let mut labels: metamodelica::List<ArcStr>;
            ::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REDUCTION_METHOD.clone())?) {
                Deref @ "substitution" => (),
                _ => return Err("pattern mismatch"),
            } };
            (e, vars, idx, labels, _) =
                addLabelToExpForSubstitution(inExp1.clone(), inVarLst.clone(), inIntdex, reduceList, inVarRepl)?;
            Ok((e.clone(), vars.clone(), idx, labels.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut vars: SimCodeVar::SimVars;
            let mut idx: (i32, i32);
            let mut labels: metamodelica::List<ArcStr>;
            ::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REDUCTION_METHOD.clone())?) {
                Deref @ "linearization" => (),
                _ => return Err("pattern mismatch"),
            } };
            (e, vars, idx, labels) =
                addLabelToExpForLinearization(inExp1.clone(), inVarLst.clone(), inIntdex, reduceList, inVarRepl);
            Ok((e.clone(), vars.clone(), idx, labels.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outVarLst, outIntdex, outStringList))
}

fn addLabelToExpForDeletion(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIntdex: (i32, i32),
    mut add: bool,
    mut reduceList: &metamodelica::List<i32>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIntdex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outExp, outVarLst, outIntdex, outStringList) = 'mc: {
        let __mc_input = (inExp1, inVarLst, inIntdex);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::ADD { .. }, exp2: e2 }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut labels5: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to add exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2, true, reduceList)?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), false, idx3, vars_2.clone(), reduceList)?;
                    } else {
                        (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add, idx3, vars_2.clone(), reduceList)?;
                    }
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4, labels5.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::SUB { .. }, exp2: e2 }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut labels5: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sub exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2, true, reduceList)?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), false, idx3, vars_2.clone(), reduceList)?;
                    } else {
                        (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add, idx3, vars_2.clone(), reduceList)?;
                    }
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4, labels5.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::MUL { .. }, exp2: e2 }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut labels5: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to mul exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), false, reduceList)?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2, false, reduceList)?;
                    (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add, idx3, vars_2.clone(), reduceList)?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4, labels5.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV { .. }, exp2: e2 }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to div exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() }), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::POW { .. }, exp2: e2 }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut labels5: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to pow exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2, true, reduceList)?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), false, idx3, vars_2.clone(), reduceList)?;
                    } else {
                        (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add, idx3, vars_2.clone(), reduceList)?;
                    }
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4, labels5.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1.clone() }), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { .. }, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Not Implemented: Add label to relation ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, vars, idx) => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3_1: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to if exp")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e2.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e3_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e3.clone(), vars_1.clone(), idx2, true, reduceList)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() }), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("add no label to pre arguments\n"))?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("add no label to edge arguments\n"))?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("add no label to change arguments\n"))?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("add no label to sample arguments\n"))?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("add no label for no event arguments\n"))?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut labels5: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to max exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2, true, reduceList)?;
                    (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("max") }), expLst: list![e1_1.clone(), e2_1.clone()], attr: attr.clone() }), add, idx3, vars_2.clone(), reduceList)?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4, labels5.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut labels5: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to min exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2, true, reduceList)?;
                    (e3, vars_3, idx4, labels3) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("min") }), expLst: list![e1_1.clone(), e2_1.clone()], attr: attr.clone() }), add, idx3, vars_2.clone(), reduceList)?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4, labels5.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to abs exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_2, idx3, labels2) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("abs") }), expLst: list![e2.clone()], attr: attr.clone() }), false, idx2, vars_1.clone(), reduceList)?;
                    } else {
                        (e3, vars_2, idx3, labels2) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("abs") }), expLst: list![e2.clone()], attr: attr.clone() }), add, idx2, vars_1.clone(), reduceList)?;
                    }
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sqrt exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_2, idx3, labels2) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sqrt") }), expLst: list![e2.clone()], attr: attr.clone() }), false, idx2, vars_1.clone(), reduceList)?;
                    } else {
                        (e3, vars_2, idx3, labels2) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sqrt") }), expLst: list![e2.clone()], attr: attr.clone() }), add, idx2, vars_1.clone(), reduceList)?;
                    }
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sin") }), expLst: list![e2.clone()], attr: attr.clone() }), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e3, vars_2, idx3, labels2) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cos") }), expLst: list![e2.clone()], attr: attr.clone() }), add, idx2, vars_1.clone(), reduceList)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("asin") }), expLst: list![e2.clone()], attr: attr.clone() }), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e3, vars_2, idx3, labels2) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("acos") }), expLst: list![e2.clone()], attr: attr.clone() }), add, idx2, vars_1.clone(), reduceList)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to tan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("tan") }), expLst: list![e1_1.clone()], attr: attr.clone() }), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to atan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("atan") }), expLst: list![e1_1.clone()], attr: attr.clone() }), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to exp exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    (e3, vars_2, idx3, labels2) = addOneLabel(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("exp") }), expLst: list![e2.clone()], attr: attr.clone() }), add, idx2, vars_1.clone(), reduceList)?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3, labels3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, vars, idx) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to div exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("div") }), expLst: list![e1_1.clone(), e2.clone()], attr: attr.clone() }), vars_1.clone(), idx2, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, expLst: expl, attr }, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to other call function ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expl.clone(), attr: attr.clone() }), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: valueR }, vars, idx) => {
                    if !((valueR.clone() == metamodelica::OrderedFloat(0.0_f64))) { return Err("guard") }
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("Add no label to const 0.0\n"))?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RCONST { real: _ }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to real const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: valueI }, vars, idx) => {
                    if !((valueI.clone() == 0)) { return Err("guard") }
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(literal!("Add no label to const 0\n"))?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ICONST { integer: _ }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to integer const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::SCONST { string: _ }, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to string const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BCONST { bool: _ }, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to boolean const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to string variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to boolean variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add, idx.clone(), vars.clone(), reduceList)?;
                    Ok((e2.clone(), vars_1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars, idx) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unknown expression ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outVarLst, outIntdex, outStringList))
}

/*
protected function addLabelToExpList
"function that adds labels to expression lists"
  input list<Expression.Exp> inExpLst;
  input SimCodeVar.SimVars inVarLst;
  input tuple<Integer,Integer> inIndex;
  input list<Integer> reduceList;
  output list<Expression.Exp>  outExpLst;
  output SimCodeVar.SimVars outVarLst;
  output tuple<Integer,Integer> outIndex;
  output list<String> outStringList;
algorithm
  (outExpLst,outVarLst,outIndex,outStringList):=
  matchcontinue (inExpLst,inVarLst,inIndex,reduceList)
    local
      Expression.Exp e,e_1,e_2,e1;
      tuple<Integer,Integer> idx1,idx2,idx3;
      list<Expression.Exp> er,er2;
      SimCodeVar.SimVars vars,vars_1,vars_2;
      list<String> labels,labels2,labels3;
      BackendVarTransform.VariableReplacements repl;
    case ({},vars,idx1,reduceList) then ({},vars,idx1,{});
    case ((e1 :: er),vars,idx1,reduceList)
      algorithm
        repl=BackendVarTransform.emptyReplacements();
        (e_1,vars_1,idx2,labels) = addLabelToExp(e1,vars,idx1,true,reduceList,repl);
        (er2,vars_2,idx3,labels2) = addLabelToExpList(er, vars_1, idx2,reduceList);
        labels3=listAppend(labels,labels2);
      then
        (e_1::er2,vars_2,idx3,labels3);
  end matchcontinue;
end addLabelToExpList;
*/
fn addOneLabel(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut add: bool,
    mut inIndex: (i32, i32),
    mut inVarLst: SimCodeVar::SimVars,
    mut reduceList: &metamodelica::List<i32>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outExp, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inExp1, add, inIndex, inVarLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, true, (i, p), vars) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut i_1: i32;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { return Err("pattern mismatch") };
                    let true = (List::contains(reduceList, i.clone(), &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    e2 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), e.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e2.clone(), vars.clone(), (i_1, p.clone()), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, true, (i, p), vars) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut i_1: i32;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { return Err("pattern mismatch") };
                    e2 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e2.clone(), vars.clone(), (i_1, p.clone()), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, true, (i, p), vars) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut name: ArcStr;
                    let mut name1: ArcStr;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut p_1: i32;
                    let mut i_1: i32;
                    (vars_1, name) = createLabelVar(metamodelica::AsArg::as_arg(&vars), p.clone(), i.clone())?;
                    name1 = stringAppend(name.clone(), literal!("_1"));
                    e2 = multiply(e.clone(), name1.clone())?;
                    p_1 = p.clone() + 2;
                    i_1 = i.clone() + 1;
                    Ok((e2.clone(), vars_1.clone(), (i_1, p_1), list![name.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, false, (i, p), vars) => {
                    Ok((e.clone(), vars.clone(), (i.clone(), p.clone()), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outVarLst, outIndex, outStringList))
}

fn addLabelToExpForLinearization(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIndex: (i32, i32),
    mut reduceList: &metamodelica::List<i32>,
    mut inVarRepl: &BackendVarTransform::VariableReplacements,
) -> (
    metamodelica::Ref<DAE::Exp>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outExp, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inExp1, inVarLst, inIndex);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 }, vars, idx) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e1.clone())?) else { return Err("pattern mismatch") };
                    let false = (Expression::expHasCrefs(e2.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to pow exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e3, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    Ok((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: e2.clone() }), vars1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 }, vars, idx) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut e6: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let false = (Expression::expHasCrefs(e1.clone())?) else { return Err("pattern mismatch") };
                    let true = (Expression::expHasCrefs(e2.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to pow exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e3, vars1, idx1, labels) = addLabelToExpForLinearization(e2.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e4 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: e3.clone() });
                    e5 = linearizeExp(e4.clone(), e3.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e6, vars2, idx2, labels1) = addTwoLabels(e4.clone(), e5.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e6.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, vars, idx) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to binary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e3, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    (e4, vars2, idx2, labels1) = addLabelToExpForLinearization(e2.clone(), vars1.clone(), idx1, reduceList, inVarRepl);
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op.clone(), exp2: e4.clone() }), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    Ok((metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e2.clone() }), vars1.clone(), idx1, labels.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, vars, idx) => {
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to if exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e4, vars1, idx1, labels) = addLabelToExpForLinearization(e2.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    (e5, vars2, idx2, labels1) = addLabelToExpForLinearization(e3.clone(), vars1.clone(), idx1, reduceList, inVarRepl);
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e4.clone(), expElse: e5.clone() }), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sin") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cos") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to tan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("tan") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to asin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("asin") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to acos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("acos") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to atan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("atan") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to exp exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("exp") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to log exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("log") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let true = (Expression::expHasCrefs(e.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sqrt exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl);
                    e3 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sqrt") }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), metamodelica::AsArg::as_arg(&vars), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1, reduceList)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2, labels2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars, idx) => {
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outVarLst, outIndex, outStringList)
}

fn addTwoLabels(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut label: bool,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIndex: (i32, i32),
    mut reduceList: &metamodelica::List<i32>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    (outExp, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inExp1, inExp2, label, inVarLst, inIndex);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, true, vars, (i, p)) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut i_1: i32;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { return Err("pattern mismatch") };
                    let true = (List::contains(reduceList, i.clone(), &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    e3 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), e1.clone())?;
                    e4 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e2.clone())?;
                    e5 = Expression::expAdd(e3.clone(), e4.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e5.clone(), vars.clone(), (i_1, p.clone()), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, true, vars, (i, p)) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut i_1: i32;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { return Err("pattern mismatch") };
                    e3 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e1.clone())?;
                    e4 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), e2.clone())?;
                    e5 = Expression::expAdd(e3.clone(), e4.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e5.clone(), vars.clone(), (i_1, p.clone()), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, true, vars, (i, p)) => {
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut i_1: i32;
                    let mut p_1: i32;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut name: ArcStr;
                    let mut name1: ArcStr;
                    let mut name2: ArcStr;
                    (vars_1, name) = createLabelVar(metamodelica::AsArg::as_arg(&vars), p.clone(), i.clone())?;
                    name1 = stringAppend(name.clone(), literal!("_1"));
                    name2 = stringAppend(name.clone(), literal!("_2"));
                    e3 = multiply(e1.clone(), name1.clone())?;
                    e4 = multiply(e2.clone(), name2.clone())?;
                    e5 = Expression::expAdd(e3.clone(), e4.clone())?;
                    p_1 = p.clone() + 2;
                    i_1 = i.clone() + 1;
                    Ok((e5.clone(), vars_1.clone(), (i_1, p_1), list![name.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, _, false, vars, (i, p)) => {
                    Ok((e1.clone(), vars.clone(), (i.clone(), p.clone()), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outVarLst, outIndex, outStringList))
}

fn linearizeExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::Exp>,
    mut inVarLst: &SimCodeVar::SimVars,
    mut inVarRepl: BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp, source, inVarRepl)) {
        (e1, e2, repl) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            let mut e4: metamodelica::Ref<DAE::Exp>;
            let mut e5: metamodelica::Ref<DAE::Exp>;
            let mut e6: metamodelica::Ref<DAE::Exp>;
            let mut tmpExp: metamodelica::Ref<DAE::Exp>;
            let mut replExp: metamodelica::Ref<DAE::Exp>;
            let mut tmp: metamodelica::Ref<DAE::ComponentRef>;
            (replExp, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e2), metamodelica::AsArg::as_arg(&repl), None);
            (e, _) = Expression::replaceExp(e1.clone(), e2.clone(), replExp.clone())?;
            tmp = ComponentReferenceBasics::makeCrefIdent(literal!("linVar"), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
            tmpExp = Expression::crefExp(tmp.clone())?;
            (e3, _) = Expression::replaceExp(e1.clone(), e2.clone(), tmpExp.clone())?;
            e4 = Differentiate::differentiateExpSolve(e3, tmp, None)?;
            (e5, _) = Expression::replaceExp(e4, tmpExp, replExp.clone())?;
            e6 = Expression::expAdd(e, Expression::expMul(e5, Expression::expSub(e2.clone(), replExp)?)?)?;
            e6
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn addLabelToExpForSubstitution(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inVarLst: SimCodeVar::SimVars,
    mut inIndex: (i32, i32),
    mut reduceList: &metamodelica::List<i32>,
    mut inVarRepl: &BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    SimCodeVar::SimVars,
    (i32, i32),
    metamodelica::List<ArcStr>,
    bool,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: metamodelica::List<ArcStr>;
    let mut substitute: bool;
    (outExp, outVarLst, outIndex, outStringList, substitute) = 'mc: {
        let __mc_input = (inExp1, inVarLst, inIndex);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, vars, idx) => {
                    let mut ex: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut subs1: bool;
                    let mut subs2: bool;
                    let mut subs3: bool;
                    let mut subs4: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ex = metamodelica::Own::own(__pa0);
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1, reduceList, inVarRepl)?;
                    subs3 = boolAnd(subs1, subs2);
                    (e5, vars3, idx3, labels2) = addTwoLabels(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op.clone(), exp2: e4.clone() }), ex.clone(), subs3, vars2.clone(), idx2, reduceList)?;
                    subs4 = boolOr(subs1, subs2);
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to binary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((e5.clone(), vars3.clone(), idx3, labels4.clone(), subs4))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e2.clone() }), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, vars, idx) => {
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to if exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e4, vars1, idx1, labels, _) = addLabelToExpForSubstitution(e2.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (e5, vars2, idx2, labels1, _) = addLabelToExpForSubstitution(e3.clone(), vars1.clone(), idx1, reduceList, inVarRepl)?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e4.clone(), expElse: e5.clone() }), vars2.clone(), idx2, labels2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, vars, idx) => {
                    let mut ex: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut subs1: bool;
                    let mut subs2: bool;
                    let mut subs3: bool;
                    let mut subs4: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ex = metamodelica::Own::own(__pa0);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to max exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1, reduceList, inVarRepl)?;
                    subs3 = boolAnd(subs1, subs2);
                    (e5, vars3, idx3, labels2) = addTwoLabels(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("max") }), expLst: list![e3.clone(), e4.clone()], attr: attr.clone() }), ex.clone(), subs3, vars2.clone(), idx2, reduceList)?;
                    subs4 = boolOr(subs1, subs2);
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    Ok((e5.clone(), vars3.clone(), idx3, labels4.clone(), subs4))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, vars, idx) => {
                    let mut ex: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut subs1: bool;
                    let mut subs2: bool;
                    let mut subs3: bool;
                    let mut subs4: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ex = metamodelica::Own::own(__pa0);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to min exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1, reduceList, inVarRepl)?;
                    subs3 = boolAnd(subs1, subs2);
                    (e5, vars3, idx3, labels2) = addTwoLabels(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("min") }), expLst: list![e3.clone(), e4.clone()], attr: attr.clone() }), ex.clone(), subs3, vars2.clone(), idx2, reduceList)?;
                    subs4 = boolOr(subs1, subs2);
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    Ok((e5.clone(), vars3.clone(), idx3, labels4.clone(), subs4))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to abs exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sqrt exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to tan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to asin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to acos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to atan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars, idx) => {
                    let mut ex: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut subs: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ex = metamodelica::Own::own(__pa0);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to exp exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(ex.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), subs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, vars, idx) => {
                    let mut ex: metamodelica::Ref<DAE::Exp>;
                    let mut e3: metamodelica::Ref<DAE::Exp>;
                    let mut e4: metamodelica::Ref<DAE::Exp>;
                    let mut e5: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let mut labels1: metamodelica::List<ArcStr>;
                    let mut labels2: metamodelica::List<ArcStr>;
                    let mut labels3: metamodelica::List<ArcStr>;
                    let mut labels4: metamodelica::List<ArcStr>;
                    let mut subs1: bool;
                    let mut subs2: bool;
                    let mut subs3: bool;
                    let mut subs4: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ex = metamodelica::Own::own(__pa0);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to div exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList, inVarRepl)?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1, reduceList, inVarRepl)?;
                    subs3 = boolAnd(subs1, subs2);
                    (e5, vars3, idx3, labels2) = addTwoLabels(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("div") }), expLst: list![e3.clone(), e4.clone()], attr: attr.clone() }), ex.clone(), subs3, vars2.clone(), idx2, reduceList)?;
                    subs4 = boolOr(subs1, subs2);
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    Ok((e5.clone(), vars3.clone(), idx3, labels4.clone(), subs4))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_INTEGER { varLst: _ } }, vars, idx) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to integer variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addTwoLabels(e.clone(), e1.clone(), true, vars.clone(), idx.clone(), reduceList)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_REAL { varLst: _ } }, vars, idx) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to real variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    (e2, vars1, idx1, labels) = addTwoLabels(e.clone(), e1.clone(), true, vars.clone(), idx.clone(), reduceList)?;
                    Ok((e2.clone(), vars1.clone(), idx1, labels.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars, idx) => {
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outVarLst, outIndex, outStringList, substitute))
}

/*
protected function addLabelToExpListForSubstitution
"function that adds labels to expressions for substitution"
  input list<Expression.Exp> inExpLst;
  input SimCodeVar.SimVars inVarLst;
  input tuple<Integer,Integer> inIndex;
  input list<Integer> reduceList;
  input BackendVarTransform.VariableReplacements inVarRepl;
  output list<Expression.Exp> outExpLst;
  output SimCodeVar.SimVars outVarLst;
  output tuple<Integer,Integer> outIndex;
  output list<String> outStringList;
algorithm
  (outExpLst,outVarLst,outIndex,outStringList):=matchcontinue(inExpLst,inVarLst,inIndex,reduceList,inVarRepl)
    local
      Expression.Exp e,e_1,e_2,e1;
      tuple<Integer,Integer> idx1,idx2,idx3;
      list<Expression.Exp> er,er2;
      SimCodeVar.SimVars vars,vars_1,vars_2;
      list<String> labels,labels2,labels3;
      BackendVarTransform.VariableReplacements repl;
    case ({},vars,idx1,reduceList,repl) then ({},vars,idx1,{});
    case ((e1 :: er),vars,idx1,reduceList,repl)
      algorithm
        (e_1,vars_1,idx2,labels) = addLabelToExpForSubstitution(e1,vars,idx1,reduceList,repl);
        (er2,vars_2,idx3,labels2) = addLabelToExpListForSubstitution(er, vars_1, idx2,reduceList,repl);
        labels3=listAppend(labels,labels2);
      then
        (e_1::er2,vars_2,idx3,labels3);
  end matchcontinue;

end addLabelToExpListForSubstitution;
*/
fn substituteExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVarRepl: BackendVarTransform::VariableReplacements,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replPerformed: bool;
    (outExp, replPerformed) = (::match_deref::match_deref! { match &((inExp, inVarRepl)) {
        (e, repl) => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            (e1, replPerformed) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), None);
            (e1, replPerformed)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, replPerformed)
}

fn multiply(mut inExp: metamodelica::Ref<DAE::Exp>, mut inString: ArcStr) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp, inString)) {
        (e, name) => {
            let mut e2: metamodelica::Ref<DAE::Exp>;
            e2 = Expression::expMul(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }), e.clone())?;
            if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("generate label  ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(" for term ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
            }
            e2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn createLabelVar(
    mut inVariables: &SimCodeVar::SimVars,
    mut inInteger: i32,
    mut inInteger2: i32,
) -> Result<(SimCodeVar::SimVars, ArcStr)> {
    let mut outVariables: SimCodeVar::SimVars;
    let mut outString: ArcStr;
    (outVariables, outString) = (match (inVariables.clone(), inInteger, inInteger2) {
        (
            SimCodeVar::SimVars {
                stateVars: ref states,
                derivativeVars: ref derVar,
                algVars: ref alg,
                discreteAlgVars: ref disAlg,
                intAlgVars: ref intAlg,
                boolAlgVars: ref boolAlg,
                inputVars: ref inVar,
                outputVars: ref outVar,
                aliasVars: ref algAlias,
                intAliasVars: ref intAlias,
                boolAliasVars: ref boolAlias,
                paramVars: ref param,
                intParamVars: ref intParam,
                boolParamVars: ref boolParam,
                stringAlgVars: ref stringAlg,
                stringParamVars: ref stringParam,
                stringAliasVars: ref stringAlias,
                extObjVars: ref extObjVar,
                constVars: ref r#const,
                intConstVars: ref intConst,
                boolConstVars: ref boolConst,
                stringConstVars: ref stringConst,
                jacobianVars: ref jacobianVar,
                seedVars: ref seedVar,
                realOptimizeConstraintsVars: ref realOptConst,
                realOptimizeFinalConstraintsVars: ref realOptFinalConst,
                sensitivityVars: ref sensVar,
                dataReconSetcVars: ref setcVar,
                dataReconinputVars: ref datareconinputvar,
                dataReconSetBVars: ref setBVar,
            },
            mut p,
            mut i,
        ) => {
            let mut simVar_1: metamodelica::Ref<SimCodeVar::SimVar>;
            let mut simVar_2: metamodelica::Ref<SimCodeVar::SimVar>;
            let mut param_1: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
            let mut param_2: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
            let mut name: ArcStr;
            let mut name1: ArcStr;
            let mut name2: ArcStr;
            let mut indexStr: ArcStr;
            let mut param = param.clone();
            indexStr = intString(i);
            name = stringAppend(arcstr::literal!(LABELNAME), indexStr);
            name1 = stringAppend(name.clone(), literal!("_1"));
            name2 = stringAppend(name.clone(), literal!("_2"));
            simVar_1 = metamodelica::Ref::new(SimCodeVar::SimVar {
                name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: name1,
                    identType: DAE::T_REAL_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                varKind: openmodelica_backend_types::BackendDAE::VarKind::PARAM,
                comment: literal!(""),
                unit: literal!(""),
                displayUnit: literal!(""),
                index: p,
                minValue: None,
                maxValue: None,
                initialValue: Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(1.0_f64),
                })),
                nominalValue: None,
                isFixed: true,
                type_: DAE::T_REAL_DEFAULT().clone(),
                isDiscrete: false,
                arrayCref: None,
                aliasvar: openmodelica_simcode_types::SimCodeVar::AliasVariable::NOALIAS,
                source: DAE::emptyElementSource().clone(),
                causality: Some(openmodelica_simcode_types::SimCodeVar::Causality::LOCAL),
                variable_index: None,
                fmi_index: None,
                numArrayElement: metamodelica::nil(),
                isValueChangeable: false,
                isProtected: false,
                hideResult: None,
                isEncrypted: false,
                inputIndex: None,
                initNonlinear: false,
                matrixName: None,
                variability: None,
                initial_: None,
                exportVar: None,
                relativeQuantity: false,
                isConnectorFlow: false,
            });
            param = param.clone().reverse();
            param_1 = metamodelica::cons(simVar_1, param.clone());
            p = p + 1;
            simVar_2 = metamodelica::Ref::new(SimCodeVar::SimVar {
                name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: name2,
                    identType: DAE::T_REAL_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                varKind: openmodelica_backend_types::BackendDAE::VarKind::PARAM,
                comment: literal!(""),
                unit: literal!(""),
                displayUnit: literal!(""),
                index: p,
                minValue: None,
                maxValue: None,
                initialValue: Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                })),
                nominalValue: None,
                isFixed: true,
                type_: DAE::T_REAL_DEFAULT().clone(),
                isDiscrete: false,
                arrayCref: None,
                aliasvar: openmodelica_simcode_types::SimCodeVar::AliasVariable::NOALIAS,
                source: DAE::emptyElementSource().clone(),
                causality: Some(openmodelica_simcode_types::SimCodeVar::Causality::LOCAL),
                variable_index: None,
                fmi_index: None,
                numArrayElement: metamodelica::nil(),
                isValueChangeable: false,
                isProtected: false,
                hideResult: None,
                isEncrypted: false,
                inputIndex: None,
                initNonlinear: false,
                matrixName: None,
                variability: None,
                initial_: None,
                exportVar: None,
                relativeQuantity: false,
                isConnectorFlow: false,
            });
            param_2 = metamodelica::cons(simVar_2, param_1);
            param_2 = param_2.reverse();
            (
                SimCodeVar::SimVars {
                    stateVars: states.clone(),
                    derivativeVars: derVar.clone(),
                    algVars: alg.clone(),
                    discreteAlgVars: disAlg.clone(),
                    intAlgVars: intAlg.clone(),
                    boolAlgVars: boolAlg.clone(),
                    inputVars: inVar.clone(),
                    outputVars: outVar.clone(),
                    aliasVars: algAlias.clone(),
                    intAliasVars: intAlias.clone(),
                    boolAliasVars: boolAlias.clone(),
                    paramVars: param_2,
                    intParamVars: intParam.clone(),
                    boolParamVars: boolParam.clone(),
                    stringAlgVars: stringAlg.clone(),
                    stringParamVars: stringParam.clone(),
                    stringAliasVars: stringAlias.clone(),
                    extObjVars: extObjVar.clone(),
                    constVars: r#const.clone(),
                    intConstVars: intConst.clone(),
                    boolConstVars: boolConst.clone(),
                    stringConstVars: stringConst.clone(),
                    jacobianVars: jacobianVar.clone(),
                    seedVars: seedVar.clone(),
                    realOptimizeConstraintsVars: realOptConst.clone(),
                    realOptimizeFinalConstraintsVars: realOptFinalConst.clone(),
                    sensitivityVars: sensVar.clone(),
                    dataReconSetcVars: setcVar.clone(),
                    dataReconinputVars: datareconinputvar.clone(),
                    dataReconSetBVars: setBVar.clone(),
                },
                name,
            )
        }
    });
    Ok((outVariables, outString))
}

fn makeReduceList(
    mut expLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inList: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((expLst, inList)) {
            (Deref @ metamodelica::ListNode::Nil, lst) => {
                return Ok(lst.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::INTEGER { value: v }, tail: expLstRest }, lst) => {
                let mut i: i32;
                let mut lst2: metamodelica::List<i32>;
                let mut lst3: metamodelica::List<i32>;
                i = v.clone();
                lst2 = listAppend(lst.clone(), list![i]);
                { (expLst, inList) = (expLstRest.clone(), lst2); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn StringDelimit2Int(mut inString: ArcStr, mut inDelim: ArcStr) -> metamodelica::List<i32> {
    let mut outList: metamodelica::List<i32>;
    outList = 'mc: {
        let __mc_input = (inString, inDelim);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut v, mut delim) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut lst: metamodelica::List<ArcStr>;
            let mut lst2: metamodelica::List<i32>;
            lst = Util::stringSplitAtChar(v.clone(), delim.clone())?;
            lst2 = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut s in (lst.clone()).into_iter().cloned() {
                    let __x = stringInt(s.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            Ok(lst2.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outList
}

pub(crate) fn createBackendLabelVars(
    mut modelInfo: &SimCode::ModelInfo,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut labelList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    labelList = (match modelInfo.clone() {
        SimCode::ModelInfo {
            varInfo: SimCode::VarInfo {
                numParams: mut numParams,
                ..
            },
            labels: mut labels,
            ..
        } => {
            let mut list1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            list1 = createBackendLabelVars2(metamodelica::AsArg::as_arg(&labels), numParams.clone())?;
            list1
        }
    });
    Ok(labelList)
}

fn createBackendLabelVars2(
    mut inLabels: &metamodelica::List<ArcStr>,
    mut inIndex: i32,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outList = (::match_deref::match_deref! { match inLabels {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: name, tail: rest } => {
            let mut p = inIndex;
            let mut name1: ArcStr;
            let mut name2: ArcStr;
            let mut var1: metamodelica::Ref<BackendDAE::Var>;
            let mut var2: metamodelica::Ref<BackendDAE::Var>;
            let mut list1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut list2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut list3: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            name1 = stringAppend(name.clone(), literal!("_1"));
            name2 = stringAppend(name.clone(), literal!("_2"));
            var1 = metamodelica::Ref::new(BackendDAE::Var { varName: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name1, identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: openmodelica_backend_types::BackendDAE::VarKind::PARAM, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: Some(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })), arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None })), tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
            p = p + 1;
            var2 = metamodelica::Ref::new(BackendDAE::Var { varName: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name2, identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: openmodelica_backend_types::BackendDAE::VarKind::PARAM, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: Some(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None })), tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
            list1 = list![var1, var2];
            p = p + 1;
            list2 = createBackendLabelVars2(rest, p)?;
            list3 = listAppend(list1, list2);
            list3
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outList)
}
