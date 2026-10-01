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

use crate::ConnectUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTable3;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an edge is a tuple with two component references
pub type Edge = (
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
);

/// A list of edges
pub type Edges = metamodelica::List<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
)>;

/// a tuple with two crefs and dae elements for equatityConstraint function call
pub type DaeEdge = (
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
);

/// A list of edges, each edge associated with two lists of DAE elements
/// (these elements represent equations to be added if the edge
/// is broken)
pub type DaeEdges = metamodelica::List<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)>;

/// root defined with Connection.root
pub type DefiniteRoot = metamodelica::Ref<DAE::ComponentRef>;

/// roots defined with Connection.root
pub type DefiniteRoots = metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;

/// roots defined with Connection.uniqueRoot
pub type UniqueRoots = metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;

/// potential root defined with Connections.potentialRoot
pub type PotentialRoot = (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real);

/// potential roots defined with Connections.potentialRoot
pub type PotentialRoots = metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real)>;

/// Input structure for connection breaking algorithm. It is collected during instantiation phase.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ConnectionGraph {
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
    pub connections: DaeEdges,
}

impl metamodelica::gc::MMTrace for ConnectionGraph {
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
impl Default for ConnectionGraph {
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

pub type GRAPH = ConnectionGraph;

thread_local! { static __EMPTY_TLS: ConnectionGraph = ConnectionGraph { updateGraph: true, definiteRoots: metamodelica::nil(), potentialRoots: metamodelica::nil(), uniqueRoots: metamodelica::nil(), branches: metamodelica::nil(), connections: metamodelica::nil() }; }
pub fn EMPTY() -> ConnectionGraph {
    __EMPTY_TLS.with(|__t| __t.clone())
}

thread_local! { static __NOUPDATE_EMPTY_TLS: ConnectionGraph = ConnectionGraph { updateGraph: false, definiteRoots: metamodelica::nil(), potentialRoots: metamodelica::nil(), uniqueRoots: metamodelica::nil(), branches: metamodelica::nil(), connections: metamodelica::nil() }; }
pub(crate) fn NOUPDATE_EMPTY() -> ConnectionGraph {
    __NOUPDATE_EMPTY_TLS.with(|__t| __t.clone())
}

pub(crate) fn handleOverconstrainedConnections(
    mut inGraph: ConnectionGraph,
    mut modelNameQualified: &ArcStr,
    mut inDAE: DAE::DAElist,
) -> Result<(DAE::DAElist, DaeEdges, DaeEdges)> {
    let mut outDAE: DAE::DAElist;
    let mut outConnected: DaeEdges;
    let mut outBroken: DaeEdges;
    (outDAE, outConnected, outBroken) = 'mc: {
        let __mc_input = (inGraph, &inDAE);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ConnectionGraph { updateGraph: _, definiteRoots: Deref @ metamodelica::ListNode::Nil, potentialRoots: Deref @ metamodelica::ListNode::Nil, uniqueRoots: Deref @ metamodelica::ListNode::Nil, branches: Deref @ metamodelica::ListNode::Nil, connections: Deref @ metamodelica::ListNode::Nil }, _) => {
                    Ok((inDAE.clone(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (graph, DAE::DAElist { elementLst: elts }) => {
                    let mut roots: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut broken: DaeEdges;
                    let mut connected: DaeEdges;
                    let mut elts = (*elts).clone();
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Summary: \n\t")); __mm_s.push_str(&*literal!("Nr Roots:           ")); __mm_s.push_str(&*intString((((getDefiniteRoots(metamodelica::AsArg::as_arg(&graph)))).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Potential Roots: ")); __mm_s.push_str(&*intString((((getPotentialRoots(metamodelica::AsArg::as_arg(&graph)))).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Unique Roots:    ")); __mm_s.push_str(&*intString((((getUniqueRoots(metamodelica::AsArg::as_arg(&graph)))).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Branches:        ")); __mm_s.push_str(&*intString((((getBranches(metamodelica::AsArg::as_arg(&graph)))).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Connections:     ")); __mm_s.push_str(&*intString((((getConnections(metamodelica::AsArg::as_arg(&graph)))).len() as i32))); ArcStr::from(__mm_s) })?;
                    }
                    (roots, connected, broken) = findResultGraph(metamodelica::AsArg::as_arg(&graph), modelNameQualified)?;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(roots.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!(", "))); ArcStr::from(__mm_s) })?;
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Broken connections: ")); __mm_s.push_str(&*stringDelimitList(List::map1(broken.clone(), &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::Element>>), __a1: ArcStr| printConnectionStr(&__a0, &__a1), literal!("broken"))?, literal!(", "))); ArcStr::from(__mm_s) })?;
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowed connections: ")); __mm_s.push_str(&*stringDelimitList(List::map1(connected.clone(), &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::Element>>), __a1: ArcStr| printConnectionStr(&__a0, &__a1), literal!("allowed"))?, literal!(", "))); ArcStr::from(__mm_s) })?;
                    }
                    elts = evalConnectionsOperators(roots.clone(), graph.clone(), elts.clone())?;
                    Ok((DAE::DAElist { elementLst: elts.clone() }, connected.clone(), broken.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::CGRAPH.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.handleOverconstrainedConnections failed for model: ")); __mm_s.push_str(&*modelNameQualified); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outDAE, outConnected, outBroken))
}

pub(crate) fn addDefiniteRoot(
    mut inGraph: &ConnectionGraph,
    mut inRoot: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (match inGraph.clone() {
        ConnectionGraph {
            updateGraph: mut updateGraph,
            definiteRoots: mut definiteRoots,
            potentialRoots: mut potentialRoots,
            uniqueRoots: mut uniqueRoots,
            branches: mut branches,
            connections: mut connections,
        } => {
            let mut root = inRoot;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- ConnectionGraph.addDefiniteRoot("));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&root)?);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                })?;
            }
            ConnectionGraph {
                updateGraph: updateGraph.clone(),
                definiteRoots: metamodelica::cons(root, definiteRoots.clone()),
                potentialRoots: potentialRoots.clone(),
                uniqueRoots: uniqueRoots.clone(),
                branches: branches.clone(),
                connections: connections.clone(),
            }
        }
    });
    Ok(outGraph)
}

pub(crate) fn addPotentialRoot(
    mut inGraph: &ConnectionGraph,
    mut inRoot: metamodelica::Ref<DAE::ComponentRef>,
    mut inPriority: metamodelica::Real,
) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (match inGraph.clone() {
        ConnectionGraph {
            updateGraph: mut updateGraph,
            definiteRoots: mut definiteRoots,
            potentialRoots: mut potentialRoots,
            uniqueRoots: mut uniqueRoots,
            branches: mut branches,
            connections: mut connections,
        } => {
            let mut root = inRoot;
            let mut priority = inPriority;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- ConnectionGraph.addPotentialRoot("));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&root)?);
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*realString(priority));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                })?;
            }
            ConnectionGraph {
                updateGraph: updateGraph.clone(),
                definiteRoots: definiteRoots.clone(),
                potentialRoots: metamodelica::cons((root, priority), potentialRoots.clone()),
                uniqueRoots: uniqueRoots.clone(),
                branches: branches.clone(),
                connections: connections.clone(),
            }
        }
    });
    Ok(outGraph)
}

pub(crate) fn addUniqueRoots<'__b>(
    mut inGraph: ConnectionGraph,
    mut inRoots: metamodelica::Ref<DAE::Exp>,
    mut inMessage: &'__b metamodelica::Ref<DAE::Exp>,
) -> Result<ConnectionGraph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inGraph.clone(), inRoots)) {
            (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, Deref @ DAE::Exp::CREF { componentRef: root, ty: _ }) => {
                if Flags::isSet(Flags::CGRAPH.clone())? {
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addUniqueRoots(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&root))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inMessage.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
                }
                return Ok(ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: metamodelica::cons((root.clone(), inMessage.clone()), uniqueRoots.clone()), branches: branches.clone(), connections: connections.clone() })
            },
            (ConnectionGraph { .. }, Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: Deref @ metamodelica::ListNode::Nil }) => {
                return Ok(inGraph)
            },
            (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, Deref @ DAE::Exp::ARRAY { ty, scalar, array: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: root, ty: _ }, tail: rest } }) => {
                let mut graph: ConnectionGraph;
                if Flags::isSet(Flags::CGRAPH.clone())? {
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addUniqueRoots(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&root))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inMessage.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
                }
                graph = ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: metamodelica::cons((root.clone(), inMessage.clone()), uniqueRoots.clone()), branches: branches.clone(), connections: connections.clone() };
                { (inGraph, inRoots, inMessage) = (graph, metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: scalar.clone(), array: rest.clone() }), inMessage); continue '__tco; }
            },
            (_, _) => {
                return Ok(inGraph)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn addBranch(
    mut inGraph: &ConnectionGraph,
    mut inRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inRef2: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (match inGraph.clone() {
        ConnectionGraph {
            updateGraph: mut updateGraph,
            definiteRoots: mut definiteRoots,
            potentialRoots: mut potentialRoots,
            uniqueRoots: mut uniqueRoots,
            branches: mut branches,
            connections: mut connections,
        } => {
            let mut ref1 = inRef1;
            let mut ref2 = inRef2;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- ConnectionGraph.addBranch("));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&ref1)?);
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&ref2)?);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                })?;
            }
            ConnectionGraph {
                updateGraph: updateGraph.clone(),
                definiteRoots: definiteRoots.clone(),
                potentialRoots: potentialRoots.clone(),
                uniqueRoots: uniqueRoots.clone(),
                branches: metamodelica::cons((ref1, ref2), branches.clone()),
                connections: connections.clone(),
            }
        }
    });
    Ok(outGraph)
}

