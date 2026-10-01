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

use crate::HashSet;
use crate::HashTableSM1;
use crate::InnerOuter;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTable3;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/// Collecting information about a state/mode
#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct SMNode {
    pub componentRef: metamodelica::Ref<DAE::ComponentRef>,
    pub isInitial: bool,
    /// relations to other modes due to in- and out-going transitions
    pub edges: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ),
}

impl metamodelica::gc::MMTrace for SMNode {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.componentRef, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isInitial, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.edges, __mmv)?;
        Ok(())
    }
}
impl PartialEq for SMNode {
    fn eq(&self, other: &Self) -> bool {
        self.componentRef == other.componentRef
            && self.isInitial == other.isInitial
            && (match ((&self.edges), (&other.edges)) {
                ((__lt0, __lt1, __lt2, __lt3, __lt4), (__rt0, __rt1, __rt2, __rt3, __rt4)) => {
                    (__lt0 == __rt0)
                        && (__lt1 == __rt1)
                        && (__lt2 == __rt2)
                        && (__lt3 == __rt3)
                        && (match (__lt4, __rt4) {
                            ((__lt0, __lt1, __lt2), (__rt0, __rt1, __rt2)) => {
                                std::sync::Arc::ptr_eq(__lt0, __rt0)
                                    && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                    && std::sync::Arc::ptr_eq(__lt2, __rt2)
                            }
                        })
                }
            })
    }
}
impl Eq for SMNode {}
impl PartialOrd for SMNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SMNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.componentRef
            .cmp(&other.componentRef)
            .then_with(|| self.isInitial.cmp(&other.isInitial))
            .then_with(|| {
                (match ((&self.edges), (&other.edges)) {
                    ((__lt0, __lt1, __lt2, __lt3, __lt4), (__rt0, __rt1, __rt2, __rt3, __rt4)) => __lt0
                        .cmp(__rt0)
                        .then_with(|| __lt1.cmp(__rt1))
                        .then_with(|| __lt2.cmp(__rt2))
                        .then_with(|| __lt3.cmp(__rt3))
                        .then_with(|| {
                            (match (__lt4, __rt4) {
                                ((__lt0, __lt1, __lt2), (__rt0, __rt1, __rt2)) => (std::sync::Arc::as_ptr(__lt0)
                                    as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt1) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                                    })
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt2) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                                    }),
                            })
                        }),
                })
            })
    }
}
impl std::fmt::Debug for SMNode {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("SMNode");
        __ds.field("componentRef", &self.componentRef);
        __ds.field("isInitial", &self.isInitial);
        __ds.field(
            "edges",
            &format_args!("<dyn-fn-container@{:p}>", (&self.edges) as *const _),
        );
        __ds.finish()
    }
}

impl Default for SMNode {
    fn default() -> Self {
        Self {
            componentRef: Default::default(),
            isInitial: Default::default(),
            edges: (
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashSet::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashSet::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashSet::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
        }
    }
}

pub type SMNODE = SMNode;

/// Collecting information about a group of state components forming a flat state machine
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FlatSMGroup {
    pub initState: metamodelica::Ref<DAE::ComponentRef>,
    pub states: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>,
}

impl metamodelica::gc::MMTrace for FlatSMGroup {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.initState, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.states, __mmv)?;
        Ok(())
    }
}
pub type FLAT_SM_GROUP = FlatSMGroup;

#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct AdjacencyTable {
    /// Map cref to corresponding index in adjacency matrix
    pub cref2index: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    ),
    /// Adjacency matrix showing which modes are connected by transitions
    pub adjacency: metamodelica::Array<metamodelica::Array<bool>>,
}

impl metamodelica::gc::MMTrace for AdjacencyTable {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.cref2index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.adjacency, __mmv)?;
        Ok(())
    }
}
impl PartialEq for AdjacencyTable {
    fn eq(&self, other: &Self) -> bool {
        (match ((&self.cref2index), (&other.cref2index)) {
            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                (__lt0 == __rt0)
                    && (__lt1 == __rt1)
                    && (__lt2 == __rt2)
                    && (match (__lt3, __rt3) {
                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                            std::sync::Arc::ptr_eq(__lt0, __rt0)
                                && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                && std::sync::Arc::ptr_eq(__lt3, __rt3)
                        }
                    })
            }
        }) && self.adjacency == other.adjacency
    }
}
impl Eq for AdjacencyTable {}
impl PartialOrd for AdjacencyTable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AdjacencyTable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (match ((&self.cref2index), (&other.cref2index)) {
            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                .cmp(__rt0)
                .then_with(|| __lt1.cmp(__rt1))
                .then_with(|| __lt2.cmp(__rt2))
                .then_with(|| {
                    (match (__lt3, __rt3) {
                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (std::sync::Arc::as_ptr(__lt0)
                            as *const ())
                            .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt1) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                            })
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt2) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                            })
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt3) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                            }),
                    })
                }),
        })
        .then_with(|| self.adjacency.cmp(&other.adjacency))
    }
}
impl std::fmt::Debug for AdjacencyTable {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("AdjacencyTable");
        __ds.field(
            "cref2index",
            &format_args!("<dyn-fn-container@{:p}>", (&self.cref2index) as *const _),
        );
        __ds.field("adjacency", &self.adjacency);
        __ds.finish()
    }
}

impl Default for AdjacencyTable {
    fn default() -> Self {
        Self {
            cref2index: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTable::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            adjacency: Default::default(),
        }
    }
}

pub type ADJACENCY_TABLE = AdjacencyTable;

// Table having crefs as keys and corresponding SMNODE as value
pub type SMNodeTable = (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, SMNode)>>,
    ),
    i32,
    (
        HashTableSM1::FuncHashCref,
        HashTableSM1::FuncCrefEqual,
        HashTableSM1::FuncCrefStr,
        HashTableSM1::FuncExpStr,
    ),
);

// Table mapping crefs of SMNodes to corresponding crefs of FlatSMGroup
pub type SMNodeToFlatSMGroupTable = (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<DAE::ComponentRef>,
            )>,
        >,
    ),
    i32,
    (
        HashTableCG::FuncHashCref,
        HashTableCG::FuncCrefEqual,
        HashTableCG::FuncCrefStr,
        HashTableCG::FuncExpStr,
    ),
);

pub(crate) const SMS_PRE: &'static str = "smOf";

pub(crate) const DEBUG_SMDUMP: bool = false;

