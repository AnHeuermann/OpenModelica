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

use crate::Error;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn buildGraph<
    NodeType: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inNodes: metamodelica::List<NodeType>,
    mut inEdgeFunc: &dyn ::std::ops::Fn(NodeType, ArgType) -> Result<metamodelica::List<NodeType>>,
    mut inEdgeArg: ArgType,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type EdgeFunc<NodeType: Clone + 'static, ArgType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, ArgType) -> Result<metamodelica::List<NodeType>> + 'static>;

    let mut outGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    outGraph = List::zip(inNodes.clone(), List::map1(inNodes, inEdgeFunc, inEdgeArg)?);
    Ok(outGraph)
}

pub fn emptyGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNodes: metamodelica::List<NodeType>,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    let mut outGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    outGraph = List::map(inNodes, &fnptr!(emptyGraphHelper, _))?;
    Ok(outGraph)
}

fn emptyGraphHelper<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut nt: NodeType,
) -> (NodeType, metamodelica::List<NodeType>) {
    let mut out: (NodeType, metamodelica::List<NodeType>);
    out = (nt, metamodelica::nil());
    out
}

pub fn topologicalSort<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>,
) -> Result<(
    metamodelica::List<NodeType>,
    metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
)> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outNodes: metamodelica::List<NodeType>;
    let mut outRemainingGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    let mut start_nodes: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    let mut rest_nodes: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    (rest_nodes, start_nodes) = List::splitOnTrue(inGraph, &move |__a0: _| -> metamodelica::Result<_> {
        ::std::result::Result::Ok(hasOutgoingEdges(&__a0))
    })?;
    (outNodes, outRemainingGraph) =
        topologicalSort2(start_nodes, rest_nodes, metamodelica::nil(), inEqualFunc.clone())?;
    Ok((outNodes, outRemainingGraph))
}

