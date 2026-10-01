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

use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnections::BrokenEdges;
use crate::NFConnector as Connector;
use openmodelica_util::DisjointSets;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub mod ConnectionSets {
    use super::*;
    pub(crate) fn EntryHash(mut entry: Entry) -> Result<i32> {
        let mut hash: i32;
        hash = Connector::hash(&entry)?;
        Ok(hash)
    }

    pub(crate) fn EntryEqual(mut entry1: Entry, mut entry2: Entry) -> Result<bool> {
        let mut isEqual: bool;
        isEqual = Connector::isEqual(&entry1, &entry2)?;
        Ok(isEqual)
    }

    pub(crate) fn EntryString(mut entry: Entry) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = Connector::toString(&entry)?;
        Ok(r#str)
    }

    pub(crate) fn fromConnections(mut connections: &metamodelica::Ref<Connections::NFConnections>) -> Result<Sets> {
        let mut sets: Sets;
        sets = emptySets(((connections.connections).len() as i32) + ((connections.flows).len() as i32));
        if !(Flags::isSet(Flags::DISABLE_SINGLE_FLOW_EQ.clone())?) {
            sets = List::fold(&connections.flows, &addSingleConnector, sets)?;
        }
        sets = List::fold1(
            &connections.connections,
            &move |__a0: metamodelica::Ref<Connection::NFConnection>,
                   __a1: metamodelica::List<Connections::BrokenEdge>,
                   __a2: Sets| addConnection(&__a0, &__a1, __a2),
            connections.broken.clone(),
            sets,
        )?;
        Ok(sets)
    }

    pub(crate) fn addScalarConnector(
        mut conn: metamodelica::Ref<Connector::NFConnector>,
        mut sets: Sets,
    ) -> Result<Sets> {
        let mut sets: Sets = sets;
        (sets, _) = add(conn, sets)?;
        Ok(sets)
    }

    pub(crate) fn addConnector(mut conn: &metamodelica::Ref<Connector::NFConnector>, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        sets = addList(&(Connector::scalarize(conn)?), sets)?;
        Ok(sets)
    }

    pub(crate) fn addSingleConnector(
        mut conn: metamodelica::Ref<Connector::NFConnector>,
        mut sets: Sets,
    ) -> Result<Sets> {
        let mut sets: Sets = sets;
        (sets, _) = find(conn, sets)?;
        Ok(sets)
    }

    pub(crate) fn addConnection(
        mut connection: &metamodelica::Ref<Connection::NFConnection>,
        mut broken: &metamodelica::List<Connections::BrokenEdge>,
        mut sets: Sets,
    ) -> Result<Sets> {
        let mut sets: Sets = sets;
        if !((broken).is_empty()) && isBroken(&connection.lhs, &connection.rhs, broken)? {
            return Ok(sets);
        }
        sets = merge(connection.lhs.clone(), connection.rhs.clone(), sets)?;
        Ok(sets)
    }

    pub(crate) fn isBroken(
        mut c1: &metamodelica::Ref<Connector::NFConnector>,
        mut c2: &metamodelica::Ref<Connector::NFConnector>,
        mut broken: &metamodelica::List<Connections::BrokenEdge>,
    ) -> Result<bool> {
        let mut b: bool = false;
        let mut cr1: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut cr2: metamodelica::Ref<ComponentRef::NFComponentRef>;
        cr1 = Connector::name(c1);
        cr2 = Connector::name(c2);
        for mut c in &**broken {
            if ComponentRef::isPrefix(&c.lhs, &cr1)? && ComponentRef::isPrefix(&c.rhs, &cr2)?
                || ComponentRef::isPrefix(&c.lhs, &cr2)? && ComponentRef::isPrefix(&c.rhs, &cr1)?
            {
                b = true;
                break;
            }
        }
        Ok(b)
    }

    pub type Entry = metamodelica::Ref<Connector::NFConnector>;

    pub type IndexTable = metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Connector::NFConnector>, i32>>;

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
        mut entries: &metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
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
                    dyn ::std::ops::Fn(metamodelica::Ref<Connector::NFConnector>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(EntryEqual)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Connector::NFConnector>,
                            metamodelica::Ref<Connector::NFConnector>,
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
        metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
        Sets,
    )> {
        let mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>;
        let mut assignedSets: Sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut set_idx: i32 = 0;
        let mut idx: i32;
        let mut entries: metamodelica::Array<(metamodelica::Ref<Connector::NFConnector>, i32)>;
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
    ) -> Result<Option<metamodelica::Ref<Connector::NFConnector>>> {
        let mut outEntry: Option<metamodelica::Ref<Connector::NFConnector>>;
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
        let mut entries: metamodelica::List<(metamodelica::Ref<Connector::NFConnector>, i32)>;
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