pub(crate) fn createSMNodeToFlatSMGroupTable(mut inDae: DAE::DAElist) -> Result<SMNodeToFlatSMGroupTable> {
    let mut smNodeToFlatSMGroup: SMNodeToFlatSMGroupTable;
    let mut elementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smNodeTable: SMNodeTable;
    let mut nStates: i32;
    let mut iTable: AdjacencyTable;
    let mut transClosure: AdjacencyTable;
    let mut initialStates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut flatSMGroup: metamodelica::List<FlatSMGroup>;
    if intLt(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 33) {
        smNodeToFlatSMGroup = HashTableCG::emptyHashTableSized(1);
        return Ok(smNodeToFlatSMGroup);
    }
    let DAE::DAE { elementLst: __pa0 } = inDae;
    elementLst = metamodelica::Own::own(__pa0);
    smNodeTable = getSMNodeTable(elementLst)?;
    nStates = BaseHashTable::hashTableCurrentSize(&smNodeTable);
    if nStates > 0 {
        smNodeToFlatSMGroup = HashTableCG::emptyHashTable();
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!(
                "***** InstStateMachineUtil.createSMNodeToFlatSMGroupTable: START ***** \n"
            ));
        }
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!("***** State machine node table: ***** \n"));
        }
        if DEBUG_SMDUMP.clone() {
            BaseHashTable::dumpHashTable(&smNodeTable)?;
        }
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!("***** Adjacency Matrix: ***** \n"));
        }
        iTable = createAdjacencyTable(&smNodeTable, nStates)?;
        if DEBUG_SMDUMP.clone() {
            printAdjacencyTable(iTable.clone(), nStates)?;
        }
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!("***** Transitive Closure: ***** \n"));
        }
        transClosure = transitiveClosure(iTable, nStates)?;
        if DEBUG_SMDUMP.clone() {
            printAdjacencyTable(transClosure.clone(), nStates)?;
        }
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!("***** Initial States: ***** \n"));
        }
        initialStates = extractInitialStates(&smNodeTable)?;
        if DEBUG_SMDUMP.clone() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    List::map(initialStates.clone(), &move |__a0: metamodelica::Ref<
                        DAE::ComponentRef,
                    >| {
                        ComponentReferenceBasics::printComponentRefStr(&__a0)
                    })?,
                    literal!(", "),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!("***** Flat State Machine Groups: ***** \n"));
        }
        flatSMGroup = extractFlatSMGroup(&initialStates, transClosure, nStates)?;
        if DEBUG_SMDUMP.clone() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    List::map(flatSMGroup.clone(), &dumpFlatSMGroupStr)?,
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!("***** SM Node cref to SM Group cref mapping: ***** \n"));
        }
        smNodeToFlatSMGroup = List::fold(&flatSMGroup, &relateNodesToGroup, smNodeToFlatSMGroup)?;
        if DEBUG_SMDUMP.clone() {
            BaseHashTable::dumpHashTable(&smNodeToFlatSMGroup)?;
        }
        if DEBUG_SMDUMP.clone() {
            metamodelica::print(literal!(
                "***** InstStateMachineUtil.createSMNodeToFlatSMGroupTable: END ***** \n"
            ));
        }
    } else {
        smNodeToFlatSMGroup = HashTableCG::emptyHashTableSized(1);
    }
    Ok(smNodeToFlatSMGroup)
}

pub(crate) fn wrapSMCompsInFlatSMs(
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inDae1: DAE::DAElist,
    mut inDae2: DAE::DAElist,
    mut smNodeToFlatSMGroup: SMNodeToFlatSMGroupTable,
    mut smInitialCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(DAE::DAElist, DAE::DAElist)> {
    let mut outDae1: DAE::DAElist;
    let mut outDae2: DAE::DAElist;
    let mut elementLst1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elementLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smCompsLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smTransitionsLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut flatSmLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut flatSMsAndMergingEqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let DAE::DAE { elementLst: __pa0 } = inDae1;
    elementLst1 = metamodelica::Own::own(__pa0);
    (smCompsLst, otherLst1) = List::extractOnTrue(
        &elementLst1,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isSMComp(&__a0))
        },
    )?;
    let DAE::DAE { elementLst: __pa1 } = inDae2;
    elementLst2 = metamodelica::Own::own(__pa1);
    (smTransitionsLst, otherLst2) = List::extractOnTrue(&elementLst2, &move |__a0: metamodelica::Ref<
        DAE::Element,
    >| isSMStatement2(&__a0))?;
    flatSmLst = List::map2(
        smInitialCrefs,
        &createFlatSM,
        listAppend(smCompsLst, smTransitionsLst),
        smNodeToFlatSMGroup,
    )?;
    flatSMsAndMergingEqns = List::fold1(
        &flatSmLst,
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: metamodelica::List<InnerOuter::TopInstance>,
               __a2: metamodelica::List<metamodelica::Ref<DAE::Element>>| {
            mergeVariableDefinitions(&__a0, __a1, __a2)
        },
        inIH,
        metamodelica::nil(),
    )?;
    outDae1 = DAE::DAElist {
        elementLst: listAppend(flatSMsAndMergingEqns, otherLst1),
    };
    outDae2 = DAE::DAElist { elementLst: otherLst2 };
    Ok((outDae1, outDae2))
}

