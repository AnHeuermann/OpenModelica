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

use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnections;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_util::Debug;
use openmodelica_util::DisjointSets;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/// a tuple with two crefs and equation(s) for calling the equalityConstraint function call
pub type FlatEdge = NFConnections::BrokenEdge;

/// a lit of broken edges
pub type FlatEdges = metamodelica::List<NFConnections::BrokenEdge>;

/// an edge is a tuple with two component references
pub type Edge = (
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
);

/// A list of edges
pub type Edges = metamodelica::List<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)>;

/// root defined with Connection.root
pub type DefiniteRoot = metamodelica::Ref<ComponentRef::NFComponentRef>;

/// roots defined with Connection.root
pub type DefiniteRoots = metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;

/// roots defined with Connection.uniqueRoot
pub type UniqueRoots = metamodelica::List<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<Expression::NFExpression>,
)>;

/// potential root defined with Connections.potentialRoot
pub type PotentialRoot = (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real);

/// potential roots defined with Connections.potentialRoot
pub type PotentialRoots = metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real)>;

/// Input structure for connection breaking algorithm. It is collected during instantiation phase.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFOCConnectionGraph {
    pub updateGraph: bool,
    /// Roots defined with Connection.root
    pub definiteRoots: DefiniteRoots,
    /// Roots defined with Connection.potentialRoot
    pub potentialRoots: PotentialRoots,
    /// Roots defined with Connection.uniqueRoot
    pub uniqueRoots: UniqueRoots,
    /// Edges defined with Connection.branch
    pub branches: Edges,
    /// Edges defined with connect statement
    pub connections: FlatEdges,
}

impl metamodelica::gc::MMTrace for NFOCConnectionGraph {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.updateGraph, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.definiteRoots, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.potentialRoots, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.uniqueRoots, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.branches, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.connections, __mmv)?;
        Ok(())
    }
}
impl Default for NFOCConnectionGraph {
    fn default() -> Self {
        Self {
            updateGraph: Default::default(),
            definiteRoots: Default::default(),
            potentialRoots: Default::default(),
            uniqueRoots: Default::default(),
            branches: Default::default(),
            connections: Default::default(),
        }
    }
}

pub type GRAPH = NFOCConnectionGraph;

thread_local! { static __EMPTY_TLS: NFOCConnectionGraph = NFOCConnectionGraph { updateGraph: true, definiteRoots: metamodelica::nil(), potentialRoots: metamodelica::nil(), uniqueRoots: metamodelica::nil(), branches: metamodelica::nil(), connections: metamodelica::nil() }; }
pub(crate) fn EMPTY() -> NFOCConnectionGraph {
    __EMPTY_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ConnectionsOperator {
    BRANCH = 1,
    ROOT = 2,
    POTENTIAL_ROOT = 3,
    IS_ROOT = 4,
    ROOTED = 5,
    UNIQUE_ROOT = 6,
    UNIQUE_ROOT_INDICES = 7,
    NOT_OPERATOR = 8,
}
impl PartialOrd for ConnectionsOperator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ConnectionsOperator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ConnectionsOperator {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub type CrefCrefTable = metamodelica::Ref<
    UnorderedMap::UnorderedMap<
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<ComponentRef::NFComponentRef>,
    >,
>;

pub type CrefIndexTable =
    metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;

pub type CrefRootsTable = metamodelica::Ref<
    UnorderedMap::UnorderedMap<
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
>;

pub mod CrefSets {
    use super::*;
    pub(crate) fn EntryHash(mut entry: Entry) -> Result<i32> {
        let mut hash: i32;
        hash = ComponentRef::hash(&entry)?;
        Ok(hash)
    }

    pub(crate) fn EntryEqual(mut entry1: Entry, mut entry2: Entry) -> Result<bool> {
        let mut isEqual: bool;
        isEqual = ComponentRef::isEqual(&entry1, &entry2)?;
        Ok(isEqual)
    }

    pub(crate) fn EntryString(mut entry: Entry) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = ComponentRef::toString(&entry)?;
        Ok(r#str)
    }

    pub type Entry = metamodelica::Ref<ComponentRef::NFComponentRef>;

    pub type IndexTable =
        metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;

    /// This is a disjoint sets data structure. The nodes are stored in an array of
    ///   Integers. The root elements of a set is given a negative value that
    ///   corresponds to its rank, while other elements are given positive values that
    ///   corresponds to the index of their parent in the array. The hashtable is used
    ///   to look up the array index of a entry, and is also used to store the entries.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Sets {
        /// An array of nodes
        pub nodes: metamodelica::Array<i32>,
        /// An Entry->Integer table.
        pub elements: IndexTable,
        /// The number of nodes stored in the sets.
        pub nodeCount: i32,
    }

    impl metamodelica::gc::MMTrace for Sets {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.nodes, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.elements, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nodeCount, __mmv)?;
            Ok(())
        }
    }
    impl Default for Sets {
        fn default() -> Self {
            Self {
                nodes: Default::default(),
                elements: Default::default(),
                nodeCount: Default::default(),
            }
        }
    }

    pub type DISJOINT_SETS = Sets;

    pub(crate) fn add(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
        let mut sets: Sets = sets;
        let mut index: i32;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut node_count: i32;
        let Sets {
            nodes: __pa0,
            elements: __pa1,
            nodeCount: __pa2,
        } = sets;
        nodes = metamodelica::Own::own(__pa0);
        elements = metamodelica::Own::own(__pa1);
        node_count = metamodelica::Own::own(__pa2);
        index = node_count + 1;
        if index > metamodelica::arrayLength(nodes.clone()) {
            nodes = Array::expand(
                ((intReal(index) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32),
                nodes.clone(),
                -1,
            )?;
        }
        UnorderedMap::addNew(entry, index, elements.clone())?;
        sets = Sets {
            nodes: nodes.clone(),
            elements: elements,
            nodeCount: index,
        };
        Ok((sets, index))
    }

    pub(crate) fn addList(
        mut entries: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut sets: Sets,
    ) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut node_count: i32;
        let mut sz: i32;
        let mut index: i32;
        let Sets {
            nodes: __pa0,
            elements: __pa1,
            nodeCount: __pa2,
        } = sets;
        nodes = metamodelica::Own::own(__pa0);
        elements = metamodelica::Own::own(__pa1);
        node_count = metamodelica::Own::own(__pa2);
        sz = ((entries).len() as i32);
        index = node_count + 1;
        node_count = node_count + sz;
        if node_count > metamodelica::arrayLength(nodes.clone()) {
            nodes = Array::expand(
                ((intReal(node_count) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32),
                nodes.clone(),
                -1,
            )?;
        }
        for mut e in &**entries {
            UnorderedMap::addNew(e.clone(), index, elements.clone())?;
            index = index + 1;
        }
        sets = Sets {
            nodes: nodes.clone(),
            elements: elements,
            nodeCount: node_count,
        };
        Ok(sets)
    }

    pub(crate) fn contains(mut entry: Entry, mut sets: &Sets) -> Result<bool> {
        let mut found: bool;
        found = (UnorderedMap::get(entry, sets.elements.clone())?).is_some();
        Ok(found)
    }

    pub(crate) fn emptySets(mut setCount: i32) -> Sets {
        let mut sets: Sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut sz: i32;
        sz = std::cmp::max(setCount, 3);
        nodes = arrayCreate(sz, -1);
        elements = UnorderedMap::new(
            (std::sync::Arc::new(EntryHash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(EntryEqual)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        sets = Sets {
            nodes: nodes.clone(),
            elements: elements,
            nodeCount: 0,
        };
        sets
    }

    pub(crate) fn extractSets(
        mut sets: &Sets,
    ) -> Result<(
        metamodelica::Array<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        Sets,
    )> {
        let mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
        let mut assignedSets: Sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut set_idx: i32 = 0;
        let mut idx: i32;
        let mut entries: metamodelica::Array<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>;
        let mut e: Entry;
        nodes = sets.nodes.clone();
        for mut i in 1..=sets.nodeCount.clone() {
            if ({
                let __elt = (*metamodelica::index_checked(&nodes.borrow(), i)?).clone();
                __elt
            }) < 0
            {
                set_idx = set_idx + 1;
                {
                    let __cell0 = -(set_idx);
                    let __idx0 = i;
                    *metamodelica::index_mut_checked(&mut nodes.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
        }
        setsArray = arrayCreate(set_idx, metamodelica::nil());
        entries = UnorderedMap::toArray(sets.elements.clone());
        for mut i in ({
            let __s = metamodelica::arrayLength(entries.clone());
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            (e, idx) = metamodelica::Dangerous::arrayGetNoBoundsChecking(entries.clone(), i);
            set_idx = ({
                let __elt = (*metamodelica::index_checked(&nodes.borrow(), idx)?).clone();
                __elt
            });
            while set_idx > 0 {
                set_idx = ({
                    let __elt = (*metamodelica::index_checked(&nodes.borrow(), set_idx)?).clone();
                    __elt
                });
            }
            set_idx = -(set_idx);
            {
                let __cell1 = metamodelica::cons(
                    e,
                    ({
                        let __elt = (*metamodelica::index_checked(&setsArray.borrow(), set_idx)?).clone();
                        __elt
                    }),
                );
                let __idx1 = set_idx;
                *metamodelica::index_mut_checked(&mut setsArray.clone().borrow_mut(), __idx1)? = __cell1;
            }
        }
        assignedSets = Sets {
            nodes: nodes.clone(),
            elements: sets.elements.clone(),
            nodeCount: sets.nodeCount.clone(),
        };
        Ok((setsArray, assignedSets))
    }

    pub(crate) fn find(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
        let mut sets: Sets = sets;
        let mut index: i32;
        let mut oindex: Option<i32>;
        oindex = UnorderedMap::get(entry.clone(), sets.elements.clone())?;
        if (oindex).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(oindex) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            index = metamodelica::Own::own(__pa0);
        } else {
            (sets, index) = add(entry, sets)?;
        }
        Ok((sets, index))
    }

    pub(crate) fn findRoot(mut nodeIndex: i32, mut nodes: metamodelica::Array<i32>) -> Result<i32> {
        let mut rootIndex: i32 = nodeIndex;
        let mut parent: i32 = ({
            let __elt = (*metamodelica::index_checked(&nodes.borrow(), nodeIndex)?).clone();
            __elt
        });
        let mut idx: i32 = nodeIndex;
        while parent > 0 {
            rootIndex = parent;
            parent = ({
                let __elt = (*metamodelica::index_checked(&nodes.borrow(), parent)?).clone();
                __elt
            });
        }
        parent = ({
            let __elt = (*metamodelica::index_checked(&nodes.borrow(), nodeIndex)?).clone();
            __elt
        });
        while parent > 0 {
            metamodelica::arrayUpdate(nodes.clone(), idx, rootIndex)?;
            idx = parent;
            parent = ({
                let __elt = (*metamodelica::index_checked(&nodes.borrow(), parent)?).clone();
                __elt
            });
        }
        Ok(rootIndex)
    }

    pub(crate) fn findSet(mut entry: Entry, mut sets: Sets) -> Result<(i32, Sets)> {
        let mut set: i32;
        let mut updatedSets: Sets;
        let mut index: i32;
        (updatedSets, index) = find(entry, sets)?;
        set = findRoot(index, updatedSets.nodes.clone())?;
        Ok((set, updatedSets))
    }

    pub(crate) fn findSetArrayIndex(mut entry: Entry, mut sets: &Sets) -> Result<i32> {
        let mut set: i32;
        set = UnorderedMap::getOrFail(entry, sets.elements.clone())?;
        while set > 0 {
            set = ({
                let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set)?).clone();
                __elt
            });
        }
        set = -(set);
        Ok(set)
    }

    pub(crate) fn getEntry(
        mut entry: Entry,
        mut sets: &Sets,
    ) -> Result<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut outEntry: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        outEntry = UnorderedMap::getKey(entry, sets.elements.clone())?;
        Ok(outEntry)
    }

    pub(crate) fn getNodeCount(mut sets: &Sets) -> i32 {
        let mut nodeCount: i32 = sets.nodeCount.clone();
        nodeCount
    }

    pub(crate) fn merge(mut entry1: Entry, mut entry2: Entry, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut set1: i32;
        let mut set2: i32;
        (set1, sets) = findSet(entry1, sets)?;
        (set2, sets) = findSet(entry2, sets)?;
        sets = union(set1, set2, sets)?;
        Ok(sets)
    }

    pub(crate) fn printSets(mut sets: &Sets) -> Result<()> {
        let mut nodes: metamodelica::Array<i32>;
        let mut entries: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>;
        let mut e: Entry;
        let mut i: i32;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(sets.nodeCount.clone()));
            __mm_s.push_str(&*literal!(" sets:\n"));
            ArcStr::from(__mm_s)
        });
        nodes = sets.nodes.clone();
        entries = UnorderedMap::toList(sets.elements.clone());
        for mut p in &*entries {
            (e, i) = p.clone();
            metamodelica::print(literal!("["));
            metamodelica::print(ArcStr::from(::std::format!("{}", i)));
            metamodelica::print(literal!("]"));
            metamodelica::print(EntryString(e)?);
            metamodelica::print(literal!(" -> "));
            metamodelica::print(ArcStr::from(::std::format!(
                "{}",
                ({
                    let __elt = (*metamodelica::index_checked(&nodes.borrow(), i)?).clone();
                    __elt
                })
            )));
            metamodelica::print(literal!("\n"));
        }
        Ok(())
    }

    pub(crate) fn union(mut set1: i32, mut set2: i32, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut rank1: i32;
        let mut rank2: i32;
        if set1 != set2 {
            rank1 = ({
                let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set1)?).clone();
                __elt
            });
            rank2 = ({
                let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set2)?).clone();
                __elt
            });
            if rank1 > rank2 {
                metamodelica::arrayUpdate(sets.nodes.clone(), set2, set1)?;
            } else if rank1 < rank2 {
                metamodelica::arrayUpdate(sets.nodes.clone(), set1, set2)?;
            } else {
                metamodelica::arrayUpdate(
                    sets.nodes.clone(),
                    set1,
                    ({
                        let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set1)?).clone();
                        __elt
                    }) - 1,
                )?;
                metamodelica::arrayUpdate(sets.nodes.clone(), set2, set1)?;
            }
        }
        Ok(sets)
    }
}

