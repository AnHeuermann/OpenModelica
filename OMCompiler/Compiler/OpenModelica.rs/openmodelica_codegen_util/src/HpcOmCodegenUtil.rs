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

use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

pub fn getTasksOfTaskList(
    mut iTaskList: &HpcOmSimCode::TaskList,
) -> metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut tasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    oTasks = (match iTaskList.clone() {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        }
        HpcOmSimCode::TaskList::SERIALTASKLIST {
            tasks: mut __esc_tasks, ..
        } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        }
        _ => {
            metamodelica::print(literal!("getTasksOfTaskList failed! Unsupported task list.\n"));
            metamodelica::nil()
        }
    });
    oTasks
}

pub fn convertFixedLevelScheduleToLevelThreadLists(
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iNumOfThreads: i32,
) -> Result<metamodelica::List<metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>>> {
    let mut oLevelThreadLists: metamodelica::List<
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    >;
    let mut tasksOfLevels: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut tmpLevelThreadLists: metamodelica::List<
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    >;
    oLevelThreadLists = (match &**iSchedule {
        HpcOmSimCode::Schedule::LEVELSCHEDULE {
            tasksOfLevels: __esc_tasksOfLevels,
            useFixedAssignments: true,
        } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            tmpLevelThreadLists = List::map(
                tasksOfLevels.clone(),
                &({
                    let __pe_b1 = iNumOfThreads;
                    move |__pe_a0| convertFixedLevelScheduleToLevelThreadLists0(&__pe_a0, __pe_b1.clone())
                }),
            )?;
            tmpLevelThreadLists
        }
        _ => metamodelica::nil(),
    });
    Ok(oLevelThreadLists)
}

