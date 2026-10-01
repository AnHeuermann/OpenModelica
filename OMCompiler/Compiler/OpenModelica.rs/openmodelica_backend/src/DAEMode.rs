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

use crate::BackendDAEFunc;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::Initialization;
use crate::Matching;
use openmodelica_backend_types::BackendDAE;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::StackOverflow;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn getEqSystemDAEmode(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut fileNamePrefix: &ArcStr,
    mut strPreOptModules: Option<metamodelica::List<ArcStr>>,
    mut strmatchingAlgorithm: Option<ArcStr>,
    mut strdaeHandler: Option<ArcStr>,
    mut strPostOptModules: Option<metamodelica::List<ArcStr>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::BackendDAE>,
    metamodelica::Ref<BackendDAE::BackendDAE>,
    Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outDAEmode: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    let mut outInitDAE: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    let mut outInitDAE_lambda0_option: Option<metamodelica::Ref<BackendDAE::BackendDAE>> = None;
    let mut outRemovedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> =
        metamodelica::nil();
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    let mut simDAE: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    let mut preOptModules: metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)> = metamodelica::nil();
    let mut postOptModules: metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)> = metamodelica::nil();
    let mut daeHandler: (
        BackendDAEFunc::StructurallySingularSystemHandlerFunc,
        ArcStr,
        BackendDAEFunc::stateDeselectionFunc,
        ArcStr,
    ) = (
        std::sync::Arc::new(|_, _, _, _, _, _, _| unreachable!("checkpoint placeholder")),
        arcstr::literal!(""),
        std::sync::Arc::new(|_, _| unreachable!("checkpoint placeholder")),
        arcstr::literal!(""),
    );
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr) = (
        std::sync::Arc::new(|_, _, _, _, _, _| unreachable!("checkpoint placeholder")),
        arcstr::literal!(""),
    );
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut numCheckpoints: i32;
    let mut oldSize: i32 = 0;
    numCheckpoints = ErrorExt::getNumCheckpoints();
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        StackOverflow::clearStacktraceMessages();
        preOptModules = BackendDAEUtil::getPreOptModules(strPreOptModules.clone())?;
        postOptModules = BackendDAEUtil::getPostOptModules(
            (::match_deref::match_deref! { match &(&strPostOptModules) {
                None => Some(getPostOptModulesDAEString()?),
                _ => strPostOptModules.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }),
        )?;
        matchingAlgorithm = BackendDAEUtil::getMatchingAlgorithm(strmatchingAlgorithm.clone())?;
        FlagsUtil::setConfigString(Flags::INDEX_REDUCTION_METHOD.clone(), literal!("dummyDerivatives"))?;
        daeHandler = BackendDAEUtil::getIndexReductionMethod(strdaeHandler.clone())?;
        if Flags::isSet(Flags::DUMP_DAE_LOW.clone())? {
            BackendDump::dumpBackendDAE(&inDAE, &(literal!("dumpdaelow")))?;
            if Flags::isSet(Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone())? {
                BackendDump::graphvizAdjacencyMatrix(&inDAE, literal!("dumpdaelow"))?;
            }
        }
        dae = BackendDAEUtil::preOptimizeDAE(inDAE.clone(), &preOptModules)?;
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("pre-optimization done (n="));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendDAEUtil::daeSize(&dae)?)));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
        dae = BackendDAEUtil::causalizeDAE(&dae, None, &matchingAlgorithm, &daeHandler, true)?;
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("matching and sorting (n="));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendDAEUtil::daeSize(&dae)?)));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
        if Flags::isSet(Flags::GRAPHML.clone())? {
            BackendDump::dumpBipartiteGraphDAE(&dae, fileNamePrefix)?;
        }
        if Flags::isSet(Flags::EVAL_OUTPUT_ONLY.clone())? {
            oldSize = BackendDAEUtil::daeSize(&dae)?;
            dae = BackendDAEOptimize::evaluateOutputsOnly(dae.clone())?;
            execStat(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("evaluateOutputsOnly (n="));
                    __mm_s.push_str(&*intString(oldSize));
                    __mm_s.push_str(&*literal!(" -> n="));
                    __mm_s.push_str(&*intString(BackendDAEUtil::daeSize(&dae)?));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            BackendDump::bltdump(literal!("bltdump"), &dae)?;
        }
        (
            outInitDAE,
            outInitDAE_lambda0_option,
            outRemovedInitialEquationLst,
            globalKnownVars,
            dae,
        ) = Initialization::solveInitialSystem(dae.clone())?;
        simDAE = BackendDAEUtil::setFunctionTree(&dae, BackendDAEUtil::getFunctions(&outInitDAE.shared));
        simDAE = BackendDAEUtil::setDAEGlobalKnownVars(&simDAE, globalKnownVars.clone());
        simDAE = BackendDAEOptimize::addInitialStmtsToAlgorithms(&simDAE, false)?;
        simDAE = Initialization::removeInitializationStuff(simDAE.clone())?;
        simDAE = BackendDAEUtil::postOptimizeDAE(simDAE.clone(), &postOptModules, &matchingAlgorithm, &daeHandler)?;
        simDAE = BackendDAEUtil::sortGlobalKnownVarsInDAE(simDAE.clone())?;
        if Flags::isSet(Flags::DUMP_INDX_DAE.clone())? {
            BackendDump::dumpBackendDAE(&simDAE, &(literal!("dumpindxdae")))?;
        }
        outDAEmode = simDAE.clone();
        return Ok(true);
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok((
                    outDAEmode.clone(),
                    outInitDAE.clone(),
                    outInitDAE_lambda0_option.clone(),
                    outRemovedInitialEquationLst.clone(),
                ));
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            {
                let __v = None;
                openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints);
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Stack overflow in "));
                    __mm_s.push_str(&*literal!("DAEMode.getEqSystemDAEmode"));
                    __mm_s.push_str(&*literal!("...\n"));
                    __mm_s.push_str(&*stringDelimitList(
                        StackOverflow::readableStacktraceMessages()?,
                        literal!("\n"),
                    ));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/DAEMode.mo"),
            )?;
            StackOverflow::clearStacktraceMessages();
        }
    }
    return Err("fail");
    Ok((
        outDAEmode,
        outInitDAE,
        outInitDAE_lambda0_option,
        outRemovedInitialEquationLst,
    ))
}

