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
use crate::BackendVarTransform;
use crate::HpcOmSchedulerExt;
use crate::HpcOmSimCodeMain;
use crate::HpcOmTaskGraph;
use crate::SimCodeUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_util::HpcOmCodegenUtil;
use openmodelica_codegen_util::SimCodeCodegenUtil;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::HashTableCrefSimVar;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub type TaskAssignment = metamodelica::Array<i32>;

//the information which node <idx> is assigned to which processor <value>
//--------------
// No Scheduling
//--------------
pub(crate) fn createEmptySchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut allTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut taskIdx: i32 = 0;
    let mut weighting: i32;
    let mut index: i32;
    let mut threadIdx: i32;
    let mut calcTime: metamodelica::Real;
    let mut timeFinished: metamodelica::Real;
    let mut eqIdc: metamodelica::List<i32>;
    taskGraphT =
        AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta, &convertNodeToTask);
    for mut taskIdx in &*List::intRange(metamodelica::arrayLength(allCalcTasks.clone())).reverse() {
        let mut taskIdx = taskIdx.clone();
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(metamodelica::arrayGet(allCalcTasks.clone(), taskIdx)?) {
            (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __pa0, index: __pa1, calcTime: __pa2, timeFinished: __pa3, threadIdx: __pa4, eqIdc: __pa5 }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        weighting = metamodelica::Own::own(__pa0);
        index = metamodelica::Own::own(__pa1);
        calcTime = metamodelica::Own::own(__pa2);
        timeFinished = metamodelica::Own::own(__pa3);
        threadIdx = metamodelica::Own::own(__pa4);
        eqIdc = metamodelica::Own::own(__pa5);
        eqIdc = List::map(
            List::map1(eqIdc, &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?,
            &move |__a0: _| List::last(&__a0),
        )?;
        allTasks = metamodelica::cons(
            metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
                weighting: weighting,
                index: index,
                calcTime: calcTime,
                timeFinished: timeFinished,
                threadIdx: threadIdx,
                eqIdc: eqIdc,
            }),
            allTasks,
        );
    }
    allTasks = List::sort(
        allTasks,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| {
                compareTasksByEqIdc(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::Ref<HpcOmSimCode::Task>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    oSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::EMPTYSCHEDULE {
        tasks: HpcOmSimCode::TaskList::SERIALTASKLIST {
            tasks: allTasks,
            masterOnly: true,
        },
    });
    Ok(oSchedule)
}

//----------------
// List Scheduling
//----------------
pub(crate) fn createListSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeList_refCount: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut nodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut rootNodes: metamodelica::List<i32>;
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        commCosts: __pa0,
        inComps: __pa1,
        ..
    } = &iTaskGraphMeta;
    commCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    taskGraphT =
        AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), &iTaskGraphMeta, &convertNodeToTask);
    nodeList_refCount = List::map1(rootNodes, &getTaskByIndex, allCalcTasks.clone())?;
    nodeList = List::map(nodeList_refCount, &fnptr!(Util::tuple21, _))?;
    nodeList = List::sort(
        nodeList,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| {
                compareTasksByWeighting(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::Ref<HpcOmSimCode::Task>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    threadReadyTimes = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
    tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: threadTasks.clone(),
        outgoingDepTasks: metamodelica::nil(),
        scheduledTasks: metamodelica::nil(),
        allCalcTasks: allCalcTasks.clone(),
    });
    (tmpSchedule, _) = createListSchedule1(
        nodeList,
        threadReadyTimes.clone(),
        iTaskGraph.clone(),
        taskGraphT.clone(),
        commCosts.clone(),
        inComps.clone(),
        iSccSimEqMapping.clone(),
        iSimVarMapping.clone(),
        &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>,
               __a1: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
               __a2: i32,
               __a3: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
               __a4: metamodelica::Array<metamodelica::List<i32>>,
               __a5: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>| {
            getLocksByPredecessorList(&__a0, &__a1, __a2, __a3, __a4, __a5)
        },
        tmpSchedule,
    )?;
    tmpSchedule = addSuccessorLocksToSchedule(
        iTaskGraph.clone(),
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                  __a1: metamodelica::Ref<HpcOmSimCode::Task>,
                  __a2: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                  __a3: metamodelica::Array<metamodelica::List<i32>>,
                  __a4: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                  __a5: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>| {
                addReleaseLocksToSchedule(&__a0, __a1, __a2, __a3, __a4, __a5)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                        metamodelica::Array<metamodelica::List<i32>>,
                        metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                    )
                        -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
                    + 'static,
            >),
        commCosts.clone(),
        inComps.clone(),
        iSimVarMapping.clone(),
        &tmpSchedule,
    )?;
    oSchedule = setScheduleLockIds(&tmpSchedule)?;
    Ok(oSchedule)
}

fn createListSchedule1<'__b>(
    mut iNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iLockWithPredecessorHandler: &'__b dyn ::std::ops::Fn(
        metamodelica::Ref<HpcOmSimCode::Task>,
        metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
        i32,
        metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    )>,
    mut iSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>,
) -> Result<(
    metamodelica::Ref<HpcOmSimCode::Schedule>,
    metamodelica::Array<metamodelica::Real>,
)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
                i32,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
            ) -> Result<(
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            )> + 'static,
    >;

    '__tco: loop {
        let mut head: metamodelica::Ref<HpcOmSimCode::Task>;
        let mut newTask: metamodelica::Ref<HpcOmSimCode::Task>;
        let mut newTaskRefCount: i32;
        let mut rest: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
        let mut lastChildFinishTime: metamodelica::Real;
        let mut lastChild: metamodelica::Ref<HpcOmSimCode::Task>;
        let mut predecessors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
        let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
        let mut successorIdc: metamodelica::List<i32>;
        let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
        let mut newOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
        let mut threadFinishTimes: metamodelica::Array<metamodelica::Real>;
        let mut firstEq: i32;
        let mut allThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
        let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
        let mut lockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
        let mut threadId: i32;
        let mut threadFinishTime: metamodelica::Real;
        let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
        let mut tmpNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
        let mut weighting: i32;
        let mut index: i32;
        let mut calcTime: metamodelica::Real;
        let mut eqIdc: metamodelica::List<i32>;
        let mut simEqIdc: metamodelica::List<i32>;
        let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
        let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
        ::match_deref::match_deref! { match &((iNodeList, iSchedule.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: __esc_head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weighting, index: __esc_index, calcTime: __esc_calcTime, eqIdc: __esc_eqIdc @ Deref @ metamodelica::ListNode::Cons { head: __esc_firstEq, tail: _ }, .. }, tail: __esc_rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __esc_allThreadTasks, outgoingDepTasks: __esc_outgoingDepTasks, allCalcTasks: __esc_allCalcTasks, .. }) => {
                head = (*__esc_head).clone();
                weighting = (*__esc_weighting).clone();
                index = (*__esc_index).clone();
                calcTime = (*__esc_calcTime).clone();
                eqIdc = (*__esc_eqIdc).clone();
                firstEq = (*__esc_firstEq).clone();
                rest = (*__esc_rest).clone();
                allThreadTasks = (*__esc_allThreadTasks).clone();
                outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
                allCalcTasks = (*__esc_allCalcTasks).clone();
                (predecessors, _) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraphT.clone(), allCalcTasks.clone())?;
                (successors, successorIdc) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraph.clone(), allCalcTasks.clone())?;
                if boolNot((predecessors).is_empty()) {
                    lastChild = getTaskWithHighestFinishTime(&predecessors, None)?;
                    let __pa0 = ::match_deref::match_deref! { match &(lastChild) {
                        Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    lastChildFinishTime = metamodelica::Own::own(__pa0);
                } else {
                    lastChildFinishTime = metamodelica::OrderedFloat(0.0_f64);
                }
                threadFinishTimes = calculateFinishTimes(lastChildFinishTime, metamodelica::AsArg::as_arg(&head), &predecessors, iCommCosts.clone(), iThreadReadyTimes.clone());
                (threadId, threadFinishTime) = getThreadFinishTimesMin(1, threadFinishTimes.clone(), -1, metamodelica::OrderedFloat(0.0_f64));
                tmpThreadReadyTimes = metamodelica::arrayUpdate(iThreadReadyTimes.clone(), threadId, threadFinishTime)?;
                threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                if boolNot((predecessors).is_empty()) {
                    (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors, threadId, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks);
                    threadTasks = listAppend(lockTasks, threadTasks);
                    simEqIdc = List::map(List::map1(eqIdc.clone(), &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?, &move |__a0: _| List::last(&__a0))?;
                } else {
                    simEqIdc = List::flatten(List::map1(eqIdc.clone(), &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?)?;
                }
                newTask = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc });
                threadTasks = metamodelica::cons(newTask.clone(), threadTasks);
                allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks)?;
                (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), &successorIdc, &(metamodelica::nil()));
                tmpNodeList = listAppend(tmpNodeList, rest.clone());
                tmpNodeList = List::sort(tmpNodeList, (std::sync::Arc::new(move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| compareTasksByWeighting(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::Ref<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask, newTaskRefCount))?;
                { (iNodeList, iThreadReadyTimes, iTaskGraph, iTaskGraphT, iCommCosts, iCompTaskMapping, iSccSimEqMapping, iSimVarMapping, iLockWithPredecessorHandler, iSchedule) = (tmpNodeList, tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler, metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() })); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Nil, _) => return Ok((iSchedule, iThreadReadyTimes.clone())),
            _ => {
                metamodelica::print(literal!("HpcOmScheduler.createListSchedule1 failed\n"));
                return Ok((iSchedule, iThreadReadyTimes.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

//----------------
// Random Scheduling
//----------------
pub(crate) fn createRandomSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeList_refCount: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut nodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut rootNodes: metamodelica::List<i32>;
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        commCosts: __pa0,
        inComps: __pa1,
        ..
    } = &iTaskGraphMeta;
    commCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    taskGraphT =
        AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), &iTaskGraphMeta, &convertNodeToTask);
    nodeList_refCount = List::map1(rootNodes, &getTaskByIndex, allCalcTasks.clone())?;
    nodeList = List::map(nodeList_refCount, &fnptr!(Util::tuple21, _))?;
    nodeList = List::sort(
        nodeList,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| {
                compareTasksByWeighting(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::Ref<HpcOmSimCode::Task>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    threadReadyTimes = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
    tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: threadTasks.clone(),
        outgoingDepTasks: metamodelica::nil(),
        scheduledTasks: metamodelica::nil(),
        allCalcTasks: allCalcTasks.clone(),
    });
    (tmpSchedule, _) = createRandomSchedule1(
        &nodeList,
        threadReadyTimes.clone(),
        iTaskGraph.clone(),
        taskGraphT.clone(),
        commCosts.clone(),
        inComps.clone(),
        iSccSimEqMapping.clone(),
        iSimVarMapping.clone(),
        &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>,
               __a1: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
               __a2: i32,
               __a3: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
               __a4: metamodelica::Array<metamodelica::List<i32>>,
               __a5: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>| {
            getLocksByPredecessorList(&__a0, &__a1, __a2, __a3, __a4, __a5)
        },
        iNumberOfThreads,
        &tmpSchedule,
    )?;
    tmpSchedule = addSuccessorLocksToSchedule(
        iTaskGraph.clone(),
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                  __a1: metamodelica::Ref<HpcOmSimCode::Task>,
                  __a2: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                  __a3: metamodelica::Array<metamodelica::List<i32>>,
                  __a4: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                  __a5: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>| {
                addReleaseLocksToSchedule(&__a0, __a1, __a2, __a3, __a4, __a5)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                        metamodelica::Array<metamodelica::List<i32>>,
                        metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                    )
                        -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
                    + 'static,
            >),
        commCosts.clone(),
        inComps.clone(),
        iSimVarMapping.clone(),
        &tmpSchedule,
    )?;
    oSchedule = setScheduleLockIds(&tmpSchedule)?;
    Ok(oSchedule)
}

fn createRandomSchedule1(
    mut iNodeList: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iLockWithPredecessorHandler: &dyn ::std::ops::Fn(
        metamodelica::Ref<HpcOmSimCode::Task>,
        metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
        i32,
        metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    )>,
    mut iNumberOfThreads: i32,
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
) -> Result<(
    metamodelica::Ref<HpcOmSimCode::Schedule>,
    metamodelica::Array<metamodelica::Real>,
)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
                i32,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
            ) -> Result<(
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            )> + 'static,
    >;

    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut oThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut head: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTaskRefCount: i32 = 0;
    let mut rest: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut predecessors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = metamodelica::nil();
    let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = metamodelica::nil();
    let mut successorIdc: metamodelica::List<i32> = metamodelica::nil();
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut newOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut firstEq: i32;
    let mut allThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut lockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut tmpNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut weighting: i32;
    let mut index: i32;
    let mut calcTime: metamodelica::Real;
    let mut eqIdc: metamodelica::List<i32>;
    let mut simEqIdc: metamodelica::List<i32> = metamodelica::nil();
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    (oSchedule, oThreadReadyTimes) = 'mc: {
        let __mc_input = (&**iNodeList, &**iSchedule);
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
        )) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::ListNode::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut lockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = lockTasks.clone();
                    let mut newOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = newOutgoingDepTasks.clone();
                    let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = newTask.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut predecessors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = predecessors.clone();
                    let mut simEqIdc: metamodelica::List<i32> = simEqIdc.clone();
                    let mut successorIdc: metamodelica::List<i32> = successorIdc.clone();
                    let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real> = threadFinishTimes.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = threadTasks.clone();
                    let mut tmpNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = tmpNodeList.clone();
                    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real> = tmpThreadReadyTimes.clone();
                    (predecessors, _) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraphT.clone(), allCalcTasks.clone())?;
                    (successors, successorIdc) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraph.clone(), allCalcTasks.clone())?;
                    let false = ((predecessors).is_empty()) else { return Err("pattern mismatch") };
                    threadId = System::intRandom(iNumberOfThreads) + 1;
                    threadFinishTimes = calculateFinishTimes(metamodelica::OrderedFloat(0.0_f64), metamodelica::AsArg::as_arg(&head), &(metamodelica::nil()), iCommCosts.clone(), iThreadReadyTimes.clone());
                    threadFinishTime = metamodelica::arrayGet(threadFinishTimes.clone(), threadId)?;
                    tmpThreadReadyTimes = metamodelica::arrayUpdate(iThreadReadyTimes.clone(), threadId, threadFinishTime)?;
                    threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                    (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors.clone(), threadId, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks.clone());
                    threadTasks = listAppend(lockTasks.clone(), threadTasks.clone());
                    simEqIdc = List::map(List::map1(eqIdc.clone(), &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?, &move |__a0: _| List::last(&__a0))?;
                    newTask = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                    threadTasks = metamodelica::cons(newTask.clone(), threadTasks.clone());
                    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks.clone())?;
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), &successorIdc, &(metamodelica::nil()));
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| compareTasksByWeighting(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::Ref<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                    (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                    metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                    (tmpSchedule, tmpThreadReadyTimes) = createRandomSchedule1(&tmpNodeList, tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler, iNumberOfThreads, &(metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() })))?;
                    Ok(((tmpSchedule.clone(), tmpThreadReadyTimes.clone()), lockTasks.clone(), newOutgoingDepTasks.clone(), newTask.clone(), newTaskRefCount.clone(), predecessors.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadFinishTimes.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone(), tmpThreadReadyTimes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            lockTasks = __wb0;
            newOutgoingDepTasks = __wb1;
            newTask = __wb2;
            newTaskRefCount = __wb3;
            predecessors = __wb4;
            simEqIdc = __wb5;
            successorIdc = __wb6;
            successors = __wb7;
            threadFinishTime = __wb8;
            threadFinishTimes = __wb9;
            threadId = __wb10;
            threadTasks = __wb11;
            tmpNodeList = __wb12;
            tmpSchedule = __wb13;
            tmpThreadReadyTimes = __wb14;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11)) =
            (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::ListNode::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                        let mut allThreadTasks = (*allThreadTasks).clone();
                        let mut allCalcTasks = (*allCalcTasks).clone();
                        let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = newTask.clone();
                        let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                        let mut simEqIdc: metamodelica::List<i32> = simEqIdc.clone();
                        let mut successorIdc: metamodelica::List<i32> = successorIdc.clone();
                        let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = successors.clone();
                        let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                        let mut threadFinishTimes: metamodelica::Array<metamodelica::Real> = threadFinishTimes.clone();
                        let mut threadId: i32 = threadId.clone();
                        let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = threadTasks.clone();
                        let mut tmpNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = tmpNodeList.clone();
                        let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                        let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real> = tmpThreadReadyTimes.clone();
                        (successors, successorIdc) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraph.clone(), allCalcTasks.clone())?;
                        threadId = System::intRandom(iNumberOfThreads) + 1;
                        threadFinishTimes = calculateFinishTimes(metamodelica::OrderedFloat(0.0_f64), metamodelica::AsArg::as_arg(&head), &(metamodelica::nil()), iCommCosts.clone(), iThreadReadyTimes.clone());
                        threadFinishTime = metamodelica::arrayGet(threadFinishTimes.clone(), threadId)?;
                        tmpThreadReadyTimes = metamodelica::arrayUpdate(iThreadReadyTimes.clone(), threadId, threadFinishTime)?;
                        threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                        simEqIdc = List::flatten(List::map1(eqIdc.clone(), &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?)?;
                        newTask = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                        allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, metamodelica::cons(newTask.clone(), threadTasks.clone()))?;
                        (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), &successorIdc, &(metamodelica::nil()));
                        tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                        tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| compareTasksByWeighting(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::Ref<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                        (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                        metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                        (tmpSchedule, tmpThreadReadyTimes) = createRandomSchedule1(&tmpNodeList, tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler, iNumberOfThreads, &(metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() })))?;
                        Ok(((tmpSchedule.clone(), tmpThreadReadyTimes.clone()), newTask.clone(), newTaskRefCount.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadFinishTimes.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone(), tmpThreadReadyTimes.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })()
        {
            newTask = __wb0;
            newTaskRefCount = __wb1;
            simEqIdc = __wb2;
            successorIdc = __wb3;
            successors = __wb4;
            threadFinishTime = __wb5;
            threadFinishTimes = __wb6;
            threadId = __wb7;
            threadTasks = __wb8;
            tmpNodeList = __wb9;
            tmpSchedule = __wb10;
            tmpThreadReadyTimes = __wb11;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((iSchedule.clone(), iThreadReadyTimes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("HpcOmScheduler.createRandomSchedule1 failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oSchedule, oThreadReadyTimes))
}

//------------------------
// List Scheduling reverse
//------------------------
pub(crate) fn createListScheduleReverse(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeList_refCount: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut nodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut leaveNodes: metamodelica::List<i32>;
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut commCostsT: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        commCosts: __pa0,
        inComps: __pa1,
        ..
    } = &iTaskGraphMeta;
    commCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    taskGraphT =
        AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    commCostsT = HpcOmTaskGraph::transposeCommCosts(commCosts.clone())?;
    leaveNodes = HpcOmTaskGraph::getLeafNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(
        iTaskGraph.clone(),
        &iTaskGraphMeta,
        &move |__a0: i32, __a1: HpcOmTaskGraph::TaskGraphMeta| convertNodeToTaskReverse(__a0, &__a1),
    );
    nodeList_refCount = List::map1(leaveNodes, &getTaskByIndex, allCalcTasks.clone())?;
    nodeList = List::map(nodeList_refCount, &fnptr!(Util::tuple21, _))?;
    nodeList = List::sort(
        nodeList,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| {
                compareTasksByWeighting(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::Ref<HpcOmSimCode::Task>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    threadReadyTimes = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
    tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: threadTasks.clone(),
        outgoingDepTasks: metamodelica::nil(),
        scheduledTasks: metamodelica::nil(),
        allCalcTasks: allCalcTasks.clone(),
    });
    (tmpSchedule, _) = createListSchedule1(
        nodeList,
        threadReadyTimes.clone(),
        taskGraphT.clone(),
        iTaskGraph.clone(),
        commCostsT.clone(),
        inComps.clone(),
        iSccSimEqMapping.clone(),
        iSimVarMapping.clone(),
        &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>,
               __a1: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
               __a2: i32,
               __a3: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
               __a4: metamodelica::Array<metamodelica::List<i32>>,
               __a5: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>| {
            getLockTasksByPredecessorListReverse(&__a0, &__a1, __a2, __a3, __a4, __a5)
        },
        tmpSchedule,
    )?;
    tmpSchedule = addSuccessorLocksToSchedule(
        taskGraphT.clone(),
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                  __a1: metamodelica::Ref<HpcOmSimCode::Task>,
                  __a2: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                  __a3: metamodelica::Array<metamodelica::List<i32>>,
                  __a4: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                  __a5: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>| {
                addAssignLocksToSchedule(&__a0, __a1, __a2, __a3, __a4, __a5)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                        metamodelica::Array<metamodelica::List<i32>>,
                        metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                    )
                        -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
                    + 'static,
            >),
        commCosts.clone(),
        inComps.clone(),
        iSimVarMapping.clone(),
        &tmpSchedule,
    )?;
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tmpSchedule) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa2, outgoingDepTasks: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    threadTasks = metamodelica::Own::own(__pa2);
    outgoingDepTasks = metamodelica::Own::own(__pa3);
    threadTasks = Array::map(
        threadTasks.clone(),
        &fnptr!(
            metamodelica::listReverse,
            metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>
        ),
    )?;
    tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: threadTasks.clone(),
        outgoingDepTasks: outgoingDepTasks,
        scheduledTasks: metamodelica::nil(),
        allCalcTasks: allCalcTasks.clone(),
    });
    oSchedule = setScheduleLockIds(&tmpSchedule)?;
    Ok(oSchedule)
}

fn addSuccessorLocksToSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iCreateLockFunction: Arc<
        dyn ::std::ops::Fn(
                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >;

    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut allThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    oSchedule = (match &**iSchedule {
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks: __esc_allThreadTasks,
            outgoingDepTasks: __esc_outgoingDepTasks,
            allCalcTasks: __esc_allCalcTasks,
            ..
        } => {
            allThreadTasks = (*__esc_allThreadTasks).clone();
            outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
            allCalcTasks = (*__esc_allCalcTasks).clone();
            (allThreadTasks, _) = Array::fold(
                allThreadTasks.clone(),
                &({
                    let __pe_b1 = iTaskGraph.clone();
                    let __pe_b2 = allCalcTasks.clone();
                    let __pe_b3 = iSimVarMapping.clone();
                    let __pe_b4 = iCommCosts.clone();
                    let __pe_b5 = iCompTaskMapping.clone();
                    let __pe_b6: Arc<
                        dyn ::std::ops::Fn(
                                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                                metamodelica::Ref<HpcOmSimCode::Task>,
                                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                            )
                                -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
                            + 'static,
                    > = iCreateLockFunction.clone();
                    move |__pe_a0, __pe_a7| {
                        addSuccessorLocksToSchedule0(
                            &__pe_a0,
                            __pe_b1.clone(),
                            __pe_b2.clone(),
                            __pe_b3.clone(),
                            __pe_b4.clone(),
                            __pe_b5.clone(),
                            __pe_b6.clone(),
                            __pe_a7,
                        )
                    }
                }),
                (allThreadTasks.clone(), 1),
            )?;
            tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: allThreadTasks.clone(),
                outgoingDepTasks: outgoingDepTasks.clone(),
                scheduledTasks: metamodelica::nil(),
                allCalcTasks: allCalcTasks.clone(),
            });
            tmpSchedule
        }
        _ => {
            metamodelica::print(literal!("HpcOmScheduler.addReleaseLocksToSchedule failed\n"));
            return Err("fail");
        }
    });
    Ok(oSchedule)
}

fn addSuccessorLocksToSchedule0(
    mut iThreadTaskList: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iCreateLockFunction: Arc<
        dyn ::std::ops::Fn(
                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >,
    mut iThreadTasks: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    i32,
)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >;

    let mut oThreadTasks: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        i32,
    );
    let mut threadId: i32;
    let mut allThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    (allThreadTasks, threadId) = iThreadTasks;
    threadTasks = List::fold(
        iThreadTaskList,
        &({
            let __pe_b1 = iTaskGraph.clone();
            let __pe_b2 = iAllCalcTasks.clone();
            let __pe_b3 = iSimVarMapping.clone();
            let __pe_b4 = iCommCosts.clone();
            let __pe_b5 = iCompTaskMapping.clone();
            let __pe_b6 = (threadId, iCreateLockFunction.clone());
            move |__pe_a0, __pe_a7| {
                addSuccessorLocksToSchedule1(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    &__pe_b6,
                    __pe_a7,
                )
            }
        }),
        metamodelica::nil(),
    )?;
    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks)?;
    oThreadTasks = (allThreadTasks.clone(), threadId + 1);
    Ok(oThreadTasks)
}

fn addSuccessorLocksToSchedule1(
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iThreadIdLockFunction: &(
        i32,
        Arc<
            dyn ::std::ops::Fn(
                    (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                    metamodelica::Ref<HpcOmSimCode::Task>,
                    metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
                + 'static,
        >,
    ),
    mut iThreadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >;

    let mut oThreadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut threadIdx: i32;
    let mut index: i32;
    let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut tmpThreadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut releaseTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut iCreateLockFunction: Arc<
        dyn ::std::ops::Fn(
                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >;
    oThreadTasks = (::match_deref::match_deref! { match &((iTask.clone(), iThreadIdLockFunction.clone(), iThreadTasks)) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx: __esc_threadIdx, index: __esc_index, .. }, (_, __esc_iCreateLockFunction), __esc_tmpThreadTasks) => {
            threadIdx = (*__esc_threadIdx).clone();
            index = (*__esc_index).clone();
            iCreateLockFunction = (*__esc_iCreateLockFunction).clone();
            tmpThreadTasks = (*__esc_tmpThreadTasks).clone();
            (successors, _) = getSuccessorsByTask(&iTask, iTaskGraph.clone(), iAllCalcTasks.clone())?;
            successors = List::removeOnTrue(threadIdx.clone(), &move |__a0: i32, __a1: (metamodelica::Ref<HpcOmSimCode::Task>, i32)| compareTaskWithThreadIdx(__a0, &__a1), successors)?;
            releaseTasks = List::fold4(&successors, &*(iCreateLockFunction.clone()), iTask.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), metamodelica::nil())?;
            tmpThreadTasks = listAppend(releaseTasks, tmpThreadTasks.clone());
            tmpThreadTasks = metamodelica::cons(iTask, tmpThreadTasks.clone());
            tmpThreadTasks.clone()
        },
        (_, _, __esc_tmpThreadTasks) => {
            tmpThreadTasks = (*__esc_tmpThreadTasks).clone();
            tmpThreadTasks = metamodelica::cons(iTask, tmpThreadTasks.clone());
            tmpThreadTasks.clone()
        },
        _ => {
            metamodelica::print(literal!("HpcOmScheduler.addReleaseLocksToSchedule0 failed\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadTasks)
}

fn addReleaseLocksToSchedule(
    mut iSuccessorTask: &(metamodelica::Ref<HpcOmSimCode::Task>, i32),
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iReleaseTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut oReleaseTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut successorTask: metamodelica::Ref<HpcOmSimCode::Task>;
    (successorTask, _) = iSuccessorTask.clone();
    tmpTask = createDepTaskAndCommunicationInfo(
        iTask,
        successorTask,
        true,
        iCommCosts.clone(),
        iCompTaskMapping.clone(),
        iSimVarMapping.clone(),
    )?;
    oReleaseTasks = metamodelica::cons(tmpTask, iReleaseTasks);
    Ok(oReleaseTasks)
}

fn addAssignLocksToSchedule(
    mut iSuccessorTask: &(metamodelica::Ref<HpcOmSimCode::Task>, i32),
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iReleaseTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut oReleaseTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut successorTask: metamodelica::Ref<HpcOmSimCode::Task>;
    (successorTask, _) = iSuccessorTask.clone();
    tmpTask = createDepTaskAndCommunicationInfo(
        successorTask,
        iTask,
        false,
        iCommCosts.clone(),
        iCompTaskMapping.clone(),
        iSimVarMapping.clone(),
    )?;
    oReleaseTasks = metamodelica::cons(tmpTask, iReleaseTasks);
    Ok(oReleaseTasks)
}

fn getSimEqSysIdxForComp(
    mut compIdx: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut simEqSysIdcs: metamodelica::List<i32>;
    simEqSysIdcs = metamodelica::arrayGet(iSccSimEqMapping.clone(), compIdx)?;
    Ok(simEqSysIdcs)
}

fn getSimEqSysIdcsForCompLst(
    mut compIdcs: metamodelica::List<i32>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut simEqSysIdcs: metamodelica::List<i32>;
    simEqSysIdcs = List::flatten(List::map1(compIdcs, &Array::getIndexFirst, iSccSimEqMapping.clone())?)?;
    Ok(simEqSysIdcs)
}

pub(crate) fn getSimEqSysIdcsForNodeLst(
    mut nodeIdcs: metamodelica::List<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut simEqSysIdcsLst: metamodelica::List<metamodelica::List<i32>>;
    simEqSysIdcsLst = List::map1(nodeIdcs, &getSimEqSysIdcsForCompLst, iSccSimEqMapping.clone())?;
    Ok(simEqSysIdcsLst)
}

fn getLocksByPredecessorList(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iPredecessorList: &metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iThreadIdx: i32,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
)> {
    let mut oLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut oOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    oLockTasks = List::fold(
        iPredecessorList,
        &({
            let __pe_b1 = iTask.clone();
            let __pe_b2 = iThreadIdx;
            let __pe_b3 = iCommCosts.clone();
            let __pe_b4 = iCompTaskMapping.clone();
            let __pe_b5 = iSimVarMapping.clone();
            move |__pe_a0, __pe_a6| {
                Ok(getLockTasksByPredecessorList(
                    &__pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    __pe_a6,
                ))
            }
        }),
        metamodelica::nil(),
    )?;
    oOutgoingDepTasks = oLockTasks.clone();
    Ok((oLockTasks, oOutgoingDepTasks))
}

fn getLockTasksByPredecessorList(
    mut iPredecessorTask: &(metamodelica::Ref<HpcOmSimCode::Task>, i32),
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iThreadIdx: i32,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut threadIdx: i32;
    let mut tmpLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut predTask: metamodelica::Ref<HpcOmSimCode::Task>;
    oLockTasks = 'mc: {
        let __mc_input = (iPredecessorTask, &*iTask, iLockTasks.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((predTask @ Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, index: _, .. }, _), Deref @ HpcOmSimCode::Task::CALCTASK { index: _, .. }, tmpLockTasks) => {
                    let mut tmpLockTasks = (*tmpLockTasks).clone();
                    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task> = tmpTask.clone();
                    let true = (intNe(iThreadIdx, threadIdx.clone())) else { return Err("pattern mismatch") };
                    tmpTask = createDepTaskAndCommunicationInfo(predTask.clone(), iTask.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    tmpLockTasks = metamodelica::cons(tmpTask.clone(), tmpLockTasks.clone());
                    Ok((tmpLockTasks.clone(), tmpTask.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tmpTask = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLockTasks.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oLockTasks
}

fn getLockTasksByPredecessorListReverse(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iPredecessorList: &metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iThreadIdx: i32,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
)> {
    let mut oLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut oOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    oLockTasks = List::fold(
        iPredecessorList,
        &({
            let __pe_b1 = iTask.clone();
            let __pe_b2 = iThreadIdx;
            let __pe_b3 = iCommCosts.clone();
            let __pe_b4 = iCompTaskMapping.clone();
            let __pe_b5 = iSimVarMapping.clone();
            move |__pe_a0, __pe_a6| {
                Ok(getLockTasksByPredecessorListReverse0(
                    &__pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    __pe_a6,
                ))
            }
        }),
        metamodelica::nil(),
    )?;
    oOutgoingDepTasks = oLockTasks.clone();
    Ok((oLockTasks, oOutgoingDepTasks))
}

fn getLockTasksByPredecessorListReverse0(
    mut iPredecessorTask: &(metamodelica::Ref<HpcOmSimCode::Task>, i32),
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iThreadIdx: i32,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut index: i32;
    let mut threadIdx: i32;
    let mut predTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    oLockTasks = 'mc: {
        let __mc_input = iPredecessorTask;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (predTask @ Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, index, .. }, _) => {
                    let mut tmpLockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = tmpLockTasks.clone();
                    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task> = tmpTask.clone();
                    let true = (intNe(iThreadIdx, threadIdx.clone())) else { return Err("pattern mismatch") };
                    tmpTask = createDepTaskAndCommunicationInfo(iTask.clone(), predTask.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    tmpLockTasks = metamodelica::cons(tmpTask.clone(), iLockTasks.clone());
                    Ok((tmpLockTasks.clone(), tmpLockTasks.clone(), tmpTask.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tmpLockTasks = __wb0;
            tmpTask = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLockTasks.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oLockTasks
}

fn getCommunicationObjBetweenMergedTasks(
    mut parentNode: i32,
    mut node: i32,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut inCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
) -> Result<HpcOmTaskGraph::Communication> {
    let mut oCommunication: HpcOmTaskGraph::Communication;
    let mut nodeTasks: metamodelica::List<i32>;
    let mut parentTasks: metamodelica::List<i32>;
    let mut commFold: HpcOmTaskGraph::Communication;
    let mut edgesFromParents: metamodelica::List<HpcOmTaskGraph::Communication>;
    nodeTasks = metamodelica::arrayGet(inComps.clone(), node)?;
    parentTasks = metamodelica::arrayGet(inComps.clone(), parentNode)?;
    commFold = HpcOmTaskGraph::Communication {
        numberOfVars: 0,
        integerVars: metamodelica::nil(),
        floatVars: metamodelica::nil(),
        booleanVars: metamodelica::nil(),
        stringVars: metamodelica::nil(),
        childNode: node,
        requiredTime: metamodelica::OrderedFloat(-1.0_f64),
    };
    edgesFromParents = List::flatten(List::map1(parentTasks, &Array::getIndexFirst, inCommCosts.clone())?)?;
    oCommunication = List::fold(
        &edgesFromParents,
        &({
            let __pe_b1 = nodeTasks;
            move |__pe_a0, __pe_a2| {
                Ok(getCommunicationObjBetweenMergedTasks1(
                    &__pe_a0,
                    __pe_b1.clone(),
                    __pe_a2,
                ))
            }
        }),
        commFold,
    )?;
    Ok(oCommunication)
}

fn getCommunicationObjBetweenMergedTasks1(
    mut parentCommCost: &HpcOmTaskGraph::Communication,
    mut tasks: metamodelica::List<i32>,
    mut iCommunication: HpcOmTaskGraph::Communication,
) -> HpcOmTaskGraph::Communication {
    let mut oCommunication: HpcOmTaskGraph::Communication;
    oCommunication = (match (parentCommCost.clone(), iCommunication.clone()) {
        (
            HpcOmTaskGraph::Communication {
                numberOfVars: mut nV1,
                integerVars: ref ints1,
                floatVars: ref fl1,
                booleanVars: ref b1,
                stringVars: ref s1,
                childNode: mut childNode,
                requiredTime: mut reqT1,
            },
            HpcOmTaskGraph::Communication {
                numberOfVars: mut nV2,
                integerVars: ref ints2,
                floatVars: ref fl2,
                booleanVars: ref b2,
                stringVars: ref s2,
                childNode: _,
                requiredTime: mut reqT2,
            },
        ) if (listMember(childNode.clone(), tasks.clone())) => HpcOmTaskGraph::Communication {
            numberOfVars: nV1.clone() + nV2.clone(),
            integerVars: listAppend(ints1.clone(), ints2.clone()),
            floatVars: listAppend(fl1.clone(), fl2.clone()),
            booleanVars: listAppend(b1.clone(), b2.clone()),
            stringVars: listAppend(s1.clone(), s2.clone()),
            childNode: childNode.clone(),
            requiredTime: reqT1.clone() + reqT2.clone(),
        },
        _ => iCommunication,
    });
    oCommunication
}

fn convertCommunicationToCommInfo(
    mut iCommunication: &HpcOmTaskGraph::Communication,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<HpcOmSimCode::CommunicationInfo> {
    let mut oCommInfo: HpcOmSimCode::CommunicationInfo;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut intSimVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut floatSimVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut boolSimVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    oCommInfo = (match iCommunication.clone() {
        HpcOmTaskGraph::Communication {
            integerVars: mut __esc_integerVars,
            floatVars: mut __esc_floatVars,
            booleanVars: mut __esc_booleanVars,
            ..
        } => {
            integerVars = __esc_integerVars.clone();
            floatVars = __esc_floatVars.clone();
            booleanVars = __esc_booleanVars.clone();
            intSimVars = List::fold1(
                metamodelica::AsArg::as_arg(&integerVars),
                &convertVarIdxToSimVar,
                iSimVarMapping.clone(),
                metamodelica::nil(),
            )?;
            floatSimVars = List::fold1(
                metamodelica::AsArg::as_arg(&floatVars),
                &convertVarIdxToSimVar,
                iSimVarMapping.clone(),
                metamodelica::nil(),
            )?;
            boolSimVars = List::fold1(
                metamodelica::AsArg::as_arg(&booleanVars),
                &convertVarIdxToSimVar,
                iSimVarMapping.clone(),
                metamodelica::nil(),
            )?;
            HpcOmSimCode::CommunicationInfo {
                floatVars: floatSimVars,
                intVars: intSimVars,
                boolVars: boolSimVars,
            }
        }
    });
    Ok(oCommInfo)
}

fn convertVarIdxToSimVar(
    mut iVarIdx: i32,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimVar: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut oSimVar: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut tmpSimVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    tmpSimVars = metamodelica::arrayGet(iSimVarMapping.clone(), iVarIdx)?;
    oSimVar = listAppend(iSimVar, tmpSimVars);
    Ok(oSimVar)
}

fn createDepTask(
    mut iSourceTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTargetTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iOutgoing: bool,
    mut commInfo: HpcOmSimCode::CommunicationInfo,
) -> metamodelica::Ref<HpcOmSimCode::Task> {
    let mut oAssignTask: metamodelica::Ref<HpcOmSimCode::Task>;
    oAssignTask = metamodelica::Ref::new(HpcOmSimCode::Task::DEPTASK {
        sourceTask: iSourceTask,
        targetTask: iTargetTask,
        outgoing: iOutgoing,
        id: 0,
        communicationInfo: commInfo,
    });
    oAssignTask
}

fn createDepTaskAndCommunicationInfo(
    mut iSourceTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTargetTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iOutgoing: bool,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oAssignTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut predIndex: i32;
    let mut taskIndex: i32;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut commBetweenTasks: HpcOmTaskGraph::Communication =
        <HpcOmTaskGraph::Communication as ::std::default::Default>::default();
    let mut commInfo: HpcOmSimCode::CommunicationInfo =
        <HpcOmSimCode::CommunicationInfo as ::std::default::Default>::default();
    oAssignTask = 'mc: {
        let __mc_input = (&*iSourceTask, &*iTargetTask);
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Task::CALCTASK { index: predIndex, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIndex, .. }) => {
                    let mut commBetweenTasks: HpcOmTaskGraph::Communication = commBetweenTasks.clone();
                    let mut commInfo: HpcOmSimCode::CommunicationInfo = commInfo.clone();
                    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task> = tmpTask.clone();
                    commBetweenTasks = getCommunicationObjBetweenMergedTasks(predIndex.clone(), taskIndex.clone(), iCompTaskMapping.clone(), iCommCosts.clone())?;
                    commInfo = convertCommunicationToCommInfo(&commBetweenTasks, iSimVarMapping.clone())?;
                    tmpTask = createDepTask(iSourceTask.clone(), iTargetTask.clone(), iOutgoing, commInfo.clone());
                    Ok((tmpTask.clone(), commBetweenTasks.clone(), commInfo.clone(), tmpTask.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            commBetweenTasks = __wb0;
            commInfo = __wb1;
            tmpTask = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("CreateDepTaskAndCommunicationInfo failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oAssignTask)
}

fn createDepTaskByTaskIdc(
    mut iSourceTaskIdx: i32,
    mut iTargetTaskIdx: i32,
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iOutgoing: bool,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oAssignTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut sourceTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut targetTask: metamodelica::Ref<HpcOmSimCode::Task>;
    sourceTask = Util::tuple21(metamodelica::arrayGet(iAllCalcTasks.clone(), iSourceTaskIdx)?);
    targetTask = Util::tuple21(metamodelica::arrayGet(iAllCalcTasks.clone(), iTargetTaskIdx)?);
    oAssignTask = createDepTaskAndCommunicationInfo(
        sourceTask,
        targetTask,
        iOutgoing,
        iCommCosts.clone(),
        iCompTaskMapping.clone(),
        iSimVarMapping.clone(),
    )?;
    Ok(oAssignTask)
}

fn createDepTaskByTaskIdcR(
    mut iSourceTaskIdx: i32,
    mut iTargetTaskIdx: i32,
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iOutgoing: bool,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oAssignTask: metamodelica::Ref<HpcOmSimCode::Task>;
    oAssignTask = createDepTaskByTaskIdc(
        iTargetTaskIdx,
        iSourceTaskIdx,
        iAllCalcTasks.clone(),
        iOutgoing,
        iCommCosts.clone(),
        iCompTaskMapping.clone(),
        iSimVarMapping.clone(),
    )?;
    Ok(oAssignTask)
}

fn updateRefCounterBySuccessorIdc(
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iSuccessorIdc: &metamodelica::List<i32>,
    mut iRefZeroTasks: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> (
    metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) {
    let mut oAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut oRefZeroTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut head: i32;
    let mut currentRefCount: i32 = 0;
    let mut rest: metamodelica::List<i32>;
    let mut tmpRefZeroTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut currentTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = Default::default();
    (oAllCalcTasks, oRefZeroTasks) = 'mc: {
        let __mc_input = &**iSuccessorIdc;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut currentRefCount: i32 = currentRefCount.clone();
                    let mut currentTask: metamodelica::Ref<HpcOmSimCode::Task> = currentTask.clone();
                    let mut tmpAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = tmpAllCalcTasks.clone();
                    let mut tmpRefZeroTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = tmpRefZeroTasks.clone();
                    (currentTask, currentRefCount) = metamodelica::arrayGet(iAllCalcTasks.clone(), head.clone())?;
                    let true = (intEq(currentRefCount, 1)) else { return Err("pattern mismatch") };
                    tmpAllCalcTasks = metamodelica::arrayUpdate(iAllCalcTasks.clone(), head.clone(), (currentTask.clone(), 0))?;
                    tmpRefZeroTasks = metamodelica::cons(currentTask.clone(), iRefZeroTasks.clone());
                    (tmpAllCalcTasks, tmpRefZeroTasks) = updateRefCounterBySuccessorIdc(tmpAllCalcTasks.clone(), metamodelica::AsArg::as_arg(&rest), &tmpRefZeroTasks);
                    Ok(((tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()), currentRefCount.clone(), currentTask.clone(), tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            currentRefCount = __wb0;
            currentTask = __wb1;
            tmpAllCalcTasks = __wb2;
            tmpRefZeroTasks = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut currentRefCount: i32 = currentRefCount.clone();
                    let mut currentTask: metamodelica::Ref<HpcOmSimCode::Task> = currentTask.clone();
                    let mut tmpAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = tmpAllCalcTasks.clone();
                    let mut tmpRefZeroTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = tmpRefZeroTasks.clone();
                    (currentTask, currentRefCount) = metamodelica::arrayGet(iAllCalcTasks.clone(), head.clone())?;
                    tmpAllCalcTasks = metamodelica::arrayUpdate(iAllCalcTasks.clone(), head.clone(), (currentTask.clone(), currentRefCount - 1))?;
                    (tmpAllCalcTasks, tmpRefZeroTasks) = updateRefCounterBySuccessorIdc(tmpAllCalcTasks.clone(), metamodelica::AsArg::as_arg(&rest), iRefZeroTasks);
                    Ok(((tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()), currentRefCount.clone(), currentTask.clone(), tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            currentRefCount = __wb0;
            currentTask = __wb1;
            tmpAllCalcTasks = __wb2;
            tmpRefZeroTasks = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iAllCalcTasks.clone(), iRefZeroTasks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oAllCalcTasks, oRefZeroTasks)
}

fn getThreadFinishTimesMin(
    mut iThreadIdx: i32,
    mut iThreadFinishTimes: metamodelica::Array<metamodelica::Real>,
    mut iCurrentMinThreadIdx: i32,
    mut iCurrentMinFinishTime: metamodelica::Real,
) -> (i32, metamodelica::Real) {
    let mut minThreadTime_Idx: (i32, metamodelica::Real);
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    minThreadTime_Idx = 'mc: {
        let __mc_input = iCurrentMinFinishTime;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(iThreadIdx, metamodelica::arrayLength(iThreadFinishTimes.clone()))) else {
                return Err("pattern mismatch");
            };
            Ok((iCurrentMinThreadIdx, iCurrentMinFinishTime))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
            threadFinishTime = metamodelica::arrayGet(iThreadFinishTimes.clone(), iThreadIdx)?;
            let true = (realLt(threadFinishTime, iCurrentMinFinishTime) || intEq(iCurrentMinThreadIdx, -1)) else {
                return Err("pattern mismatch");
            };
            Ok((
                getThreadFinishTimesMin(iThreadIdx + 1, iThreadFinishTimes.clone(), iThreadIdx, threadFinishTime),
                threadFinishTime.clone(),
            ))
        })() {
            threadFinishTime = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(getThreadFinishTimesMin(
                iThreadIdx + 1,
                iThreadFinishTimes.clone(),
                iCurrentMinThreadIdx,
                iCurrentMinFinishTime,
            ))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    minThreadTime_Idx
}

fn getTaskWithHighestFinishTime(
    mut iTasks: &metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iCurrentTask: Option<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut head: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut tail: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut timeFinishedHead: metamodelica::Real;
    let mut timeFinishedCurrent: metamodelica::Real;
    oTask = 'mc: {
        let __mc_input = (&**iTasks, &iCurrentTask);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (head, _), tail: tail }, None) => {
                    Ok(getTaskWithHighestFinishTime(metamodelica::AsArg::as_arg(&tail), Some(head.clone()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (head @ Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: timeFinishedHead, .. }, _), tail: tail }, Some(Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: timeFinishedCurrent, .. })) => {
                    let true = (realGt(timeFinishedHead.clone(), timeFinishedCurrent.clone())) else { return Err("pattern mismatch") };
                    Ok(getTaskWithHighestFinishTime(metamodelica::AsArg::as_arg(&tail), Some(head.clone()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (head, _), tail: tail }, Some(_)) => {
                    Ok(getTaskWithHighestFinishTime(metamodelica::AsArg::as_arg(&tail), iCurrentTask.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Some(tmpTask)) => {
                    Ok(tmpTask.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("HpcOmScheduler.getTaskWithHighestFinishTime failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oTask)
}

fn convertTaskGraphToTasks(
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iConverterFunc: &dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Ref<HpcOmSimCode::Task>>
            + 'static,
    >;

    let mut oTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut tmpTaskArray: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    tmpTaskArray = arrayCreate(
        metamodelica::arrayLength(iTaskGraphT.clone()),
        (openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY(), 0),
    );
    oTasks = convertTaskGraphToTasks1(
        iTaskGraphMeta,
        iTaskGraphT.clone(),
        1,
        iConverterFunc,
        tmpTaskArray.clone(),
    );
    oTasks
}

fn convertTaskGraphToTasks1(
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iIndex: i32,
    mut iConverterFunc: &dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
) -> metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Ref<HpcOmSimCode::Task>>
            + 'static,
    >;

    let mut oTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut tmpTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut refCount: i32 = 0;
    let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    oTasks = 'mc: {
        let __mc_input = iTasks.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = newTask.clone();
            let mut refCount: i32 = refCount.clone();
            let mut tmpTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = tmpTasks.clone();
            let true = (intLe(iIndex, metamodelica::arrayLength(iTaskGraphT.clone()))) else {
                return Err("pattern mismatch");
            };
            refCount = ((metamodelica::arrayGet(iTaskGraphT.clone(), iIndex)?).len() as i32);
            newTask = iConverterFunc(iIndex, iTaskGraphMeta.clone())?;
            tmpTasks = metamodelica::arrayUpdate(iTasks.clone(), iIndex, (newTask.clone(), refCount))?;
            tmpTasks = convertTaskGraphToTasks1(
                iTaskGraphMeta,
                iTaskGraphT.clone(),
                iIndex + 1,
                iConverterFunc,
                tmpTasks.clone(),
            );
            Ok((tmpTasks.clone(), newTask.clone(), refCount.clone(), tmpTasks.clone()))
        })() {
            newTask = __wb0;
            refCount = __wb1;
            tmpTasks = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iTasks.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oTasks
}

fn convertNodeToTask(
    mut iNodeIdx: i32,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut nodeMark: i32;
    let mut primalComp: i32;
    let mut components: metamodelica::List<i32>;
    let mut exeCost: metamodelica::Real;
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    oTask = (match iTaskGraphMeta.clone() {
        HpcOmTaskGraph::TaskGraphMeta {
            inComps: mut __esc_inComps,
            nodeMark: mut __esc_nodeMarks,
            exeCosts: mut __esc_exeCosts,
            ..
        } => {
            inComps = __esc_inComps.clone();
            nodeMarks = __esc_nodeMarks.clone();
            exeCosts = __esc_exeCosts.clone();
            components = metamodelica::arrayGet(inComps.clone(), iNodeIdx)?;
            primalComp = (components).get(1)?;
            nodeMark = metamodelica::arrayGet(nodeMarks.clone(), primalComp)?;
            (_, exeCost) = HpcOmTaskGraph::getExeCost(iNodeIdx, iTaskGraphMeta)?;
            metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
                weighting: nodeMark,
                index: iNodeIdx,
                calcTime: exeCost,
                timeFinished: metamodelica::OrderedFloat(-1.0_f64),
                threadIdx: -1,
                eqIdc: components,
            })
        }
        _ => {
            metamodelica::print(literal!("HpcOmScheduler.convertNodeToTask failed!\n"));
            return Err("fail");
        }
    });
    Ok(oTask)
}

fn convertNodeToTaskReverse(
    mut iNodeIdx: i32,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut nodeMark: i32;
    let mut primalComp: i32;
    let mut components: metamodelica::List<i32>;
    let mut exeCost: metamodelica::Real;
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    oTask = (match iTaskGraphMeta.clone() {
        HpcOmTaskGraph::TaskGraphMeta {
            inComps: mut __esc_inComps,
            nodeMark: mut __esc_nodeMarks,
            exeCosts: mut __esc_exeCosts,
            ..
        } => {
            inComps = __esc_inComps.clone();
            nodeMarks = __esc_nodeMarks.clone();
            exeCosts = __esc_exeCosts.clone();
            components = metamodelica::arrayGet(inComps.clone(), iNodeIdx)?;
            primalComp = (components).get(1)?;
            nodeMark = metamodelica::arrayGet(nodeMarks.clone(), primalComp)?;
            (_, exeCost) = metamodelica::arrayGet(exeCosts.clone(), iNodeIdx)?;
            nodeMark = nodeMark * -1;
            metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
                weighting: nodeMark,
                index: iNodeIdx,
                calcTime: exeCost,
                timeFinished: metamodelica::OrderedFloat(-1.0_f64),
                threadIdx: -1,
                eqIdc: components,
            })
        }
        _ => {
            metamodelica::print(literal!("HpcOmScheduler.convertNodeToTask failed!\n"));
            return Err("fail");
        }
    });
    Ok(oTask)
}

fn calculateFinishTimes(
    mut iPredecessorTaskLastFinished: metamodelica::Real,
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iPredecessorTasks: &metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>,
) -> metamodelica::Array<metamodelica::Real> {
    let mut oFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real>;
    tmpFinishTimes = arrayCreate(
        metamodelica::arrayLength(iThreadReadyTimes.clone()),
        metamodelica::OrderedFloat(0.0_f64),
    );
    tmpFinishTimes = calculateFinishTimes1(
        iPredecessorTaskLastFinished,
        iTask,
        iPredecessorTasks,
        iCommCosts.clone(),
        iThreadReadyTimes.clone(),
        1,
        tmpFinishTimes.clone(),
    );
    oFinishTimes = tmpFinishTimes.clone();
    oFinishTimes
}

fn calculateFinishTimes1(
    mut iPredecessorTaskLastFinished: metamodelica::Real,
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iPredecessorTasks: &metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>,
    mut iThreadIdx: i32,
    mut iFinishTimes: metamodelica::Array<metamodelica::Real>,
) -> metamodelica::Array<metamodelica::Real> {
    let mut oFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut thFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut thReadyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real> = Default::default();
    oFinishTimes = 'mc: {
        let __mc_input = iFinishTimes.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut thFinishTime: metamodelica::Real = thFinishTime.clone();
            let mut thReadyTime: metamodelica::Real = thReadyTime.clone();
            let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real> = tmpFinishTimes.clone();
            let true = (intLe(iThreadIdx, metamodelica::arrayLength(iThreadReadyTimes.clone()))) else {
                return Err("pattern mismatch");
            };
            thReadyTime = metamodelica::arrayGet(iThreadReadyTimes.clone(), iThreadIdx)?;
            thFinishTime = calculateFinishTimeByThreadId(
                thReadyTime,
                iPredecessorTaskLastFinished,
                iThreadIdx,
                iTask.clone(),
                iPredecessorTasks.clone(),
                iCommCosts.clone(),
            )?;
            tmpFinishTimes = metamodelica::arrayUpdate(iFinishTimes.clone(), iThreadIdx, thFinishTime)?;
            Ok((
                calculateFinishTimes1(
                    iPredecessorTaskLastFinished,
                    iTask,
                    iPredecessorTasks,
                    iCommCosts.clone(),
                    iThreadReadyTimes.clone(),
                    iThreadIdx + 1,
                    tmpFinishTimes.clone(),
                ),
                thFinishTime.clone(),
                thReadyTime.clone(),
                tmpFinishTimes.clone(),
            ))
        })() {
            thFinishTime = __wb0;
            thReadyTime = __wb1;
            tmpFinishTimes = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iFinishTimes.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oFinishTimes
}

fn calculateFinishTimeByThreadId(
    mut iThreadReadyTime: metamodelica::Real,
    mut iPredecessorTaskLastFinished: metamodelica::Real,
    mut iThreadId: i32,
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iPredecessorTasks: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
) -> Result<metamodelica::Real> {
    let mut oFinishTime: metamodelica::Real;
    let mut predecessorTasksOtherTh: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut commCost: metamodelica::Real;
    let mut calcTime: metamodelica::Real;
    let mut startTime: metamodelica::Real;
    oFinishTime = (match &*iTask.clone() {
        HpcOmSimCode::Task::CALCTASK {
            calcTime: __esc_calcTime,
            ..
        } => {
            calcTime = (*__esc_calcTime).clone();
            predecessorTasksOtherTh = List::removeOnTrue(
                iThreadId,
                &move |__a0: i32, __a1: (metamodelica::Ref<HpcOmSimCode::Task>, i32)| {
                    compareTaskWithThreadIdx(__a0, &__a1)
                },
                iPredecessorTasks,
            )?;
            startTime = realMax(iThreadReadyTime, iPredecessorTaskLastFinished);
            commCost = getMaxCommCostsByTaskList(iTask, &predecessorTasksOtherTh, iCommCosts.clone())?;
            ((startTime) + (commCost)) + (calcTime.clone())
        }
        _ => {
            metamodelica::print(literal!(
                "HpcOmScheduler.calculateFinishTimeByThreadId can only handle CALCTASKs\n"
            ));
            return Err("fail");
        }
    });
    Ok(oFinishTime)
}

fn getMaxCommCostsByTaskList(
    mut iParentTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTaskList: &metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
) -> Result<metamodelica::Real> {
    let mut oCommCost: metamodelica::Real;
    oCommCost = List::fold2(
        iTaskList,
        &move |__a0: (metamodelica::Ref<HpcOmSimCode::Task>, i32),
               __a1: metamodelica::Ref<HpcOmSimCode::Task>,
               __a2: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
               __a3: metamodelica::Real|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getMaxCommCostsByTaskList1(&__a0, &__a1, __a2, __a3))
        },
        iParentTask,
        iCommCosts.clone(),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    Ok(oCommCost)
}

fn getMaxCommCostsByTaskList1(
    mut iTask: &(metamodelica::Ref<HpcOmSimCode::Task>, i32),
    mut iParentTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCurrentMax: metamodelica::Real,
) -> metamodelica::Real {
    let mut oCommCost: metamodelica::Real;
    let mut reqCycles: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: metamodelica::List<i32>;
    let mut parentEqIdc: metamodelica::List<i32>;
    let mut childCommCosts: metamodelica::List<HpcOmTaskGraph::Communication> = metamodelica::nil();
    oCommCost = 'mc: {
        let __mc_input = (iTask, &**iParentTask);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ HpcOmSimCode::Task::CALCTASK { index: _, eqIdc, .. }, _), Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: parentEqIdc, .. }) => {
                    let mut childCommCosts: metamodelica::List<HpcOmTaskGraph::Communication> = childCommCosts.clone();
                    let mut reqCycles: metamodelica::Real = reqCycles.clone();
                    childCommCosts = metamodelica::arrayGet(iCommCosts.clone(), (eqIdc).head().cloned()?)?;
                    let HpcOmTaskGraph::COMMUNICATION { requiredTime: __pa0, .. } = getMaxCommCostsByTaskList2(&childCommCosts, (parentEqIdc).head().cloned()?)?;
                    reqCycles = metamodelica::Own::own(__pa0);
                    let true = (realGt(reqCycles, iCurrentMax)) else { return Err("pattern mismatch") };
                    Ok((reqCycles, childCommCosts.clone(), reqCycles.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            childCommCosts = __wb0;
            reqCycles = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iCurrentMax)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oCommCost
}

fn getMaxCommCostsByTaskList2(
    mut iCommCosts: &metamodelica::List<HpcOmTaskGraph::Communication>,
    mut iIdx: i32,
) -> Result<HpcOmTaskGraph::Communication> {
    let mut oComm: HpcOmTaskGraph::Communication;
    let mut childIdxHead: i32;
    let mut tail: metamodelica::List<HpcOmTaskGraph::Communication>;
    let mut head: HpcOmTaskGraph::Communication;
    oComm = 'mc: {
        let __mc_input = &**iCommCosts;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head @ HpcOmTaskGraph::Communication { childNode: childIdxHead, .. }, tail: tail } => {
                    let true = (intEq(childIdxHead.clone(), iIdx)) else { return Err("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: tail } => {
                    Ok(getMaxCommCostsByTaskList2(metamodelica::AsArg::as_arg(&tail), iIdx)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("HpcOmScheduler.getMaxCommCostsByTaskList2 failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oComm)
}

fn getTaskByIndex(
    mut iTaskIdx: i32,
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
) -> Result<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> {
    let mut oTask: (metamodelica::Ref<HpcOmSimCode::Task>, i32);
    oTask = metamodelica::arrayGet(iAllCalcTasks.clone(), iTaskIdx)?;
    Ok(oTask)
}

pub(crate) fn getSuccessorsByTask(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    metamodelica::List<i32>,
)> {
    let mut oTasks: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut oTaskIdc: metamodelica::List<i32>;
    let mut taskIdx: i32;
    let mut successors: metamodelica::List<i32> = metamodelica::nil();
    let mut tmpTasks: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = metamodelica::nil();
    (oTasks, oTaskIdc) = 'mc: {
        let __mc_input = &**iTask;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIdx, .. } => {
                    let mut successors: metamodelica::List<i32> = successors.clone();
                    let mut tmpTasks: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = tmpTasks.clone();
                    successors = metamodelica::arrayGet(iTaskGraph.clone(), taskIdx.clone())?;
                    tmpTasks = List::map1(successors.clone(), &getTaskByIndex, iAllCalcTasks.clone())?;
                    Ok(((tmpTasks.clone(), successors.clone()), successors.clone(), tmpTasks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            successors = __wb0;
            tmpTasks = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("HpcOmScheduler.getSuccessorsByTask can only handle CALCTASKs."));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oTasks, oTaskIdc))
}

fn compareTasksByWeighting(
    mut iTask1: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTask2: &metamodelica::Ref<HpcOmSimCode::Task>,
) -> Result<bool> {
    let mut oResult: bool;
    let mut weightingTask1: i32;
    let mut weightingTask2: i32;
    oResult = (::match_deref::match_deref! { match (iTask1, iTask2) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weightingTask1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weightingTask2, .. }) => {
            weightingTask1 = (*__esc_weightingTask1).clone();
            weightingTask2 = (*__esc_weightingTask2).clone();
            intGt(weightingTask1.clone(), weightingTask2.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.compareTasksByWeighting can only compare CALCTASKs! Task 1 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask1)); __mm_s.push_str(&*literal!(" and task 2 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask2)); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oResult)
}

fn compareTasksByEqIdc(
    mut iTask1: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTask2: &metamodelica::Ref<HpcOmSimCode::Task>,
) -> Result<bool> {
    let mut oResult: bool;
    let mut eqIdcTask1: metamodelica::List<i32>;
    let mut eqIdcTask2: metamodelica::List<i32>;
    oResult = (::match_deref::match_deref! { match (iTask1, iTask2) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: __esc_eqIdcTask1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: __esc_eqIdcTask2, .. }) => {
            eqIdcTask1 = (*__esc_eqIdcTask1).clone();
            eqIdcTask2 = (*__esc_eqIdcTask2).clone();
            intGt(List::last(metamodelica::AsArg::as_arg(&eqIdcTask1))?, List::last(metamodelica::AsArg::as_arg(&eqIdcTask2))?)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.compareTasksByEqIdc can only compare CALCTASKs with at least one equation index! Task 1 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask1)); __mm_s.push_str(&*literal!(" and task 2 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask2)); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oResult)
}

fn compareTaskWithThreadIdx(
    mut iThreadIdx: i32,
    mut iTask1: &(metamodelica::Ref<HpcOmSimCode::Task>, i32),
) -> Result<bool> {
    let mut oMatch: bool;
    let mut threadIdx: i32;
    oMatch = (::match_deref::match_deref! { match &(iTask1) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx: __esc_threadIdx, .. }, _) => {
            threadIdx = (*__esc_threadIdx).clone();
            intEq(threadIdx.clone(), iThreadIdx)
        },
        _ => {
            metamodelica::print(literal!("HpcOmScheduler.compareTaskWithThreadIdx can only compare CALCTASKs!\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oMatch)
}

fn dumpThreadSchedule(
    mut iTaskList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iThreadIdx: i32,
) -> Result<(ArcStr, i32)> {
    let mut r#str: ArcStr;
    let mut oThreadIdx: i32;
    r#str = literal!("--------------\n");
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("Thread "));
        __mm_s.push_str(&*intString(iThreadIdx));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("--------------\n"));
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*dumpTaskList(iTaskList)?);
        ArcStr::from(__mm_s)
    };
    oThreadIdx = iThreadIdx + 1;
    Ok((r#str, oThreadIdx))
}

fn dumpTaskDepSchedule(
    mut iTaskInfo: &(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::List<i32>),
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: ArcStr;
    let mut iTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut iDependencies: metamodelica::List<i32>;
    (iTask, iDependencies) = iTaskInfo.clone();
    s = literal!("Task: \n");
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*dumpTask(&iTask)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!("-> Parents: "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(iDependencies, &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!("---------------------\n"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn printTaskList(mut iTaskList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>) -> Result<()> {
    metamodelica::print(dumpTaskList(iTaskList)?);
    Ok(())
}

fn dumpTaskList(mut iTaskList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        List::map(iTaskList, &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>| {
            dumpTask(&__a0)
        })?,
        literal!(""),
    );
    Ok(r#str)
}

fn dumpTask(mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>) -> Result<ArcStr> {
    let mut oString: ArcStr;
    let mut weighting: i32;
    let mut index: i32;
    let mut threadIdx: i32;
    let mut compIdx: i32;
    let mut numThreads: i32;
    let mut sourceIndex: i32;
    let mut targetIndex: i32;
    let mut eqIdc: metamodelica::List<i32>;
    let mut nodeIdc: metamodelica::List<i32>;
    let mut timeFinished: metamodelica::Real;
    let mut s: ArcStr;
    let mut taskSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut outgoing: bool;
    let mut threadIdx: i32;
    oString = (::match_deref::match_deref! { match iTask {
        Deref @ HpcOmSimCode::Task::SCHEDULED_TASK { compIdx: __esc_compIdx, numThreads: __esc_numThreads, taskSchedule: __esc_taskSchedule } => {
            compIdx = (*__esc_compIdx).clone();
            numThreads = (*__esc_numThreads).clone();
            taskSchedule = (*__esc_taskSchedule).clone();
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Scheduled Task (comp: ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(", numThreads: ")); __mm_s.push_str(&*intString(numThreads.clone())); __mm_s.push_str(&*literal!("):\n------------------------------------------------------\n")); ArcStr::from(__mm_s) };
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*System::stringReplace(dumpSchedule(metamodelica::AsArg::as_arg(&taskSchedule))?, literal!("\n"), literal!("\n\t"))?); ArcStr::from(__mm_s) };
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("------------------------------------------------------\n")); ArcStr::from(__mm_s) };
            s
        },
        Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weighting, timeFinished: __esc_timeFinished, index: __esc_index, eqIdc: __esc_eqIdc, .. } => {
            weighting = (*__esc_weighting).clone();
            timeFinished = (*__esc_timeFinished).clone();
            index = (*__esc_index).clone();
            eqIdc = (*__esc_eqIdc).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task with index ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(" including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!(" is finished at  ")); __mm_s.push_str(&*realString(timeFinished.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: __esc_eqIdc, nodeIdc: __esc_nodeIdc, threadIdx: None } => {
            eqIdc = (*__esc_eqIdc).clone();
            nodeIdc = (*__esc_nodeIdc).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task (")); __mm_s.push_str(&*stringDelimitList(List::map(nodeIdc.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!(") including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: __esc_eqIdc, nodeIdc: __esc_nodeIdc, threadIdx: Some(__esc_threadIdx) } => {
            eqIdc = (*__esc_eqIdc).clone();
            nodeIdc = (*__esc_nodeIdc).clone();
            threadIdx = (*__esc_threadIdx).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task (")); __mm_s.push_str(&*stringDelimitList(List::map(nodeIdc.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!(") including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!(" by thread ")); __mm_s.push_str(&*intString(threadIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_sourceIndex, .. }, targetTask: Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_targetIndex, .. }, outgoing: __esc_outgoing, .. } => {
            sourceIndex = (*__esc_sourceIndex).clone();
            targetIndex = (*__esc_targetIndex).clone();
            outgoing = (*__esc_outgoing).clone();
            s = literal!("Dependency task ");
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*if (outgoing.clone()) {literal!("(outgoing)")} else {literal!("(incoming)")}); ArcStr::from(__mm_s) };
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" between ")); __mm_s.push_str(&*intString(sourceIndex.clone())); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*intString(targetIndex.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
            s
        },
        Deref @ HpcOmSimCode::Task::TASKEMPTY { .. } => literal!("empty task\n"),
        _ => {
            metamodelica::print(literal!("HpcOmScheduler.dumpTask failed\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oString)
}

pub(crate) fn printTask(mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>) -> Result<()> {
    metamodelica::print(dumpTask(iTask)?);
    Ok(())
}

pub(crate) fn convertScheduleStrucToInfo(
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iTaskCount: i32,
) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut tmpScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tasksOfLevels: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut allTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    oScheduleInfo = (match &**iSchedule {
        HpcOmSimCode::Schedule::EMPTYSCHEDULE {
            tasks: HpcOmSimCode::TaskList::SERIALTASKLIST {
                tasks: __esc_allTasks, ..
            },
        } => {
            allTasks = (*__esc_allTasks).clone();
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            threadTasks = arrayCreate(1, allTasks.clone());
            tmpScheduleInfo = Array::fold(
                threadTasks.clone(),
                &move |__a0: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                       __a1: metamodelica::Array<(i32, i32, metamodelica::Real)>| {
                    convertScheduleStrucToInfo0(&__a0, __a1)
                },
                tmpScheduleInfo.clone(),
            )?;
            tmpScheduleInfo.clone()
        }
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks: __esc_threadTasks,
            ..
        } => {
            threadTasks = (*__esc_threadTasks).clone();
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo = Array::fold(
                threadTasks.clone(),
                &move |__a0: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                       __a1: metamodelica::Array<(i32, i32, metamodelica::Real)>| {
                    convertScheduleStrucToInfo0(&__a0, __a1)
                },
                tmpScheduleInfo.clone(),
            )?;
            tmpScheduleInfo.clone()
        }
        HpcOmSimCode::Schedule::LEVELSCHEDULE {
            tasksOfLevels: __esc_tasksOfLevels,
            ..
        } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo = convertScheduleStrucToInfoLevel(
                metamodelica::AsArg::as_arg(&tasksOfLevels),
                1,
                tmpScheduleInfo.clone(),
            )?;
            tmpScheduleInfo.clone()
        }
        HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ } => {
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo.clone()
        }
        _ => {
            metamodelica::print(literal!(
                "HpcOmScheduler.convertScheduleStrucToInfo unknown Schedule-Type.\n"
            ));
            return Err("fail");
        }
    });
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfo0(
    mut iTaskList: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    (oScheduleInfo, _) = List::fold(
        iTaskList,
        &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>,
               __a1: (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)| {
            convertScheduleStrucToInfo1(&__a0, __a1)
        },
        (iScheduleInfo.clone(), 1),
    )?;
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfo1(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iScheduleInfo: (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32),
) -> Result<(metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)> {
    let mut oScheduleInfo: (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32);
    let mut taskIdx: i32;
    let mut taskNumber: i32;
    let mut threadIdx: i32;
    let mut tmpScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut timeFinished: metamodelica::Real;
    oScheduleInfo = (::match_deref::match_deref! { match &((iTask.clone(), iScheduleInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_taskIdx, threadIdx: __esc_threadIdx, timeFinished: __esc_timeFinished, .. }, (__esc_tmpScheduleInfo, __esc_taskNumber)) => {
            taskIdx = (*__esc_taskIdx).clone();
            threadIdx = (*__esc_threadIdx).clone();
            timeFinished = (*__esc_timeFinished).clone();
            tmpScheduleInfo = (*__esc_tmpScheduleInfo).clone();
            taskNumber = (*__esc_taskNumber).clone();
            tmpScheduleInfo = metamodelica::arrayUpdate(tmpScheduleInfo.clone(), taskIdx.clone(), (threadIdx.clone(), taskNumber.clone(), timeFinished.clone()))?;
            (tmpScheduleInfo.clone(), taskNumber.clone() + 1)
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { .. }, _) => iScheduleInfo,
        _ => {
            metamodelica::print(literal!("HpcOmScheduler.convertScheduleStrucToInfo1 failed. Unknown Task-Type.\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfoLevel(
    mut taskLst: &metamodelica::List<HpcOmSimCode::TaskList>,
    mut sectionsNumber: i32,
    mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    oScheduleInfo = 'mc: {
        let __mc_input = &**taskLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(iScheduleInfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks }, tail: rest } => {
                    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    scheduleInfo = convertScheduleStrucToInfoLevel1(metamodelica::AsArg::as_arg(&tasks), sectionsNumber, 1, iScheduleInfo.clone())?;
                    Ok(convertScheduleStrucToInfoLevel(metamodelica::AsArg::as_arg(&rest), sectionsNumber + 1, scheduleInfo.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks, .. }, tail: rest } => {
                    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    scheduleInfo = convertScheduleStrucToInfoLevel1(metamodelica::AsArg::as_arg(&tasks), sectionsNumber, 1, iScheduleInfo.clone())?;
                    Ok(convertScheduleStrucToInfoLevel(metamodelica::AsArg::as_arg(&rest), sectionsNumber + 1, scheduleInfo.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("convertScheduleStrucToInfoLevel failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfoLevel1<'__b>(
    mut tasks: &'__b metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut sectionsNumber: i32,
    mut sectionIdx: i32,
    mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match tasks {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iScheduleInfo.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc, threadIdx: threadIdxOpt, .. }, tail: rest } => {
                let mut numNodes: i32;
                let mut threadIdx: i32;
                let mut tuplLst: metamodelica::List<(i32, i32, metamodelica::Real)>;
                numNodes = ((nodeIdc).len() as i32);
                threadIdx = Util::getOptionOrDefault(threadIdxOpt.clone(), -1);
                tuplLst = List::threadMap1(List::fill(threadIdx, numNodes), List::fill(-1, numNodes), &fnptr!(Util::make3Tuple, _, _, _), metamodelica::OrderedFloat(0.0_f64))?;
                List::threadMap1_0(metamodelica::AsArg::as_arg(&nodeIdc), tuplLst, &Array::updateIndexFirst, iScheduleInfo.clone())?;
                { (tasks, sectionsNumber, sectionIdx, iScheduleInfo) = (rest, sectionsNumber, sectionIdx + 1, iScheduleInfo.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

//-----------------
// Balanced Level Scheduling
//-----------------
pub(crate) fn createBalancedLevelScheduling(
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(metamodelica::Ref<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut targetCost: metamodelica::Real;
    let mut levelAss: metamodelica::Array<i32>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut critPathNodes: metamodelica::List<i32>;
    let mut critPathCosts: metamodelica::List<metamodelica::Real>;
    let mut level: metamodelica::List<metamodelica::List<i32>>;
    let mut allSections: metamodelica::List<metamodelica::List<metamodelica::List<i32>>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut graphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut levelTasks: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
    targetCost = metamodelica::OrderedFloat(1000.0_f64);
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = &iMeta;
    inComps = metamodelica::Own::own(__pa0);
    graphT = AdjacencyMatrix::transposeAdjacencyMatrix(iGraph.clone(), metamodelica::arrayLength(iGraph.clone()))?;
    level = HpcOmTaskGraph::getLevelNodes(iGraph.clone())?;
    levelAss = arrayCreate(metamodelica::arrayLength(inComps.clone()), -1);
    (_, levelAss) = List::fold(
        &level,
        &move |__a0: metamodelica::List<i32>, __a1: (i32, metamodelica::Array<i32>)| getLevelAssignment(&__a0, __a1),
        (1, levelAss.clone()),
    )?;
    let __pa1 = ::match_deref::match_deref! { match &(HpcOmTaskGraph::getCriticalPaths(iGraph.clone(), iMeta.clone())) {
        (_, (Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, _)) => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    critPathNodes = metamodelica::Own::own(__pa1);
    critPathCosts = List::map1(
        critPathNodes.clone(),
        &HpcOmTaskGraph::getExeCostReqCycles,
        iMeta.clone(),
    )?;
    allSections = BLS_fillParallelSections(
        &level,
        levelAss.clone(),
        &critPathNodes,
        1,
        targetCost,
        iGraph.clone(),
        graphT.clone(),
        &iMeta,
        &(metamodelica::nil()),
        &(metamodelica::nil()),
    )?;
    allSections = List::map2(allSections, &BLS_mergeSmallSections, iMeta.clone(), targetCost)?;
    levelTasks = List::map2(
        allSections.clone(),
        &move |__a0: metamodelica::List<metamodelica::List<i32>>,
               __a1: HpcOmTaskGraph::TaskGraphMeta,
               __a2: metamodelica::Array<metamodelica::List<i32>>| BLS_generateSchedule(__a0, &__a1, __a2),
        iMeta.clone(),
        iSccSimEqMapping.clone(),
    )?;
    oSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::LEVELSCHEDULE {
        tasksOfLevels: levelTasks,
        useFixedAssignments: false,
    });
    let HpcOmTaskGraph::TASKGRAPHMETA {
        inComps: __pa2,
        varCompMapping: __pa3,
        eqCompMapping: __pa4,
        compParamMapping: __pa5,
        compNames: __pa6,
        compDescs: __pa7,
        exeCosts: __pa8,
        commCosts: __pa9,
        compInformations: __pa10,
        ..
    } = iMeta;
    inComps = metamodelica::Own::own(__pa2);
    varCompMapping = metamodelica::Own::own(__pa3);
    eqCompMapping = metamodelica::Own::own(__pa4);
    compParamMapping = metamodelica::Own::own(__pa5);
    compNames = metamodelica::Own::own(__pa6);
    compDescs = metamodelica::Own::own(__pa7);
    exeCosts = metamodelica::Own::own(__pa8);
    commCosts = metamodelica::Own::own(__pa9);
    compInformations = metamodelica::Own::own(__pa10);
    nodeMark = arrayCreate(metamodelica::arrayLength(inComps.clone()), -1);
    level = List::map(allSections, &List::flatten)?;
    (_, nodeMark) = List::fold(
        &level,
        &move |__a0: metamodelica::List<i32>, __a1: (i32, metamodelica::Array<i32>)| getLevelAssignment(&__a0, __a1),
        (1, nodeMark.clone()),
    )?;
    oMeta = HpcOmTaskGraph::TaskGraphMeta {
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
    Ok((oSchedule, oMeta))
}

fn BLS_mergeSmallSections(
    mut sectionsIn: metamodelica::List<metamodelica::List<i32>>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut targetCosts: metamodelica::Real,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut sectionsOut: metamodelica::List<metamodelica::List<i32>>;
    sectionsOut = (match targetCosts {
        _ => {
            let mut costs: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut mergedSectionIdcs: metamodelica::List<metamodelica::List<i32>>;
            let mut sectionsNew: metamodelica::List<metamodelica::List<i32>>;
            let mut sectionsNewUnflattened: metamodelica::List<metamodelica::List<metamodelica::List<i32>>>;
            let mut sectionCosts: metamodelica::List<metamodelica::Real>;
            costs = List::map1List(sectionsIn.clone(), &HpcOmTaskGraph::getExeCostReqCycles, iMeta)?;
            sectionCosts = List::map(costs, &move |__a0: metamodelica::List<metamodelica::Real>| {
                realSum(&__a0)
            })?;
            (mergedSectionIdcs, _) = BLS_mergeToTargetSize(
                &(List::intRange(((sectionsIn).len() as i32))),
                &sectionCosts,
                targetCosts,
                &(metamodelica::nil()),
            )?;
            sectionsNewUnflattened = List::map1List(
                mergedSectionIdcs,
                &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
                sectionsIn,
            )?;
            sectionsNew = List::map(sectionsNewUnflattened, &List::flatten)?;
            sectionsNew = List::map1(
                sectionsNew,
                &List::sort,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            sectionsNew
        }
    });
    Ok(sectionsOut)
}

fn BLS_generateSchedule(
    mut level: metamodelica::List<metamodelica::List<i32>>,
    mut iMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<HpcOmSimCode::TaskList> {
    let mut taskLstOut: HpcOmSimCode::TaskList;
    taskLstOut = 'mc: {
        let __mc_input = (&*level, iMeta);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: section, tail: Deref @ metamodelica::ListNode::Nil }, HpcOmTaskGraph::TaskGraphMeta { inComps, .. }) => {
                    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut taskLst: HpcOmSimCode::TaskList;
                    task = makeCalcTaskLevel(section.clone(), inComps.clone(), iSccSimEqMapping.clone())?;
                    taskLst = HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: list![task.clone()], masterOnly: true };
                    Ok(taskLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, HpcOmTaskGraph::TaskGraphMeta { inComps, .. }) => {
                    let mut taskLst: HpcOmSimCode::TaskList;
                    taskLst = makeCalcLevelParTaskLstForMergedNodes(level.clone(), iSccSimEqMapping.clone(), inComps.clone())?;
                    Ok(taskLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(taskLstOut)
}

fn BLS_fillParallelSections(
    mut levelIn: &metamodelica::List<metamodelica::List<i32>>,
    mut levelAssIn: metamodelica::Array<i32>,
    mut critPathNodes: &metamodelica::List<i32>,
    mut levelIdx: i32,
    mut targetCosts: metamodelica::Real,
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut unassNodesIn: &metamodelica::List<i32>,
    mut sectionsIn: &metamodelica::List<metamodelica::List<metamodelica::List<i32>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::List<i32>>>> {
    let mut sectionsOut: metamodelica::List<metamodelica::List<metamodelica::List<i32>>>;
    sectionsOut = 'mc: {
        let __mc_input = &**critPathNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(sectionsIn.clone().reverse())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: critPathNode, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut critNodeLevel: i32;
                    let mut levelNodes: metamodelica::List<i32>;
                    let mut unassNodes: metamodelica::List<i32>;
                    let mut levelNodeCluster: metamodelica::List<metamodelica::List<i32>>;
                    let mut followingLevel: metamodelica::List<metamodelica::List<i32>>;
                    let mut sectionLst: metamodelica::List<metamodelica::List<metamodelica::List<i32>>>;
                    critNodeLevel = metamodelica::arrayGet(levelAssIn.clone(), critPathNode.clone())?;
                    critNodeLevel = intMin(levelIdx, critNodeLevel);
                    (_, followingLevel) = List::split(levelIn.clone(), critNodeLevel - 1)?;
                    levelNodes = List::flatten(followingLevel.clone())?;
                    unassNodes = listAppend(levelNodes.clone(), unassNodesIn.clone());
                    levelNodeCluster = BLS_mergeDependentLevelTask(unassNodes.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::nil())?;
                    sectionLst = metamodelica::cons(levelNodeCluster.clone(), sectionsIn.clone());
                    sectionLst = BLS_fillParallelSections(levelIn, levelAssIn.clone(), &(metamodelica::nil()), critNodeLevel + 1, targetCosts, iGraph.clone(), iGraphT.clone(), iMeta, &unassNodes, &sectionLst)?;
                    Ok(sectionLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: critPathNode, tail: restCritNodes } => {
                    let mut critPathCost: metamodelica::Real;
                    let mut critNodeLevel: i32;
                    let mut section: metamodelica::List<i32>;
                    let mut levelNodes: metamodelica::List<i32>;
                    let mut unassNodes: metamodelica::List<i32>;
                    let mut necessaryPredecessors: metamodelica::List<i32>;
                    let mut level: metamodelica::List<metamodelica::List<i32>>;
                    let mut sectionLst: metamodelica::List<metamodelica::List<metamodelica::List<i32>>>;
                    critPathCost = HpcOmTaskGraph::getExeCostReqCycles(critPathNode.clone(), iMeta.clone())?;
                    critNodeLevel = metamodelica::arrayGet(levelAssIn.clone(), critPathNode.clone())?;
                    let true = (critPathCost < targetCosts) else { return Err("pattern mismatch") };
                    levelNodes = List::flatten(List::map1(List::intRange2(levelIdx, critNodeLevel), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), levelIn.clone())?)?;
                    (levelNodes, _) = List::deleteMemberOnTrue(critPathNode.clone(), levelNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    necessaryPredecessors = metamodelica::arrayGet(iGraphT.clone(), (restCritNodes).head().cloned()?)?;
                    unassNodes = listAppend(levelNodes.clone(), unassNodesIn.clone());
                    necessaryPredecessors = List::flatten(List::map4(List::map(necessaryPredecessors.clone(), &fnptr!(List::create, _))?, &move |__a0: metamodelica::List<i32>, __a1: metamodelica::Array<metamodelica::List<i32>>, __a2: metamodelica::Array<metamodelica::List<i32>>, __a3: metamodelica::List<i32>, __a4: metamodelica::List<i32>| BLS_getDependentGroups(&__a0, __a1, __a2, &__a3, &__a4), iGraph.clone(), iGraphT.clone(), unassNodes.clone(), metamodelica::nil())?)?;
                    necessaryPredecessors = List::unique(&necessaryPredecessors);
                    (necessaryPredecessors, _, unassNodes) = List::intersection1OnTrue(necessaryPredecessors.clone(), unassNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    section = metamodelica::cons(critPathNode.clone(), necessaryPredecessors.clone());
                    section = List::unique(&section);
                    sectionLst = metamodelica::cons(list![section.clone()], sectionsIn.clone());
                    List::map2_0(&section, &Array::updateIndexFirst, critNodeLevel, levelAssIn.clone())?;
                    level = List::map1(levelIn.clone(), &deleteIntListMembers, section.clone())?;
                    level = List::set(level.clone(), critNodeLevel, section.clone())?;
                    sectionLst = BLS_fillParallelSections(&level, levelAssIn.clone(), metamodelica::AsArg::as_arg(&restCritNodes), critNodeLevel + 1, targetCosts, iGraph.clone(), iGraphT.clone(), iMeta, &unassNodes, &sectionLst)?;
                    Ok(sectionLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: critPathNode, tail: restCritNodes } => {
                    let mut critPathCost: metamodelica::Real;
                    let mut critNodeLevel: i32;
                    let mut levelNodes: metamodelica::List<i32>;
                    let mut unassNodes: metamodelica::List<i32>;
                    let mut level: metamodelica::List<metamodelica::List<i32>>;
                    let mut levelNodeCluster: metamodelica::List<metamodelica::List<i32>>;
                    let mut sectionLst: metamodelica::List<metamodelica::List<metamodelica::List<i32>>>;
                    critPathCost = HpcOmTaskGraph::getExeCostReqCycles(critPathNode.clone(), iMeta.clone())?;
                    critNodeLevel = metamodelica::arrayGet(levelAssIn.clone(), critPathNode.clone())?;
                    let true = (critPathCost >= targetCosts) else { return Err("pattern mismatch") };
                    levelNodes = List::flatten(List::map1(List::intRange2(levelIdx, critNodeLevel), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), levelIn.clone())?)?;
                    (levelNodes, _) = List::deleteMemberOnTrue(critPathNode.clone(), levelNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    metamodelica::arrayGet(iGraphT.clone(), (restCritNodes).head().cloned()?)?;
                    unassNodes = listAppend(unassNodesIn.clone(), levelNodes.clone());
                    unassNodes = metamodelica::cons(critPathNode.clone(), unassNodes.clone());
                    unassNodes = List::unique(&unassNodes);
                    levelNodeCluster = BLS_mergeDependentLevelTask(unassNodes.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::nil())?;
                    (_, unassNodes, _) = List::intersection1OnTrue(unassNodes.clone(), List::flatten(levelNodeCluster.clone())?, &fnptr!(intEq, i32, i32))?;
                    sectionLst = metamodelica::cons(levelNodeCluster.clone(), sectionsIn.clone());
                    List::map2_0(&(List::flatten(levelNodeCluster.clone())?), &Array::updateIndexFirst, critNodeLevel, levelAssIn.clone())?;
                    level = List::map1(levelIn.clone(), &deleteIntListMembers, List::flatten(levelNodeCluster.clone())?)?;
                    level = List::set(level.clone(), critNodeLevel, List::flatten(levelNodeCluster.clone())?)?;
                    sectionLst = BLS_fillParallelSections(&level, levelAssIn.clone(), metamodelica::AsArg::as_arg(&restCritNodes), critNodeLevel + 1, targetCosts, iGraph.clone(), iGraphT.clone(), iMeta, &(metamodelica::nil()), &sectionLst)?;
                    Ok(sectionLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(sectionsOut)
}

fn BLS_mergeDependentLevelTask(
    mut nodesIn: metamodelica::List<i32>,
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut sectionsIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(nodesIn.clone()) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(sectionsIn.reverse())
            },
            Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
                let mut dependentNodes: metamodelica::List<i32>;
                let mut section: metamodelica::List<i32>;
                let mut sections: metamodelica::List<metamodelica::List<i32>>;
                let mut rest = (*rest).clone();
                dependentNodes = BLS_getDependentGroups(&(list![node.clone()]), iGraph.clone(), iGraphT.clone(), &nodesIn, &(metamodelica::nil()))?;
                section = metamodelica::cons(node.clone(), dependentNodes.clone());
                section = List::unique(&section);
                (_, rest, _) = List::intersection1OnTrue(rest.clone(), dependentNodes, &fnptr!(intEq, i32, i32))?;
                section = section.reverse();
                { (nodesIn, iGraph, iGraphT, sectionsIn) = (rest.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::cons(section, sectionsIn)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn BLS_getDependentGroups(
    mut nodes: &metamodelica::List<i32>,
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut referenceNodesIn: &metamodelica::List<i32>,
    mut dependentsIn: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut dependentsOut: metamodelica::List<i32>;
    dependentsOut = 'mc: {
        let __mc_input = &**nodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(List::unique(dependentsIn))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
                    let mut successors: metamodelica::List<i32>;
                    let mut predecessors: metamodelica::List<i32>;
                    let mut dependentNodes: metamodelica::List<i32>;
                    let mut referenceNodes: metamodelica::List<i32>;
                    let mut allNodes: metamodelica::List<i32>;
                    successors = metamodelica::arrayGet(iGraph.clone(), node.clone())?;
                    predecessors = metamodelica::arrayGet(iGraphT.clone(), node.clone())?;
                    (successors, _, referenceNodes) = List::intersection1OnTrue(successors.clone(), referenceNodesIn.clone(), &fnptr!(intEq, i32, i32))?;
                    (predecessors, _, referenceNodes) = List::intersection1OnTrue(predecessors.clone(), referenceNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    dependentNodes = listAppend(predecessors.clone(), successors.clone());
                    allNodes = metamodelica::cons(node.clone(), dependentNodes.clone());
                    dependentNodes = BLS_getDependentGroups(&(listAppend(rest.clone(), dependentNodes.clone())), iGraph.clone(), iGraphT.clone(), &referenceNodes, &(listAppend(allNodes.clone(), dependentsIn.clone())))?;
                    Ok(dependentNodes.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("BLS_getDependentGroups failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(dependentsOut)
}

fn BLS_mergeToTargetSize(
    mut nodesIn: &metamodelica::List<i32>,
    mut costsIn: &metamodelica::List<metamodelica::Real>,
    mut targetSize: metamodelica::Real,
    mut mergedNodesIn: &metamodelica::List<(metamodelica::List<i32>, metamodelica::Real)>,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::Real>,
)> {
    let mut clustersOut: metamodelica::List<metamodelica::List<i32>>;
    let mut clusterCostsOut: metamodelica::List<metamodelica::Real>;
    (clustersOut, clusterCostsOut) = 'mc: {
        let __mc_input = (&**nodesIn, &**costsIn, &**mergedNodesIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => {
                    let mut cluster: metamodelica::List<i32>;
                    let mut clusterTmp: metamodelica::List<metamodelica::List<i32>>;
                    let mut clusterCostsTmp: metamodelica::List<metamodelica::Real>;
                    clusterCostsTmp = List::map(mergedNodesIn.clone(), &fnptr!(Util::tuple22, _))?;
                    clusterTmp = List::map(mergedNodesIn.clone(), &fnptr!(Util::tuple21, _))?.reverse();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(clusterTmp.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cluster = metamodelica::Own::own(__pa0);
                    clusterTmp = metamodelica::Own::own(__pa1);
                    cluster = if ((clusterTmp).is_empty()) {cluster.clone().reverse()} else {cluster.clone()};
                    clusterTmp = metamodelica::cons(cluster.clone(), clusterTmp.clone());
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::ListNode::Cons { head: cost, tail: costRest }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut clusterTmp: metamodelica::List<metamodelica::List<i32>>;
                    let mut clusterCostsTmp: metamodelica::List<metamodelica::Real>;
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(metamodelica::AsArg::as_arg(&nodeRest), metamodelica::AsArg::as_arg(&costRest), targetSize, &(list![(list![node.clone()], cost.clone())]))?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::ListNode::Cons { head: cost, tail: costRest }, Deref @ metamodelica::ListNode::Cons { head: group, tail: restGroups }) => {
                    let mut clusterCost: metamodelica::Real;
                    let mut cluster: metamodelica::List<i32>;
                    let mut clusterTmp: metamodelica::List<metamodelica::List<i32>>;
                    let mut clusterCostsTmp: metamodelica::List<metamodelica::Real>;
                    let mut group = (*group).clone();
                    (cluster, clusterCost) = group.clone();
                    let true = (clusterCost + cost.clone() < targetSize) else { return Err("pattern mismatch") };
                    group = (metamodelica::cons(node.clone(), cluster.clone()), cost.clone() + clusterCost);
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(metamodelica::AsArg::as_arg(&nodeRest), metamodelica::AsArg::as_arg(&costRest), targetSize, &(metamodelica::cons(group.clone(), restGroups.clone())))?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::ListNode::Cons { head: cost, tail: costRest }, Deref @ metamodelica::ListNode::Cons { head: group, tail: restGroups }) => {
                    let mut clusterCost: metamodelica::Real;
                    let mut cluster: metamodelica::List<i32>;
                    let mut clusterTmp: metamodelica::List<metamodelica::List<i32>>;
                    let mut clusterCostsTmp: metamodelica::List<metamodelica::Real>;
                    let mut group = (*group).clone();
                    let mut restGroups = (*restGroups).clone();
                    (cluster, clusterCost) = group.clone();
                    let true = (clusterCost + cost.clone() >= targetSize) else { return Err("pattern mismatch") };
                    cluster = cluster.clone().reverse();
                    restGroups = metamodelica::cons((cluster.clone(), clusterCost), restGroups.clone());
                    group = (list![node.clone()], cost.clone());
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(metamodelica::AsArg::as_arg(&nodeRest), metamodelica::AsArg::as_arg(&costRest), targetSize, &(metamodelica::cons(group.clone(), restGroups.clone())))?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("BLS_mergeToTargetSize failed!"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((clustersOut, clusterCostsOut))
}

fn realSum(mut reals: &metamodelica::List<metamodelica::Real>) -> Result<metamodelica::Real> {
    let mut sum: metamodelica::Real;
    sum = List::fold(
        reals,
        &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    Ok(sum)
}

fn deleteIntListMembers(
    mut lst1: metamodelica::List<i32>,
    mut lst2: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut lstOut: metamodelica::List<i32>;
    (_, lstOut, _) = List::intersection1OnTrue(lst1, lst2, &fnptr!(intEq, i32, i32))?;
    Ok(lstOut)
}

//-----------------
// Level Scheduling
//-----------------
pub(crate) fn createLevelSchedule(
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(metamodelica::Ref<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut levelTasks: metamodelica::List<metamodelica::List<i32>>;
    let mut levelTaskLists: metamodelica::List<HpcOmSimCode::TaskList>;
    levelTasks = HpcOmTaskGraph::getLevelNodes(iGraph.clone())?;
    levelTaskLists = List::fold(
        &levelTasks,
        &({
            let __pe_b1 = iGraph.clone();
            let __pe_b2 = iMeta.clone();
            let __pe_b3 = iSccSimEqMapping.clone();
            move |__pe_a0, __pe_a4| {
                createLevelScheduleForLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4)
            }
        }),
        metamodelica::nil(),
    )?;
    levelTaskLists = levelTaskLists.reverse();
    oSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::LEVELSCHEDULE {
        tasksOfLevels: levelTaskLists,
        useFixedAssignments: false,
    });
    oMeta = iMeta;
    Ok((oSchedule, oMeta))
}

fn createLevelScheduleForLevel(
    mut iTasksOfLevel: metamodelica::List<i32>,
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iLevelTaskLists: metamodelica::List<HpcOmSimCode::TaskList>,
) -> Result<metamodelica::List<HpcOmSimCode::TaskList>> {
    let mut oLevelTaskLists: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut taskList: HpcOmSimCode::TaskList;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut sortedTasksOfLevel: metamodelica::List<i32>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        exeCosts: __pa0,
        inComps: __pa1,
        ..
    } = iMeta;
    exeCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    sortedTasksOfLevel = iTasksOfLevel;
    taskList = makeCalcLevelParTaskLst(sortedTasksOfLevel, iSccSimEqMapping.clone(), inComps.clone())?;
    oLevelTaskLists = metamodelica::cons(taskList, iLevelTaskLists);
    Ok(oLevelTaskLists)
}

fn getLevelAssignment(
    mut level: &metamodelica::List<i32>,
    mut tplIn: (i32, metamodelica::Array<i32>),
) -> Result<(i32, metamodelica::Array<i32>)> {
    let mut tplOut: (i32, metamodelica::Array<i32>);
    let mut idx: i32;
    let mut ass: metamodelica::Array<i32>;
    (idx, ass) = tplIn;
    List::map2_0(level, &Array::updateIndexFirst, idx, ass.clone())?;
    tplOut = (idx + 1, ass.clone());
    Ok(tplOut)
}

fn makeCalcLevelParTaskLst(
    mut iNodeIdc: metamodelica::List<i32>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iNodeSccMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<HpcOmSimCode::TaskList> {
    let mut oTasks: HpcOmSimCode::TaskList;
    let mut tmpList: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeIdx: i32 = 0;
    for mut nodeIdx in &*iNodeIdc.reverse() {
        let mut nodeIdx = nodeIdx.clone();
        tmpList = metamodelica::cons(list![nodeIdx], tmpList);
    }
    oTasks = makeCalcLevelParTaskLstForMergedNodes(tmpList, iSccSimEqMapping.clone(), iNodeSccMapping.clone())?;
    Ok(oTasks)
}

fn makeCalcLevelParTaskLstForMergedNodes(
    mut iNodeIdc: metamodelica::List<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iNodeSccMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<HpcOmSimCode::TaskList> {
    let mut oTasks: HpcOmSimCode::TaskList;
    let mut tmpList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    tmpList = List::map(
        iNodeIdc,
        &({
            let __pe_b1 = iNodeSccMapping.clone();
            let __pe_b2 = iSccSimEqMapping.clone();
            move |__pe_a0| makeCalcTaskLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        }),
    )?;
    oTasks = HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: tmpList };
    Ok(oTasks)
}

fn makeCalcTaskLevel(
    mut iNodeIdc: metamodelica::List<i32>,
    mut iNodeSccMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut simEqs: metamodelica::List<i32> = metamodelica::nil();
    let mut sccs: metamodelica::List<i32>;
    let mut sccIdx: i32 = 0;
    for mut nodeIdx in &*iNodeIdc {
        sccs = metamodelica::arrayGet(iNodeSccMapping.clone(), nodeIdx.clone())?;
        for mut sccIdx in &*sccs {
            let mut sccIdx = sccIdx.clone();
            simEqs = List::append_reverse(&(metamodelica::arrayGet(iSccSimEqMapping.clone(), sccIdx)?), simEqs);
        }
    }
    oTask = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK_LEVEL {
        eqIdc: simEqs.reverse(),
        nodeIdc: iNodeIdc,
        threadIdx: None,
    });
    Ok(oTask)
}

pub(crate) fn makeCalcTask(
    mut simEqs: metamodelica::List<i32>,
    mut node: i32,
    mut threadIdx: i32,
) -> metamodelica::Ref<HpcOmSimCode::Task> {
    let mut taskOut: metamodelica::Ref<HpcOmSimCode::Task>;
    taskOut = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
        weighting: 0,
        index: node,
        calcTime: metamodelica::OrderedFloat(1.0_f64),
        timeFinished: metamodelica::OrderedFloat(1.0_f64),
        threadIdx: threadIdx,
        eqIdc: simEqs,
    });
    taskOut
}

fn arrayIntIsNegative(mut node: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let mut isAss: bool;
    isAss = intLt(metamodelica::arrayGet(ass.clone(), node)?, 0);
    Ok(isAss)
}

fn dumpLevelSchedule(mut iLevelInfo: &HpcOmSimCode::TaskList, mut iLevel: i32) -> Result<(ArcStr, i32)> {
    let mut levelStr: ArcStr;
    let mut oLevel: i32;
    let mut s: ArcStr;
    let mut tasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    (levelStr, oLevel) = (match iLevelInfo.clone() {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Parallel Level "));
                __mm_s.push_str(&*intString(iLevel));
                __mm_s.push_str(&*literal!(":\n"));
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*dumpTaskList(tasks.clone())?);
                ArcStr::from(__mm_s)
            };
            (s, iLevel + 1)
        }
        HpcOmSimCode::TaskList::SERIALTASKLIST {
            tasks: mut __esc_tasks, ..
        } => {
            tasks = __esc_tasks.clone();
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Serial Level "));
                __mm_s.push_str(&*intString(iLevel));
                __mm_s.push_str(&*literal!(":\n"));
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*dumpTaskList(tasks.clone())?);
                ArcStr::from(__mm_s)
            };
            (s, iLevel + 1)
        }
        _ => {
            metamodelica::print(literal!("printLevelSchedule failed!\n"));
            return Err("fail");
        }
    });
    Ok((levelStr, oLevel))
}

//-----------------------
// Fixed level Scheduling
//-----------------------
pub(crate) fn createFixedLevelSchedule(
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(metamodelica::Ref<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut levelTasks: metamodelica::List<metamodelica::List<i32>>;
    let mut adviceLists: metamodelica::Array<metamodelica::List<i32>>;
    let mut levelTaskLists: metamodelica::List<HpcOmSimCode::TaskList>;
    levelTasks = HpcOmTaskGraph::getLevelNodes(iGraph.clone())?;
    adviceLists = arrayCreate(metamodelica::arrayLength(iGraph.clone()), metamodelica::nil());
    levelTaskLists = List::fold(
        &levelTasks,
        &({
            let __pe_b1 = adviceLists.clone();
            let __pe_b2 = iGraph.clone();
            let __pe_b3 = iMeta.clone();
            let __pe_b4 = iNumberOfThreads;
            let __pe_b5 = iSccSimEqMapping.clone();
            move |__pe_a0, __pe_a6| {
                createFixedLevelScheduleForLevel(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    __pe_a6,
                )
            }
        }),
        metamodelica::nil(),
    )?;
    levelTaskLists = levelTaskLists.reverse();
    oSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::LEVELSCHEDULE {
        tasksOfLevels: levelTaskLists,
        useFixedAssignments: true,
    });
    oMeta = iMeta;
    Ok((oSchedule, oMeta))
}

fn createFixedLevelScheduleForLevel(
    mut iTasksOfLevel: metamodelica::List<i32>,
    mut iAdviceList: metamodelica::Array<metamodelica::List<i32>>,
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iLevelTaskLists: metamodelica::List<HpcOmSimCode::TaskList>,
) -> Result<metamodelica::List<HpcOmSimCode::TaskList>> {
    let mut oLevelTaskLists: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut levelExecCosts: metamodelica::Real;
    let mut threadReadyList: metamodelica::Array<metamodelica::Real>;
    let mut threadTaskList: metamodelica::Array<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut taskList: HpcOmSimCode::TaskList;
    let mut tasksOfLevel: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut sortedTasksOfLevel: metamodelica::List<i32>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        exeCosts: __pa0,
        inComps: __pa1,
        ..
    } = &iMeta;
    exeCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    levelExecCosts = HpcOmTaskGraph::getCostsForContractedNodes(&iTasksOfLevel, exeCosts.clone())?;
    threadReadyList = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTaskList = arrayCreate(iNumberOfThreads, metamodelica::nil());
    sortedTasksOfLevel = List::sort(
        iTasksOfLevel,
        (std::sync::Arc::new({
            let __pe_b2 = inComps.clone();
            let __pe_b3 = exeCosts.clone();
            let __pe_b4 = true;
            move |__pe_a0, __pe_a1| {
                HpcOmTaskGraph::compareTasksByExecTime(
                    __pe_a0,
                    __pe_a1,
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                )
            }
        }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    List::fold(
        &sortedTasksOfLevel,
        &({
            let __pe_b1 = levelExecCosts;
            let __pe_b2 = iAdviceList.clone();
            let __pe_b3 = threadReadyList.clone();
            let __pe_b4 = iGraph.clone();
            let __pe_b5 = iMeta;
            move |__pe_a0, __pe_a6| {
                createFixedLevelScheduleForTask(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    __pe_a6,
                )
            }
        }),
        threadTaskList.clone(),
    )?;
    threadTaskList = Array::map(
        threadTaskList.clone(),
        &fnptr!(metamodelica::listReverse, metamodelica::List<i32>),
    )?;
    (_, tasksOfLevel) = Array::fold(
        threadTaskList.clone(),
        &({
            let __pe_b1 = inComps.clone();
            let __pe_b2 = iSccSimEqMapping.clone();
            move |__pe_a0, __pe_a3| {
                createFixedLevelScheduleForLevel0(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_a3)
            }
        }),
        (1, metamodelica::nil()),
    )?;
    taskList = HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: tasksOfLevel };
    oLevelTaskLists = metamodelica::cons(taskList, iLevelTaskLists);
    Ok(oLevelTaskLists)
}

fn createFixedLevelScheduleForLevel0(
    mut iTaskList: &metamodelica::List<i32>,
    mut iComps: metamodelica::Array<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iIdxTaskList: &(i32, metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>),
) -> Result<(i32, metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>)> {
    let mut oIdxTaskList: (i32, metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>);
    let mut threadIdx: i32;
    let mut taskList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut newTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut components: metamodelica::List<i32>;
    let mut simEqs: metamodelica::List<i32>;
    let mut taskIdx: i32 = 0;
    (threadIdx, taskList) = iIdxTaskList.clone();
    for mut taskIdx in &**iTaskList {
        let mut taskIdx = taskIdx.clone();
        components = metamodelica::arrayGet(iComps.clone(), taskIdx)?;
        simEqs = List::flatten(List::map(
            List::map1(components, &Array::getIndexFirst, iSccSimEqMapping.clone())?,
            &fnptr!(metamodelica::listReverse, _),
        )?)?;
        if !((simEqs).is_empty()) {
            simEqs = simEqs;
            newTask = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK_LEVEL {
                eqIdc: simEqs,
                nodeIdc: list![taskIdx],
                threadIdx: Some(threadIdx),
            });
            taskList = metamodelica::cons(newTask, taskList);
        }
    }
    oIdxTaskList = (threadIdx + 1, taskList);
    Ok(oIdxTaskList)
}

fn createFixedLevelScheduleForTask(
    mut iTaskIdx: i32,
    mut iLevelExecCosts: metamodelica::Real,
    mut iAdviceList: metamodelica::Array<metamodelica::List<i32>>,
    mut iThreadReadyList: metamodelica::Array<metamodelica::Real>,
    mut iGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iThreadTasks: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oThreadTasks: metamodelica::Array<metamodelica::List<i32>>;
    let mut adviceElem: metamodelica::List<i32>;
    let mut threadTasks: metamodelica::List<i32>;
    let mut successorList: metamodelica::List<i32>;
    let mut threadIdx: i32;
    let mut threadReadyTime: metamodelica::Real;
    let mut exeCost: metamodelica::Real;
    adviceElem = metamodelica::arrayGet(iAdviceList.clone(), iTaskIdx)?;
    adviceElem = flattenAdviceList(&adviceElem, metamodelica::arrayLength(iThreadReadyList.clone()))?;
    threadIdx = getBestFittingThread(&adviceElem, iLevelExecCosts, iThreadReadyList.clone())?;
    threadTasks = metamodelica::arrayGet(iThreadTasks.clone(), threadIdx)?;
    successorList = metamodelica::arrayGet(iGraph.clone(), iTaskIdx)?;
    List::fold1(
        &successorList,
        &createFixedLevelScheduleForTask0,
        threadIdx,
        iAdviceList.clone(),
    )?;
    threadReadyTime = metamodelica::arrayGet(iThreadReadyList.clone(), threadIdx)?;
    (_, exeCost) = HpcOmTaskGraph::getExeCost(iTaskIdx, iMeta)?;
    threadReadyTime = (threadReadyTime) + (exeCost);
    metamodelica::arrayUpdate(iThreadReadyList.clone(), threadIdx, threadReadyTime)?;
    threadTasks = metamodelica::cons(iTaskIdx, threadTasks);
    oThreadTasks = metamodelica::arrayUpdate(iThreadTasks.clone(), threadIdx, threadTasks)?;
    Ok(oThreadTasks)
}

fn createFixedLevelScheduleForTask0(
    mut iSuccessor: i32,
    mut iThreadAdvice: i32,
    mut iAdviceList: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oAdviceList: metamodelica::Array<metamodelica::List<i32>>;
    let mut adviceElem: metamodelica::List<i32>;
    adviceElem = metamodelica::arrayGet(iAdviceList.clone(), iSuccessor)?;
    adviceElem = metamodelica::cons(iThreadAdvice, adviceElem);
    oAdviceList = metamodelica::arrayUpdate(iAdviceList.clone(), iSuccessor, adviceElem)?;
    Ok(oAdviceList)
}

fn flattenAdviceList(
    mut iAdviceList: &metamodelica::List<i32>,
    mut iNumOfThreads: i32,
) -> Result<metamodelica::List<i32>> {
    let mut oAdviceList: metamodelica::List<i32>;
    let mut counterArray: metamodelica::Array<i32>;
    let mut tupleList: metamodelica::List<(i32, i32)>;
    counterArray = arrayCreate(iNumOfThreads, 0);
    counterArray = List::fold(iAdviceList, &flattenAdviceListElem, counterArray.clone())?;
    tupleList = arrayToTupleListZeroRemoved(counterArray.clone(), 1, &(metamodelica::nil()));
    oAdviceList = List::map(
        List::sort(
            tupleList,
            (std::sync::Arc::new(fnptr!(intTpl22Gt, (i32, i32), (i32, i32)))
                as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), (i32, i32)) -> Result<bool> + 'static>),
        )?,
        &fnptr!(Util::tuple21, _),
    )?;
    Ok(oAdviceList)
}

fn flattenAdviceListElem(
    mut iAdviceElem: i32,
    mut iCounterArray: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut oCounterArray: metamodelica::Array<i32>;
    let mut counter: i32;
    counter = metamodelica::arrayGet(iCounterArray.clone(), iAdviceElem)?;
    counter = counter + 1;
    oCounterArray = metamodelica::arrayUpdate(iCounterArray.clone(), iAdviceElem, counter)?;
    Ok(oCounterArray)
}

fn arrayToTupleListZeroRemoved(
    mut iArray: metamodelica::Array<i32>,
    mut iCurrentIdx: i32,
    mut iTupleList: &metamodelica::List<(i32, i32)>,
) -> metamodelica::List<(i32, i32)> {
    let mut oTupleList: metamodelica::List<(i32, i32)>;
    let mut tmpTupleList: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut currentValue: i32 = 0;
    oTupleList = 'mc: {
        let __mc_input = &**iTupleList;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut currentValue: i32 = currentValue.clone();
                    let mut tmpTupleList: metamodelica::List<(i32, i32)> = tmpTupleList.clone();
                    let true = (intLe(iCurrentIdx, metamodelica::arrayLength(iArray.clone()))) else { return Err("pattern mismatch") };
                    currentValue = metamodelica::arrayGet(iArray.clone(), iCurrentIdx)?;
                    let true = (intNe(currentValue, 0)) else { return Err("pattern mismatch") };
                    tmpTupleList = metamodelica::cons((iCurrentIdx, currentValue), iTupleList.clone());
                    tmpTupleList = arrayToTupleListZeroRemoved(iArray.clone(), iCurrentIdx + 1, &tmpTupleList);
                    Ok((tmpTupleList.clone(), currentValue.clone(), tmpTupleList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            currentValue = __wb0;
            tmpTupleList = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpTupleList: metamodelica::List<(i32, i32)> = tmpTupleList.clone();
                    let true = (intLe(iCurrentIdx, metamodelica::arrayLength(iArray.clone()))) else { return Err("pattern mismatch") };
                    tmpTupleList = arrayToTupleListZeroRemoved(iArray.clone(), iCurrentIdx + 1, iTupleList);
                    Ok((tmpTupleList.clone(), tmpTupleList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tmpTupleList = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iTupleList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oTupleList
}

fn intTpl22Gt(mut iTpl1: (i32, i32), mut iTpl2: (i32, i32)) -> bool {
    let mut oRes: bool;
    let mut val1: i32;
    let mut val2: i32;
    (_, val1) = iTpl1;
    (_, val2) = iTpl2;
    oRes = intGt(val1, val2);
    oRes
}

fn getBestFittingThread(
    mut iAdviceList: &metamodelica::List<i32>,
    mut iLevelExecCosts: metamodelica::Real,
    mut iThreadReadyList: metamodelica::Array<metamodelica::Real>,
) -> Result<i32> {
    let mut oThreadIdx: i32;
    let mut averageThreadTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut readyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut numOfThreads: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut head: i32;
    let mut tail: metamodelica::List<i32>;
    oThreadIdx = 'mc: {
        let __mc_input = &**iAdviceList;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut threadIdx: i32 = threadIdx.clone();
                    threadIdx = getFirstReadyThread(iThreadReadyList.clone())?;
                    Ok((threadIdx, threadIdx.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            threadIdx = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: tail } => {
                    let mut averageThreadTime: metamodelica::Real = averageThreadTime.clone();
                    let mut numOfThreads: i32 = numOfThreads.clone();
                    let mut readyTime: metamodelica::Real = readyTime.clone();
                    readyTime = metamodelica::arrayGet(iThreadReadyList.clone(), head.clone())?;
                    numOfThreads = metamodelica::arrayLength(iThreadReadyList.clone());
                    averageThreadTime = realDiv(iLevelExecCosts, intReal(numOfThreads));
                    let true = (realLt(readyTime, averageThreadTime)) else { return Err("pattern mismatch") };
                    Ok((head.clone(), averageThreadTime.clone(), numOfThreads.clone(), readyTime.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            averageThreadTime = __wb0;
            numOfThreads = __wb1;
            readyTime = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: tail } => {
                    Ok(getBestFittingThread(metamodelica::AsArg::as_arg(&tail), iLevelExecCosts, iThreadReadyList.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oThreadIdx)
}

fn getFirstReadyThread(mut iThreadReadyList: metamodelica::Array<metamodelica::Real>) -> Result<i32> {
    let mut oFirstReadyThreadIdx: i32;
    (oFirstReadyThreadIdx, _, _) = Array::fold(
        iThreadReadyList.clone(),
        &fnptr!(getFirstReadyThread0, metamodelica::Real, (i32, metamodelica::Real, i32)),
        (-1, metamodelica::OrderedFloat(-1.0_f64), 1),
    )?;
    Ok(oFirstReadyThreadIdx)
}

fn getFirstReadyThread0(
    mut iThreadReadyTime: metamodelica::Real,
    mut iFirstReadyThread: (i32, metamodelica::Real, i32),
) -> (i32, metamodelica::Real, i32) {
    let mut oFirstReadyThread: (i32, metamodelica::Real, i32);
    let mut firstThreadIdx: i32;
    let mut currentThreadIdx: i32;
    let mut readyTime: metamodelica::Real;
    let mut isLower: bool;
    oFirstReadyThread = (match iFirstReadyThread {
        ((-1), _, mut __esc_currentThreadIdx) => {
            currentThreadIdx = __esc_currentThreadIdx.clone();
            (currentThreadIdx, iThreadReadyTime, currentThreadIdx + 1)
        }
        (mut __esc_firstThreadIdx, mut __esc_readyTime, mut __esc_currentThreadIdx) => {
            firstThreadIdx = __esc_firstThreadIdx.clone();
            readyTime = __esc_readyTime.clone();
            currentThreadIdx = __esc_currentThreadIdx.clone();
            isLower = realLt(iThreadReadyTime, readyTime);
            firstThreadIdx = if (isLower) { currentThreadIdx } else { firstThreadIdx };
            readyTime = if (isLower) { iThreadReadyTime } else { readyTime };
            (firstThreadIdx, readyTime, currentThreadIdx + 1)
        }
        _ => {
            metamodelica::print(literal!("getFirstReadyThread0 failed\n"));
            iFirstReadyThread
        }
    });
    oFirstReadyThread
}

//---------------------------
// Task Dependency Scheduling
//---------------------------
pub(crate) fn createTaskDepSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut nodeLevelMap: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>)> =
        metamodelica::nil();
    let mut filteredNodeLevelMap: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::List<i32>)> =
        metamodelica::nil();
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta {
                inComps: mut inComps,
                nodeMark: mut nodeMark,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut filteredNodeLevelMap: metamodelica::List<(
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::List<i32>,
            )> = filteredNodeLevelMap.clone();
            let mut nodeLevelMap: metamodelica::List<(
                metamodelica::Ref<HpcOmSimCode::Task>,
                i32,
                metamodelica::List<i32>,
            )> = nodeLevelMap.clone();
            let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = taskGraphT.clone();
            let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(
                iTaskGraph.clone(),
                metamodelica::arrayLength(iTaskGraph.clone()),
            )?;
            (_, nodeLevelMap) = Array::fold(
                taskGraphT.clone(),
                &({
                    let __pe_b1 = nodeMark.clone();
                    let __pe_b2 = inComps.clone();
                    let __pe_b3 = iSccSimEqMapping.clone();
                    move |__pe_a0, __pe_a4| {
                        createNodeLevelMapping(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), &__pe_a4)
                    }
                }),
                (1, metamodelica::nil()),
            )?;
            nodeLevelMap = List::sort(nodeLevelMap.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>), __a1: (metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>)| sortNodeLevelMapping(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>), (metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>)) -> Result<bool> + 'static>))?;
            filteredNodeLevelMap = List::map(nodeLevelMap.clone(), &move |__a0: (
                metamodelica::Ref<HpcOmSimCode::Task>,
                i32,
                metamodelica::List<i32>,
            )|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(filterNodeLevelMapping(&__a0))
            })?;
            filteredNodeLevelMap = filteredNodeLevelMap.clone().reverse();
            tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::TASKDEPSCHEDULE {
                tasks: filteredNodeLevelMap.clone(),
            });
            Ok((
                tmpSchedule.clone(),
                filteredNodeLevelMap.clone(),
                nodeLevelMap.clone(),
                taskGraphT.clone(),
                tmpSchedule.clone(),
            ))
        })() {
            filteredNodeLevelMap = __wb0;
            nodeLevelMap = __wb1;
            taskGraphT = __wb2;
            tmpSchedule = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("HpcOmScheduler.createTaskDepSchedule failed.\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oSchedule)
}

fn createNodeLevelMapping(
    mut iNodeDependenciesT: metamodelica::List<i32>,
    mut nodeMarks: metamodelica::Array<i32>,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iNodeInfo: &(
        i32,
        metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>)>,
    ),
) -> Result<(
    i32,
    metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>)>,
)> {
    let mut oNodeInfo: (
        i32,
        metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>)>,
    );
    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut nodeIdx: i32;
    let mut nodeMark: i32;
    let mut components: metamodelica::List<i32>;
    let mut simEqIdc: metamodelica::List<i32>;
    let mut nodeLevelMap: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>)>;
    (nodeIdx, nodeLevelMap) = iNodeInfo.clone();
    components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
    nodeMark = metamodelica::arrayGet(nodeMarks.clone(), List::last(&components)?)?;
    simEqIdc = List::map(
        List::map1(components, &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?,
        &move |__a0: _| List::last(&__a0),
    )?;
    task = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
        weighting: -1,
        index: nodeIdx,
        calcTime: metamodelica::OrderedFloat(-1.0_f64),
        timeFinished: metamodelica::OrderedFloat(-1.0_f64),
        threadIdx: -1,
        eqIdc: simEqIdc,
    });
    nodeLevelMap = metamodelica::cons((task, nodeMark, iNodeDependenciesT), nodeLevelMap);
    oNodeInfo = (nodeIdx + 1, nodeLevelMap);
    Ok(oNodeInfo)
}

fn sortNodeLevelMapping(
    mut iElem1: &(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>),
    mut iElem2: &(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>),
) -> Result<bool> {
    let mut oResult: bool;
    let mut elemLvl1: i32;
    let mut elemLvl2: i32;
    let mut task1Idx: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match iElem1 {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa0, .. }, __pa1, _) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    task1Idx = metamodelica::Own::own(__pa0);
    elemLvl1 = metamodelica::Own::own(__pa1);
    (_, elemLvl2, _) = iElem2.clone();
    oResult = intGe(elemLvl1, elemLvl2);
    Ok(oResult)
}

fn filterNodeLevelMapping(
    mut iElem: &(metamodelica::Ref<HpcOmSimCode::Task>, i32, metamodelica::List<i32>),
) -> (metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::List<i32>) {
    let mut oElem: (metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::List<i32>);
    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut childTasks: metamodelica::List<i32>;
    (task, _, childTasks) = iElem.clone();
    oElem = (task, childTasks);
    oElem
}

//-----------------
// Metis Scheduling
//-----------------
pub(crate) fn createMetisSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut extInfo: metamodelica::List<i32> = metamodelica::nil();
    let mut xadj: metamodelica::Array<i32> = Default::default();
    let mut adjncy: metamodelica::Array<i32> = Default::default();
    let mut vwgt: metamodelica::Array<i32> = Default::default();
    let mut adjwgt: metamodelica::Array<i32> = Default::default();
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    let mut extInfoArr: metamodelica::Array<i32> = Default::default();
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        Default::default();
    let mut rootNodes: metamodelica::List<i32> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut priorityArr: metamodelica::Array<i32> = Default::default();
    let mut levelNodes: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut procAss: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut priorityTasks: metamodelica::List<i32> = metamodelica::nil();
    let mut otherTasks: metamodelica::List<i32> = metamodelica::nil();
    let mut order: metamodelica::List<i32> = metamodelica::nil();
    let mut removeLocks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
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
        )) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta {
                commCosts: mut commCosts,
                inComps: mut inComps,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut adjncy: metamodelica::Array<i32> = adjncy.clone();
            let mut adjwgt: metamodelica::Array<i32> = adjwgt.clone();
            let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> =
                allCalcTasks.clone();
            let mut extInfo: metamodelica::List<i32> = extInfo.clone();
            let mut extInfoArr: metamodelica::Array<i32> = extInfoArr.clone();
            let mut levelNodes: metamodelica::List<metamodelica::List<i32>> = levelNodes.clone();
            let mut order: metamodelica::List<i32> = order.clone();
            let mut otherTasks: metamodelica::List<i32> = otherTasks.clone();
            let mut priorityArr: metamodelica::Array<i32> = priorityArr.clone();
            let mut priorityTasks: metamodelica::List<i32> = priorityTasks.clone();
            let mut procAss: metamodelica::Array<metamodelica::List<i32>> = procAss.clone();
            let mut removeLocks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = removeLocks.clone();
            let mut rootNodes: metamodelica::List<i32> = rootNodes.clone();
            let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = taskGraphT.clone();
            let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                threadTasks.clone();
            let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
            let mut vwgt: metamodelica::Array<i32> = vwgt.clone();
            let mut xadj: metamodelica::Array<i32> = xadj.clone();
            (xadj, adjncy, vwgt, adjwgt) = prepareMetis(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
            if intGt(iNumberOfThreads, 1) {
                extInfo = HpcOmSchedulerExt::scheduleMetis(
                    xadj.clone(),
                    adjncy.clone(),
                    vwgt.clone(),
                    adjwgt.clone(),
                    iNumberOfThreads,
                )?;
                extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            } else {
                extInfoArr = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 1);
                extInfo = extInfoArr
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>();
            }
            let true = (intEq(
                metamodelica::arrayLength(iTaskGraph.clone()),
                metamodelica::arrayLength(extInfoArr.clone()),
            )) else {
                return Err("pattern mismatch");
            };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(
                iTaskGraph.clone(),
                metamodelica::arrayLength(iTaskGraph.clone()),
            )?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            priorityArr = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
            createMetisSchedule1(
                &(List::intRange(metamodelica::arrayLength(iTaskGraph.clone()))),
                extInfoArr.clone(),
                iTaskGraph.clone(),
                taskGraphT.clone(),
                priorityArr.clone(),
            )?;
            levelNodes = HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone())?;
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), &iTaskGraphMeta, &convertNodeToTask);
            (priorityTasks, otherTasks) = createMetisSchedule2(
                &levelNodes,
                priorityArr.clone(),
                metamodelica::nil(),
                metamodelica::nil(),
            )?;
            order = listAppend(priorityTasks.clone(), otherTasks.clone());
            procAss = arrayCreate(iNumberOfThreads, metamodelica::nil());
            List::map2_0(
                &(List::intRange(metamodelica::arrayLength(iTaskGraph.clone()))),
                &getProcAss,
                extInfoArr.clone(),
                procAss.clone(),
            )?;
            threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
            removeLocks = metamodelica::nil();
            tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTasks.clone(),
                outgoingDepTasks: metamodelica::nil(),
                scheduledTasks: metamodelica::nil(),
                allCalcTasks: allCalcTasks.clone(),
            });
            (tmpSchedule, removeLocks) = createScheduleFromAssignments(
                extInfoArr.clone(),
                procAss.clone(),
                Some(order.clone()),
                iTaskGraph.clone(),
                taskGraphT.clone(),
                &iTaskGraphMeta,
                iSccSimEqMapping.clone(),
                removeLocks.clone(),
                &(order.clone()),
                iSimVarMapping.clone(),
                tmpSchedule.clone(),
            )?;
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("number of removed superfluous locks: "));
                    __mm_s.push_str(&*intString(intDiv(((removeLocks).len() as i32), 2)));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            tmpSchedule =
                traverseAndUpdateThreadsInSchedule(tmpSchedule.clone(), &removeLocksFromThread, removeLocks.clone())?;
            tmpSchedule =
                updateLockIdcsInThreadschedule(tmpSchedule.clone(), &removeLocksFromLockList, removeLocks.clone())?;
            Ok((
                setScheduleLockIds(&tmpSchedule)?,
                adjncy.clone(),
                adjwgt.clone(),
                allCalcTasks.clone(),
                extInfo.clone(),
                extInfoArr.clone(),
                levelNodes.clone(),
                order.clone(),
                otherTasks.clone(),
                priorityArr.clone(),
                priorityTasks.clone(),
                procAss.clone(),
                removeLocks.clone(),
                rootNodes.clone(),
                taskGraphT.clone(),
                threadTasks.clone(),
                tmpSchedule.clone(),
                vwgt.clone(),
                xadj.clone(),
            ))
        })() {
            adjncy = __wb0;
            adjwgt = __wb1;
            allCalcTasks = __wb2;
            extInfo = __wb3;
            extInfoArr = __wb4;
            levelNodes = __wb5;
            order = __wb6;
            otherTasks = __wb7;
            priorityArr = __wb8;
            priorityTasks = __wb9;
            procAss = __wb10;
            removeLocks = __wb11;
            rootNodes = __wb12;
            taskGraphT = __wb13;
            threadTasks = __wb14;
            tmpSchedule = __wb15;
            vwgt = __wb16;
            xadj = __wb17;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!(
                "HpcOmScheduler.createMetisSchedule not every node has a scheduler-info.\n"
            ));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oSchedule)
}

fn getProcAss(
    mut idx: i32,
    mut taskAss: metamodelica::Array<i32>,
    mut procAss: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut thread: i32;
    thread = metamodelica::arrayGet(taskAss.clone(), idx)?;
    Array::appendToElement(thread, list![idx], procAss.clone())?;
    Ok(())
}

fn createMetisSchedule2<'__b>(
    mut levelNodes: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut priorityArr: metamodelica::Array<i32>,
    mut prioLstIn: metamodelica::List<i32>,
    mut otherLstIn: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match levelNodes {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((prioLstIn, otherLstIn))
            },
            Deref @ metamodelica::ListNode::Cons { head: level, tail: rest } => {
                let mut prioLst: metamodelica::List<i32>;
                let mut otherLst: metamodelica::List<i32>;
                (prioLst, otherLst) = List::split1OnTrue(metamodelica::AsArg::as_arg(&level), &isPrioNode, priorityArr.clone())?;
                prioLst = listAppend(prioLstIn, prioLst);
                otherLst = listAppend(otherLstIn, otherLst);
                { (levelNodes, priorityArr, prioLstIn, otherLstIn) = (rest, priorityArr.clone(), prioLst, otherLst); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn isPrioNode(mut idx: i32, mut prioArr: metamodelica::Array<i32>) -> Result<bool> {
    let mut isPrio: bool;
    isPrio = intEq(1, metamodelica::arrayGet(prioArr.clone(), idx)?);
    Ok(isPrio)
}

fn createMetisSchedule1(
    mut taskIdcs: &metamodelica::List<i32>,
    mut threadIds: metamodelica::Array<i32>,
    mut taskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut priorityArr: metamodelica::Array<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**taskIdcs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: taskIdx, tail: rest } => {
                    let mut preds: metamodelica::List<i32>;
                    let mut rest = (*rest).clone();
                    let true = (intEq(1, metamodelica::arrayGet(priorityArr.clone(), taskIdx.clone())?)) else { return Err("pattern mismatch") };
                    preds = metamodelica::arrayGet(taskGraphT.clone(), taskIdx.clone())?;
                    preds = List::filter1OnTrue(preds.clone(), (std::sync::Arc::new(arrayIntIsNotOne) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), priorityArr.clone())?;
                    List::map2_0(&preds, &Array::updateIndexFirst, 1, priorityArr.clone())?;
                    rest = listAppend(preds.clone(), rest.clone());
                    createMetisSchedule1(metamodelica::AsArg::as_arg(&rest), threadIds.clone(), taskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: taskIdx, tail: rest } => {
                    let mut threadId: i32;
                    let mut preds: metamodelica::List<i32>;
                    let mut predThreads: metamodelica::List<i32>;
                    let mut rest = (*rest).clone();
                    threadId = metamodelica::arrayGet(threadIds.clone(), taskIdx.clone())?;
                    preds = metamodelica::arrayGet(taskGraphT.clone(), taskIdx.clone())?;
                    predThreads = List::map1(preds.clone(), &Array::getIndexFirst, threadIds.clone())?;
                    (predThreads, preds) = List::filter1OnTrueSync(&predThreads, &fnptr!(intNe, i32, i32), threadId, preds.clone())?;
                    if !((predThreads).is_empty()) {
                        List::map2_0(&preds, &Array::updateIndexFirst, 1, priorityArr.clone())?;
                        rest = listAppend(preds.clone(), rest.clone());
                    } else {
                        metamodelica::arrayUpdate(priorityArr.clone(), taskIdx.clone(), 0)?;
                    }
                    createMetisSchedule1(metamodelica::AsArg::as_arg(&rest), threadIds.clone(), taskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn arrayIntIsNotOne(mut idx: i32, mut arr: metamodelica::Array<i32>) -> Result<bool> {
    let mut isOne: bool;
    isOne = intNe(1, metamodelica::arrayGet(arr.clone(), idx)?);
    Ok(isOne)
}

pub(crate) fn createHMetisSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut extInfo: metamodelica::List<i32> = metamodelica::nil();
    let mut xadj: metamodelica::Array<i32> = Default::default();
    let mut adjncy: metamodelica::Array<i32> = Default::default();
    let mut vwgt: metamodelica::Array<i32> = Default::default();
    let mut adjwgt: metamodelica::Array<i32> = Default::default();
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    let mut extInfoArr: metamodelica::Array<i32> = Default::default();
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        Default::default();
    let mut rootNodes: metamodelica::List<i32> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut nodeList_refCount: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = metamodelica::nil();
    let mut nodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11, __wb12)) =
            (|| -> Result<_> {
                let HpcOmTaskGraph::TaskGraphMeta {
                    commCosts: mut commCosts,
                    inComps: mut inComps,
                    ..
                } = __mc_input.clone()
                else {
                    return Err("nomatch");
                };
                let mut adjncy: metamodelica::Array<i32> = adjncy.clone();
                let mut adjwgt: metamodelica::Array<i32> = adjwgt.clone();
                let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> =
                    allCalcTasks.clone();
                let mut extInfo: metamodelica::List<i32> = extInfo.clone();
                let mut extInfoArr: metamodelica::Array<i32> = extInfoArr.clone();
                let mut nodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = nodeList.clone();
                let mut nodeList_refCount: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> =
                    nodeList_refCount.clone();
                let mut rootNodes: metamodelica::List<i32> = rootNodes.clone();
                let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = taskGraphT.clone();
                let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                    threadTasks.clone();
                let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                let mut vwgt: metamodelica::Array<i32> = vwgt.clone();
                let mut xadj: metamodelica::Array<i32> = xadj.clone();
                metamodelica::print(literal!("Funktionsaufruf!"));
                (xadj, adjncy, vwgt, adjwgt) = preparehMetis(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
                extInfo = HpcOmSchedulerExt::schedulehMetis(
                    xadj.clone(),
                    adjncy.clone(),
                    vwgt.clone(),
                    adjwgt.clone(),
                    iNumberOfThreads,
                )?;
                extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
                metamodelica::print(literal!("Hier geht MetaModelica los!\n"));
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("External scheduling info: "));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(extInfo.clone(), &fnptr!(intString, i32))?,
                        literal!(","),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                let true = (intEq(
                    metamodelica::arrayLength(iTaskGraph.clone()),
                    metamodelica::arrayLength(extInfoArr.clone()),
                )) else {
                    return Err("pattern mismatch");
                };
                taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(
                    iTaskGraph.clone(),
                    metamodelica::arrayLength(iTaskGraph.clone()),
                )?;
                rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
                allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), &iTaskGraphMeta, &convertNodeToTask);
                nodeList_refCount = List::map1(rootNodes.clone(), &getTaskByIndex, allCalcTasks.clone())?;
                nodeList = List::map(nodeList_refCount.clone(), &fnptr!(Util::tuple21, _))?;
                nodeList = List::sort(
                    nodeList.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<HpcOmSimCode::Task>,
                              __a1: metamodelica::Ref<HpcOmSimCode::Task>| {
                            compareTasksByWeighting(&__a0, &__a1)
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<HpcOmSimCode::Task>,
                                    metamodelica::Ref<HpcOmSimCode::Task>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                )?;
                threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
                tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                    threadTasks: threadTasks.clone(),
                    outgoingDepTasks: metamodelica::nil(),
                    scheduledTasks: metamodelica::nil(),
                    allCalcTasks: allCalcTasks.clone(),
                });
                tmpSchedule = createExtSchedule1(&nodeList, extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>, __a2: i32, __a3: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>, __a4: metamodelica::Array<metamodelica::List<i32>>, __a5: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>| getLocksByPredecessorList(&__a0, &__a1, __a2, __a3, __a4, __a5), &tmpSchedule)?;
                tmpSchedule = addSuccessorLocksToSchedule(
                    iTaskGraph.clone(),
                    (std::sync::Arc::new(
                        move |__a0: (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                              __a1: metamodelica::Ref<HpcOmSimCode::Task>,
                              __a2: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                              __a3: metamodelica::Array<metamodelica::List<i32>>,
                              __a4: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                              __a5: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>| {
                            addReleaseLocksToSchedule(&__a0, __a1, __a2, __a3, __a4, __a5)
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                                    metamodelica::Ref<HpcOmSimCode::Task>,
                                    metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                                    metamodelica::Array<metamodelica::List<i32>>,
                                    metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                                    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                                )
                                    -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
                                + 'static,
                        >),
                    commCosts.clone(),
                    inComps.clone(),
                    iSimVarMapping.clone(),
                    &tmpSchedule,
                )?;
                Ok((
                    setScheduleLockIds(&tmpSchedule)?,
                    adjncy.clone(),
                    adjwgt.clone(),
                    allCalcTasks.clone(),
                    extInfo.clone(),
                    extInfoArr.clone(),
                    nodeList.clone(),
                    nodeList_refCount.clone(),
                    rootNodes.clone(),
                    taskGraphT.clone(),
                    threadTasks.clone(),
                    tmpSchedule.clone(),
                    vwgt.clone(),
                    xadj.clone(),
                ))
            })()
        {
            adjncy = __wb0;
            adjwgt = __wb1;
            allCalcTasks = __wb2;
            extInfo = __wb3;
            extInfoArr = __wb4;
            nodeList = __wb5;
            nodeList_refCount = __wb6;
            rootNodes = __wb7;
            taskGraphT = __wb8;
            threadTasks = __wb9;
            tmpSchedule = __wb10;
            vwgt = __wb11;
            xadj = __wb12;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!(
                "HpcOmScheduler.createHMetisSchedule not every node has a scheduler-info.\n"
            ));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oSchedule)
}

fn sumEdge(mut edges: &metamodelica::List<i32>, mut innumedge: i32) -> i32 {
    let mut outnumedge: i32;
    outnumedge = innumedge + ((edges).len() as i32);
    outnumedge
}

fn getSingleRelations(
    mut edge: i32,
    mut n: i32,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut irelations: metamodelica::List<(i32, i32, i32)>,
) -> Result<metamodelica::List<(i32, i32, i32)>> {
    let mut orelations: metamodelica::List<(i32, i32, i32)>;
    let mut costs: metamodelica::Real;
    let mut costsInt: i32;
    costs = HpcOmTaskGraph::getCommCostTimeBetweenNodes(n, edge, iTaskGraphMeta)?;
    costsInt = ((costs).0.floor() as i32);
    orelations = List::appendElt((edge, n, costsInt), irelations);
    orelations = List::appendElt((n, edge, costsInt), orelations);
    Ok(orelations)
}

fn getRelations(
    mut edges: &metamodelica::List<i32>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut irelations: &(metamodelica::List<(i32, i32, i32)>, i32),
) -> Result<(metamodelica::List<(i32, i32, i32)>, i32)> {
    let mut orelations: (metamodelica::List<(i32, i32, i32)>, i32);
    let mut n: i32;
    let mut relations: metamodelica::List<(i32, i32, i32)>;
    let mut orel: metamodelica::List<(i32, i32, i32)>;
    (relations, n) = irelations.clone();
    orel = List::fold2(edges, &getSingleRelations, n, iTaskGraphMeta, relations)?;
    orelations = (orel, n + 1);
    Ok(orelations)
}

fn sortEdgeHelp(
    mut edge: (i32, i32, i32),
    mut actnode: i32,
    mut adjncy: metamodelica::Array<i32>,
    mut adjwgt: metamodelica::Array<i32>,
    mut imarker: i32,
) -> i32 {
    let mut omarker: i32;
    omarker = 'mc: {
        let __mc_input = edge;
        if let Ok(__v) = (|| -> Result<_> {
            let (mut fromnode, mut tonode, mut cost) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (intEq(fromnode.clone(), actnode)) else {
                return Err("pattern mismatch");
            };
            metamodelica::arrayUpdate(adjwgt.clone(), imarker, cost.clone())?;
            metamodelica::arrayUpdate(adjncy.clone(), imarker, tonode.clone() - 1)?;
            Ok(imarker + 1)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(imarker)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    omarker
}

fn sortEdge(
    mut actnode: i32,
    mut xadj: metamodelica::Array<i32>,
    mut adjncy: metamodelica::Array<i32>,
    mut adjwgt: metamodelica::Array<i32>,
    mut help: &metamodelica::List<(i32, i32, i32)>,
    mut imarker: i32,
) -> Result<i32> {
    let mut omarker: i32;
    omarker = List::fold3(
        help,
        &fnptr!(
            sortEdgeHelp,
            (i32, i32, i32),
            i32,
            metamodelica::Array<i32>,
            metamodelica::Array<i32>,
            i32
        ),
        actnode,
        adjncy.clone(),
        adjwgt.clone(),
        imarker,
    )?;
    metamodelica::arrayUpdate(xadj.clone(), actnode + 1, omarker - 1)?;
    Ok(omarker)
}

fn setVwgt(
    mut node: i32,
    mut vwgt: metamodelica::Array<i32>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<()> {
    let mut value: (i32, metamodelica::Real);
    let mut rv: metamodelica::Real;
    value = HpcOmTaskGraph::getExeCost(node, iTaskGraphMeta)?;
    (_, rv) = value;
    metamodelica::arrayUpdate(vwgt.clone(), node, ((rv).0.floor() as i32))?;
    Ok(())
}

fn prepareMetis(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
)> {
    let mut xadj: metamodelica::Array<i32>;
    let mut adjncy: metamodelica::Array<i32>;
    let mut vwgt: metamodelica::Array<i32>;
    let mut adjwgt: metamodelica::Array<i32>;
    let mut n: i32;
    let mut m: i32;
    let mut adjundirected: (metamodelica::List<(i32, i32, i32)>, i32);
    let mut help: metamodelica::List<(i32, i32, i32)>;
    let mut allTheNodes: metamodelica::List<i32>;
    help = metamodelica::nil();
    n = metamodelica::arrayLength(iTaskGraph.clone());
    xadj = arrayCreate(n + 1, 0);
    m = Array::fold(
        iTaskGraph.clone(),
        &move |__a0: metamodelica::List<i32>, __a1: i32| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(sumEdge(&__a0, __a1))
        },
        0,
    )?;
    adjwgt = arrayCreate(2 * m, 0);
    adjundirected = Array::fold(
        iTaskGraph.clone(),
        &({
            let __pe_b1 = iTaskGraphMeta.clone();
            move |__pe_a0, __pe_a2| getRelations(&__pe_a0, __pe_b1.clone(), &__pe_a2)
        }),
        (metamodelica::nil(), 1),
    )?;
    (help, _) = adjundirected;
    allTheNodes = List::intRange(n);
    adjncy = arrayCreate(2 * m, 0);
    xadj = metamodelica::arrayUpdate(xadj.clone(), 1, 0)?;
    List::fold4(
        &allTheNodes,
        &move |__a0: i32,
               __a1: metamodelica::Array<i32>,
               __a2: metamodelica::Array<i32>,
               __a3: metamodelica::Array<i32>,
               __a4: metamodelica::List<(i32, i32, i32)>,
               __a5: i32| sortEdge(__a0, __a1, __a2, __a3, &__a4, __a5),
        xadj.clone(),
        adjncy.clone(),
        adjwgt.clone(),
        help,
        1,
    )?;
    vwgt = arrayCreate(n, 0);
    List::map2_0(&allTheNodes, &setVwgt, vwgt.clone(), iTaskGraphMeta)?;
    Ok((xadj, adjncy, vwgt, adjwgt))
}

fn listNodes(mut node: i32, mut l_eint: metamodelica::List<i32>) -> metamodelica::List<i32> {
    let mut l_eint_out: metamodelica::List<i32>;
    let mut actnode: i32;
    actnode = node - 1;
    l_eint_out = listAppend(l_eint, list![actnode]);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("l_eint length:"));
        __mm_s.push_str(&*intString(((l_eint_out).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    l_eint_out
}

fn getHedge(
    mut childnodes: &metamodelica::List<i32>,
    mut actnode: &(
        i32,
        i32,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
) -> Result<(
    i32,
    i32,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut actnode_out: (
        i32,
        i32,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    actnode_out = (::match_deref::match_deref! { match &((&**childnodes, actnode)) {
        (Deref @ metamodelica::ListNode::Nil, (node, position, l_eptr, l_eint, l_hewgts)) => {
            let mut help: (i32, i32, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>);
            help = (node.clone() + 1, position.clone(), l_eptr.clone(), l_eint.clone(), l_hewgts.clone());
            help
        },
        (_, (node, position, l_eptr, l_eint, l_hewgts)) => {
            let mut n: i32;
            let mut help: (i32, i32, metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>);
            let mut l_eptr = (*l_eptr).clone();
            let mut l_eint = (*l_eint).clone();
            n = node.clone() - 1;
            l_eint = List::appendElt(n, l_eint.clone());
            l_eint = List::fold(childnodes, &fnptr!(listNodes, i32, metamodelica::List<i32>), l_eint.clone())?;
            n = position.clone() + ((childnodes).len() as i32) + 1;
            l_eptr = List::appendElt(n, l_eptr.clone());
            help = (node.clone() + 1, n, l_eptr.clone(), l_eint.clone(), l_hewgts.clone());
            help
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(actnode_out)
}

fn preparehMetis(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
)> {
    let mut vwgts: metamodelica::Array<i32>;
    let mut eptr: metamodelica::Array<i32>;
    let mut eint: metamodelica::Array<i32>;
    let mut hewgts: metamodelica::Array<i32>;
    let mut n: i32;
    let mut l_eptr: metamodelica::List<i32>;
    let mut l_eint: metamodelica::List<i32>;
    let mut l_hewgts: metamodelica::List<i32>;
    let mut allTheNodes: metamodelica::List<i32>;
    let mut result: (
        i32,
        i32,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    n = metamodelica::arrayLength(iTaskGraph.clone());
    result = Array::fold(
        iTaskGraph.clone(),
        &move |__a0: metamodelica::List<i32>,
               __a1: (
            i32,
            i32,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        )| getHedge(&__a0, &__a1),
        (1, 0, list![0], metamodelica::nil(), metamodelica::nil()),
    )?;
    (_, _, l_eptr, l_eint, l_hewgts) = result;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Diagnostic length: "));
        __mm_s.push_str(&*intString(((l_eptr).len() as i32)));
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*intString(((l_eint).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    allTheNodes = List::intRange(n);
    vwgts = arrayCreate(n, 0);
    List::map2_0(&allTheNodes, &setVwgt, vwgts.clone(), iTaskGraphMeta)?;
    eptr = metamodelica::arrayFromVec(l_eptr.into_iter().cloned().collect());
    eint = metamodelica::arrayFromVec(l_eint.into_iter().cloned().collect());
    hewgts = metamodelica::arrayFromVec(l_hewgts.into_iter().cloned().collect());
    Ok((vwgts, eptr, eint, hewgts))
}

//--------------------
// External Scheduling //TODO: Rename to Yed Scheduling
//--------------------
pub(crate) fn createExtSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iGraphMLFile: ArcStr,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut extInfo: metamodelica::List<i32> = metamodelica::nil();
    let mut extInfoArr: metamodelica::Array<i32> = Default::default();
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        Default::default();
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut rootNodes: metamodelica::List<i32> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut nodeList_refCount: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = metamodelica::nil();
    let mut nodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta {
                commCosts: mut commCosts,
                inComps: mut inComps,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> =
                allCalcTasks.clone();
            let mut extInfo: metamodelica::List<i32> = extInfo.clone();
            let mut extInfoArr: metamodelica::Array<i32> = extInfoArr.clone();
            let mut nodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = nodeList.clone();
            let mut nodeList_refCount: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> =
                nodeList_refCount.clone();
            let mut rootNodes: metamodelica::List<i32> = rootNodes.clone();
            let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>> = taskGraphT.clone();
            let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                threadTasks.clone();
            let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
            extInfo = HpcOmSchedulerExt::readScheduleFromGraphMl(iGraphMLFile.clone())?;
            extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            let true = (intEq(
                metamodelica::arrayLength(iTaskGraph.clone()),
                metamodelica::arrayLength(extInfoArr.clone()),
            )) else {
                return Err("pattern mismatch");
            };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(
                iTaskGraph.clone(),
                metamodelica::arrayLength(iTaskGraph.clone()),
            )?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta, &convertNodeToTask);
            nodeList_refCount = List::map1(rootNodes.clone(), &getTaskByIndex, allCalcTasks.clone())?;
            nodeList = List::map(nodeList_refCount.clone(), &fnptr!(Util::tuple21, _))?;
            nodeList = List::sort(
                nodeList.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| {
                        compareTasksByWeighting(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<HpcOmSimCode::Task>,
                                metamodelica::Ref<HpcOmSimCode::Task>,
                            ) -> Result<bool>
                            + 'static,
                    >),
            )?;
            threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
            tmpSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTasks.clone(),
                outgoingDepTasks: metamodelica::nil(),
                scheduledTasks: metamodelica::nil(),
                allCalcTasks: allCalcTasks.clone(),
            });
            tmpSchedule = createExtSchedule1(&nodeList, extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>, __a2: i32, __a3: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>, __a4: metamodelica::Array<metamodelica::List<i32>>, __a5: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>| getLocksByPredecessorList(&__a0, &__a1, __a2, __a3, __a4, __a5), &tmpSchedule)?;
            tmpSchedule = addSuccessorLocksToSchedule(
                iTaskGraph.clone(),
                (std::sync::Arc::new(
                    move |__a0: (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                          __a1: metamodelica::Ref<HpcOmSimCode::Task>,
                          __a2: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                          __a3: metamodelica::Array<metamodelica::List<i32>>,
                          __a4: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                          __a5: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>| {
                        addReleaseLocksToSchedule(&__a0, __a1, __a2, __a3, __a4, __a5)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                (metamodelica::Ref<HpcOmSimCode::Task>, i32),
                                metamodelica::Ref<HpcOmSimCode::Task>,
                                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
                                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                            )
                                -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
                            + 'static,
                    >),
                commCosts.clone(),
                inComps.clone(),
                iSimVarMapping.clone(),
                &tmpSchedule,
            )?;
            Ok((
                tmpSchedule.clone(),
                allCalcTasks.clone(),
                extInfo.clone(),
                extInfoArr.clone(),
                nodeList.clone(),
                nodeList_refCount.clone(),
                rootNodes.clone(),
                taskGraphT.clone(),
                threadTasks.clone(),
                tmpSchedule.clone(),
            ))
        })() {
            allCalcTasks = __wb0;
            extInfo = __wb1;
            extInfoArr = __wb2;
            nodeList = __wb3;
            nodeList_refCount = __wb4;
            rootNodes = __wb5;
            taskGraphT = __wb6;
            threadTasks = __wb7;
            tmpSchedule = __wb8;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!(
                "HpcOmScheduler.createExtSchedule not every node has a scheduler-info.\n"
            ));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oSchedule)
}

fn createExtSchedule1(
    mut iNodeList: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iThreadAssignments: metamodelica::Array<i32>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iLockWithPredecessorHandler: &dyn ::std::ops::Fn(
        metamodelica::Ref<HpcOmSimCode::Task>,
        metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
        i32,
        metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    )>,
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<HpcOmSimCode::Task>,
                metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
                i32,
                metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
            ) -> Result<(
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
            )> + 'static,
    >;

    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut head: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTaskRefCount: i32 = 0;
    let mut rest: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut predecessors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = metamodelica::nil();
    let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = metamodelica::nil();
    let mut successorIdc: metamodelica::List<i32> = metamodelica::nil();
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut newOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut firstEq: i32;
    let mut allThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut lockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut weighting: i32;
    let mut index: i32;
    let mut calcTime: metamodelica::Real;
    let mut eqIdc: metamodelica::List<i32>;
    let mut simEqIdc: metamodelica::List<i32> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    oSchedule = 'mc: {
        let __mc_input = (&**iNodeList, &**iSchedule);
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11, __wb12)) =
            (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::ListNode::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                        let mut allThreadTasks = (*allThreadTasks).clone();
                        let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                        let mut allCalcTasks = (*allCalcTasks).clone();
                        let mut lockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = lockTasks.clone();
                        let mut newOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = newOutgoingDepTasks.clone();
                        let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = newTask.clone();
                        let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                        let mut predecessors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = predecessors.clone();
                        let mut simEqIdc: metamodelica::List<i32> = simEqIdc.clone();
                        let mut successorIdc: metamodelica::List<i32> = successorIdc.clone();
                        let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = successors.clone();
                        let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                        let mut threadId: i32 = threadId.clone();
                        let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = threadTasks.clone();
                        let mut tmpNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = tmpNodeList.clone();
                        let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                        (predecessors, _) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraphT.clone(), allCalcTasks.clone())?;
                        (successors, successorIdc) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraph.clone(), allCalcTasks.clone())?;
                        let false = ((predecessors).is_empty()) else { return Err("pattern mismatch") };
                        threadId = metamodelica::arrayGet(iThreadAssignments.clone(), index.clone())?;
                        threadFinishTime = metamodelica::OrderedFloat(-1.0_f64);
                        threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                        (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors.clone(), threadId, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                        outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks.clone());
                        threadTasks = listAppend(lockTasks.clone(), threadTasks.clone());
                        simEqIdc = List::map(List::map1(eqIdc.clone(), &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?, &move |__a0: _| List::last(&__a0))?;
                        newTask = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                        threadTasks = metamodelica::cons(newTask.clone(), threadTasks.clone());
                        allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks.clone())?;
                        (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), &successorIdc, &(metamodelica::nil()));
                        tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                        tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| compareTasksByWeighting(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::Ref<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                        (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                        metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                        tmpSchedule = createExtSchedule1(&tmpNodeList, iThreadAssignments.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler, &(metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() })))?;
                        Ok((tmpSchedule.clone(), lockTasks.clone(), newOutgoingDepTasks.clone(), newTask.clone(), newTaskRefCount.clone(), predecessors.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })()
        {
            lockTasks = __wb0;
            newOutgoingDepTasks = __wb1;
            newTask = __wb2;
            newTaskRefCount = __wb3;
            predecessors = __wb4;
            simEqIdc = __wb5;
            successorIdc = __wb6;
            successors = __wb7;
            threadFinishTime = __wb8;
            threadId = __wb9;
            threadTasks = __wb10;
            tmpNodeList = __wb11;
            tmpSchedule = __wb12;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::ListNode::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut newTask: metamodelica::Ref<HpcOmSimCode::Task> = newTask.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut simEqIdc: metamodelica::List<i32> = simEqIdc.clone();
                    let mut successorIdc: metamodelica::List<i32> = successorIdc.clone();
                    let mut successors: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, i32)> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = threadTasks.clone();
                    let mut tmpNodeList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = tmpNodeList.clone();
                    let mut tmpSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                    (successors, successorIdc) = getSuccessorsByTask(metamodelica::AsArg::as_arg(&head), iTaskGraph.clone(), allCalcTasks.clone())?;
                    threadId = metamodelica::arrayGet(iThreadAssignments.clone(), index.clone())?;
                    threadFinishTime = metamodelica::OrderedFloat(-1.0_f64);
                    threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                    simEqIdc = List::flatten(List::map1(eqIdc.clone(), &getSimEqSysIdxForComp, iSccSimEqMapping.clone())?)?;
                    newTask = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, metamodelica::cons(newTask.clone(), threadTasks.clone()))?;
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), &successorIdc, &(metamodelica::nil()));
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: metamodelica::Ref<HpcOmSimCode::Task>| compareTasksByWeighting(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::Ref<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                    (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                    metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                    tmpSchedule = createExtSchedule1(&tmpNodeList, iThreadAssignments.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler, &(metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() })))?;
                    Ok((tmpSchedule.clone(), newTask.clone(), newTaskRefCount.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            newTask = __wb0;
            newTaskRefCount = __wb1;
            simEqIdc = __wb2;
            successorIdc = __wb3;
            successors = __wb4;
            threadFinishTime = __wb5;
            threadId = __wb6;
            threadTasks = __wb7;
            tmpNodeList = __wb8;
            tmpSchedule = __wb9;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(iSchedule.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("HpcOmScheduler.createExtSchedule1 failed. Tasks in List:\n"));
                    printTaskList(iNodeList.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oSchedule)
}

//---------------------------------
// Task Duplication-based Scheduler
//---------------------------------
pub(crate) fn TDS_schedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut numProc: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimCode: &metamodelica::Ref<SimCode::SimCode>,
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
    let mut size: i32;
    let mut queue: metamodelica::List<i32>;
    let mut levels: metamodelica::List<metamodelica::Real>;
    let mut ectArray: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevelArray: metamodelica::Array<metamodelica::Real>;
    let mut lastArray: metamodelica::Array<metamodelica::Real>;
    let mut lactArray: metamodelica::Array<metamodelica::Real>;
    let mut fpredArray: metamodelica::Array<i32>;
    let mut initClusters: metamodelica::List<metamodelica::List<i32>>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        commCosts: __pa0,
        inComps: __pa1,
        ..
    } = &iTaskGraphMeta;
    commCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    (_, _, ectArray) = computeGraphValuesBottomUp(iTaskGraph.clone(), &iTaskGraphMeta)?;
    (_, lastArray, lactArray, tdsLevelArray) = computeGraphValuesTopDown(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    fpredArray = computeFavouritePred(iTaskGraph.clone(), iTaskGraphMeta.clone(), ectArray.clone())?;
    (levels, queue) = quicksortWithOrder(
        tdsLevelArray
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
    )?;
    initClusters = TDS_InitialCluster(
        iTaskGraph.clone(),
        taskGraphT.clone(),
        &iTaskGraphMeta,
        lastArray.clone(),
        lactArray.clone(),
        fpredArray.clone(),
        &queue,
    )?;
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = TDS_schedule1(
        &initClusters,
        iTaskGraph.clone(),
        taskGraphT.clone(),
        &iTaskGraphMeta,
        tdsLevelArray.clone(),
        numProc,
        iSccSimEqMapping.clone(),
        iSimCode,
        commCosts.clone(),
        inComps.clone(),
        iSimVarMapping.clone(),
    )?;
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn insertLocksInSchedule(
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut taskAss: metamodelica::Array<i32>,
    mut procAss: metamodelica::Array<metamodelica::List<i32>>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut threads: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*iSchedule)) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa0, allCalcTasks: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    threadTasks = metamodelica::Own::own(__pa0);
    allCalcTasks = metamodelica::Own::own(__pa1);
    threads = threadTasks
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    (threads, outgoingDepTasks) = List::fold(
        &threads,
        &({
            let __pe_b1 = (iTaskGraph.clone(), iTaskGraphT.clone());
            let __pe_b2 = (taskAss.clone(), procAss.clone());
            let __pe_b3 = allCalcTasks.clone();
            let __pe_b4 = iCommCosts.clone();
            let __pe_b5 = iCompTaskMapping.clone();
            let __pe_b6 = iSimVarMapping.clone();
            move |__pe_a0, __pe_a7| {
                insertLocksInSchedule1(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    __pe_b6.clone(),
                    __pe_a7,
                )
            }
        }),
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    threads = List::filterOnFalse(threads, &fnptr!(listEmpty, _))?;
    threads = List::map(
        threads,
        &fnptr!(
            metamodelica::listReverse,
            metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>
        ),
    )?;
    threads = threads.reverse();
    threadTasks = metamodelica::arrayFromVec(threads.into_iter().cloned().collect());
    outgoingDepTasks = List::unique(&outgoingDepTasks);
    oSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: threadTasks.clone(),
        outgoingDepTasks: outgoingDepTasks,
        scheduledTasks: metamodelica::nil(),
        allCalcTasks: allCalcTasks.clone(),
    });
    Ok(oSchedule)
}

fn insertLocksInSchedule1(
    mut threadsIn: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iTaskGraphTransposed: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
    mut taskProcAss: (metamodelica::Array<i32>, metamodelica::Array<metamodelica::List<i32>>),
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut foldIn: (
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((threadsIn.clone(), iTaskGraphTransposed.clone(), taskProcAss.clone(), foldIn)) {
            (Deref @ metamodelica::ListNode::Nil, _, _, (threads, outgoingDepTasks)) => {
                let mut threads = (*threads).clone();
                threads = metamodelica::cons(metamodelica::nil(), threads.clone());
                return Ok((threads.clone(), outgoingDepTasks.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ HpcOmSimCode::Task::CALCTASK { index: idx, threadIdx: thr, .. }, tail: rest }, (iTaskGraph, iTaskGraphT), (taskAss, _), (threads, outgoingDepTasks)) => {
                let mut preds: metamodelica::List<i32>;
                let mut succs: metamodelica::List<i32>;
                let mut predThr: metamodelica::List<i32>;
                let mut succThr: metamodelica::List<i32>;
                let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut relLocks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut assLocks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut tasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                let mut threads = (*threads).clone();
                let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                task = (threadsIn).head().cloned()?;
                preds = metamodelica::arrayGet(iTaskGraphT.clone(), idx.clone())?;
                succs = metamodelica::arrayGet(iTaskGraph.clone(), idx.clone())?;
                predThr = List::map1(preds.clone(), &Array::getIndexFirst, taskAss.clone())?;
                succThr = List::map1(succs.clone(), &Array::getIndexFirst, taskAss.clone())?;
                (_, preds) = List::filter1OnTrueSync(&predThr, &fnptr!(intNe, i32, i32), thr.clone(), preds)?;
                (_, succs) = List::filter1OnTrueSync(&succThr, &fnptr!(intNe, i32, i32), thr.clone(), succs)?;
                assLocks = List::map6(preds, &createDepTaskByTaskIdc, idx.clone(), iAllCalcTasks.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                relLocks = List::map6(succs, &createDepTaskByTaskIdc, idx.clone(), iAllCalcTasks.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                tasks = listAppend(listAppend(relLocks.clone(), list![task]), assLocks.clone());
                thread = if (!((threads).is_empty())) {(threads).head().cloned()?} else {metamodelica::nil()};
                thread = listAppend(tasks, thread);
                threads = if (!((threads).is_empty())) {List::replaceAt(thread, 1, threads.clone())?} else {list![thread]};
                outgoingDepTasks = listAppend(relLocks, outgoingDepTasks.clone());
                outgoingDepTasks = listAppend(assLocks, outgoingDepTasks.clone());
                { (threadsIn, iTaskGraphTransposed, taskProcAss, iAllCalcTasks, iCommCosts, iCompTaskMapping, iSimVarMapping, foldIn) = (rest.clone(), iTaskGraphTransposed, taskProcAss, iAllCalcTasks.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), (threads.clone(), outgoingDepTasks.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn TDS_schedule1(
    mut clustersIn: &metamodelica::List<metamodelica::List<i32>>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut TDSLevel: metamodelica::Array<metamodelica::Real>,
    mut numProc: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimCode: &metamodelica::Ref<SimCode::SimCode>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
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
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = 'mc: {
        let __mc_input = iSimVarMapping.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut sccSimEqMap: metamodelica::Array<metamodelica::List<i32>>;
            let mut clusters: metamodelica::List<metamodelica::List<i32>>;
            let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
            let mut meta: HpcOmTaskGraph::TaskGraphMeta;
            let mut simCode: metamodelica::Ref<SimCode::SimCode>;
            let true = (((clustersIn).len() as i32) < numProc) else {
                return Err("pattern mismatch");
            };
            metamodelica::print(literal!(
                "There are less initial clusters than processors. we need duplication, but since this is a rare case, it is not done. Less processors are used.\n"
            ));
            clusters = List::map(
                clustersIn.clone(),
                &fnptr!(metamodelica::listReverse, metamodelica::List<i32>),
            )?;
            FlagsUtil::setConfigInt(Flags::NUM_PROC.clone(), ((clustersIn).len() as i32))?;
            (schedule, simCode, taskGraph, meta, sccSimEqMap) = TDS_schedule1(
                &clusters,
                iTaskGraph.clone(),
                iTaskGraphT.clone(),
                iTaskGraphMeta,
                TDSLevel.clone(),
                ((clustersIn).len() as i32),
                iSccSimEqMapping.clone(),
                iSimCode,
                iCommCosts.clone(),
                iCompTaskMapping.clone(),
                iSimVarMapping.clone(),
            )?;
            Ok((
                schedule.clone(),
                simCode.clone(),
                taskGraph.clone(),
                meta.clone(),
                sccSimEqMap.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut sccSimEqMap: metamodelica::Array<metamodelica::List<i32>>;
            let mut clusters: metamodelica::List<metamodelica::List<i32>>;
            let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
            let mut meta: HpcOmTaskGraph::TaskGraphMeta;
            let mut simCode: metamodelica::Ref<SimCode::SimCode>;
            let true = (((clustersIn).len() as i32) > numProc) else {
                return Err("pattern mismatch");
            };
            clusters = TDS_CompactClusters(
                clustersIn.clone(),
                iTaskGraph.clone(),
                iTaskGraphMeta.clone(),
                TDSLevel.clone(),
                numProc,
            )?;
            (schedule, simCode, taskGraph, meta, sccSimEqMap) = TDS_schedule1(
                &clusters,
                iTaskGraph.clone(),
                iTaskGraphT.clone(),
                iTaskGraphMeta,
                TDSLevel.clone(),
                numProc,
                iSccSimEqMapping.clone(),
                iSimCode,
                iCommCosts.clone(),
                iCompTaskMapping.clone(),
                iSimVarMapping.clone(),
            )?;
            Ok((
                schedule.clone(),
                simCode.clone(),
                taskGraph.clone(),
                meta.clone(),
                sccSimEqMap.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut sizeTasks: i32;
            let mut numDupl: i32;
            let mut threadIdx: i32;
            let mut compIdx: i32;
            let mut simVarIdx: i32;
            let mut simEqSysIdx: i32;
            let mut taskIdx: i32;
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut taskAss: metamodelica::Array<i32>;
            let mut taskDuplAss: metamodelica::Array<i32>;
            let mut nodeMark: metamodelica::Array<i32>;
            let mut newIdxAss: metamodelica::Array<i32>;
            let mut procAss: metamodelica::Array<metamodelica::List<i32>>;
            let mut sccSimEqMap: metamodelica::Array<metamodelica::List<i32>>;
            let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
            let mut comps: metamodelica::Array<metamodelica::List<i32>>;
            let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
            let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
            let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
            let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
            let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
            let mut compNames: metamodelica::Array<ArcStr>;
            let mut compDescs: metamodelica::Array<ArcStr>;
            let mut clusters: metamodelica::List<metamodelica::List<i32>>;
            let mut duplSccSimEqMap: metamodelica::List<metamodelica::List<i32>>;
            let mut duplComps: metamodelica::List<metamodelica::List<i32>>;
            let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
            let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
            let mut meta: HpcOmTaskGraph::TaskGraphMeta;
            let mut simCode: metamodelica::Ref<SimCode::SimCode>;
            let mut simVars: SimCodeVar::SimVars;
            let mut algVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
            let mut threadTask: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
            let mut odes: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
            let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
            let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
            let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
            let true = (((clustersIn).len() as i32) == numProc) else {
                return Err("pattern mismatch");
            };
            clusters = List::map1(
                clustersIn.clone(),
                &move |__a0: metamodelica::List<i32>, __a1: metamodelica::Array<metamodelica::Real>| {
                    TDS_SortCompactClusters(&__a0, __a1)
                },
                TDSLevel.clone(),
            )?;
            let __arc2 = &(*iSimCode);
            let SimCode::SIMCODE {
                modelInfo: SimCode::MODELINFO { vars: __pa0, .. },
                odeEquations: __pa1,
                ..
            } = &**__arc2;
            simVars = metamodelica::Own::own(__pa0);
            odes = metamodelica::Own::own(__pa1);
            let SimCodeVar::SIMVARS { algVars: __pa3, .. } = &simVars;
            algVars = metamodelica::Own::own(__pa3);
            let HpcOmTaskGraph::TASKGRAPHMETA {
                inComps: __pa4,
                varCompMapping: __pa5,
                eqCompMapping: __pa6,
                compParamMapping: __pa7,
                compNames: __pa8,
                compDescs: __pa9,
                exeCosts: __pa10,
                commCosts: __pa11,
                nodeMark: __pa12,
                compInformations: __pa13,
            } = &iTaskGraphMeta;
            inComps = metamodelica::Own::own(__pa4);
            varCompMapping = metamodelica::Own::own(__pa5);
            eqCompMapping = metamodelica::Own::own(__pa6);
            compParamMapping = metamodelica::Own::own(__pa7);
            compNames = metamodelica::Own::own(__pa8);
            compDescs = metamodelica::Own::own(__pa9);
            exeCosts = metamodelica::Own::own(__pa10);
            commCosts = metamodelica::Own::own(__pa11);
            nodeMark = metamodelica::Own::own(__pa12);
            compInformations = metamodelica::Own::own(__pa13);
            sizeTasks = List::fold(
                &(List::map(clusters.clone(), &fnptr!(listLength, _))?),
                &fnptr!(intAdd, i32, i32),
                0,
            )?;
            taskAss = arrayCreate(sizeTasks, -1);
            procAss = arrayCreate(((clusters).len() as i32), metamodelica::nil());
            taskGraph = arrayCreate(sizeTasks, metamodelica::nil());
            taskDuplAss = arrayCreate(sizeTasks, -1);
            threadTask = arrayCreate(numProc, metamodelica::nil());
            allCalcTasks = arrayCreate(
                sizeTasks,
                (openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY(), 0),
            );
            schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTask.clone(),
                outgoingDepTasks: metamodelica::nil(),
                scheduledTasks: metamodelica::nil(),
                allCalcTasks: allCalcTasks.clone(),
            });
            duplSccSimEqMap = metamodelica::nil();
            duplComps = metamodelica::nil();
            threadIdx = 1;
            compIdx = metamodelica::arrayLength(iSccSimEqMapping.clone()) + 1;
            taskIdx = metamodelica::arrayLength(iTaskGraph.clone()) + 1;
            simVarIdx = ({
                let mut __acc: Option<i32> = None;
                for mut v in (algVars.clone()).into_iter().cloned() {
                    let __x = v.index.clone();
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
            }) + 1;
            simEqSysIdx = SimCodeCodegenUtil::getMaxSimEqSystemIndex(iSimCode)? + 1;
            lsIdx = List::fold(
                &(List::map(
                    List::flatten(odes.clone())?,
                    &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(SimCodeUtil::getLSindex(&__a0))
                    },
                )?),
                &fnptr!(intMax, i32, i32),
                0,
            )? + 1;
            nlsIdx = List::fold(
                &(List::map(
                    List::flatten(odes.clone())?,
                    &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(SimCodeUtil::getNLSindex(&__a0))
                    },
                )?),
                &fnptr!(intMax, i32, i32),
                0,
            )? + 1;
            mIdx = List::fold(
                &(List::map(
                    List::flatten(odes.clone())?,
                    &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(SimCodeUtil::getMixedindex(&__a0))
                    },
                )?),
                &fnptr!(intMax, i32, i32),
                0,
            )? + 1;
            (
                taskAss,
                procAss,
                taskGraph,
                taskDuplAss,
                idcs,
                simCode,
                schedule,
                duplSccSimEqMap,
                duplComps,
            ) = TDS_duplicateTasks(
                &clusters,
                taskAss.clone(),
                procAss.clone(),
                (threadIdx, taskIdx, compIdx, simVarIdx, simEqSysIdx, lsIdx, nlsIdx, mIdx),
                iTaskGraph.clone(),
                iTaskGraphT.clone(),
                taskGraph.clone(),
                taskDuplAss.clone(),
                iTaskGraphMeta,
                iSimCode,
                &schedule,
                iSccSimEqMapping.clone(),
                &duplSccSimEqMap,
                &duplComps,
            )?;
            simCode = TDS_updateModelInfo(simCode.clone(), idcs);
            numDupl = List::fold(
                &(List::map(duplComps.clone(), &fnptr!(listLength, _))?),
                &fnptr!(intAdd, i32, i32),
                0,
            )?;
            procAss = Array::map(
                procAss.clone(),
                &fnptr!(metamodelica::listReverse, metamodelica::List<i32>),
            )?;
            sccSimEqMap = metamodelica::arrayAppend(
                iSccSimEqMapping.clone(),
                metamodelica::arrayFromVec(duplSccSimEqMap.clone().reverse().into_iter().cloned().collect()),
            );
            comps = metamodelica::arrayAppend(
                inComps.clone(),
                metamodelica::arrayFromVec(duplComps.clone().reverse().into_iter().cloned().collect()),
            );
            varCompMapping = metamodelica::arrayAppend(varCompMapping.clone(), arrayCreate(numDupl, (0, 0, 0)));
            eqCompMapping = metamodelica::arrayAppend(eqCompMapping.clone(), arrayCreate(numDupl, (0, 0, 0)));
            compParamMapping =
                metamodelica::arrayAppend(compParamMapping.clone(), arrayCreate(numDupl, metamodelica::nil()));
            compNames = metamodelica::arrayAppend(compNames.clone(), arrayCreate(numDupl, literal!("duplicated")));
            compDescs = metamodelica::arrayAppend(compDescs.clone(), arrayCreate(numDupl, literal!("duplicated")));
            exeCosts = metamodelica::arrayAppend(
                exeCosts.clone(),
                arrayCreate(numDupl, (1, metamodelica::OrderedFloat(1.0_f64))),
            );
            nodeMark = metamodelica::arrayAppend(nodeMark.clone(), arrayCreate(numDupl, -1));
            meta = HpcOmTaskGraph::TaskGraphMeta {
                inComps: comps.clone(),
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
            newIdxAss = arrayCreate(SimCodeCodegenUtil::getMaxSimEqSystemIndex(&simCode)?, -1);
            (simCode, newIdxAss) = TDS_assignNewSimEqSysIdxs(simCode.clone(), newIdxAss.clone())?;
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(
                taskGraph.clone(),
                metamodelica::arrayLength(taskGraph.clone()),
            )?;
            schedule = insertLocksInSchedule(
                &schedule,
                taskGraph.clone(),
                taskGraphT.clone(),
                taskAss.clone(),
                procAss.clone(),
                iCommCosts.clone(),
                iCompTaskMapping.clone(),
                iSimVarMapping.clone(),
            )?;
            schedule = TDS_replaceSimEqSysIdxsInSchedule(&schedule, newIdxAss.clone())?;
            Ok((
                schedule.clone(),
                simCode.clone(),
                taskGraph.clone(),
                meta.clone(),
                sccSimEqMap.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("TDS_schedule1 failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn TDS_replaceSimEqSysIdxsInSchedule(
    mut scheduleIn: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut assIn: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut scheduleOut: metamodelica::Ref<HpcOmSimCode::Schedule>;
    scheduleOut = (match &**scheduleIn {
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks,
            outgoingDepTasks,
            scheduledTasks,
            allCalcTasks,
        } => {
            let mut threadTasks = (*threadTasks).clone();
            let mut scheduledTasks = (*scheduledTasks).clone();
            scheduledTasks = List::map1(
                scheduledTasks.clone(),
                &fnptr!(
                    TDS_replaceSimEqSysIdxsInTask,
                    metamodelica::Ref<HpcOmSimCode::Task>,
                    metamodelica::Array<i32>
                ),
                assIn.clone(),
            )?;
            threadTasks = Array::map1(threadTasks.clone(), &TDS_replaceSimEqSysIdxsInTaskLst, assIn.clone())?;
            metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTasks.clone(),
                outgoingDepTasks: outgoingDepTasks.clone(),
                scheduledTasks: scheduledTasks.clone(),
                allCalcTasks: allCalcTasks.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(scheduleOut)
}

fn TDS_replaceSimEqSysIdxsInTask(
    mut taskIn: metamodelica::Ref<HpcOmSimCode::Task>,
    mut assIn: metamodelica::Array<i32>,
) -> metamodelica::Ref<HpcOmSimCode::Task> {
    let mut taskOut: metamodelica::Ref<HpcOmSimCode::Task>;
    taskOut = 'mc: {
        let __mc_input = &*taskIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, timeFinished, threadIdx, eqIdc } => {
                    let mut eqIdc = (*eqIdc).clone();
                    eqIdc = List::map1(eqIdc.clone(), &Array::getIndexFirst, assIn.clone())?;
                    Ok(metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    taskOut
}

fn TDS_replaceSimEqSysIdxsInTaskLst(
    mut taskLstIn: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut assIn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut taskLstOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    taskLstOut = List::map1(
        taskLstIn,
        &fnptr!(
            TDS_replaceSimEqSysIdxsInTask,
            metamodelica::Ref<HpcOmSimCode::Task>,
            metamodelica::Array<i32>
        ),
        assIn.clone(),
    )?;
    Ok(taskLstOut)
}

fn TDS_assignNewSimEqSysIdxs(
    mut simCodeIn: metamodelica::Ref<SimCode::SimCode>,
    mut idxAssIn: metamodelica::Array<i32>,
) -> Result<(metamodelica::Ref<SimCode::SimCode>, metamodelica::Array<i32>)> {
    let mut simCodeOut: metamodelica::Ref<SimCode::SimCode> = simCodeIn;
    let mut idxAssOut: metamodelica::Array<i32>;
    let mut modelInfo: SimCode::ModelInfo;
    let mut varInfo: SimCode::VarInfo;
    let mut jacObts: metamodelica::List<Option<metamodelica::Ref<SimCode::JacobianMatrix>>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut idx: i32;
    let mut ass: metamodelica::Array<i32>;
    modelInfo = simCodeOut.modelInfo.clone();
    varInfo = modelInfo.varInfo.clone();
    let (__pa0, (__pa1, __pa2)) = List::mapFold(
        &simCodeOut.initialEquations,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (1, idxAssIn.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa0);
    idx = metamodelica::Own::own(__pa1);
    ass = metamodelica::Own::own(__pa2);
    assign_field!(simCodeOut.initialEquations = eqs);
    let (__pa3, (__pa4, __pa5)) = List::mapFold(
        &simCodeOut.allEquations,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (idx, ass.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa3);
    idx = metamodelica::Own::own(__pa4);
    ass = metamodelica::Own::own(__pa5);
    assign_field!(simCodeOut.allEquations = eqs);
    let (__pa6, (__pa7, __pa8)) = List::mapFold(
        &simCodeOut.startValueEquations,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (idx, ass.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa6);
    idx = metamodelica::Own::own(__pa7);
    ass = metamodelica::Own::own(__pa8);
    assign_field!(simCodeOut.startValueEquations = eqs);
    let (__pa9, (__pa10, __pa11)) = List::mapFold(
        &simCodeOut.nominalValueEquations,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (idx, ass.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa9);
    idx = metamodelica::Own::own(__pa10);
    ass = metamodelica::Own::own(__pa11);
    assign_field!(simCodeOut.nominalValueEquations = eqs);
    let (__pa12, (__pa13, __pa14)) = List::mapFold(
        &simCodeOut.minValueEquations,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (idx, ass.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa12);
    idx = metamodelica::Own::own(__pa13);
    ass = metamodelica::Own::own(__pa14);
    assign_field!(simCodeOut.minValueEquations = eqs);
    let (__pa15, (__pa16, __pa17)) = List::mapFold(
        &simCodeOut.maxValueEquations,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (idx, ass.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa15);
    idx = metamodelica::Own::own(__pa16);
    ass = metamodelica::Own::own(__pa17);
    assign_field!(simCodeOut.maxValueEquations = eqs);
    let (__pa18, (__pa19, __pa20)) = List::mapFold(
        &simCodeOut.parameterEquations,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (idx, ass.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa18);
    idx = metamodelica::Own::own(__pa19);
    ass = metamodelica::Own::own(__pa20);
    assign_field!(simCodeOut.parameterEquations = eqs);
    let (__pa21, (__pa22, __pa23)) = List::mapFold(
        &simCodeOut.algorithmAndEquationAsserts,
        &TDS_replaceSimEqSysIndexWithUpdate,
        (idx, ass.clone()),
    )?;
    eqs = metamodelica::Own::own(__pa21);
    idx = metamodelica::Own::own(__pa22);
    ass = metamodelica::Own::own(__pa23);
    assign_field!(
        simCodeOut.algorithmAndEquationAsserts = eqs,
        simCodeOut.odeEquations =
            List::map1List(simCodeOut.odeEquations.clone(), &TDS_replaceSimEqSysIndex, ass.clone())?,
        simCodeOut.algebraicEquations = List::map1List(
            simCodeOut.algebraicEquations.clone(),
            &TDS_replaceSimEqSysIndex,
            ass.clone()
        )?,
        simCodeOut.equationsForZeroCrossings = List::map1(
            simCodeOut.equationsForZeroCrossings.clone(),
            &TDS_replaceSimEqSysIndex,
            ass.clone()
        )?
    );
    jacObts = List::map(simCodeOut.jacobianMatrices.clone(), &fnptr!(Util::makeOption, _))?;
    jacObts = List::map1(
        jacObts,
        &fnptr!(
            TDS_replaceSimEqSysIdxInJacobianMatrix,
            Option<metamodelica::Ref<SimCode::JacobianMatrix>>,
            metamodelica::Array<i32>
        ),
        ass.clone(),
    )?;
    assign_field!(simCodeOut.jacobianMatrices = List::map(jacObts, &Util::getOption)?);
    varInfo.numEquations = idx;
    modelInfo.varInfo = varInfo;
    assign_field!(simCodeOut.modelInfo = modelInfo);
    idxAssOut = ass.clone();
    Ok((simCodeOut, idxAssOut))
}

fn TDS_replaceSimEqSysIndex(
    mut simEqIn: metamodelica::Ref<SimCode::SimEqSystem>,
    mut assIn: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut simEqOut: metamodelica::Ref<SimCode::SimEqSystem>;
    simEqOut = 'mc: {
        let __mc_input = simEqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { eqs, jacobianMatrix, .. }, .. } => {
                    let mut newIdx: i32;
                    let mut oldIdx: i32;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    eqs = List::map1(eqs.clone(), &TDS_replaceSimEqSysIndex, assIn.clone())?;
                    oldIdx = SimCodeCodegenUtil::simEqSystemIndex(&simEqIn)?;
                    newIdx = metamodelica::arrayGet(assIn.clone(), oldIdx)?;
                    jacobianMatrix = TDS_replaceSimEqSysIdxInJacobianMatrix(jacobianMatrix.clone(), assIn.clone());
                    assign_field!(
                        nlSystem.jacobianMatrix = jacobianMatrix.clone(),
                        nlSystem.index = newIdx,
                        nlSystem.eqs = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
                    Ok(simEqSys.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { residual: eqs, jacobianMatrix, .. }, .. } => {
                    let mut newIdx: i32;
                    let mut oldIdx: i32;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    eqs = List::map1(eqs.clone(), &TDS_replaceSimEqSysIndex, assIn.clone())?;
                    oldIdx = SimCodeCodegenUtil::simEqSystemIndex(&simEqIn)?;
                    newIdx = metamodelica::arrayGet(assIn.clone(), oldIdx)?;
                    jacobianMatrix = TDS_replaceSimEqSysIdxInJacobianMatrix(jacobianMatrix.clone(), assIn.clone());
                    assign_field!(
                        lSystem.jacobianMatrix = jacobianMatrix.clone(),
                        lSystem.index = newIdx,
                        lSystem.residual = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    Ok(simEqSys.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut newIdx: i32;
                    let mut oldIdx: i32;
                    let mut simEqSys: metamodelica::Ref<SimCode::SimEqSystem>;
                    oldIdx = SimCodeCodegenUtil::simEqSystemIndex(&simEqIn)?;
                    newIdx = metamodelica::arrayGet(assIn.clone(), oldIdx)?;
                    simEqSys = SimCodeUtil::replaceSimEqSysIndex(simEqIn.clone(), newIdx)?;
                    Ok(simEqSys.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(simEqOut)
}

fn TDS_replaceSimEqSysIndexWithUpdate(
    mut simEqIn: metamodelica::Ref<SimCode::SimEqSystem>,
    mut tplIn: (i32, metamodelica::Array<i32>),
) -> Result<(metamodelica::Ref<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> {
    let mut simEqOut: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (simEqOut, tplOut) = 'mc: {
        let __mc_input = (simEqIn.clone(), tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { index: oldIdx, eqs, jacobianMatrix, .. }, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(metamodelica::AsArg::as_arg(&eqs), &TDS_replaceSimEqSysIndexWithUpdate, (newIdx.clone(), ass.clone()))?;
                    eqs = metamodelica::Own::own(__pa0);
                    newIdx = metamodelica::Own::own(__pa1);
                    ass = metamodelica::Own::own(__pa2);
                    let (__pa3, (__pa4, __pa5)) = TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(jacobianMatrix.clone(), (newIdx.clone(), ass.clone()));
                    jacobianMatrix = metamodelica::Own::own(__pa3);
                    newIdx = metamodelica::Own::own(__pa4);
                    ass = metamodelica::Own::own(__pa5);
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx.clone(), newIdx.clone())?;
                    assign_field!(
                        nlSystem.jacobianMatrix = jacobianMatrix.clone(),
                        nlSystem.index = newIdx.clone(),
                        nlSystem.eqs = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { index: oldIdx, residual: eqs, jacobianMatrix, .. }, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(metamodelica::AsArg::as_arg(&eqs), &TDS_replaceSimEqSysIndexWithUpdate, (newIdx.clone(), ass.clone()))?;
                    eqs = metamodelica::Own::own(__pa0);
                    newIdx = metamodelica::Own::own(__pa1);
                    ass = metamodelica::Own::own(__pa2);
                    let (__pa3, (__pa4, __pa5)) = TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(jacobianMatrix.clone(), (newIdx.clone(), ass.clone()));
                    jacobianMatrix = metamodelica::Own::own(__pa3);
                    newIdx = metamodelica::Own::own(__pa4);
                    ass = metamodelica::Own::own(__pa5);
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx.clone(), newIdx.clone())?;
                    assign_field!(
                        lSystem.jacobianMatrix = jacobianMatrix.clone(),
                        lSystem.index = newIdx.clone(),
                        lSystem.residual = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { index: oldIdx, cont, discEqs: eqs, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cont = (*cont).clone();
                    let mut eqs = (*eqs).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = TDS_replaceSimEqSysIndexWithUpdate(cont.clone(), (newIdx.clone(), ass.clone()))?;
                    cont = metamodelica::Own::own(__pa0);
                    newIdx = metamodelica::Own::own(__pa1);
                    ass = metamodelica::Own::own(__pa2);
                    let (__pa3, (__pa4, __pa5)) = List::mapFold(metamodelica::AsArg::as_arg(&eqs), &TDS_replaceSimEqSysIndexWithUpdate, (newIdx.clone(), ass.clone()))?;
                    eqs = metamodelica::Own::own(__pa3);
                    newIdx = metamodelica::Own::own(__pa4);
                    ass = metamodelica::Own::own(__pa5);
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx.clone(), newIdx.clone())?;
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_MIXED;
                        cont = cont.clone(),
                        discEqs = eqs.clone()
                    );
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (newIdx, ass)) => {
                    let mut oldIdx: i32;
                    let mut simEqSys: metamodelica::Ref<SimCode::SimEqSystem>;
                    let mut ass = (*ass).clone();
                    oldIdx = SimCodeCodegenUtil::simEqSystemIndex(&simEqIn)?;
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx, newIdx.clone())?;
                    simEqSys = SimCodeUtil::replaceSimEqSysIndex(simEqIn.clone(), newIdx.clone())?;
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((simEqOut, tplOut))
}

fn TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(
    mut jacIn: Option<metamodelica::Ref<SimCode::JacobianMatrix>>,
    mut tplIn: (i32, metamodelica::Array<i32>),
) -> (
    Option<metamodelica::Ref<SimCode::JacobianMatrix>>,
    (i32, metamodelica::Array<i32>),
) {
    let mut jacOut: Option<metamodelica::Ref<SimCode::JacobianMatrix>>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (jacOut, tplOut) = 'mc: {
        let __mc_input = (&jacIn, &tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ SimCode::JacobianMatrix { columns: jacCols, seedVars: vars, matrixName: name, sparsityMatrix, sparsity, sparsityT, nonlinear: nonlinearPat, nonlinearT: nonlinearPatT, coloredCols: colCols, coloredRows: colRows, maxColorCols: maxCol, jacobianIndex: jacIdx, partitionIndex: partIdx, generic_loop_calls: Deref @ metamodelica::ListNode::Nil, crefsHT: crefToSimVarHTJacobian, isAdjoint: isAdj, isBidirectional: false, adjointJacobianIndex: (-1), adjointMatrixName: Deref @ "" }), (newIdx, ass)) => {
                    let mut jacCols = (*jacCols).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(metamodelica::AsArg::as_arg(&jacCols), &fnptr!(TDS_replaceSimEqSysIdxInJacobianColumnWithUpdate, metamodelica::Ref<SimCode::JacobianColumn>, (i32, metamodelica::Array<i32>)), (newIdx.clone(), ass.clone()))?;
                    jacCols = metamodelica::Own::own(__pa0);
                    newIdx = metamodelica::Own::own(__pa1);
                    ass = metamodelica::Own::own(__pa2);
                    Ok((Some(metamodelica::Ref::new(SimCode::JacobianMatrix { columns: jacCols.clone(), seedVars: vars.clone(), matrixName: name.clone(), sparsityMatrix: sparsityMatrix.clone(), sparsity: sparsity.clone(), sparsityT: sparsityT.clone(), nonlinear: nonlinearPat.clone(), nonlinearT: nonlinearPatT.clone(), coloredCols: colCols.clone(), coloredRows: colRows.clone(), maxColorCols: maxCol.clone(), jacobianIndex: jacIdx.clone(), partitionIndex: partIdx.clone(), generic_loop_calls: metamodelica::nil(), crefsHT: crefToSimVarHTJacobian.clone(), isAdjoint: isAdj.clone(), isBidirectional: false, adjointJacobianIndex: -1, adjointMatrixName: literal!("") })), (newIdx.clone(), ass.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((jacIn.clone(), tplIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (jacOut, tplOut)
}

fn TDS_replaceSimEqSysIdxInJacobianColumnWithUpdate(
    mut jacIn: metamodelica::Ref<SimCode::JacobianColumn>,
    mut tplIn: (i32, metamodelica::Array<i32>),
) -> (
    metamodelica::Ref<SimCode::JacobianColumn>,
    (i32, metamodelica::Array<i32>),
) {
    let mut jacOut: metamodelica::Ref<SimCode::JacobianColumn>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (jacOut, tplOut) = 'mc: {
        let __mc_input = (&*jacIn, &tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SimCode::JacobianColumn { columnEqns: simEqs, columnVars: simVars, numberOfResultVars: rowLen, constantEqns: constEqns }, (newIdx, ass)) => {
                    let mut simEqs = (*simEqs).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(metamodelica::AsArg::as_arg(&simEqs), &TDS_replaceSimEqSysIndexWithUpdate, (newIdx.clone(), ass.clone()))?;
                    simEqs = metamodelica::Own::own(__pa0);
                    newIdx = metamodelica::Own::own(__pa1);
                    ass = metamodelica::Own::own(__pa2);
                    Ok((metamodelica::Ref::new(SimCode::JacobianColumn { columnEqns: simEqs.clone(), columnVars: simVars.clone(), numberOfResultVars: rowLen.clone(), constantEqns: constEqns.clone() }), (newIdx.clone(), ass.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((jacIn.clone(), tplIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (jacOut, tplOut)
}

fn TDS_replaceSimEqSysIdxInJacobianMatrix(
    mut jacIn: Option<metamodelica::Ref<SimCode::JacobianMatrix>>,
    mut assIn: metamodelica::Array<i32>,
) -> Option<metamodelica::Ref<SimCode::JacobianMatrix>> {
    let mut jacOut: Option<metamodelica::Ref<SimCode::JacobianMatrix>> = jacIn.clone();
    jacOut = 'mc: {
        let __mc_input = &jacIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(jacMatrix @ Deref @ SimCode::JacobianMatrix { .. }) => {
                    let mut jacMatrix = (*jacMatrix).clone();
                    assign_field!(jacMatrix.columns = List::map1(jacMatrix.columns.clone(), &TDS_replaceSimEqSysIdxInJacobianColumn, assIn.clone())?);
                    Ok(Some(jacMatrix.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(jacIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    jacOut
}

fn TDS_replaceSimEqSysIdxInJacobianColumn(
    mut jacIn: metamodelica::Ref<SimCode::JacobianColumn>,
    mut assIn: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<SimCode::JacobianColumn>> {
    let mut jacOut: metamodelica::Ref<SimCode::JacobianColumn> = jacIn;
    assign_field!(jacOut.columnEqns = List::map1(jacOut.columnEqns.clone(), &TDS_replaceSimEqSysIndex, assIn.clone())?);
    Ok(jacOut)
}

fn TDS_updateModelInfo(
    mut simCodeIn: metamodelica::Ref<SimCode::SimCode>,
    mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32),
) -> metamodelica::Ref<SimCode::SimCode> {
    let mut simCodeOut: metamodelica::Ref<SimCode::SimCode> = simCodeIn.clone();
    let mut lsIdx: i32;
    let mut nlsIdx: i32;
    let mut mIdx: i32;
    let mut modelInfo: SimCode::ModelInfo;
    let mut varInfo: SimCode::VarInfo;
    (_, _, _, _, _, lsIdx, nlsIdx, mIdx) = idcs;
    modelInfo = simCodeIn.modelInfo.clone();
    varInfo = modelInfo.varInfo.clone();
    varInfo.numStateVars = ((modelInfo.vars.stateVars).len() as i32);
    varInfo.numAlgVars = ((modelInfo.vars.algVars).len() as i32);
    varInfo.numLinearSystems = if (intEq(varInfo.numLinearSystems.clone(), 0)) {
        0
    } else {
        lsIdx
    };
    varInfo.numNonLinearSystems = if (intEq(varInfo.numNonLinearSystems.clone(), 0)) {
        0
    } else {
        nlsIdx
    };
    modelInfo.varInfo = varInfo;
    assign_field!(simCodeOut.modelInfo = modelInfo);
    simCodeOut
}

fn TDS_duplicateTasks(
    mut clustersIn: &metamodelica::List<metamodelica::List<i32>>,
    mut taskAssIn: metamodelica::Array<i32>,
    mut procAssIn: metamodelica::Array<metamodelica::List<i32>>,
    mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32),
    mut taskGraphOrig: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphTOrig: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskDuplAssIn: metamodelica::Array<i32>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut simCodeIn: &metamodelica::Ref<SimCode::SimCode>,
    mut scheduleIn: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut sccSimEqMappingIn: metamodelica::Array<metamodelica::List<i32>>,
    mut duplSccSimEqMapIn: &metamodelica::List<metamodelica::List<i32>>,
    mut duplCompsIn: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    (i32, i32, i32, i32, i32, i32, i32, i32),
    metamodelica::Ref<SimCode::SimCode>,
    metamodelica::Ref<HpcOmSimCode::Schedule>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskGraphOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: metamodelica::Ref<SimCode::SimCode>;
    let mut scheduleOut: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut duplSccSimEqMapOut: metamodelica::List<metamodelica::List<i32>>;
    let mut duplCompsOut: metamodelica::List<metamodelica::List<i32>>;
    (
        taskAssOut,
        procAssOut,
        taskGraphOut,
        taskDuplAssOut,
        idcsOut,
        simCodeOut,
        scheduleOut,
        duplSccSimEqMapOut,
        duplCompsOut,
    ) = (::match_deref::match_deref! { match clustersIn {
        Deref @ metamodelica::ListNode::Nil => {
            (taskAssIn.clone(), procAssIn.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), idcsIn, simCodeIn.clone(), scheduleIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: cluster, tail: rest } => {
            let mut threadIdx: i32;
            let mut compIdx: i32;
            let mut simVarIdx: i32;
            let mut simEqSysIdx: i32;
            let mut taskIdx: i32;
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut duplSccSimEqMap: metamodelica::List<metamodelica::List<i32>>;
            let mut duplComps: metamodelica::List<metamodelica::List<i32>>;
            let mut taskAss: metamodelica::Array<i32>;
            let mut taskDuplAss: metamodelica::Array<i32>;
            let mut procAss: metamodelica::Array<metamodelica::List<i32>>;
            let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut simCode: metamodelica::Ref<SimCode::SimCode>;
            let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
            let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
            let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
            let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
            let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
            repl = BackendVarTransform::emptyReplacements();
            let (__pa0, __pa1, __pa2, __pa3, __pa4, (__pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12), __pa13, __pa14, __pa15) = TDS_duplicateTasks1(metamodelica::AsArg::as_arg(&cluster), clustersIn, &repl, taskAssIn.clone(), procAssIn.clone(), &(metamodelica::nil()), idcsIn, taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta, simCodeIn, sccSimEqMappingIn.clone(), duplSccSimEqMapIn, duplCompsIn)?;
            taskAss = metamodelica::Own::own(__pa0);
            procAss = metamodelica::Own::own(__pa1);
            taskGraph = metamodelica::Own::own(__pa2);
            taskDuplAss = metamodelica::Own::own(__pa3);
            thread = metamodelica::Own::own(__pa4);
            threadIdx = metamodelica::Own::own(__pa5);
            taskIdx = metamodelica::Own::own(__pa6);
            compIdx = metamodelica::Own::own(__pa7);
            simVarIdx = metamodelica::Own::own(__pa8);
            simEqSysIdx = metamodelica::Own::own(__pa9);
            lsIdx = metamodelica::Own::own(__pa10);
            nlsIdx = metamodelica::Own::own(__pa11);
            mIdx = metamodelica::Own::own(__pa12);
            simCode = metamodelica::Own::own(__pa13);
            duplSccSimEqMap = metamodelica::Own::own(__pa14);
            duplComps = metamodelica::Own::own(__pa15);
            let __arc16 = simCode.clone();
            let SimCode::SIMCODE { .. } = &*__arc16;
            let (__pa17, __pa18, __pa19) = ::match_deref::match_deref! { match &((*scheduleIn)) {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa17, outgoingDepTasks: __pa18, allCalcTasks: __pa19, .. } => (__pa17.clone(), __pa18.clone(), __pa19.clone()),
                _ => return Err("pattern mismatch"),
            } };
            threadTasks = metamodelica::Own::own(__pa17);
            outgoingDepTasks = metamodelica::Own::own(__pa18);
            allCalcTasks = metamodelica::Own::own(__pa19);
            threadTasks = metamodelica::arrayUpdate(threadTasks.clone(), threadIdx, thread.reverse())?;
            schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks, scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            threadIdx = threadIdx + 1;
            (taskAss, procAss, taskGraph, taskDuplAss, idcs, simCode, schedule, duplSccSimEqMap, duplComps) = TDS_duplicateTasks(rest, taskAss.clone(), procAss.clone(), (threadIdx, taskIdx, compIdx, simVarIdx, simEqSysIdx, lsIdx, nlsIdx, mIdx), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta, &simCode, &schedule, sccSimEqMappingIn.clone(), &duplSccSimEqMap, &duplComps)?;
            (taskAssIn.clone(), procAssIn.clone(), taskGraph.clone(), taskDuplAss.clone(), idcs, simCode, schedule, duplSccSimEqMap, duplComps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((
        taskAssOut,
        procAssOut,
        taskGraphOut,
        taskDuplAssOut,
        idcsOut,
        simCodeOut,
        scheduleOut,
        duplSccSimEqMapOut,
        duplCompsOut,
    ))
}

fn TDS_duplicateTasks1(
    mut clusterIn: &metamodelica::List<i32>,
    mut allCluster: &metamodelica::List<metamodelica::List<i32>>,
    mut replIn: &BackendVarTransform::VariableReplacements,
    mut taskAssIn: metamodelica::Array<i32>,
    mut procAssIn: metamodelica::Array<metamodelica::List<i32>>,
    mut threadIn: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32),
    mut taskGraphOrig: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphTOrig: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskDuplAssIn: metamodelica::Array<i32>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut simCodeIn: &metamodelica::Ref<SimCode::SimCode>,
    mut sccSimEqMappingIn: metamodelica::Array<metamodelica::List<i32>>,
    mut duplSccSimEqMapIn: &metamodelica::List<metamodelica::List<i32>>,
    mut duplCompsIn: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    (i32, i32, i32, i32, i32, i32, i32, i32),
    metamodelica::Ref<SimCode::SimCode>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskGraphOut: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut threadOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: metamodelica::Ref<SimCode::SimCode>;
    let mut duplSccSimEqMapOut: metamodelica::List<metamodelica::List<i32>>;
    let mut duplCompsOut: metamodelica::List<metamodelica::List<i32>>;
    (
        taskAssOut,
        procAssOut,
        taskGraphOut,
        taskDuplAssOut,
        threadOut,
        idcsOut,
        simCodeOut,
        duplSccSimEqMapOut,
        duplCompsOut,
    ) = 'mc: {
        let __mc_input = &**clusterIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((taskAssIn.clone(), procAssIn.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), threadIn.clone(), idcsIn, simCodeIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
                    let mut ass: i32;
                    let mut duplSccSimEqMap: metamodelica::List<metamodelica::List<i32>>;
                    let mut duplComps: metamodelica::List<metamodelica::List<i32>>;
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut taskDuplAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<metamodelica::List<i32>>;
                    let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
                    let mut simCode: metamodelica::Ref<SimCode::SimCode>;
                    let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    ass = metamodelica::arrayGet(taskAssIn.clone(), node.clone())?;
                    let true = (intNe(ass, -1)) else { return Err("pattern mismatch") };
                    (repl, taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks2(node.clone(), allCluster, replIn.clone(), taskAssIn.clone(), procAssIn.clone(), threadIn.clone(), idcsIn, taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta.clone(), simCodeIn.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())?;
                    (taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks1(metamodelica::AsArg::as_arg(&rest), allCluster, &repl, taskAss.clone(), procAss.clone(), &thread, idcs, taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta, &simCode, sccSimEqMappingIn.clone(), &duplSccSimEqMap, &duplComps)?;
                    Ok((taskAss.clone(), procAss.clone(), taskGraph.clone(), taskDuplAss.clone(), thread.clone(), idcs, simCode.clone(), duplSccSimEqMap.clone(), duplComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
                    let mut ass: i32;
                    let mut threadIdx: i32;
                    let mut comps: metamodelica::List<i32>;
                    let mut simEqs: metamodelica::List<i32>;
                    let mut taskLst: metamodelica::List<i32>;
                    let mut origPredTasks: metamodelica::List<i32>;
                    let mut clPredTasks: metamodelica::List<i32>;
                    let mut duplPredTasks: metamodelica::List<i32>;
                    let mut clTasks: metamodelica::List<i32>;
                    let mut pos: metamodelica::List<i32>;
                    let mut duplSccSimEqMap: metamodelica::List<metamodelica::List<i32>>;
                    let mut duplComps: metamodelica::List<metamodelica::List<i32>>;
                    let mut simEqsLst: metamodelica::List<metamodelica::List<i32>>;
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut taskDuplAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<metamodelica::List<i32>>;
                    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
                    let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
                    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
                    let mut simCode: metamodelica::Ref<SimCode::SimCode>;
                    let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut odes: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
                    let mut simEqSysts: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut allEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut taskGraphOut: metamodelica::Array<metamodelica::List<i32>> = taskGraphOut.clone();
                    ass = metamodelica::arrayGet(taskAssIn.clone(), node.clone())?;
                    let true = (intEq(ass, -1)) else { return Err("pattern mismatch") };
                    (threadIdx, _, _, _, _, _, _, _) = idcsIn;
                    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = &iTaskGraphMeta;
                    inComps = metamodelica::Own::own(__pa0);
                    taskAss = metamodelica::arrayUpdate(taskAssIn.clone(), node.clone(), threadIdx)?;
                    taskLst = metamodelica::arrayGet(procAssIn.clone(), threadIdx)?;
                    procAss = metamodelica::arrayUpdate(procAssIn.clone(), threadIdx, metamodelica::cons(node.clone(), taskLst.clone()))?;
                    comps = metamodelica::arrayGet(inComps.clone(), node.clone())?;
                    simEqsLst = List::map1(comps.clone(), &Array::getIndexFirst, sccSimEqMappingIn.clone())?;
                    simEqs = List::flatten(simEqsLst.clone())?;
                    simEqs = simEqs.clone().reverse();
                    let __arc3 = &(*simCodeIn);
                    let SimCode::SIMCODE { odeEquations: __pa1, allEquations: __pa2, .. } = &**__arc3;
                    odes = metamodelica::Own::own(__pa1);
                    allEqs = metamodelica::Own::own(__pa2);
                    simEqSysts = List::map1(simEqs.clone(), &move |__a0: i32, __a1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>| SimCodeCodegenUtil::getSimEqSysForIndex(__a0, &__a1), List::flatten(odes.clone())?)?;
                    (simEqSysts, _) = replaceInSimEqSystemLst(&simEqSysts, replIn.clone())?;
                    allEqs = replaceSimEqSystemLstWithSameIndex(&simEqSysts, allEqs.clone())?;
                    odes = List::map1r(odes.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>, __a1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>| replaceSimEqSystemLstWithSameIndex(&__a0, __a1), simEqSysts.clone())?;
                    simCode = SimCodeUtil::replaceODEandALLequations(allEqs.clone(), odes.clone(), simCodeIn.clone());
                    clTasks = (allCluster).head().cloned()?;
                    origPredTasks = metamodelica::arrayGet(taskGraphTOrig.clone(), node.clone())?;
                    (clPredTasks, origPredTasks, _) = List::intersection1OnTrue(origPredTasks.clone(), clTasks.clone(), &fnptr!(intEq, i32, i32))?;
                    pos = List::map1(clPredTasks.clone(), &move |__a0: _, __a1: _| List::position(__a0, &__a1), clTasks.clone())?;
                    clTasks = metamodelica::arrayGet(procAssIn.clone(), threadIdx)?;
                    clTasks = clTasks.clone().reverse();
                    clPredTasks = List::map1(pos.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), clTasks.clone())?;
                    (duplPredTasks, _, _) = List::intersection1OnTrue(clPredTasks.clone(), clTasks.clone(), &fnptr!(intEq, i32, i32))?;
                    taskGraph = List::fold1(&duplPredTasks, &Array::appendToElement, list![node.clone()], taskGraphIn.clone())?;
                    taskGraphOut = List::fold1(&origPredTasks, &Array::appendToElement, list![node.clone()], taskGraph.clone())?;
                    task = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: 1, index: node.clone(), calcTime: metamodelica::OrderedFloat(0.0_f64), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: threadIdx, eqIdc: simEqs.clone() });
                    thread = metamodelica::cons(task.clone(), threadIn.clone());
                    taskDuplAss = metamodelica::arrayUpdate(taskDuplAssIn.clone(), node.clone(), node.clone())?;
                    (taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks1(metamodelica::AsArg::as_arg(&rest), allCluster, replIn, taskAss.clone(), procAss.clone(), &thread, idcsIn, taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta, &simCode, sccSimEqMappingIn.clone(), duplSccSimEqMapIn, duplCompsIn)?;
                    Ok(((taskAss.clone(), procAss.clone(), taskGraph.clone(), taskDuplAss.clone(), thread.clone(), idcs, simCode.clone(), duplSccSimEqMap.clone(), duplComps.clone()), taskGraphOut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            taskGraphOut = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        taskAssOut,
        procAssOut,
        taskGraphOut,
        taskDuplAssOut,
        threadOut,
        idcsOut,
        simCodeOut,
        duplSccSimEqMapOut,
        duplCompsOut,
    ))
}

fn TDS_duplicateTasks2(
    mut node: i32,
    mut allCluster: &metamodelica::List<metamodelica::List<i32>>,
    mut replIn: BackendVarTransform::VariableReplacements,
    mut taskAssIn: metamodelica::Array<i32>,
    mut procAssIn: metamodelica::Array<metamodelica::List<i32>>,
    mut threadIn: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32),
    mut taskGraphOrig: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphTOrig: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskDuplAssIn: metamodelica::Array<i32>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut simCodeIn: metamodelica::Ref<SimCode::SimCode>,
    mut sccSimEqMappingIn: metamodelica::Array<metamodelica::List<i32>>,
    mut duplSccSimEqMapIn: metamodelica::List<metamodelica::List<i32>>,
    mut duplCompsIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    BackendVarTransform::VariableReplacements,
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    (i32, i32, i32, i32, i32, i32, i32, i32),
    metamodelica::Ref<SimCode::SimCode>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskGraphOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut threadOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: metamodelica::Ref<SimCode::SimCode>;
    let mut duplSccSimEqMapOut: metamodelica::List<metamodelica::List<i32>>;
    let mut duplCompsOut: metamodelica::List<metamodelica::List<i32>>;
    let mut crefAppend: ArcStr;
    let mut threadIdx: i32;
    let mut compIdx: i32;
    let mut simVarIdx: i32;
    let mut simVarIdx2: i32;
    let mut simEqSysIdx: i32;
    let mut simEqSysIdx2: i32;
    let mut simEqSysIdx3: i32;
    let mut numVars: i32;
    let mut numEqs: i32;
    let mut numInitEqs: i32;
    let mut taskIdx: i32;
    let mut lsIdx: i32;
    let mut nlsIdx: i32;
    let mut mIdx: i32;
    let mut comps: metamodelica::List<i32>;
    let mut simVarSysIdcs2: metamodelica::List<i32>;
    let mut simEqSysIdcs: metamodelica::List<i32>;
    let mut simEqSysIdcs2: metamodelica::List<i32>;
    let mut simEqSysIdcsInit: metamodelica::List<i32>;
    let mut thread: metamodelica::List<i32>;
    let mut clTasks: metamodelica::List<i32>;
    let mut origPredTasks: metamodelica::List<i32>;
    let mut clPredTasks: metamodelica::List<i32>;
    let mut duplPredTasks: metamodelica::List<i32>;
    let mut pos: metamodelica::List<i32>;
    let mut simEqIdxLst: metamodelica::List<metamodelica::List<i32>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrefSimVar::FuncHashCref,
            HashTableCrefSimVar::FuncCrefEqual,
            HashTableCrefSimVar::FuncCrefStr,
            HashTableCrefSimVar::FuncExpStr,
        ),
    );
    let mut simVars: SimCodeVar::SimVars;
    let mut simCode: metamodelica::Ref<SimCode::SimCode>;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefsDupl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut crefsDuplExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut simVarLst: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut simVarDupl: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut simEqSysts: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut simEqSystsDupl: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut initEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut odes: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    let __arc4 = simCodeIn.clone();
    let SimCode::SIMCODE {
        modelInfo: SimCode::MODELINFO { vars: __pa1, .. },
        odeEquations: __pa2,
        crefToSimVarHT: __pa3,
        ..
    } = &*__arc4;
    simVars = metamodelica::Own::own(__pa1);
    odes = metamodelica::Own::own(__pa2);
    ht = metamodelica::Own::own(__pa3);
    (threadIdx, taskIdx, compIdx, simVarIdx, simEqSysIdx, lsIdx, nlsIdx, mIdx) = idcsIn;
    comps = metamodelica::arrayGet(inComps.clone(), node)?;
    comps = comps.reverse();
    simEqIdxLst = List::map1(comps.clone(), &Array::getIndexFirst, sccSimEqMappingIn.clone())?;
    simEqSysIdcs = List::flatten(simEqIdxLst)?;
    crefLst = List::map1(
        simEqSysIdcs.clone(),
        &move |__a0: i32, __a1: metamodelica::Ref<SimCode::SimCode>| SimCodeUtil::getAssignedCrefsOfSimEq(__a0, &__a1),
        simCodeIn.clone(),
    )?;
    crefs = List::flatten(crefLst)?;
    simVarLst = List::map1(
        crefs.clone(),
        &move |__a0: _, __a1: _| BaseHashTable::get(__a0, &__a1),
        ht.clone(),
    )?;
    numVars = ((simVarLst).len() as i32);
    simVarSysIdcs2 = List::intRange2(simVarIdx, simVarIdx + numVars - 1);
    crefAppend = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("_thr"));
        __mm_s.push_str(&*intString(threadIdx));
        ArcStr::from(__mm_s)
    };
    crefsDupl = List::map1r(
        crefs.clone(),
        &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReference::appendStringLastIdent(&__a0, &__a1)
        },
        crefAppend,
    )?;
    crefsDuplExp = List::map(crefsDupl.clone(), &Expression::crefExp)?;
    simVarDupl = List::threadMap(
        crefsDupl.clone(),
        simVarLst,
        &fnptr!(
            SimCodeUtil::replaceSimVarName,
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<SimCodeVar::SimVar>
        ),
    )?;
    simVarDupl = List::threadMap(
        simVarSysIdcs2,
        simVarDupl,
        &fnptr!(
            SimCodeUtil::replaceSimVarIndex,
            i32,
            metamodelica::Ref<SimCodeVar::SimVar>
        ),
    )?;
    simCode = List::fold(
        &simVarDupl,
        &fnptr!(
            SimCodeUtil::addSimVarToAlgVars,
            metamodelica::Ref<SimCodeVar::SimVar>,
            metamodelica::Ref<SimCode::SimCode>
        ),
        simCodeIn,
    )?;
    simVarIdx2 = simVarIdx + numVars;
    ht = List::fold(&simVarDupl, &HashTableCrefSimVar::addSimVarToHashTable, ht)?;
    repl = BackendVarTransform::addReplacements(replIn, &crefs, &crefsDuplExp, None)?;
    simEqSysts = List::map1(
        simEqSysIdcs,
        &move |__a0: i32, __a1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>| {
            SimCodeCodegenUtil::getSimEqSysForIndex(__a0, &__a1)
        },
        List::flatten(odes)?,
    )?;
    numEqs = ((simEqSysts).len() as i32);
    simEqSysIdcs2 = List::intRange2(simEqSysIdx, simEqSysIdx + numEqs - 1);
    (simEqSystsDupl, _) = List::map1_2(
        &simEqSysts,
        &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: BackendVarTransform::VariableReplacements| {
            replaceExpsInSimEqSystem(__a0, &__a1)
        },
        repl.clone(),
    )?;
    let (__pa5, (__pa6, __pa7, __pa8)) = List::mapFold(
        &simEqSystsDupl,
        &fnptr!(
            replaceSystemIndex,
            metamodelica::Ref<SimCode::SimEqSystem>,
            (i32, i32, i32)
        ),
        (lsIdx, nlsIdx, mIdx),
    )?;
    simEqSystsDupl = metamodelica::Own::own(__pa5);
    lsIdx = metamodelica::Own::own(__pa6);
    nlsIdx = metamodelica::Own::own(__pa7);
    mIdx = metamodelica::Own::own(__pa8);
    simEqSystsDupl = List::threadMap(
        simEqSystsDupl,
        simEqSysIdcs2.clone(),
        &SimCodeUtil::replaceSimEqSysIndex,
    )?;
    simEqSysIdx2 = simEqSysIdx + numEqs;
    (simEqSystsDupl, simEqSysIdx2) =
        TDS_duplicateSystemOfEquations(&simEqSystsDupl, simEqSysIdx2, &repl, &(metamodelica::nil()))?;
    duplSccSimEqMapOut = listAppend(
        List::map(simEqSysIdcs2.clone(), &fnptr!(List::create, _))?,
        duplSccSimEqMapIn,
    );
    simCode = List::fold1(&simEqSystsDupl, &SimCodeUtil::addSimEqSysToODEquations, 1, simCode)?;
    threadOut = metamodelica::cons(
        metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
            weighting: 1,
            index: taskIdx,
            calcTime: metamodelica::OrderedFloat(0.0_f64),
            timeFinished: metamodelica::OrderedFloat(-1.0_f64),
            threadIdx: threadIdx,
            eqIdc: simEqSysIdcs2,
        }),
        threadIn,
    );
    numInitEqs = ((crefs).len() as i32);
    simEqSysIdcsInit = List::intRange2(simEqSysIdx2, simEqSysIdx2 + numInitEqs - 1);
    initEqs = List::thread3Map(crefsDupl, crefs, simEqSysIdcsInit, &makeSEScrefAssignment)?;
    simCode = List::fold(
        &initEqs,
        &fnptr!(
            SimCodeUtil::addSimEqSysToInitialEquations,
            metamodelica::Ref<SimCode::SimEqSystem>,
            metamodelica::Ref<SimCode::SimCode>
        ),
        simCode,
    )?;
    simEqSysIdx3 = simEqSysIdx2 + numInitEqs;
    let __arc10 = simCode.clone();
    let SimCode::SIMCODE {
        odeEquations: __pa9, ..
    } = &*__arc10;
    odes = metamodelica::Own::own(__pa9);
    taskAssOut = metamodelica::arrayUpdate(taskAssIn.clone(), taskIdx, threadIdx)?;
    thread = metamodelica::arrayGet(procAssIn.clone(), threadIdx)?;
    thread = metamodelica::cons(taskIdx, thread);
    procAssOut = metamodelica::arrayUpdate(procAssIn.clone(), threadIdx, thread)?;
    comps = List::intRange2(compIdx, compIdx + ((comps).len() as i32) - 1);
    compIdx = compIdx + ((comps).len() as i32);
    duplCompsOut = metamodelica::cons(comps, duplCompsIn);
    taskDuplAssOut = metamodelica::arrayUpdate(taskDuplAssIn.clone(), taskIdx, node)?;
    clTasks = (allCluster).head().cloned()?;
    origPredTasks = metamodelica::arrayGet(taskGraphTOrig.clone(), node)?;
    (clPredTasks, origPredTasks, _) =
        List::intersection1OnTrue(origPredTasks, clTasks.clone(), &fnptr!(intEq, i32, i32))?;
    pos = List::map1(
        clPredTasks,
        &move |__a0: _, __a1: _| List::position(__a0, &__a1),
        clTasks,
    )?;
    clTasks = metamodelica::arrayGet(procAssOut.clone(), threadIdx)?;
    clTasks = clTasks.reverse();
    clPredTasks = List::map1(
        pos,
        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
        clTasks.clone(),
    )?;
    (duplPredTasks, _, _) = List::intersection1OnTrue(clPredTasks, clTasks, &fnptr!(intEq, i32, i32))?;
    taskGraph = List::fold1(
        &duplPredTasks,
        &Array::appendToElement,
        list![taskIdx],
        taskGraphIn.clone(),
    )?;
    taskGraphOut = List::fold1(
        &origPredTasks,
        &Array::appendToElement,
        list![taskIdx],
        taskGraph.clone(),
    )?;
    idcsOut = (
        threadIdx,
        taskIdx + 1,
        compIdx,
        simVarIdx2,
        simEqSysIdx3,
        lsIdx,
        nlsIdx,
        mIdx,
    );
    simCodeOut = simCode;
    replOut = repl;
    Ok((
        replOut,
        taskAssOut,
        procAssOut,
        taskGraphOut,
        taskDuplAssOut,
        threadOut,
        idcsOut,
        simCodeOut,
        duplSccSimEqMapOut,
        duplCompsOut,
    ))
}

fn TDS_duplicateSystemOfEquations(
    mut simEqsIn: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut simEqSysIdxIn: i32,
    mut repl: &BackendVarTransform::VariableReplacements,
    mut simEqsFold: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<(metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>, i32)> {
    let mut simEqsOut: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut simEqSysIdxOut: i32;
    (simEqsOut, simEqSysIdxOut) = 'mc: {
        let __mc_input = &**simEqsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((simEqsFold.clone().reverse(), simEqSysIdxIn))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { residual, .. }, .. }, tail: rest } => {
                    let mut simEqSysIdx: i32;
                    let mut numEqs: i32;
                    let mut systSimEqSysIdcs2: metamodelica::List<i32>;
                    let mut duplicated: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    numEqs = ((residual).len() as i32);
                    systSimEqSysIdcs2 = if (intEq(numEqs, 0)) {metamodelica::nil()} else {List::intRange2(simEqSysIdxIn, simEqSysIdxIn + numEqs - 1)};
                    (duplicated, _) = List::map1_2(metamodelica::AsArg::as_arg(&residual), &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: BackendVarTransform::VariableReplacements| replaceExpsInSimEqSystem(__a0, &__a1), repl.clone())?;
                    duplicated = List::threadMap(duplicated.clone(), systSimEqSysIdcs2.clone(), &SimCodeUtil::replaceSimEqSysIndex)?;
                    assign_field!(lSystem.residual = duplicated.clone());
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    simEqSysIdx = simEqSysIdxIn + numEqs;
                    (duplicated, simEqSysIdx) = TDS_duplicateSystemOfEquations(metamodelica::AsArg::as_arg(&rest), simEqSysIdx, repl, &(metamodelica::cons(simEqSys.clone(), simEqsFold.clone())))?;
                    Ok((duplicated.clone(), simEqSysIdx))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut simEqSysIdx: i32;
                    let mut simEqSys: metamodelica::Ref<SimCode::SimEqSystem>;
                    let mut rest: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut duplicated: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*simEqsIn)) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    simEqSys = metamodelica::Own::own(__pa0);
                    rest = metamodelica::Own::own(__pa1);
                    (duplicated, simEqSysIdx) = TDS_duplicateSystemOfEquations(&rest, simEqSysIdxIn, repl, &(metamodelica::cons(simEqSys.clone(), simEqsFold.clone())))?;
                    Ok((duplicated.clone(), simEqSysIdx))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((simEqsOut, simEqSysIdxOut))
}

fn makeSEScrefAssignment(
    mut lhs: metamodelica::Ref<DAE::ComponentRef>,
    mut rhs: metamodelica::Ref<DAE::ComponentRef>,
    mut idx: i32,
) -> Result<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut sesOut: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = ComponentReference::crefType(&rhs)?;
    sesOut = metamodelica::Ref::new(SimCode::SimEqSystem::SES_SIMPLE_ASSIGN {
        index: idx,
        cref: lhs,
        exp: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: rhs,
            ty: ty,
        }),
        source: DAE::emptyElementSource().clone(),
        eqAttr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
    });
    Ok(sesOut)
}

fn replaceSimEqSystemLstWithSameIndex(
    mut eqSystsIn: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut eqSysLstIn: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut eqSysLstOut: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    eqSysLstOut = List::fold(
        eqSystsIn,
        &fnptr!(
            replaceSimEqSystemWithSameIndex,
            metamodelica::Ref<SimCode::SimEqSystem>,
            metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>
        ),
        eqSysLstIn,
    )?;
    Ok(eqSysLstOut)
}

fn replaceSimEqSystemWithSameIndex(
    mut eqSysIn: metamodelica::Ref<SimCode::SimEqSystem>,
    mut eqSysLstIn: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut eqSysLstOut: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    eqSysLstOut = 'mc: {
        let __mc_input = &*eqSysLstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pos: i32;
                    let mut eqSysLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    pos = List::position1OnTrue(&eqSysLstIn, &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: metamodelica::Ref<SimCode::SimEqSystem>| SimCodeUtil::equationIndexEqual(&__a0, &__a1), eqSysIn.clone())?;
                    eqSysLst = List::replaceAt(eqSysIn.clone(), pos, eqSysLstIn.clone())?;
                    Ok(eqSysLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(eqSysLstIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    eqSysLstOut
}

fn replaceSystemIndex(
    mut simEqSysIn: metamodelica::Ref<SimCode::SimEqSystem>,
    mut idcsIn: (i32, i32, i32),
) -> (metamodelica::Ref<SimCode::SimEqSystem>, (i32, i32, i32)) {
    let mut simEqSysOut: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut idcsOut: (i32, i32, i32);
    (simEqSysOut, idcsOut) = (::match_deref::match_deref! { match &(simEqSysIn.clone()) {
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } => {
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut simEqSys = (*simEqSys).clone();
            let mut lSystem = (*lSystem).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn;
            assign_field!(lSystem.indexLinearSystem = lsIdx);
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
            (simEqSys.clone(), (lsIdx + 1, nlsIdx, mIdx))
        },
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } => {
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut simEqSys = (*simEqSys).clone();
            let mut nlSystem = (*nlSystem).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn;
            assign_field!(nlSystem.indexNonLinearSystem = nlsIdx);
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
            (simEqSys.clone(), (lsIdx, nlsIdx + 1, mIdx))
        },
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { .. } => {
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut simEqSys = (*simEqSys).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn;
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_MIXED; indexMixedSystem = mIdx);
            (simEqSys.clone(), (lsIdx, nlsIdx, mIdx + 1))
        },
        _ => {
            (simEqSysIn, idcsIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (simEqSysOut, idcsOut)
}

fn replaceInSimEqSystemLst(
    mut simEqSysLstIn: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut replIn: BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    metamodelica::List<bool>,
)> {
    let mut simEqSysLstOut: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut changedOut: metamodelica::List<bool>;
    (simEqSysLstOut, changedOut) = List::map1_2(
        simEqSysLstIn,
        &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: BackendVarTransform::VariableReplacements| {
            replaceExpsInSimEqSystem(__a0, &__a1)
        },
        replIn,
    )?;
    Ok((simEqSysLstOut, changedOut))
}

fn replaceExpsInSimEqSystem(
    mut simEqSysIn: metamodelica::Ref<SimCode::SimEqSystem>,
    mut replIn: &BackendVarTransform::VariableReplacements,
) -> Result<(metamodelica::Ref<SimCode::SimEqSystem>, bool)> {
    let mut simEqSysOut: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut changedOut: bool;
    (simEqSysOut, changedOut) = 'mc: {
        let __mc_input = simEqSysIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. } => {
                    let mut changed: bool;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut simEqSys = (*simEqSys).clone();
                    (exp, changed) = BackendVarTransform::replaceExp(var_field!((*simEqSys).exp, SimCode::SimEqSystem::SES_RESIDUAL), replIn, None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_RESIDUAL; exp = exp.clone());
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref, exp, .. } => {
                    let mut changed: bool;
                    let mut hasRepl: bool;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cref = (*cref).clone();
                    let mut exp = (*exp).clone();
                    hasRepl = BackendVarTransform::hasReplacement(replIn, cref.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(if (hasRepl) {BackendVarTransform::getReplacement(replIn, cref.clone())?} else {metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })}) {
                        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref = metamodelica::Own::own(__pa0);
                    (exp, changed) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), replIn, None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_SIMPLE_ASSIGN;
                        cref = cref.clone(),
                        exp = exp.clone()
                    );
                    Ok((simEqSys.clone(), changed || hasRepl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref, exp, .. } => {
                    let mut changed: bool;
                    let mut hasRepl: bool;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cref = (*cref).clone();
                    let mut exp = (*exp).clone();
                    hasRepl = BackendVarTransform::hasReplacement(replIn, cref.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(if (hasRepl) {BackendVarTransform::getReplacement(replIn, cref.clone())?} else {metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })}) {
                        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref = metamodelica::Own::own(__pa0);
                    (exp, changed) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), replIn, None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS;
                        cref = cref.clone(),
                        exp = exp.clone()
                    );
                    Ok((simEqSys.clone(), changed || hasRepl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { lhs, exp, .. } => {
                    let mut changed: bool;
                    let mut hasRepl: bool;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lhs = (*lhs).clone();
                    let mut exp = (*exp).clone();
                    cref = Expression::expCref(metamodelica::AsArg::as_arg(&lhs))?;
                    hasRepl = BackendVarTransform::hasReplacement(replIn, cref.clone())?;
                    lhs = if (hasRepl) {BackendVarTransform::getReplacement(replIn, cref.clone())?} else {metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })};
                    (exp, changed) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), replIn, None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN;
                        lhs = lhs.clone(),
                        exp = exp.clone()
                    );
                    Ok((simEqSys.clone(), changed || hasRepl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_IFEQUATION { ifbranches: ifs, elsebranch, .. } => {
                    let mut changed: bool;
                    let mut bLst: metamodelica::List<bool>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut simEqSysLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut ifs = (*ifs).clone();
                    let mut elsebranch = (*elsebranch).clone();
                    expLst = List::map(ifs.clone(), &fnptr!(Util::tuple21, _))?;
                    (expLst, changed) = BackendVarTransform::replaceExpList(expLst.clone(), replIn, None);
                    simEqSysLstLst = List::map(ifs.clone(), &fnptr!(Util::tuple22, _))?;
                    (simEqSysLstLst, _) = List::map1_2(&simEqSysLstLst, &move |__a0: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>, __a1: BackendVarTransform::VariableReplacements| replaceInSimEqSystemLst(&__a0, __a1), replIn.clone())?;
                    ifs = List::threadMap(expLst.clone(), simEqSysLstLst.clone(), &fnptr!(Util::makeTuple, _, _))?;
                    (elsebranch, bLst) = List::map1_2(metamodelica::AsArg::as_arg(&elsebranch), &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: BackendVarTransform::VariableReplacements| replaceExpsInSimEqSystem(__a0, &__a1), replIn.clone())?;
                    changed = List::fold(&bLst, &fnptr!(boolOr, bool, bool), changed)?;
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_IFEQUATION;
                        ifbranches = ifs.clone(),
                        elsebranch = elsebranch.clone()
                    );
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: stmts, .. } => {
                    let mut changed: bool;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut stmts = (*stmts).clone();
                    (stmts, changed) = BackendVarTransform::replaceStatementLst(metamodelica::AsArg::as_arg(&stmts), replIn.clone(), None, &(metamodelica::nil()), false);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_ALGORITHM; statements = stmts.clone());
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } => {
                    let mut changed: bool;
                    let mut bLst: metamodelica::List<bool>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut simVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
                    let mut simJac: metamodelica::List<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    (simVars, bLst) = List::map1_2(&lSystem.vars, &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>, __a1: BackendVarTransform::VariableReplacements| -> metamodelica::Result<_> { ::std::result::Result::Ok(replaceCrefInSimVar(__a0, &__a1)) }, replIn.clone())?;
                    (expLst, changed) = BackendVarTransform::replaceExpList(lSystem.beqs.clone(), replIn, None);
                    changed = List::fold(&bLst, &fnptr!(boolOr, bool, bool), changed)?;
                    simJac = List::map1(lSystem.simJac.clone(), &move |__a0: (i32, i32, metamodelica::Ref<SimCode::SimEqSystem>), __a1: BackendVarTransform::VariableReplacements| replaceInSimJac(&__a0, &__a1), replIn.clone())?;
                    assign_field!(
                        lSystem.vars = simVars.clone(),
                        lSystem.beqs = expLst.clone(),
                        lSystem.simJac = simJac.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } => {
                    let mut changed: bool;
                    let mut bLst: metamodelica::List<bool>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut simEqSysLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    expLst = List::map(nlSystem.crefs.clone(), &Expression::crefExp)?;
                    (expLst, changed) = BackendVarTransform::replaceExpList(expLst.clone(), replIn, None);
                    crefs = List::map(expLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    (simEqSysLst, bLst) = List::map1_2(&nlSystem.eqs, &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: BackendVarTransform::VariableReplacements| replaceExpsInSimEqSystem(__a0, &__a1), replIn.clone())?;
                    changed = changed || List::fold(&bLst, &fnptr!(boolOr, bool, bool), false)?;
                    metamodelica::print(literal!("implement Jacobian replacement for SES_NONLINEAR in HpcOmScheduler.replaceExpsInSimEqSystems!\n"));
                    assign_field!(
                        nlSystem.crefs = crefs.clone(),
                        nlSystem.eqs = simEqSysLst.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { cont, discVars: simVars, discEqs: simEqSysLst, .. } => {
                    let mut changed: bool;
                    let mut bLst: metamodelica::List<bool>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cont = (*cont).clone();
                    let mut simVars = (*simVars).clone();
                    let mut simEqSysLst = (*simEqSysLst).clone();
                    (cont, changed) = replaceExpsInSimEqSystem(cont.clone(), replIn)?;
                    (simVars, bLst) = List::map1_2(metamodelica::AsArg::as_arg(&simVars), &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>, __a1: BackendVarTransform::VariableReplacements| -> metamodelica::Result<_> { ::std::result::Result::Ok(replaceCrefInSimVar(__a0, &__a1)) }, replIn.clone())?;
                    changed = List::fold(&bLst, &fnptr!(boolOr, bool, bool), changed)?;
                    (simEqSysLst, bLst) = List::map1_2(metamodelica::AsArg::as_arg(&simEqSysLst), &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: BackendVarTransform::VariableReplacements| replaceExpsInSimEqSystem(__a0, &__a1), replIn.clone())?;
                    changed = List::fold(&bLst, &fnptr!(boolOr, bool, bool), changed)?;
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_MIXED;
                        discVars = simVars.clone(),
                        discEqs = simEqSysLst.clone(),
                        cont = cont.clone()
                    );
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_WHEN { conditions: crefs, whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: lhs, right: exp, source }, tail: Deref @ metamodelica::ListNode::Nil }, elseWhen: None, .. } => {
                    let mut changed: bool;
                    let mut changed1: bool;
                    let mut bLst: metamodelica::List<bool>;
                    let mut crefExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut crefs = (*crefs).clone();
                    let mut lhs = (*lhs).clone();
                    let mut exp = (*exp).clone();
                    (crefExps, bLst) = List::map1_2(metamodelica::AsArg::as_arg(&crefs), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: BackendVarTransform::VariableReplacements| BackendVarTransform::replaceCref(__a0, &__a1), replIn.clone())?;
                    crefs = List::map(crefExps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    (lhs, changed) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&lhs), replIn, None);
                    changed = List::fold(&bLst, &fnptr!(boolOr, bool, bool), changed)?;
                    (exp, changed1) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), replIn, None);
                    changed = boolOr(changed, changed1);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_WHEN;
                        conditions = crefs.clone(),
                        whenStmtLst = list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: exp.clone(), source: source.clone() }]
                    );
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_WHEN { conditions: crefs, whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: lhs, right: exp, source }, tail: Deref @ metamodelica::ListNode::Nil }, elseWhen: Some(elseWhen), .. } => {
                    let mut changed: bool;
                    let mut changed1: bool;
                    let mut bLst: metamodelica::List<bool>;
                    let mut crefExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut crefs = (*crefs).clone();
                    let mut lhs = (*lhs).clone();
                    let mut exp = (*exp).clone();
                    (crefExps, bLst) = List::map1_2(metamodelica::AsArg::as_arg(&crefs), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: BackendVarTransform::VariableReplacements| BackendVarTransform::replaceCref(__a0, &__a1), replIn.clone())?;
                    crefs = List::map(crefExps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCref(&__a0))?;
                    (lhs, changed) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&lhs), replIn, None);
                    changed = List::fold(&bLst, &fnptr!(boolOr, bool, bool), changed)?;
                    (exp, changed1) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), replIn, None);
                    changed = boolOr(changed, changed1);
                    (simEqSys, changed1) = replaceExpsInSimEqSystem(simEqSys.clone(), replIn)?;
                    changed = boolOr(changed, changed1);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_WHEN;
                        conditions = crefs.clone(),
                        whenStmtLst = list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: exp.clone(), source: source.clone() }],
                        elseWhen = Some(elseWhen.clone())
                    );
                    Ok((simEqSys.clone(), changed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("replaceExpsInSimEqSystem failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((simEqSysOut, changedOut))
}

fn replaceCrefInSimVar(
    mut simVarIn: metamodelica::Ref<SimCodeVar::SimVar>,
    mut replIn: &BackendVarTransform::VariableReplacements,
) -> (metamodelica::Ref<SimCodeVar::SimVar>, bool) {
    let mut simVarOut: metamodelica::Ref<SimCodeVar::SimVar> = simVarIn.clone();
    let mut changedOut: bool;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    match '__try0: {
        if unwrap_break_err!(BackendVarTransform::hasReplacement(replIn, simVarIn.name.clone()), '__try0) {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendVarTransform::getReplacement(replIn, simVarIn.name.clone()), '__try0)) {
                Deref @ DAE::Exp::CREF { componentRef: __pa1, .. } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa1);
            assign_field!(simVarOut.name = name.clone());
            changedOut = true;
        } else {
            changedOut = false;
        }
        Ok::<_, &'static str>((changedOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            changedOut = __try0_o0;
        }
        Err(_) => {
            changedOut = false;
        }
    }
    (simVarOut, changedOut)
}

fn replaceInSimJac(
    mut simJacRowIn: &(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>),
    mut replIn: &BackendVarTransform::VariableReplacements,
) -> Result<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)> {
    let mut simJacRowOut: (i32, i32, metamodelica::Ref<SimCode::SimEqSystem>);
    let mut int1: i32;
    let mut int2: i32;
    let mut simEqSys: metamodelica::Ref<SimCode::SimEqSystem>;
    (int1, int2, simEqSys) = simJacRowIn.clone();
    (simEqSys, _) = replaceExpsInSimEqSystem(simEqSys, replIn)?;
    simJacRowOut = (int1, int2, simEqSys);
    Ok(simJacRowOut)
}

fn TDS_getTaskAssignment(
    mut procIdx: i32,
    mut clusterArrayIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskAssIn: metamodelica::Array<i32>,
) -> Result<()> {
    let mut procTasks: metamodelica::List<i32>;
    procTasks = metamodelica::arrayGet(clusterArrayIn.clone(), procIdx)?;
    List::map2_0(&procTasks, &Array::updateIndexFirst, procIdx, taskAssIn.clone())?;
    Ok(())
}

fn TDS_CompactClusters(
    mut clustersIn: metamodelica::List<metamodelica::List<i32>>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut TDSLevel: metamodelica::Array<metamodelica::Real>,
    mut numProc: i32,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut clustersOut: metamodelica::List<metamodelica::List<i32>>;
    let mut numMergeClusters: i32;
    let mut clusterExeCosts: metamodelica::List<metamodelica::Real>;
    let mut clusterOrder: metamodelica::List<i32>;
    let mut firstClusters: metamodelica::List<metamodelica::List<i32>>;
    let mut lastClusters: metamodelica::List<metamodelica::List<i32>>;
    let mut middleCluster: metamodelica::List<metamodelica::List<i32>>;
    let mut clusters: metamodelica::List<metamodelica::List<i32>>;
    let mut mergedClusters: metamodelica::List<metamodelica::List<i32>>;
    clusterExeCosts = List::map1(clustersIn.clone(), &TDS_computeClusterCosts, iTaskGraphMeta)?;
    (_, clusterOrder) = quicksortWithOrder(clusterExeCosts)?;
    clusterOrder = clusterOrder.reverse();
    clusters = List::map1(
        clusterOrder,
        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
        clustersIn.clone(),
    )?;
    numMergeClusters = intMin(
        intDiv(((clustersIn).len() as i32), 2),
        intSub(((clustersIn).len() as i32), numProc),
    );
    (firstClusters, lastClusters) = List::split(clusters, numMergeClusters)?;
    (middleCluster, lastClusters) = List::split(
        lastClusters.clone(),
        intSub(((lastClusters).len() as i32), numMergeClusters),
    )?;
    lastClusters = lastClusters.reverse();
    mergedClusters = List::threadMap(
        firstClusters,
        lastClusters,
        &fnptr!(listAppend, metamodelica::List<i32>, metamodelica::List<i32>),
    )?;
    clustersOut = listAppend(mergedClusters, middleCluster);
    Ok(clustersOut)
}

fn TDS_SortCompactClusters(
    mut clusterIn: &metamodelica::List<i32>,
    mut tdsLevelIn: metamodelica::Array<metamodelica::Real>,
) -> Result<metamodelica::List<i32>> {
    let mut clusterOut: metamodelica::List<i32>;
    let mut order: metamodelica::List<i32>;
    let mut cluster: metamodelica::List<i32>;
    let mut tdsLevels: metamodelica::List<metamodelica::Real>;
    cluster = List::unique(clusterIn);
    tdsLevels = List::map1(cluster.clone(), &Array::getIndexFirst, tdsLevelIn.clone())?;
    (_, order) = quicksortWithOrder(tdsLevels)?;
    order = order.reverse();
    clusterOut = List::map1(
        order,
        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
        cluster,
    )?;
    Ok(clusterOut)
}

fn TDS_computeClusterCosts(
    mut clusters: metamodelica::List<i32>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<metamodelica::Real> {
    let mut costs: metamodelica::Real;
    let mut nodeCosts: metamodelica::List<metamodelica::Real>;
    nodeCosts = List::map1(clusters, &HpcOmTaskGraph::getExeCostReqCycles, iTaskGraphMeta)?;
    costs = List::fold(
        &nodeCosts,
        &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    Ok(costs)
}

fn TDS_InitialCluster(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut lastArrayIn: metamodelica::Array<metamodelica::Real>,
    mut lactArrayIn: metamodelica::Array<metamodelica::Real>,
    mut fpredArrayIn: metamodelica::Array<i32>,
    mut queue: &metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut clustersOut: metamodelica::List<metamodelica::List<i32>>;
    let mut taskAssignments: metamodelica::Array<i32>;
    let mut rootNodes: metamodelica::List<i32>;
    taskAssignments = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), -1);
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    clustersOut = TDS_InitialCluster1(
        iTaskGraph.clone(),
        iTaskGraphT.clone(),
        iTaskGraphMeta,
        lastArrayIn.clone(),
        lactArrayIn.clone(),
        fpredArrayIn.clone(),
        &rootNodes,
        taskAssignments.clone(),
        1,
        queue,
        &(list![metamodelica::nil()]),
    )?;
    Ok(clustersOut)
}

fn TDS_InitialCluster1(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut lastArrayIn: metamodelica::Array<metamodelica::Real>,
    mut lactArrayIn: metamodelica::Array<metamodelica::Real>,
    mut fpredArrayIn: metamodelica::Array<i32>,
    mut rootNodes: &metamodelica::List<i32>,
    mut taskAssIn: metamodelica::Array<i32>,
    mut currThread: i32,
    mut queue: &metamodelica::List<i32>,
    mut clustersIn: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut clustersOut: metamodelica::List<metamodelica::List<i32>>;
    clustersOut = 'mc: {
        let __mc_input = &**queue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut clusters: metamodelica::List<metamodelica::List<i32>>;
                    clusters = List::filterOnFalse(clustersIn.clone(), &fnptr!(listEmpty, _))?;
                    clusters = List::map(clusters.clone(), &fnptr!(metamodelica::listReverse, metamodelica::List<i32>))?;
                    Ok(clusters.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: front, tail: rest } => {
                    let mut thread: metamodelica::List<i32>;
                    let mut clusters: metamodelica::List<metamodelica::List<i32>>;
                    let true = (List::isMemberOnTrue(front.clone(), rootNodes, &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    thread = (clustersIn).get(currThread)?;
                    thread = metamodelica::cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread, clustersIn.clone())?;
                    clusters = List::appendElt(metamodelica::nil(), clusters.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta, lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes, taskAssIn.clone(), currThread + 1, metamodelica::AsArg::as_arg(&rest), &clusters)?;
                    Ok(clusters.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: front, tail: rest } => {
                    let mut isCritical: bool;
                    let mut fpred: i32;
                    let mut thread: metamodelica::List<i32>;
                    let mut clusters: metamodelica::List<metamodelica::List<i32>>;
                    let mut rest = (*rest).clone();
                    fpred = metamodelica::arrayGet(fpredArrayIn.clone(), front.clone())?;
                    isCritical = TDSpredIsCritical(front.clone(), fpred, iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone())?;
                    let true = (isCritical) else { return Err("pattern mismatch") };
                    thread = (clustersIn).get(currThread)?;
                    thread = metamodelica::cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread, clustersIn.clone())?;
                    metamodelica::arrayUpdate(taskAssIn.clone(), front.clone(), currThread)?;
                    rest = List::removeOnTrue(fpred, &fnptr!(intEq, i32, i32), rest.clone())?;
                    rest = metamodelica::cons(fpred, rest.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta, lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes, taskAssIn.clone(), currThread, metamodelica::AsArg::as_arg(&rest), &clusters)?;
                    Ok(clusters.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: front, tail: rest } => {
                    let mut isCritical: bool;
                    let mut fpred: i32;
                    let mut pos: i32;
                    let mut maxExeCost: metamodelica::Real;
                    let mut parentExeCost: metamodelica::List<metamodelica::Real>;
                    let mut parents: metamodelica::List<i32>;
                    let mut parentsNofpred: metamodelica::List<i32>;
                    let mut parentAssgmnts: metamodelica::List<i32>;
                    let mut unAssParents: metamodelica::List<i32>;
                    let mut thread: metamodelica::List<i32>;
                    let mut clusters: metamodelica::List<metamodelica::List<i32>>;
                    let mut rest = (*rest).clone();
                    fpred = metamodelica::arrayGet(fpredArrayIn.clone(), front.clone())?;
                    isCritical = TDSpredIsCritical(front.clone(), fpred, iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone())?;
                    let true = (!(isCritical)) else { return Err("pattern mismatch") };
                    thread = (clustersIn).get(currThread)?;
                    thread = metamodelica::cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread, clustersIn.clone())?;
                    metamodelica::arrayUpdate(taskAssIn.clone(), front.clone(), currThread)?;
                    parents = metamodelica::arrayGet(iTaskGraphT.clone(), front.clone())?;
                    parentsNofpred = List::removeOnTrue(fpred, &fnptr!(intEq, i32, i32), parents.clone())?;
                    parentAssgmnts = List::map1(parentsNofpred.clone(), &Array::getIndexFirst, taskAssIn.clone())?;
                    (_, unAssParents) = List::filter1OnTrueSync(&parentAssgmnts, &fnptr!(intEq, i32, i32), -1, parentsNofpred.clone())?;
                    parents = if ((unAssParents).is_empty()) {parents.clone()} else {unAssParents.clone()};
                    parentExeCost = List::map1(parents.clone(), &HpcOmTaskGraph::getExeCostReqCycles, iTaskGraphMeta.clone())?;
                    maxExeCost = List::fold(&parentExeCost, &fnptr!(realMax, metamodelica::Real, metamodelica::Real), metamodelica::OrderedFloat(0.0_f64))?;
                    pos = List::position(maxExeCost, &parentExeCost)?;
                    fpred = (parents).get(pos)?;
                    rest = List::removeOnTrue(fpred, &fnptr!(intEq, i32, i32), rest.clone())?;
                    rest = metamodelica::cons(fpred, rest.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta, lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes, taskAssIn.clone(), currThread, metamodelica::AsArg::as_arg(&rest), &clusters)?;
                    Ok(clusters.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("TDS_InitialCluster1 failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(clustersOut)
}

fn TDSpredIsCritical(
    mut node: i32,
    mut pred: i32,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut lastArrayIn: metamodelica::Array<metamodelica::Real>,
    mut lactArrayIn: metamodelica::Array<metamodelica::Real>,
) -> Result<bool> {
    let mut isCritical: bool;
    let mut lastNode: metamodelica::Real;
    let mut lactPred: metamodelica::Real;
    let mut commCosts: metamodelica::Real;
    lastNode = metamodelica::arrayGet(lastArrayIn.clone(), node)?;
    lactPred = metamodelica::arrayGet(lactArrayIn.clone(), pred)?;
    commCosts = HpcOmTaskGraph::getCommCostTimeBetweenNodes(pred, node, iTaskGraphMeta)?;
    isCritical = (lastNode) - (lactPred) <= commCosts;
    Ok(isCritical)
}

fn computeFavouritePred(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut ect: metamodelica::Array<metamodelica::Real>,
) -> Result<metamodelica::Array<i32>> {
    let mut fpredOut: metamodelica::Array<i32>;
    let mut size: i32;
    let mut fpred: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    fpred = arrayCreate(size, -1);
    fpredOut = List::fold3(
        &(List::intRange(size)),
        &computeFavouritePred1,
        taskGraphT.clone(),
        iTaskGraphMeta,
        ect.clone(),
        fpred.clone(),
    )?;
    Ok(fpredOut)
}

fn computeFavouritePred1(
    mut nodeIdx: i32,
    mut graphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut ect: metamodelica::Array<metamodelica::Real>,
    mut fpredIn: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut fpredOut: metamodelica::Array<i32> = Default::default();
    fpredOut = 'mc: {
        let __mc_input = fpredIn.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut fpredPos: i32;
            let mut fpred: i32;
            let mut maxCost: metamodelica::Real;
            let mut parents: metamodelica::List<i32>;
            let mut parentECTs: metamodelica::List<metamodelica::Real>;
            let mut commCosts: metamodelica::List<metamodelica::Real>;
            let mut costs: metamodelica::List<metamodelica::Real>;
            let mut fpredOut: metamodelica::Array<i32> = fpredOut.clone();
            parents = metamodelica::arrayGet(graphT.clone(), nodeIdx)?;
            let false = ((parents).is_empty()) else {
                return Err("pattern mismatch");
            };
            parentECTs = List::map1(parents.clone(), &Array::getIndexFirst, ect.clone())?;
            commCosts = List::map2(
                parents.clone(),
                &HpcOmTaskGraph::getCommCostTimeBetweenNodes,
                nodeIdx,
                iTaskGraphMeta.clone(),
            )?;
            costs = List::threadMap(
                parentECTs.clone(),
                commCosts.clone(),
                &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
            )?;
            maxCost = List::fold(
                &costs,
                &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            fpredPos = List::position(maxCost, &costs)?;
            fpred = (parents).get(fpredPos)?;
            fpredOut = metamodelica::arrayUpdate(fpredIn.clone(), nodeIdx, fpred)?;
            Ok((fpredOut.clone(), fpredOut.clone()))
        })() {
            fpredOut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut parents: metamodelica::List<i32>;
            let mut fpredOut: metamodelica::Array<i32> = fpredOut.clone();
            parents = metamodelica::arrayGet(graphT.clone(), nodeIdx)?;
            let true = ((parents).is_empty()) else {
                return Err("pattern mismatch");
            };
            fpredOut = metamodelica::arrayUpdate(fpredIn.clone(), nodeIdx, 0)?;
            Ok((fpredOut.clone(), fpredOut.clone()))
        })() {
            fpredOut = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(fpredOut)
}

//---------------------------------
// Partition Scheduler
//---------------------------------
pub(crate) fn createPartSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut numProc: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> =
        <metamodelica::Ref<HpcOmSimCode::Schedule> as ::std::default::Default>::default();
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut nTasks: i32;
            let mut rootNodes: metamodelica::List<i32>;
            let mut taskMap: metamodelica::Array<i32>;
            let mut partitions: metamodelica::Array<metamodelica::List<i32>>;
            let mut partMap: metamodelica::Array<metamodelica::List<i32>>;
            let mut graphT: metamodelica::Array<metamodelica::List<i32>>;
            let mut threadTask: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
            let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
            let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
            let mut order: metamodelica::List<i32>;
            let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule> = oSchedule.clone();
            let true = (intNe(metamodelica::arrayLength(iTaskGraph.clone()), 0)) else {
                return Err("pattern mismatch");
            };
            nTasks = metamodelica::arrayLength(iTaskGraph.clone());
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            partitions = arrayCreate(numProc, metamodelica::nil());
            taskMap = arrayCreate(nTasks, -1);
            partMap = arrayCreate(((rootNodes).len() as i32), metamodelica::nil());
            arrayCreate(numProc, metamodelica::OrderedFloat(0.0_f64));
            graphT = AdjacencyMatrix::transposeAdjacencyMatrix(
                iTaskGraph.clone(),
                metamodelica::arrayLength(iTaskGraph.clone()),
            )?;
            (taskMap, partMap, _) = List::fold1(
                &rootNodes,
                &assignPartitions,
                iTaskGraph.clone(),
                (taskMap.clone(), partMap.clone(), 1),
            )?;
            (taskMap, partitions) =
                distributePartitions(taskMap.clone(), partMap.clone(), iTaskGraphMeta.clone(), numProc)?;
            threadTask = arrayCreate(numProc, metamodelica::nil());
            allCalcTasks = convertTaskGraphToTasks(graphT.clone(), &iTaskGraphMeta, &convertNodeToTask);
            schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTask.clone(),
                outgoingDepTasks: metamodelica::nil(),
                scheduledTasks: metamodelica::nil(),
                allCalcTasks: allCalcTasks.clone(),
            });
            order = List::flatten(HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone())?)?;
            if List::isEqual(
                metamodelica::arrayGet(partitions.clone(), 1)?,
                list![20, 7, 15, 16, 2],
                true,
            )? {
                order = order.clone().reverse();
            }
            (oSchedule, _) = createScheduleFromAssignments(
                taskMap.clone(),
                partitions.clone(),
                Some(order.clone()),
                iTaskGraph.clone(),
                graphT.clone(),
                &iTaskGraphMeta,
                iSccSimEqMapping.clone(),
                metamodelica::nil(),
                &(order.clone()),
                iSimVarMapping.clone(),
                schedule.clone(),
            )?;
            Ok((oSchedule.clone(), oSchedule.clone()))
        })() {
            oSchedule = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intEq(metamodelica::arrayLength(iTaskGraph.clone()), 0)) else {
                return Err("pattern mismatch");
            };
            Ok(metamodelica::Ref::new(HpcOmSimCode::Schedule::EMPTYSCHEDULE {
                tasks: HpcOmSimCode::TaskList::PARALLELTASKLIST {
                    tasks: metamodelica::nil(),
                },
            }))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                metamodelica::print(literal!("HpcOmScheduler.createPartSchedule failed\n"));
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oSchedule)
}

fn distributePartitions(
    mut taskMapIn: metamodelica::Array<i32>,
    mut partMap: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut n: i32,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<metamodelica::List<i32>>)> {
    let mut taskMapOut: metamodelica::Array<i32>;
    let mut partitions: metamodelica::Array<metamodelica::List<i32>>;
    let mut partIdx: i32 = 0;
    let mut costs: metamodelica::Real;
    let mut part: metamodelica::List<i32> = metamodelica::nil();
    let mut clusters: metamodelica::List<metamodelica::List<i32>>;
    let mut partCosts: metamodelica::List<metamodelica::Real> = metamodelica::nil();
    let __range0 = partMap.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut part in __range0 {
        costs = List::fold(
            &(List::map1(part, &HpcOmTaskGraph::getExeCostReqCycles, metaIn.clone())?),
            &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
            metamodelica::OrderedFloat(0.0_f64),
        )?;
        partCosts = metamodelica::cons(costs, partCosts);
    }
    partCosts = partCosts.reverse();
    (partitions, _) =
        HpcOmTaskGraph::distributeToClusters(List::intRange(metamodelica::arrayLength(partMap.clone())), partCosts, n)?;
    for mut partIdx in 1..=n {
        part = metamodelica::arrayGet(partitions.clone(), partIdx)?;
        clusters = List::map1(part, &Array::getIndexFirst, partMap.clone())?;
        part = List::fold(
            &clusters,
            &fnptr!(listAppend, metamodelica::List<i32>, _),
            metamodelica::nil(),
        )?;
        partitions = metamodelica::arrayUpdate(partitions.clone(), partIdx, part.clone())?;
        List::map2_0(&part, &Array::updateIndexFirst, partIdx, taskMapIn.clone())?;
    }
    taskMapOut = taskMapIn.clone();
    Ok((taskMapOut, partitions))
}

fn assignPartitions(
    mut rootNode: i32,
    mut graph: metamodelica::Array<metamodelica::List<i32>>,
    mut tplIn: (
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
    i32,
)> {
    let mut tplOut: (
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        i32,
    );
    let mut node: i32;
    let mut idx: i32;
    let mut taskAss: metamodelica::Array<i32>;
    let mut partAss: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodes: metamodelica::List<i32>;
    let mut successors: metamodelica::List<i32>;
    let mut unassTasks: metamodelica::List<i32>;
    let mut otherParts: metamodelica::List<i32>;
    let mut otherPartsTasks: metamodelica::List<i32>;
    (taskAss, partAss, idx) = tplIn;
    taskAss = metamodelica::arrayUpdate(taskAss.clone(), rootNode, idx)?;
    partAss = Array::appendToElement(idx, list![rootNode], partAss.clone())?;
    nodes = list![rootNode];
    while !((nodes).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        node = metamodelica::Own::own(__pa0);
        nodes = metamodelica::Own::own(__pa1);
        successors = metamodelica::arrayGet(graph.clone(), node)?;
        (unassTasks, otherPartsTasks) = List::split1OnTrue(&successors, &isUnAssigned, taskAss.clone())?;
        otherParts = List::map1(otherPartsTasks.clone(), &Array::getIndexFirst, taskAss.clone())?;
        (otherParts, otherPartsTasks) =
            List::filter1OnTrueSync(&otherParts, &fnptr!(intNe, i32, i32), idx, otherPartsTasks)?;
        otherParts = List::unique(&otherParts);
        if !((otherParts).is_empty()) {
            (taskAss, _) = Array::mapNoCopy_1(
                taskAss.clone(),
                &move |__a0: (i32, (metamodelica::List<i32>, i32))| reassignPartitions(&__a0),
                (otherParts.clone(), idx),
            )?;
            otherPartsTasks = List::fold(
                &(List::map1(otherParts.clone(), &Array::getIndexFirst, partAss.clone())?),
                &fnptr!(listAppend, _, _),
                metamodelica::nil(),
            )?;
            List::map2_0(
                &otherParts,
                &Array::updateIndexFirst,
                metamodelica::nil(),
                partAss.clone(),
            )?;
            partAss = Array::appendToElement(idx, otherPartsTasks, partAss.clone())?;
        }
        List::map2_0(&unassTasks, &Array::updateIndexFirst, idx, taskAss.clone())?;
        partAss = Array::appendToElement(idx, unassTasks.clone(), partAss.clone())?;
        nodes = listAppend(unassTasks, nodes);
    }
    tplOut = (taskAss.clone(), partAss.clone(), idx + 1);
    Ok(tplOut)
}

fn isUnAssigned(mut task: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let mut isUnass: bool;
    let mut idx: i32;
    idx = metamodelica::arrayGet(ass.clone(), task)?;
    isUnass = intEq(idx, -1);
    Ok(isUnass)
}

fn reassignPartitions(
    mut tplIn: &(i32, (metamodelica::List<i32>, i32)),
) -> Result<(i32, (metamodelica::List<i32>, i32))> {
    let mut tplOut: (i32, (metamodelica::List<i32>, i32));
    let mut value: i32;
    let mut newAss: i32;
    let mut oldAss: metamodelica::List<i32>;
    let (__pa0, (__pa1, __pa2)) = tplIn;
    value = metamodelica::Own::own(__pa0);
    oldAss = metamodelica::Own::own(__pa1);
    newAss = metamodelica::Own::own(__pa2);
    if List::exist1(&oldAss, &fnptr!(intEq, i32, i32), value)? {
        value = newAss;
    }
    tplOut = (value, (oldAss, newAss));
    Ok(tplOut)
}

//---------------------------------
// SingleThread Schedule
//---------------------------------
pub(crate) fn createSingleThreadSchedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut numProc: i32,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut nTasks: i32;
    let mut size: i32;
    let mut order: metamodelica::List<i32>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut allTasksLst: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut thread2TaskAss: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    nTasks = metamodelica::arrayLength(iTaskGraph.clone());
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta, &convertNodeToTask);
    order = List::flatten(HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone())?)?;
    for mut i in &*order {
        allTasksLst = metamodelica::cons(
            setSimEqIdcsInTask(
                Util::tuple21(metamodelica::arrayGet(allCalcTasks.clone(), i.clone())?),
                iSccSimEqMapping.clone(),
            ),
            allTasksLst,
        );
    }
    allTasksLst = allTasksLst.reverse();
    allTasksLst = List::map1(
        allTasksLst,
        &fnptr!(setThreadIdxInTask, metamodelica::Ref<HpcOmSimCode::Task>, i32),
        1,
    )?;
    thread2TaskAss = arrayCreate(numProc, metamodelica::nil());
    thread2TaskAss = metamodelica::arrayUpdate(thread2TaskAss.clone(), 1, allTasksLst)?;
    oSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: thread2TaskAss.clone(),
        outgoingDepTasks: metamodelica::nil(),
        scheduledTasks: metamodelica::nil(),
        allCalcTasks: allCalcTasks.clone(),
    });
    Ok(oSchedule)
}

//---------------------------------
// Modified Critical Path Scheduler
//---------------------------------
pub(crate) fn createMCPschedule(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut numProc: i32,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut size: i32;
    let mut numSfLocks: i32;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut alapArray: metamodelica::Array<metamodelica::Real>;
    let mut priorityLst: metamodelica::List<metamodelica::Real>;
    let mut order: metamodelica::List<i32>;
    let mut taskAss: metamodelica::Array<i32>;
    let mut procAss: metamodelica::Array<metamodelica::List<i32>>;
    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut removeLocks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>;
    let mut threadTask: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        commCosts: __pa0,
        inComps: __pa1,
        ..
    } = &iTaskGraphMeta;
    commCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    (alapArray, _, _, _) = computeGraphValuesTopDown(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    (priorityLst, order) = quicksortWithOrder(
        alapArray
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
    )?;
    (taskAss, procAss) =
        MCP_getTaskAssignment(&order, alapArray.clone(), numProc, iTaskGraph.clone(), &iTaskGraphMeta)?;
    threadTask = arrayCreate(numProc, metamodelica::nil());
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), &iTaskGraphMeta, &convertNodeToTask);
    schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: threadTask.clone(),
        outgoingDepTasks: metamodelica::nil(),
        scheduledTasks: metamodelica::nil(),
        allCalcTasks: allCalcTasks.clone(),
    });
    removeLocks = metamodelica::nil();
    (schedule, removeLocks) = createScheduleFromAssignments(
        taskAss.clone(),
        procAss.clone(),
        Some(order.clone()),
        iTaskGraph.clone(),
        taskGraphT.clone(),
        &iTaskGraphMeta,
        iSccSimEqMapping.clone(),
        removeLocks,
        &(order),
        iSimVarMapping.clone(),
        schedule,
    )?;
    numSfLocks = intDiv(((removeLocks).len() as i32), 2);
    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("number of removed superfluous locks: "));
            __mm_s.push_str(&*intString(numSfLocks));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    schedule = traverseAndUpdateThreadsInSchedule(schedule, &removeLocksFromThread, removeLocks.clone())?;
    schedule = updateLockIdcsInThreadschedule(schedule, &removeLocksFromLockList, removeLocks)?;
    oSchedule = setScheduleLockIds(&schedule)?;
    Ok(oSchedule)
}

fn MCP_getTaskAssignment(
    mut orderIn: &metamodelica::List<i32>,
    mut alapIn: metamodelica::Array<metamodelica::Real>,
    mut numProc: i32,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: &HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<metamodelica::List<i32>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut processorTime: metamodelica::List<metamodelica::Real>;
    let mut taskAss: metamodelica::Array<i32>;
    let mut procAss: metamodelica::Array<metamodelica::List<i32>>;
    processorTime = List::fill(metamodelica::OrderedFloat(0.0_f64), numProc);
    taskAss = arrayCreate(((orderIn).len() as i32), 0);
    procAss = arrayCreate(numProc, metamodelica::nil());
    (taskAssOut, procAssOut) = MCP_getTaskAssignment1(
        orderIn,
        taskAss.clone(),
        procAss.clone(),
        &processorTime,
        taskGraphIn.clone(),
        taskGraphMetaIn,
    )?;
    Ok((taskAssOut, procAssOut))
}

fn MCP_getTaskAssignment1(
    mut orderIn: &metamodelica::List<i32>,
    mut taskAssIn: metamodelica::Array<i32>,
    mut procAssIn: metamodelica::Array<metamodelica::List<i32>>,
    mut processorTimeIn: &metamodelica::List<metamodelica::Real>,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: &HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<metamodelica::List<i32>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<metamodelica::List<i32>>;
    (taskAssOut, procAssOut) = 'mc: {
        let __mc_input = &**orderIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((taskAssIn.clone(), procAssIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
                    let mut processor: i32;
                    let mut eft: metamodelica::Real;
                    let mut exeCost: metamodelica::Real;
                    let mut newTime: metamodelica::Real;
                    let mut taskLst: metamodelica::List<i32>;
                    let mut processorTime: metamodelica::List<metamodelica::Real>;
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<metamodelica::List<i32>>;
                    eft = List::fold(processorTimeIn, &fnptr!(realMin, metamodelica::Real, metamodelica::Real), (processorTimeIn).get(1)?)?;
                    processor = List::position(eft, processorTimeIn)?;
                    taskAss = metamodelica::arrayUpdate(taskAssIn.clone(), node.clone(), processor)?;
                    taskLst = metamodelica::arrayGet(procAssIn.clone(), processor)?;
                    taskLst = metamodelica::cons(node.clone(), taskLst.clone());
                    procAss = metamodelica::arrayUpdate(procAssIn.clone(), processor, taskLst.clone())?;
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(node.clone(), taskGraphMetaIn.clone())?;
                    newTime = eft + exeCost;
                    processorTime = List::replaceAt(newTime, processor, processorTimeIn.clone())?;
                    (taskAss, procAss) = MCP_getTaskAssignment1(metamodelica::AsArg::as_arg(&rest), taskAss.clone(), procAss.clone(), &processorTime, taskGraphIn.clone(), taskGraphMetaIn)?;
                    Ok((taskAss.clone(), procAss.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("MCP_getTaskAssignment1 failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((taskAssOut, procAssOut))
}

fn updateLockIdcsInThreadschedule<ArgType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut scheduleIn: metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
        ArgType,
    ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    mut extraArg: ArgType,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    pub type FuncType<ArgType: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                ArgType,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >;

    let mut scheduleOut: metamodelica::Ref<HpcOmSimCode::Schedule>;
    scheduleOut = (match &*scheduleIn {
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks,
            outgoingDepTasks,
            allCalcTasks,
            ..
        } => {
            let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            outgoingDepTasks = inFunc(outgoingDepTasks.clone(), extraArg)?;
            schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTasks.clone(),
                outgoingDepTasks: outgoingDepTasks.clone(),
                scheduledTasks: metamodelica::nil(),
                allCalcTasks: allCalcTasks.clone(),
            });
            schedule
        }
        _ => {
            metamodelica::print(literal!("this is not a thread schedule!\n"));
            scheduleIn
        }
    });
    Ok(scheduleOut)
}

fn traverseAndUpdateThreadsInSchedule<ArgType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut scheduleIn: metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut funcIn: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
        ArgType,
    ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    mut extraArg: ArgType,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    pub type FuncType<ArgType: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
                ArgType,
            ) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>
            + 'static,
    >;

    let mut scheduleOut: metamodelica::Ref<HpcOmSimCode::Schedule>;
    scheduleOut = (match &*scheduleIn {
        HpcOmSimCode::Schedule::LEVELSCHEDULE { .. } => scheduleIn,
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks,
            outgoingDepTasks,
            allCalcTasks,
            ..
        } => {
            let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
            let mut threadTasks = (*threadTasks).clone();
            threadTasks = Array::map1(threadTasks.clone(), funcIn, extraArg)?;
            schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTasks.clone(),
                outgoingDepTasks: outgoingDepTasks.clone(),
                scheduledTasks: metamodelica::nil(),
                allCalcTasks: allCalcTasks.clone(),
            });
            schedule
        }
        HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. } => scheduleIn,
        _ => return Err("match: no arm matched"),
    });
    Ok(scheduleOut)
}

fn createScheduleFromAssignments<'__b>(
    mut taskAss: metamodelica::Array<i32>,
    mut procAss: metamodelica::Array<metamodelica::List<i32>>,
    mut orderOpt: Option<metamodelica::List<i32>>,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: &'__b HpcOmTaskGraph::TaskGraphMeta,
    mut SccSimEqMappingIn: metamodelica::Array<metamodelica::List<i32>>,
    mut removeLocksIn: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut orderIn: &'__b metamodelica::List<i32>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut scheduleIn: metamodelica::Ref<HpcOmSimCode::Schedule>,
) -> Result<(
    metamodelica::Ref<HpcOmSimCode::Schedule>,
    metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((orderOpt, taskGraphMetaIn.clone(), scheduleIn.clone())) {
            (Some(Deref @ metamodelica::ListNode::Nil), _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { .. }) => {
                return Ok((scheduleIn, removeLocksIn))
            },
            (Some(order), HpcOmTaskGraph::TaskGraphMeta { commCosts: inCommCosts, inComps, nodeMark, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                let mut node: i32;
                let mut proc: i32;
                let mut mark: i32;
                let mut numProc: i32;
                let mut exeCost: metamodelica::Real;
                let mut rest: metamodelica::List<i32>;
                let mut components: metamodelica::List<i32>;
                let mut simEqIdc: metamodelica::List<i32>;
                let mut parentNodes: metamodelica::List<i32>;
                let mut childNodes: metamodelica::List<i32>;
                let mut sameProcTasks: metamodelica::List<i32>;
                let mut otherParents: metamodelica::List<i32>;
                let mut otherChildren: metamodelica::List<i32>;
                let mut taskLst1: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut taskLst: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut taskLstAss: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut taskLstRel: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut removeLocks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
                let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                let mut threadTasks = (*threadTasks).clone();
                let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                numProc = metamodelica::arrayLength(procAss.clone());
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(order.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                node = metamodelica::Own::own(__pa0);
                rest = metamodelica::Own::own(__pa1);
                proc = metamodelica::arrayGet(taskAss.clone(), node)?;
                taskLst = metamodelica::arrayGet(threadTasks.clone(), proc)?;
                parentNodes = metamodelica::arrayGet(taskGraphTIn.clone(), node)?;
                childNodes = metamodelica::arrayGet(taskGraphIn.clone(), node)?;
                sameProcTasks = metamodelica::arrayGet(procAss.clone(), proc)?;
                (_, otherParents, _) = List::intersection1OnTrue(parentNodes, sameProcTasks.clone(), &fnptr!(intEq, i32, i32))?;
                (_, otherChildren, _) = List::intersection1OnTrue(childNodes, sameProcTasks, &fnptr!(intEq, i32, i32))?;
                removeLocks = getSuperfluousLocks(otherParents.clone(), node, taskAss.clone(), orderIn.clone(), numProc, allCalcTasks.clone(), inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone(), removeLocksIn)?;
                taskLstAss = List::map6(otherParents, &createDepTaskByTaskIdc, node, allCalcTasks.clone(), false, inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone())?;
                taskLstRel = List::map6(otherChildren, &createDepTaskByTaskIdcR, node, allCalcTasks.clone(), true, inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone())?;
                components = metamodelica::arrayGet(inComps.clone(), node)?;
                mark = metamodelica::arrayGet(nodeMark.clone(), node)?;
                (_, exeCost) = HpcOmTaskGraph::getExeCost(node, taskGraphMetaIn.clone())?;
                simEqIdc = List::map(List::map1(components, &getSimEqSysIdxForComp, SccSimEqMappingIn.clone())?, &move |__a0: _| List::last(&__a0))?;
                task = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: mark, index: node, calcTime: exeCost, timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: proc, eqIdc: simEqIdc });
                taskLst1 = metamodelica::cons(task, taskLstRel);
                taskLst1 = listAppend(taskLstAss.clone(), taskLst1);
                taskLst1 = listAppend(taskLst, taskLst1);
                threadTasks = metamodelica::arrayUpdate(threadTasks.clone(), proc, taskLst1)?;
                outgoingDepTasks = listAppend(outgoingDepTasks.clone(), taskLstAss);
                schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
                { (taskAss, procAss, orderOpt, taskGraphIn, taskGraphTIn, taskGraphMetaIn, SccSimEqMappingIn, removeLocksIn, orderIn, iSimVarMapping, scheduleIn) = (taskAss.clone(), procAss.clone(), Some(rest), taskGraphIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn, SccSimEqMappingIn.clone(), removeLocks, orderIn, iSimVarMapping.clone(), schedule); continue '__tco; }
            },
            (None, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { .. }) => {
                metamodelica::print(literal!("createSchedulerFromAssignments failed.implement this!\n"));
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn setSimEqIdcsInTask(
    mut taskIn: metamodelica::Ref<HpcOmSimCode::Task>,
    mut SccSimEqMappingIn: metamodelica::Array<metamodelica::List<i32>>,
) -> metamodelica::Ref<HpcOmSimCode::Task> {
    let mut taskOut: metamodelica::Ref<HpcOmSimCode::Task>;
    taskOut = 'mc: {
        let __mc_input = &*taskIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, timeFinished, threadIdx, eqIdc } => {
                    let mut eqIdc = (*eqIdc).clone();
                    eqIdc = List::flatten(List::map1(eqIdc.clone(), &getSimEqSysIdxForComp, SccSimEqMappingIn.clone())?)?;
                    Ok(metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    taskOut
}

fn setThreadIdxInTask(
    mut taskIn: metamodelica::Ref<HpcOmSimCode::Task>,
    mut threadIdx: i32,
) -> metamodelica::Ref<HpcOmSimCode::Task> {
    let mut taskOut: metamodelica::Ref<HpcOmSimCode::Task>;
    taskOut = (match &*taskIn {
        HpcOmSimCode::Task::CALCTASK {
            weighting,
            index,
            calcTime,
            timeFinished,
            eqIdc,
            ..
        } => metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
            weighting: weighting.clone(),
            index: index.clone(),
            calcTime: calcTime.clone(),
            timeFinished: timeFinished.clone(),
            threadIdx: threadIdx,
            eqIdc: eqIdc.clone(),
        }),
        _ => taskIn,
    });
    taskOut
}

fn tasksEqual(
    mut task1: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut task2: &metamodelica::Ref<HpcOmSimCode::Task>,
) -> Result<bool> {
    let mut isEqOut: bool;
    isEqOut = (::match_deref::match_deref! { match (task1, task2) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: id1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { index: id2, .. }) => {
            let mut isEq: bool;
            isEq = intEq(id1.clone(), id2.clone());
            isEq
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: nodeIdc1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: nodeIdc2, .. }) => {
            let mut isEq: bool;
            isEq = List::isEqual(nodeIdc1.clone(), nodeIdc2.clone(), true)?;
            isEq
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask1, targetTask: targetTask1, .. }, Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask2, targetTask: targetTask2, .. }) => {
            let mut isEq: bool;
            isEq = tasksEqual(sourceTask1, sourceTask2)?;
            isEq = boolAnd(isEq, tasksEqual(targetTask1, targetTask2)?);
            isEq
        },
        (Deref @ HpcOmSimCode::Task::TASKEMPTY { .. }, Deref @ HpcOmSimCode::Task::TASKEMPTY { .. }) => {
            false
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqOut)
}

fn removeLocksFromLockList(
    mut lockIdsIn: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut lockTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut lockIdsOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    (_, lockIdsOut, _) = List::intersection1OnTrue(lockIdsIn, lockTasks, &move |__a0: metamodelica::Ref<
        HpcOmSimCode::Task,
    >,
                                                                                __a1: metamodelica::Ref<
        HpcOmSimCode::Task,
    >| tasksEqual(&__a0, &__a1))?;
    Ok(lockIdsOut)
}

fn removeLocksFromThread(
    mut threadIn: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut lockLst: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut threadOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    (_, threadOut, _) = List::intersection1OnTrue(threadIn, lockLst, &move |__a0: metamodelica::Ref<
        HpcOmSimCode::Task,
    >,
                                                                            __a1: metamodelica::Ref<
        HpcOmSimCode::Task,
    >| tasksEqual(&__a0, &__a1))?;
    Ok(threadOut)
}

fn getSuperfluousLocks(
    mut otherParentsIn: metamodelica::List<i32>,
    mut nodeIn: i32,
    mut taskAssIn: metamodelica::Array<i32>,
    mut orderIn: metamodelica::List<i32>,
    mut numProc: i32,
    mut iAllCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<HpcOmTaskGraph::Communication>>,
    mut iCompTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSimVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut removeLocksIn: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut removeLocksOut: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut parentsOnThreads: metamodelica::Array<metamodelica::List<i32>>;
    let mut otherParentsProcs: metamodelica::List<i32>;
    let mut lockCandidatesFlat: metamodelica::List<i32>;
    let mut lockCandidates: metamodelica::List<metamodelica::List<i32>>;
    let mut removeLocks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut taskLstAss: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut taskLstRel: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    otherParentsProcs = List::map1(otherParentsIn.clone(), &Array::getIndexFirst, taskAssIn.clone())?;
    parentsOnThreads = arrayCreate(numProc, metamodelica::nil());
    parentsOnThreads = List::fold1(
        &(List::intRange(((otherParentsProcs).len() as i32))),
        &move |__a0: i32, __a1: metamodelica::List<i32>, __a2: metamodelica::Array<metamodelica::List<i32>>| {
            listIndecesForValues(__a0, &__a1, __a2)
        },
        otherParentsProcs,
        parentsOnThreads.clone(),
    )?;
    parentsOnThreads = Array::map1(parentsOnThreads.clone(), &mapListGet, otherParentsIn)?;
    lockCandidates = List::filterOnTrue(
        parentsOnThreads
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
        (std::sync::Arc::new(move |__a0: metamodelica::List<i32>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(lengthNotOne(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<bool> + 'static>),
    )?;
    lockCandidates = List::map1(lockCandidates, &removeLatestTaskFromList, orderIn)?;
    lockCandidatesFlat = List::flatten(lockCandidates)?;
    taskLstAss = List::map6(
        lockCandidatesFlat.clone(),
        &createDepTaskByTaskIdc,
        nodeIn,
        iAllCalcTasks.clone(),
        false,
        iCommCosts.clone(),
        iCompTaskMapping.clone(),
        iSimVarMapping.clone(),
    )?;
    taskLstRel = List::map6(
        lockCandidatesFlat,
        &createDepTaskByTaskIdc,
        nodeIn,
        iAllCalcTasks.clone(),
        true,
        iCommCosts.clone(),
        iCompTaskMapping.clone(),
        iSimVarMapping.clone(),
    )?;
    removeLocks = listAppend(removeLocksIn, taskLstAss);
    removeLocksOut = listAppend(removeLocks, taskLstRel);
    Ok(removeLocksOut)
}

fn removeLatestTaskFromList(
    mut taskLstIn: metamodelica::List<i32>,
    mut taskOrderIn: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut taskLstOut: metamodelica::List<i32>;
    taskLstOut = (::match_deref::match_deref! { match &(taskLstIn.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            taskLstIn
        },
        _ => {
            let mut posInOrder: metamodelica::List<i32>;
            let mut taskLst: metamodelica::List<i32>;
            let mut latestTask: i32;
            posInOrder = List::map1(taskLstIn.clone(), &move |__a0: _, __a1: _| List::position(__a0, &__a1), taskOrderIn.clone())?;
            posInOrder = List::map1(posInOrder, &fnptr!(intSub, i32, i32), 1)?;
            latestTask = List::fold(&posInOrder, &fnptr!(intMax, i32, i32), -1)?;
            latestTask = (taskOrderIn).get(latestTask + 1)?;
            taskLst = List::removeOnTrue(latestTask, &fnptr!(intEq, i32, i32), taskLstIn)?;
            taskLst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(taskLstOut)
}

fn lengthNotOne(mut lstIn: &metamodelica::List<i32>) -> bool {
    let mut b: bool;
    b = intNe(((lstIn).len() as i32), 1);
    b
}

fn mapListGet(
    mut mapLstIn: metamodelica::List<i32>,
    mut argLst: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut mapLstOut: metamodelica::List<i32>;
    mapLstOut = List::map1(
        mapLstIn,
        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
        argLst,
    )?;
    Ok(mapLstOut)
}

fn listIndecesForValues(
    mut idx: i32,
    mut lstIn: &metamodelica::List<i32>,
    mut arrayIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut arrayOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut value: i32;
    let mut valueLst: metamodelica::List<i32>;
    value = (lstIn).get(idx)?;
    valueLst = metamodelica::arrayGet(arrayIn.clone(), value)?;
    valueLst = metamodelica::cons(idx, valueLst);
    arrayOut = metamodelica::arrayUpdate(arrayIn.clone(), value, valueLst)?;
    Ok(arrayOut)
}

//---------------------------
// quicksort with order
//---------------------------
pub(crate) fn quicksortWithOrder(
    mut lstIn: metamodelica::List<metamodelica::Real>,
) -> Result<(metamodelica::List<metamodelica::Real>, metamodelica::List<i32>)> {
    let mut lstOut: metamodelica::List<metamodelica::Real>;
    let mut orderOut: metamodelica::List<i32>;
    (lstOut, orderOut) = 'mc: {
        let __mc_input = &*lstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut length: i32;
                    let mut pivotIdx: i32;
                    let mut r1: metamodelica::Real;
                    let mut r2: metamodelica::Real;
                    let mut r3: metamodelica::Real;
                    let mut pivotValue: metamodelica::Real;
                    let mut orderTmp: metamodelica::List<i32>;
                    let mut lstTmp: metamodelica::List<metamodelica::Real>;
                    length = ((lstIn).len() as i32);
                    orderTmp = List::intRange(length);
                    r1 = (lstIn).head().cloned()?;
                    r2 = List::last(&lstIn)?;
                    r3 = (lstIn).get(intDiv(length, 2))?;
                    (pivotValue, _) = getMedian3(r1, r2, r3)?;
                    pivotIdx = List::position(pivotValue, &lstIn)?;
                    (lstTmp, orderTmp) = quicksortWithOrder1(lstIn.clone(), orderTmp.clone(), pivotIdx, lstIn.clone(), length)?;
                    Ok((lstTmp.clone(), orderTmp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r1, tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok((list![r1.clone()], list![1]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((lstOut, orderOut))
}

fn quicksortWithOrder1(
    mut lstIn: metamodelica::List<metamodelica::Real>,
    mut orderIn: metamodelica::List<i32>,
    mut pivotIdx: i32,
    mut markedIn: metamodelica::List<metamodelica::Real>,
    mut size: i32,
) -> Result<(metamodelica::List<metamodelica::Real>, metamodelica::List<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((lstIn.clone(), markedIn.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((metamodelica::nil(), metamodelica::nil()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
                return Ok((list![e.clone()], list![1]))
            },
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((lstIn, orderIn))
            },
            _ => {
                let mut b1: bool;
                let mut b2: bool;
                let mut lIdx: i32;
                let mut rIdx: i32;
                let mut pivot: i32;
                let mut p: metamodelica::Real;
                let mut orderTmp: metamodelica::List<i32>;
                let mut marked: metamodelica::List<metamodelica::Real>;
                let mut lstTmp: metamodelica::List<metamodelica::Real>;
                let mut leftLst: metamodelica::List<metamodelica::Real>;
                let mut rightLst: metamodelica::List<metamodelica::Real>;
                p = (lstIn).get(pivotIdx)?;
                (leftLst, rightLst) = List::split(lstIn.clone(), pivotIdx)?;
                rightLst = rightLst.reverse();
                (_, lIdx, b1) = getMemberOnTrueWithIdx(p, &leftLst, &fnptr!(realLt, metamodelica::Real, metamodelica::Real));
                (_, rIdx, b2) = getMemberOnTrueWithIdx(p, &rightLst, &fnptr!(realGt, metamodelica::Real, metamodelica::Real));
                rIdx = size + 1 - rIdx;
                lstTmp = if (b1) {swapEntriesInList(pivotIdx, lIdx, lstIn)?} else {lstIn};
                lstTmp = if (b2) {swapEntriesInList(pivotIdx, rIdx, lstTmp)?} else {lstTmp};
                orderTmp = if (b1) {swapEntriesInList(pivotIdx, lIdx, orderIn)?} else {orderIn};
                orderTmp = if (b2) {swapEntriesInList(pivotIdx, rIdx, orderTmp)?} else {orderTmp};
                if !(b1) && !(b2) {
                    (marked, pivot) = getNextPivot(&lstTmp, markedIn, pivotIdx)?;
                } else {
                    marked = markedIn;
                    pivot = pivotIdx;
                }
                { (lstIn, orderIn, pivotIdx, markedIn, size) = (lstTmp, orderTmp, pivot, marked, size); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getNextPivot(
    mut lstIn: &metamodelica::List<metamodelica::Real>,
    mut markedLstIn: metamodelica::List<metamodelica::Real>,
    mut pivotIdx: i32,
) -> Result<(metamodelica::List<metamodelica::Real>, i32)> {
    let mut marked: metamodelica::List<metamodelica::Real>;
    let mut newIdx: i32;
    (marked, newIdx) = (::match_deref::match_deref! { match &(markedLstIn.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
            (metamodelica::nil(), 0)
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
            let mut midIdx: i32;
            let mut pivotElement: metamodelica::Real;
            let mut r1: metamodelica::Real;
            let mut r2: metamodelica::Real;
            let mut r3: metamodelica::Real;
            pivotElement = (lstIn).get(pivotIdx)?;
            (marked, _) = List::deleteMemberOnTrue(pivotElement, markedLstIn, &fnptr!(realEq, metamodelica::Real, metamodelica::Real))?;
            r1 = (marked).head().cloned()?;
            r2 = List::last(&marked)?;
            midIdx = intDiv(((marked).len() as i32), 2);
            midIdx = if (intEq(midIdx, 0)) {1} else {midIdx};
            r3 = (marked).get(midIdx)?;
            (pivotElement, _) = getMedian3(r1, r2, r3)?;
            newIdx = List::position(pivotElement, lstIn)?;
            (marked, newIdx)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((marked, newIdx))
}

fn getMemberOnTrueWithIdx(
    mut inValue: metamodelica::Real,
    mut inList: &metamodelica::List<metamodelica::Real>,
    mut inCompFunc: &dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool>,
) -> (metamodelica::Real, i32, bool) {
    pub type CompFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>;

    let mut outElement: metamodelica::Real;
    let mut outIdx: i32;
    let mut found: bool;
    (outElement, outIdx, found) = getMemberOnTrueWithIdx1(1, inValue, inList, inCompFunc);
    (outElement, outIdx, found)
}

fn getMemberOnTrueWithIdx1(
    mut inIdx: i32,
    mut inValue: metamodelica::Real,
    mut inList: &metamodelica::List<metamodelica::Real>,
    mut inCompFunc: &dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool>,
) -> (metamodelica::Real, i32, bool) {
    pub type CompFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>;

    let mut outElement: metamodelica::Real;
    let mut outIdx: i32;
    let mut found: bool;
    (outElement, outIdx, found) = 'mc: {
        let __mc_input = &**inList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::OrderedFloat(0.0_f64), 0, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: _ } => {
                    let mut b: bool;
                    b = inCompFunc(inValue, e.clone())?;
                    let true = (b) else { return Err("pattern mismatch") };
                    Ok((e.clone(), inIdx, b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut value: metamodelica::Real;
                    let mut idx: i32;
                    let mut b: bool;
                    (value, idx, b) = getMemberOnTrueWithIdx1(inIdx + 1, inValue, metamodelica::AsArg::as_arg(&rest), inCompFunc);
                    Ok((value, idx, b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outElement, outIdx, found)
}

fn swapEntriesInList<ElementType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut idx1: i32,
    mut idx2: i32,
    mut lstIn: metamodelica::List<ElementType>,
) -> Result<metamodelica::List<ElementType>> {
    let mut lstOut: metamodelica::List<ElementType>;
    let mut r1: ElementType;
    let mut r2: ElementType;
    let mut lstTmp: metamodelica::List<ElementType>;
    r1 = (lstIn).get(idx1)?;
    r2 = (lstIn).get(idx2)?;
    lstTmp = List::replaceAt(r1, idx2, lstIn)?;
    lstOut = List::replaceAt(r2, idx1, lstTmp)?;
    Ok(lstOut)
}

fn getMedian3(
    mut r1: metamodelica::Real,
    mut r2: metamodelica::Real,
    mut r3: metamodelica::Real,
) -> Result<(metamodelica::Real, i32)> {
    let mut rOut: metamodelica::Real;
    let mut which: i32;
    let mut r: metamodelica::List<metamodelica::Real>;
    r = List::sort(
        list![r1, r2, r3],
        (std::sync::Arc::new(fnptr!(realGt, metamodelica::Real, metamodelica::Real))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>),
    )?;
    rOut = (r).get(2)?;
    which = List::position(rOut, &(list![r1, r2, r3]))?;
    Ok((rOut, which))
}

//----------------------------
// traverse the task graph bottoms up (beginning at the root nodes)
//----------------------------
fn computeGraphValuesBottomUp(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
)> {
    let mut asapOut: metamodelica::Array<metamodelica::Real>;
    let mut estOut: metamodelica::Array<metamodelica::Real>;
    let mut ectOut: metamodelica::Array<metamodelica::Real>;
    let mut size: i32;
    let mut rootNodes: metamodelica::List<i32>;
    let mut asap: metamodelica::Array<metamodelica::Real>;
    let mut ect: metamodelica::Array<metamodelica::Real>;
    let mut est: metamodelica::Array<metamodelica::Real>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    asap = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    est = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    ect = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    (asapOut, estOut, ectOut) = computeGraphValuesBottomUp1(
        rootNodes,
        iTaskGraph.clone(),
        taskGraphT.clone(),
        iTaskGraphMeta,
        asap.clone(),
        est.clone(),
        ect.clone(),
    )?;
    Ok((asapOut, estOut, ectOut))
}

fn computeGraphValuesBottomUp1<'__b>(
    mut parentsIn: metamodelica::List<i32>,
    mut graph: metamodelica::Array<metamodelica::List<i32>>,
    mut graphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &'__b HpcOmTaskGraph::TaskGraphMeta,
    mut asapIn: metamodelica::Array<metamodelica::Real>,
    mut estIn: metamodelica::Array<metamodelica::Real>,
    mut ectIn: metamodelica::Array<metamodelica::Real>,
) -> Result<(
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((parentsIn, asapIn.clone(), estIn.clone(), ectIn.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: node, tail: rest }, asap, est, ect) => {
                let mut children: metamodelica::List<i32>;
                let mut asap = (*asap).clone();
                let mut est = (*est).clone();
                let mut ect = (*ect).clone();
                (asap, est, ect, children) = computeGraphValuesBottomUp2(node.clone(), graph.clone(), graphT.clone(), iTaskGraphMeta.clone(), asap.clone(), est.clone(), ect.clone())?;
                { (parentsIn, graph, graphT, iTaskGraphMeta, asapIn, estIn, ectIn) = (listAppend(rest.clone(), children), graph.clone(), graphT.clone(), iTaskGraphMeta, asap.clone(), est.clone(), ect.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                return Ok((asapIn.clone(), estIn.clone(), ectIn.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn computeGraphValuesBottomUp2(
    mut node: i32,
    mut graph: metamodelica::Array<metamodelica::List<i32>>,
    mut graphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut asapIn: metamodelica::Array<metamodelica::Real>,
    mut estIn: metamodelica::Array<metamodelica::Real>,
    mut ectIn: metamodelica::Array<metamodelica::Real>,
) -> Result<(
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
    metamodelica::List<i32>,
)> {
    let mut asapOut: metamodelica::Array<metamodelica::Real>;
    let mut estOut: metamodelica::Array<metamodelica::Real>;
    let mut ectOut: metamodelica::Array<metamodelica::Real>;
    let mut children: metamodelica::List<i32> = metamodelica::nil();
    (asapOut, estOut, ectOut, children) = 'mc: {
        let __mc_input = ectIn.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut maxASAP: metamodelica::Real;
            let mut maxEct: metamodelica::Real;
            let mut exeCost: metamodelica::Real;
            let mut asap: metamodelica::Array<metamodelica::Real>;
            let mut ect: metamodelica::Array<metamodelica::Real>;
            let mut est: metamodelica::Array<metamodelica::Real>;
            let mut parents: metamodelica::List<i32>;
            let mut parentEcts: metamodelica::List<metamodelica::Real>;
            let mut parentAsaps: metamodelica::List<metamodelica::Real>;
            let mut parentAsaps2: metamodelica::List<metamodelica::Real>;
            let mut parentsExeCosts: metamodelica::List<metamodelica::Real>;
            let mut commCosts: metamodelica::List<metamodelica::Real>;
            let mut children: metamodelica::List<i32> = children.clone();
            parents = metamodelica::arrayGet(graphT.clone(), node)?;
            parentAsaps = List::map1(parents.clone(), &Array::getIndexFirst, asapIn.clone())?;
            let false = (List::isMemberOnTrue(
                metamodelica::OrderedFloat(-1.0_f64),
                &parentAsaps,
                &fnptr!(realEq, metamodelica::Real, metamodelica::Real),
            )?) else {
                return Err("pattern mismatch");
            };
            exeCost = HpcOmTaskGraph::getExeCostReqCycles(node, iTaskGraphMeta.clone())?;
            parentsExeCosts = List::map1(
                parents.clone(),
                &HpcOmTaskGraph::getExeCostReqCycles,
                iTaskGraphMeta.clone(),
            )?;
            commCosts = List::map2(
                parents.clone(),
                &HpcOmTaskGraph::getCommCostTimeBetweenNodes,
                node,
                iTaskGraphMeta.clone(),
            )?;
            parentAsaps2 = List::threadMap(
                parentAsaps.clone(),
                parentsExeCosts.clone(),
                &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
            )?;
            parentAsaps2 = List::threadMap(
                parentAsaps2.clone(),
                commCosts.clone(),
                &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
            )?;
            maxASAP = List::fold(
                &parentAsaps2,
                &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            asap = metamodelica::arrayUpdate(asapIn.clone(), node, maxASAP)?;
            parentEcts = List::map1(parents.clone(), &Array::getIndexFirst, ectIn.clone())?;
            maxEct = List::fold(
                &parentEcts,
                &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            est = metamodelica::arrayUpdate(estIn.clone(), node, maxEct)?;
            ect = metamodelica::arrayUpdate(ectIn.clone(), node, (maxEct) + (exeCost))?;
            children = metamodelica::arrayGet(graph.clone(), node)?;
            Ok((
                (asap.clone(), est.clone(), ect.clone(), children.clone()),
                children.clone(),
            ))
        })() {
            children = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut parents: metamodelica::List<i32>;
            let mut parentAsaps: metamodelica::List<metamodelica::Real>;
            parents = metamodelica::arrayGet(graphT.clone(), node)?;
            parentAsaps = List::map1(parents.clone(), &Array::getIndexFirst, asapIn.clone())?;
            let true = (List::isMemberOnTrue(
                metamodelica::OrderedFloat(-1.0_f64),
                &parentAsaps,
                &fnptr!(realEq, metamodelica::Real, metamodelica::Real),
            )?) else {
                return Err("pattern mismatch");
            };
            Ok((asapIn.clone(), estIn.clone(), ectIn.clone(), list![node]))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("computeGraphValuesBottomUp2 failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((asapOut, estOut, ectOut, children))
}

//----------------------------
// traverse the task graph top down (beginning at the leaf nodes)
//----------------------------
fn computeGraphValuesTopDown(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
    metamodelica::Array<metamodelica::Real>,
)> {
    let mut alapOut: metamodelica::Array<metamodelica::Real>;
    let mut lastOut: metamodelica::Array<metamodelica::Real>;
    let mut lactOut: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevelOut: metamodelica::Array<metamodelica::Real>;
    let mut size: i32;
    let mut lastNodeInCP: i32;
    let mut cp: metamodelica::Real;
    let mut cpWithComm: metamodelica::Real;
    let mut endNodes: metamodelica::List<i32>;
    let mut alap: metamodelica::Array<metamodelica::Real>;
    let mut lact: metamodelica::Array<metamodelica::Real>;
    let mut last: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevel: metamodelica::Array<metamodelica::Real>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut visitedNodes: metamodelica::Array<bool>;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    endNodes = HpcOmTaskGraph::getLeafNodes(iTaskGraph.clone())?;
    alap = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    last = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    lact = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    tdsLevel = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    visitedNodes = arrayCreate(size, false);
    computeGraphValuesTopDown1(
        endNodes,
        iTaskGraph.clone(),
        taskGraphT.clone(),
        iTaskGraphMeta,
        alap.clone(),
        last.clone(),
        lact.clone(),
        tdsLevel.clone(),
        visitedNodes.clone(),
    )?;
    cpWithComm = Array::fold(
        alap.clone(),
        &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    lastNodeInCP = Array::position(alap.clone(), cpWithComm, size)?;
    cp = Array::fold(
        last.clone(),
        &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    alapOut = Array::map1(
        alap.clone(),
        &fnptr!(realSubr, metamodelica::Real, metamodelica::Real),
        cpWithComm,
    )?;
    lactOut = Array::map1(
        lact.clone(),
        &fnptr!(realSubr, metamodelica::Real, metamodelica::Real),
        cp,
    )?;
    lastOut = Array::map1(
        last.clone(),
        &fnptr!(realSubr, metamodelica::Real, metamodelica::Real),
        cp,
    )?;
    tdsLevelOut = tdsLevel.clone();
    Ok((alapOut, lastOut, lactOut, tdsLevelOut))
}

fn computeGraphValuesTopDown1(
    mut nodesIn: metamodelica::List<i32>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut alapIn: metamodelica::Array<metamodelica::Real>,
    mut lastIn: metamodelica::Array<metamodelica::Real>,
    mut lactIn: metamodelica::Array<metamodelica::Real>,
    mut tdsLevelIn: metamodelica::Array<metamodelica::Real>,
    mut visitedNodes: metamodelica::Array<bool>,
) -> Result<()> {
    let mut nodes: metamodelica::List<i32> = nodesIn;
    let mut alap: metamodelica::Array<metamodelica::Real> = alapIn;
    let mut last: metamodelica::Array<metamodelica::Real> = lastIn;
    let mut lact: metamodelica::Array<metamodelica::Real> = lactIn;
    let mut tdsLevel: metamodelica::Array<metamodelica::Real> = tdsLevelIn;
    while !((nodes).is_empty()) {
        if metamodelica::arrayGet(visitedNodes.clone(), (nodes).head().cloned()?)? {
            nodes = (nodes).rest()?;
        } else {
            nodes = computeGraphValuesTopDown2(
                &nodes,
                iTaskGraph.clone(),
                iTaskGraphT.clone(),
                iTaskGraphMeta.clone(),
                alap.clone(),
                last.clone(),
                lact.clone(),
                tdsLevel.clone(),
                visitedNodes.clone(),
            )?;
        }
    }
    Ok(())
}

fn computeGraphValuesTopDown2(
    mut nodesIn: &metamodelica::List<i32>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut alapIn: metamodelica::Array<metamodelica::Real>,
    mut lastIn: metamodelica::Array<metamodelica::Real>,
    mut lactIn: metamodelica::Array<metamodelica::Real>,
    mut tdsLevelIn: metamodelica::Array<metamodelica::Real>,
    mut visitedNodes: metamodelica::Array<bool>,
) -> Result<metamodelica::List<i32>> {
    let mut nodesOut: metamodelica::List<i32>;
    let mut nodeIdx: i32;
    let mut nodeExeCost: metamodelica::Real;
    let mut maxLevel: metamodelica::Real;
    let mut maxAlap: metamodelica::Real;
    let mut maxLast: metamodelica::Real;
    let mut rest: metamodelica::List<i32>;
    let mut parentNodes: metamodelica::List<i32>;
    let mut childNodes: metamodelica::List<i32>;
    let mut childTDSLevels: metamodelica::List<metamodelica::Real>;
    let mut childAlaps: metamodelica::List<metamodelica::Real>;
    let mut childLasts: metamodelica::List<metamodelica::Real>;
    let mut childLacts: metamodelica::List<metamodelica::Real>;
    let mut commCostsToChilds: metamodelica::List<metamodelica::Real>;
    let mut alap: metamodelica::Array<metamodelica::Real>;
    let mut last: metamodelica::Array<metamodelica::Real>;
    let mut lact: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevel: metamodelica::Array<metamodelica::Real>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*nodesIn)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    nodeIdx = metamodelica::Own::own(__pa0);
    rest = metamodelica::Own::own(__pa1);
    childNodes = metamodelica::arrayGet(iTaskGraph.clone(), nodeIdx)?;
    nodeExeCost = HpcOmTaskGraph::getExeCostReqCycles(nodeIdx, iTaskGraphMeta.clone())?;
    metamodelica::arrayUpdate(visitedNodes.clone(), nodeIdx, true)?;
    if (childNodes).is_empty() {
        alap = metamodelica::arrayUpdate(alapIn.clone(), nodeIdx, nodeExeCost)?;
        last = metamodelica::arrayUpdate(lastIn.clone(), nodeIdx, nodeExeCost)?;
        lact = metamodelica::arrayUpdate(lactIn.clone(), nodeIdx, metamodelica::OrderedFloat(0.0_f64))?;
        tdsLevel = metamodelica::arrayUpdate(tdsLevelIn.clone(), nodeIdx, nodeExeCost)?;
        parentNodes = metamodelica::arrayGet(iTaskGraphT.clone(), nodeIdx)?;
        nodesOut = listAppend(rest, parentNodes);
    } else {
        childTDSLevels = List::map1(childNodes.clone(), &Array::getIndexFirst, tdsLevelIn.clone())?;
        if List::isMemberOnTrue(
            metamodelica::OrderedFloat(-1.0_f64),
            &childTDSLevels,
            &fnptr!(realEq, metamodelica::Real, metamodelica::Real),
        )? {
            nodesOut = listAppend(rest, list![nodeIdx]);
            metamodelica::arrayUpdate(visitedNodes.clone(), nodeIdx, false)?;
        } else {
            commCostsToChilds = ({
                let mut __acc: metamodelica::List<metamodelica::Real> = metamodelica::nil();
                for mut n in (childNodes.clone()).into_iter().cloned() {
                    let __x = HpcOmTaskGraph::getCommCostTimeBetweenNodes(nodeIdx, n.clone(), iTaskGraphMeta.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            childAlaps = List::map1(childNodes.clone(), &Array::getIndexFirst, alapIn.clone())?;
            childAlaps = List::threadMap(
                childAlaps,
                commCostsToChilds,
                &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
            )?;
            childLasts = List::map1(childNodes.clone(), &Array::getIndexFirst, lastIn.clone())?;
            childLacts = List::map1(childNodes, &Array::getIndexFirst, lactIn.clone())?;
            maxLevel = List::fold(
                &childTDSLevels,
                &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            maxAlap = List::fold(
                &childAlaps,
                &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            maxLast = List::fold(
                &childLasts,
                &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            tdsLevel = metamodelica::arrayUpdate(tdsLevelIn.clone(), nodeIdx, nodeExeCost + maxLevel)?;
            alap = metamodelica::arrayUpdate(alapIn.clone(), nodeIdx, nodeExeCost + maxAlap)?;
            last = metamodelica::arrayUpdate(lastIn.clone(), nodeIdx, nodeExeCost + maxLast)?;
            lact = metamodelica::arrayUpdate(lactIn.clone(), nodeIdx, maxLast)?;
            parentNodes = metamodelica::arrayGet(iTaskGraphT.clone(), nodeIdx)?;
            nodesOut = listAppend(rest, parentNodes);
        }
    }
    Ok(nodesOut)
}

fn realSubr(mut r1: metamodelica::Real, mut r2: metamodelica::Real) -> metamodelica::Real {
    let mut r3: metamodelica::Real;
    r3 = (r2) - (r1);
    r3
}

//-----
// Util
//-----
pub(crate) fn printSchedule(mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>) -> Result<()> {
    metamodelica::print(dumpSchedule(iSchedule)?);
    Ok(())
}

fn dumpSchedule(mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: ArcStr;
    let mut sLst: metamodelica::List<ArcStr>;
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut allTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tasksOfLevels: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut taskDepTasks: metamodelica::List<(metamodelica::Ref<HpcOmSimCode::Task>, metamodelica::List<i32>)>;
    r#str = (match &**iSchedule {
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks: __esc_threadTasks,
            outgoingDepTasks: __esc_outgoingDepTasks,
            ..
        } => {
            threadTasks = (*__esc_threadTasks).clone();
            outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
            (sLst, _) = List::mapFold(
                &(threadTasks
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>()),
                &dumpThreadSchedule,
                1,
            )?;
            s = stringDelimitList(sLst, literal!("\n"));
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("\nDependency tasks: {\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(outgoingDepTasks.clone(), &move |__a0: metamodelica::Ref<
                        HpcOmSimCode::Task,
                    >| dumpTask(&__a0))?,
                    literal!(""),
                ));
                __mm_s.push_str(&*literal!("}\n"));
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("THREADSCHEDULE\n"));
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s
        }
        HpcOmSimCode::Schedule::LEVELSCHEDULE {
            tasksOfLevels: __esc_tasksOfLevels,
            ..
        } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            (sLst, _) = List::mapFold(
                metamodelica::AsArg::as_arg(&tasksOfLevels),
                &move |__a0: HpcOmSimCode::TaskList, __a1: i32| dumpLevelSchedule(&__a0, __a1),
                1,
            )?;
            s = stringDelimitList(sLst, literal!("\n"));
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("LEVELSCHEDULE\n"));
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s
        }
        HpcOmSimCode::Schedule::TASKDEPSCHEDULE {
            tasks: __esc_taskDepTasks,
        } => {
            taskDepTasks = (*__esc_taskDepTasks).clone();
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    List::map(taskDepTasks.clone(), &move |__a0: (
                        metamodelica::Ref<HpcOmSimCode::Task>,
                        metamodelica::List<i32>,
                    )| dumpTaskDepSchedule(&__a0))?,
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("TASKDEPSCHEDULE\n"));
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s
        }
        HpcOmSimCode::Schedule::EMPTYSCHEDULE {
            tasks: HpcOmSimCode::TaskList::SERIALTASKLIST {
                tasks: __esc_allTasks, ..
            },
        } => {
            allTasks = (*__esc_allTasks).clone();
            (s, _) = dumpThreadSchedule(allTasks.clone(), 1)?;
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("EMPTYSCHEDULE\n"));
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s
        }
        _ => return Err("fail"),
    });
    Ok(r#str)
}

pub(crate) fn analyseScheduledTaskGraph(
    mut scheduleIn: metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut numProcIn: i32,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut inSystemName: &ArcStr,
) -> ArcStr {
    let mut criticalPathInfoOut: ArcStr;
    criticalPathInfoOut = 'mc: {
        let __mc_input = &*scheduleIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: _ } => {
                    let mut criticalPaths: metamodelica::List<metamodelica::List<i32>>;
                    let mut criticalPathsWoC: metamodelica::List<metamodelica::List<i32>>;
                    let mut cpCosts: metamodelica::Real;
                    let mut cpCostsWoC: metamodelica::Real;
                    let mut criticalPathInfo: ArcStr;
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone());
                    criticalPaths = metamodelica::Own::own(__pa0);
                    cpCosts = metamodelica::Own::own(__pa1);
                    criticalPathsWoC = metamodelica::Own::own(__pa2);
                    cpCostsWoC = metamodelica::Own::own(__pa3);
                    criticalPathInfo = HpcOmTaskGraph::dumpCriticalPathInfo(&((criticalPaths.clone(), cpCosts)), &((criticalPathsWoC.clone(), cpCostsWoC)))?;
                    Ok(criticalPathInfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels, useFixedAssignments: false } => {
                    let mut criticalPathInfo: ArcStr;
                    criticalPathInfo = analyseScheduledTaskGraphLevel(tasksOfLevels.clone(), numProcIn, taskGraphIn.clone(), taskGraphMetaIn.clone(), (std::sync::Arc::new(move |__a0: HpcOmSimCode::TaskList, __a1: metamodelica::Array<metamodelica::List<i32>>, __a2: HpcOmTaskGraph::TaskGraphMeta, __a3: i32| getLevelParallelTime(&__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<metamodelica::List<i32>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real> + 'static>))?;
                    Ok(criticalPathInfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels, useFixedAssignments: true } => {
                    let mut criticalPathInfo: ArcStr;
                    criticalPathInfo = analyseScheduledTaskGraphLevel(tasksOfLevels.clone(), numProcIn, taskGraphIn.clone(), taskGraphMetaIn.clone(), (std::sync::Arc::new(move |__a0: HpcOmSimCode::TaskList, __a1: metamodelica::Array<metamodelica::List<i32>>, __a2: HpcOmTaskGraph::TaskGraphMeta, __a3: i32| getLevelParallelTime(&__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<metamodelica::List<i32>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real> + 'static>))?;
                    Ok(criticalPathInfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks, .. } => {
                    let mut criticalPaths: metamodelica::List<metamodelica::List<i32>>;
                    let mut criticalPathsWoC: metamodelica::List<metamodelica::List<i32>>;
                    let mut cpCosts: metamodelica::Real;
                    let mut cpCostsWoC: metamodelica::Real;
                    let mut serTime: metamodelica::Real;
                    let mut parTime: metamodelica::Real;
                    let mut speedUp: metamodelica::Real;
                    let mut speedUpMax: metamodelica::Real;
                    let mut criticalPathInfo: ArcStr;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the number of locks: ")); __mm_s.push_str(&*intString(((outgoingDepTasks).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone());
                    criticalPaths = metamodelica::Own::own(__pa0);
                    cpCosts = metamodelica::Own::own(__pa1);
                    criticalPathsWoC = metamodelica::Own::own(__pa2);
                    cpCostsWoC = metamodelica::Own::own(__pa3);
                    criticalPathInfo = HpcOmTaskGraph::dumpCriticalPathInfo(&((criticalPaths.clone(), cpCosts)), &((criticalPathsWoC.clone(), cpCostsWoC)))?;
                    (serTime, parTime, speedUp, speedUpMax) = predictExecutionTime(scheduleIn.clone(), Some(cpCostsWoC), numProcIn, taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    serTime = HpcOmTaskGraph::roundReal(serTime, 2)?;
                    parTime = HpcOmTaskGraph::roundReal(parTime, 2)?;
                    cpCostsWoC = HpcOmTaskGraph::roundReal(cpCostsWoC, 2)?;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the serialCosts: ")); __mm_s.push_str(&*realString(serTime)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the parallelCosts: ")); __mm_s.push_str(&*realString(parTime)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the cpCosts: ")); __mm_s.push_str(&*realString(cpCostsWoC)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    if realLe(speedUpMax, metamodelica::OrderedFloat(2.0_f64)) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("There is no parallel potential in the ")); __mm_s.push_str(&*inSystemName); __mm_s.push_str(&*literal!(" model!\n")); ArcStr::from(__mm_s) });
                    }
                    if realLe(serTime, metamodelica::OrderedFloat(20000.0_f64)) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The ")); __mm_s.push_str(&*inSystemName); __mm_s.push_str(&*literal!(" model is not big enough to perform an effective parallel simulation!\n")); ArcStr::from(__mm_s) });
                    }
                    printPredictedExeTimeInfo(serTime, parTime, speedUp, speedUpMax, numProcIn)?;
                    Ok(criticalPathInfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { .. } => {
                    let mut criticalPaths: metamodelica::List<metamodelica::List<i32>>;
                    let mut criticalPathsWoC: metamodelica::List<metamodelica::List<i32>>;
                    let mut cpCosts: metamodelica::Real;
                    let mut cpCostsWoC: metamodelica::Real;
                    let mut criticalPathInfo: ArcStr;
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone());
                    criticalPaths = metamodelica::Own::own(__pa0);
                    cpCosts = metamodelica::Own::own(__pa1);
                    criticalPathsWoC = metamodelica::Own::own(__pa2);
                    cpCostsWoC = metamodelica::Own::own(__pa3);
                    criticalPathInfo = HpcOmTaskGraph::dumpCriticalPathInfo(&((criticalPaths.clone(), cpCosts)), &((criticalPathsWoC.clone(), cpCostsWoC)))?;
                    Ok(criticalPathInfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("HpcOmScheduler.analyseScheduledTaskGraph failed\n"));
                    Ok(literal!("HpcOmScheduler.analyseScheduledTaskGraph failed\n"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    criticalPathInfoOut
}

fn analyseScheduledTaskGraphLevel(
    mut iLevelTasks: metamodelica::List<HpcOmSimCode::TaskList>,
    mut iNumProc: i32,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iParallelSectionCalculator: Arc<
        dyn ::std::ops::Fn(
                HpcOmSimCode::TaskList,
                metamodelica::Array<metamodelica::List<i32>>,
                HpcOmTaskGraph::TaskGraphMeta,
                i32,
            ) -> Result<metamodelica::Real>
            + 'static,
    >,
) -> Result<ArcStr> {
    pub type LevelParallelSectionFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                HpcOmSimCode::TaskList,
                metamodelica::Array<metamodelica::List<i32>>,
                HpcOmTaskGraph::TaskGraphMeta,
                i32,
            ) -> Result<metamodelica::Real>
            + 'static,
    >;

    let mut oCriticalPathInfo: ArcStr;
    let mut i: i32;
    let mut costShare: i32;
    let mut levelCosts: metamodelica::List<metamodelica::Real>;
    let mut criticalPaths: metamodelica::List<metamodelica::List<i32>>;
    let mut criticalPathsWoC: metamodelica::List<metamodelica::List<i32>>;
    let mut levelSectionCosts: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut cpCosts: metamodelica::Real;
    let mut cpCostsWoC: metamodelica::Real;
    let mut serTime: metamodelica::Real;
    let mut parTime: metamodelica::Real;
    let mut speedUp: metamodelica::Real;
    let mut speedUpMax: metamodelica::Real;
    let mut levelCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(iTaskGraph.clone(), iTaskGraphMeta.clone());
    criticalPaths = metamodelica::Own::own(__pa0);
    cpCosts = metamodelica::Own::own(__pa1);
    criticalPathsWoC = metamodelica::Own::own(__pa2);
    cpCostsWoC = metamodelica::Own::own(__pa3);
    levelSectionCosts = List::map1(
        iLevelTasks.clone(),
        &move |__a0: HpcOmSimCode::TaskList, __a1: HpcOmTaskGraph::TaskGraphMeta| getLevelListTaskCosts(&__a0, __a1),
        iTaskGraphMeta.clone(),
    )?;
    serTime = realSum(
        &(List::map(levelSectionCosts, &move |__a0: metamodelica::List<
            metamodelica::Real,
        >| realSum(&__a0))?),
    )?;
    serTime = HpcOmTaskGraph::roundReal(serTime, 2)?;
    levelCosts = List::map(
        iLevelTasks,
        &({
            let __pe_b1 = iTaskGraph.clone();
            let __pe_b2 = iTaskGraphMeta;
            let __pe_b3 = iNumProc;
            move |__pe_a0| iParallelSectionCalculator(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    parTime = realSum(&levelCosts)?;
    parTime = HpcOmTaskGraph::roundReal(parTime, 2)?;
    oCriticalPathInfo =
        HpcOmTaskGraph::dumpCriticalPathInfo(&((criticalPaths, cpCosts)), &((criticalPathsWoC, cpCostsWoC)))?;
    cpCostsWoC = HpcOmTaskGraph::roundReal(cpCostsWoC, 2)?;
    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("the serialCosts: "));
            __mm_s.push_str(&*realString(serTime));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("the parallelCosts: "));
            __mm_s.push_str(&*realString(parTime));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("the cpCosts: "));
            __mm_s.push_str(&*realString(cpCostsWoC));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        i = 1;
        for mut levelCost in &*levelCosts {
            let mut levelCost = levelCost.clone();
            costShare = intDiv(((levelCost).0.floor() as i32) * 100, ((parTime).0.floor() as i32));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\tcosts for level "));
                __mm_s.push_str(&*intString(i));
                __mm_s.push_str(&*literal!(": "));
                __mm_s.push_str(&*realString(levelCost));
                __mm_s.push_str(&*literal!(" ("));
                __mm_s.push_str(&*System::snprintff(
                    literal!("%.0f"),
                    5,
                    metamodelica::OrderedFloat((costShare) as f64),
                )?);
                __mm_s.push_str(&*literal!("%)\n"));
                ArcStr::from(__mm_s)
            });
            i = i + 1;
        }
    }
    speedUp = metamodelica::OrderedFloat(0.0_f64);
    speedUpMax = metamodelica::OrderedFloat(0.0_f64);
    if realNe(parTime, metamodelica::OrderedFloat(0.0_f64)) {
        speedUp = realDiv(serTime, parTime);
    }
    if realNe(cpCostsWoC, metamodelica::OrderedFloat(0.0_f64)) {
        speedUpMax = realDiv(serTime, cpCostsWoC);
    }
    printPredictedExeTimeInfo(serTime, parTime, speedUp, speedUpMax, iNumProc)?;
    Ok(oCriticalPathInfo)
}

fn getLevelParallelTime(
    mut iLevelTaskList: &HpcOmSimCode::TaskList,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iNumProc: i32,
) -> Result<metamodelica::Real> {
    let mut oLevelCost: metamodelica::Real;
    let mut workload: metamodelica::Array<metamodelica::Real>;
    let mut levelTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    levelTasks = HpcOmCodegenUtil::getTasksOfTaskList(iLevelTaskList);
    workload = arrayCreate(iNumProc, metamodelica::OrderedFloat(0.0_f64));
    workload = List::fold(
        &levelTasks,
        &({
            let __pe_b1 = iTaskGraphMeta.clone();
            move |__pe_a0, __pe_a2| getLevelParallelTime1(&__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        workload.clone(),
    )?;
    oLevelCost = Array::fold(
        workload.clone(),
        &fnptr!(realMax, metamodelica::Real, metamodelica::Real),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    Ok(oLevelCost)
}

fn getLevelParallelTime1(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iThreadWorkLoad: metamodelica::Array<metamodelica::Real>,
) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oThreadWorkLoad: metamodelica::Array<metamodelica::Real>;
    let mut minWorkLoad: metamodelica::Real;
    let mut taskCosts: metamodelica::Real;
    let mut threadIdx: i32;
    let mut tmpThreadWorkLoad: metamodelica::Array<metamodelica::Real>;
    oThreadWorkLoad = (match &**iTask {
        HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: None, .. } => {
            taskCosts = getLevelTaskCosts(iTask, iTaskGraphMeta)?;
            minWorkLoad = Array::fold(
                iThreadWorkLoad.clone(),
                &fnptr!(realMin, metamodelica::Real, metamodelica::Real),
                metamodelica::arrayGet(iThreadWorkLoad.clone(), 1)?,
            )?;
            threadIdx = List::position(
                minWorkLoad,
                &(iThreadWorkLoad
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>()),
            )?;
            tmpThreadWorkLoad = metamodelica::arrayUpdate(iThreadWorkLoad.clone(), threadIdx, minWorkLoad + taskCosts)?;
            tmpThreadWorkLoad.clone()
        }
        HpcOmSimCode::Task::CALCTASK_LEVEL {
            threadIdx: Some(__esc_threadIdx),
            ..
        } => {
            threadIdx = (*__esc_threadIdx).clone();
            taskCosts = getLevelTaskCosts(iTask, iTaskGraphMeta)?;
            tmpThreadWorkLoad = metamodelica::arrayUpdate(
                iThreadWorkLoad.clone(),
                threadIdx.clone(),
                metamodelica::arrayGet(iThreadWorkLoad.clone(), threadIdx.clone())? + taskCosts,
            )?;
            tmpThreadWorkLoad.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(oThreadWorkLoad)
}

fn getLevelListTaskCosts(
    mut iTaskList: &HpcOmSimCode::TaskList,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<metamodelica::List<metamodelica::Real>> {
    let mut costsOut: metamodelica::List<metamodelica::Real>;
    let mut tasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    tasks = HpcOmCodegenUtil::getTasksOfTaskList(iTaskList);
    costsOut = List::map1(
        tasks,
        &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>, __a1: HpcOmTaskGraph::TaskGraphMeta| {
            getLevelTaskCosts(&__a0, __a1)
        },
        iMeta,
    )?;
    Ok(costsOut)
}

fn getLevelTaskCosts(
    mut levelTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iMeta: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    costsOut = (match &**levelTask {
        HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc, .. } => {
            let mut nodeCosts: metamodelica::List<metamodelica::Real>;
            let mut costs: metamodelica::Real;
            nodeCosts = List::map1(nodeIdc.clone(), &HpcOmTaskGraph::getExeCostReqCycles, iMeta)?;
            costs = List::fold(
                &nodeCosts,
                &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            costs
        }
        _ => {
            metamodelica::print(literal!("getLevelTaskCosts failed!\n"));
            return Err("fail");
        }
    });
    Ok(costsOut)
}

pub(crate) fn predictExecutionTime(
    mut scheduleIn: metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut cpCostsOption: Option<metamodelica::Real>,
    mut numProc: i32,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(
    metamodelica::Real,
    metamodelica::Real,
    metamodelica::Real,
    metamodelica::Real,
)> {
    let mut serialTimeOut: metamodelica::Real;
    let mut parallelTimeOut: metamodelica::Real;
    let mut speedUpOut: metamodelica::Real;
    let mut speedUpMaxOut: metamodelica::Real;
    let mut parTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut serTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUpMax: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut helper: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    if intNe(metamodelica::arrayLength(taskGraphIn.clone()), 0) {
        serTime = getSerialExecutionTime(taskGraphMetaIn.clone())?;
        (_, parTime) = getFinishingTimesForSchedule(scheduleIn, numProc, taskGraphIn.clone(), taskGraphMetaIn)?;
        speedUp = metamodelica::real_div_checked(serTime, parTime)?;
        helper = Util::getOptionOrDefault(cpCostsOption, (metamodelica::OrderedFloat(-1.0_f64)) * (serTime));
        speedUpMax = realDiv(serTime, helper);
    }
    serialTimeOut = serTime;
    parallelTimeOut = parTime;
    speedUpOut = speedUp;
    speedUpMaxOut = speedUpMax;
    Ok((serialTimeOut, parallelTimeOut, speedUpOut, speedUpMaxOut))
}

fn printPredictedExeTimeInfo(
    mut serTime: metamodelica::Real,
    mut parTime: metamodelica::Real,
    mut speedUp: metamodelica::Real,
    mut speedUpMax: metamodelica::Real,
    mut numProc: i32,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = speedUpMax;
        if let Ok(__v) = (|| -> Result<_> {
            let __rlit_0 = __mc_input.clone() else {
                return Err("nomatch");
            };
            if !(__rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64))) {
                return Err("guard");
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (speedUpMax == metamodelica::OrderedFloat(-1.0_f64)) else {
                return Err("pattern mismatch");
            };
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("The predicted SpeedUp with "));
                    __mm_s.push_str(&*intString(numProc));
                    __mm_s.push_str(&*literal!(" processors is "));
                    __mm_s.push_str(&*System::snprintff(literal!("%.2f"), 25, speedUp)?);
                    __mm_s.push_str(&*literal!(".\n"));
                    ArcStr::from(__mm_s)
                });
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                if speedUp > speedUpMax {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Something is weird. The predicted SpeedUp is "));
                        __mm_s.push_str(&*System::snprintff(literal!("%.2f"), 25, speedUp)?);
                        __mm_s.push_str(&*literal!(" and the theoretical maximum speedUp is "));
                        __mm_s.push_str(&*System::snprintff(literal!("%.2f"), 25, speedUpMax)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                } else if speedUp <= speedUpMax {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("The predicted SpeedUp with "));
                        __mm_s.push_str(&*intString(numProc));
                        __mm_s.push_str(&*literal!(" processors is: "));
                        __mm_s.push_str(&*System::snprintff(literal!("%.2f"), 25, speedUp)?);
                        __mm_s.push_str(&*literal!(" With a theoretical maximmum speedUp of: "));
                        __mm_s.push_str(&*System::snprintff(literal!("%.2f"), 25, speedUpMax)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub(crate) fn getSerialExecutionTime(mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut serialTimeOut: metamodelica::Real;
    let mut odeComps: metamodelica::List<i32>;
    let mut exeCostsReal: metamodelica::List<metamodelica::Real>;
    let mut exeCosts1: metamodelica::Array<metamodelica::Real>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        exeCosts: __pa0,
        inComps: __pa1,
        ..
    } = taskGraphMetaIn;
    exeCosts = metamodelica::Own::own(__pa0);
    inComps = metamodelica::Own::own(__pa1);
    odeComps = Array::fold(
        inComps.clone(),
        &fnptr!(listAppend, metamodelica::List<i32>, _),
        metamodelica::nil(),
    )?;
    exeCosts1 = Array::map(exeCosts.clone(), &fnptr!(Util::tuple22, _))?;
    exeCostsReal = List::map1(odeComps, &Array::getIndexFirst, exeCosts1.clone())?;
    serialTimeOut = List::fold(
        &exeCostsReal,
        &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    Ok(serialTimeOut)
}

fn getFinishingTimesForSchedule(
    mut scheduleIn: metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut numProc: i32,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(metamodelica::Ref<HpcOmSimCode::Schedule>, metamodelica::Real)> {
    let mut scheduleOut: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut finishingTime: metamodelica::Real;
    (scheduleOut, finishingTime) = 'mc: {
        let __mc_input = &*scheduleIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. } => {
                    let mut finTime: metamodelica::Real;
                    let mut taskIdcs: metamodelica::Array<i32>;
                    let mut finTimes: metamodelica::Array<metamodelica::Real>;
                    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
                    let mut checkedTasks: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    taskIdcs = arrayCreate(metamodelica::arrayLength(threadTasks.clone()), 1);
                    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraphIn.clone(), metamodelica::arrayLength(taskGraphIn.clone()))?;
                    checkedTasks = arrayCreate(metamodelica::arrayLength(taskGraphIn.clone()), openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY());
                    computeTimeFinished(threadTasks.clone(), taskIdcs.clone(), 1, checkedTasks.clone(), taskGraphIn.clone(), taskGraphT.clone(), taskGraphMetaIn.clone(), numProc, metamodelica::nil())?;
                    finTimes = Array::map(threadTasks.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>| getTimeFinishedOfLastTask(&__a0))?;
                    finTime = Array::fold(finTimes.clone(), &fnptr!(realMax, metamodelica::Real, metamodelica::Real), metamodelica::OrderedFloat(0.0_f64))?;
                    schedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
                    Ok((schedule.clone(), finTime))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: _, useFixedAssignments: _ } => {
                    let mut finTime: metamodelica::Real;
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    schedule = scheduleIn.clone();
                    finTime = metamodelica::OrderedFloat(0.0_f64);
                    Ok((schedule.clone(), finTime))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. } => {
                    let mut finTime: metamodelica::Real;
                    let mut schedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
                    schedule = scheduleIn.clone();
                    finTime = metamodelica::OrderedFloat(-1.0_f64);
                    Ok((schedule.clone(), finTime))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getFinishingTimesForSchedule failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((scheduleOut, finishingTime))
}

fn getTimeFinishedOfLastTask(
    mut threadTasksIn: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::Real> {
    let mut finTimeOut: metamodelica::Real;
    finTimeOut = 'mc: {
        let __mc_input = &**threadTasksIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lastTask: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut finTime: metamodelica::Real;
                    lastTask = List::last(threadTasksIn)?;
                    finTime = getTimeFinished(&lastTask);
                    Ok(finTime)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::OrderedFloat(-1.0_f64))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(finTimeOut)
}

fn computeTimeFinished(
    mut threadTasksIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    mut taskIdcsIn: metamodelica::Array<i32>,
    mut threadIdxIn: i32,
    mut checkedTasksIn: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut numProc: i32,
    mut closedThreadsIn: metamodelica::List<i32>,
) -> Result<()> {
    let mut threadIdx: i32 = threadIdxIn;
    let mut closedThreads: metamodelica::List<i32> = closedThreadsIn;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> = threadTasksIn;
    while !(((closedThreads).len() as i32) == numProc) {
        (threadIdx, closedThreads) = computeTimeFinished1(
            threadTasks.clone(),
            taskIdcsIn.clone(),
            threadIdx,
            checkedTasksIn.clone(),
            taskGraphIn.clone(),
            taskGraphTIn.clone(),
            taskGraphMetaIn.clone(),
            numProc,
            closedThreads,
        )?;
    }
    Ok(())
}

fn computeTimeFinished1(
    mut threadTasksIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    mut taskIdcsIn: metamodelica::Array<i32>,
    mut threadIdxIn: i32,
    mut checkedTasksIn: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut taskGraphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut numProc: i32,
    mut closedThreadsIn: metamodelica::List<i32>,
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut threadIdxOut: i32;
    let mut closedThreadsOut: metamodelica::List<i32>;
    (threadIdxOut, closedThreadsOut) = 'mc: {
        let __mc_input = &*closedThreadsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut taskIdx: i32;
                    let mut nextThreadIdx: i32;
                    let mut nextTaskIdx: i32;
                    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let true = (threadIdxIn <= metamodelica::arrayLength(taskIdcsIn.clone())) else { return Err("pattern mismatch") };
                    taskIdx = metamodelica::arrayGet(taskIdcsIn.clone(), threadIdxIn)?;
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdxIn)?;
                    let true = (taskIdx <= ((thread).len() as i32)) else { return Err("pattern mismatch") };
                    task = (thread).get(taskIdx)?;
                    (_, _, nextTaskIdx) = updateFinishingTime(&task, taskIdx, threadIdxIn, threadTasksIn.clone(), checkedTasksIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone())?;
                    metamodelica::arrayUpdate(taskIdcsIn.clone(), threadIdxIn, nextTaskIdx)?;
                    nextThreadIdx = getNextThreadIdx(threadIdxIn, &closedThreadsIn, numProc)?;
                    Ok((nextThreadIdx, closedThreadsIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nextThreadIdx: i32;
                    let true = (threadIdxIn > metamodelica::arrayLength(taskIdcsIn.clone())) else { return Err("pattern mismatch") };
                    nextThreadIdx = if (intGe(threadIdxIn, numProc)) {1} else {threadIdxIn + 1};
                    Ok((nextThreadIdx, closedThreadsIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut taskIdx: i32;
                    let mut nextThreadIdx: i32;
                    let mut closedThreads1: metamodelica::List<i32>;
                    let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let true = (threadIdxIn <= metamodelica::arrayLength(taskIdcsIn.clone())) else { return Err("pattern mismatch") };
                    taskIdx = metamodelica::arrayGet(taskIdcsIn.clone(), threadIdxIn)?;
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdxIn)?;
                    let true = (taskIdx > ((thread).len() as i32)) else { return Err("pattern mismatch") };
                    nextThreadIdx = if (intGe(threadIdxIn, numProc)) {1} else {threadIdxIn + 1};
                    closedThreads1 = metamodelica::cons(threadIdxIn, closedThreadsIn.clone());
                    closedThreads1 = List::unique(&closedThreads1);
                    Ok((nextThreadIdx, closedThreads1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("computeTimeFinished failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((threadIdxOut, closedThreadsOut))
}

fn getNextThreadIdx<'__b>(
    mut threadId: i32,
    mut closedThreads: &'__b metamodelica::List<i32>,
    mut numThreads: i32,
) -> Result<i32> {
    '__tco: loop {
        let mut isLastThread: bool;
        let mut isClosed: bool;
        let mut nextThread: i32;
        isLastThread = intEq(threadId, numThreads);
        nextThread = if (isLastThread) { 1 } else { threadId + 1 };
        isClosed = List::isMemberOnTrue(nextThread, closedThreads, &fnptr!(intEq, i32, i32))?;
        if (isClosed) {
            {
                (threadId, closedThreads, numThreads) = (nextThread, closedThreads, numThreads);
                continue '__tco;
            }
        } else {
            return Ok(nextThread);
        }
    }
}

fn updateFinishingTime(
    mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut taskIdxIn: i32,
    mut threadIdxIn: i32,
    mut threadTasksIn: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    mut checkedTasksIn: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut taskGraphTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta,
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
    i32,
)> {
    let mut threadTasksOut: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut checkedTasksOut: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut taskIdxOut: i32;
    (threadTasksOut, checkedTasksOut, taskIdxOut) = (match &**taskIn {
        HpcOmSimCode::Task::CALCTASK { index: taskID, .. } => {
            let mut isComputable: bool;
            let mut taskIdxNew: i32;
            let mut parentLst: metamodelica::List<i32>;
            let mut latestTask: metamodelica::Ref<HpcOmSimCode::Task>;
            let mut checkedTasks: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>;
            let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
            parentLst = metamodelica::arrayGet(taskGraphTIn.clone(), taskID.clone())?;
            (parentLst, latestTask) = List::fold1(
                &parentLst,
                &move |__a0: i32,
                       __a1: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
                       __a2: (metamodelica::List<i32>, metamodelica::Ref<HpcOmSimCode::Task>)| {
                    updateFinishingTime1(__a0, __a1, &__a2)
                },
                checkedTasksIn.clone(),
                (
                    metamodelica::nil(),
                    openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY(),
                ),
            )?;
            isComputable = (parentLst).is_empty();
            taskIdxNew = if (isComputable) { taskIdxIn + 1 } else { taskIdxIn };
            (threadTasks, checkedTasks) = if (isComputable) {
                computeFinishingTimeForOneTask(
                    &((
                        threadTasksIn.clone(),
                        checkedTasksIn.clone(),
                        taskIdxIn,
                        threadIdxIn,
                        latestTask,
                        taskGraphMetaIn,
                    )),
                )?
            } else {
                (threadTasksIn.clone(), checkedTasksIn.clone())
            };
            (threadTasks.clone(), checkedTasks.clone(), taskIdxNew)
        }
        HpcOmSimCode::Task::DEPTASK { .. } => {
            let mut taskIdxNew: i32;
            taskIdxNew = taskIdxIn + 1;
            (threadTasksIn.clone(), checkedTasksIn.clone(), taskIdxNew)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((threadTasksOut, checkedTasksOut, taskIdxOut))
}

fn updateFinishingTime1(
    mut parentIdx: i32,
    mut checkedTaskIn: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut tplIn: &(metamodelica::List<i32>, metamodelica::Ref<HpcOmSimCode::Task>),
) -> Result<(metamodelica::List<i32>, metamodelica::Ref<HpcOmSimCode::Task>)> {
    let mut tplOut: (metamodelica::List<i32>, metamodelica::Ref<HpcOmSimCode::Task>);
    let mut isCalc: bool;
    let mut finishingTime: metamodelica::Real;
    let mut finishingTimeIn: metamodelica::Real;
    let mut parentLst: metamodelica::List<i32>;
    let mut parentLstIn: metamodelica::List<i32>;
    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut taskIn: metamodelica::Ref<HpcOmSimCode::Task>;
    (parentLstIn, taskIn) = tplIn.clone();
    finishingTimeIn = getTimeFinished(&taskIn);
    task = metamodelica::arrayGet(checkedTaskIn.clone(), parentIdx)?;
    isCalc = isCalcTask(&task);
    finishingTime = if (isCalc) {
        getTimeFinished(&task)
    } else {
        metamodelica::OrderedFloat(-1.0_f64)
    };
    task = if (realGt(finishingTime, finishingTimeIn)) {
        task
    } else {
        taskIn
    };
    parentLst = if (isCalc) {
        parentLstIn
    } else {
        metamodelica::cons(parentIdx, parentLstIn)
    };
    tplOut = (parentLst, task);
    Ok(tplOut)
}

fn computeFinishingTimeForOneTask(
    mut tplIn: &(
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
        i32,
        i32,
        metamodelica::Ref<HpcOmSimCode::Task>,
        HpcOmTaskGraph::TaskGraphMeta,
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
)> {
    let mut tplOut: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>,
    );
    tplOut = 'mc: {
        let __mc_input = tplIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (threadTasksIn, checkedTasksIn, taskNum, threadIdx, latestTask, taskGraphMeta) => {
                    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
                    let mut checkedTasks: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut taskIdx: i32;
                    let mut finishingTime: metamodelica::Real;
                    let mut exeCost: metamodelica::Real;
                    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut preTask: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut threadIdx = (*threadIdx).clone();
                    let true = (isEmptyTask(metamodelica::AsArg::as_arg(&latestTask))) else { return Err("pattern mismatch") };
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdx.clone())?;
                    task = (thread).get(taskNum.clone())?;
                    threadIdx = getThreadId(&task);
                    preTask = getPredecessorCalcTask(&thread, taskNum.clone())?;
                    finishingTime = getTimeFinished(&preTask);
                    taskIdx = getTaskIdx(&task);
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(taskIdx, taskGraphMeta.clone())?;
                    finishingTime = finishingTime + exeCost;
                    task = updateTimeFinished(&task, finishingTime)?;
                    thread = List::replaceAt(task.clone(), taskNum.clone(), thread.clone())?;
                    threadTasks = metamodelica::arrayUpdate(threadTasksIn.clone(), threadIdx.clone(), thread.clone())?;
                    checkedTasks = metamodelica::arrayUpdate(checkedTasksIn.clone(), taskIdx, task.clone())?;
                    Ok((threadTasks.clone(), checkedTasks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (threadTasksIn, checkedTasksIn, taskNum, threadIdx, latestTask, taskGraphMeta) => {
                    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
                    let mut checkedTasks: metamodelica::Array<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let mut taskIdx: i32;
                    let mut taskIdxLatest: i32;
                    let mut threadIdxLatest: i32;
                    let mut commCost: metamodelica::Real;
                    let mut finishingTime: metamodelica::Real;
                    let mut finishingTime1: metamodelica::Real;
                    let mut finishingTimeComm: metamodelica::Real;
                    let mut exeCost: metamodelica::Real;
                    let mut task: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut preTask: metamodelica::Ref<HpcOmSimCode::Task>;
                    let mut thread: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
                    let false = (isEmptyTask(metamodelica::AsArg::as_arg(&latestTask))) else { return Err("pattern mismatch") };
                    finishingTime = getTimeFinished(metamodelica::AsArg::as_arg(&latestTask));
                    threadIdxLatest = getThreadId(metamodelica::AsArg::as_arg(&latestTask));
                    taskIdxLatest = getTaskIdx(metamodelica::AsArg::as_arg(&latestTask));
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdx.clone())?;
                    task = (thread).get(taskNum.clone())?;
                    taskIdx = getTaskIdx(&task);
                    commCost = HpcOmTaskGraph::getCommCostTimeBetweenNodes(taskIdxLatest, taskIdx, taskGraphMeta.clone())?;
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(taskIdx, taskGraphMeta.clone())?;
                    finishingTime = finishingTime + exeCost;
                    finishingTimeComm = finishingTime + commCost;
                    finishingTime = if (intEq(threadIdxLatest, threadIdx.clone())) {finishingTime} else {finishingTimeComm};
                    preTask = getPredecessorCalcTask(&thread, taskNum.clone())?;
                    finishingTime1 = getTimeFinished(&preTask);
                    finishingTime1 = finishingTime1 + exeCost;
                    finishingTime = realMax(finishingTime, finishingTime1);
                    task = updateTimeFinished(&task, finishingTime)?;
                    thread = List::replaceAt(task.clone(), taskNum.clone(), thread.clone())?;
                    threadTasks = metamodelica::arrayUpdate(threadTasksIn.clone(), threadIdx.clone(), thread.clone())?;
                    checkedTasks = metamodelica::arrayUpdate(checkedTasksIn.clone(), taskIdx, task.clone())?;
                    Ok((threadTasks.clone(), checkedTasks.clone()))
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

fn getPredecessorCalcTask<'__b>(
    mut threadIn: &'__b metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut indexIn: i32,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    '__tco: loop {
        let mut index: i32;
        let mut preTask: metamodelica::Ref<HpcOmSimCode::Task>;
        if indexIn == 1 {
            return Ok(openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY());
        } else {
            let true = (indexIn >= 2) else {
                return Err("pattern mismatch");
            };
            index = indexIn - 1;
            preTask = (threadIn).get(index)?;
            if (isCalcTask(&preTask)) {
                return Ok(preTask);
            } else {
                {
                    (threadIn, indexIn) = (threadIn, index);
                    continue '__tco;
                }
            }
        }
    }
}

fn updateTimeFinished(
    mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut timeFinishedIn: metamodelica::Real,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut taskOut: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut weighting: i32;
    let mut index: i32;
    let mut calcTime: metamodelica::Real;
    let mut timeFinished: metamodelica::Real;
    let mut threadIdx: i32;
    let mut eqIdc: metamodelica::List<i32>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*taskIn)) {
        Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __pa0, index: __pa1, calcTime: __pa2, timeFinished: __pa3, threadIdx: __pa4, eqIdc: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    weighting = metamodelica::Own::own(__pa0);
    index = metamodelica::Own::own(__pa1);
    calcTime = metamodelica::Own::own(__pa2);
    timeFinished = metamodelica::Own::own(__pa3);
    threadIdx = metamodelica::Own::own(__pa4);
    eqIdc = metamodelica::Own::own(__pa5);
    taskOut = metamodelica::Ref::new(HpcOmSimCode::Task::CALCTASK {
        weighting: weighting,
        index: index,
        calcTime: calcTime,
        timeFinished: timeFinishedIn,
        threadIdx: threadIdx,
        eqIdc: eqIdc,
    });
    Ok(taskOut)
}

fn getTimeFinished(mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>) -> metamodelica::Real {
    let mut finishingTime: metamodelica::Real;
    finishingTime = (match &**taskIn {
        HpcOmSimCode::Task::CALCTASK {
            timeFinished: fTime, ..
        } => fTime.clone(),
        HpcOmSimCode::Task::TASKEMPTY { .. } => metamodelica::OrderedFloat(0.0_f64),
        _ => metamodelica::OrderedFloat(-1.0_f64),
    });
    finishingTime
}

fn getThreadId(mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>) -> i32 {
    let mut threadId: i32;
    threadId = (match &**taskIn {
        HpcOmSimCode::Task::CALCTASK { threadIdx, .. } => threadIdx.clone(),
        _ => -1,
    });
    threadId
}

fn getTaskIdx(mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>) -> i32 {
    let mut idx: i32;
    idx = (match &**taskIn {
        HpcOmSimCode::Task::CALCTASK { index: taskIdx, .. } => taskIdx.clone(),
        _ => -1,
    });
    idx
}

fn getTaskTypeString(mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>) -> ArcStr {
    let mut oTypeString: ArcStr;
    oTypeString = (match &**iTask {
        HpcOmSimCode::Task::SCHEDULED_TASK { .. } => literal!("Scheduled task"),
        HpcOmSimCode::Task::CALCTASK { .. } => literal!("Calctask"),
        HpcOmSimCode::Task::CALCTASK_LEVEL { .. } => literal!("Calctask level"),
        HpcOmSimCode::Task::DEPTASK { .. } => literal!("Deptask"),
        HpcOmSimCode::Task::PREFETCHTASK { .. } => literal!("Prefetch task"),
        HpcOmSimCode::Task::TASKEMPTY { .. } => literal!("Empty task"),
        _ => literal!("Unknown"),
    });
    oTypeString
}

fn isCalcTask(mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>) -> bool {
    let mut isCalc: bool;
    isCalc = (match &**taskIn {
        HpcOmSimCode::Task::CALCTASK { .. } => true,
        _ => false,
    });
    isCalc
}

fn isEmptyTask(mut taskIn: &metamodelica::Ref<HpcOmSimCode::Task>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (match &**taskIn {
        HpcOmSimCode::Task::TASKEMPTY { .. } => true,
        _ => false,
    });
    isEmpty
}

//----------------
//  LockIdSetter
//----------------
fn setScheduleLockIds(
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut allThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tmpFoldArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut newAllThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut scheduledTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut lockIds: metamodelica::Array<metamodelica::List<(i32, i32)>>;
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut newOutgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    let mut newTuple: (i32, i32);
    let mut sourceTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut targetTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut iterTask: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut counter: i32;
    let mut id: i32;
    let mut sourceTaskId: i32;
    let mut targetTaskId: i32;
    let mut outgoing: bool;
    let mut communicationInfo: HpcOmSimCode::CommunicationInfo;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*iSchedule)) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa0, outgoingDepTasks: __pa1, scheduledTasks: __pa2, allCalcTasks: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    allThreadTasks = metamodelica::Own::own(__pa0);
    outgoingDepTasks = metamodelica::Own::own(__pa1);
    scheduledTasks = metamodelica::Own::own(__pa2);
    allCalcTasks = metamodelica::Own::own(__pa3);
    lockIds = arrayCreate(metamodelica::arrayLength(allCalcTasks.clone()), metamodelica::nil());
    newAllThreadTasks = arrayCreate(metamodelica::arrayLength(allThreadTasks.clone()), metamodelica::nil());
    counter = 0;
    for mut iterTask in &*outgoingDepTasks {
        let mut iterTask = iterTask.clone();
        let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(iterTask) {
            Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: __pa4, targetTask: __pa5, outgoing: __pa6, id: __pa7, communicationInfo: __pa8 } => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
            _ => return Err("pattern mismatch"),
        } };
        sourceTask = metamodelica::Own::own(__pa4);
        targetTask = metamodelica::Own::own(__pa5);
        outgoing = metamodelica::Own::own(__pa6);
        id = metamodelica::Own::own(__pa7);
        communicationInfo = metamodelica::Own::own(__pa8);
        let __pa9 = ::match_deref::match_deref! { match &(sourceTask.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa9, .. } => __pa9.clone(),
            _ => return Err("pattern mismatch"),
        } };
        sourceTaskId = metamodelica::Own::own(__pa9);
        let __pa10 = ::match_deref::match_deref! { match &(targetTask.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa10, .. } => __pa10.clone(),
            _ => return Err("pattern mismatch"),
        } };
        targetTaskId = metamodelica::Own::own(__pa10);
        newTuple = (targetTaskId, counter);
        metamodelica::arrayUpdate(
            lockIds.clone(),
            sourceTaskId,
            listAppend(metamodelica::arrayGet(lockIds.clone(), sourceTaskId)?, list![newTuple]),
        )?;
        newOutgoingDepTasks = metamodelica::cons(
            metamodelica::Ref::new(HpcOmSimCode::Task::DEPTASK {
                sourceTask: sourceTask,
                targetTask: targetTask,
                outgoing: outgoing,
                id: counter,
                communicationInfo: communicationInfo,
            }),
            newOutgoingDepTasks,
        );
        counter = counter + 1;
    }
    tmpFoldArray = arrayCreate(metamodelica::arrayLength(allThreadTasks.clone()), metamodelica::nil());
    (newAllThreadTasks, _) = Array::fold(
        allThreadTasks.clone(),
        &({
            let __pe_b1 = lockIds.clone();
            move |__pe_a0, __pe_a2| replaceDepTaskIdsByLockIds(&__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        (tmpFoldArray.clone(), 1),
    )?;
    oSchedule = metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
        threadTasks: newAllThreadTasks.clone(),
        outgoingDepTasks: newOutgoingDepTasks,
        scheduledTasks: scheduledTasks,
        allCalcTasks: allCalcTasks.clone(),
    });
    Ok(oSchedule)
}

fn replaceDepTaskIdsByLockIds(
    mut inTasks: &metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut lockIds: metamodelica::Array<metamodelica::List<(i32, i32)>>,
    mut iAllThreadTasks: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    i32,
)> {
    let mut oTasks: (
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        i32,
    );
    let mut allThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tmpList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut threadId: i32;
    (allThreadTasks, threadId) = iAllThreadTasks;
    tmpList = List::fold(
        inTasks,
        &({
            let __pe_b1 = lockIds.clone();
            move |__pe_a0, __pe_a2| replaceDepTasksInListByLockIds(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        metamodelica::nil(),
    )?
    .reverse();
    metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, tmpList)?;
    oTasks = (allThreadTasks.clone(), threadId + 1);
    Ok(oTasks)
}

fn replaceDepTasksInListByLockIds(
    mut inTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut lockIds: metamodelica::Array<metamodelica::List<(i32, i32)>>,
    mut tmpTaskList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
) -> Result<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut oList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task>;
    tmpTask = findTaskWithLockId(lockIds.clone(), inTask)?;
    oList = metamodelica::cons(tmpTask, tmpTaskList);
    Ok(oList)
}

fn findTaskWithLockId(
    mut lockIds: metamodelica::Array<metamodelica::List<(i32, i32)>>,
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut tmpTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut sourceTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut targetTask: metamodelica::Ref<HpcOmSimCode::Task>;
    let mut outgoing: bool;
    let mut lockId: i32;
    let mut sourceTaskId: i32;
    let mut targetTaskId: i32;
    let mut communicationInfo: HpcOmSimCode::CommunicationInfo;
    oTask = (match &*iTask {
        HpcOmSimCode::Task::DEPTASK {
            sourceTask: __esc_sourceTask,
            targetTask: __esc_targetTask,
            outgoing: __esc_outgoing,
            communicationInfo: __esc_communicationInfo,
            ..
        } => {
            sourceTask = (*__esc_sourceTask).clone();
            targetTask = (*__esc_targetTask).clone();
            outgoing = (*__esc_outgoing).clone();
            communicationInfo = (*__esc_communicationInfo).clone();
            let __pa0 = ::match_deref::match_deref! { match &(sourceTask.clone()) {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            sourceTaskId = metamodelica::Own::own(__pa0);
            let __pa1 = ::match_deref::match_deref! { match &(targetTask.clone()) {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa1, .. } => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            targetTaskId = metamodelica::Own::own(__pa1);
            lockId = findInIntTuple1(&(metamodelica::arrayGet(lockIds.clone(), sourceTaskId)?), targetTaskId)?;
            tmpTask = metamodelica::Ref::new(HpcOmSimCode::Task::DEPTASK {
                sourceTask: sourceTask.clone(),
                targetTask: targetTask.clone(),
                outgoing: outgoing.clone(),
                id: lockId,
                communicationInfo: communicationInfo.clone(),
            });
            tmpTask
        }
        _ => iTask,
    });
    Ok(oTask)
}

fn findInIntTuple1(mut liste: &metamodelica::List<(i32, i32)>, mut toFind: i32) -> Result<i32> {
    let mut secondElement: i32;
    let mut first: i32;
    let mut second: i32;
    let mut iter: (i32, i32) = (0, 0);
    for mut iter in &**liste {
        let mut iter = iter.clone();
        (first, second) = iter;
        if intEq(first, toFind) {
            secondElement = second;
            return Ok(secondElement);
        }
    }
    return Err("fail");
    Ok(secondElement)
}

fn printRealArray(mut inArray: metamodelica::Array<metamodelica::Real>, mut header: &ArcStr) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("The "));
        __mm_s.push_str(&*header);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(literal!("-----------------------------------------\n"));
    Array::fold(
        inArray.clone(),
        &({
            let __pe_b1 = header.clone();
            move |__pe_a0, __pe_a2| Ok(printRealArray1(__pe_a0, &__pe_b1, __pe_a2))
        }),
        1,
    )?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn printRealArray1(mut inValue: metamodelica::Real, mut header: &ArcStr, mut idxIn: i32) -> i32 {
    let mut idxOut: i32;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("node: "));
        __mm_s.push_str(&*intString(idxIn));
        __mm_s.push_str(&*literal!(" has the "));
        __mm_s.push_str(&*header);
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*realString(inValue));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    idxOut = idxIn + 1;
    idxOut
}

fn intListString(mut lstIn: metamodelica::List<i32>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(List::map(lstIn.clone(), &fnptr!(intString, i32))?, literal!(" , "));
    s = if ((lstIn).is_empty()) { literal!("{}") } else { s };
    Ok(s)
}

fn intListListString(mut lstIn: metamodelica::List<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(List::map(lstIn, &intListString)?, literal!(" | "));
    Ok(s)
}

pub(crate) fn expandSchedule(
    mut iNumProc: i32,
    mut iNumUsedProc: i32,
    mut iSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>,
) -> Result<metamodelica::Ref<HpcOmSimCode::Schedule>> {
    let mut oSchedule: metamodelica::Ref<HpcOmSimCode::Schedule>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut outgoingDepTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut scheduledTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut allCalcTasks: metamodelica::Array<(metamodelica::Ref<HpcOmSimCode::Task>, i32)>;
    oSchedule = (match &*iSchedule {
        HpcOmSimCode::Schedule::LEVELSCHEDULE { .. } => iSchedule,
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks: __esc_threadTasks,
            outgoingDepTasks: __esc_outgoingDepTasks,
            scheduledTasks: __esc_scheduledTasks,
            allCalcTasks: __esc_allCalcTasks,
        } => {
            threadTasks = (*__esc_threadTasks).clone();
            outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
            scheduledTasks = (*__esc_scheduledTasks).clone();
            allCalcTasks = (*__esc_allCalcTasks).clone();
            threadTasks = Array::expandToSize(iNumProc, threadTasks.clone(), metamodelica::nil())?;
            metamodelica::Ref::new(HpcOmSimCode::Schedule::THREADSCHEDULE {
                threadTasks: threadTasks.clone(),
                outgoingDepTasks: outgoingDepTasks.clone(),
                scheduledTasks: scheduledTasks.clone(),
                allCalcTasks: allCalcTasks.clone(),
            })
        }
        HpcOmSimCode::Schedule::TASKDEPSCHEDULE { .. } => iSchedule,
        HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. } => iSchedule,
    });
    Ok(oSchedule)
}