fn convertFixedLevelScheduleToLevelThreadLists0(
    mut iTasksOfLevel: &HpcOmSimCode::TaskList,
    mut iNumOfThreads: i32,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>> {
    let mut oLevelThreadLists: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut task: metamodelica::Ref<HpcOmSimCode::Task> = metamodelica::Ref::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut threadIdx: i32;
    let mut tmpLevelThreadLists: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    tasks = getTasksOfTaskList(iTasksOfLevel);
    tmpLevelThreadLists = arrayCreate(iNumOfThreads, metamodelica::nil());
    for mut task in &*tasks.reverse() {
        let mut task = task.clone();
        let __pa0 = ::match_deref::match_deref! { match &(task.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(__pa0), .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        threadIdx = metamodelica::Own::own(__pa0);
        tmpLevelThreadLists = metamodelica::arrayUpdate(
            tmpLevelThreadLists.clone(),
            threadIdx,
            metamodelica::cons(task, metamodelica::arrayGet(tmpLevelThreadLists.clone(), threadIdx)?),
        )?;
    }
    oLevelThreadLists = tmpLevelThreadLists.clone();
    Ok(oLevelThreadLists)
}

pub fn convertFixedLevelScheduleToTaskLists(
    mut iOdeSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iDaeSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iZeroFuncSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iNumOfThreads: i32,
) -> Result<
    metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )>,
> {
    let mut oThreadLevelTasks: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )>;
    let mut tasksOfLevelsOde: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut tasksOfLevelsDae: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut tasksOfLevelsZeroFunc: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut tmpThreadLevelTasksDae: metamodelica::List<
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    >;
    let mut tmpThreadLevelTasksOde: metamodelica::List<
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    >;
    let mut tmpThreadLevelTasksZeroFunc: metamodelica::List<
        metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    >;
    let mut tmpResultLists: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )>;
    oThreadLevelTasks = (::match_deref::match_deref! { match (iOdeSchedule, iDaeSchedule, iZeroFuncSchedule) {
        (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevelsOde, useFixedAssignments: true }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevelsDae, useFixedAssignments: true }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevelsZeroFunc, useFixedAssignments: true }) => {
            tasksOfLevelsOde = (*__esc_tasksOfLevelsOde).clone();
            tasksOfLevelsDae = (*__esc_tasksOfLevelsDae).clone();
            tasksOfLevelsZeroFunc = (*__esc_tasksOfLevelsZeroFunc).clone();
            tmpResultLists = arrayCreate(iNumOfThreads, (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
            tmpThreadLevelTasksOde = List::map1(tasksOfLevelsOde.clone(), &move |__a0: HpcOmSimCode::TaskList, __a1: i32| convertFixedLevelScheduleToTaskListsForLevel(&__a0, __a1), iNumOfThreads)?;
            tmpThreadLevelTasksDae = List::map1(tasksOfLevelsDae.clone(), &move |__a0: HpcOmSimCode::TaskList, __a1: i32| convertFixedLevelScheduleToTaskListsForLevel(&__a0, __a1), iNumOfThreads)?;
            tmpThreadLevelTasksZeroFunc = List::map1(tasksOfLevelsZeroFunc.clone(), &move |__a0: HpcOmSimCode::TaskList, __a1: i32| convertFixedLevelScheduleToTaskListsForLevel(&__a0, __a1), iNumOfThreads)?;
            tmpResultLists = List::fold(&tmpThreadLevelTasksOde, &({ let __pe_b1 = 1; let __pe_b2 = 0; move |__pe_a0, __pe_a3| Ok(convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }), tmpResultLists.clone())?;
            tmpResultLists = List::fold(&tmpThreadLevelTasksDae, &({ let __pe_b1 = 1; let __pe_b2 = 1; move |__pe_a0, __pe_a3| Ok(convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }), tmpResultLists.clone())?;
            tmpResultLists = List::fold(&tmpThreadLevelTasksZeroFunc, &({ let __pe_b1 = 1; let __pe_b2 = 2; move |__pe_a0, __pe_a3| Ok(convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }), tmpResultLists.clone())?;
            tmpResultLists = revertTaskLists(1, tmpResultLists.clone());
            tmpResultLists.clone()
        },
        _ => {
            tmpResultLists = arrayCreate(iNumOfThreads, (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
            tmpResultLists.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadLevelTasks)
}

fn convertFixedLevelScheduleToTaskLists1(
    mut iLevelTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    mut iCurrentThreadIdx: i32,
    mut iModifiedSystemIdx: i32,
    mut iResultList: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )>,
) -> metamodelica::Array<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
)> {
    let mut oResultList: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )>;
    let mut tmpResultList: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )> = Default::default();
    let mut entryOde: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        metamodelica::nil();
    let mut entryDae: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        metamodelica::nil();
    let mut entryZeroFunc: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        metamodelica::nil();
    oResultList = 'mc: {
        let __mc_input = iResultList.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut entryDae: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                entryDae.clone();
            let mut entryOde: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                entryOde.clone();
            let mut entryZeroFunc: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                entryZeroFunc.clone();
            let mut tmpResultList: metamodelica::Array<(
                metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
                metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
                metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
            )> = tmpResultList.clone();
            let true = (intLe(iCurrentThreadIdx, metamodelica::arrayLength(iLevelTasks.clone()))) else {
                return Err("pattern mismatch");
            };
            (entryOde, entryDae, entryZeroFunc) = metamodelica::arrayGet(iResultList.clone(), iCurrentThreadIdx)?;
            if intEq(iModifiedSystemIdx, 0) {
                entryOde = metamodelica::cons(
                    metamodelica::arrayGet(iLevelTasks.clone(), iCurrentThreadIdx)?,
                    entryOde.clone(),
                );
            } else {
                if intEq(iModifiedSystemIdx, 1) {
                    entryDae = metamodelica::cons(
                        metamodelica::arrayGet(iLevelTasks.clone(), iCurrentThreadIdx)?,
                        entryDae.clone(),
                    );
                } else {
                    entryZeroFunc = metamodelica::cons(
                        metamodelica::arrayGet(iLevelTasks.clone(), iCurrentThreadIdx)?,
                        entryZeroFunc.clone(),
                    );
                }
            }
            tmpResultList = metamodelica::arrayUpdate(
                iResultList.clone(),
                iCurrentThreadIdx,
                (entryOde.clone(), entryDae.clone(), entryZeroFunc.clone()),
            )?;
            tmpResultList = convertFixedLevelScheduleToTaskLists1(
                iLevelTasks.clone(),
                iCurrentThreadIdx + 1,
                iModifiedSystemIdx,
                tmpResultList.clone(),
            );
            Ok((
                tmpResultList.clone(),
                entryDae.clone(),
                entryOde.clone(),
                entryZeroFunc.clone(),
                tmpResultList.clone(),
            ))
        })() {
            entryDae = __wb0;
            entryOde = __wb1;
            entryZeroFunc = __wb2;
            tmpResultList = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iResultList.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oResultList
}

fn revertTaskLists(
    mut iCurrentArrayIdx: i32,
    mut iResultList: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )>,
) -> metamodelica::Array<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
)> {
    let mut oResultList: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )>;
    let mut entryOde: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        metamodelica::nil();
    let mut entryDae: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        metamodelica::nil();
    let mut entryZeroFunc: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        metamodelica::nil();
    let mut tmpResultList: metamodelica::Array<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    )> = Default::default();
    oResultList = 'mc: {
        let __mc_input = iResultList.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut entryDae: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                entryDae.clone();
            let mut entryOde: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                entryOde.clone();
            let mut entryZeroFunc: metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                entryZeroFunc.clone();
            let mut tmpResultList: metamodelica::Array<(
                metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
                metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
                metamodelica::List<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
            )> = tmpResultList.clone();
            let true = (intLe(iCurrentArrayIdx, metamodelica::arrayLength(iResultList.clone()))) else {
                return Err("pattern mismatch");
            };
            (entryOde, entryDae, entryZeroFunc) = metamodelica::arrayGet(iResultList.clone(), iCurrentArrayIdx)?;
            entryOde = entryOde.clone().reverse();
            entryDae = entryDae.clone().reverse();
            entryZeroFunc = entryZeroFunc.clone().reverse();
            tmpResultList = metamodelica::arrayUpdate(
                iResultList.clone(),
                iCurrentArrayIdx,
                (entryOde.clone(), entryDae.clone(), entryZeroFunc.clone()),
            )?;
            tmpResultList = revertTaskLists(iCurrentArrayIdx + 1, tmpResultList.clone());
            Ok((
                tmpResultList.clone(),
                entryDae.clone(),
                entryOde.clone(),
                entryZeroFunc.clone(),
                tmpResultList.clone(),
            ))
        })() {
            entryDae = __wb0;
            entryOde = __wb1;
            entryZeroFunc = __wb2;
            tmpResultList = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iResultList.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oResultList
}