/*
get config function
*/
fn getPostOptModulesDAEString() -> Result<metamodelica::List<ArcStr>> {
    let mut strpostOptModules: metamodelica::List<ArcStr>;
    strpostOptModules = Config::getPostOptModulesDAE()?;
    Ok(strpostOptModules)
}

// =============================================================================
// public section for createDAEmodeBDAE
//
// =============================================================================
pub(crate) fn createDAEmodeBDAE(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, &createDAEmodeEqSystem)?;
    Ok(outDAE)
}

// =============================================================================
// protected section for createDAEmodeBDAE
//
// =============================================================================
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TraverseEqnAryFold {
    pub globalDAEData: BackendDAE::BackendDAEModeData,
    pub newDAEVars: BackendDAE::Variables,
    pub newDAEEquations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    pub systemVars: BackendDAE::Variables,
    pub functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    pub recursiveStrongComponentRun: bool,
    pub shared: metamodelica::Ref<BackendDAE::Shared>,
}

impl metamodelica::gc::MMTrace for TraverseEqnAryFold {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.globalDAEData, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.newDAEVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.newDAEEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.systemVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.functionTree, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.recursiveStrongComponentRun, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.shared, __mmv)?;
        Ok(())
    }
}
impl Default for TraverseEqnAryFold {
    fn default() -> Self {
        Self {
            globalDAEData: Default::default(),
            newDAEVars: Default::default(),
            newDAEEquations: Default::default(),
            systemVars: Default::default(),
            functionTree: Default::default(),
            recursiveStrongComponentRun: Default::default(),
            shared: Default::default(),
        }
    }
}

pub type TRAVERSER_CREATE_DAE = TraverseEqnAryFold;