fn mergeVariableDefinitions(
    mut inFlatSM: &metamodelica::Ref<DAE::Element>,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStartElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outerOutputCrefToSMCompCref: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    let mut outerOutputCrefToInnerCref: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    let mut innerCrefToOuterOutputCrefs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    let mut hashEntries_outerOutputCrefToInnerCref: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>;
    let mut innerCrefToOuterOutputCrefs_der: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )> = metamodelica::nil();
    let mut innerCrefToOuterOutputCrefs_nonDer: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )> = metamodelica::nil();
    let mut uniqueHashValues: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut derCrefsAcc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut outerOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut derCrefsSet: (
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
    let mut emptyTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut mergeEqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut mergeEqns_der: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut aliasEqns_der: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut nOfHits: i32;
    let mut hasDer: bool;
    let mut ident: ArcStr;
    let mut dAElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inFlatSM)) {
        Deref @ DAE::Element::FLAT_SM { ident: __pa0, dAElist: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ident = metamodelica::Own::own(__pa0);
    dAElist = metamodelica::Own::own(__pa1);
    outerOutputCrefToSMCompCref = List::fold(
        &dAElist,
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    )>,
                >,
            ),
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        )| collectOuterOutputs(&__a0, __a1),
        HashTableCG::emptyHashTable(),
    )?;
    outerOutputCrefToInnerCref = List::fold1(
        &(BaseHashTable::hashTableKeyList(&outerOutputCrefToSMCompCref)?),
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>,
               __a1: metamodelica::List<InnerOuter::TopInstance>,
               __a2: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    )>,
                >,
            ),
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        )| matchOuterWithInner(__a0, &__a1, __a2),
        inIH,
        HashTableCG::emptyHashTable(),
    )?;
    hashEntries_outerOutputCrefToInnerCref = BaseHashTable::hashTableList(&outerOutputCrefToInnerCref)?;
    uniqueHashValues = List::unique(&(BaseHashTable::hashTableValueList(&outerOutputCrefToInnerCref)?));
    innerCrefToOuterOutputCrefs = List::fold1(
        &uniqueHashValues,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>,
               __a1: metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<DAE::ComponentRef>,
        )>,
               __a2: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )>,
                >,
            ),
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
                Arc<
                    dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr>
                        + 'static,
                >,
            ),
        )| collectCorrespondingKeys(__a0, &__a1, __a2),
        hashEntries_outerOutputCrefToInnerCref,
        HashTable3::emptyHashTable(),
    )?;
    emptyTree = openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY();
    let (DAE::DAE { elementLst: __pa2 }, _, _) = DAEUtil::traverseDAE(
        DAE::DAElist { elementLst: dAElist },
        emptyTree.clone(),
        (std::sync::Arc::new(traverserHelperSubsOuterByInnerExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<(
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::ComponentRef>,
                                    )>,
                                >,
                            ),
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
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >,
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<(
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::ComponentRef>,
                                    )>,
                                >,
                            ),
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
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >,
                            ),
                        ),
                    )> + 'static,
            >),
        outerOutputCrefToInnerCref,
    )?;
    dAElist = metamodelica::Own::own(__pa2);
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        crefs = BaseHashTable::hashTableKeyList(&outerOutputCrefToSMCompCref)?;
        for mut cref in &*crefs {
            nOfHits = 0;
            let (_, _, (_, (_, __pa3))) = DAEUtil::traverseDAE(
                DAE::DAElist {
                    elementLst: dAElist.clone(),
                },
                emptyTree.clone(),
                (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
                (
                    (std::sync::Arc::new(traversingCountDer)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    (metamodelica::Ref<DAE::ComponentRef>, i32),
                                ) -> Result<(
                                    metamodelica::Ref<DAE::Exp>,
                                    (metamodelica::Ref<DAE::ComponentRef>, i32),
                                )> + 'static,
                        >),
                    (cref.clone(), 0),
                ),
            )?;
            nOfHits = metamodelica::Own::own(__pa3);
            if nOfHits > 0 {
                derCrefsAcc = metamodelica::cons(cref.clone(), derCrefsAcc);
            }
        }
        derCrefsSet = HashSet::emptyHashSetSized(((derCrefsAcc).len() as i32));
        derCrefsSet = List::fold(
            &derCrefsAcc,
            &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1),
            derCrefsSet,
        )?;
        for mut hashEntry in &*BaseHashTable::hashTableList(&innerCrefToOuterOutputCrefs)? {
            (_, outerOutputCrefs) = hashEntry.clone();
            hasDer = List::any(
                &outerOutputCrefs,
                &({
                    let __pe_b1 = derCrefsSet.clone();
                    move |__pe_a0| BaseHashSet::has(__pe_a0, &__pe_b1)
                }),
            )?;
            if hasDer {
                innerCrefToOuterOutputCrefs_der =
                    metamodelica::cons(hashEntry.clone(), innerCrefToOuterOutputCrefs_der);
            } else {
                innerCrefToOuterOutputCrefs_nonDer =
                    metamodelica::cons(hashEntry.clone(), innerCrefToOuterOutputCrefs_nonDer);
            }
        }
        aliasEqns_der = List::flatten(List::map(innerCrefToOuterOutputCrefs_der.clone(), &move |__a0: (
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )| {
            freshAliasEqn_der(&__a0)
        })?)?;
        mergeEqns_der = listAppend(
            List::map(innerCrefToOuterOutputCrefs_der, &move |__a0: (
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )| {
                freshMergingEqn_der(&__a0)
            })?,
            aliasEqns_der,
        );
        mergeEqns = listAppend(
            List::map(innerCrefToOuterOutputCrefs_nonDer, &move |__a0: (
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )| {
                freshMergingEqn(&__a0)
            })?,
            mergeEqns_der,
        );
    } else {
        mergeEqns = List::map(
            BaseHashTable::hashTableList(&innerCrefToOuterOutputCrefs)?,
            &move |__a0: (
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )| freshMergingEqn(&__a0),
        )?;
    }
    outElementLst = listAppend(
        inStartElementLst,
        metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::FLAT_SM {
                ident: ident,
                dAElist: dAElist,
            }),
            mergeEqns,
        ),
    );
    Ok(outElementLst)
}

fn freshAliasEqn_der(
    mut inInnerCrefToOuterOutputCrefs: &(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut innerCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outerCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    (innerCref, outerCrefs) = inInnerCrefToOuterOutputCrefs.clone();
    ty = ComponentReference::crefLastType(&innerCref)?;
    outEqns = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
        for mut outerCref in (outerCrefs).into_iter().cloned() {
            let __x = metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: innerCref.clone(),
                    ty: ty.clone(),
                }),
                scalar: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: outerCref.clone(),
                    ty: ty.clone(),
                }),
                source: DAE::emptyElementSource().clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEqns)
}

fn freshMergingEqn_der(
    mut inInnerCrefToOuterOutputCrefs: &(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outEqn: metamodelica::Ref<DAE::Element>;
    let mut innerCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outerCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut outerCrefsStripped: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut outerCrefDers: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    (innerCref, outerCrefs) = inInnerCrefToOuterOutputCrefs.clone();
    ty = ComponentReference::crefLastType(&innerCref)?;
    outerCrefsStripped = List::map(outerCrefs.clone(), &move |__a0: metamodelica::Ref<
        DAE::ComponentRef,
    >| {
        ComponentReference::crefStripLastIdent(&__a0)
    })?;
    outerCrefDers = List::map(
        outerCrefs,
        &({
            let __pe_b0 = literal!("_der$");
            move |__pe_a1| ComponentReference::appendStringLastIdent(&__pe_b0, &__pe_a1)
        }),
    )?;
    exp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }),
        expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: innerCref.clone(),
            ty: ty.clone()
        })],
        attr: DAE::callAttrBuiltinReal().clone(),
    });
    outEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: mergingRhs_der(&outerCrefDers, &innerCref, ty)?,
        source: DAE::emptyElementSource().clone(),
    });
    Ok(outEqn)
}