pub type IsDeletedFn =
    std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>;

pub(crate) fn handleOverconstrainedConnections(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut conns: &metamodelica::Ref<NFConnections::NFConnections>,
    mut isDeleted: &dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool>,
) -> Result<(metamodelica::Ref<FlatModel::NFFlatModel>, FlatEdges)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut broken: FlatEdges;
    let mut graph: NFOCConnectionGraph = EMPTY().clone();
    let mut connected: FlatEdges;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut print_trace: bool = Flags::isSet(Flags::CGRAPH.clone())?;
    graph = addBreakableBranches(&conns.connections, isDeleted, print_trace, graph)?;
    (eql, graph) = addRootsAndBranches(&flatModel.equations, print_trace, graph)?;
    assign_field!(flatModel.equations = eql);
    (flatModel, connected, broken) = handleOverconstrainedConnections_dispatch(&graph, flatModel)?;
    assign_field!(
        flatModel.equations = removeBrokenConnects(flatModel.equations.clone(), &connected, &broken, isDeleted)?
    );
    Ok((flatModel, broken))
}

fn addBreakableBranches(
    mut connections: &metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
    mut isDeleted: &dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool>,
    mut printTrace: bool,
    mut graph: NFOCConnectionGraph,
) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    let mut breakable: CrefSets::Sets;
    let mut c1: metamodelica::Ref<Connector::NFConnector>;
    let mut c2: metamodelica::Ref<Connector::NFConnector>;
    let mut lhs_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut rhs_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut rhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut lhs_set: i32;
    let mut rhs_set: i32;
    breakable = CrefSets::emptySets(3);
    for mut conn in &**connections {
        let __arc2 = conn.clone();
        let Connection::CONNECTION { lhs: __pa0, rhs: __pa1 } = &*__arc2;
        c1 = metamodelica::Own::own(__pa0);
        c2 = metamodelica::Own::own(__pa1);
        lhs_crefs = getOverconstrainedCrefs(&c1, isDeleted)?;
        rhs_crefs = getOverconstrainedCrefs(&c2, isDeleted)?;
        for mut lhs in &*lhs_crefs {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rhs_crefs) {
                Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            rhs = metamodelica::Own::own(__pa3);
            rhs_crefs = metamodelica::Own::own(__pa4);
            (lhs_set, breakable) = CrefSets::findSet(lhs.clone(), breakable)?;
            (rhs_set, breakable) = CrefSets::findSet(rhs.clone(), breakable)?;
            if lhs_set != rhs_set {
                graph = addConnection(lhs.clone(), rhs, c1.source.clone(), printTrace, graph)?;
                breakable = CrefSets::union(lhs_set, rhs_set, breakable)?;
            }
        }
    }
    Ok(graph)
}