fn createDAEmodeEqSystem(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut travArgs: TraverseEqnAryFold;
    let mut globalDAEData: BackendDAE::BackendDAEModeData;
    let mut retSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut newDAEVars: BackendDAE::Variables;
    let mut newDAEEquations: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut systemSize: i32;
    let mut debug: bool = Flags::isSet(Flags::DEBUG_DAEMODE.clone())?;
    let exec: bool = false;
    globalDAEData = shared.daeModeData.clone();
    systemSize = BackendDAEUtil::systemSize(&syst)?;
    newDAEVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    newDAEEquations = BackendEquation::emptyEqnsSized(systemSize);
    travArgs = TraverseEqnAryFold {
        globalDAEData: globalDAEData,
        newDAEVars: newDAEVars,
        newDAEEquations: newDAEEquations,
        systemVars: syst.orderedVars.clone(),
        functionTree: shared.functionTree.clone(),
        recursiveStrongComponentRun: false,
        shared: shared.clone(),
    };
    if debug {
        BackendDump::printEqSystem(syst.clone())?;
    }
    travArgs = BackendDAEUtil::traverseEqSystemStrongComponents(
        &syst,
        &move |__a0: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
               __a1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
               __a2: metamodelica::List<i32>,
               __a3: metamodelica::List<i32>,
               __a4: TraverseEqnAryFold| traverserStrongComponents(&__a0, __a1, &__a2, &__a3, __a4),
        travArgs,
    )?;
    if exec {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("DAEmode: created residual equations for system size :  "));
                __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(&syst)?));
                __mm_s.push_str(&*literal!(": "));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    globalDAEData = travArgs.globalDAEData.clone();
    if (globalDAEData.modelVars).is_some() {
        globalDAEData.modelVars = Some(BackendVariable::addVariables(
            travArgs.systemVars.clone(),
            globalDAEData.modelVars.clone().ok_or("pattern mismatch")?,
        )?);
    } else {
        globalDAEData.modelVars = Some(travArgs.systemVars.clone());
    }
    if exec {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("DAEmode: adding residual variables:  "));
                __mm_s.push_str(&*intString(BackendVariable::varsSize(
                    &(globalDAEData.modelVars.clone().ok_or("pattern mismatch")?),
                )));
                __mm_s.push_str(&*literal!(": "));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    retSystem = BackendDAEUtil::createEqSystem(
        travArgs.newDAEVars.clone(),
        BackendEquation::emptyEqns(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    retSystem = BackendDAEUtil::setEqSystEqs(retSystem, travArgs.newDAEEquations.clone());
    retSystem = BackendDAEUtil::setEqSystRemovedEqns(retSystem, syst.removedEqs.clone());
    retSystem = BackendEquation::requationsAddDAE(&(ExpandableArray::toList(shared.removedEqs.clone())?), retSystem)?;
    if exec {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("DAEmode: created system:  "));
                __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(&retSystem)?));
                __mm_s.push_str(&*literal!(": "));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    syst = retSystem;
    assign_field!(shared.daeModeData = globalDAEData.clone());
    if debug {
        BackendDump::printEqSystem(syst.clone())?;
    }
    if debug {
        BackendDump::dumpBackendDAEModeData(&globalDAEData)?;
    }
    Ok((syst, shared))
}