fn revertTaskList(
    mut iCurrentArrayIdx: i32,
    mut iResultList: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
) -> metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> {
    let mut oResultList: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut entry: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = metamodelica::nil();
    let mut tmpResultList: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
        Default::default();
    oResultList = 'mc: {
        let __mc_input = iResultList.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut entry: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> = entry.clone();
            let mut tmpResultList: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>> =
                tmpResultList.clone();
            let true = (intLe(iCurrentArrayIdx, metamodelica::arrayLength(iResultList.clone()))) else {
                return Err("pattern mismatch");
            };
            entry = metamodelica::arrayGet(iResultList.clone(), iCurrentArrayIdx)?;
            entry = entry.clone().reverse();
            tmpResultList = metamodelica::arrayUpdate(iResultList.clone(), iCurrentArrayIdx, entry.clone())?;
            Ok((tmpResultList.clone(), entry.clone(), tmpResultList.clone()))
        })() {
            entry = __wb0;
            tmpResultList = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iResultList.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oResultList
}

fn convertFixedLevelScheduleToTaskListsForLevel(
    mut iTasksOfLevel: &HpcOmSimCode::TaskList,
    mut iThreadCount: i32,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>> {
    let mut oThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tmpTaskLists: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    oThreadTasks = (match iTasksOfLevel.clone() {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tmpTaskLists = arrayCreate(iThreadCount, metamodelica::nil());
            tmpTaskLists = List::fold(
                metamodelica::AsArg::as_arg(&tasks),
                &convertFixedLevelScheduleToTaskListsForTask,
                tmpTaskLists.clone(),
            )?;
            tmpTaskLists = revertTaskList(1, tmpTaskLists.clone());
            tmpTaskLists.clone()
        }
        HpcOmSimCode::TaskList::SERIALTASKLIST {
            tasks: mut __esc_tasks, ..
        } => {
            tasks = __esc_tasks.clone();
            tmpTaskLists = arrayCreate(iThreadCount, metamodelica::nil());
            tmpTaskLists = metamodelica::arrayUpdate(tmpTaskLists.clone(), 1, tasks.clone())?;
            tmpTaskLists.clone()
        }
    });
    Ok(oThreadTasks)
}