fn topologicalSort2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStartNodes: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inRestNodes: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inAccumNodes: metamodelica::List<NodeType>,
    mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>,
) -> Result<(
    metamodelica::List<NodeType>,
    metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
)> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    '__tco: loop {
        ::match_deref::match_deref! { match &((inStartNodes, inRestNodes.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((inAccumNodes.reverse(), inRestNodes))
            },
            (rest_start, Deref @ metamodelica::ListNode::Nil) => {
                let mut node1: NodeType;
                let mut result: metamodelica::List<NodeType>;
                result = inAccumNodes;
                for mut n in &*rest_start.clone() {
                    let __pa0 = ::match_deref::match_deref! { match &(n.clone()) {
                        (__pa0, Deref @ metamodelica::ListNode::Nil) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    node1 = metamodelica::Own::own(__pa0);
                    result = metamodelica::cons(node1, result);
                }
                result = result.reverse();
                return Ok((result, metamodelica::nil()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: (node1, Deref @ metamodelica::ListNode::Nil), tail: rest_start }, rest_rest) => {
                let mut rest_start_: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
                let mut new_start: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
                let mut result: metamodelica::List<NodeType>;
                let mut rest_rest = (*rest_rest).clone();
                rest_rest = List::map2(rest_rest.clone(), &move |__a0: _, __a1: _, __a2: _| removeEdge(&__a0, __a1, metamodelica::arc_ref(&__a2)), node1.clone(), inEqualFunc.clone())?;
                (rest_rest, new_start) = List::splitOnTrue(metamodelica::AsArg::as_arg(&rest_rest), &move |__a0: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(hasOutgoingEdges(&__a0)) })?;
                rest_start_ = listAppend(rest_start.clone(), new_start);
                { (inStartNodes, inRestNodes, inAccumNodes, inEqualFunc) = (rest_start_, rest_rest.clone(), metamodelica::cons(node1.clone(), inAccumNodes), inEqualFunc.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn hasOutgoingEdges<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: &(NodeType, metamodelica::List<NodeType>),
) -> bool {
    let mut outHasOutEdges: bool;
    outHasOutEdges = (::match_deref::match_deref! { match &(inNode) {
        (_, Deref @ metamodelica::ListNode::Nil) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outHasOutEdges
}

fn removeEdge<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: &(NodeType, metamodelica::List<NodeType>),
    mut inRemovedNode: NodeType,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<(NodeType, metamodelica::List<NodeType>)> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outNode: (NodeType, metamodelica::List<NodeType>);
    let mut node: NodeType;
    let mut edges: metamodelica::List<NodeType>;
    (node, edges) = inNode.clone();
    (edges, _) = List::deleteMemberOnTrue(inRemovedNode, edges, inEqualFunc)?;
    outNode = (node, edges);
    Ok(outNode)
}

pub fn findCycles<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<metamodelica::List<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycles: metamodelica::List<metamodelica::List<NodeType>>;
    outCycles = findCycles2(inGraph, inGraph, inEqualFunc)?;
    Ok(outCycles)
}

pub(crate) fn findCycles2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNodes: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<metamodelica::List<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycles: metamodelica::List<metamodelica::List<NodeType>>;
    outCycles = 'mc: {
        let __mc_input = &**inNodes;
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
                Deref @ metamodelica::ListNode::Cons { head: node, tail: rest_nodes } => {
                    let mut cycle: metamodelica::List<NodeType>;
                    let mut rest_cycles: metamodelica::List<metamodelica::List<NodeType>>;
                    let mut rest_nodes = (*rest_nodes).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(findCycleForNode(&(node.clone()), inGraph, metamodelica::nil(), inEqualFunc)?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cycle = metamodelica::Own::own(__pa0);
                    rest_nodes = removeNodesFromGraph(&cycle, metamodelica::AsArg::as_arg(&rest_nodes), inEqualFunc)?;
                    rest_cycles = findCycles2(metamodelica::AsArg::as_arg(&rest_nodes), inGraph, inEqualFunc)?;
                    Ok(metamodelica::cons(cycle.clone(), rest_cycles.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_nodes } => {
                    let mut rest_cycles: metamodelica::List<metamodelica::List<NodeType>>;
                    rest_cycles = findCycles2(metamodelica::AsArg::as_arg(&rest_nodes), inGraph, inEqualFunc)?;
                    Ok(rest_cycles.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCycles)
}

fn findCycleForNode<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: &(NodeType, metamodelica::List<NodeType>),
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inVisitedNodes: metamodelica::List<NodeType>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<Option<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycle: Option<metamodelica::List<NodeType>>;
    outCycle = 'mc: {
        let __mc_input = (inNode, &*inVisitedNodes);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, _), Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut start_node: NodeType;
                    let mut is_start_node: bool;
                    let mut opt_cycle: Option<metamodelica::List<NodeType>>;
                    let true = (List::isMemberOnTrue(node.clone(), &inVisitedNodes, inEqualFunc)?) else { return Err("pattern mismatch") };
                    start_node = List::last(&inVisitedNodes)?;
                    is_start_node = inEqualFunc(node.clone(), start_node.clone())?;
                    opt_cycle = if (is_start_node) {Some(inVisitedNodes.clone())} else {None};
                    Ok(opt_cycle.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, edges), _) => {
                    let mut visited_nodes: metamodelica::List<NodeType>;
                    let mut cycle: metamodelica::List<NodeType>;
                    visited_nodes = metamodelica::cons(node.clone(), inVisitedNodes.clone());
                    cycle = findCycleForNode2(metamodelica::AsArg::as_arg(&edges), inGraph, &visited_nodes, inEqualFunc)?;
                    Ok(Some(cycle.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCycle)
}

fn findCycleForNode2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNodes: &metamodelica::List<NodeType>,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inVisitedNodes: &metamodelica::List<NodeType>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<metamodelica::List<NodeType>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycle: metamodelica::List<NodeType>;
    outCycle = 'mc: {
        let __mc_input = &**inNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: node, tail: _ } => {
                    let mut cycle: metamodelica::List<NodeType>;
                    let mut graph_node: (NodeType, metamodelica::List<NodeType>);
                    graph_node = findNodeInGraph(node.clone(), inGraph, inEqualFunc)?;
                    let __pa0 = ::match_deref::match_deref! { match &(findCycleForNode(&graph_node, inGraph, inVisitedNodes.clone(), inEqualFunc)?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cycle = metamodelica::Own::own(__pa0);
                    Ok(cycle.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_nodes } => {
                    let mut cycle: metamodelica::List<NodeType>;
                    cycle = findCycleForNode2(metamodelica::AsArg::as_arg(&rest_nodes), inGraph, inVisitedNodes, inEqualFunc)?;
                    Ok(cycle.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCycle)
}

fn findNodeInGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: NodeType,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<(NodeType, metamodelica::List<NodeType>)> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outNode: (NodeType, metamodelica::List<NodeType>);
    outNode = 'mc: {
        let __mc_input = &**inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: graph_node @ (node, _), tail: _ } => {
                    let true = (inEqualFunc(inNode.clone(), node.clone())?) else { return Err("pattern mismatch") };
                    Ok(graph_node.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_graph } => {
                    Ok(findNodeInGraph(inNode.clone(), metamodelica::AsArg::as_arg(&rest_graph), inEqualFunc)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outNode)
}

fn findIndexofNodeInGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: NodeType,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
    mut inIndex: i32,
) -> Result<i32> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outIndex: i32;
    outIndex = 'mc: {
        let __mc_input = &**inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (node, _), tail: _ } => {
                    let true = (inEqualFunc(inNode.clone(), node.clone())?) else { return Err("pattern mismatch") };
                    Ok(inIndex)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_graph } => {
                    Ok(findIndexofNodeInGraph(inNode.clone(), metamodelica::AsArg::as_arg(&rest_graph), inEqualFunc, inIndex + 1)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outIndex)
}

fn removeNodesFromGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNodes: &metamodelica::List<NodeType>,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    outGraph = 'mc: {
        let __mc_input = (&**inNodes, &**inGraph);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inGraph.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: (node, _), tail: rest_graph }) => {
                    let mut rest_nodes: metamodelica::List<NodeType>;
                    let __pa0 = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(node.clone(), inNodes.clone(), inEqualFunc)?) {
                        (__pa0, Some(_)) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rest_nodes = metamodelica::Own::own(__pa0);
                    Ok(removeNodesFromGraph(&rest_nodes, metamodelica::AsArg::as_arg(&rest_graph), inEqualFunc)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: graph_node, tail: rest_graph }) => {
                    let mut rest_graph = (*rest_graph).clone();
                    rest_graph = removeNodesFromGraph(inNodes, metamodelica::AsArg::as_arg(&rest_graph), inEqualFunc)?;
                    Ok(metamodelica::cons(graph_node.clone(), rest_graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

pub fn transposeGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut intmpGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    outGraph = 'mc: {
        let __mc_input = &**inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(intmpGraph.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (node, nodeList), tail: restGraph } => {
                    let mut tmpGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
                    tmpGraph = List::fold2(metamodelica::AsArg::as_arg(&nodeList), &move |__a0: _, __a1: _, __a2: _, __a3: _| insertNodetoGraph(__a0, __a1, metamodelica::arc_ref(&__a2), &__a3), node.clone(), inEqualFunc.clone(), intmpGraph.clone())?;
                    tmpGraph = transposeGraph(&tmpGraph, metamodelica::AsArg::as_arg(&restGraph), inEqualFunc.clone())?;
                    Ok(tmpGraph.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Graph.transpose failed.")], &(metamodelica::sourceInfo!("Util/Graph.mo")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

fn insertNodetoGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inNode: NodeType,
    mut inVertex: NodeType,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    outGraph = 'mc: {
        let __mc_input = &**inGraph;
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
                Deref @ metamodelica::ListNode::Cons { head: (node, rest), tail: restGraph } => {
                    let mut rest = (*rest).clone();
                    let mut restGraph = (*restGraph).clone();
                    let true = (inEqualFunc(node.clone(), inNode.clone())?) else { return Err("pattern mismatch") };
                    rest = List::unionList(&(list![rest.clone(), list![inVertex.clone()]]))?;
                    restGraph = insertNodetoGraph(inNode.clone(), inVertex.clone(), inEqualFunc, metamodelica::AsArg::as_arg(&restGraph))?;
                    Ok(metamodelica::cons((node.clone(), rest.clone()), restGraph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (node, rest), tail: restGraph } => {
                    let mut restGraph = (*restGraph).clone();
                    let false = (inEqualFunc(node.clone(), inNode.clone())?) else { return Err("pattern mismatch") };
                    restGraph = insertNodetoGraph(inNode.clone(), inVertex.clone(), inEqualFunc, metamodelica::AsArg::as_arg(&restGraph))?;
                    Ok(metamodelica::cons((node.clone(), rest.clone()), restGraph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

pub fn allReachableNodes<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut intmpstorage: &(metamodelica::List<NodeType>, metamodelica::List<NodeType>),
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<metamodelica::List<NodeType>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut reachableNodes: metamodelica::List<NodeType>;
    let __pa0 = ::match_deref::match_deref! { match &(allReachableNodesWork(intmpstorage, inGraph, inEqualFunc)?) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    reachableNodes = metamodelica::Own::own(__pa0);
    Ok(reachableNodes)
}

fn allReachableNodesWork<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut intmpstorage: &(metamodelica::List<NodeType>, metamodelica::List<NodeType>),
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<Option<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut reachableNodes: Option<metamodelica::List<NodeType>>;
    reachableNodes = 'mc: {
        let __mc_input = intmpstorage;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, L) => {
                    let mut L = (*L).clone();
                    L = L.clone().reverse();
                    Ok(Some(L.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: M }, L) => {
                    List::getMemberOnTrue(node.clone(), metamodelica::AsArg::as_arg(&L), inEqualFunc)?;
                    Ok(allReachableNodesWork(&((M.clone(), L.clone())), inGraph, inEqualFunc)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: M }, L) => {
                    let mut edges: metamodelica::List<NodeType>;
                    let mut M = (*M).clone();
                    let mut L = (*L).clone();
                    L = metamodelica::cons(node.clone(), L.clone());
                    (_, edges) = findNodeInGraph(node.clone(), inGraph, inEqualFunc)?;
                    M = listAppend(edges.clone(), M.clone());
                    Ok(allReachableNodesWork(&((M.clone(), L.clone())), inGraph, inEqualFunc)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Graph.allReachableNodes failed.")], &(metamodelica::sourceInfo!("Util/Graph.mo")))?;
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(reachableNodes)
}

pub(crate) fn partialDistance2color<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut toColorNodes: &metamodelica::List<NodeType>,
    mut inforbiddenColor: metamodelica::Array<Option<metamodelica::List<NodeType>>>,
    mut inColors: &metamodelica::List<i32>,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inGraphT: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inColored: metamodelica::Array<i32>,
    mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>,
    mut inPrintFunc: &dyn ::std::ops::Fn(metamodelica::List<NodeType>, ArcStr) -> Result<()>,
) -> Result<metamodelica::Array<i32>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type PrintFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<NodeType>, ArcStr) -> Result<()> + 'static>;

    let mut outColored: metamodelica::Array<i32>;
    outColored = 'mc: {
        let __mc_input = &**toColorNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inColored.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
                    let mut nodes: metamodelica::List<NodeType>;
                    let mut forbiddenColor: metamodelica::Array<Option<metamodelica::List<NodeType>>>;
                    let mut colored: metamodelica::Array<i32>;
                    let mut color: i32;
                    let mut index: i32;
                    index = metamodelica::arrayLength(inColored.clone()) - ((rest).len() as i32);
                    (_, nodes) = findNodeInGraph(node.clone(), inGraphT, &*inEqualFunc)?;
                    forbiddenColor = addForbiddenColors(node.clone(), &nodes, inColored.clone(), inforbiddenColor.clone(), inGraph, inEqualFunc.clone(), inPrintFunc)?;
                    color = arrayFindMinColorIndex(forbiddenColor.clone(), node.clone(), 1, metamodelica::arrayLength(inColored.clone()) + 1, &*inEqualFunc, inPrintFunc)?;
                    colored = metamodelica::arrayUpdate(inColored.clone(), index, color)?;
                    colored = partialDistance2color(metamodelica::AsArg::as_arg(&rest), forbiddenColor.clone(), inColors, inGraph, inGraphT, colored.clone(), inEqualFunc.clone(), inPrintFunc)?;
                    Ok(colored.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Graph.partialDistance2color failed.")], &(metamodelica::sourceInfo!("Util/Graph.mo")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outColored)
}

fn addForbiddenColors<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: NodeType,
    mut inNodes: &metamodelica::List<NodeType>,
    mut inColored: metamodelica::Array<i32>,
    mut inForbiddenColor: metamodelica::Array<Option<metamodelica::List<NodeType>>>,
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>,
    mut inPrintFunc: &dyn ::std::ops::Fn(metamodelica::List<NodeType>, ArcStr) -> Result<()>,
) -> Result<metamodelica::Array<Option<metamodelica::List<NodeType>>>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type PrintFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<NodeType>, ArcStr) -> Result<()> + 'static>;

    let mut outForbiddenColor: metamodelica::Array<Option<metamodelica::List<NodeType>>>;
    outForbiddenColor = 'mc: {
        let __mc_input = (&**inNodes, inForbiddenColor.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inForbiddenColor.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: rest }, forbiddenColor) => {
                    let mut nodes: metamodelica::List<NodeType>;
                    let mut indexes: metamodelica::List<i32>;
                    let mut indexesColor: metamodelica::List<i32>;
                    let mut forbiddenColor1: metamodelica::Array<Option<metamodelica::List<NodeType>>>;
                    (_, nodes) = findNodeInGraph(node.clone(), inGraph, &*inEqualFunc)?;
                    indexes = List::map3(nodes.clone(), &move |__a0: _, __a1: _, __a2: _, __a3: i32| findIndexofNodeInGraph(__a0, &__a1, metamodelica::arc_ref(&__a2), __a3), inGraph.clone(), inEqualFunc.clone(), 1)?;
                    indexes = List::select1(indexes.clone(), (std::sync::Arc::new(arrayElemetGtZero) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), inColored.clone())?;
                    indexesColor = List::map1(indexes.clone(), &getArrayElem, inColored.clone())?;
                    List::map2_0(&indexesColor, &arrayUpdateListAppend, forbiddenColor.clone(), Some(list![inNode.clone()]))?;
                    forbiddenColor1 = addForbiddenColors(inNode.clone(), metamodelica::AsArg::as_arg(&rest), inColored.clone(), forbiddenColor.clone(), inGraph, inEqualFunc.clone(), inPrintFunc)?;
                    Ok(forbiddenColor1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Graph.addForbiddenColors failed.")], &(metamodelica::sourceInfo!("Util/Graph.mo")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outForbiddenColor)
}

fn getArrayElem<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIndex: i32,
    mut inArray: metamodelica::Array<Type_a>,
) -> Result<Type_a> {
    let mut outElem: Type_a;
    outElem = metamodelica::arrayGet(inArray.clone(), inIndex)?;
    Ok(outElem)
}

fn arrayUpdateListAppend<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIndex: i32,
    mut inArray: metamodelica::Array<Option<metamodelica::List<NodeType>>>,
    mut inNode: Option<metamodelica::List<NodeType>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inArray.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::arrayUpdate(inArray.clone(), inIndex, inNode.clone())?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addSourceMessage(
                &(Error::INTERNAL_ERROR.clone()),
                list![literal!("Graph.arrayUpdateListAppend failed.")],
                &(metamodelica::sourceInfo!("Util/Graph.mo")),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn arrayElemetGtZero(mut inIndex: i32, mut inArray: metamodelica::Array<i32>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = intGt(metamodelica::arrayGet(inArray.clone(), inIndex)?, 0);
    Ok(outBoolean)
}

fn arrayFindMinColorIndex<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inForbiddenColor: metamodelica::Array<Option<metamodelica::List<NodeType>>>,
    mut inNode: NodeType,
    mut inIndex: i32,
    mut inmaxIndex: i32,
    mut inEqualFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
    mut inPrintFunc: &dyn ::std::ops::Fn(metamodelica::List<NodeType>, ArcStr) -> Result<()>,
) -> Result<i32> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type PrintFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<NodeType>, ArcStr) -> Result<()> + 'static>;

    let mut outColor: i32;
    outColor = 'mc: {
        let __mc_input = inPrintFunc.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(metamodelica::arrayGet(inForbiddenColor.clone(), inIndex)?) {
                None => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(inIndex)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nodes: metamodelica::List<NodeType>;
            let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(inForbiddenColor.clone(), inIndex)?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            nodes = metamodelica::Own::own(__pa0);
            if '__try1: {
                unwrap_break_err!(List::getMemberOnTrue(inNode.clone(), &nodes, inEqualFunc), '__try1);
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            Ok(inIndex)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nodes: metamodelica::List<NodeType>;
            let mut index: i32;
            let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(inForbiddenColor.clone(), inIndex)?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            nodes = metamodelica::Own::own(__pa0);
            List::getMemberOnTrue(inNode.clone(), &nodes, inEqualFunc)?;
            index = arrayFindMinColorIndex(
                inForbiddenColor.clone(),
                inNode.clone(),
                inIndex + 1,
                inmaxIndex,
                inEqualFunc,
                inPrintFunc,
            )?;
            Ok(index)
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outColor)
}

pub fn printGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inPrintFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>,
) -> Result<ArcStr> {
    pub type NodeToString<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = stringDelimitList(
        List::map1(
            inGraph,
            &move |__a0: _, __a1: _| printNode(&__a0, metamodelica::arc_ref(&__a1)),
            inPrintFunc.clone(),
        )?,
        literal!("\n"),
    );
    Ok(outString)
}

pub(crate) fn printNode<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: &(NodeType, metamodelica::List<NodeType>),
    mut inPrintFunc: &dyn ::std::ops::Fn(NodeType) -> Result<ArcStr>,
) -> Result<ArcStr> {
    pub type NodeToString<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    let mut node: NodeType;
    let mut edges: metamodelica::List<NodeType>;
    let mut node_str: ArcStr;
    let mut edges_str: ArcStr;
    (node, edges) = inNode.clone();
    node_str = inPrintFunc(node)?;
    edges_str = stringDelimitList(List::map(edges, inPrintFunc)?, literal!(", "));
    outString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*node_str);
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*edges_str);
        ArcStr::from(__mm_s)
    };
    Ok(outString)
}

/* Functions for Integer graphs */
pub fn printGraphInt(mut inGraph: &metamodelica::List<(i32, metamodelica::List<i32>)>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inGraph {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (node, edges), tail: restGraph } => {
            let mut strEdges: metamodelica::List<ArcStr>;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node : ")); __mm_s.push_str(&*intString(node.clone())); __mm_s.push_str(&*literal!(" Edges: ")); ArcStr::from(__mm_s) });
            strEdges = List::map(edges.clone(), &fnptr!(intString, i32))?;
            strEdges = List::map1(strEdges, &fnptr!(stringAppend, ArcStr, ArcStr), literal!(" "))?;
            List::map_0(&strEdges, &fnptr!(print, ArcStr))?;
            metamodelica::print(literal!("\n"));
            printGraphInt(restGraph)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn printNodesInt(mut inListNodes: metamodelica::List<i32>, mut inName: &ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inListNodes.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            ()
        },
        _ => {
            let mut strNodes: metamodelica::List<ArcStr>;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!(" : ")); ArcStr::from(__mm_s) });
            strNodes = List::map(inListNodes, &fnptr!(intString, i32))?;
            strNodes = List::map1(strNodes, &fnptr!(stringAppend, ArcStr, ArcStr), literal!(" "))?;
            List::map_0(&strNodes, &fnptr!(print, ArcStr))?;
            metamodelica::print(literal!("\n"));
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn allReachableNodesInt(
    mut intmpstorage: &(metamodelica::List<i32>, metamodelica::List<i32>),
    mut inGraph: metamodelica::Array<(i32, metamodelica::List<i32>)>,
    mut inMaxGraphNode: i32,
    mut inMaxNodexIndex: i32,
) -> Result<metamodelica::List<i32>> {
    let mut reachableNodes: metamodelica::List<i32> = metamodelica::nil();
    reachableNodes = 'mc: {
        let __mc_input = intmpstorage;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, L) => {
                    Ok(L.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: M }, L) => {
                    let mut edges: metamodelica::List<i32>;
                    let mut M = (*M).clone();
                    let mut L = (*L).clone();
                    let mut reachableNodes: metamodelica::List<i32> = reachableNodes.clone();
                    L = List::union(metamodelica::AsArg::as_arg(&L), &(list![node.clone()]));
                    let false = (intGe(node.clone(), inMaxGraphNode)) else { return Err("pattern mismatch") };
                    (_, edges) = metamodelica::arrayGet(inGraph.clone(), node.clone())?;
                    edges = List::filter1OnTrue(edges.clone(), std::sync::Arc::new(fnptr!(List::notMember, _, _)), L.clone())?;
                    M = List::union(metamodelica::AsArg::as_arg(&M), &edges);
                    reachableNodes = allReachableNodesInt(&((M.clone(), L.clone())), inGraph.clone(), inMaxGraphNode, inMaxNodexIndex)?;
                    Ok((reachableNodes.clone(), reachableNodes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            reachableNodes = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: node, tail: M }, L) => {
                    let mut L = (*L).clone();
                    let mut reachableNodes: metamodelica::List<i32> = reachableNodes.clone();
                    L = List::union(metamodelica::AsArg::as_arg(&L), &(list![node.clone()]));
                    let true = (intGe(node.clone(), inMaxGraphNode)) else { return Err("pattern mismatch") };
                    reachableNodes = allReachableNodesInt(&((M.clone(), L.clone())), inGraph.clone(), inMaxGraphNode, inMaxNodexIndex)?;
                    Ok((reachableNodes.clone(), reachableNodes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            reachableNodes = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Graph.allReachableNodesInt failed.")], &(metamodelica::sourceInfo!("Util/Graph.mo")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(reachableNodes)
}

pub fn partialDistance2colorInt(
    mut inGraphT: &metamodelica::List<(i32, metamodelica::List<i32>)>,
    mut numNodes: i32,
    mut inColored: metamodelica::Array<i32>,
) -> Result<()> {
    let wordBits: i32 = 30;
    let fullWord: i32 = 1073741823;
    let mut node: i32;
    let mut color: i32;
    let mut words: i32 = 1;
    let mut usedWords: i32 = 1;
    let mut k: i32;
    let mut w: i32;
    let mut bit: i32;
    let mut nodes: metamodelica::List<i32>;
    let mut nodeColors: metamodelica::Array<metamodelica::Array<i32>> = arrayCreate(numNodes, arrayCreate(0, 0));
    let mut forbidden: metamodelica::Array<i32> = arrayCreate(1, 0);
    let mut colors: metamodelica::Array<i32>;
    for mut i in 1..=numNodes {
        metamodelica::arrayUpdate(nodeColors.clone(), i, arrayCreate(words, 0))?;
    }
    for mut tpl in &**inGraphT {
        (node, nodes) = tpl.clone();
        for mut j in 1..=usedWords {
            metamodelica::arrayUpdate(forbidden.clone(), j, 0)?;
        }
        for mut n in &*nodes {
            colors = metamodelica::arrayGet(nodeColors.clone(), n.clone())?;
            for mut j in 1..=usedWords {
                metamodelica::arrayUpdate(
                    forbidden.clone(),
                    j,
                    intBitOr(
                        metamodelica::arrayGet(forbidden.clone(), j)?,
                        metamodelica::arrayGet(colors.clone(), j)?,
                    ),
                )?;
            }
        }
        color = 0;
        k = 1;
        while color == 0 {
            if k > usedWords {
                color = (k - 1) * wordBits + 1;
            } else {
                w = metamodelica::arrayGet(forbidden.clone(), k)?;
                if w != fullWord {
                    bit = 0;
                    while intBitAnd(w, intBitLShift(1, bit)) != 0 {
                        bit = bit + 1;
                    }
                    color = (k - 1) * wordBits + bit + 1;
                }
                k = k + 1;
            }
        }
        metamodelica::arrayUpdate(inColored.clone(), node, color)?;
        k = intDiv(color - 1, wordBits) + 1;
        if k > words {
            words = 2 * words;
            forbidden = Array::expandToSize(words, forbidden.clone(), 0)?;
            for mut i in 1..=numNodes {
                metamodelica::arrayUpdate(
                    nodeColors.clone(),
                    i,
                    Array::expandToSize(words, metamodelica::arrayGet(nodeColors.clone(), i)?, 0)?,
                )?;
            }
        }
        usedWords = intMax(usedWords, k);
        bit = intBitLShift(1, intMod(color - 1, wordBits));
        for mut n in &*nodes {
            colors = metamodelica::arrayGet(nodeColors.clone(), n.clone())?;
            metamodelica::arrayUpdate(
                colors.clone(),
                k,
                intBitOr(metamodelica::arrayGet(colors.clone(), k)?, bit),
            )?;
        }
    }
    Ok(())
}

pub fn filterGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inGraph: &metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut inCondFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type CondFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>;

    let mut outGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    outGraph = List::accumulateMapAccum(
        inGraph,
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = inCondFunc.clone();
            move |__pe_a0, __pe_a2| filterGraph2(&__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
    )?;
    Ok(outGraph)
}

fn filterGraph2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: &(NodeType, metamodelica::List<NodeType>),
    mut inCondFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>,
    mut inAccumGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type CondFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>;

    let mut outNode: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    outNode = 'mc: {
        let __mc_input = inNode;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (node, _) => {
                    let false = (inCondFunc(node.clone())?) else { return Err("pattern mismatch") };
                    Ok(inAccumGraph.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (node, edges) => {
                    let mut edges = (*edges).clone();
                    edges = List::filterOnTrue(edges.clone(), inCondFunc.clone())?;
                    Ok(metamodelica::cons((node.clone(), edges.clone()), inAccumGraph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outNode)
}

pub fn merge<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut graph1: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut graph2: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut eqFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
    mut compareFunc: Arc<
        dyn ::std::ops::Fn(
                (NodeType, metamodelica::List<NodeType>),
                (NodeType, metamodelica::List<NodeType>),
            ) -> Result<bool>
            + 'static,
    >,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type CompareFunc<NodeType: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                (NodeType, metamodelica::List<NodeType>),
                (NodeType, metamodelica::List<NodeType>),
            ) -> Result<bool>
            + 'static,
    >;

    let mut graph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    graph = merge2(
        List::sort(listAppend(graph1, graph2), compareFunc.clone())?,
        eqFunc,
        metamodelica::nil(),
    )?;
    Ok(graph)
}

fn merge2<'__b, NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inGraph: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut eqFunc: &'__b dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
    mut inAcc: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
) -> Result<metamodelica::List<(NodeType, metamodelica::List<NodeType>)>> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    '__tco: loop {
        ::match_deref::match_deref! { match &(inGraph) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inAcc.reverse())
            },
            Deref @ metamodelica::ListNode::Cons { head: node, tail: Deref @ metamodelica::ListNode::Nil } => {
                return Ok(metamodelica::cons(node.clone(), inAcc).reverse())
            },
            Deref @ metamodelica::ListNode::Cons { head: (n1, e1), tail: Deref @ metamodelica::ListNode::Cons { head: (n2, e2), tail: rest } } => {
                let mut node: (NodeType, metamodelica::List<NodeType>);
                let mut b: bool;
                let mut rest = (*rest).clone();
                b = eqFunc(n1.clone(), n2.clone())?;
                (node, rest) = merge3(b, n1.clone(), e1.clone(), n2.clone(), e2.clone(), rest.clone(), eqFunc)?;
                { (inGraph, eqFunc, inAcc) = (rest.clone(), eqFunc, metamodelica::cons(node, inAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn merge3<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut b: bool,
    mut n1: NodeType,
    mut e1: metamodelica::List<NodeType>,
    mut n2: NodeType,
    mut e2: metamodelica::List<NodeType>,
    mut rest: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
    mut eqFunc: &dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool>,
) -> Result<(
    (NodeType, metamodelica::List<NodeType>),
    metamodelica::List<(NodeType, metamodelica::List<NodeType>)>,
)> {
    pub type EqualFunc<NodeType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut elt: (NodeType, metamodelica::List<NodeType>);
    let mut outRest: metamodelica::List<(NodeType, metamodelica::List<NodeType>)>;
    (elt, outRest) = (match b {
        true => ((n1, List::unionOnTrue(&e1, &e2, eqFunc)?), rest),
        false => ((n1, e1), metamodelica::cons((n2, e2), rest)),
    });
    Ok((elt, outRest))
}