fn mergingRhs_der(
    mut inOuterCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inInnerCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    let mut callAttributes: metamodelica::Ref<DAE::CallAttributes> = metamodelica::Ref::new(DAE::CallAttributes {
        ty: ty.clone(),
        tuple_: false,
        builtin: true,
        isImpure: false,
        isFunctionPointerCall: false,
        inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
        noReturn: DAE::NoReturn::RETURNS.clone(),
    });
    res = (::match_deref::match_deref! { match inOuterCrefs {
        Deref @ metamodelica::ListNode::Cons { head: outerCref, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut crefState: metamodelica::Ref<DAE::ComponentRef>;
            let mut outerCrefExp: metamodelica::Ref<DAE::Exp>;
            let mut crefStateExp: metamodelica::Ref<DAE::Exp>;
            let mut ifExp: metamodelica::Ref<DAE::Exp>;
            let mut expCond: metamodelica::Ref<DAE::Exp>;
            let mut expElse: metamodelica::Ref<DAE::Exp>;
            outerCrefExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(metamodelica::AsArg::as_arg(&outerCref))?;
            crefStateExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: crefState, ty: ty });
            expCond = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("activeState") }), expLst: list![crefStateExp], attr: callAttributes });
            expElse = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) });
            ifExp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: expCond, expThen: outerCrefExp, expElse: expElse });
            ifExp
        },
        Deref @ metamodelica::ListNode::Cons { head: outerCref, tail: rest } => {
            let mut crefState: metamodelica::Ref<DAE::ComponentRef>;
            let mut outerCrefExp: metamodelica::Ref<DAE::Exp>;
            let mut crefStateExp: metamodelica::Ref<DAE::Exp>;
            let mut ifExp: metamodelica::Ref<DAE::Exp>;
            let mut expCond: metamodelica::Ref<DAE::Exp>;
            let mut expElse: metamodelica::Ref<DAE::Exp>;
            outerCrefExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(metamodelica::AsArg::as_arg(&outerCref))?;
            crefStateExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: crefState, ty: ty.clone() });
            expCond = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("activeState") }), expLst: list![crefStateExp], attr: callAttributes });
            expElse = mergingRhs_der(rest, inInnerCref, ty)?;
            ifExp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: expCond, expThen: outerCrefExp, expElse: expElse });
            ifExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn traversingCountDer(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCref_HitCount: (metamodelica::Ref<DAE::ComponentRef>, i32),
) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, i32))> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outCref_HitCount: (metamodelica::Ref<DAE::ComponentRef>, i32);
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut hitCount: i32;
    (cref, hitCount) = inCref_HitCount.clone();
    (outExp, outCref_HitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&componentRef), &cref)?) => {
            (inExp, (cref.clone(), hitCount + 1))
        },
        _ => {
            (inExp, inCref_HitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outCref_HitCount))
}

fn freshMergingEqn(
    mut inInnerCrefToOuterOutputCrefs: &(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outEqn: metamodelica::Ref<DAE::Element>;
    let mut innerCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outerCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut outerCrefsStripped: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    (innerCref, outerCrefs) = inInnerCrefToOuterOutputCrefs.clone();
    ty = ComponentReference::crefLastType(&innerCref)?;
    outerCrefsStripped = List::map(outerCrefs.clone(), &move |__a0: metamodelica::Ref<
        DAE::ComponentRef,
    >| {
        ComponentReference::crefStripLastIdent(&__a0)
    })?;
    outEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: innerCref.clone(),
            ty: ty.clone(),
        }),
        scalar: mergingRhs(&outerCrefs, &innerCref, ty)?,
        source: DAE::emptyElementSource().clone(),
    });
    Ok(outEqn)
}

fn mergingRhs(
    mut inOuterCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inInnerCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    let mut callAttributes: metamodelica::Ref<DAE::CallAttributes> = metamodelica::Ref::new(DAE::CallAttributes {
        ty: ty.clone(),
        tuple_: false,
        builtin: true,
        isImpure: false,
        isFunctionPointerCall: false,
        inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
        noReturn: DAE::NoReturn::RETURNS.clone(),
    });
    res = (::match_deref::match_deref! { match inOuterCrefs {
        Deref @ metamodelica::ListNode::Cons { head: outerCref, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut crefState: metamodelica::Ref<DAE::ComponentRef>;
            let mut outerCrefExp: metamodelica::Ref<DAE::Exp>;
            let mut innerCrefExp: metamodelica::Ref<DAE::Exp>;
            let mut crefStateExp: metamodelica::Ref<DAE::Exp>;
            let mut ifExp: metamodelica::Ref<DAE::Exp>;
            let mut expCond: metamodelica::Ref<DAE::Exp>;
            let mut expElse: metamodelica::Ref<DAE::Exp>;
            outerCrefExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            innerCrefExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: inInnerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(metamodelica::AsArg::as_arg(&outerCref))?;
            crefStateExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: crefState, ty: ty });
            expCond = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("activeState") }), expLst: list![crefStateExp], attr: callAttributes.clone() });
            expElse = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("previous") }), expLst: list![innerCrefExp], attr: callAttributes });
            ifExp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: expCond, expThen: outerCrefExp, expElse: expElse });
            ifExp
        },
        Deref @ metamodelica::ListNode::Cons { head: outerCref, tail: rest } => {
            let mut crefState: metamodelica::Ref<DAE::ComponentRef>;
            let mut outerCrefExp: metamodelica::Ref<DAE::Exp>;
            let mut crefStateExp: metamodelica::Ref<DAE::Exp>;
            let mut ifExp: metamodelica::Ref<DAE::Exp>;
            let mut expCond: metamodelica::Ref<DAE::Exp>;
            let mut expElse: metamodelica::Ref<DAE::Exp>;
            outerCrefExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(metamodelica::AsArg::as_arg(&outerCref))?;
            crefStateExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: crefState, ty: ty.clone() });
            expCond = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("activeState") }), expLst: list![crefStateExp], attr: callAttributes });
            expElse = mergingRhs(rest, inInnerCref, ty)?;
            ifExp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: expCond, expThen: outerCrefExp, expElse: expElse });
            ifExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn collectCorrespondingKeys(
    mut inInnerCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inHashEntries: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
    mut inInnerCrefToOuterOutputCrefs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
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
            Arc<
                dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
        >,
    ),
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
        Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outInnerCrefToOuterOutputCrefs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    ) = inInnerCrefToOuterOutputCrefs;
    let mut outerRefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outerRefs = List::filterMap1(
        inHashEntries,
        &move |__a0: (
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<DAE::ComponentRef>,
        ),
               __a1: metamodelica::Ref<DAE::ComponentRef>| crefEqualTuple22(__a0, &__a1),
        inInnerCref.clone(),
    );
    outInnerCrefToOuterOutputCrefs =
        BaseHashTable::addUnique((inInnerCref, outerRefs), &outInnerCrefToOuterOutputCrefs)?;
    Ok(outInnerCrefToOuterOutputCrefs)
}

fn crefEqualTuple22(
    mut inHashEntry: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    ),
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut isEqual: bool;
    let mut tuple22: metamodelica::Ref<DAE::ComponentRef>;
    tuple22 = Util::tuple22(inHashEntry.clone());
    isEqual = ComponentReferenceBasics::crefEqual(&tuple22, inCref)?;
    if !(isEqual) {
        return Err("fail");
    }
    outCref = Util::tuple21(inHashEntry);
    Ok(outCref)
}

fn traverserHelperSubsOuterByInnerExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inOuterToInner: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outOuterToInner: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    (outExp, outOuterToInner) =
        Expression::traverseExpBottomUp(inExp, &traverserHelperSubsOuterByInner, inOuterToInner)?;
    Ok((outExp, outOuterToInner))
}

fn traverserHelperSubsOuterByInner(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inOuterToInner: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outOuterToInner: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    (outExp, outOuterToInner) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef, ty }, tail: Deref @ metamodelica::ListNode::Nil }, attr } if (BaseHashTable::hasKey(componentRef.clone(), &inOuterToInner)?) => {
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("previous") }), expLst: list![metamodelica::Ref::new(DAE::Exp::CREF { componentRef: BaseHashTable::get(componentRef.clone(), &inOuterToInner)?, ty: ty.clone() })], attr: attr.clone() }), inOuterToInner.clone())
        },
        _ => {
            (inExp, inOuterToInner.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outOuterToInner))
}

fn matchOuterWithInner(
    mut inOuterCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inOuterCrefToInnerCref: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<DAE::ComponentRef>,
            )>,
        >,
    ),
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outOuterCrefToInnerCref: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    ) = inOuterCrefToInnerCref;
    let mut crefIdent: metamodelica::Ref<DAE::ComponentRef>;
    let mut crefFound: metamodelica::Ref<DAE::ComponentRef>;
    let mut strippedCref1: metamodelica::Ref<DAE::ComponentRef>;
    let mut strippedCref2: metamodelica::Ref<DAE::ComponentRef>;
    crefIdent = ComponentReferenceBasics::crefLastCref(&inOuterCref)?;
    strippedCref1 = ComponentReference::crefStripLastIdent(&inOuterCref)?;
    strippedCref2 = if (ComponentReference::crefDepth(&strippedCref1)? >= 2) {
        ComponentReference::joinCrefs(
            &(ComponentReference::crefStripLastIdent(&strippedCref1)?),
            crefIdent.clone(),
        )?
    } else {
        crefIdent.clone()
    };
    crefFound = findInner(strippedCref2, crefIdent, inIH)?;
    outOuterCrefToInnerCref = BaseHashTable::addUnique((inOuterCref, crefFound), &outOuterCrefToInnerCref)?;
    Ok(outOuterCrefToInnerCref)
}

fn findInner(
    mut inCrefTest: metamodelica::Ref<DAE::ComponentRef>,
    mut inCrefIdent: metamodelica::Ref<DAE::ComponentRef>,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCrefFound: metamodelica::Ref<DAE::ComponentRef>;
    let mut strippedCref1: metamodelica::Ref<DAE::ComponentRef>;
    let mut strippedCref2: metamodelica::Ref<DAE::ComponentRef>;
    let mut ht: InnerOuter::InstHierarchyHashTable;
    let InnerOuter::TOP_INSTANCE { ht: __pa0, .. } = (inIH).head().cloned()?;
    ht = metamodelica::Own::own(__pa0);
    match '__try1: {
        unwrap_break_err!(InnerOuter::get(&inCrefTest, &ht), '__try1);
        outCrefFound = inCrefTest.clone();
        Ok::<_, &'static str>((outCrefFound.clone(),))
    } {
        Ok((__try1_o0,)) => {
            outCrefFound = __try1_o0;
        }
        Err(_) => {
            strippedCref1 = ComponentReference::crefStripLastIdent(&inCrefTest)?;
            strippedCref2 = if (ComponentReference::crefDepth(&strippedCref1)? >= 2) {
                ComponentReference::joinCrefs(
                    &(ComponentReference::crefStripLastIdent(&strippedCref1)?),
                    inCrefIdent.clone(),
                )?
            } else {
                inCrefIdent.clone()
            };
            outCrefFound = findInner(strippedCref2.clone(), inCrefIdent.clone(), inIH)?;
        }
    }
    Ok(outCrefFound)
}

fn collectOuterOutputs(
    mut inElem: &metamodelica::Ref<DAE::Element>,
    mut inOuterAcc: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<DAE::ComponentRef>,
            )>,
        >,
    ),
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outOuterAcc: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    ) = inOuterAcc.clone();
    let mut outerOutputs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outerOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut outerOutputCrefToSMCompCref: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>;
    let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut dAElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outOuterAcc = (match &**inElem {
        DAE::Element::SM_COMP {
            componentRef: __esc_componentRef,
            dAElist: __esc_dAElist,
        } => {
            componentRef = (*__esc_componentRef).clone();
            dAElist = (*__esc_dAElist).clone();
            outerOutputs = List::filterOnTrue(
                dAElist.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(isOuterOutput(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
            )?;
            outerOutputCrefs = List::map(outerOutputs, &move |__a0: metamodelica::Ref<DAE::Element>| {
                DAEUtil::varCref(&__a0)
            })?;
            outerOutputCrefToSMCompCref = List::map(
                outerOutputCrefs,
                &({
                    let __pe_b1 = componentRef.clone();
                    move |__pe_a0| Ok(Util::makeTuple(__pe_a0, __pe_b1.clone()))
                }),
            )?;
            List::fold(
                &outerOutputCrefToSMCompCref,
                &move |__a0: _, __a1: _| BaseHashTable::addUnique(__a0, &__a1),
                outOuterAcc,
            )?
        }
        _ => inOuterAcc,
    });
    Ok(outOuterAcc)
}

fn isOuterOutput(mut inElem: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outB: bool;
    outB = (match &**inElem {
        DAE::Element::VAR {
            direction: DAE::VarDirection::OUTPUT { .. },
            innerOuter: Absyn::InnerOuter::OUTER { .. },
            ..
        } => true,
        DAE::Element::VAR {
            direction: DAE::VarDirection::OUTPUT { .. },
            innerOuter: Absyn::InnerOuter::INNER_OUTER { .. },
            ..
        } => true,
        _ => false,
    });
    outB
}

fn createFlatSM(
    mut smInitialCref: metamodelica::Ref<DAE::ComponentRef>,
    mut smElemsLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut smNodeToFlatSMGroup: SMNodeToFlatSMGroupTable,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut flatSM: metamodelica::Ref<DAE::Element>;
    let mut smElemsInFlatSM: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    smElemsInFlatSM = List::filter2OnTrue(
        smElemsLst,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>,
                  __a1: metamodelica::Ref<DAE::ComponentRef>,
                  __a2: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        )>,
                    >,
                ),
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
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                ),
            )| isInFlatSM(__a0, &__a1, &__a2),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Element>,
                        metamodelica::Ref<DAE::ComponentRef>,
                        (
                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<(
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::ComponentRef>,
                                    )>,
                                >,
                            ),
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
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
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
        smInitialCref.clone(),
        smNodeToFlatSMGroup,
    )?;
    flatSM = metamodelica::Ref::new(DAE::Element::FLAT_SM {
        ident: ComponentReferenceBasics::printComponentRefStr(&smInitialCref)?,
        dAElist: smElemsInFlatSM,
    });
    Ok(flatSM)
}