fn convertFixedLevelScheduleToTaskListsForTask(
    mut iTask: metamodelica::Ref<HpcOmSimCode::Task>,
    mut iThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>> {
    let mut oThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut tmpTaskLists: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut threadIdx: i32;
    let mut oldTaskList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    oThreadTasks = (match &*iTask.clone() {
        HpcOmSimCode::Task::CALCTASK_LEVEL {
            threadIdx: Some(__esc_threadIdx),
            ..
        } => {
            threadIdx = (*__esc_threadIdx).clone();
            oldTaskList = metamodelica::arrayGet(iThreadTasks.clone(), threadIdx.clone())?;
            tmpTaskLists = metamodelica::arrayUpdate(
                iThreadTasks.clone(),
                threadIdx.clone(),
                metamodelica::cons(iTask, oldTaskList),
            )?;
            tmpTaskLists.clone()
        }
        _ => {
            metamodelica::print(literal!(
                "ConvertFixedLevelScheduleToTaskListsForTask can just handle CALCTASK_LEVEL with defined thread idx!\n"
            ));
            iThreadTasks.clone()
        }
    });
    Ok(oThreadTasks)
}

pub fn getSimCodeEqByIndex(
    mut iEqs: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut iIdx: i32,
) -> Result<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut oEq: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut rest: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut head: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut headIdx: i32 = 0;
    let mut headIdx2: i32 = 0;
    oEq = 'mc: {
        let __mc_input = &**iEqs;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut headIdx: i32 = headIdx.clone();
                    let mut headIdx2: i32 = headIdx2.clone();
                    (headIdx, headIdx2) = getIndexBySimCodeEq(metamodelica::AsArg::as_arg(&head))?;
                    let true = (intEq(headIdx, iIdx) || intEq(headIdx2, iIdx)) else { return Err("pattern mismatch") };
                    Ok((head.clone(), headIdx.clone(), headIdx2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            headIdx = __wb0;
            headIdx2 = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    Ok(getSimCodeEqByIndex(metamodelica::AsArg::as_arg(&rest), iIdx)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSimCodeEqByIndex failed. Looking for Index ")); __mm_s.push_str(&*intString(iIdx)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oEq)
}

pub fn getIndexBySimCodeEq(mut iEq: &metamodelica::Ref<SimCode::SimEqSystem>) -> Result<(i32, i32)> {
    let mut oIdx: i32;
    let mut oIdx2: i32;
    let mut index: i32;
    let mut index2: i32;
    (oIdx, oIdx2) = (::match_deref::match_deref! { match iEq {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: __esc_index, .. }, alternativeTearing: None, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: __esc_index, .. }, alternativeTearing: None, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: __esc_index, .. }, alternativeTearing: Some(Deref @ SimCode::LinearSystem { index: __esc_index2, .. }), .. } => {
            index = (*__esc_index).clone();
            index2 = (*__esc_index2).clone();
            (index.clone(), index2.clone())
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: __esc_index, .. }, alternativeTearing: Some(Deref @ SimCode::NonlinearSystem { index: __esc_index2, .. }), .. } => {
            index = (*__esc_index).clone();
            index2 = (*__esc_index2).clone();
            (index.clone(), index2.clone())
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmCodegenUtil.getIndexBySimCodeEq")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/HpcOmCodegenUtil.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oIdx, oIdx2))
}