pub(crate) fn addConnection(
    mut inGraph: &ConnectionGraph,
    mut inRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inRef2: metamodelica::Ref<DAE::ComponentRef>,
    mut inDae: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (match inGraph.clone() {
        ConnectionGraph {
            updateGraph: mut updateGraph,
            definiteRoots: mut definiteRoots,
            potentialRoots: mut potentialRoots,
            uniqueRoots: mut uniqueRoots,
            branches: mut branches,
            connections: mut connections,
        } => {
            let mut ref1 = inRef1;
            let mut ref2 = inRef2;
            let mut dae = inDae;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::trace({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- ConnectionGraph.addConnection("));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&ref1)?);
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&ref2)?);
                    __mm_s.push_str(&*literal!(")\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
            ConnectionGraph {
                updateGraph: updateGraph.clone(),
                definiteRoots: definiteRoots.clone(),
                potentialRoots: potentialRoots.clone(),
                uniqueRoots: uniqueRoots.clone(),
                branches: branches.clone(),
                connections: metamodelica::cons((ref1, ref2, dae), connections.clone()),
            }
        }
    });
    Ok(outGraph)
}

// ************************************* //
// ********* protected section ********* //
// ************************************* //
fn canonical(
    mut inPartition: (
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
    mut inRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCanonical: metamodelica::Ref<DAE::ComponentRef>;
    outCanonical = 'mc: {
        let __mc_input = (inPartition, inRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, r#ref) => {
                    let mut parent: metamodelica::Ref<DAE::ComponentRef>;
                    let mut parentCanonical: metamodelica::Ref<DAE::ComponentRef>;
                    parent = BaseHashTable::get(r#ref.clone(), &(partition.clone()))?;
                    parentCanonical = canonical(partition.clone(), parent.clone())?;
                    Ok(parentCanonical.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#ref) => {
                    Ok(r#ref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCanonical)
}

fn areInSameComponent(
    mut inPartition: (
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
    mut inRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inRef2: metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut outResult: bool;
    outResult = 'mc: {
        let __mc_input = (inPartition, inRef1, inRef2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let mut canon1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut canon2: metamodelica::Ref<DAE::ComponentRef>;
                    canon1 = canonical(partition.clone(), ref1.clone())?;
                    canon2 = canonical(partition.clone(), ref2.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(&canon1, &canon2)?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outResult
}

fn connectBranchComponents(
    mut inPartition: (
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
    mut inRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inRef2: metamodelica::Ref<DAE::ComponentRef>,
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
    let mut outPartition: (
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
    outPartition = 'mc: {
        let __mc_input = (inPartition, inRef1, inRef2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let mut canon1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut canon2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut partition = (*partition).clone();
                    canon1 = canonical(partition.clone(), ref1.clone())?;
                    canon2 = canonical(partition.clone(), ref2.clone())?;
                    let (__pa0, true) = (connectCanonicalComponents(partition.clone(), canon1.clone(), canon2.clone())?) else { return Err("pattern mismatch") };
                    partition = metamodelica::Own::own(__pa0);
                    Ok(partition.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, _, _) => {
                    Ok(partition.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPartition)
}

fn connectComponents(
    mut inPartition: (
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
    mut inDaeEdge: DaeEdge,
) -> Result<(
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
    DaeEdges,
    DaeEdges,
)> {
    let mut outPartition: (
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
    let mut outConnectedConnections: DaeEdges;
    let mut outBrokenConnections: DaeEdges;
    (outPartition, outConnectedConnections, outBrokenConnections) = 'mc: {
        let __mc_input = (inPartition, &inDaeEdge);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (ref1, _, _)) => {
                    if '__try0: {
                        unwrap_break_err!(canonical(partition.clone(), ref1.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((partition.clone(), list![inDaeEdge.clone()], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (_, ref2, _)) => {
                    if '__try0: {
                        unwrap_break_err!(canonical(partition.clone(), ref2.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((partition.clone(), list![inDaeEdge.clone()], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (ref1, ref2, _)) => {
                    let mut canon1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut canon2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut partition = (*partition).clone();
                    canon1 = canonical(partition.clone(), ref1.clone())?;
                    canon2 = canonical(partition.clone(), ref2.clone())?;
                    let (__pa0, true) = (connectCanonicalComponents(partition.clone(), canon1.clone(), canon2.clone())?) else { return Err("pattern mismatch") };
                    partition = metamodelica::Own::own(__pa0);
                    Ok((partition.clone(), list![inDaeEdge.clone()], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (ref1, ref2, _)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.connectComponents: should remove equations generated from: connect(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&ref1))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&ref2))?); __mm_s.push_str(&*literal!(") and add {0, ..., 0} = equalityConstraint(cr1, cr2) instead.\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((partition.clone(), metamodelica::nil(), list![inDaeEdge.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outPartition, outConnectedConnections, outBrokenConnections))
}

fn connectCanonicalComponents(
    mut inPartition: (
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
    mut inRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inRef2: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
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
    bool,
)> {
    let mut outPartition: (
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
    let mut outReallyConnected: bool;
    (outPartition, outReallyConnected) = 'mc: {
        let __mc_input = (inPartition, inRef1, inRef2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&ref1), metamodelica::AsArg::as_arg(&ref2))?) else { return Err("pattern mismatch") };
                    Ok((partition.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let mut partition = (*partition).clone();
                    partition = BaseHashTable::add((ref1.clone(), ref2.clone()), partition.clone())?;
                    Ok((partition.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outPartition, outReallyConnected))
}

fn addRootsToTable(
    mut inTable: (
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
    mut inRoots: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inFirstRoot: metamodelica::Ref<DAE::ComponentRef>,
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
    '__tco: loop {
        ::match_deref::match_deref! { match &((inTable, inRoots, inFirstRoot)) {
            (table, Deref @ metamodelica::ListNode::Cons { head: root, tail: tail }, firstRoot) => {
                let mut table = (*table).clone();
                table = BaseHashTable::add((root.clone(), firstRoot.clone()), table.clone())?;
                { (inTable, inRoots, inFirstRoot) = (table.clone(), tail.clone(), firstRoot.clone()); continue '__tco; }
            },
            (table, Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(table.clone())
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn resultGraphWithRoots(
    mut roots: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
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
    let mut outTable: (
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
    let mut table0: (
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
    let mut dummyRoot: metamodelica::Ref<DAE::ComponentRef>;
    dummyRoot = ComponentReferenceBasics::makeCrefIdent(
        literal!("__DUMMY_ROOT"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
    );
    table0 = HashTableCG::emptyHashTable();
    outTable = addRootsToTable(table0, roots, dummyRoot)?;
    Ok(outTable)
}

fn addBranchesToTable(
    mut inTable: (
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
    mut inBranches: Edges,
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
    '__tco: loop {
        ::match_deref::match_deref! { match &((inTable, inBranches)) {
            (table, Deref @ metamodelica::ListNode::Cons { head: (ref1, ref2), tail: tail }) => {
                let mut table1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                let mut table2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                table1 = connectBranchComponents(table.clone(), ref1.clone(), ref2.clone())?;
                { (inTable, inBranches) = (table1, tail.clone()); continue '__tco; }
            },
            (table, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(table.clone())
            },
            _ => return Err("match: no arm matched"),
        } }
    }
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
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
                    s2 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?;
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
    mut inTable: (
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
    mut inPotentialRoots: &PotentialRoots,
    mut inRoots: DefiniteRoots,
    mut inFirstRoot: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
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
    DefiniteRoots,
)> {
    let mut outTable: (
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
    let mut outRoots: DefiniteRoots;
    (outTable, outRoots) = 'mc: {
        let __mc_input = (inTable, &**inPotentialRoots, inRoots, inFirstRoot);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (table, Deref @ metamodelica::ListNode::Nil, roots, _) => {
                    Ok((table.clone(), roots.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (table, Deref @ metamodelica::ListNode::Cons { head: (potentialRoot, _), tail: tail }, roots, firstRoot) => {
                    let mut canon1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut canon2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut finalRoots: DefiniteRoots;
                    let mut table = (*table).clone();
                    canon1 = canonical(table.clone(), potentialRoot.clone())?;
                    canon2 = canonical(table.clone(), firstRoot.clone())?;
                    let (__pa0, true) = (connectCanonicalComponents(table.clone(), canon1.clone(), canon2.clone())?) else { return Err("pattern mismatch") };
                    table = metamodelica::Own::own(__pa0);
                    (table, finalRoots) = addPotentialRootsToTable(table.clone(), metamodelica::AsArg::as_arg(&tail), metamodelica::cons(potentialRoot.clone(), roots.clone()), firstRoot.clone())?;
                    Ok((table.clone(), finalRoots.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (table, Deref @ metamodelica::ListNode::Cons { head: _, tail: tail }, roots, firstRoot) => {
                    let mut finalRoots: DefiniteRoots;
                    let mut table = (*table).clone();
                    (table, finalRoots) = addPotentialRootsToTable(table.clone(), metamodelica::AsArg::as_arg(&tail), roots.clone(), firstRoot.clone())?;
                    Ok((table.clone(), finalRoots.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outTable, outRoots))
}

fn addConnections(
    mut inTable: (
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
    mut inConnections: &DaeEdges,
) -> Result<(
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
    DaeEdges,
    DaeEdges,
)> {
    let mut outTable: (
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
    let mut outConnectedConnections: DaeEdges;
    let mut outBrokenConnections: DaeEdges;
    (outTable, outConnectedConnections, outBrokenConnections) = (::match_deref::match_deref! { match inConnections {
        Deref @ metamodelica::ListNode::Nil => {
            let mut table = inTable;
            (table, metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: tail } => {
            let mut table = inTable;
            let mut broken1: DaeEdges;
            let mut broken2: DaeEdges;
            let mut broken: DaeEdges;
            let mut connected1: DaeEdges;
            let mut connected2: DaeEdges;
            let mut connected: DaeEdges;
            (table, connected1, broken1) = connectComponents(table, e.clone())?;
            (table, connected2, broken2) = addConnections(table, tail)?;
            connected = listAppend(connected1, connected2);
            broken = listAppend(broken1, broken2);
            (table, connected, broken)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outTable, outConnectedConnections, outBrokenConnections))
}

fn findResultGraph(
    mut inGraph: &ConnectionGraph,
    mut modelNameQualified: &ArcStr,
) -> Result<(DefiniteRoots, DaeEdges, DaeEdges)> {
    let mut outRoots: DefiniteRoots;
    let mut outConnectedConnections: DaeEdges;
    let mut outBrokenConnections: DaeEdges;
    (outRoots, outConnectedConnections, outBrokenConnections) = (::match_deref::match_deref! { match &(inGraph) {
        ConnectionGraph { definiteRoots: Deref @ metamodelica::ListNode::Nil, potentialRoots: Deref @ metamodelica::ListNode::Nil, uniqueRoots: Deref @ metamodelica::ListNode::Nil, branches: Deref @ metamodelica::ListNode::Nil, connections: Deref @ metamodelica::ListNode::Nil, .. } => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        ConnectionGraph { definiteRoots, potentialRoots, uniqueRoots, branches, connections, .. } => {
            let mut finalRoots: DefiniteRoots;
            let mut orderedPotentialRoots: PotentialRoots;
            let mut broken: DaeEdges;
            let mut connected: DaeEdges;
            let mut table: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
            let mut dummyRoot: metamodelica::Ref<DAE::ComponentRef>;
            let mut brokenConnectsViaGraphViz: ArcStr;
            let mut userBrokenLst: metamodelica::List<ArcStr>;
            let mut userBrokenLstLst: metamodelica::List<metamodelica::List<ArcStr>>;
            let mut userBrokenTplLst: metamodelica::List<(ArcStr, ArcStr)>;
            let mut connections = (*connections).clone();
            connections = connections.clone().reverse();
            table = resultGraphWithRoots(definiteRoots.clone())?;
            table = addBranchesToTable(table, branches.clone())?;
            orderedPotentialRoots = List::sort(potentialRoots.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real), __a1: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real)| -> metamodelica::Result<_> { ::std::result::Result::Ok(ord(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real), (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real)) -> Result<bool> + 'static>))?;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Ordered Potential Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(orderedPotentialRoots.clone(), &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real)| printPotentialRootTuple(&__a0))?, literal!(", "))); ArcStr::from(__mm_s) })?;
            }
            (table, connected, broken) = addConnections(table, metamodelica::AsArg::as_arg(&connections))?;
            dummyRoot = ComponentReferenceBasics::makeCrefIdent(literal!("__DUMMY_ROOT"), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
            (table, finalRoots) = addPotentialRootsToTable(table, &orderedPotentialRoots, definiteRoots.clone(), dummyRoot)?;
            brokenConnectsViaGraphViz = generateGraphViz(modelNameQualified.clone(), definiteRoots.clone(), potentialRoots.clone(), metamodelica::AsArg::as_arg(&uniqueRoots), branches.clone(), connections.clone(), finalRoots.clone(), broken.clone())?;
            if stringEq(&brokenConnectsViaGraphViz, &(literal!(""))) {
            } else {
                userBrokenLst = Util::stringSplitAtChar(brokenConnectsViaGraphViz, literal!("#"))?;
                userBrokenLstLst = List::map1(userBrokenLst, &Util::stringSplitAtChar, literal!("|"))?;
                userBrokenTplLst = makeTuple(&userBrokenLstLst)?;
                Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("User selected the following connect edges for breaking:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(userBrokenTplLst.clone(), &move |__a0: (ArcStr, ArcStr)| -> metamodelica::Result<_> { ::std::result::Result::Ok(printTupleStr(&__a0)) })?, literal!("\n\t"))); ArcStr::from(__mm_s) })?;
                printDaeEdges(metamodelica::AsArg::as_arg(&connections))?;
                connections = orderConnectsGuidedByUser(metamodelica::AsArg::as_arg(&connections), userBrokenTplLst)?;
                connections = connections.clone().reverse();
                metamodelica::print(literal!("\nAfer ordering:\n"));
                (finalRoots, connected, broken) = findResultGraph(&(ConnectionGraph { updateGraph: false, definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }), modelNameQualified)?;
            }
            (finalRoots, connected, broken)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outRoots, outConnectedConnections, outBrokenConnections))
}

fn orderConnectsGuidedByUser(
    mut inConnections: &DaeEdges,
    mut inUserSelectedBreaking: metamodelica::List<(ArcStr, ArcStr)>,
) -> Result<DaeEdges> {
    let mut outOrderedConnections: DaeEdges;
    let mut front: DaeEdges = metamodelica::nil();
    let mut back: DaeEdges = metamodelica::nil();
    let mut c1: metamodelica::Ref<DAE::ComponentRef>;
    let mut c2: metamodelica::Ref<DAE::ComponentRef>;
    let mut sc1: ArcStr;
    let mut sc2: ArcStr;
    for mut e in &**inConnections {
        (c1, c2, _) = e.clone();
        sc1 = ComponentReferenceBasics::printComponentRefStr(&c1)?;
        sc2 = ComponentReferenceBasics::printComponentRefStr(&c2)?;
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following output from GraphViz OpenModelica assistant cannot be parsed:")); __mm_s.push_str(&*stringDelimitList(bad.clone(), literal!(", "))); __mm_s.push_str(&*literal!("\nExpected format from GrapViz: cref1|cref2#cref3|cref4#. Ignoring malformed input.")); ArcStr::from(__mm_s) })?;
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
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStr)
}

fn setRootDistance(
    mut finalRoots: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut table: &(
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
    mut distance: i32,
    mut nextLevel: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut irooted: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut orooted: (
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
    orooted = 'mc: {
        let __mc_input = (&**finalRoots, &**nextLevel);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(irooted.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(setRootDistance(nextLevel, table, distance + 1, &(metamodelica::nil()), irooted)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest }, _) => {
                    let mut rooted: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut next: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let false = (BaseHashTable::hasKey(cr.clone(), irooted)?) else { return Err("pattern mismatch") };
                    rooted = BaseHashTable::add((cr.clone(), distance), irooted.clone())?;
                    next = BaseHashTable::get(cr.clone(), table)?;
                    next = listAppend(nextLevel.clone(), next.clone());
                    Ok(setRootDistance(metamodelica::AsArg::as_arg(&rest), table, distance, &next, &rooted)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest }, _) => {
                    let mut rooted: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let false = (BaseHashTable::hasKey(cr.clone(), irooted)?) else { return Err("pattern mismatch") };
                    rooted = BaseHashTable::add((cr.clone(), distance), irooted.clone())?;
                    Ok(setRootDistance(metamodelica::AsArg::as_arg(&rest), table, distance, nextLevel, &rooted)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    Ok(setRootDistance(metamodelica::AsArg::as_arg(&rest), table, distance, nextLevel, irooted)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(orooted)
}

fn addBranches(
    mut edge: &Edge,
    mut itable: (
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
    let mut otable: (
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
    let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cref2: metamodelica::Ref<DAE::ComponentRef>;
    (cref1, cref2) = edge.clone();
    otable = addConnectionRooted(cref1.clone(), cref2.clone(), itable)?;
    otable = addConnectionRooted(cref2, cref1, otable)?;
    Ok(otable)
}

fn addConnectionsRooted(
    mut connection: &DaeEdge,
    mut itable: (
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
    let mut otable: (
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
    let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cref2: metamodelica::Ref<DAE::ComponentRef>;
    (cref1, cref2, _) = connection.clone();
    otable = addConnectionRooted(cref1.clone(), cref2.clone(), itable)?;
    otable = addConnectionRooted(cref2, cref1, otable)?;
    Ok(otable)
}

fn addConnectionRooted(
    mut cref1: metamodelica::Ref<DAE::ComponentRef>,
    mut cref2: metamodelica::Ref<DAE::ComponentRef>,
    mut itable: (
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
    let mut otable: (
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
    otable = (match itable.clone() {
        _ => {
            let mut table: (
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
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crefs = 'mc: {
                let __mc_input = ();
                if let Ok(__v) = (|| -> Result<_> {
                    let () = __mc_input.clone() else { return Err("nomatch") };
                    Ok(BaseHashTable::get(cref1.clone(), &itable)?)
                })() {
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { return Err("nomatch") };
                    Ok(metamodelica::nil())
                })() {
                    break 'mc __v;
                }
                return Err("matchcontinue: no arm matched");
            };
            table = BaseHashTable::add((cref1, metamodelica::cons(cref2, crefs)), itable)?;
            table
        }
    });
    Ok(otable)
}

fn evalConnectionsOperators(
    mut inRoots: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut graph: ConnectionGraph,
    mut inDae: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outDae: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outDae = (::match_deref::match_deref! { match &(inDae.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        _ => {
            let mut rooted: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut table: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
            let mut branches: Edges;
            let mut connections: DaeEdges;
            table = HashTable3::emptyHashTable();
            branches = getBranches(&graph);
            table = List::fold(&branches, &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>), __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> + 'static>))| addBranches(&__a0, __a1), table)?;
            connections = getConnections(&graph);
            table = List::fold(&connections, &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::Element>>), __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> + 'static>))| addConnectionsRooted(&__a0, __a1), table)?;
            rooted = setRootDistance(&inRoots, &table, 0, &(metamodelica::nil()), &(HashTable::emptyHashTable()))?;
            (outDae, _) = DAEUtil::traverseDAEElementList(inDae, (std::sync::Arc::new(fnptr!(evalConnectionsOperatorsHelper, metamodelica::Ref<DAE::Exp>, ((metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, ConnectionGraph))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ((metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, ConnectionGraph)) -> Result<(metamodelica::Ref<DAE::Exp>, ((metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, ConnectionGraph))> + 'static>), (rooted, inRoots, graph))?;
            outDae
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDae)
}

fn evalConnectionsOperatorsHelper(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inRoots: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ConnectionGraph,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ConnectionGraph,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outRoots: (
        (
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
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ConnectionGraph,
    );
    (outExp, outRoots) = 'mc: {
        let __mc_input = (inExp.clone(), &inRoots);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rooted" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (rooted, roots, graph)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = false")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rooted" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (rooted, roots, graph)) => {
                    let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut result: bool;
                    let mut branches: Edges;
                    branches = getBranches(metamodelica::AsArg::as_arg(&graph));
                    cref1 = getEdge(metamodelica::AsArg::as_arg(&cref), &branches)?;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: Found Branche Partner ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cref1)?); ArcStr::from(__mm_s) })?;
                    }
                    result = getRooted(cref.clone(), cref1.clone(), &(rooted.clone()));
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result)); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::BCONST { bool: result }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (rooted, roots @ Deref @ metamodelica::ListNode::Nil, graph)) => {
                    Ok((exp.clone(), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (rooted, roots, graph)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = false")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, (rooted, roots, graph)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = false")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (rooted, roots, graph)) => {
                    let mut result: bool;
                    result = List::isMemberOnTrue(cref.clone(), metamodelica::AsArg::as_arg(&roots), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1))?;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result)); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::BCONST { bool: result }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, (rooted, roots, graph)) => {
                    let mut result: bool;
                    result = List::isMemberOnTrue(cref.clone(), metamodelica::AsArg::as_arg(&roots), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1))?;
                    result = boolNot(result);
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result)); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::BCONST { bool: result }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "uniqueRootIndices" } }, expLst: Deref @ metamodelica::ListNode::Cons { head: uroots @ Deref @ DAE::Exp::ARRAY { array: lst, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: nodes, tail: Deref @ metamodelica::ListNode::Cons { head: message, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, (rooted, roots, graph)) => {
                    let mut lst = (*lst).clone();
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: Connections.uniqueRootsIndicies(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(uroots.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ExpressionBasics::printExpStr(nodes.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ExpressionBasics::printExpStr(message.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
                    }
                    lst = List::fill(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), ((lst).len() as i32));
                    Ok((metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: lst.clone() }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inRoots.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outRoots)
}

fn getRooted(
    mut cref1: metamodelica::Ref<DAE::ComponentRef>,
    mut cref2: metamodelica::Ref<DAE::ComponentRef>,
    mut rooted: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> bool {
    let mut result: bool;
    result = 'mc: {
        let __mc_input = rooted.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut i1: i32;
            let mut i2: i32;
            i1 = BaseHashTable::get(cref1.clone(), rooted)?;
            i2 = BaseHashTable::get(cref2.clone(), rooted)?;
            Ok(intLt(i1, i2))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(true)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    result
}

fn getEdge(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut edges: &Edges,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut ocr: metamodelica::Ref<DAE::ComponentRef>;
    ocr = 'mc: {
        let __mc_input = &**edges;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (cref1, cref2), tail: _ } => {
                    let mut cref1 = (*cref1).clone();
                    cref1 = getEdge1(cr, cref1.clone(), cref2.clone())?;
                    Ok(cref1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getEdge(cr, metamodelica::AsArg::as_arg(&rest))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ocr)
}

fn getEdge1(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut cref1: metamodelica::Ref<DAE::ComponentRef>,
    mut cref2: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut ocr: metamodelica::Ref<DAE::ComponentRef>;
    ocr = 'mc: {
        let __mc_input = &*cref2;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr, &cref1)?) else { return Err("pattern mismatch") };
                    Ok(cref2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr, &cref2)?) else { return Err("pattern mismatch") };
                    Ok(cref1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ocr)
}

fn printConnectionStr(mut connectTuple: &DaeEdge, mut ty: &ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (::match_deref::match_deref! { match &(connectTuple) {
        (c1, c2, _) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ty); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStr)
}

fn printEdges(mut inEdges: &Edges) -> Result<()> {
    let () = (::match_deref::match_deref! { match inEdges {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (c1, c2), tail: tail } => {
            metamodelica::print(literal!("    "));
            metamodelica::print(ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?);
            metamodelica::print(literal!(" -- "));
            metamodelica::print(ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?);
            metamodelica::print(literal!("\n"));
            printEdges(tail)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printDaeEdges(mut inEdges: &DaeEdges) -> Result<()> {
    let () = (::match_deref::match_deref! { match inEdges {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (c1, c2, _), tail: tail } => {
            metamodelica::print(literal!("    "));
            metamodelica::print(ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?);
            metamodelica::print(literal!(" -- "));
            metamodelica::print(ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?);
            metamodelica::print(literal!("\n"));
            printDaeEdges(tail)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printConnectionGraph(mut inGraph: &ConnectionGraph) -> Result<()> {
    let () = (match inGraph.clone() {
        ConnectionGraph {
            connections: mut connections,
            branches: mut branches,
            ..
        } => {
            metamodelica::print(literal!("Connections:\n"));
            printDaeEdges(metamodelica::AsArg::as_arg(&connections))?;
            metamodelica::print(literal!("Branches:\n"));
            printEdges(metamodelica::AsArg::as_arg(&branches))?;
            ()
        }
    });
    Ok(())
}

fn getDefiniteRoots(mut inGraph: &ConnectionGraph) -> DefiniteRoots {
    let mut outResult: DefiniteRoots;
    outResult = (match inGraph.clone() {
        ConnectionGraph {
            definiteRoots: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

fn getUniqueRoots(mut inGraph: &ConnectionGraph) -> UniqueRoots {
    let mut outResult: UniqueRoots;
    outResult = (match inGraph.clone() {
        ConnectionGraph {
            uniqueRoots: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

fn getPotentialRoots(mut inGraph: &ConnectionGraph) -> PotentialRoots {
    let mut outResult: PotentialRoots;
    outResult = (match inGraph.clone() {
        ConnectionGraph {
            potentialRoots: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

fn getBranches(mut inGraph: &ConnectionGraph) -> Edges {
    let mut outResult: Edges;
    outResult = (match inGraph.clone() {
        ConnectionGraph {
            branches: ref result, ..
        } => result.clone(),
    });
    outResult
}

fn getConnections(mut inGraph: &ConnectionGraph) -> DaeEdges {
    let mut outResult: DaeEdges;
    outResult = (match inGraph.clone() {
        ConnectionGraph {
            connections: ref result,
            ..
        } => result.clone(),
    });
    outResult
}

pub(crate) fn merge(mut inGraph1: ConnectionGraph, mut inGraph2: ConnectionGraph) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (::match_deref::match_deref! { match &((&inGraph1, &inGraph2)) {
        (_, ConnectionGraph { definiteRoots: Deref @ metamodelica::ListNode::Nil, potentialRoots: Deref @ metamodelica::ListNode::Nil, uniqueRoots: Deref @ metamodelica::ListNode::Nil, branches: Deref @ metamodelica::ListNode::Nil, connections: Deref @ metamodelica::ListNode::Nil, .. }) => {
            inGraph1.clone()
        },
        (ConnectionGraph { definiteRoots: Deref @ metamodelica::ListNode::Nil, potentialRoots: Deref @ metamodelica::ListNode::Nil, uniqueRoots: Deref @ metamodelica::ListNode::Nil, branches: Deref @ metamodelica::ListNode::Nil, connections: Deref @ metamodelica::ListNode::Nil, .. }, _) => {
            inGraph2.clone()
        },
        (_, _) if (inGraph1.clone() == inGraph2.clone()) => {
            inGraph1.clone()
        },
        (ConnectionGraph { updateGraph: updateGraph1, definiteRoots: definiteRoots1, potentialRoots: potentialRoots1, uniqueRoots: uniqueRoots1, branches: branches1, connections: connections1 }, ConnectionGraph { updateGraph: updateGraph2, definiteRoots: definiteRoots2, potentialRoots: potentialRoots2, uniqueRoots: uniqueRoots2, branches: branches2, connections: connections2 }) => {
            let mut updateGraph: bool;
            let mut definiteRoots: DefiniteRoots;
            let mut uniqueRoots: UniqueRoots;
            let mut potentialRoots: PotentialRoots;
            let mut branches: Edges;
            let mut connections: DaeEdges;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::trace(literal!("- ConnectionGraph.merge()\n"))?;
            }
            updateGraph = boolOr(updateGraph1.clone(), updateGraph2.clone());
            definiteRoots = List::union(metamodelica::AsArg::as_arg(&definiteRoots1), metamodelica::AsArg::as_arg(&definiteRoots2));
            potentialRoots = List::union(metamodelica::AsArg::as_arg(&potentialRoots1), metamodelica::AsArg::as_arg(&potentialRoots2));
            uniqueRoots = List::union(metamodelica::AsArg::as_arg(&uniqueRoots1), metamodelica::AsArg::as_arg(&uniqueRoots2));
            branches = List::union(metamodelica::AsArg::as_arg(&branches1), metamodelica::AsArg::as_arg(&branches2));
            connections = List::union(metamodelica::AsArg::as_arg(&connections1), metamodelica::AsArg::as_arg(&connections2));
            ConnectionGraph { updateGraph: updateGraph, definiteRoots: definiteRoots, potentialRoots: potentialRoots, uniqueRoots: uniqueRoots, branches: branches, connections: connections }
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
            strEdge = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!("\" -- \"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [color = blue, dir = \"none\", fontcolor=blue, label = \"branch\"];\n\t")); ArcStr::from(__mm_s) };
            strEdge
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn graphVizDaeEdge(mut inDaeEdge: DaeEdge, mut inBrokenDaeEdges: DaeEdges) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match &(inDaeEdge.clone()) {
        (c1, c2, _) => {
            let mut sc1: ArcStr;
            let mut sc2: ArcStr;
            let mut strDaeEdge: ArcStr;
            let mut label: ArcStr;
            let mut labelFontSize: ArcStr;
            let mut decorate: ArcStr;
            let mut color: ArcStr;
            let mut style: ArcStr;
            let mut fontColor: ArcStr;
            let mut isBroken: bool;
            isBroken = listMember(inDaeEdge, inBrokenDaeEdges);
            label = if (isBroken) {literal!("[[broken connect]]")} else {literal!("connect")};
            color = if (isBroken) {literal!("red")} else {literal!("green")};
            style = if (isBroken) {literal!("\"bold, dashed\"")} else {literal!("solid")};
            decorate = boolString(isBroken);
            fontColor = if (isBroken) {literal!("red")} else {literal!("green")};
            labelFontSize = if (isBroken) {literal!("labelfontsize = 20.0, ")} else {literal!("")};
            sc1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
            sc2 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?;
            strDaeEdge = stringAppendList(list![literal!("\""), sc1, literal!("\" -- \""), sc2, literal!("\" ["), literal!("dir = \"none\", "), literal!("style = "), style, literal!(", "), literal!("decorate = "), decorate, literal!(", "), literal!("color = "), color, literal!(", "), labelFontSize, literal!("fontcolor = "), fontColor, literal!(", "), literal!("label = \""), label, literal!("\""), literal!("];\n\t")]);
            strDaeEdge
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn graphVizDefiniteRoot(mut inDefiniteRoot: DefiniteRoot, mut inFinalRoots: DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match &(inDefiniteRoot) {
        c => {
            let mut strDefiniteRoot: ArcStr;
            let mut isSelectedRoot: bool;
            isSelectedRoot = listMember(c.clone(), inFinalRoots);
            strDefiniteRoot = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = red, rank = \"source\", label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot) {literal!("shape=polygon, sides=8, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) };
            strDefiniteRoot
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn graphVizPotentialRoot(mut inPotentialRoot: &PotentialRoot, mut inFinalRoots: DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match &(inPotentialRoot) {
        (c, priority) => {
            let mut strPotentialRoot: ArcStr;
            let mut isSelectedRoot: bool;
            isSelectedRoot = listMember(c.clone(), inFinalRoots);
            strPotentialRoot = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = orangered, rank = \"min\" label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!("\\n")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot) {literal!("shape=ploygon, sides=7, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) };
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
    mut connections: DaeEdges,
    mut finalRoots: DefiniteRoots,
    mut broken: DaeEdges,
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
                    infoNode = list![literal!("// Generated by OpenModelica. \n"), literal!("// Overconstrained connection graph for model: \n//    "), modelNameQualified.clone(), literal!("\n"), literal!("// \n"), literal!("// Summary: \n"), literal!("//   Roots:                      "), nrDR.clone(), literal!("\n"), literal!("//   Potential Roots:    "), nrPR.clone(), literal!("\n"), literal!("//   Unique Roots:       "), nrUR.clone(), literal!("\n"), literal!("//   Branches:           "), nrBR.clone(), literal!("\n"), literal!("//   Connections:        "), nrCO.clone(), literal!("\n"), literal!("//   Final Roots:        "), nrFR.clone(), literal!("\n"), literal!("//   Broken Connections: "), nrBC.clone(), literal!("\n")];
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
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map1(definiteRoots.clone(), &graphVizDefiniteRoot, finalRoots.clone())?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n"), i.clone(), literal!("// Potential Roots (Connections.potentialRoot)"), literal!("\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map1(potentialRoots.clone(), &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real), __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| graphVizPotentialRoot(&__a0, __a1), finalRoots.clone())?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n"), i.clone(), literal!("// Branches (Connections.branch)"), literal!("\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map(branches.clone(), &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)| graphVizEdge(&__a0))?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n"), i.clone(), literal!("// Connections (connect)"), literal!("\n"), i.clone()]))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(List::map1(connections.clone(), &graphVizDaeEdge, broken.clone())?))?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n}\n")]))?;
                    tEnd = clock();
                    t = tEnd - tStart;
                    timeStr = realString(t);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), &(list![literal!("\n\n\n// graph generation took: "), timeStr.clone(), literal!(" seconds\n")]))?;
                    System::writeFile(fileName.clone(), IOStream::string(&graphVizStream)?)?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz with connection graph for model: ")); __mm_s.push_str(&*modelNameQualified); __mm_s.push_str(&*literal!(" was writen to file: ")); __mm_s.push_str(&*fileName); ArcStr::from(__mm_s) })?;
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
            Debug::traceln(literal!(
                "Tyring to start GraphViz *lefty* to visualize the graph. You need to have lefty in your PATH variable"
            ))?;
            Debug::traceln(literal!(
                "Make sure you quit GraphViz *lefty* via Right Click->quit to be sure the process will be exited."
            ))?;
            Debug::traceln(literal!(
                "If you quit the GraphViz *lefty* window via X, please kill the process in task manager to continue."
            ))?;
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
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Running command: "));
                __mm_s.push_str(&*literal!("lefty -e "));
                __mm_s.push_str(&*leftyCMD);
                __mm_s.push_str(&*literal!(" > "));
                __mm_s.push_str(&*fileNameTraceRemovedConnections);
                ArcStr::from(__mm_s)
            })?;
            leftyExitStatus = System::systemCall(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("lefty -e "));
                    __mm_s.push_str(&*leftyCMD);
                    ArcStr::from(__mm_s)
                },
                fileNameTraceRemovedConnections.clone(),
            );
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("GraphViz *lefty* exited with status:"));
                __mm_s.push_str(&*intString(leftyExitStatus));
                ArcStr::from(__mm_s)
            })?;
            brokenConnects = System::readFile(fileNameTraceRemovedConnections.clone())?;
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "GraphViz OpenModelica assistant returned the following broken connects: "
                ));
                __mm_s.push_str(&*brokenConnects);
                ArcStr::from(__mm_s)
            })?;
            Ok(brokenConnects.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(brokenConnectsViaGraphViz)
}

pub(crate) fn removeBrokenConnects(
    mut inConnects: metamodelica::List<DAE::Connect::ConnectorElement>,
    mut inConnected: &DaeEdges,
    mut inBroken: &DaeEdges,
) -> Result<metamodelica::List<metamodelica::List<DAE::Connect::ConnectorElement>>> {
    let mut outConnects: metamodelica::List<metamodelica::List<DAE::Connect::ConnectorElement>>;
    outConnects = (::match_deref::match_deref! { match inBroken {
        Deref @ metamodelica::ListNode::Nil => {
            list![inConnects]
        },
        _ => {
            let mut toRemove: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut toKeep: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut intersect: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut cset: metamodelica::List<DAE::Connect::ConnectorElement>;
            let mut csets: metamodelica::List<metamodelica::List<DAE::Connect::ConnectorElement>>;
            toRemove = filterFromSet(&inConnects, inBroken, &(metamodelica::nil()), &(literal!("removed")))?;
            if (toRemove).is_empty() {
                csets = list![inConnects];
            } else {
                toKeep = filterFromSet(&inConnects, inConnected, &(metamodelica::nil()), &(literal!("allowed")))?;
                intersect = List::intersectionOnTrue(&toRemove, &toKeep, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1))?;
                if Flags::isSet(Flags::CGRAPH.clone())? {
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: CS: ")); __mm_s.push_str(&*stringDelimitList(List::map(inConnects.clone(), &move |__a0: DAE::Connect::ConnectorElement| ConnectUtil::printElementStr(&__a0))?, literal!("\n"))); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: keep: ")); __mm_s.push_str(&*stringDelimitList(List::map(toKeep, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!(", "))); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: delete: ")); __mm_s.push_str(&*stringDelimitList(List::map(toRemove.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!(", "))); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: allow = remove - keep: ")); __mm_s.push_str(&*stringDelimitList(List::map(intersect.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!(", "))); ArcStr::from(__mm_s) })?;
                }
                toRemove = List::setDifference(toRemove, &intersect)?;
                if Flags::isSet(Flags::CGRAPH.clone())? {
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: allow - delete: ")); __mm_s.push_str(&*stringDelimitList(List::map(toRemove.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!(", "))); ArcStr::from(__mm_s) })?;
                }
                cset = removeFromConnects(inConnects, toRemove)?;
                csets = splitSetByAllowed(&cset, inConnected)?;
            }
            csets
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outConnects)
}

fn splitSetByAllowed(
    mut inConnects: &metamodelica::List<DAE::Connect::ConnectorElement>,
    mut inConnected: &DaeEdges,
) -> Result<metamodelica::List<metamodelica::List<DAE::Connect::ConnectorElement>>> {
    let mut outConnects: metamodelica::List<metamodelica::List<DAE::Connect::ConnectorElement>>;
    let mut cset: metamodelica::List<DAE::Connect::ConnectorElement>;
    let mut csets: metamodelica::List<metamodelica::List<DAE::Connect::ConnectorElement>>;
    let mut e: DaeEdge = (
        metamodelica::Ref::new(DAE::ComponentRef::WILD),
        metamodelica::Ref::new(DAE::ComponentRef::WILD),
        metamodelica::nil(),
    );
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
    let mut ce: DAE::Connect::ConnectorElement = <DAE::Connect::ConnectorElement as ::std::default::Default>::default();
    csets = metamodelica::nil();
    for mut e in &**inConnected {
        let mut e = e.clone();
        cset = metamodelica::nil();
        (cr1, cr2, _) = e;
        for mut ce in &**inConnects {
            let mut ce = ce.clone();
            if ComponentReferenceBasics::crefPrefixOf(&cr1, &ce.name)? {
                cset = metamodelica::cons(ce.clone(), cset);
            }
            if ComponentReferenceBasics::crefPrefixOf(&cr2, &ce.name)? {
                cset = metamodelica::cons(ce, cset);
            }
        }
        if !((cset).is_empty()) {
            csets = metamodelica::cons(cset, csets);
        }
    }
    outConnects = csets;
    Ok(outConnects)
}

fn filterFromSet(
    mut inConnects: &metamodelica::List<DAE::Connect::ConnectorElement>,
    mut inFilter: &DaeEdges,
    mut inAcc: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut msg: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut filteredCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    filteredCrefs = 'mc: {
        let __mc_input = &**inFilter;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(List::unique(inAcc))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (c1, c2, _), tail: rest } => {
                    let mut filtered: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let true = (ConnectUtil::isReferenceInConnects(inConnects, metamodelica::AsArg::as_arg(&c1))?) else { return Err("pattern mismatch") };
                    let true = (ConnectUtil::isReferenceInConnects(inConnects, metamodelica::AsArg::as_arg(&c2))?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.filterFromSet: ")); __mm_s.push_str(&*msg); __mm_s.push_str(&*literal!(" connect(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
                    }
                    filtered = filterFromSet(inConnects, metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(c1.clone(), metamodelica::cons(c2.clone(), inAcc.clone()))), msg)?;
                    Ok(filtered.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut filtered: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    filtered = filterFromSet(inConnects, metamodelica::AsArg::as_arg(&rest), inAcc, msg)?;
                    Ok(filtered.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(filteredCrefs)
}

fn removeFromConnects(
    mut inConnects: metamodelica::List<DAE::Connect::ConnectorElement>,
    mut inToRemove: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<DAE::Connect::ConnectorElement>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inConnects.clone(), inToRemove)) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(inConnects)
            },
            (cset, Deref @ metamodelica::ListNode::Cons { head: c, tail: rest }) => {
                let mut cset = (*cset).clone();
                let __pa0 = ::match_deref::match_deref! { match &(ConnectUtil::removeReferenceFromConnects(cset.clone(), c.clone())?) {
                    (__pa0, true) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                cset = metamodelica::Own::own(__pa0);
                { (inConnects, inToRemove) = (cset.clone(), rest.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn addBrokenEqualityConstraintEquations(
    mut inDAE: DAE::DAElist,
    mut inBroken: DaeEdges,
) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist;
    outDAE = (::match_deref::match_deref! { match &(inBroken.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            inDAE
        },
        _ => {
            let mut equalityConstraintElements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut dae: DAE::DAElist;
            equalityConstraintElements = List::flatten(List::map(inBroken, &fnptr!(Util::tuple33, _))?)?;
            dae = DAEUtil::joinDaes(&(DAE::DAElist { elementLst: equalityConstraintElements }), &inDAE)?;
            dae
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}