fn isInFlatSM(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut smInitialCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut smNodeToFlatSMGroup: &SMNodeToFlatSMGroupTable,
) -> Result<bool> {
    let mut outResult: bool;
    let mut crefCorrespondingFlatSMGroup: metamodelica::Ref<DAE::ComponentRef>;
    crefCorrespondingFlatSMGroup = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: cref1, .. } if (BaseHashTable::hasKey(cref1.clone(), smNodeToFlatSMGroup)?) => {
            BaseHashTable::get(cref1.clone(), smNodeToFlatSMGroup)?
        },
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: _ }, .. }, .. } if (BaseHashTable::hasKey(cref1.clone(), smNodeToFlatSMGroup)?) => {
            BaseHashTable::get(cref1.clone(), smNodeToFlatSMGroup)?
        },
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } if (BaseHashTable::hasKey(cref1.clone(), smNodeToFlatSMGroup)?) => {
            BaseHashTable::get(cref1.clone(), smNodeToFlatSMGroup)?
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstStateMachineUtil.isInFlatSM failed: Hash table lookup failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![inElement]))?); ArcStr::from(__mm_s) })?;
            BaseHashTable::dumpHashTableStatistics(smNodeToFlatSMGroup);
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult = ComponentReferenceBasics::crefEqual(&crefCorrespondingFlatSMGroup, smInitialCref)?;
    Ok(outResult)
}

fn isSMComp(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outResult: bool;
    outResult = (match &**inElement {
        DAE::Element::SM_COMP {
            componentRef: _,
            dAElist: _,
        } => true,
        _ => false,
    });
    outResult
}

fn relateNodesToGroup(
    mut flatSMGroup: FlatSMGroup,
    mut inNodeToGroup: SMNodeToFlatSMGroupTable,
) -> Result<SMNodeToFlatSMGroupTable> {
    let mut outNodeToGroup: SMNodeToFlatSMGroupTable = inNodeToGroup;
    let mut nodeGroup: metamodelica::Array<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>;
    let mut initState: metamodelica::Ref<DAE::ComponentRef>;
    let mut states: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let FlatSMGroup {
        initState: __pa0,
        states: __pa1,
    } = flatSMGroup;
    initState = metamodelica::Own::own(__pa0);
    states = metamodelica::Own::own(__pa1);
    nodeGroup = Array::map(
        states.clone(),
        &({
            let __pe_b1 = initState;
            move |__pe_a0| Ok(Util::makeTuple(__pe_a0, __pe_b1.clone()))
        }),
    )?;
    outNodeToGroup = Array::fold(nodeGroup.clone(), &BaseHashTable::add, outNodeToGroup)?;
    Ok(outNodeToGroup)
}

fn extractFlatSMGroup(
    mut initialStates: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut iTable: AdjacencyTable,
    mut nStates: i32,
) -> Result<metamodelica::List<FlatSMGroup>> {
    let mut flatSMGroup: metamodelica::List<FlatSMGroup>;
    let mut cref2index: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut entries: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
    let mut i2cref: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    let mut members: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut membersArr: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut memberSet: (
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
    let mut n: i32;
    let mut i: i32;
    let mut j: i32 = 0;
    let AdjacencyTable {
        cref2index: __pa0,
        adjacency: __pa1,
    } = iTable;
    cref2index = metamodelica::Own::own(__pa0);
    adjacency = metamodelica::Own::own(__pa1);
    n = BaseHashTable::hashTableCurrentSize(&cref2index);
    assert!(
        n == nStates,
        "{}",
        &*literal!("Value of nStates needs to be equal to number of modes within state table argument.")
    );
    entries = BaseHashTable::hashTableList(&cref2index)?;
    entries = List::sort(
        entries,
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<DAE::ComponentRef>, i32),
                  __a1: (metamodelica::Ref<DAE::ComponentRef>, i32)|
                  -> metamodelica::Result<_> { ::std::result::Result::Ok(crefIndexCmp(&__a0, &__a1)) },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    i2cref = metamodelica::arrayFromVec(
        List::map(entries, &fnptr!(Util::tuple21, _))?
            .into_iter()
            .cloned()
            .collect(),
    );
    flatSMGroup = metamodelica::nil();
    for mut cref in &**initialStates {
        let mut cref = cref.clone();
        i = BaseHashTable::get(cref.clone(), &cref2index)?;
        members = metamodelica::nil();
        for mut j in 1..=n {
            if metamodelica::arrayGet(metamodelica::arrayGet(adjacency.clone(), i)?, j)? {
                members = metamodelica::cons(metamodelica::arrayGet(i2cref.clone(), j)?, members);
            }
        }
        memberSet = HashSet::emptyHashSetSized(((members).len() as i32));
        memberSet = List::fold(
            &members,
            &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1),
            memberSet,
        )?;
        memberSet = BaseHashSet::delete(cref.clone(), &memberSet)?;
        membersArr = metamodelica::arrayFromVec(
            metamodelica::cons(cref.clone(), BaseHashSet::hashSetList(&memberSet)?)
                .into_iter()
                .cloned()
                .collect(),
        );
        flatSMGroup = metamodelica::cons(
            FlatSMGroup {
                initState: cref,
                states: membersArr.clone(),
            },
            flatSMGroup,
        );
    }
    Ok(flatSMGroup)
}

pub(crate) fn dumpFlatSMGroupStr(mut flatA: FlatSMGroup) -> Result<ArcStr> {
    let mut flatStr: ArcStr;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut initialStateStr: ArcStr;
    let mut statesStr: ArcStr;
    let mut statesStrs: metamodelica::List<ArcStr>;
    let mut initState: metamodelica::Ref<DAE::ComponentRef>;
    let mut states: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let FlatSMGroup {
        initState: __pa0,
        states: __pa1,
    } = flatA;
    initState = metamodelica::Own::own(__pa0);
    states = metamodelica::Own::own(__pa1);
    initialStateStr = ComponentReferenceBasics::printComponentRefStr(&initState)?;
    crefs = states
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    statesStrs = List::map(crefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
        ComponentReferenceBasics::printComponentRefStr(&__a0)
    })?;
    statesStr = stringDelimitList(statesStrs, literal!(", "));
    flatStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*initialStateStr);
        __mm_s.push_str(&*literal!("( states("));
        __mm_s.push_str(&*statesStr);
        __mm_s.push_str(&*literal!("))"));
        ArcStr::from(__mm_s)
    };
    Ok(flatStr)
}