fn traverserStrongComponents(
    mut inEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut varIdxs: &metamodelica::List<i32>,
    mut eqnIdxs: &metamodelica::List<i32>,
    mut traverserArgs: TraverseEqnAryFold,
) -> Result<TraverseEqnAryFold> {
    let mut traverserArgs: TraverseEqnAryFold = traverserArgs;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inVars.clone();
    let mut varCrefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut recursiveStrongComponentRun: bool;
    let mut isStateVarInvolved: bool;
    let mut isDiscrete: bool;
    varCrefLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (inVars.clone()).into_iter().cloned() {
            let __x = v.varName.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    isStateVarInvolved = !(Flags::getConfigBool(Flags::CAUSALIZE_DAE_MODE.clone())?)
        || List::any(
            &inVars,
            &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isStateVar(&__a0))
            },
        )?;
    isDiscrete = List::any(
        &inVars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::isVarDiscrete(&__a0))
        },
    )?;
    traverserArgs = ({
        let mut debug: bool = false;
        'mc: {
            let __mc_input = (
                inEqns.clone(),
                traverserArgs.recursiveStrongComponentRun.clone(),
                isStateVarInvolved,
                isDiscrete,
            );
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            (Deref @ metamodelica::ListNode::Cons { head: eq, tail: Deref @ metamodelica::ListNode::Nil }, false, false, _) => {
                                if !((List::all(&vars, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isCSEVar(&__a0)) })?)) { return Err("guard") }
                                let mut newResVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                                let mut new_eq: metamodelica::Ref<BackendDAE::Equation>;
                                let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                                newResVars = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
                    for mut v in (vars.clone()).into_iter().cloned() {
                                let __x = BackendVariable::setVarKind(v.clone(), openmodelica_backend_types::BackendDAE::VarKind::DAE_AUX_VAR)?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                new_eq = BackendEquation::setEquationAttributes(metamodelica::AsArg::as_arg(&eq), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                                traverserArgs.newDAEVars = BackendVariable::addNewVars(&newResVars, traverserArgs.newDAEVars.clone())?;
                                traverserArgs.newDAEEquations = BackendEquation::addList(&(list![new_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                                traverserArgs.systemVars = BackendVariable::removeCrefs(&varCrefLst, traverserArgs.systemVars.clone())?;
                                if debug {
                                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added solved aux vars. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                }
                                Ok((traverserArgs.clone(), traverserArgs.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: eq, tail: Deref @ metamodelica::ListNode::Nil }, false, _, true) => {
                        let mut new_eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                        new_eq = BackendEquation::setEquationAttributes(metamodelica::AsArg::as_arg(&eq), BackendDAE::EQ_ATTR_DEFAULT_DISCRETE.clone())?;
                        traverserArgs.newDAEVars = BackendVariable::addNewVars(&vars, traverserArgs.newDAEVars.clone())?;
                        traverserArgs.newDAEEquations = BackendEquation::addList(&(list![new_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                        if debug {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved discrete equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![new_eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        Ok((traverserArgs.clone(), traverserArgs.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, false, _, _) => {
                        let mut new_eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                        new_eq = BackendEquation::setEquationAttributes(metamodelica::AsArg::as_arg(&eq), BackendDAE::EQ_ATTR_DEFAULT_DISCRETE.clone())?;
                        traverserArgs.newDAEVars = BackendVariable::addNewVars(&vars, traverserArgs.newDAEVars.clone())?;
                        traverserArgs.newDAEEquations = BackendEquation::addList(&(list![new_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                        if debug {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved when equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![new_eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        Ok((traverserArgs.clone(), traverserArgs.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, false, false, _) => {
                        let mut var: metamodelica::Ref<BackendDAE::Var>;
                        let mut new_eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut eq = (*eq).clone();
                        let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                        let __pa0 = ::match_deref::match_deref! { match &(vars.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        var = metamodelica::Own::own(__pa0);
                        assign_variant_field!(eq => BackendDAE::Equation::EQUATION; exp = ExpressionSolve::solve(var_field!((*eq).exp, BackendDAE::Equation::EQUATION).clone(), var_field!((*eq).scalar, BackendDAE::Equation::EQUATION).clone(), Expression::crefExp(var.varName.clone())?, None)?.0);
                        new_eq = metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: var.varName.clone(), exp: var_field!((*eq).exp, BackendDAE::Equation::EQUATION).clone(), source: var_field!((*eq).source, BackendDAE::Equation::EQUATION).clone(), attr: var_field!((*eq).attr, BackendDAE::Equation::EQUATION).clone() });
                        new_eq = BackendEquation::setEquationAttributes(&new_eq, BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                        traverserArgs.newDAEVars = BackendVariable::addNewVars(&vars, traverserArgs.newDAEVars.clone())?;
                        traverserArgs.newDAEEquations = BackendEquation::addList(&(list![new_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                        if debug {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![new_eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        Ok((traverserArgs.clone(), traverserArgs.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, false, false, _) => {
                        let mut new_eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                        new_eq = BackendEquation::setEquationAttributes(metamodelica::AsArg::as_arg(&eq), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                        traverserArgs.newDAEVars = BackendVariable::addNewVars(&vars, traverserArgs.newDAEVars.clone())?;
                        traverserArgs.newDAEEquations = BackendEquation::addList(&(list![new_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                        if debug {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved complex equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![new_eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        Ok((traverserArgs.clone(), traverserArgs.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, false, false, _) => {
                        let mut new_eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                        new_eq = BackendEquation::setEquationAttributes(metamodelica::AsArg::as_arg(&eq), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                        traverserArgs.newDAEVars = BackendVariable::addNewVars(&vars, traverserArgs.newDAEVars.clone())?;
                        traverserArgs.newDAEEquations = BackendEquation::addList(&(list![new_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                        if debug {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved array equations. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![new_eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        Ok((traverserArgs.clone(), traverserArgs.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::ALGORITHM { alg, source, expand: crefExpand, .. }, tail: Deref @ metamodelica::ListNode::Nil }, false, false, _) => {
                        let mut new_eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                        let true = (CheckModel::isCrefListAlgorithmOutput(&varCrefLst, metamodelica::AsArg::as_arg(&alg), metamodelica::AsArg::as_arg(&source), crefExpand.clone())?) else { return Err("pattern mismatch") };
                        new_eq = BackendEquation::setEquationAttributes(metamodelica::AsArg::as_arg(&eq), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                        traverserArgs.newDAEVars = BackendVariable::addNewVars(&vars, traverserArgs.newDAEVars.clone())?;
                        traverserArgs.newDAEEquations = BackendEquation::addList(&(list![new_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                        if debug {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved algorithms. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![new_eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        Ok((traverserArgs.clone(), traverserArgs.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            (Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: exp, .. }, tail: Deref @ metamodelica::ListNode::Nil }, b1, b2, _) => {
                                if !((Expression::isCref(metamodelica::AsArg::as_arg(&exp)) && (b1.clone() || b2.clone()))) { return Err("guard") }
                                let mut newResEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                                let mut newResVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                                let mut newAuxVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                                let mut aux_eq: metamodelica::Ref<BackendDAE::Equation>;
                                let mut newnumResVars: i32;
                                let mut globalDAEData: BackendDAE::BackendDAEModeData;
                                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                                let mut newCref: metamodelica::Ref<DAE::ComponentRef>;
                                let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                                let mut eq = (*eq).clone();
                                let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                                globalDAEData = traverserArgs.globalDAEData.clone();
                                cref = Expression::expCref(metamodelica::AsArg::as_arg(&exp))?;
                                (newAuxVars, _) = BackendVariable::getVar(cref.clone(), &traverserArgs.systemVars)?;
                                crlst = ComponentReference::expandCref(&cref, true)?;
                                newAuxVars = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
                    let __thr_src0 = crlst.clone();
                    let mut __thr_it0 = (&__thr_src0).into_iter();
                    let __thr_src1 = newAuxVars.clone();
                    let mut __thr_it1 = (&__thr_src1).into_iter();
                    loop {
                                match (__thr_it0.next(), __thr_it1.next()) {
                                    (Some(cr), Some(v)) => {
                                        let __x = BackendVariable::copyVarNewName(ComponentReference::crefPrefixAux(cr.clone()), v.clone());
                                        __acc = cons(__x, __acc);
                                    }
                                    (None, None) => break,
                                    _ => return Err("threaded for: ranges of unequal length"),
                                }
                    }
                    __acc.reverse()
                });
                                newAuxVars = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
                    for mut v in (newAuxVars.clone()).into_iter().cloned() {
                                let __x = BackendVariable::setVarKind(v.clone(), openmodelica_backend_types::BackendDAE::VarKind::DAE_AUX_VAR)?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                traverserArgs.newDAEVars = BackendVariable::addNewVars(&newAuxVars, traverserArgs.newDAEVars.clone())?;
                                newCref = ComponentReference::crefPrefixAux(cref.clone());
                                assign_variant_field!(eq => BackendDAE::Equation::ARRAY_EQUATION; left = Expression::crefExp(newCref.clone())?);
                                aux_eq = eq.clone();
                                aux_eq = BackendEquation::setEquationAttributes(&aux_eq, BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                                traverserArgs.newDAEEquations = BackendEquation::addList(&(list![aux_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                                globalDAEData = traverserArgs.globalDAEData.clone();
                                assign_variant_field!(eq => BackendDAE::Equation::ARRAY_EQUATION; right = Expression::crefToExp(cref.clone())?);
                                newResEqns = BackendEquation::equationToScalarResidualForm(eq.clone(), &traverserArgs.functionTree)?;
                                (newResEqns, newResVars, newnumResVars) = BackendEquation::convertResidualsIntoSolvedEquations(&newResEqns, &(literal!("$DAEres")), globalDAEData.numResVars.clone(), true)?;
                                globalDAEData.numResVars = newnumResVars;
                                newResEqns = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
                    for mut e in (newResEqns.clone()).into_iter().cloned() {
                                let __x = BackendEquation::setEquationAttributes(&(e.clone()), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone())?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                traverserArgs.newDAEVars = BackendVariable::addNewVars(&newResVars, traverserArgs.newDAEVars.clone())?;
                                traverserArgs.newDAEEquations = BackendEquation::addList(&newResEqns, traverserArgs.newDAEEquations.clone())?;
                                globalDAEData = addVarsGlobalData(globalDAEData.clone(), vars.clone())?;
                                traverserArgs.globalDAEData = globalDAEData.clone();
                                if debug {
                                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added residual array equation\n")); __mm_s.push_str(&*BackendDump::varListString(&newResVars, &(literal!("")))?); __mm_s.push_str(&*literal!("states:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eqs:\n")); __mm_s.push_str(&*BackendDump::equationListString(&newResEqns, &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                }
                                Ok((traverserArgs.clone(), traverserArgs.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            (Deref @ metamodelica::ListNode::Cons { head: eq, tail: Deref @ metamodelica::ListNode::Nil }, b1, b2, _) => {
                                if !((b1.clone() || b2.clone())) { return Err("guard") }
                                let mut newResEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                                let mut newResVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                                let mut newnumResVars: i32;
                                let mut globalDAEData: BackendDAE::BackendDAEModeData;
                                let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                                globalDAEData = traverserArgs.globalDAEData.clone();
                                newResEqns = BackendEquation::equationToScalarResidualForm(eq.clone(), &traverserArgs.functionTree)?;
                                (newResEqns, newResVars, newnumResVars) = BackendEquation::convertResidualsIntoSolvedEquations(&newResEqns, &(literal!("$DAEres")), globalDAEData.numResVars.clone(), true)?;
                                globalDAEData.numResVars = newnumResVars;
                                newResEqns = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
                    for mut e in (newResEqns.clone()).into_iter().cloned() {
                                let __x = BackendEquation::setEquationAttributes(&(e.clone()), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone())?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                traverserArgs.newDAEVars = BackendVariable::addNewVars(&newResVars, traverserArgs.newDAEVars.clone())?;
                                traverserArgs.newDAEEquations = BackendEquation::addList(&newResEqns, traverserArgs.newDAEEquations.clone())?;
                                globalDAEData = addVarsGlobalData(globalDAEData.clone(), vars.clone())?;
                                traverserArgs.globalDAEData = globalDAEData.clone();
                                if debug {
                                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added strong component or state eqns\n")); __mm_s.push_str(&*BackendDump::varListString(&newResVars, &(literal!("")))?); __mm_s.push_str(&*literal!("states:\n")); __mm_s.push_str(&*BackendDump::varListString(&vars, &(literal!("")))?); __mm_s.push_str(&*literal!("eqs:\n")); __mm_s.push_str(&*BackendDump::equationListString(&newResEqns, &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                }
                                Ok((traverserArgs.clone(), traverserArgs.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            (Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: exp, .. }, tail: Deref @ metamodelica::ListNode::Nil }, _, _, _) => {
                                if !((Expression::isCref(metamodelica::AsArg::as_arg(&exp)))) { return Err("guard") }
                                let mut newResEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                                let mut newResVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                                let mut newAuxVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                                let mut aux_eq: metamodelica::Ref<BackendDAE::Equation>;
                                let mut newnumResVars: i32;
                                let mut globalDAEData: BackendDAE::BackendDAEModeData;
                                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                                let mut newCref: metamodelica::Ref<DAE::ComponentRef>;
                                let mut eq = (*eq).clone();
                                let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                                if debug {
                                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("case: Complex: ")); __mm_s.push_str(&*BackendDump::equationListString(inEqns, &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                }
                                cref = Expression::expCref(metamodelica::AsArg::as_arg(&exp))?;
                                newAuxVars = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
                    let __thr_src0 = varCrefLst.clone();
                    let mut __thr_it0 = (&__thr_src0).into_iter();
                    let __thr_src1 = vars.clone();
                    let mut __thr_it1 = (&__thr_src1).into_iter();
                    loop {
                                match (__thr_it0.next(), __thr_it1.next()) {
                                    (Some(cr), Some(v)) => {
                                        let __x = BackendVariable::copyVarNewName(ComponentReference::crefPrefixAux(cr.clone()), v.clone());
                                        __acc = cons(__x, __acc);
                                    }
                                    (None, None) => break,
                                    _ => return Err("threaded for: ranges of unequal length"),
                                }
                    }
                    __acc.reverse()
                });
                                newAuxVars = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
                    for mut v in (newAuxVars.clone()).into_iter().cloned() {
                                let __x = BackendVariable::setVarKind(v.clone(), openmodelica_backend_types::BackendDAE::VarKind::DAE_AUX_VAR)?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                traverserArgs.newDAEVars = BackendVariable::addNewVars(&newAuxVars, traverserArgs.newDAEVars.clone())?;
                                newCref = ComponentReference::crefPrefixAux(cref.clone());
                                assign_variant_field!(eq => BackendDAE::Equation::COMPLEX_EQUATION; left = Expression::crefToExp(newCref.clone())?);
                                aux_eq = eq.clone();
                                aux_eq = BackendEquation::setEquationAttributes(&aux_eq, BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                                traverserArgs.newDAEEquations = BackendEquation::addList(&(list![aux_eq.clone()]), traverserArgs.newDAEEquations.clone())?;
                                globalDAEData = traverserArgs.globalDAEData.clone();
                                assign_variant_field!(eq => BackendDAE::Equation::COMPLEX_EQUATION; right = Expression::crefToExp(cref.clone())?);
                                newResEqns = BackendEquation::equationToScalarResidualForm(eq.clone(), &traverserArgs.functionTree)?;
                                (newResEqns, newResVars, newnumResVars) = BackendEquation::convertResidualsIntoSolvedEquations(&newResEqns, &(literal!("$DAEres")), globalDAEData.numResVars.clone(), true)?;
                                globalDAEData.numResVars = newnumResVars;
                                newResEqns = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
                    for mut e in (newResEqns.clone()).into_iter().cloned() {
                                let __x = BackendEquation::setEquationAttributes(&(e.clone()), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone())?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                traverserArgs.newDAEVars = BackendVariable::addNewVars(&newResVars, traverserArgs.newDAEVars.clone())?;
                                traverserArgs.newDAEEquations = BackendEquation::addList(&newResEqns, traverserArgs.newDAEEquations.clone())?;
                                globalDAEData = addVarsGlobalData(globalDAEData.clone(), vars.clone())?;
                                traverserArgs.globalDAEData = globalDAEData.clone();
                                if debug {
                                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added complex residual equation with aux variables. Res-vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&newResVars, &(literal!("")))?); __mm_s.push_str(&*literal!("eqs:\n")); __mm_s.push_str(&*BackendDump::equationListString(&newResEqns, &(literal!("")))?); __mm_s.push_str(&*literal!("aux vars:\n")); __mm_s.push_str(&*BackendDump::varListString(&newAuxVars, &(literal!("")))?); __mm_s.push_str(&*literal!("aux eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(&(list![aux_eq.clone()]), &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                }
                                Ok((traverserArgs.clone(), traverserArgs.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (_, false, _, _) => {
                        let mut newAuxVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                        let mut discVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                        let mut contVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                        let mut size: i32;
                        let mut discEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut contEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                        (discVars, contVars) = List::splitOnTrue(&inVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isVarDiscrete(&__a0)) })?;
                        (discEqns, contEqns) = getDiscAndContEqns(&inVars, inEqns, &discVars, &contVars, traverserArgs.shared.functionTree.clone(), BackendDAEUtil::isInitializationDAE(&traverserArgs.shared))?;
                        for mut e in &*discEqns {
                            size = BackendEquation::equationSize(metamodelica::AsArg::as_arg(&e))?;
                            newAuxVars = List::firstN(discVars.clone(), size)?;
                            traverserArgs = traverserStrongComponents(&(list![e.clone()]), newAuxVars.clone(), &(metamodelica::nil()), &(metamodelica::nil()), traverserArgs.clone())?;
                            discVars = List::stripN(discVars.clone(), size)?;
                        }
                        for mut e in &*contEqns {
                            size = BackendEquation::equationSize(metamodelica::AsArg::as_arg(&e))?;
                            newAuxVars = List::firstN(contVars.clone(), size)?;
                            traverserArgs.recursiveStrongComponentRun = true;
                            traverserArgs = traverserStrongComponents(&(list![e.clone()]), newAuxVars.clone(), &(metamodelica::nil()), &(metamodelica::nil()), traverserArgs.clone())?;
                            traverserArgs.recursiveStrongComponentRun = false;
                            contVars = List::stripN(contVars.clone(), size)?;
                        }
                        Ok((traverserArgs.clone(), traverserArgs.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                traverserArgs = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEMode.traverserStrongComponents failed on equation:\n")); __mm_s.push_str(&*BackendDump::equationListString(inEqns, &(literal!("")))?); __mm_s.push_str(&*literal!("\nVariables:\n")); __mm_s.push_str(&*BackendDump::varListString(&inVars, &(literal!("")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/DAEMode.mo"))?;
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
    Ok(traverserArgs)
}

fn getDiscAndContEqns(
    mut inAllVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inAllEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inDiscVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inContVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut isInitial: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut discEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut contEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut adjMatrix: metamodelica::Array<metamodelica::List<i32>>;
    let mut varsIndex: metamodelica::List<i32>;
    let mut eqnIndex: metamodelica::List<i32>;
    let mut assignVarEqn: metamodelica::Array<i32>;
    let mut assignEqnVar: metamodelica::Array<i32>;
    let mut mapEqnScalarArray: metamodelica::Array<i32>;
    let debug: bool = false;
    match '__try0: {
        syst = BackendDAEUtil::createEqSystem(
            unwrap_break_err!(BackendVariable::listVar1(inAllVars), '__try0),
            unwrap_break_err!(BackendEquation::listEquation(inAllEqns), '__try0),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        if debug {
            unwrap_break_err!(BackendDump::printEqSystem(syst.clone()), '__try0);
        }
        (adjMatrix, _, _, mapEqnScalarArray) = unwrap_break_err!(BackendDAEUtil::adjacencyMatrixScalar(&syst, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(functionTree.clone()), isInitial), '__try0);
        if debug {
            unwrap_break_err!(BackendDump::dumpAdjacencyMatrix(adjMatrix.clone()), '__try0);
        }
        let (__pa1, __pa2, true, _, _) = (unwrap_break_err!(Matching::RegularMatching(adjMatrix.clone(), unwrap_break_err!(BackendDAEUtil::systemSize(&syst), '__try0), unwrap_break_err!(BackendDAEUtil::systemSize(&syst), '__try0)), '__try0))
        else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        assignVarEqn = metamodelica::Own::own(__pa1);
        assignEqnVar = metamodelica::Own::own(__pa2);
        if debug {
            unwrap_break_err!(BackendDump::dumpMatching(assignVarEqn.clone()), '__try0);
        }
        varsIndex = BackendVariable::getVarIndexFromVars(inDiscVars, &syst.orderedVars);
        if debug {
            metamodelica::print(literal!("discVarsIndex: "));
            unwrap_break_err!(BackendDump::dumpAdjacencyRow(&varsIndex), '__try0);
        }
        eqnIndex =
            unwrap_break_err!(List::map1(varsIndex.clone(), &Array::getIndexFirst, assignVarEqn.clone()), '__try0);
        if debug {
            metamodelica::print(literal!("discEqnIndex: "));
            unwrap_break_err!(BackendDump::dumpAdjacencyRow(&eqnIndex), '__try0);
        }
        eqnIndex = List::unique(
            &({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (eqnIndex.clone()).into_iter().cloned() {
                    let __x = ({
                        let __elt = (*unwrap_break_err!(metamodelica::index_checked(&mapEqnScalarArray.borrow(), i.clone()), '__try0)).clone();
                        __elt
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        );
        discEqns = unwrap_break_err!(BackendEquation::getList(eqnIndex.clone(), syst.orderedEqs.clone()), '__try0);
        if debug {
            unwrap_break_err!(BackendDump::equationListString(&discEqns, &(literal!("Discrete Equations"))), '__try0);
        }
        varsIndex = BackendVariable::getVarIndexFromVars(inContVars, &syst.orderedVars);
        eqnIndex =
            unwrap_break_err!(List::map1(varsIndex.clone(), &Array::getIndexFirst, assignVarEqn.clone()), '__try0);
        eqnIndex = List::unique(
            &({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (eqnIndex.clone()).into_iter().cloned() {
                    let __x = ({
                        let __elt = (*unwrap_break_err!(metamodelica::index_checked(&mapEqnScalarArray.borrow(), i.clone()), '__try0)).clone();
                        __elt
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        );
        if debug {
            metamodelica::print(literal!("contEqnIndex: "));
            unwrap_break_err!(BackendDump::dumpAdjacencyRow(&eqnIndex), '__try0);
        }
        contEqns = unwrap_break_err!(BackendEquation::getList(eqnIndex.clone(), syst.orderedEqs.clone()), '__try0);
        if debug {
            unwrap_break_err!(BackendDump::equationListString(&contEqns, &(literal!("Continuous Equations"))), '__try0);
        }
        Ok::<_, &'static str>((
            adjMatrix.clone(),
            assignEqnVar.clone(),
            assignVarEqn.clone(),
            contEqns.clone(),
            discEqns.clone(),
            eqnIndex.clone(),
            mapEqnScalarArray.clone(),
            syst.clone(),
            varsIndex.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            adjMatrix = __try0_o0;
            assignEqnVar = __try0_o1;
            assignVarEqn = __try0_o2;
            contEqns = __try0_o3;
            discEqns = __try0_o4;
            eqnIndex = __try0_o5;
            mapEqnScalarArray = __try0_o6;
            syst = __try0_o7;
            varsIndex = __try0_o8;
        }
        Err(__try0_err) => {
            return Err(__try0_err);
        }
    }
    Ok((discEqns, contEqns))
}

fn addVarsGlobalData(
    mut globalDAEData: BackendDAE::BackendDAEModeData,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<BackendDAE::BackendDAEModeData> {
    let mut globalDAEData: BackendDAE::BackendDAEModeData = globalDAEData;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    vars = List::filterOnTrue(
        inVars.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isNonStateVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (vars).into_iter().cloned() {
            let __x =
                BackendVariable::setVarKind(v.clone(), openmodelica_backend_types::BackendDAE::VarKind::ALG_STATE)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    globalDAEData.algStateVars = listAppend(vars, globalDAEData.algStateVars.clone());
    globalDAEData.stateVars = listAppend(
        List::filterOnTrue(
            inVars,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::isStateVar(&__a0))
                },
            )
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
        )?,
        globalDAEData.stateVars.clone(),
    );
    Ok(globalDAEData)
}

fn setNonStateVarAlgState(
    mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = varList;
    for mut v in &*varList {
        let mut v = v.clone();
        v = (match &*v {
            BackendDAE::Var {
                varKind: BackendDAE::VarKind::STATE { .. },
                ..
            } => v,
            BackendDAE::Var {
                varKind: BackendDAE::VarKind::VARIABLE { .. },
                ..
            } => {
                v = BackendVariable::setVarKind(v, openmodelica_backend_types::BackendDAE::VarKind::ALG_STATE)?;
                v
            }
            _ => return Err("fail"),
        });
    }
    varList = varList.reverse();
    Ok(varList)
}