fn addRootsAndBranches(
    mut equations: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut printTrace: bool,
    mut graph: NFOCConnectionGraph,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    NFOCConnectionGraph,
)> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut graph: NFOCConnectionGraph = graph;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut root: metamodelica::Ref<Expression::NFExpression>;
    let mut msg: metamodelica::Ref<Expression::NFExpression>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut lhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut rhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut priority: i32;
    for mut eq in &**equations {
        outEquations = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { arguments: __esc_args, .. } }, .. } => {
                call = (*__esc_call).clone();
                args = (*__esc_args).clone();
                (match identifyConnectionsOperator(&(Function::name(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL)))) {
            ConnectionsOperator::ROOT => {
                let __pa0 = ::match_deref::match_deref! { match &(args.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                cref = metamodelica::Own::own(__pa0);
                graph = addDefiniteRoot(cref.clone(), printTrace, graph)?;
                outEquations
            },
            ConnectionsOperator::POTENTIAL_ROOT => {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                arg1 = metamodelica::Own::own(__pa0);
                arg2 = metamodelica::Own::own(__pa1);
                let __pa3 = ::match_deref::match_deref! { match &(arg1) {
                    Deref @ Expression::CREF { cref: __pa3, .. } => __pa3.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                cref = metamodelica::Own::own(__pa3);
                let __pa4 = ::match_deref::match_deref! { match &(Ceval::evalExp(arg2, &(Ceval::noTarget().clone()))?) {
                    Deref @ Expression::INTEGER { value: __pa4 } => __pa4.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                priority = metamodelica::Own::own(__pa4);
                graph = addPotentialRoot(cref.clone(), metamodelica::OrderedFloat((priority) as f64), printTrace, graph)?;
                outEquations
            },
            ConnectionsOperator::UNIQUE_ROOT => {
                graph = (::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_root @ Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                root = (*__esc_root).clone();
                cref = (*__esc_cref).clone();
                addUniqueRoots(metamodelica::AsArg::as_arg(&root), metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") }), printTrace, graph)?
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_root @ Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_msg, tail: Deref @ metamodelica::ListNode::Nil } } => {
                root = (*__esc_root).clone();
                cref = (*__esc_cref).clone();
                msg = (*__esc_msg).clone();
                addUniqueRoots(metamodelica::AsArg::as_arg(&root), msg.clone(), printTrace, graph)?
            },
            _ => return Err("match: no arm matched"),
        } });
                outEquations
            },
            ConnectionsOperator::BRANCH { .. } => {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                lhs = metamodelica::Own::own(__pa0);
                rhs = metamodelica::Own::own(__pa1);
                graph = addBranch(lhs, rhs, printTrace, graph)?;
                outEquations
            },
            _ => metamodelica::cons(eq.clone(), outEquations),
        })
            },
            _ => metamodelica::cons(eq.clone(), outEquations),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outEquations = metamodelica::Dangerous::listReverseInPlace(outEquations);
    Ok((outEquations, graph))
}

fn generateEqualityConstraintEquation(
    mut lhs: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhs: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut equalityConstraintEq: metamodelica::Ref<Equation::NFEquation>;
    let mut context: i32;
    let mut fcref_rhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut fcref_lhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut fn_node_rhs: metamodelica::Ref<InstNode::InstNode>;
    let mut fn_node_lhs: metamodelica::Ref<InstNode::InstNode>;
    let mut exp_rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut exp_lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut info: SourceInfo = ElementSource::getInfo(source.clone());
    context = intBitOr(InstContext::EQUATION.clone(), InstContext::CONNECT.clone());
    fcref_rhs = Function::lookupFunctionSimple(
        literal!("equalityConstraint"),
        NFInstNode::InstNode::classScope(ComponentRef::node(&lhs)?)?,
        context,
    )?;
    (fcref_rhs, fn_node_rhs, _) = Function::instFunctionRef(fcref_rhs, context, Absyn::dummyInfo.clone())?;
    exp_rhs = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: metamodelica::Ref::new(Call::NFCall::UNTYPED_CALL {
            r#ref: fcref_rhs,
            arguments: list![
                Expression::fromCref(lhs.clone(), false)?,
                Expression::fromCref(rhs, false)?
            ],
            named_args: metamodelica::nil(),
            call_scope: NFInstNode::InstNode::scopeRef(fn_node_rhs),
        }),
    });
    (exp_rhs, ty, _, _) = Typing::typeExp(exp_rhs, context, &info, false)?;
    fcref_lhs = Function::lookupFunctionSimple(
        literal!("fill"),
        NFInstNode::InstNode::topScope(ComponentRef::node(&lhs)?)?,
        context,
    )?;
    (fcref_lhs, fn_node_lhs, _) = Function::instFunctionRef(fcref_lhs, context, Absyn::dummyInfo.clone())?;
    exp_lhs = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: metamodelica::Ref::new(Call::NFCall::UNTYPED_CALL {
            r#ref: fcref_lhs,
            arguments: metamodelica::cons(
                metamodelica::Ref::new(Expression::NFExpression::REAL {
                    value: metamodelica::OrderedFloat(0.0_f64),
                }),
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                        metamodelica::nil();
                    for mut d in (Type::arrayDims(ty)).into_iter().cloned() {
                        let __x = Dimension::sizeExp(&(d.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            ),
            named_args: metamodelica::nil(),
            call_scope: NFInstNode::InstNode::scopeRef(fn_node_lhs),
        }),
    });
    (exp_lhs, ty, _, _) = Typing::typeExp(exp_lhs, context, &info, false)?;
    equalityConstraintEq = Equation::makeEquality(
        exp_rhs,
        exp_lhs,
        ty,
        source,
        crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        Equation::ScalarizeMode::NO_PREFERENCE.clone(),
    );
    Ok(equalityConstraintEq)
}