fn extractInitialStates(
    mut smNodeTable: &SMNodeTable,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut initialStates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut entries: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SMNode)>;
    let mut e: (metamodelica::Ref<DAE::ComponentRef>, SMNode) = (
        metamodelica::Ref::new(DAE::ComponentRef::WILD),
        <SMNode as ::std::default::Default>::default(),
    );
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut smNode: SMNode;
    let mut isInitial: bool;
    entries = BaseHashTable::hashTableList(smNodeTable)?;
    initialStates = metamodelica::nil();
    for mut e in &*entries {
        let mut e = e.clone();
        (cref, smNode) = e;
        let SMNode { isInitial: __pa0, .. } = smNode;
        isInitial = metamodelica::Own::own(__pa0);
        if isInitial {
            initialStates = metamodelica::cons(cref, initialStates);
        }
    }
    Ok(initialStates)
}

fn transitiveClosure(mut iTable: AdjacencyTable, mut nStates: i32) -> Result<AdjacencyTable> {
    let mut transClosure: AdjacencyTable;
    let mut cref2index: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut n: i32;
    let mut k: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let AdjacencyTable {
        cref2index: __pa0,
        adjacency: __pa1,
    } = iTable;
    cref2index = metamodelica::Own::own(__pa0);
    adjacency = metamodelica::Own::own(__pa1);
    n = BaseHashTable::hashTableCurrentSize(&cref2index);
    assert!(
        n == nStates,
        "{}",
        &*literal!("Value of nStates needs to be equal to number of states within state table argument.")
    );
    for mut k in 1..=n {
        for mut i in 1..=n {
            if metamodelica::arrayGet(metamodelica::arrayGet(adjacency.clone(), i)?, k)? {
                for mut j in 1..=n {
                    if metamodelica::arrayGet(metamodelica::arrayGet(adjacency.clone(), k)?, j)? {
                        metamodelica::arrayUpdate(metamodelica::arrayGet(adjacency.clone(), i)?, j, true)?;
                    }
                }
            }
        }
    }
    transClosure = AdjacencyTable {
        cref2index: cref2index,
        adjacency: adjacency.clone(),
    };
    Ok(transClosure)
}

fn createAdjacencyTable(mut smNodes: &SMNodeTable, mut nStates: i32) -> Result<AdjacencyTable> {
    let mut iTable: AdjacencyTable;
    let mut cref2index: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut n: i32;
    let mut m: i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut edges: (
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
    let mut crefs1: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefs2: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    crefs1 = metamodelica::arrayFromVec(BaseHashTable::hashTableKeyList(smNodes)?.into_iter().cloned().collect());
    n = metamodelica::arrayLength(crefs1.clone());
    cref2index = HashTable::emptyHashTableSized(n);
    assert!(
        n == nStates,
        "{}",
        &*literal!("Value of nStates needs to be equal to number of modes within mode table argument.")
    );
    adjacency = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<metamodelica::Array<bool>> = metamodelica::nil();
            for mut i in (1..=n).into_iter() {
                let __x = arrayCreate(n, false);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    for mut i in 1..=n {
        cref2index = BaseHashTable::addNoUpdCheck(
            (
                ({
                    let __elt = (*metamodelica::index_checked(&crefs1.borrow(), i)?).clone();
                    __elt
                }),
                i,
            ),
            &cref2index,
        )?;
    }
    for mut i in 1..=n {
        let SMNode { edges: __pa0, .. } = BaseHashTable::get(
            ({
                let __elt = (*metamodelica::index_checked(&crefs1.borrow(), i)?).clone();
                __elt
            }),
            smNodes,
        )?;
        edges = metamodelica::Own::own(__pa0);
        crefs2 = metamodelica::arrayFromVec(BaseHashSet::hashSetList(&edges)?.into_iter().cloned().collect());
        m = metamodelica::arrayLength(crefs2.clone());
        for mut j in 1..=m {
            cref = ({
                let __elt = (*metamodelica::index_checked(&crefs2.borrow(), j)?).clone();
                __elt
            });
            k = BaseHashTable::get(cref, &cref2index)?;
            metamodelica::arrayUpdate(metamodelica::arrayGet(adjacency.clone(), i)?, k, true)?;
        }
    }
    iTable = AdjacencyTable {
        cref2index: cref2index,
        adjacency: adjacency.clone(),
    };
    Ok(iTable)
}

fn printAdjacencyTable(mut iTable: AdjacencyTable, mut nStates: i32) -> Result<()> {
    let mut cref2index: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut entries: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
    let mut entry: (metamodelica::Ref<DAE::ComponentRef>, i32) = (metamodelica::Ref::new(DAE::ComponentRef::WILD), 0);
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut n: i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut padn: i32;
    let mut r#str: ArcStr;
    let mut pads: ArcStr;
    let mut b: bool;
    let AdjacencyTable {
        cref2index: __pa0,
        adjacency: __pa1,
    } = iTable;
    cref2index = metamodelica::Own::own(__pa0);
    adjacency = metamodelica::Own::own(__pa1);
    entries = BaseHashTable::hashTableList(&cref2index)?;
    n = ((entries).len() as i32);
    assert!(
        n == nStates,
        "{}",
        &*literal!("Value of nStates needs to be equal to number of modes within state table argument.")
    );
    entries = List::sort(
        entries,
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<DAE::ComponentRef>, i32),
                  __a1: (metamodelica::Ref<DAE::ComponentRef>, i32)|
                  -> metamodelica::Result<_> { ::std::result::Result::Ok(crefIndexCmp(&__a0, &__a1)) },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    for mut entry in &*entries {
        let mut entry = entry.clone();
        (cref, i) = entry;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cref)?);
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    pads = literal!(" ");
    padn = 8;
    r#str = Util::stringPadRight(literal!("i"), padn, pads.clone());
    for mut i in 1..=n {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*Util::stringPadLeft(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*intString(i));
                    __mm_s.push_str(&*literal!(","));
                    ArcStr::from(__mm_s)
                },
                padn,
                pads.clone(),
            ));
            ArcStr::from(__mm_s)
        };
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    for mut i in 1..=n {
        r#str = Util::stringPadRight(intString(i), padn, pads.clone());
        for mut j in 1..=n {
            b = metamodelica::arrayGet(metamodelica::arrayGet(adjacency.clone(), i)?, j)?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*Util::stringPadLeft(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*boolString(b));
                        __mm_s.push_str(&*literal!(","));
                        ArcStr::from(__mm_s)
                    },
                    padn,
                    pads.clone(),
                ));
                ArcStr::from(__mm_s)
            };
        }
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn crefIndexCmp(
    mut inElement1: &(metamodelica::Ref<DAE::ComponentRef>, i32),
    mut inElement2: &(metamodelica::Ref<DAE::ComponentRef>, i32),
) -> bool {
    let mut inRes: bool;
    let mut i1: i32;
    let mut i2: i32;
    (_, i1) = inElement1.clone();
    (_, i2) = inElement2.clone();
    inRes = i1 > i2;
    inRes
}

