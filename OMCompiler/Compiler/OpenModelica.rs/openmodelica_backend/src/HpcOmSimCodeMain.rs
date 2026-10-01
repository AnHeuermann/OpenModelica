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
use crate::HpcOmEqSystems;
use crate::HpcOmMemory;
use crate::HpcOmScheduler;
use crate::HpcOmTaskGraph;
use crate::SimCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_util::SimCodeCodegenUtil;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub fn createSimCode(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE_lambda0: Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    mut inRemovedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut filenamePrefix: ArcStr,
    mut inString11: ArcStr,
    mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    mut externalFunctionIncludes: metamodelica::List<ArcStr>,
    mut includeDirs: metamodelica::List<ArcStr>,
    mut libs: metamodelica::List<ArcStr>,
    mut libPaths: metamodelica::List<ArcStr>,
    mut program: Absyn::Program,
    mut simSettingsOpt: Option<SimCode::SimulationSettings>,
    mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>,
    mut literals: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<metamodelica::Ref<SimCode::SimCode>> {
    let mut simCode: metamodelica::Ref<SimCode::SimCode> =
        <metamodelica::Ref<SimCode::SimCode> as ::std::default::Default>::default();
    simCode = 'mc: {
        let __mc_input = &*inBackendDAE;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { .. } => {
                    let mut lastEqMappingIdx: i32;
                    let mut equationSccMapping: metamodelica::List<(i32, i32)>;
                    let mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
                    let mut daeSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
                    let mut simeqCompMapping: metamodelica::Array<i32>;
                    let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphDae: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphOde: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphData: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataDae: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOde: HpcOmTaskGraph::TaskGraphMeta;
                    let mut fileName: ArcStr;
                    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    let mut partData: SimCode::PartitionData;
                    let mut simCode: metamodelica::Ref<SimCode::SimCode> = simCode.clone();
                    let true = (Flags::isSet(Flags::MULTIRATE_PARTITION.clone())?) else { return Err("pattern mismatch") };
                    metamodelica::print(literal!("DO MULTIRATE\n"));
                    let (__pa0, (__pa1, __pa2)) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE, inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args, false, &(literal!("")), literal!(""), &(metamodelica::nil()))?;
                    simCode = metamodelica::Own::own(__pa0);
                    lastEqMappingIdx = metamodelica::Own::own(__pa1);
                    equationSccMapping = metamodelica::Own::own(__pa2);
                    (simeqCompMapping, sccSimEqMapping, daeSccSimEqMapping) = HpcOmTaskGraph::setUpHpcOmMapping(&inBackendDAE, &simCode, lastEqMappingIdx, equationSccMapping.clone())?;
                    ExecStat::execStat(&(literal!("hpcom setup")))?;
                    (taskGraph, taskGraphData) = HpcOmTaskGraph::createTaskGraph(&inBackendDAE, false)?;
                    taskGraphDae = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataDae = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone());
                    (taskGraphDae, taskGraphDataDae) = HpcOmTaskGraph::appendRemovedEquations(inBackendDAE.clone(), taskGraphDae.clone(), taskGraphDataDae.clone());
                    taskGraphDataDae = HpcOmTaskGraph::createCosts(&inBackendDAE, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("_eqs_prof")); ArcStr::from(__mm_s) }), simeqCompMapping.clone(), taskGraphDataDae.clone())?;
                    taskGraphData = HpcOmTaskGraph::copyCosts(taskGraphDataDae.clone(), taskGraphData.clone())?;
                    taskGraphOde = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataOde = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone());
                    (taskGraphOde, taskGraphDataOde) = HpcOmTaskGraph::getOdeSystem(taskGraphOde.clone(), taskGraphDataOde.clone(), &inBackendDAE)?;
                    fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("_ODE.graphml")); ArcStr::from(__mm_s) };
                    schedulerInfo = arrayCreate(metamodelica::arrayLength(taskGraphOde.clone()), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOde.clone(), taskGraphDataOde.clone(), fileName.clone(), literal!(""), metamodelica::nil(), metamodelica::nil(), daeSccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    partData = HpcOmTaskGraph::multirate_partitioning(taskGraphOde.clone(), &taskGraphDataOde, &inBackendDAE, &simCode, sccSimEqMapping.clone())?;
                    assign_field!(simCode.partitionData = partData.clone());
                    Ok((simCode.clone(), simCode.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            simCode = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { eqs, .. } => {
                    let mut lastEqMappingIdx: i32;
                    let mut equationSccMapping: metamodelica::List<(i32, i32)>;
                    let mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
                    let mut daeSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
                    let mut simeqCompMapping: metamodelica::Array<i32>;
                    let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphDae: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphOde: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphZeroFuncs: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphOdeSimplified: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphDaeSimplified: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphZeroFuncSimplified: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphOdeScheduled: metamodelica::Array<metamodelica::List<i32>>;
                    let mut taskGraphData: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataDae: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOde: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataZeroFuncs: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOdeSimplified: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataDaeSimplified: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataZeroFuncSimplified: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOdeScheduled: HpcOmTaskGraph::TaskGraphMeta;
                    let mut fileName: ArcStr;
                    let mut numProc: i32;
                    let mut criticalPaths: metamodelica::List<metamodelica::List<i32>>;
                    let mut criticalPathsWoC: metamodelica::List<metamodelica::List<i32>>;
                    let mut cpCosts: metamodelica::Real;
                    let mut cpCostsWoC: metamodelica::Real;
                    let mut scheduledTasksOde: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut scheduledTasksDae: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut scheduledTasksZeroFunc: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut zeroFuncsSimEqIdc: metamodelica::List<i32>;
                    let mut taskGraphMetaValid: bool;
                    let mut criticalPathInfo: ArcStr;
                    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    let mut scheduleOde: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    let mut scheduleDae: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    let mut scheduleZeroFunc: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    let mut graphCosts: metamodelica::Real;
                    let mut graphOps: i32;
                    let mut optTmpMemoryMap: Option<HpcOmSimCode::MemoryMap>;
                    let mut simVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>;
                    let mut varToArrayIndexMapping: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::List<i32>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
                    let mut varToIndexMapping: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
                    let mut simCode: metamodelica::Ref<SimCode::SimCode> = simCode.clone();
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { return Err("pattern mismatch") };
                    System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT_HPCOM_MODULES.clone())?;
                    let (__pa0, (__pa1, __pa2)) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE, inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args, false, &(literal!("")), literal!(""), &(metamodelica::nil()))?;
                    simCode = metamodelica::Own::own(__pa0);
                    lastEqMappingIdx = metamodelica::Own::own(__pa1);
                    equationSccMapping = metamodelica::Own::own(__pa2);
                    simVarMapping = SimCodeUtil::getSimVarMappingOfBackendMapping(simCode.backendMapping.clone());
                    (simeqCompMapping, sccSimEqMapping, daeSccSimEqMapping) = HpcOmTaskGraph::setUpHpcOmMapping(&inBackendDAE, &simCode, lastEqMappingIdx, equationSccMapping.clone())?;
                    ExecStat::execStat(&(literal!("hpcom setup")))?;
                    (taskGraph, taskGraphData) = HpcOmTaskGraph::createTaskGraph(&inBackendDAE, false)?;
                    taskGraphDae = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataDae = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone());
                    (taskGraphDae, taskGraphDataDae) = HpcOmTaskGraph::appendRemovedEquations(inBackendDAE.clone(), taskGraphDae.clone(), taskGraphDataDae.clone());
                    schedulerInfo = arrayCreate(metamodelica::arrayLength(taskGraphDae.clone()), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    ExecStat::execStat(&(literal!("hpcom create DAE TaskGraph")))?;
                    checkTaskGraphMetaConsistency(taskGraphDae.clone(), taskGraphDataDae.clone(), &(literal!("DAE system")));
                    ExecStat::execStat(&(literal!("hpcom validate DAE TaskGraph")))?;
                    taskGraphDataDae = HpcOmTaskGraph::createCosts(&inBackendDAE, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("_eqs_prof")); ArcStr::from(__mm_s) }), simeqCompMapping.clone(), taskGraphDataDae.clone())?;
                    taskGraphData = HpcOmTaskGraph::copyCosts(taskGraphDataDae.clone(), taskGraphData.clone())?;
                    ExecStat::execStat(&(literal!("hpcom create costs")))?;
                    taskGraphOde = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataOde = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone());
                    (taskGraphOde, taskGraphDataOde) = HpcOmTaskGraph::getOdeSystem(taskGraphOde.clone(), taskGraphDataOde.clone(), &inBackendDAE)?;
                    ExecStat::execStat(&(literal!("hpcom create ODE TaskGraph")))?;
                    taskGraphMetaValid = HpcOmTaskGraph::validateTaskGraphMeta(taskGraphDataOde.clone(), &inBackendDAE);
                    if boolNot(taskGraphMetaValid) {
                        metamodelica::print(literal!("TaskgraphMeta ODE invalid\n"));
                    }
                    ExecStat::execStat(&(literal!("hpcom validate ODE TaskGraph")))?;
                    taskGraphDataDae = HpcOmTaskGraph::markSystemComponents(taskGraphOde.clone(), taskGraphDataOde.clone(), (false, true, false), taskGraphDataDae.clone())?;
                    taskGraphZeroFuncs = metamodelica::arrayFromVec(taskGraphDae.clone().borrow().clone());
                    taskGraphDataZeroFuncs = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphDataDae.clone());
                    zeroFuncsSimEqIdc = List::map(simCode.equationsForZeroCrossings.clone(), &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>| SimCodeCodegenUtil::simEqSystemIndex(&__a0))?;
                    (taskGraphZeroFuncs, taskGraphDataZeroFuncs) = HpcOmTaskGraph::getZeroFuncsSystem(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), &inBackendDAE, metamodelica::arrayLength(daeSccSimEqMapping.clone()), &zeroFuncsSimEqIdc, simeqCompMapping.clone())?;
                    fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("_ZeroFuncs.graphml")); ArcStr::from(__mm_s) };
                    schedulerInfo = arrayCreate(metamodelica::arrayLength(taskGraphZeroFuncs.clone()), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), fileName.clone(), literal!(""), metamodelica::nil(), metamodelica::nil(), daeSccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat(&(literal!("hpcom create and dump zeroFuncs TaskGraph")))?;
                    taskGraphDataDae = HpcOmTaskGraph::markSystemComponents(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), (true, false, false), taskGraphDataDae.clone())?;
                    checkTaskGraphMetaConsistency(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), &(literal!("ZeroFunc system")));
                    checkEquationCount(taskGraphDataZeroFuncs.clone(), &(literal!("ZeroFunc system")), ((zeroFuncsSimEqIdc).len() as i32), sccSimEqMapping.clone())?;
                    fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("DAE.graphml")); ArcStr::from(__mm_s) };
                    schedulerInfo = arrayCreate(metamodelica::arrayLength(taskGraphDae.clone()), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphDae.clone(), taskGraphDataDae.clone(), fileName.clone(), literal!(""), metamodelica::nil(), metamodelica::nil(), daeSccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat(&(literal!("hpcom dump DAE TaskGraph")))?;
                    let ((__pa3, __pa4), (__pa5, __pa6)) = HpcOmTaskGraph::getCriticalPaths(taskGraphOde.clone(), taskGraphDataOde.clone());
                    criticalPaths = metamodelica::Own::own(__pa3);
                    cpCosts = metamodelica::Own::own(__pa4);
                    criticalPathsWoC = metamodelica::Own::own(__pa5);
                    cpCostsWoC = metamodelica::Own::own(__pa6);
                    criticalPathInfo = HpcOmTaskGraph::dumpCriticalPathInfo(&((criticalPaths.clone(), cpCosts)), &((criticalPathsWoC.clone(), cpCostsWoC)))?;
                    (graphOps, graphCosts) = HpcOmTaskGraph::sumUpExeCosts(taskGraphOde.clone(), &taskGraphDataOde)?;
                    graphCosts = HpcOmTaskGraph::roundReal(graphCosts, 2)?;
                    criticalPathInfo = { let mut __mm_s = String::new(); __mm_s.push_str(&*criticalPathInfo); __mm_s.push_str(&*literal!(" sum: (")); __mm_s.push_str(&*realString(graphCosts)); __mm_s.push_str(&*literal!(" ; ")); __mm_s.push_str(&*intString(graphOps)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("ODE.graphml")); ArcStr::from(__mm_s) };
                    schedulerInfo = arrayCreate(metamodelica::arrayLength(taskGraphOde.clone()), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    ExecStat::execStat(&(literal!("hpcom assign levels / get crit. path")))?;
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOde.clone(), taskGraphDataOde.clone(), fileName.clone(), criticalPathInfo.clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(&((criticalPaths).head().cloned()?)), HpcOmTaskGraph::convertNodeListToEdgeTuples(&((criticalPathsWoC).head().cloned()?)), sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: true, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat(&(literal!("hpcom dump ODE TaskGraph")))?;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        metamodelica::print(literal!("Critical Path successfully calculated\n"));
                    }
                    scheduledTasksDae = metamodelica::nil();
                    (scheduledTasksOde, _) = HpcOmEqSystems::parallelizeTornSystems(taskGraphOde.clone(), &taskGraphDataOde, sccSimEqMapping.clone(), simVarMapping.clone(), &inBackendDAE);
                    scheduledTasksZeroFunc = metamodelica::nil();
                    (taskGraphDaeSimplified, taskGraphDataDaeSimplified) = applyGRS(taskGraphDae.clone(), taskGraphDataDae.clone())?;
                    (taskGraphOdeSimplified, taskGraphDataOdeSimplified) = applyGRS(taskGraphOde.clone(), taskGraphDataOde.clone())?;
                    (taskGraphZeroFuncSimplified, taskGraphDataZeroFuncSimplified) = applyGRS(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone())?;
                    ExecStat::execStat(&(literal!("hpcom GRS")))?;
                    fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("ODE_merged.graphml")); ArcStr::from(__mm_s) };
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOdeSimplified.clone(), taskGraphDataOdeSimplified.clone(), fileName.clone(), criticalPathInfo.clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(&((criticalPaths).head().cloned()?)), HpcOmTaskGraph::convertNodeListToEdgeTuples(&((criticalPathsWoC).head().cloned()?)), sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: true, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat(&(literal!("hpcom dump simplified TaskGraph")))?;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Filter successfully applied. Merged ")); __mm_s.push_str(&*intString(intSub(metamodelica::arrayLength(taskGraphOde.clone()), metamodelica::arrayLength(taskGraphOdeSimplified.clone())))); __mm_s.push_str(&*literal!(" tasks.\n")); ArcStr::from(__mm_s) });
                    }
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    (numProc, _) = setNumProc(numProc, cpCostsWoC, taskGraphDataOde.clone())?;
                    (scheduleDae, simCode, _, _, sccSimEqMapping) = createSchedule(taskGraphDaeSimplified.clone(), taskGraphDataDaeSimplified.clone(), daeSccSimEqMapping.clone(), simVarMapping.clone(), &filenamePrefix, numProc, numProc, simCode.clone(), scheduledTasksDae.clone(), &(literal!("DAE system")), Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?)?;
                    (scheduleOde, simCode, taskGraphOdeScheduled, taskGraphDataOdeScheduled, sccSimEqMapping) = createSchedule(taskGraphOdeSimplified.clone(), taskGraphDataOdeSimplified.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), &filenamePrefix, numProc, numProc, simCode.clone(), scheduledTasksOde.clone(), &(literal!("ODE system")), Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?)?;
                    (scheduleZeroFunc, simCode, _, _, sccSimEqMapping) = createSchedule(taskGraphZeroFuncSimplified.clone(), taskGraphDataZeroFuncSimplified.clone(), daeSccSimEqMapping.clone(), simVarMapping.clone(), &filenamePrefix, numProc, numProc, simCode.clone(), scheduledTasksZeroFunc.clone(), &(literal!("ZeroFunc system")), Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?)?;
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    criticalPathInfo = HpcOmScheduler::analyseScheduledTaskGraph(scheduleOde.clone(), numProc, taskGraphOdeScheduled.clone(), taskGraphDataOdeScheduled.clone(), &(literal!("ODE system")));
                    schedulerInfo = HpcOmScheduler::convertScheduleStrucToInfo(&scheduleOde, metamodelica::arrayLength(taskGraphOdeScheduled.clone()))?;
                    ExecStat::execStat(&(literal!("hpcom create schedule")))?;
                    fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix); __mm_s.push_str(&*literal!("ODE_schedule.graphml")); ArcStr::from(__mm_s) };
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOdeScheduled.clone(), taskGraphDataOdeScheduled.clone(), fileName.clone(), criticalPathInfo.clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(&((criticalPaths).head().cloned()?)), HpcOmTaskGraph::convertNodeListToEdgeTuples(&((criticalPathsWoC).head().cloned()?)), sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: true, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat(&(literal!("hpcom dump schedule TaskGraph")))?;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        metamodelica::print(literal!("Schedule created\n"));
                    }
                    System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT_HPCOM_MODULES.clone())?;
                    checkOdeSystemSize(taskGraphDataOdeScheduled.clone(), simCode.odeEquations.clone(), sccSimEqMapping.clone())?;
                    ExecStat::execStat(&(literal!("hpcom check ODE system size")))?;
                    (optTmpMemoryMap, varToArrayIndexMapping, varToIndexMapping) = HpcOmMemory::createMemoryMap(simCode.modelInfo.clone(), simCode.varToArrayIndexMapping.clone(), simCode.varToIndexMapping.clone(), taskGraphOdeSimplified.clone(), AdjacencyMatrix::transposeAdjacencyMatrix(taskGraphOdeSimplified.clone(), metamodelica::arrayLength(taskGraphOdeSimplified.clone()))?, taskGraphDataOdeSimplified.clone(), eqs.clone(), &filenamePrefix, schedulerInfo.clone(), &scheduleOde, sccSimEqMapping.clone(), &criticalPaths, &criticalPathsWoC, criticalPathInfo.clone(), numProc, &((HpcOmTaskGraph::getSystemComponents(&inBackendDAE)?).0), BackendDAEUtil::isInitializationDAE(&inBackendDAE.shared))?;
                    ExecStat::execStat(&(literal!("hpcom create memory map")))?;
                    assign_field!(
                        simCode.varToArrayIndexMapping = varToArrayIndexMapping.clone(),
                        simCode.varToIndexMapping = varToIndexMapping.clone(),
                        simCode.hpcomData = HpcOmSimCode::HpcOmData { schedules: Some((scheduleOde.clone(), scheduleDae.clone(), scheduleZeroFunc.clone())), hpcOmMemory: optTmpMemoryMap.clone() }
                    );
                    ExecStat::execStat(&(literal!("hpcom other")))?;
                    metamodelica::print(literal!("HpcOm is still under construction.\n"));
                    Ok((simCode.clone(), simCode.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            simCode = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("function createSimCode failed.")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(simCode)
}

fn createAndExportInitialSystemTaskGraph(
    mut iInitDae: Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    mut iFileNamePrefix: &ArcStr,
) -> Result<()> {
    let mut initDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut tmpTaskGraph: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut fileName: ArcStr;
    let mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let () = (::match_deref::match_deref! { match &(iInitDae) {
        Some(__esc_initDAE) => {
            initDAE = (*__esc_initDAE).clone();
            (tmpTaskGraph, tmpTaskGraphMeta) = HpcOmTaskGraph::createTaskGraph(metamodelica::AsArg::as_arg(&initDAE), false)?;
            fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFileNamePrefix); __mm_s.push_str(&*literal!("_init.graphml")); ArcStr::from(__mm_s) };
            schedulerInfo = arrayCreate(metamodelica::arrayLength(tmpTaskGraph.clone()), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            sccSimEqMapping = arrayCreate(metamodelica::arrayLength(tmpTaskGraph.clone()), metamodelica::nil());
            HpcOmTaskGraph::dumpAsGraphMLSccLevel(tmpTaskGraph.clone(), tmpTaskGraphMeta, fileName, literal!(""), metamodelica::nil(), metamodelica::nil(), sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn setNumProc(
    mut numProcFlag: i32,
    mut cpCosts: metamodelica::Real,
    mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(i32, bool)> {
    let mut numProcOut: i32;
    let mut numFixed: bool;
    (numProcOut, numFixed) = (match numProcFlag {
        0 => {
            let mut numProcSys: i32;
            let mut numProc: i32;
            let mut numProcSched: i32;
            let mut serCosts: metamodelica::Real;
            let mut maxSpeedUp: metamodelica::Real;
            let mut string1: ArcStr;
            let mut string2: ArcStr;
            serCosts = HpcOmScheduler::getSerialExecutionTime(taskGraphMetaIn)?;
            if realNe(serCosts, metamodelica::OrderedFloat(0.0_f64)) {
                maxSpeedUp = realDiv(serCosts, cpCosts);
                numProcSched = (((maxSpeedUp) + (metamodelica::OrderedFloat(1.0_f64))).0.floor() as i32);
                numProcSys = System::numProcessors();
                numProc = intMin(numProcSched, numProcSys);
                string1 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Your system provides only "));
                    __mm_s.push_str(&*intString(numProcSys));
                    __mm_s.push_str(&*literal!(" processors!\n"));
                    ArcStr::from(__mm_s)
                };
                string2 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*intString(numProcSched));
                    __mm_s.push_str(&*literal!(" processors might be a reasonable number of processors.\n"));
                    ArcStr::from(__mm_s)
                };
                string1 = if (intGt(numProcSched, numProcSys)) {
                    string1
                } else {
                    string2
                };
                metamodelica::print(literal!("Please set the number of processors you want to use!\n"));
                metamodelica::print(string1);
            } else {
                numProc = 1;
                metamodelica::print(literal!(
                    "You did not choose a number of cores. Since there is no ODE-System, the number of cores is set to 1!\n"
                ));
            }
            FlagsUtil::setConfigInt(Flags::NUM_PROC.clone(), numProc)?;
            (numProc, true)
        }
        _ => {
            let mut numProcSys: i32;
            numProcSys = System::numProcessors();
            if intGt(numProcFlag, numProcSys) && Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Warning: Your system provides only "));
                    __mm_s.push_str(&*intString(numProcSys));
                    __mm_s.push_str(&*literal!(" processors!\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (numProcFlag, true)
        }
    });
    Ok((numProcOut, numFixed))
}

pub(crate) fn applyGRS(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    HpcOmTaskGraph::TaskGraphMeta,
)> {
    let mut oTaskGraph: metamodelica::Array<metamodelica::List<i32>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut taskGraph1: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta;
    let mut contractedTasks: metamodelica::Array<i32>;
    taskGraph1 = metamodelica::arrayFromVec(iTaskGraph.clone().borrow().clone());
    taskGraphT =
        AdjacencyMatrix::transposeAdjacencyMatrix(taskGraph1.clone(), metamodelica::arrayLength(taskGraph1.clone()))?;
    taskGraphMeta1 = HpcOmTaskGraph::copyTaskGraphMeta(iTaskGraphMeta);
    contractedTasks = arrayCreate(metamodelica::arrayLength(taskGraph1.clone()), 0);
    (taskGraph1, taskGraphT, taskGraphMeta1) = applyGRS1(
        taskGraph1.clone(),
        taskGraphT.clone(),
        taskGraphMeta1,
        contractedTasks.clone(),
        true,
    )?;
    (oTaskGraph, oTaskGraphMeta) = GRS_newGraph(taskGraph1.clone(), taskGraphMeta1, contractedTasks.clone())?;
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn applyGRS1(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iContractedTasks: metamodelica::Array<i32>,
    mut again: bool,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    HpcOmTaskGraph::TaskGraphMeta,
)> {
    '__tco: loop {
        match again {
            true => {
                let mut changed: bool;
                let mut changed2: bool;
                let mut tmpTaskGraph: metamodelica::Array<metamodelica::List<i32>>;
                let mut tmpTaskGraphT: metamodelica::Array<metamodelica::List<i32>>;
                let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
                let mut tmpContractedTasks: metamodelica::Array<i32>;
                (
                    tmpTaskGraph,
                    tmpTaskGraphT,
                    tmpTaskGraphMeta,
                    tmpContractedTasks,
                    changed,
                ) = HpcOmTaskGraph::mergeSimpleNodes(
                    iTaskGraph.clone(),
                    iTaskGraphT.clone(),
                    iTaskGraphMeta,
                    iContractedTasks.clone(),
                )?;
                (
                    tmpTaskGraph,
                    tmpTaskGraphT,
                    tmpTaskGraphMeta,
                    tmpContractedTasks,
                    changed2,
                ) = HpcOmTaskGraph::mergeParentNodes(
                    tmpTaskGraph.clone(),
                    tmpTaskGraphT.clone(),
                    tmpTaskGraphMeta,
                    tmpContractedTasks.clone(),
                )?;
                changed = changed || changed2;
                {
                    (iTaskGraph, iTaskGraphT, iTaskGraphMeta, iContractedTasks, again) = (
                        tmpTaskGraph.clone(),
                        tmpTaskGraphT.clone(),
                        tmpTaskGraphMeta,
                        tmpContractedTasks.clone(),
                        changed,
                    );
                    continue '__tco;
                }
            }
            _ => return Ok((iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta)),
        }
    }
}

fn applyGRSForScheduler(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iContractedTasks: metamodelica::Array<i32>,
) -> (
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    HpcOmTaskGraph::TaskGraphMeta,
) {
    let mut oTaskGraph: metamodelica::Array<metamodelica::List<i32>>;
    let mut oTaskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut flagValue: ArcStr = arcstr::literal!("");
    let mut levelNodes: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut contractedNodes: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpTaskGraph: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut tmpTaskGraphT: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta =
        <HpcOmTaskGraph::TaskGraphMeta as ::std::default::Default>::default();
    (oTaskGraph, oTaskGraphT, oTaskGraphMeta) = 'mc: {
        let __mc_input = iContractedTasks.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut contractedNodes: metamodelica::List<metamodelica::List<i32>> = contractedNodes.clone();
            let mut flagValue: ArcStr = flagValue.clone();
            let mut levelNodes: metamodelica::List<metamodelica::List<i32>> = levelNodes.clone();
            let mut tmpTaskGraph: metamodelica::Array<metamodelica::List<i32>> = tmpTaskGraph.clone();
            let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta = tmpTaskGraphMeta.clone();
            let mut tmpTaskGraphT: metamodelica::Array<metamodelica::List<i32>> = tmpTaskGraphT.clone();
            flagValue = Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?;
            let true = (stringEq(&flagValue, &(literal!("levelfix")))) else {
                return Err("pattern mismatch");
            };
            levelNodes = HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone())?;
            contractedNodes = applyGRSForLevelFixScheduler(
                &iTaskGraphMeta,
                iContractedTasks.clone(),
                levelNodes.clone(),
                metamodelica::nil(),
            )?;
            (tmpTaskGraph, tmpTaskGraphT, tmpTaskGraphMeta, _) = HpcOmTaskGraph::contractNodesInGraph(
                &contractedNodes,
                iTaskGraph.clone(),
                iTaskGraphT.clone(),
                iTaskGraphMeta.clone(),
                iContractedTasks.clone(),
            )?;
            Ok((
                (tmpTaskGraph.clone(), tmpTaskGraphT.clone(), tmpTaskGraphMeta.clone()),
                contractedNodes.clone(),
                flagValue.clone(),
                levelNodes.clone(),
                tmpTaskGraph.clone(),
                tmpTaskGraphMeta.clone(),
                tmpTaskGraphT.clone(),
            ))
        })() {
            contractedNodes = __wb0;
            flagValue = __wb1;
            levelNodes = __wb2;
            tmpTaskGraph = __wb3;
            tmpTaskGraphMeta = __wb4;
            tmpTaskGraphT = __wb5;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oTaskGraph, oTaskGraphT, oTaskGraphMeta)
}

pub(crate) fn applyGRSForLevelFixScheduler<'__b>(
    mut iTaskGraphMeta: &'__b HpcOmTaskGraph::TaskGraphMeta,
    mut iContractedTasks: metamodelica::Array<i32>,
    mut iLevelNodes: metamodelica::List<metamodelica::List<i32>>,
    mut iContractedLevelfixTasks: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    '__tco: loop {
        let mut rest: metamodelica::List<metamodelica::List<i32>>;
        let mut head: metamodelica::List<i32>;
        let mut sortedHead: metamodelica::List<i32>;
        let mut sortedHeadArray: metamodelica::Array<i32>;
        let mut tmpContractedLevelfixTasks: metamodelica::List<metamodelica::List<i32>>;
        let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
        let mut bigTaskExecTime: metamodelica::Real;
        let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
        ::match_deref::match_deref! { match &((iTaskGraphMeta.clone(), iLevelNodes)) {
            (HpcOmTaskGraph::TaskGraphMeta { exeCosts: __esc_exeCosts, inComps: __esc_inComps, .. }, Deref @ metamodelica::ListNode::Cons { head: __esc_head, tail: __esc_rest }) => {
                exeCosts = (*__esc_exeCosts).clone();
                inComps = (*__esc_inComps).clone();
                head = (*__esc_head).clone();
                rest = (*__esc_rest).clone();
                sortedHead = List::sort(head.clone(), (std::sync::Arc::new({ let __pe_b2 = inComps.clone(); let __pe_b3 = exeCosts.clone(); let __pe_b4 = false; move |__pe_a0, __pe_a1| HpcOmTaskGraph::compareTasksByExecTime(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                sortedHeadArray = metamodelica::arrayFromVec(sortedHead.into_iter().cloned().collect());
                if intGt(metamodelica::arrayLength(sortedHeadArray.clone()), 0) {
                    bigTaskExecTime = HpcOmTaskGraph::getExeCostReqCycles(metamodelica::arrayGet(sortedHeadArray.clone(), metamodelica::arrayLength(sortedHeadArray.clone()))?, iTaskGraphMeta.clone())?;
                } else {
                    bigTaskExecTime = metamodelica::OrderedFloat(0.0_f64);
                }
                tmpContractedLevelfixTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta, iContractedTasks.clone(), 500, sortedHeadArray.clone(), 1, &((metamodelica::arrayLength(sortedHeadArray.clone()), metamodelica::nil(), bigTaskExecTime)), iContractedLevelfixTasks);
                { (iTaskGraphMeta, iContractedTasks, iLevelNodes, iContractedLevelfixTasks) = (iTaskGraphMeta, iContractedTasks.clone(), rest.clone(), tmpContractedLevelfixTasks); continue '__tco; }
            },
            _ => return Ok(iContractedLevelfixTasks),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn applyGRSForLevelFixSchedulerLevel(
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iContractedTasks: metamodelica::Array<i32>,
    mut iCriticalSize: i32,
    mut iSortedLevelTasks: metamodelica::Array<i32>,
    mut iCurrentSmallTask: i32,
    mut iCurrentBigTask: &(i32, metamodelica::List<i32>, metamodelica::Real),
    mut iContractedLevelfixTasks: metamodelica::List<metamodelica::List<i32>>,
) -> metamodelica::List<metamodelica::List<i32>> {
    let mut oContractedLevelfixTasks: metamodelica::List<metamodelica::List<i32>>;
    let mut tmpContractedTasks: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut bigTaskChilds: metamodelica::List<i32>;
    let mut mergedGroupExecTime: metamodelica::Real;
    let mut bigTaskIdx: i32;
    oContractedLevelfixTasks = 'mc: {
        let __mc_input = (iCurrentBigTask, iContractedLevelfixTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((bigTaskIdx, bigTaskChilds, mergedGroupExecTime), tmpContractedTasks) => {
                    let mut tmpContractedTasks = (*tmpContractedTasks).clone();
                    let true = (intLe(bigTaskIdx.clone(), iCurrentSmallTask)) else { return Err("pattern mismatch") };
                    if !((bigTaskChilds).is_empty()) {
                        tmpContractedTasks = metamodelica::cons(metamodelica::cons(metamodelica::arrayGet(iSortedLevelTasks.clone(), bigTaskIdx.clone())?, bigTaskChilds.clone()), tmpContractedTasks.clone());
                    }
                    Ok(tmpContractedTasks.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((bigTaskIdx, bigTaskChilds, mergedGroupExecTime), _) => {
                    let mut mergedGroupExecTime = (*mergedGroupExecTime).clone();
                    let mut tmpContractedTasks: metamodelica::List<metamodelica::List<i32>> = tmpContractedTasks.clone();
                    let true = (HpcOmTaskGraph::isNodeContracted(bigTaskIdx.clone(), iContractedTasks.clone())?) else { return Err("pattern mismatch") };
                    if intGt(bigTaskIdx.clone(), 1) {
                        mergedGroupExecTime = HpcOmTaskGraph::getExeCostReqCycles(metamodelica::arrayGet(iSortedLevelTasks.clone(), bigTaskIdx.clone() - 1)?, iTaskGraphMeta.clone())?;
                    } else {
                        mergedGroupExecTime = metamodelica::OrderedFloat(0.0_f64);
                    }
                    tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta, iContractedTasks.clone(), iCriticalSize, iSortedLevelTasks.clone(), iCurrentSmallTask, &((bigTaskIdx.clone() - 1, metamodelica::nil(), mergedGroupExecTime.clone())), iContractedLevelfixTasks.clone());
                    Ok((tmpContractedTasks.clone(), tmpContractedTasks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tmpContractedTasks = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((bigTaskIdx, bigTaskChilds, mergedGroupExecTime), _) => {
                    let mut tmpContractedTasks: metamodelica::List<metamodelica::List<i32>> = tmpContractedTasks.clone();
                    let true = (HpcOmTaskGraph::isNodeContracted(iCurrentSmallTask, iContractedTasks.clone())?) else { return Err("pattern mismatch") };
                    tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta, iContractedTasks.clone(), iCriticalSize, iSortedLevelTasks.clone(), iCurrentSmallTask + 1, &((bigTaskIdx.clone(), bigTaskChilds.clone(), mergedGroupExecTime.clone())), iContractedLevelfixTasks.clone());
                    Ok((tmpContractedTasks.clone(), tmpContractedTasks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tmpContractedTasks = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((bigTaskIdx, bigTaskChilds, mergedGroupExecTime), tmpContractedTasks) => {
                    let mut mergedGroupExecTime = (*mergedGroupExecTime).clone();
                    let mut tmpContractedTasks = (*tmpContractedTasks).clone();
                    mergedGroupExecTime = mergedGroupExecTime.clone() + HpcOmTaskGraph::getExeCostReqCycles(metamodelica::arrayGet(iSortedLevelTasks.clone(), iCurrentSmallTask)?, iTaskGraphMeta.clone())?;
                    if realGe(mergedGroupExecTime.clone(), metamodelica::OrderedFloat((iCriticalSize) as f64)) {
                        if !((bigTaskChilds).is_empty()) {
                            tmpContractedTasks = metamodelica::cons(metamodelica::cons(metamodelica::arrayGet(iSortedLevelTasks.clone(), bigTaskIdx.clone())?, bigTaskChilds.clone()), tmpContractedTasks.clone());
                        }
                        if intGt(bigTaskIdx.clone(), 1) {
                            mergedGroupExecTime = HpcOmTaskGraph::getExeCostReqCycles(metamodelica::arrayGet(iSortedLevelTasks.clone(), bigTaskIdx.clone() - 1)?, iTaskGraphMeta.clone())?;
                        } else {
                            mergedGroupExecTime = metamodelica::OrderedFloat(0.0_f64);
                        }
                        tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta, iContractedTasks.clone(), iCriticalSize, iSortedLevelTasks.clone(), iCurrentSmallTask, &((bigTaskIdx.clone() - 1, metamodelica::nil(), mergedGroupExecTime.clone())), tmpContractedTasks.clone());
                    } else {
                        tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta, iContractedTasks.clone(), iCriticalSize, iSortedLevelTasks.clone(), iCurrentSmallTask + 1, &((bigTaskIdx.clone(), metamodelica::cons(metamodelica::arrayGet(iSortedLevelTasks.clone(), iCurrentSmallTask)?, bigTaskChilds.clone()), mergedGroupExecTime.clone())), tmpContractedTasks.clone());
                    }
                    Ok(tmpContractedTasks.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iContractedLevelfixTasks.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oContractedLevelfixTasks
}

fn GRS_newGraph(
    mut graphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut contrTasks: metamodelica::Array<i32>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    HpcOmTaskGraph::TaskGraphMeta,
)> {
    let mut graphOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut metaOut: HpcOmTaskGraph::TaskGraphMeta;
    let mut newSize: i32;
    let mut notRemovedNodes: metamodelica::List<i32>;
    let mut removedNodes: metamodelica::List<i32>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut inCompsNew: metamodelica::Array<metamodelica::List<i32>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = &metaIn;
    inComps = metamodelica::Own::own(__pa0);
    notRemovedNodes = HpcOmTaskGraph::filterContractedNodes(
        List::intRange(metamodelica::arrayLength(graphIn.clone())),
        contrTasks.clone(),
    )?;
    removedNodes = HpcOmTaskGraph::filterNonContractedNodes(
        List::intRange(metamodelica::arrayLength(graphIn.clone())),
        contrTasks.clone(),
    )?;
    newSize = ((notRemovedNodes).len() as i32);
    graphOut = arrayCreate(newSize, metamodelica::nil());
    inCompsNew = arrayCreate(newSize, metamodelica::nil());
    (graphOut, inCompsNew) = GRS_newGraph2(
        &notRemovedNodes,
        &removedNodes,
        contrTasks.clone(),
        graphIn.clone(),
        inComps.clone(),
        graphOut.clone(),
        inCompsNew.clone(),
        1,
    )?;
    metaOut = HpcOmTaskGraph::setInCompsInMeta(inCompsNew.clone(), metaIn);
    Ok((graphOut, metaOut))
}

fn GRS_newGraph2<'__b>(
    mut origNodes: &'__b metamodelica::List<i32>,
    mut removedNodes: &'__b metamodelica::List<i32>,
    mut contrTasks: metamodelica::Array<i32>,
    mut origGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut origInComps: metamodelica::Array<metamodelica::List<i32>>,
    mut newGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut newInComps: metamodelica::Array<metamodelica::List<i32>>,
    mut newNode: i32,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match origNodes {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((newGraph.clone(), newInComps.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
                let mut row: metamodelica::List<i32>;
                let mut comps: metamodelica::List<i32>;
                row = metamodelica::arrayGet(origGraph.clone(), node.clone())?;
                row = HpcOmTaskGraph::filterContractedNodes(row, contrTasks.clone())?;
                row = HpcOmTaskGraph::updateContinuousEntriesInList(row, removedNodes.clone())?;
                comps = metamodelica::arrayGet(origInComps.clone(), node.clone())?;
                metamodelica::arrayUpdate(newGraph.clone(), newNode, row)?;
                metamodelica::arrayUpdate(newInComps.clone(), newNode, comps)?;
                { (origNodes, removedNodes, contrTasks, origGraph, origInComps, newGraph, newInComps, newNode) = (rest, removedNodes, contrTasks.clone(), origGraph.clone(), origInComps.clone(), newGraph.clone(), newInComps.clone(), newNode + 1); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn createSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iFilenamePrefix: &ArcStr,
    mut iNumProc: i32,
    mut iNumProcToUse: i32,
    mut iSimCode: metamodelica::Ref<SimCode::SimCode>,
    mut iScheduledTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iSystemName: &ArcStr,
    mut iSchedulerName: ArcStr,
) -> Result<(
    metamodelica::Ref<HpcOmSimCode::Schedule>,
    metamodelica::Ref<SimCode::SimCode>,
    metamodelica::Array<metamodelica::List<i32>>,
    HpcOmTaskGraph::TaskGraphMeta,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut oSimCode: metamodelica::Ref<SimCode::SimCode>;
    let mut oTaskGraph: metamodelica::Array<metamodelica::List<i32>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut knownScheduler: metamodelica::List<ArcStr> = list![
        literal!("none"),
        literal!("level"),
        literal!("levelfix"),
        literal!("ext"),
        literal!("metis"),
        literal!("hmet"),
        literal!("listr"),
        literal!("rand"),
        literal!("list"),
        literal!("mcp"),
        literal!("part"),
        literal!("taskdep"),
        literal!("tds"),
        literal!("bls"),
        literal!("sbs"),
        literal!("sts")
    ];
    let mut schedulerName: ArcStr = iSchedulerName.clone();
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut numProcToUse: i32 = iNumProcToUse;
    if boolNot(List::exist1(
        &knownScheduler,
        &fnptr!(stringEq, ArcStr, ArcStr),
        schedulerName.clone(),
    )?) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("HpcOmScheduler.createSchedule warning: The scheduler '"));
            __mm_s.push_str(&*iSchedulerName);
            __mm_s.push_str(&*literal!(
                "' is unknown. The list-scheduling algorithm is used instead for the "
            ));
            __mm_s.push_str(&*iSystemName);
            __mm_s.push_str(&*literal!(".\n"));
            ArcStr::from(__mm_s)
        });
        schedulerName = literal!("list");
    }
    if intGt(iNumProcToUse, iNumProc) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "HpcOmScheduler.createSchedule warning: Cannot schedule the the task graph to "
            ));
            __mm_s.push_str(&*intString(iNumProcToUse));
            __mm_s.push_str(&*literal!(
                " processors, because the number is larger than the available processors ("
            ));
            __mm_s.push_str(&*intString(iNumProc));
            __mm_s.push_str(&*literal!(").\n"));
            ArcStr::from(__mm_s)
        });
        numProcToUse = iNumProc;
    }
    (tmpSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = createSchedule1(
        iTaskGraph.clone(),
        iTaskGraphMeta,
        iSccSimEqMapping.clone(),
        iSimVarMapping.clone(),
        iFilenamePrefix,
        numProcToUse,
        iSimCode,
        iScheduledTasks,
        iSystemName,
        &schedulerName,
    )?;
    oSchedule = HpcOmScheduler::expandSchedule(iNumProc, numProcToUse, tmpSchedule)?;
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn createSchedule1(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iFilenamePrefix: &ArcStr,
    mut iNumProc: i32,
    mut iSimCode: metamodelica::Ref<SimCode::SimCode>,
    mut iScheduledTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iSystemName: &ArcStr,
    mut iSchedulerName: &ArcStr,
) -> Result<(
    metamodelica::Ref<HpcOmSimCode::Schedule>,
    metamodelica::Ref<SimCode::SimCode>,
    metamodelica::Array<metamodelica::List<i32>>,
    HpcOmTaskGraph::TaskGraphMeta,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut oSimCode: metamodelica::Ref<SimCode::SimCode>;
    let mut oTaskGraph: metamodelica::Array<metamodelica::List<i32>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut sccSimEqMap: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    let mut taskGraph1: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta =
        <HpcOmTaskGraph::TaskGraphMeta as ::std::default::Default>::default();
    let mut simCode: metamodelica::Ref<SimCode::SimCode> =
        <metamodelica::Ref<SimCode::SimCode> as ::std::default::Default>::default();
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = 'mc: {
        let __mc_input = iSchedulerName.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "none" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using serial code for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createEmptySchedule(iTaskGraph.clone(), &iTaskGraphMeta, iSccSimEqMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "level" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta = taskGraphMeta1.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using level Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    (schedule, taskGraphMeta1) = HpcOmScheduler::createLevelSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), taskGraphMeta1.clone(), iSccSimEqMapping.clone()), schedule.clone(), taskGraphMeta1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            taskGraphMeta1 = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "levelfix" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta = taskGraphMeta1.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using fixed level Scheduler (experimental) for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    (schedule, taskGraphMeta1) = HpcOmScheduler::createFixedLevelSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), taskGraphMeta1.clone(), iSccSimEqMapping.clone()), schedule.clone(), taskGraphMeta1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            taskGraphMeta1 = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "ext" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using external Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createExtSchedule(iTaskGraph.clone(), &iTaskGraphMeta, iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFilenamePrefix); __mm_s.push_str(&*literal!("_ext.graphml")); ArcStr::from(__mm_s) })?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "metis" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using METIS Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createMetisSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "hmet" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using hMETIS Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createHMetisSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "listr" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using list reverse Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createListScheduleReverse(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "rand" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Random Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createRandomSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "list" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using list Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createListSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "mcp" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Modified Critical Path Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createMCPschedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "part" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using partition Scheduler for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createPartSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "taskdep" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using dynamic task dependencies for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createTaskDepSchedule(iTaskGraph.clone(), &iTaskGraphMeta, iSccSimEqMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "tds" => {
                    let mut sccSimEqMap: metamodelica::Array<metamodelica::List<i32>> = sccSimEqMap.clone();
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    let mut simCode: metamodelica::Ref<SimCode::SimCode> = simCode.clone();
                    let mut taskGraph1: metamodelica::Array<metamodelica::List<i32>> = taskGraph1.clone();
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta = taskGraphMeta1.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Task Duplication-based Scheduling for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    (schedule, simCode, taskGraph1, taskGraphMeta1, sccSimEqMap) = HpcOmScheduler::TDS_schedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc, iSccSimEqMapping.clone(), iSimVarMapping.clone(), &iSimCode)?;
                    Ok(((schedule.clone(), simCode.clone(), taskGraph1.clone(), taskGraphMeta1.clone(), sccSimEqMap.clone()), sccSimEqMap.clone(), schedule.clone(), simCode.clone(), taskGraph1.clone(), taskGraphMeta1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            sccSimEqMap = __wb0;
            schedule = __wb1;
            simCode = __wb2;
            taskGraph1 = __wb3;
            taskGraphMeta1 = __wb4;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "bls" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta = taskGraphMeta1.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Balanced Level Scheduling for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    (schedule, taskGraphMeta1) = HpcOmScheduler::createBalancedLevelScheduling(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), taskGraphMeta1.clone(), iSccSimEqMapping.clone()), schedule.clone(), taskGraphMeta1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            taskGraphMeta1 = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "sbs" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Single Block Scheduling for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmEqSystems::createSingleBlockSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iScheduledTasks.clone(), iSccSimEqMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "sts" => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Single Thread Scheduling for the ")); __mm_s.push_str(&*iSystemName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    schedule = HpcOmScheduler::createSingleThreadSchedule(iTaskGraph.clone(), &iTaskGraphMeta, iSccSimEqMapping.clone(), iNumProc)?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule> = schedule.clone();
                    metamodelica::print(literal!("HpcOmSimCode.createSchedule failed!\n"));
                    schedule = HpcOmScheduler::createEmptySchedule(iTaskGraph.clone(), &iTaskGraphMeta, iSccSimEqMapping.clone())?;
                    Ok(((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()), schedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            schedule = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

// test functions
//------------------------------------------
//------------------------------------------
fn checkOdeSystemSize(
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iOdeEqs: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<bool> {
    let mut oIsCorrect: bool;
    let mut scc: i32 = 0;
    let mut sccs: metamodelica::List<i32>;
    let mut actualSizePre: i32;
    let mut actualSize: i32;
    let mut targetSize: i32;
    sccs = List::sort(
        HpcOmTaskGraph::getAllSCCsOfGraph(iTaskGraphMeta)?,
        (std::sync::Arc::new(fnptr!(intGt, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    actualSizePre = ((sccs).len() as i32);
    actualSize = ((List::sortedUnique(sccs.clone(), &fnptr!(intEq, i32, i32))?).len() as i32);
    if intNe(actualSizePre, actualSize) {
        metamodelica::print(literal!(
            "There are simCode-equations multiple times in the graph structure.\n"
        ));
    }
    actualSize = 0;
    for mut scc in &*sccs {
        let mut scc = scc.clone();
        actualSize = actualSize + ((metamodelica::arrayGet(iSccSimEqMapping.clone(), scc)?).len() as i32);
    }
    targetSize = ((List::flatten(iOdeEqs.clone())?).len() as i32);
    oIsCorrect = intEq(targetSize, actualSize);
    if oIsCorrect {
    } else {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("the size of the ODE-system should be "));
            __mm_s.push_str(&*intString(targetSize));
            __mm_s.push_str(&*literal!(" but it is "));
            __mm_s.push_str(&*intString(actualSize));
            __mm_s.push_str(&*literal!("!\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("expected the following sim code equations: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(
                    List::map(List::flatten(iOdeEqs)?, &move |__a0: metamodelica::Ref<
                        SimCode::SimEqSystem,
                    >| {
                        SimCodeCodegenUtil::simEqSystemIndex(&__a0)
                    })?,
                    &fnptr!(intString, i32),
                )?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("the ODE-system is NOT correct\n"));
    }
    Ok(oIsCorrect)
}

fn checkTaskGraphMetaConsistency(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iSystemName: &ArcStr,
) -> bool {
    let mut oIsCorrect: bool;
    let mut numberOfNodes: i32;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    numberOfNodes = metamodelica::arrayLength(iTaskGraph.clone());
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    if boolNot(intEq(numberOfNodes, metamodelica::arrayLength(inComps.clone()))) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("the number of nodes in the "));
            __mm_s.push_str(&*iSystemName);
            __mm_s.push_str(&*literal!(" task graph ("));
            __mm_s.push_str(&*intString(numberOfNodes));
            __mm_s.push_str(&*literal!(
                ") is distinguished from the number of nodes in task graph meta ("
            ));
            __mm_s.push_str(&*intString(metamodelica::arrayLength(inComps.clone())));
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        });
        oIsCorrect = false;
    } else {
        oIsCorrect = true;
    }
    oIsCorrect
}

fn checkEquationCount(
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iSystemName: &ArcStr,
    mut iExpectedNumberOfEqs: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<bool> {
    let mut oIsCorrect: bool;
    let mut inCompsIdx: i32;
    let mut eqCount: i32;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut comps: metamodelica::List<i32>;
    let mut compEqs: metamodelica::List<i32>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    inCompsIdx = metamodelica::arrayLength(inComps.clone());
    eqCount = 0;
    while intGt(inCompsIdx, 0) {
        comps = metamodelica::arrayGet(inComps.clone(), inCompsIdx)?;
        for mut comp in &*comps {
            compEqs = metamodelica::arrayGet(iSccSimEqMapping.clone(), comp.clone())?;
            eqCount = eqCount + ((compEqs).len() as i32);
        }
        inCompsIdx = inCompsIdx - 1;
    }
    oIsCorrect = intEq(iExpectedNumberOfEqs, eqCount);
    if boolNot(oIsCorrect) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("the number of equations in the "));
            __mm_s.push_str(&*iSystemName);
            __mm_s.push_str(&*literal!(" task graph ("));
            __mm_s.push_str(&*intString(eqCount));
            __mm_s.push_str(&*literal!(") is distinguished from the expected number of equations ("));
            __mm_s.push_str(&*intString(iExpectedNumberOfEqs));
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(oIsCorrect)
}

/*
protected function repeatScheduleWithOtherNumProc "author:Waurich TUD 2013-011
  checks if the scheduling with the given numProc is fine.
 if n=auto, more cores are available and more speedup could be achieved repeat schedule with increased num of procs."
  input HpcOmTaskGraph.TaskGraph taskGraphIn;
  input HpcOmTaskGraph.TaskGraphMeta taskGraphMetaIn;
  input array<list<Integer>> sccSimEqMappingIn;
  input String fileNamePrefix;
  input Real cpCostsWoC;
  input HpcOmSimCode.Schedule scheduleIn;
  input Integer numProcIn;
  input Boolean numFixed;
  output HpcOmSimCode.Schedule scheduleOut;
  output Integer numProcOut;
protected
  Integer maxNumProc, maxIter;
  Real maxDiff;
algorithm
  maxNumProc := System.numProcessors();
  maxIter := 3;
  maxDiff := 0.5;
  (scheduleOut,numProcOut,_) := repeatScheduleWithOtherNumProc1(taskGraphIn,taskGraphMetaIn,sccSimEqMappingIn,fileNamePrefix,cpCostsWoC,scheduleIn,numProcIn,numFixed,maxNumProc,maxDiff,maxIter);
end repeatScheduleWithOtherNumProc;


protected function repeatScheduleWithOtherNumProc1 "author:Waurich TUD 2013-011
  checks if the scheduling with the given numProc is fine.
 if n=auto, more cores are available and more speedup could be achieved repeat schedule with increased num of procs."
  input HpcOmTaskGraph.TaskGraph taskGraphIn;
  input HpcOmTaskGraph.TaskGraphMeta taskGraphMetaIn;
  input BackendDAE.BackendDAE inDAE;
  input array<list<Integer>> sccSimEqMappingIn;
  input String fileNamePrefix;
  input Real cpCostsWoC;
  input HpcOmSimCode.Schedule scheduleIn;
  input Integer numProcIn;
  input Boolean numFixed;
  input Integer maxNumProc;
  input Real maxDiff;
  input Integer numIterIn;
  output HpcOmSimCode.Schedule scheduleOut;
  output Integer numProcOut;
  output Integer numIterOut;
algorithm
  (scheduleOut,numProcOut,numIterOut) := matchcontinue(taskGraphIn,taskGraphMetaIn,inDAE,sccSimEqMappingIn,fileNamePrefix,cpCostsWoC,scheduleIn,numProcIn,numFixed,maxNumProc,maxDiff,numIterIn)
    local
      Boolean scheduleAgain;
      Integer numProc, numIt;
      Real serTime,parTime,speedup,speedUp,speedUpMax,diff;
      HpcOmSimCode.Schedule schedule;
    case(_,_,_,_,_,_,_,_,true,_,_,_)
      equation // do not schedule again because the number of procs was given
        then
          (scheduleIn,numProcIn,0);
    case(_,_,_,_,_,_,_,_,false,_,_,_)
      algorithm
        true = numIterIn == 0; // the max number of schedules with increased num of procs
        then
          (scheduleIn,numProcIn,0);
    case(_,_,_,_,_,_,_,_,false,_,_,_)
      algorithm
        (_,_,speedUp,speedUpMax) = HpcOmScheduler.predictExecutionTime(scheduleIn,SOME(cpCostsWoC),numProcIn,taskGraphIn,taskGraphMetaIn);
        diff = speedUpMax -. speedUp;
        //print("the new speedUp with "+intString(numProcIn)+" processors: "+realString(speedUp)+"\n");
        true = diff <. maxDiff;
        //print("the schedule is fine\n");
      then
        (scheduleIn,numProcIn,numIterIn);
    else
      algorithm
        numProc = numProcIn+1; // increase the number of procs
        numIt = numIterIn-1; // lower the counter of scheduling runs
        scheduleAgain = intLe(numProc,maxNumProc);
        //print("schedule again\n");
        numProc = if_(scheduleAgain,numProc,numProcIn);
        numIt = if_(scheduleAgain,numIt,0);
        schedule= Debug.bcallret6(scheduleAgain,createSchedule,taskGraphIn,taskGraphMetaIn,sccSimEqMappingIn,fileNamePrefix,numProc,scheduleIn);
        (schedule,numProc,numIt) = repeatScheduleWithOtherNumProc1(taskGraphIn,taskGraphMetaIn,sccSimEqMappingIn,fileNamePrefix,cpCostsWoC,schedule,numProc,numFixed,maxNumProc,maxDiff,numIt);
      then
        (schedule,numProc,numIt);
  end matchcontinue;
end repeatScheduleWithOtherNumProc1;
*/
//----------------------------
// output data about operations in equations and composition of systems of equations
//----------------------------
pub(crate) fn outputTimeBenchmark(
    mut graphData: HpcOmTaskGraph::TaskGraphMeta,
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<()> {
    let mut eqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut numCycles: metamodelica::List<metamodelica::Real>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let __arc2 = &(*dae);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqSystems = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    let HpcOmTaskGraph::TASKGRAPHMETA { exeCosts: __pa3, .. } = graphData;
    exeCosts = metamodelica::Own::own(__pa3);
    numCycles = List::mapArray(exeCosts.clone(), &fnptr!(Util::tuple22, _))?;
    metamodelica::print(literal!("start cost benchmark\n"));
    outputTimeBenchmark2(
        &(BackendDAEUtil::getStrongComponents(&((eqSystems).head().cloned()?))),
        &numCycles,
        &(eqSystems),
        &shared,
        1,
    );
    metamodelica::print(literal!("finish cost benchmark\n"));
    Ok(())
}

fn outputTimeBenchmark2(
    mut compsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut numCycles: &metamodelica::List<metamodelica::Real>,
    mut eqSystemsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut compIdx: i32,
) -> () {
    let () = 'mc: {
        let __mc_input = (&**compsIn, &**numCycles, &**eqSystemsIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, Deref @ metamodelica::ListNode::Cons { head: _, tail: eqSysRest }) => {
                    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    comps = BackendDAEUtil::getStrongComponents(&((eqSysRest).head().cloned()?));
                    outputTimeBenchmark2(&comps, numCycles, metamodelica::AsArg::as_arg(&eqSysRest), shared, compIdx);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp, tail: comps }, Deref @ metamodelica::ListNode::Cons { head: exeCost, tail: restCosts }, Deref @ metamodelica::ListNode::Cons { head: eqSys, tail: _ }) => {
                    let mut estimate: metamodelica::Real;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendDAEOptimize::countOperationstraverseComps(&(list![comp.clone()]), metamodelica::AsArg::as_arg(&eqSys), shared, &(metamodelica::nil()))?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    compInfo = metamodelica::Own::own(__pa0);
                    (_, estimate) = HpcOmTaskGraph::calculateCosts(&compInfo);
                    BackendDump::dumpCompInfo(&compInfo)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("task")); __mm_s.push_str(&*intString(compIdx)); __mm_s.push_str(&*literal!("-> measured: ")); __mm_s.push_str(&*intString(((exeCost.clone()).0.floor() as i32))); __mm_s.push_str(&*literal!(" and estimated: ")); __mm_s.push_str(&*intString(((estimate).0.floor() as i32))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    outputTimeBenchmark2(metamodelica::AsArg::as_arg(&comps), metamodelica::AsArg::as_arg(&restCosts), eqSystemsIn, shared, compIdx + 1);
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