fn getOverconstrainedCrefs(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut isDeleted: &dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut conns: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    conns = Connector::split(conn)?;
    conns = List::mapFlat(&conns, &Connector::scalarizePrefix)?;
    crefs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut c in (conns).into_iter().cloned() {
            if !(!(isDeleted(c.name.clone())?) && isOverconstrainedCref(&(c.name.clone()))?) {
                continue;
            }
            let __x = getOverconstrainedCref(c.name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    crefs = List::uniqueOnTrue(
        &crefs,
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
               __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1),
    )?;
    Ok(crefs)
}

fn isOverconstrainedCref(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut rest: metamodelica::Ref<ComponentRef::NFComponentRef>;
    b = (match &**cref {
        ComponentRef::CREF {
            origin: ComponentRef::Origin::CREF,
            restCref: __esc_rest,
            ..
        } => {
            rest = (*__esc_rest).clone();
            Class::isOverdetermined(NFInstNode::InstNode::getClass(ComponentRef::node(cref)?)?)
                || isOverconstrainedCref(metamodelica::AsArg::as_arg(&rest))?
        }
        _ => false,
    });
    Ok(b)
}

fn getOverconstrainedCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    '__tco: loop {
        let mut node: metamodelica::Ref<InstNode::InstNode>;
        let mut rest: metamodelica::Ref<ComponentRef::NFComponentRef>;
        match &*cref {
            ComponentRef::CREF {
                origin: ComponentRef::Origin::CREF,
                restCref: __esc_rest,
                ..
            } => {
                rest = (*__esc_rest).clone();
                if (Class::isOverdetermined(NFInstNode::InstNode::getClass(ComponentRef::node(&cref)?)?)) {
                    return Ok(cref);
                } else {
                    {
                        cref = rest.clone();
                        continue '__tco;
                    }
                }
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

fn handleOverconstrainedConnections_dispatch(
    mut graph: &NFOCConnectionGraph,
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<(metamodelica::Ref<FlatModel::NFFlatModel>, FlatEdges, FlatEdges)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut connected: FlatEdges;
    let mut broken: FlatEdges;
    let mut roots: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut rooted: CrefIndexTable;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Summary:\n\t"));
                __mm_s.push_str(&*literal!("Nr Roots:           "));
                __mm_s.push_str(&*intString(((getDefiniteRoots(graph)).len() as i32)));
                __mm_s.push_str(&*literal!("\n\t"));
                __mm_s.push_str(&*literal!("Nr Potential Roots: "));
                __mm_s.push_str(&*intString(((getPotentialRoots(graph)).len() as i32)));
                __mm_s.push_str(&*literal!("\n\t"));
                __mm_s.push_str(&*literal!("Nr Unique Roots:    "));
                __mm_s.push_str(&*intString(((getUniqueRoots(graph)).len() as i32)));
                __mm_s.push_str(&*literal!("\n\t"));
                __mm_s.push_str(&*literal!("Nr Branches:        "));
                __mm_s.push_str(&*intString(((getBranches(graph)).len() as i32)));
                __mm_s.push_str(&*literal!("\n\t"));
                __mm_s.push_str(&*literal!("Nr Connections:     "));
                __mm_s.push_str(&*intString(((getConnections(graph)).len() as i32)));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        (roots, connected, broken) = unwrap_break_err!(findResultGraph(graph, &(unwrap_break_err!(FlatModel::fullName(&flatModel), '__try0))), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Roots: "));
                __mm_s.push_str(&*stringDelimitList(unwrap_break_err!(List::map(roots.clone(), &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0)), '__try0), literal!(", ")));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Broken connections: "));
                __mm_s.push_str(&*stringDelimitList(unwrap_break_err!(List::map1(broken.clone(), &move |__a0: NFConnections::BrokenEdge, __a1: ArcStr| printConnectionStr(&__a0, &__a1), literal!("broken")), '__try0), literal!(", ")));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Allowed connections: "));
                __mm_s.push_str(&*stringDelimitList(unwrap_break_err!(List::map1(connected.clone(), &move |__a0: NFConnections::BrokenEdge, __a1: ArcStr| printConnectionStr(&__a0, &__a1), literal!("allowed")), '__try0), literal!(", ")));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        rooted = unwrap_break_err!(buildRootedTable(roots.clone(), graph), '__try0);
        assign_field!(
            flatModel.variables = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
                for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(evalConnectionsOperatorsVar(&roots, rooted.clone(), graph, v.clone()), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            flatModel.equations = unwrap_break_err!(evalConnectionsOperatorsEqs(&roots, rooted.clone(), graph, flatModel.equations.clone()), '__try0),
            flatModel.initialEquations = unwrap_break_err!(evalConnectionsOperatorsEqs(&roots, rooted.clone(), graph, flatModel.initialEquations.clone()), '__try0)
        );
        Ok::<_, &'static str>((
            broken.clone(),
            connected.clone(),
            flatModel.clone(),
            rooted.clone(),
            roots.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            broken = __try0_o0;
            connected = __try0_o1;
            flatModel = __try0_o2;
            rooted = __try0_o3;
            roots = __try0_o4;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::CGRAPH.clone())?) else {
                return Err("pattern mismatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "- NFOCConnectionGraph.handleOverconstrainedConnections failed for model: "
                ));
                __mm_s.push_str(&*FlatModel::fullName(&flatModel)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            return Err(__try0_err);
        }
    }
    Ok((flatModel, connected, broken))
}

fn addDefiniteRoot(
    mut root: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut printTrace: bool,
    mut graph: NFOCConnectionGraph,
) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addDefiniteRoot("));
            __mm_s.push_str(&*ComponentRef::toString(&root)?);
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        });
    }
    graph.definiteRoots = metamodelica::cons(root, graph.definiteRoots.clone());
    Ok(graph)
}

fn addPotentialRoot(
    mut root: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut priority: metamodelica::Real,
    mut printTrace: bool,
    mut graph: NFOCConnectionGraph,
) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addPotentialRoot("));
            __mm_s.push_str(&*ComponentRef::toString(&root)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*realString(priority));
            __mm_s.push_str(&*literal!(")"));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    graph.potentialRoots = metamodelica::cons((root, priority), graph.potentialRoots.clone());
    Ok(graph)
}

fn addUniqueRoots(
    mut roots: &metamodelica::Ref<Expression::NFExpression>,
    mut message: metamodelica::Ref<Expression::NFExpression>,
    mut printTrace: bool,
    mut graph: NFOCConnectionGraph,
) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    let mut unique_roots: UniqueRoots = graph.uniqueRoots.clone();
    for mut root in &*Expression::arrayScalarElements(roots) {
        unique_roots = (match &*root.clone() {
            Expression::CREF { cref: __root_cref, .. } => {
                if printTrace {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addUniqueRoots("));
                        __mm_s.push_str(&*Expression::toString(root.clone())?);
                        __mm_s.push_str(&*literal!(", "));
                        __mm_s.push_str(&*Expression::toString(message.clone())?);
                        __mm_s.push_str(&*literal!(")\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                metamodelica::cons((__root_cref.clone(), message.clone()), unique_roots)
            }
            _ => unique_roots,
        });
    }
    Ok(graph)
}

fn addBranch(
    mut ref1: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ref2: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut printTrace: bool,
    mut graph: NFOCConnectionGraph,
) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addBranch("));
            __mm_s.push_str(&*ComponentRef::toString(&ref1)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*ComponentRef::toString(&ref2)?);
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        });
    }
    graph.branches = metamodelica::cons((ref1, ref2), graph.branches.clone());
    Ok(graph)
}

fn addConnection(
    mut ref1: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ref2: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut printTrace: bool,
    mut graph: NFOCConnectionGraph,
) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addConnection("));
            __mm_s.push_str(&*ComponentRef::toString(&ref1)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*ComponentRef::toString(&ref2)?);
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        });
    }
    graph.connections = metamodelica::cons(
        NFConnections::BrokenEdge {
            lhs: ref1,
            rhs: ref2,
            source: source,
            brokenEquations: metamodelica::nil(),
        },
        graph.connections.clone(),
    );
    Ok(graph)
}

// ************************************* //
// ********* protected section ********* //
// ************************************* //
fn canonical(
    mut inPartition: CrefCrefTable,
    mut inRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut outCanonical: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cref_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    cref_opt = UnorderedMap::get(inRef.clone(), inPartition.clone())?;
    outCanonical = (::match_deref::match_deref! { match &(cref_opt) {
        Some(__esc_outCanonical) => {
            outCanonical = (*__esc_outCanonical).clone();
            canonical(inPartition, outCanonical.clone())?
        },
        _ => inRef,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCanonical)
}

fn areInSameComponent(
    mut partition: CrefCrefTable,
    mut ref1: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ref2: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut outResult: bool;
    outResult = ComponentRef::isEqual(&(canonical(partition.clone(), ref1)?), &(canonical(partition, ref2)?))?;
    Ok(outResult)
}

fn connectBranchComponents(
    mut partition: CrefCrefTable,
    mut ref1: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ref2: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<()> {
    connectCanonicalComponents(
        partition.clone(),
        canonical(partition.clone(), ref1)?,
        canonical(partition, ref2)?,
    )?;
    Ok(())
}

fn connectComponents(mut partition: CrefCrefTable, mut edge: FlatEdge) -> (FlatEdges, FlatEdges) {
    let mut outConnectedConnections: FlatEdges;
    let mut outBrokenConnections: FlatEdges;
    let mut canon1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut canon2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    match '__try0: {
        canon1 = unwrap_break_err!(canonical(partition.clone(), edge.lhs.clone()), '__try0);
        canon2 = unwrap_break_err!(canonical(partition.clone(), edge.rhs.clone()), '__try0);
        let false =
            (unwrap_break_err!(connectCanonicalComponents(partition.clone(), canon1.clone(), canon2.clone()), '__try0))
        else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            unwrap_break_err!(Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.connectComponents: should remove equations generated from: connect(")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(&edge.lhs), '__try0)); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(&edge.rhs), '__try0)); __mm_s.push_str(&*literal!(") and add {0, ..., 0} = equalityConstraint(cr1, cr2) instead.\n")); ArcStr::from(__mm_s) }), '__try0);
        }
        outConnectedConnections = metamodelica::nil();
        eq = unwrap_break_err!(generateEqualityConstraintEquation(edge.lhs.clone(), edge.rhs.clone(), edge.source.clone()), '__try0);
        outBrokenConnections = list![NFConnections::BrokenEdge {
            lhs: edge.lhs.clone(),
            rhs: edge.rhs.clone(),
            source: edge.source.clone(),
            brokenEquations: list![eq.clone()]
        }];
        Ok::<_, &'static str>((outBrokenConnections.clone(), outConnectedConnections.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outBrokenConnections = __try0_o0;
            outConnectedConnections = __try0_o1;
        }
        Err(_) => {
            outConnectedConnections = list![edge.clone()];
            outBrokenConnections = metamodelica::nil();
        }
    }
    (outConnectedConnections, outBrokenConnections)
}

fn connectCanonicalComponents(
    mut inPartition: CrefCrefTable,
    mut inRef1: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut inRef2: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut outReallyConnected: bool;
    outReallyConnected = !(ComponentRef::isEqual(&inRef1, &inRef2)?);
    if outReallyConnected {
        UnorderedMap::add(inRef1, inRef2, inPartition)?;
    }
    Ok(outReallyConnected)
}

fn addRootsToTable(
    mut table: CrefCrefTable,
    mut roots: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut firstRoot: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<()> {
    let mut root: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    for mut root in &**roots {
        let mut root = root.clone();
        UnorderedMap::add(root, firstRoot.clone(), table.clone())?;
    }
    Ok(())
}

fn resultGraphWithRoots(
    mut roots: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<CrefCrefTable> {
    let mut outTable: CrefCrefTable;
    let mut dummyRoot: metamodelica::Ref<ComponentRef::NFComponentRef>;
    dummyRoot = NFBuiltin::TIME_CREF().clone();
    outTable = newCrefCrefTable();
    addRootsToTable(outTable.clone(), roots, dummyRoot)?;
    Ok(outTable)
}

fn addBranchesToTable(mut table: CrefCrefTable, mut branches: &Edges) -> Result<()> {
    let mut ref1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ref2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    for mut branch in &**branches {
        (ref1, ref2) = branch.clone();
        connectBranchComponents(table.clone(), ref1, ref2)?;
    }
    Ok(())
}

fn ord(mut inEl1: &PotentialRoot, mut inEl2: &PotentialRoot) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (inEl1, inEl2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((c1, r1), (c2, r2)) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let true = (realEq(r1.clone(), r2.clone())) else { return Err("pattern mismatch") };
                    s1 = ComponentRef::toString(metamodelica::AsArg::as_arg(&c1))?;
                    s2 = ComponentRef::toString(metamodelica::AsArg::as_arg(&c2))?;
                    let 1 = (stringCompare(&s1, &s2)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((_, r1), (_, r2)) => {
                    Ok(r1.clone() > r2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

fn addPotentialRootsToTable(
    mut table: CrefCrefTable,
    mut potentialRoots: &PotentialRoots,
    mut roots: &DefiniteRoots,
    mut firstRoot: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> DefiniteRoots {
    let mut outRoots: DefiniteRoots;
    outRoots = 'mc: {
        let __mc_input = &**potentialRoots;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(roots.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (potentialRoot, _), tail: tail } => {
                    let mut canon1: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    let mut canon2: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    let mut finalRoots: DefiniteRoots;
                    canon1 = canonical(table.clone(), potentialRoot.clone())?;
                    canon2 = canonical(table.clone(), firstRoot.clone())?;
                    let true = (connectCanonicalComponents(table.clone(), canon1.clone(), canon2.clone())?) else { return Err("pattern mismatch") };
                    finalRoots = addPotentialRootsToTable(table.clone(), metamodelica::AsArg::as_arg(&tail), &(metamodelica::cons(potentialRoot.clone(), roots.clone())), firstRoot);
                    Ok(finalRoots.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: tail } => {
                    let mut finalRoots: DefiniteRoots;
                    finalRoots = addPotentialRootsToTable(table.clone(), metamodelica::AsArg::as_arg(&tail), roots, firstRoot);
                    Ok(finalRoots.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outRoots
}

fn addConnections(mut table: CrefCrefTable, mut inConnections: &FlatEdges) -> (FlatEdges, FlatEdges) {
    let mut outConnectedConnections: FlatEdges = metamodelica::nil();
    let mut outBrokenConnections: FlatEdges = metamodelica::nil();
    let mut connected: FlatEdges;
    let mut broken: FlatEdges;
    for mut c in &**inConnections {
        (connected, broken) = connectComponents(table.clone(), c.clone());
        outConnectedConnections = listAppend(connected, outConnectedConnections);
        outBrokenConnections = listAppend(broken, outBrokenConnections);
    }
    (outConnectedConnections, outBrokenConnections)
}

fn findResultGraph(
    mut inGraph: &NFOCConnectionGraph,
    mut modelNameQualified: &ArcStr,
) -> Result<(DefiniteRoots, FlatEdges, FlatEdges)> {
    let mut outRoots: DefiniteRoots;
    let mut outConnectedConnections: FlatEdges;
    let mut outBrokenConnections: FlatEdges;
    (outRoots, outConnectedConnections, outBrokenConnections) = (::match_deref::match_deref! { match &(inGraph) {
        NFOCConnectionGraph { definiteRoots: Deref @ metamodelica::ListNode::Nil, potentialRoots: Deref @ metamodelica::ListNode::Nil, uniqueRoots: Deref @ metamodelica::ListNode::Nil, branches: Deref @ metamodelica::ListNode::Nil, connections: Deref @ metamodelica::ListNode::Nil, .. } => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        NFOCConnectionGraph { definiteRoots, potentialRoots, uniqueRoots, branches, connections, .. } => {
            let mut finalRoots: DefiniteRoots;
            let mut orderedPotentialRoots: PotentialRoots;
            let mut broken: FlatEdges;
            let mut connected: FlatEdges;
            let mut table: CrefCrefTable;
            let mut dummyRoot: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut brokenConnectsViaGraphViz: ArcStr;
            let mut userBrokenLst: metamodelica::List<ArcStr>;
            let mut userBrokenLstLst: metamodelica::List<metamodelica::List<ArcStr>>;
            let mut userBrokenTplLst: metamodelica::List<(ArcStr, ArcStr)>;
            let mut connections = (*connections).clone();
            connections = connections.clone().reverse();
            table = resultGraphWithRoots(metamodelica::AsArg::as_arg(&definiteRoots))?;
            addBranchesToTable(table.clone(), metamodelica::AsArg::as_arg(&branches))?;
            orderedPotentialRoots = List::sort(potentialRoots.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real), __a1: (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real)| -> metamodelica::Result<_> { ::std::result::Result::Ok(ord(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real), (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real)) -> Result<bool> + 'static>))?;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Ordered Potential Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(orderedPotentialRoots.clone(), &move |__a0: (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real)| printPotentialRootTuple(&__a0))?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (connected, broken) = addConnections(table.clone(), metamodelica::AsArg::as_arg(&connections));
            dummyRoot = NFBuiltin::TIME_CREF().clone();
            finalRoots = addPotentialRootsToTable(table, &orderedPotentialRoots, metamodelica::AsArg::as_arg(&definiteRoots), &dummyRoot);
            brokenConnectsViaGraphViz = generateGraphViz(modelNameQualified.clone(), definiteRoots.clone(), potentialRoots.clone(), metamodelica::AsArg::as_arg(&uniqueRoots), branches.clone(), connections.clone(), finalRoots.clone(), broken.clone())?;
            if stringEq(&brokenConnectsViaGraphViz, &(literal!(""))) {
            } else {
                userBrokenLst = Util::stringSplitAtChar(brokenConnectsViaGraphViz, literal!("#"))?;
                userBrokenLstLst = List::map1(userBrokenLst, &Util::stringSplitAtChar, literal!("|"))?;
                userBrokenTplLst = makeTuple(&userBrokenLstLst)?;
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("User selected the following connect edges for breaking:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(userBrokenTplLst.clone(), &move |__a0: (ArcStr, ArcStr)| -> metamodelica::Result<_> { ::std::result::Result::Ok(printTupleStr(&__a0)) })?, literal!("\n\t"))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                printFlatEdges(metamodelica::AsArg::as_arg(&connections))?;
                connections = orderConnectsGuidedByUser(metamodelica::AsArg::as_arg(&connections), userBrokenTplLst)?;
                connections = connections.clone().reverse();
                metamodelica::print(literal!("\nAfer ordering:\n"));
                (finalRoots, connected, broken) = findResultGraph(&(NFOCConnectionGraph { updateGraph: false, definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }), modelNameQualified)?;
            }
            (finalRoots, connected, broken)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outRoots, outConnectedConnections, outBrokenConnections))
}

fn orderConnectsGuidedByUser(
    mut inConnections: &FlatEdges,
    mut inUserSelectedBreaking: metamodelica::List<(ArcStr, ArcStr)>,
) -> Result<FlatEdges> {
    let mut outOrderedConnections: FlatEdges;
    let mut front: FlatEdges = metamodelica::nil();
    let mut back: FlatEdges = metamodelica::nil();
    let mut sc1: ArcStr;
    let mut sc2: ArcStr;
    for mut e in &**inConnections {
        sc1 = ComponentRef::toString(&e.lhs)?;
        sc2 = ComponentRef::toString(&e.rhs)?;
        if listMember((sc1.clone(), sc2.clone()), inUserSelectedBreaking.clone())
            || listMember((sc2, sc1), inUserSelectedBreaking.clone())
        {
            back = metamodelica::cons(e.clone(), back);
        } else {
            front = metamodelica::cons(e.clone(), front);
        }
    }
    outOrderedConnections = List::append_reverse(&front, back);
    Ok(outOrderedConnections)
}

fn printTupleStr(mut inTpl: &(ArcStr, ArcStr)) -> ArcStr {
    let mut out: ArcStr;
    out = (match inTpl.clone() {
        (mut c1, mut c2) => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*c1);
            __mm_s.push_str(&*literal!(" -- "));
            __mm_s.push_str(&*c2);
            ArcStr::from(__mm_s)
        }
    });
    out
}

fn makeTuple(
    mut inLstLst: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<metamodelica::List<(ArcStr, ArcStr)>> {
    let mut outLst: metamodelica::List<(ArcStr, ArcStr)>;
    outLst = 'mc: {
        let __mc_input = &**inLstLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: c1, tail: Deref @ metamodelica::ListNode::Cons { head: c2, tail: Deref @ metamodelica::ListNode::Nil } }, tail: rest } => {
                    let mut lst: metamodelica::List<(ArcStr, ArcStr)>;
                    lst = makeTuple(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(metamodelica::cons((c1.clone(), c2.clone()), lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Nil }, tail: rest } => {
                    let mut lst: metamodelica::List<(ArcStr, ArcStr)>;
                    lst = makeTuple(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Nil, tail: rest } => {
                    let mut lst: metamodelica::List<(ArcStr, ArcStr)>;
                    lst = makeTuple(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: bad, tail: rest } => {
                    let mut lst: metamodelica::List<(ArcStr, ArcStr)>;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following output from GraphViz OpenModelica assistant cannot be parsed:")); __mm_s.push_str(&*stringDelimitList(bad.clone(), literal!(", "))); __mm_s.push_str(&*literal!("\nExpected format from GrapViz: cref1|cref2#cref3|cref4#. Ignoring malformed input.\n")); ArcStr::from(__mm_s) });
                    lst = makeTuple(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outLst)
}

fn printPotentialRootTuple(mut potentialRoot: &PotentialRoot) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (::match_deref::match_deref! { match &(potentialRoot) {
        (cr, priority) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&cr))?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStr)
}

fn buildRootedTable(
    mut roots: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut graph: &NFOCConnectionGraph,
) -> Result<CrefIndexTable> {
    let mut rooted: CrefIndexTable;
    let mut table: CrefRootsTable;
    table = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    List::map1_0(
        &(getBranches(graph)),
        &move |__a0: (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        ),
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >| addBranches(&__a0, __a1),
        table.clone(),
    )?;
    List::map1_0(
        &(getConnections(graph)),
        &move |__a0: NFConnections::BrokenEdge,
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >| addConnectionsRooted(&__a0, __a1),
        table.clone(),
    )?;
    rooted = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    setRootDistance(roots, table, 0, metamodelica::nil(), rooted.clone())?;
    Ok(rooted)
}

fn setRootDistance(
    mut finalRoots: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut table: CrefRootsTable,
    mut distance: i32,
    mut nextLevel: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut rooted: CrefIndexTable,
) -> Result<()> {
    let mut level: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = finalRoots;
    let mut next: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = nextLevel;
    let mut neighbors: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut dist: i32 = distance;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    loop {
        if (level).is_empty() {
            if (next).is_empty() {
                return Ok(());
            }
            level = next;
            next = metamodelica::nil();
            dist = dist + 1;
        } else {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(level) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            level = metamodelica::Own::own(__pa1);
            if !(UnorderedMap::contains(cr.clone(), rooted.clone())?) {
                UnorderedMap::addNew(cr.clone(), dist, rooted.clone())?;
                next = (::match_deref::match_deref! { match &(UnorderedMap::get(cr, table.clone())?) {
                    Some(__esc_neighbors) => {
                        neighbors = (*__esc_neighbors).clone();
                        listAppend(next, neighbors.clone())
                    },
                    _ => next,
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
        }
    }
    Ok(())
}

fn addBranches(mut edge: &Edge, mut table: CrefRootsTable) -> Result<()> {
    let mut cref1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cref2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    (cref1, cref2) = edge.clone();
    addConnectionRooted(cref1.clone(), &cref2, table.clone())?;
    addConnectionRooted(cref2, &cref1, table)?;
    Ok(())
}

fn addConnectionsRooted(mut connection: &FlatEdge, mut table: CrefRootsTable) -> Result<()> {
    addConnectionRooted(connection.lhs.clone(), &connection.rhs, table.clone())?;
    addConnectionRooted(connection.rhs.clone(), &connection.lhs, table)?;
    Ok(())
}

fn addConnectionRooted(
    mut cref1: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut cref2: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut table: CrefRootsTable,
) -> Result<()> {
    pub(crate) fn updateRooted(
        mut roots: Option<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut newRoot: metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> DefiniteRoots {
        let mut outRoots: DefiniteRoots;
        outRoots = (::match_deref::match_deref! { match &(roots) {
            Some(__esc_outRoots) => {
                outRoots = (*__esc_outRoots).clone();
                metamodelica::cons(newRoot, outRoots.clone())
            },
            _ => list![newRoot],
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        outRoots
    }

    UnorderedMap::addUpdate(
        cref1,
        &({
            let __pe_b1 = cref2.clone();
            move |__pe_a0| Ok(updateRooted(__pe_a0, __pe_b1.clone()))
        }),
        table,
    )?;
    Ok(())
}

fn evalConnectionsOperatorsEqs(
    mut inRoots: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut rooted: CrefIndexTable,
    mut graph: &NFOCConnectionGraph,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    equations = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
        for mut eq in (equations).into_iter().cloned() {
            let __x = Equation::mapExpShallow(
                eq.clone(),
                &({
                    let __pe_b1 = rooted.clone();
                    let __pe_b2 = inRoots.clone();
                    let __pe_b3 = graph.clone();
                    let __pe_b4 = Equation::info(&(eq.clone()));
                    move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3, &__pe_b4)
                }),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(equations)
}

fn evalConnectionsOperatorsVar(
    mut roots: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut rooted: CrefIndexTable,
    mut graph: &NFOCConnectionGraph,
    mut var: metamodelica::Ref<Variable::NFVariable>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    assign_field!(
        var.binding = Binding::mapExpShallow(
            var.binding.clone(),
            &({
                let __pe_b1 = rooted;
                let __pe_b2 = roots.clone();
                let __pe_b3 = graph.clone();
                let __pe_b4 = var.info.clone();
                move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3, &__pe_b4)
            })
        )?
    );
    Ok(var)
}

fn evaluateOperators(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut rooted: CrefIndexTable,
    mut roots: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut graph: &NFOCConnectionGraph,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = rooted;
            let __pe_b2 = roots.clone();
            let __pe_b3 = graph.clone();
            let __pe_b4 = info.clone();
            move |__pe_a0| evalConnectionsOperatorsHelper(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3, &__pe_b4)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

fn evalConnectionsOperatorsHelper(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut rooted: CrefIndexTable,
    mut roots: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut graph: &NFOCConnectionGraph,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut uroots: metamodelica::Ref<Expression::NFExpression>;
            let mut nodes: metamodelica::Ref<Expression::NFExpression>;
            let mut message: metamodelica::Ref<Expression::NFExpression>;
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut cref1: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut result: bool;
            let mut branches: Edges;
            let mut r#str: ArcStr;
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            (match identifyConnectionsOperator(&(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)))) {
        ConnectionsOperator::ROOTED => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        _ if (Expression::isEmptyArray(&((var_field!((**call).arguments, Call::NFCall::TYPED_CALL)).head().cloned()?))) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(" = false\n")); ArcStr::from(__mm_s) });
            }
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            cref = (*__esc_cref).clone();
            branches = getBranches(graph);
            cref = ComponentRef::stripIteratorSubscripts(cref.clone())?;
            match '__try0: {
                cref1 = unwrap_break_err!(getEdge(metamodelica::AsArg::as_arg(&cref), &branches), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: Found Branche Partner ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(metamodelica::AsArg::as_arg(&cref)), '__try0)); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(&cref1), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                }
                result = getRooted(cref.clone(), cref1.clone(), rooted.clone());
                if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*unwrap_break_err!(Expression::toString(exp.clone()), '__try0)); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                }
                Ok::<_, &'static str>((result.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    result = __try0_o0;
                }
                Err(_) => {
                    r#str = ComponentRef::toString(metamodelica::AsArg::as_arg(&cref))?;
                    Error::addSourceMessage(&(Error::OCG_MISSING_BRANCH.clone()), list![r#str.clone(), r#str.clone(), r#str.clone()], info)?;
                    result = false;
                }
            }
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: result })
        },
        _ => return Err("match: no arm matched"),
    } });
            res
        },
        ConnectionsOperator::IS_ROOT => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        _ if (Expression::isEmptyArray(&((var_field!((**call).arguments, Call::NFCall::TYPED_CALL)).head().cloned()?))) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(" = false\n")); ArcStr::from(__mm_s) });
            }
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            cref = (*__esc_cref).clone();
            cref = ComponentRef::stripIteratorSubscripts(cref.clone())?;
            result = List::isMemberOnTrue(cref.clone(), roots, &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1))?;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: result })
        },
        _ => return Err("match: no arm matched"),
    } });
            res
        },
        ConnectionsOperator::UNIQUE_ROOT_INDICES => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_uroots, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_nodes, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_message, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            uroots = (*__esc_uroots).clone();
            nodes = (*__esc_nodes).clone();
            message = (*__esc_message).clone();
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: Connections.uniqueRootsIndices(")); __mm_s.push_str(&*Expression::toString(uroots.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*Expression::toString(nodes.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*Expression::toString(message.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
            }
            dim = Type::nthDimension(Expression::typeOf(uroots.clone()), 1)?;
            if !(Dimension::isKnown(&dim, false)) {
                Error::addSourceMessage(&(Error::DIMENSION_NOT_KNOWN.clone()), list![Expression::toString(exp)?], info)?;
                return Err("fail");
            }
            Expression::fillArray(Dimension::size(&dim, false)?, metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }))?
        },
        _ => return Err("match: no arm matched"),
    } });
            res
        },
        _ => exp,
    })
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn getRooted(
    mut cref1: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut cref2: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rooted: CrefIndexTable,
) -> bool {
    let mut result: bool;
    result = 'mc: {
        let __mc_input = &*rooted;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut i1: i32;
                    let mut i2: i32;
                    i1 = UnorderedMap::getOrFail(cref1.clone(), rooted.clone())?;
                    i2 = UnorderedMap::getOrFail(cref2.clone(), rooted.clone())?;
                    Ok(intLt(i1, i2))
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
    result
}

fn getEdge(
    mut cr: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut edges: &Edges,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut ocr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cref1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cref2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    for mut edge in &**edges {
        (cref1, cref2) = edge.clone();
        if ComponentRef::isEqual(cr, &cref1)? {
            ocr = cref2;
            return Ok(ocr);
        } else if ComponentRef::isEqual(cr, &cref2)? {
            ocr = cref1;
            return Ok(ocr);
        }
    }
    return Err("fail");
    Ok(ocr)
}

fn printConnectionStr(mut edge: &FlatEdge, mut ty: &ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ty);
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*ComponentRef::toString(&edge.lhs)?);
        __mm_s.push_str(&*literal!(", "));
        __mm_s.push_str(&*ComponentRef::toString(&edge.rhs)?);
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Ok(outStr)
}

fn printEdges(mut inEdges: &Edges) -> Result<()> {
    let () = (::match_deref::match_deref! { match inEdges {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (c1, c2), tail: tail } => {
            metamodelica::print(literal!("    "));
            metamodelica::print(ComponentRef::toString(metamodelica::AsArg::as_arg(&c1))?);
            metamodelica::print(literal!(" -- "));
            metamodelica::print(ComponentRef::toString(metamodelica::AsArg::as_arg(&c2))?);
            metamodelica::print(literal!("\n"));
            printEdges(tail)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printFlatEdges(mut inEdges: &FlatEdges) -> Result<()> {
    for mut edge in &**inEdges {
        metamodelica::print(literal!("    "));
        metamodelica::print(ComponentRef::toString(&edge.lhs)?);
        metamodelica::print(literal!(" -- "));
        metamodelica::print(ComponentRef::toString(&edge.rhs)?);
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

fn printNFOCConnectionGraph(mut inGraph: &NFOCConnectionGraph) -> Result<()> {
    let () = (match inGraph.clone() {
        NFOCConnectionGraph {
            connections: mut connections,
            branches: mut branches,
            ..
        } => {
            metamodelica::print(literal!("Connections:\n"));
            printFlatEdges(metamodelica::AsArg::as_arg(&connections))?;
            metamodelica::print(literal!("Branches:\n"));
            printEdges(metamodelica::AsArg::as_arg(&branches))?;
            ()
        }
    });
    Ok(())
}

fn getDefiniteRoots(mut inGraph: &NFOCConnectionGraph) -> DefiniteRoots {
    let mut outResult: DefiniteRoots;
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph {
            definiteRoots: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

fn getUniqueRoots(mut inGraph: &NFOCConnectionGraph) -> UniqueRoots {
    let mut outResult: UniqueRoots;
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph {
            uniqueRoots: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

fn getPotentialRoots(mut inGraph: &NFOCConnectionGraph) -> PotentialRoots {
    let mut outResult: PotentialRoots;
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph {
            potentialRoots: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

fn getBranches(mut inGraph: &NFOCConnectionGraph) -> Edges {
    let mut outResult: Edges;
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph {
            branches: ref result, ..
        } => result.clone(),
    });
    outResult
}

fn getConnections(mut inGraph: &NFOCConnectionGraph) -> FlatEdges {
    let mut outResult: FlatEdges;
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph {
            connections: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

fn merge(mut inGraph1: NFOCConnectionGraph, mut inGraph2: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut outGraph: NFOCConnectionGraph;
    outGraph = (::match_deref::match_deref! { match &((&inGraph1, &inGraph2)) {
        (_, NFOCConnectionGraph { definiteRoots: Deref @ metamodelica::ListNode::Nil, potentialRoots: Deref @ metamodelica::ListNode::Nil, uniqueRoots: Deref @ metamodelica::ListNode::Nil, branches: Deref @ metamodelica::ListNode::Nil, connections: Deref @ metamodelica::ListNode::Nil, .. }) => {
            inGraph1.clone()
        },
        (NFOCConnectionGraph { definiteRoots: Deref @ metamodelica::ListNode::Nil, potentialRoots: Deref @ metamodelica::ListNode::Nil, uniqueRoots: Deref @ metamodelica::ListNode::Nil, branches: Deref @ metamodelica::ListNode::Nil, connections: Deref @ metamodelica::ListNode::Nil, .. }, _) => {
            inGraph2.clone()
        },
        (_, _) if (inGraph1.clone() == inGraph2.clone()) => {
            inGraph1.clone()
        },
        (NFOCConnectionGraph { updateGraph: updateGraph1, definiteRoots: definiteRoots1, potentialRoots: potentialRoots1, uniqueRoots: uniqueRoots1, branches: branches1, connections: connections1 }, NFOCConnectionGraph { updateGraph: updateGraph2, definiteRoots: definiteRoots2, potentialRoots: potentialRoots2, uniqueRoots: uniqueRoots2, branches: branches2, connections: connections2 }) => {
            let mut updateGraph: bool;
            let mut definiteRoots: DefiniteRoots;
            let mut uniqueRoots: UniqueRoots;
            let mut potentialRoots: PotentialRoots;
            let mut branches: Edges;
            let mut connections: FlatEdges;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::trace(literal!("- NFOCConnectionGraph.merge()\n"))?;
            }
            updateGraph = boolOr(updateGraph1.clone(), updateGraph2.clone());
            definiteRoots = List::union(metamodelica::AsArg::as_arg(&definiteRoots1), metamodelica::AsArg::as_arg(&definiteRoots2));
            potentialRoots = List::union(metamodelica::AsArg::as_arg(&potentialRoots1), metamodelica::AsArg::as_arg(&potentialRoots2));
            uniqueRoots = List::union(metamodelica::AsArg::as_arg(&uniqueRoots1), metamodelica::AsArg::as_arg(&uniqueRoots2));
            branches = List::union(metamodelica::AsArg::as_arg(&branches1), metamodelica::AsArg::as_arg(&branches2));
            connections = List::union(metamodelica::AsArg::as_arg(&connections1), metamodelica::AsArg::as_arg(&connections2));
            NFOCConnectionGraph { updateGraph: updateGraph, definiteRoots: definiteRoots, potentialRoots: potentialRoots, uniqueRoots: uniqueRoots, branches: branches, connections: connections }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

/* **********************************************************************************************************************/
/* ****************************************** GraphViz generation *******************************************************/
/* **********************************************************************************************************************/
fn graphVizEdge(mut inEdge: &Edge) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match &(inEdge) {
        (c1, c2) => {
            let mut strEdge: ArcStr;
            strEdge = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!("\" -- \"")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&c2))?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [color = blue, dir = \"none\", fontcolor=blue, label = \"branch\"];\n\t")); ArcStr::from(__mm_s) };
            strEdge
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn graphVizFlatEdge(mut edge: FlatEdge, mut inBrokenFlatEdges: &FlatEdges) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut sc1: ArcStr;
    let mut sc2: ArcStr;
    let mut label: ArcStr;
    let mut labelFontSize: ArcStr;
    let mut decorate: ArcStr;
    let mut color: ArcStr;
    let mut style: ArcStr;
    let mut fontColor: ArcStr;
    let mut isBroken: bool;
    isBroken = List::isMemberOnTrue(
        edge.clone(),
        inBrokenFlatEdges,
        &move |__a0: NFConnections::BrokenEdge, __a1: NFConnections::BrokenEdge| FlatEdgeIsEqual(&__a0, &__a1),
    )?;
    label = if (isBroken) {
        literal!("[[broken connect]]")
    } else {
        literal!("connect")
    };
    color = if (isBroken) { literal!("red") } else { literal!("green") };
    style = if (isBroken) {
        literal!("\"bold, dashed\"")
    } else {
        literal!("solid")
    };
    decorate = boolString(isBroken);
    fontColor = if (isBroken) { literal!("red") } else { literal!("green") };
    labelFontSize = if (isBroken) {
        literal!("labelfontsize = 20.0, ")
    } else {
        literal!("")
    };
    sc1 = ComponentRef::toString(&edge.lhs)?;
    sc2 = ComponentRef::toString(&edge.rhs)?;
    out = stringAppendList(list![
        literal!("\""),
        sc1,
        literal!("\" -- \""),
        sc2,
        literal!("\" ["),
        literal!("dir = \"none\", "),
        literal!("style = "),
        style,
        literal!(", "),
        literal!("decorate = "),
        decorate,
        literal!(", "),
        literal!("color = "),
        color,
        literal!(", "),
        labelFontSize,
        literal!("fontcolor = "),
        fontColor,
        literal!(", "),
        literal!("label = \""),
        label,
        literal!("\""),
        literal!("];\n\t")
    ]);
    Ok(out)
}

fn FlatEdgeIsEqual(mut inEdge1: &FlatEdge, mut inEdge2: &FlatEdge) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = ComponentRef::isEqual(&inEdge1.lhs, &inEdge2.lhs)? && ComponentRef::isEqual(&inEdge1.rhs, &inEdge2.rhs)?;
    Ok(isEqual)
}

fn graphVizDefiniteRoot(mut inDefiniteRoot: DefiniteRoot, mut inFinalRoots: &DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match &(inDefiniteRoot) {
        c => {
            let mut strDefiniteRoot: ArcStr;
            let mut isSelectedRoot: bool;
            isSelectedRoot = List::isMemberOnTrue(c.clone(), inFinalRoots, &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1))?;
            strDefiniteRoot = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = red, rank = \"source\", label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot) {literal!("shape=polygon, sides=8, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) };
            strDefiniteRoot
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn graphVizPotentialRoot(mut inPotentialRoot: &PotentialRoot, mut inFinalRoots: &DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match &(inPotentialRoot) {
        (c, priority) => {
            let mut strPotentialRoot: ArcStr;
            let mut isSelectedRoot: bool;
            isSelectedRoot = List::isMemberOnTrue(c.clone(), inFinalRoots, &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1))?;
            strPotentialRoot = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = orangered, rank = \"min\" label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\\n")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot) {literal!("shape=ploygon, sides=7, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) };
            strPotentialRoot
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn generateGraphViz(
    mut modelNameQualified: ArcStr,
    mut definiteRoots: DefiniteRoots,
    mut potentialRoots: PotentialRoots,
    mut uniqueRoots: &UniqueRoots,
    mut branches: Edges,
    mut connections: FlatEdges,
    mut finalRoots: DefiniteRoots,
    mut broken: FlatEdges,
) -> Result<ArcStr> {
    let mut brokenConnectsViaGraphViz: ArcStr;
    brokenConnectsViaGraphViz = 'mc: {
        let __mc_input = &*broken;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (boolOr(Flags::isSet(Flags::CGRAPH_GRAPHVIZ_FILE.clone())?, Flags::isSet(Flags::CGRAPH_GRAPHVIZ_SHOW.clone())?)) else { return Err("pattern mismatch") };
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut fileName: ArcStr;
                    let mut i: ArcStr;
                    let mut nrDR: ArcStr;
                    let mut nrPR: ArcStr;
                    let mut nrUR: ArcStr;
                    let mut nrBR: ArcStr;
                    let mut nrCO: ArcStr;
                    let mut nrFR: ArcStr;
                    let mut nrBC: ArcStr;
                    let mut timeStr: ArcStr;
                    let mut infoNodeStr: ArcStr;
                    let mut brokenConnects: ArcStr;
                    let mut tStart: metamodelica::Real;
                    let mut tEnd: metamodelica::Real;
                    let mut t: metamodelica::Real;
                    let mut graphVizStream: IOStream::IOStream;
                    let mut infoNode: metamodelica::List<ArcStr>;
                    tStart = clock();
                    i = literal!("\t");
                    fileName = stringAppend(modelNameQualified.clone(), literal!(".gv"));
                    graphVizStream = IOStream::create(fileName.clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
                    nrDR = intString(((definiteRoots).len() as i32));
                    nrPR = intString(((potentialRoots).len() as i32));
                    nrUR = intString(((uniqueRoots).len() as i32));
                    nrBR = intString(((branches).len() as i32));
                    nrCO = intString(((connections).len() as i32));
                    nrFR = intString(((finalRoots).len() as i32));
                    nrBC = intString(((broken).len() as i32));
                    infoNode = list![literal!("// Generated by OpenModelica.\n"), literal!("// Overconstrained connection graph for model:\n//    "), modelNameQualified.clone(), literal!("\n"), literal!("//\n"), literal!("// Summary:\n"), literal!("//   Roots:                      "), nrDR.clone(), literal!("\n"), literal!("//   Potential Roots:    "), nrPR.clone(), literal!("\n"), literal!("//   Unique Roots:       "), nrUR.clone(), literal!("\n"), literal!("//   Branches:           "), nrBR.clone(), literal!("\n"), literal!("//   Connections:        "), nrCO.clone(), literal!("\n"), literal!("//   Final Roots:        "), nrFR.clone(), literal!("\n"), literal!("//   Broken Connections: "), nrBC.clone(), literal!("\n")];
                    infoNodeStr = stringAppendList(infoNode.clone());
                    infoNodeStr = System::stringReplace(infoNodeStr.clone(), literal!("\n"), literal!("\\l"))?;
                    infoNodeStr = System::stringReplace(infoNodeStr.clone(), literal!("\t"), literal!(" "))?;
                    infoNodeStr = System::stringReplace(infoNodeStr.clone(), literal!("/"), literal!(""))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &infoNode)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n\n")]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("graph \""), modelNameQualified.clone(), literal!("\"\n{\n\n")]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![i.clone(), literal!("overlap=false;\n")]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![i.clone(), literal!("layout=dot;\n\n")]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![i.clone(), literal!("node ["), literal!("fillcolor = \"lightsteelblue1\", "), literal!("shape = box, "), literal!("style = \"bold, filled\", "), literal!("rank = \"max\""), literal!("]\n\n")]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![i.clone(), literal!("edge ["), literal!("color = \"black\", "), literal!("style = bold"), literal!("]\n\n")]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![i.clone(), literal!("graph [fontsize=20, fontname = \"Courier Bold\" label= \"\\n\\n"), infoNodeStr.clone(), literal!("\", size=\"6,6\"];\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n"), i.clone(), literal!("// Definite Roots (Connections.root)"), literal!("\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map1(definiteRoots.clone(), &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>| graphVizDefiniteRoot(__a0, &__a1), finalRoots.clone())?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n"), i.clone(), literal!("// Potential Roots (Connections.potentialRoot)"), literal!("\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map1(potentialRoots.clone(), &move |__a0: (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real), __a1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>| graphVizPotentialRoot(&__a0, &__a1), finalRoots.clone())?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n"), i.clone(), literal!("// Branches (Connections.branch)"), literal!("\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map(branches.clone(), &move |__a0: (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>)| graphVizEdge(&__a0))?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n"), i.clone(), literal!("// Connections (connect)"), literal!("\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map1(connections.clone(), &move |__a0: NFConnections::BrokenEdge, __a1: metamodelica::List<NFConnections::BrokenEdge>| graphVizFlatEdge(__a0, &__a1), broken.clone())?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n}\n")]))?;
                    tEnd = clock();
                    t = tEnd - tStart;
                    timeStr = realString(t);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n\n\n// graph generation took: "), timeStr.clone(), literal!(" seconds\n")]))?;
                    System::writeFile(fileName.clone(), IOStream::string(&graphVizStream)?)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz with connection graph for model: ")); __mm_s.push_str(&*modelNameQualified); __mm_s.push_str(&*literal!(" was writen to file: ")); __mm_s.push_str(&*fileName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    brokenConnects = showGraphViz(&fileName, &modelNameQualified)?;
                    Ok(brokenConnects.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(brokenConnectsViaGraphViz)
}

fn showGraphViz(mut fileNameGraphViz: &ArcStr, mut modelNameQualified: &ArcStr) -> Result<ArcStr> {
    let mut brokenConnectsViaGraphViz: ArcStr;
    brokenConnectsViaGraphViz = 'mc: {
        let __mc_input = modelNameQualified.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (Flags::isSet(Flags::CGRAPH_GRAPHVIZ_SHOW.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(literal!(""))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut leftyCMD: ArcStr;
            let mut fileNameTraceRemovedConnections: ArcStr;
            let mut omhome: ArcStr;
            let mut brokenConnects: ArcStr;
            let mut leftyExitStatus: i32;
            fileNameTraceRemovedConnections = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*modelNameQualified);
                __mm_s.push_str(&*literal!("_removed_connections.txt"));
                ArcStr::from(__mm_s)
            };
            metamodelica::print(literal!(
                "Tyring to start GraphViz *lefty* to visualize the graph. You need to have lefty in your PATH variable\n"
            ));
            metamodelica::print(literal!(
                "Make sure you quit GraphViz *lefty* via Right Click->quit to be sure the process will be exited.\n"
            ));
            metamodelica::print(literal!(
                "If you quit the GraphViz *lefty* window via X, please kill the process in task manager to continue.\n"
            ));
            omhome = Settings::getInstallationDirectoryPath()?;
            omhome = System::stringReplace(omhome.clone(), literal!("\""), literal!(""))?;
            leftyCMD = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("load('"));
                __mm_s.push_str(&*omhome);
                __mm_s.push_str(&*literal!("/share/omc/scripts/openmodelica.lefty');"));
                __mm_s.push_str(&*literal!("openmodelica.init();openmodelica.createviewandgraph('"));
                __mm_s.push_str(&*fileNameGraphViz);
                __mm_s.push_str(&*literal!("','file',null,null);txtview('off');"));
                ArcStr::from(__mm_s)
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Running command: "));
                __mm_s.push_str(&*literal!("lefty -e "));
                __mm_s.push_str(&*leftyCMD);
                __mm_s.push_str(&*literal!(" > "));
                __mm_s.push_str(&*fileNameTraceRemovedConnections);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            leftyExitStatus = System::systemCall(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("lefty -e "));
                    __mm_s.push_str(&*leftyCMD);
                    ArcStr::from(__mm_s)
                },
                fileNameTraceRemovedConnections.clone(),
            );
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("GraphViz *lefty* exited with status:"));
                __mm_s.push_str(&*intString(leftyExitStatus));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            brokenConnects = System::readFile(fileNameTraceRemovedConnections.clone())?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "GraphViz OpenModelica assistant returned the following broken connects: "
                ));
                __mm_s.push_str(&*brokenConnects);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(brokenConnects.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(brokenConnectsViaGraphViz)
}

fn removeBrokenConnects(
    mut inEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut inConnected: &FlatEdges,
    mut inBroken: &FlatEdges,
    mut isDeleted: &dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    outEquations = ({
        let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
        let mut isThere: bool = false;
        (::match_deref::match_deref! { match inBroken {
            Deref @ metamodelica::ListNode::Nil => {
                inEquations
            },
            _ => {
                let mut lhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut rhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut r#str: ArcStr;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                for mut eq in &*inEquations {
                    eql = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ Equation::CONNECT { lhs: Deref @ Expression::CREF { ty: _, cref: __esc_lhs }, rhs: Deref @ Expression::CREF { ty: _, cref: __esc_rhs }, source: __esc_source, .. } => {
                lhs = (*__esc_lhs).clone();
                rhs = (*__esc_rhs).clone();
                source = (*__esc_source).clone();
                if !(isDeleted(lhs.clone())? || isDeleted(rhs.clone())?) {
                    isThere = false;
                    for mut b in &**inBroken {
                        if ComponentRef::isEqual(&b.lhs, metamodelica::AsArg::as_arg(&lhs))? && ComponentRef::isEqual(&b.rhs, metamodelica::AsArg::as_arg(&rhs))? || ComponentRef::isEqual(&b.rhs, metamodelica::AsArg::as_arg(&lhs))? && ComponentRef::isEqual(&b.lhs, metamodelica::AsArg::as_arg(&rhs))? {
                            isThere = true;
                            break;
                        }
                    }
                }
                if !(isThere) {
                    eql = metamodelica::cons(eq.clone(), eql);
                }
                eql
            },
            _ => metamodelica::cons(eq.clone(), eql),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                }
                eql = metamodelica::Dangerous::listReverseInPlace(eql);
                if Flags::isSet(Flags::CGRAPH.clone())? {
                    r#str = literal!("");
                    for mut b in &**inBroken {
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*ComponentRef::toString(&b.lhs)?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(&b.rhs)?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) };
                    }
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.removeBrokenConnects:\n")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                }
                eql
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(outEquations)
}

fn identifyConnectionsOperator(mut functionName: &metamodelica::Ref<Absyn::Path>) -> ConnectionsOperator {
    let mut call: ConnectionsOperator;
    call = (::match_deref::match_deref! { match functionName {
        Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name } } => {
            (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "branch" => ConnectionsOperator::BRANCH.clone(),
        Deref @ "root" => ConnectionsOperator::ROOT.clone(),
        Deref @ "potentialRoot" => ConnectionsOperator::POTENTIAL_ROOT.clone(),
        Deref @ "isRoot" => ConnectionsOperator::IS_ROOT.clone(),
        Deref @ "rooted" => ConnectionsOperator::ROOTED.clone(),
        Deref @ "uniqueRoot" => ConnectionsOperator::UNIQUE_ROOT.clone(),
        Deref @ "uniqueRootIndices" => ConnectionsOperator::UNIQUE_ROOT_INDICES.clone(),
        _ => ConnectionsOperator::NOT_OPERATOR.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "rooted" } => {
            ConnectionsOperator::ROOTED.clone()
        },
        _ => {
            ConnectionsOperator::NOT_OPERATOR.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    call
}

fn newCrefCrefTable() -> CrefCrefTable {
    let mut table: CrefCrefTable;
    table = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    table
}