pub(crate) fn getSMNodeTable(
    mut elementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<SMNodeTable> {
    let mut smNodeTable: SMNodeTable;
    let mut elementLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elementLst2 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
        for mut e in (elementLst).into_iter().cloned() {
            if !(isSMStatement2(&(e.clone()))?) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !((elementLst2).is_empty()) {
        smNodeTable = List::fold(
            &elementLst2,
            &move |__a0: metamodelica::Ref<DAE::Element>,
                   __a1: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, SMNode)>>,
                ),
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
                    Arc<dyn ::std::ops::Fn(SMNode) -> Result<ArcStr> + 'static>,
                ),
            )| extractSMStates2(&__a0, __a1),
            HashTableSM1::emptyHashTable(),
        )?;
    } else {
        smNodeTable = HashTableSM1::emptyHashTableSized(1);
    }
    Ok(smNodeTable)
}

fn isSMStatement(mut inElement: &metamodelica::Ref<SCode::Equation>) -> Result<bool> {
    let mut outIsSMStatement: bool;
    outIsSMStatement = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. }, .. }, .. } => {
            (metamodelica::stringEq(&name, &(literal!("transition"))) || metamodelica::stringEq(&name, &(literal!("initialState")))) && Config::synchronousFeaturesAllowed()?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsSMStatement)
}

fn isSMStatement2(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<bool> {
    let mut outIsSMStatement: bool;
    outIsSMStatement = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, .. } => {
            (metamodelica::stringEq(&name, &(literal!("transition"))) || metamodelica::stringEq(&name, &(literal!("initialState")))) && Config::synchronousFeaturesAllowed()?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsSMStatement)
}

fn extractSMStates2(mut inElement: &metamodelica::Ref<DAE::Element>, mut inTable: SMNodeTable) -> Result<SMNodeTable> {
    let mut outTable: SMNodeTable = inTable;
    outTable = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: _ } }, .. }, .. } => {
            let mut smnode1: SMNode;
            let mut smnode2: SMNode;
            let mut isInitial1: bool;
            let mut isInitial2: bool;
            let mut edges1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut edges2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            smnode1 = if (BaseHashTable::hasKey(cref1.clone(), &outTable)?) {BaseHashTable::get(cref1.clone(), &outTable)?} else {SMNode { componentRef: cref1.clone(), isInitial: false, edges: HashSet::emptyHashSet() }};
            let SMNode { componentRef: _, isInitial: __pa0, edges: __pa1 } = smnode1;
            isInitial1 = metamodelica::Own::own(__pa0);
            edges1 = metamodelica::Own::own(__pa1);
            edges1 = BaseHashSet::add(cref1.clone(), &edges1)?;
            edges1 = BaseHashSet::add(cref2.clone(), &edges1)?;
            smnode1 = SMNode { componentRef: cref1.clone(), isInitial: isInitial1, edges: edges1 };
            outTable = BaseHashTable::add((cref1.clone(), smnode1), outTable)?;
            smnode2 = if (BaseHashTable::hasKey(cref2.clone(), &outTable)?) {BaseHashTable::get(cref2.clone(), &outTable)?} else {SMNode { componentRef: cref2.clone(), isInitial: false, edges: HashSet::emptyHashSet() }};
            let SMNode { componentRef: _, isInitial: __pa2, edges: __pa3 } = smnode2;
            isInitial2 = metamodelica::Own::own(__pa2);
            edges2 = metamodelica::Own::own(__pa3);
            edges2 = BaseHashSet::add(cref1.clone(), &edges2)?;
            edges2 = BaseHashSet::add(cref2.clone(), &edges2)?;
            smnode2 = SMNode { componentRef: cref2.clone(), isInitial: isInitial2, edges: edges2 };
            outTable = BaseHashTable::add((cref2.clone(), smnode2), outTable)?;
            outTable
        },
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
            let mut smnode1: SMNode;
            let mut edges1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            smnode1 = if (BaseHashTable::hasKey(cref1.clone(), &outTable)?) {BaseHashTable::get(cref1.clone(), &outTable)?} else {SMNode { componentRef: cref1.clone(), isInitial: true, edges: HashSet::emptyHashSet() }};
            let SMNode { componentRef: _, isInitial: _, edges: __pa0 } = smnode1;
            edges1 = metamodelica::Own::own(__pa0);
            edges1 = BaseHashSet::add(cref1.clone(), &edges1)?;
            smnode1 = SMNode { componentRef: cref1.clone(), isInitial: true, edges: edges1 };
            outTable = BaseHashTable::add((cref1.clone(), smnode1), outTable)?;
            outTable
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTable)
}

pub(crate) fn getSMStatesInContext(
    mut eqns: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inPrefix: DAE::Prefix,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut states: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut initialStates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut eqns1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut statesLL: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
    let mut initialStatesCR: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    let mut statesCR: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    eqns1 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
        for mut eq in (eqns).into_iter().cloned() {
            if !(isSMStatement(&(eq.clone()))?) {
                continue;
            }
            let __x = eq.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    initialStatesCR = List::filterMap(&eqns1, &move |__a0: metamodelica::Ref<SCode::Equation>| {
        extractInitialSMStates(&__a0)
    });
    initialStates = List::map(initialStatesCR, &move |__a0: metamodelica::Ref<Absyn::ComponentRef>| {
        ComponentReference::toExpCref(&__a0)
    })?;
    initialStates = List::map1(initialStates, &prefixCrefNoContext2, inPrefix.clone())?;
    statesLL = List::map(
        eqns1,
        &move |__a0: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(extractSMStates(&__a0))
        },
    )?;
    statesCR = List::flatten(statesLL)?;
    states = List::map(statesCR, &move |__a0: metamodelica::Ref<Absyn::ComponentRef>| {
        ComponentReference::toExpCref(&__a0)
    })?;
    states = List::map(
        states,
        &({
            let __pe_b0 = inPrefix;
            move |__pe_a1| PrefixUtil::prefixCrefNoContext(__pe_b0.clone(), __pe_a1)
        }),
    )?;
    Ok((states, initialStates))
}

fn prefixCrefNoContext2(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inPre: DAE::Prefix,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = PrefixUtil::prefixCrefNoContext(inPre, inCref)?;
    Ok(outCref)
}

fn extractInitialSMStates(
    mut inElement: &metamodelica::Ref<SCode::Equation>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outElement: metamodelica::Ref<Absyn::ComponentRef>;
    outElement = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initialState", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, .. } => {
            cref1.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outElement)
}

fn extractSMStates(
    mut inElement: &metamodelica::Ref<SCode::Equation>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outElement: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outElement = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "transition", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref2 }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }, .. } => {
            list![cref1.clone(), cref2.clone()]
        },
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initialState", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, .. } => {
            list![cref1.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElement
}
