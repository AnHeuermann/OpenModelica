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

use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_graphml::GraphML;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// dump GraphML stuff
//
// =============================================================================
pub(crate) fn dumpSystem(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inids: Option<metamodelica::Array<i32>>,
    mut filename: ArcStr,
    mut numberMode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inSystem.clone(), inids)) {
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::NO_MATCHING { .. }, .. }, None) => {
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32;
            let mut eqnsids: metamodelica::List<i32>;
            let mut neqns: i32;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            vars = BackendVariable::daeVars(&inSystem);
            eqns = BackendEquation::getEqnsFromEqSystem(&inSystem);
            funcs = BackendDAEUtil::getFunctions(inShared);
            (_, m, _) = BackendDAEUtil::getAdjacencyMatrix(inSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs), BackendDAEUtil::isInitializationDAE(inShared))?;
            mapIncRowEqn = Array::createIntRange(metamodelica::arrayLength(m.clone()));
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("G"), false, graphInfo)?;
            graphInfo = metamodelica::Own::own(__pa0);
            graph = metamodelica::Own::own(__pa1);
            let (_, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars, (std::sync::Arc::new(fnptr!(addVarGraph, metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32)))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32))) -> Result<(metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode, 1, (graphInfo, graph)))?;
            graphInfo = metamodelica::Own::own(__pa2);
            graph = metamodelica::Own::own(__pa3);
            neqns = BackendEquation::getNumberOfEquations(eqns.clone());
            eqnsids = List::intRange(neqns);
            (graphInfo, graph) = List::fold3(&eqnsids, &move |__a0: i32, __a1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, __a2: metamodelica::Array<i32>, __a3: bool, __a4: (GraphML::GraphInfo, i32)| addEqnGraph(__a0, __a1, __a2, __a3, &__a4), eqns, mapIncRowEqn.clone(), numberMode, (graphInfo, graph))?;
            (_, _, graphInfo) = List::fold(&eqnsids, &move |__a0: i32, __a1: (i32, metamodelica::Array<metamodelica::List<i32>>, GraphML::GraphInfo)| addEdgesGraph(__a0, &__a1), (1, m.clone(), graphInfo))?;
            GraphML::dumpGraph(graphInfo, filename)?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(_), matching: Deref @ BackendDAE::Matching::NO_MATCHING { .. }, .. }, None) => {
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32;
            let mut eqnsids: metamodelica::List<i32>;
            let mut neqns: i32;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            vars = BackendVariable::daeVars(&inSystem);
            eqns = BackendEquation::getEqnsFromEqSystem(&inSystem);
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("G"), false, graphInfo)?;
            graphInfo = metamodelica::Own::own(__pa0);
            graph = metamodelica::Own::own(__pa1);
            let (_, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars, (std::sync::Arc::new(fnptr!(addVarGraph, metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32)))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32))) -> Result<(metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode, 1, (graphInfo, graph)))?;
            graphInfo = metamodelica::Own::own(__pa2);
            graph = metamodelica::Own::own(__pa3);
            neqns = BackendEquation::getNumberOfEquations(eqns.clone());
            eqnsids = List::intRange(neqns);
            mapIncRowEqn = Array::createIntRange(metamodelica::arrayLength(m.clone()));
            (graphInfo, graph) = List::fold3(&eqnsids, &move |__a0: i32, __a1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, __a2: metamodelica::Array<i32>, __a3: bool, __a4: (GraphML::GraphInfo, i32)| addEqnGraph(__a0, __a1, __a2, __a3, &__a4), eqns, mapIncRowEqn.clone(), numberMode, (graphInfo, graph))?;
            (_, _, graphInfo) = List::fold(&eqnsids, &move |__a0: i32, __a1: (i32, metamodelica::Array<metamodelica::List<i32>>, GraphML::GraphInfo)| addEdgesGraph(__a0, &__a1), (1, m.clone(), graphInfo))?;
            GraphML::dumpGraph(graphInfo, filename)?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: vec1, ass2: vec2, comps: Deref @ metamodelica::ListNode::Nil }, .. }, None) => {
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32;
            let mut eqnsids: metamodelica::List<i32>;
            let mut neqns: i32;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut eqnsflag: metamodelica::Array<bool>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            vars = BackendVariable::daeVars(&inSystem);
            eqns = BackendEquation::getEqnsFromEqSystem(&inSystem);
            funcs = BackendDAEUtil::getFunctions(inShared);
            (_, m, _, _, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(inSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs), BackendDAEUtil::isInitializationDAE(inShared))?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("G"), false, graphInfo)?;
            graphInfo = metamodelica::Own::own(__pa0);
            graph = metamodelica::Own::own(__pa1);
            let (_, _, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars, (std::sync::Arc::new(fnptr!(addVarGraphMatch, metamodelica::Ref<BackendDAE::Var>, (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32)))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32))) -> Result<(metamodelica::Ref<BackendDAE::Var>, (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode, 1, vec1.clone(), (graphInfo, graph)))?;
            graphInfo = metamodelica::Own::own(__pa2);
            graph = metamodelica::Own::own(__pa3);
            neqns = BackendEquation::equationArraySize(eqns.clone())?;
            eqnsids = List::intRange(neqns);
            eqnsflag = arrayCreate(neqns, false);
            (graphInfo, graph) = List::fold3(&eqnsids, &addEqnGraphMatch, eqns, (vec2.clone(), mapIncRowEqn.clone(), eqnsflag.clone()), numberMode, (graphInfo, graph))?;
            (_, _, _, _, graphInfo) = List::fold(&eqnsids, &move |__a0: i32, __a1: (i32, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)| addDirectedEdgesGraph(__a0, &__a1), (1, m.clone(), vec2.clone(), mapIncRowEqn.clone(), graphInfo))?;
            GraphML::dumpGraph(graphInfo, filename)?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2: vec2, comps: Deref @ metamodelica::ListNode::Nil, .. }, .. }, Some(vec3)) => {
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32;
            let mut eqnsids: metamodelica::List<i32>;
            let mut neqns: i32;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            vars = BackendVariable::daeVars(&inSystem);
            eqns = BackendEquation::getEqnsFromEqSystem(&inSystem);
            funcs = BackendDAEUtil::getFunctions(inShared);
            (_, m, _, _, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(inSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs), BackendDAEUtil::isInitializationDAE(inShared))?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("G"), false, graphInfo)?;
            graphInfo = metamodelica::Own::own(__pa0);
            graph = metamodelica::Own::own(__pa1);
            let (_, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars, (std::sync::Arc::new(fnptr!(addVarGraph, metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32)))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32))) -> Result<(metamodelica::Ref<BackendDAE::Var>, (bool, i32, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode, 1, (graphInfo, graph)))?;
            graphInfo = metamodelica::Own::own(__pa2);
            graph = metamodelica::Own::own(__pa3);
            neqns = BackendEquation::equationArraySize(eqns.clone())?;
            eqnsids = List::intRange(neqns);
            (graphInfo, graph) = List::fold3(&eqnsids, &move |__a0: i32, __a1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, __a2: metamodelica::Array<i32>, __a3: bool, __a4: (GraphML::GraphInfo, i32)| addEqnGraph(__a0, __a1, __a2, __a3, &__a4), eqns, mapIncRowEqn.clone(), numberMode, (graphInfo, graph))?;
            (_, _, _, _, graphInfo) = List::fold(&eqnsids, &move |__a0: i32, __a1: (i32, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)| addDirectedNumEdgesGraph(__a0, &__a1), (1, m.clone(), vec2.clone(), vec3.clone(), graphInfo))?;
            GraphML::dumpGraph(graphInfo, filename)?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, .. }, None) => {
            let mut vars: BackendDAE::Variables;
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut mt: metamodelica::Array<metamodelica::List<i32>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32;
            let mut vec3: metamodelica::Array<i32>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            vars = BackendVariable::daeVars(&inSystem);
            funcs = BackendDAEUtil::getFunctions(inShared);
            (_, m, mt) = BackendDAEUtil::getAdjacencyMatrix(inSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs), BackendDAEUtil::isInitializationDAE(inShared))?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("G"), false, graphInfo)?;
            graphInfo = metamodelica::Own::own(__pa0);
            graph = metamodelica::Own::own(__pa1);
            vec3 = arrayCreate(metamodelica::arrayLength(mt.clone()), -1);
            (graphInfo, graph) = addCompsGraph(comps.clone(), &vars, vec3.clone(), 1, (graphInfo, graph))?;
            mapIncRowEqn = arrayCreate(metamodelica::arrayLength(mt.clone()), -1);
            graphInfo = addCompsEdgesGraph(metamodelica::AsArg::as_arg(&comps), m.clone(), vec3.clone(), 1, 1, mapIncRowEqn.clone(), 1, graphInfo)?;
            GraphML::dumpGraph(graphInfo, filename)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn addVarGraph(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (bool, i32, (GraphML::GraphInfo, i32)),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (bool, i32, (GraphML::GraphInfo, i32)),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (bool, i32, (GraphML::GraphInfo, i32));
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (true, id, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut desc: ArcStr;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    let true = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
                    labelText = intString(id.clone());
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    desc = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, arcstr::literal!(GraphML::COLOR_BLUE), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, Some(desc.clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (true, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (false, id, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    let true = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
                    labelText = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); ArcStr::from(__mm_s) };
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, arcstr::literal!(GraphML::COLOR_BLUE), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (false, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (true, id, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut b: bool;
                    let mut color: ArcStr;
                    let mut desc: ArcStr;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    b = BackendVariable::isVarDiscrete(metamodelica::AsArg::as_arg(&v));
                    color = if (b) {arcstr::literal!(GraphML::COLOR_PURPLE)} else {arcstr::literal!(GraphML::COLOR_RED)};
                    labelText = intString(id.clone());
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    desc = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, color.clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, Some(desc.clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (true, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (false, id, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut b: bool;
                    let mut color: ArcStr;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    b = BackendVariable::isVarDiscrete(metamodelica::AsArg::as_arg(&v));
                    color = if (b) {arcstr::literal!(GraphML::COLOR_PURPLE)} else {arcstr::literal!(GraphML::COLOR_RED)};
                    labelText = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); ArcStr::from(__mm_s) };
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, color.clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (false, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outTpl)
}

fn addVarGraphMatch(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32)),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32)),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32));
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (false, id, vec1, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut color: ArcStr;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    let true = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
                    color = if (intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), id.clone())?).clone(); __elt}), 0)) {arcstr::literal!(GraphML::COLOR_BLUE)} else {arcstr::literal!(GraphML::COLOR_YELLOW)};
                    labelText = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); ArcStr::from(__mm_s) };
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, color.clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (false, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (true, id, vec1, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut color: ArcStr;
                    let mut desc: ArcStr;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    let true = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
                    color = if (intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), id.clone())?).clone(); __elt}), 0)) {arcstr::literal!(GraphML::COLOR_BLUE)} else {arcstr::literal!(GraphML::COLOR_YELLOW)};
                    desc = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    labelText = intString(id.clone());
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, color.clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, Some(desc.clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (true, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (false, id, vec1, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut color: ArcStr;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    color = if (intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), id.clone())?).clone(); __elt}), 0)) {arcstr::literal!(GraphML::COLOR_RED)} else {arcstr::literal!(GraphML::COLOR_YELLOW)};
                    labelText = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); ArcStr::from(__mm_s) };
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, color.clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (false, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, .. }, (true, id, vec1, (graphInfo, graph))) => {
                    let mut label: GraphML::NodeLabel;
                    let mut color: ArcStr;
                    let mut desc: ArcStr;
                    let mut labelText: ArcStr;
                    let mut graphInfo = (*graphInfo).clone();
                    color = if (intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), id.clone())?).clone(); __elt}), 0)) {arcstr::literal!(GraphML::COLOR_RED)} else {arcstr::literal!(GraphML::COLOR_YELLOW)};
                    desc = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    labelText = intString(id.clone());
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText.clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }, color.clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, Some(desc.clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((v.clone(), (true, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outTpl)
}

fn addEqnGraph(
    mut inNode: i32,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut numberMode: bool,
    mut inGraph: &(GraphML::GraphInfo, i32),
) -> Result<(GraphML::GraphInfo, i32)> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut outGraph: (GraphML::GraphInfo, i32);
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut r#str: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut graph: i32;
    let mut label: GraphML::NodeLabel;
    let mut labelText: ArcStr;
    outGraph = (match (numberMode, inGraph.clone()) {
        (false, (mut __esc_graphInfo, mut __esc_graph)) => {
            graphInfo = __esc_graphInfo.clone();
            graph = __esc_graph.clone();
            eqn = BackendEquation::get(
                eqns,
                (*metamodelica::index_checked(&__ab_mapIncRowEqn, inNode)?).clone(),
            )?;
            r#str = BackendDump::equationString(&eqn)?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*intString(inNode));
                __mm_s.push_str(&*literal!(": "));
                __mm_s.push_str(&*BackendDump::equationString(&eqn)?);
                ArcStr::from(__mm_s)
            };
            r#str = Util::xmlEscape(r#str)?;
            label = GraphML::NodeLabel::NODELABEL_INTERNAL {
                text: r#str,
                backgroundColor: None,
                fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
            };
            (graphInfo, _) = GraphML::addNode(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("n"));
                    __mm_s.push_str(&*intString(inNode));
                    ArcStr::from(__mm_s)
                },
                arcstr::literal!(GraphML::COLOR_GREEN),
                GraphML::BORDERWIDTH_STANDARD.clone(),
                list![label],
                openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
                None,
                metamodelica::nil(),
                graph,
                graphInfo,
            )?;
            (graphInfo, graph)
        }
        (true, (mut __esc_graphInfo, mut __esc_graph)) => {
            graphInfo = __esc_graphInfo.clone();
            graph = __esc_graph.clone();
            eqn = BackendEquation::get(
                eqns,
                (*metamodelica::index_checked(&__ab_mapIncRowEqn, inNode)?).clone(),
            )?;
            r#str = BackendDump::equationString(&eqn)?;
            r#str = Util::xmlEscape(r#str)?;
            labelText = intString(inNode);
            label = GraphML::NodeLabel::NODELABEL_INTERNAL {
                text: labelText,
                backgroundColor: None,
                fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
            };
            (graphInfo, _) = GraphML::addNode(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("n"));
                    __mm_s.push_str(&*intString(inNode));
                    ArcStr::from(__mm_s)
                },
                arcstr::literal!(GraphML::COLOR_GREEN),
                GraphML::BORDERWIDTH_STANDARD.clone(),
                list![label],
                openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
                Some(r#str),
                metamodelica::nil(),
                graph,
                graphInfo,
            )?;
            (graphInfo, graph)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outGraph)
}

fn addEdgesGraph(
    mut e: i32,
    mut inTpl: &(i32, metamodelica::Array<metamodelica::List<i32>>, GraphML::GraphInfo),
) -> Result<(i32, metamodelica::Array<metamodelica::List<i32>>, GraphML::GraphInfo)> {
    let mut outTpl: (i32, metamodelica::Array<metamodelica::List<i32>>, GraphML::GraphInfo);
    let mut id: i32;
    let mut graph: GraphML::GraphInfo;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut vars: metamodelica::List<i32>;
    (id, m, graph) = inTpl.clone();
    vars = List::select(
        ({
            let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone();
            __elt
        }),
        (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
    )?;
    vars = ({
        let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone();
        __elt
    });
    (id, graph) = List::fold1(
        &vars,
        &move |__a0: i32, __a1: i32, __a2: (i32, GraphML::GraphInfo)| addEdgeGraph(__a0, __a1, &__a2),
        e,
        (id, graph),
    )?;
    outTpl = (id, m.clone(), graph);
    Ok(outTpl)
}

fn addEqnGraphMatch(
    mut inNode: i32,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut atpl: (
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::Array<bool>,
    ),
    mut numberMode: bool,
    mut inGraph: (GraphML::GraphInfo, i32),
) -> Result<(GraphML::GraphInfo, i32)> {
    let mut outGraph: (GraphML::GraphInfo, i32);
    outGraph = 'mc: {
        let __mc_input = (atpl, numberMode, inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let ((mut vec2, mut mapIncRowEqn, mut eqnsflag), false, (mut graphInfo, mut graph)) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut r#str: ArcStr;
            let mut color: ArcStr;
            let mut e: i32;
            let mut label: GraphML::NodeLabel;
            e = ({
                let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), inNode)?).clone();
                __elt
            });
            let false = ({
                let __elt = (*metamodelica::index_checked(&eqnsflag.borrow(), e)?).clone();
                __elt
            }) else {
                return Err("pattern mismatch");
            };
            eqn = BackendEquation::get(
                eqns.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), inNode)?).clone();
                    __elt
                }),
            )?;
            r#str = BackendDump::equationString(&eqn)?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*intString(e));
                __mm_s.push_str(&*literal!(": "));
                __mm_s.push_str(&*r#str);
                ArcStr::from(__mm_s)
            };
            r#str = Util::xmlEscape(r#str.clone())?;
            color = if (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&vec2.borrow(), inNode)?).clone();
                    __elt
                }),
                0,
            )) {
                arcstr::literal!(GraphML::COLOR_GREEN)
            } else {
                arcstr::literal!(GraphML::COLOR_PURPLE)
            };
            label = GraphML::NodeLabel::NODELABEL_INTERNAL {
                text: r#str.clone(),
                backgroundColor: None,
                fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
            };
            (graphInfo, _) = GraphML::addNode(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("n"));
                    __mm_s.push_str(&*intString(e));
                    ArcStr::from(__mm_s)
                },
                color.clone(),
                GraphML::BORDERWIDTH_STANDARD.clone(),
                list![label.clone()],
                openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
                None,
                metamodelica::nil(),
                graph.clone(),
                graphInfo.clone(),
            )?;
            Ok((graphInfo.clone(), graph.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let ((mut vec2, mut mapIncRowEqn, mut eqnsflag), true, (mut graphInfo, mut graph)) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut r#str: ArcStr;
            let mut color: ArcStr;
            let mut e: i32;
            let mut label: GraphML::NodeLabel;
            let mut labelText: ArcStr;
            e = ({
                let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), inNode)?).clone();
                __elt
            });
            let false = ({
                let __elt = (*metamodelica::index_checked(&eqnsflag.borrow(), e)?).clone();
                __elt
            }) else {
                return Err("pattern mismatch");
            };
            eqn = BackendEquation::get(
                eqns.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), inNode)?).clone();
                    __elt
                }),
            )?;
            r#str = BackendDump::equationString(&eqn)?;
            r#str = Util::xmlEscape(r#str.clone())?;
            color = if (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&vec2.borrow(), inNode)?).clone();
                    __elt
                }),
                0,
            )) {
                arcstr::literal!(GraphML::COLOR_GREEN)
            } else {
                arcstr::literal!(GraphML::COLOR_PURPLE)
            };
            labelText = intString(e);
            label = GraphML::NodeLabel::NODELABEL_INTERNAL {
                text: labelText.clone(),
                backgroundColor: None,
                fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
            };
            (graphInfo, _) = GraphML::addNode(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("n"));
                    __mm_s.push_str(&*intString(e));
                    ArcStr::from(__mm_s)
                },
                color.clone(),
                GraphML::BORDERWIDTH_STANDARD.clone(),
                list![label.clone()],
                openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
                Some(r#str.clone()),
                metamodelica::nil(),
                graph.clone(),
                graphInfo.clone(),
            )?;
            Ok((graphInfo.clone(), graph.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let ((_, mut mapIncRowEqn, mut eqnsflag), _, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut e: i32;
            e = ({
                let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), inNode)?).clone();
                __elt
            });
            let true = ({
                let __elt = (*metamodelica::index_checked(&eqnsflag.borrow(), e)?).clone();
                __elt
            }) else {
                return Err("pattern mismatch");
            };
            Ok(inGraph.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

fn addEdgeGraph(mut V: i32, mut e: i32, mut inTpl: &(i32, GraphML::GraphInfo)) -> Result<(i32, GraphML::GraphInfo)> {
    let mut outTpl: (i32, GraphML::GraphInfo);
    let mut id: i32;
    let mut v: i32;
    let mut graph: GraphML::GraphInfo;
    let mut ln: GraphML::LineType;
    (id, graph) = inTpl.clone();
    v = intAbs(V);
    ln = if (intGt(V, 0)) {
        openmodelica_codegen_graphml::GraphML::LineType::LINE
    } else {
        openmodelica_codegen_graphml::GraphML::LineType::DASHED
    };
    (graph, _) = GraphML::addEdge(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("e"));
            __mm_s.push_str(&*intString(id));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("n"));
            __mm_s.push_str(&*intString(e));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("v"));
            __mm_s.push_str(&*intString(v));
            ArcStr::from(__mm_s)
        },
        arcstr::literal!(GraphML::COLOR_BLACK),
        ln,
        GraphML::LINEWIDTH_STANDARD.clone(),
        false,
        metamodelica::nil(),
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
        ),
        metamodelica::nil(),
        graph,
    )?;
    outTpl = (id + 1, graph);
    Ok(outTpl)
}

fn addDirectedEdgesGraph(
    mut e: i32,
    mut inTpl: &(
        i32,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        GraphML::GraphInfo,
    ),
) -> Result<(
    i32,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    GraphML::GraphInfo,
)> {
    let mut outTpl: (
        i32,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        GraphML::GraphInfo,
    );
    let mut id: i32;
    let mut v: i32;
    let mut graph: GraphML::GraphInfo;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut vars: metamodelica::List<i32>;
    let mut vec2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    (id, m, vec2, mapIncRowEqn, graph) = inTpl.clone();
    vars = ({
        let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone();
        __elt
    });
    v = ({
        let __elt = (*metamodelica::index_checked(&vec2.borrow(), e)?).clone();
        __elt
    });
    (id, _, graph) = List::fold1(
        &vars,
        &move |__a0: i32, __a1: i32, __a2: (i32, i32, GraphML::GraphInfo)| addDirectedEdgeGraph(__a0, __a1, &__a2),
        ({
            let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), e)?).clone();
            __elt
        }),
        (id, v, graph),
    )?;
    outTpl = (id, m.clone(), vec2.clone(), mapIncRowEqn.clone(), graph);
    Ok(outTpl)
}

fn addDirectedEdgeGraph(
    mut v: i32,
    mut e: i32,
    mut inTpl: &(i32, i32, GraphML::GraphInfo),
) -> Result<(i32, i32, GraphML::GraphInfo)> {
    let mut outTpl: (i32, i32, GraphML::GraphInfo);
    let mut id: i32;
    let mut r: i32;
    let mut absv: i32;
    let mut graph: GraphML::GraphInfo;
    let mut arrow: (GraphML::ArrowType, GraphML::ArrowType);
    let mut lt: GraphML::LineType;
    (id, r, graph) = inTpl.clone();
    absv = intAbs(v);
    arrow = if (intEq(r, absv)) {
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
        )
    } else {
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
        )
    };
    lt = if (intGt(v, 0)) {
        openmodelica_codegen_graphml::GraphML::LineType::LINE
    } else {
        openmodelica_codegen_graphml::GraphML::LineType::DASHED
    };
    (graph, _) = GraphML::addEdge(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("e"));
            __mm_s.push_str(&*intString(id));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("n"));
            __mm_s.push_str(&*intString(e));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("v"));
            __mm_s.push_str(&*intString(absv));
            ArcStr::from(__mm_s)
        },
        arcstr::literal!(GraphML::COLOR_BLACK),
        lt,
        GraphML::LINEWIDTH_STANDARD.clone(),
        false,
        metamodelica::nil(),
        arrow,
        metamodelica::nil(),
        graph,
    )?;
    outTpl = (id + 1, r, graph);
    Ok(outTpl)
}

fn addDirectedNumEdgesGraph(
    mut e: i32,
    mut inTpl: &(
        i32,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        GraphML::GraphInfo,
    ),
) -> Result<(
    i32,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    GraphML::GraphInfo,
)> {
    let mut outTpl: (
        i32,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        GraphML::GraphInfo,
    );
    let mut id: i32;
    let mut v: i32;
    let mut graph: GraphML::GraphInfo;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut vars: metamodelica::List<i32>;
    let mut vec2: metamodelica::Array<i32>;
    let mut vec3: metamodelica::Array<i32>;
    let mut text: ArcStr;
    (id, m, vec2, vec3, graph) = inTpl.clone();
    vars = List::select(
        ({
            let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone();
            __elt
        }),
        (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
    )?;
    v = ({
        let __elt = (*metamodelica::index_checked(&vec2.borrow(), e)?).clone();
        __elt
    });
    text = intString(
        ({
            let __elt = (*metamodelica::index_checked(&vec3.borrow(), e)?).clone();
            __elt
        }),
    );
    (id, _, _, graph) = List::fold1(
        &vars,
        &move |__a0: i32, __a1: i32, __a2: (i32, i32, ArcStr, GraphML::GraphInfo)| {
            addDirectedNumEdgeGraph(__a0, __a1, &__a2)
        },
        e,
        (id, v, text, graph),
    )?;
    outTpl = (id, m.clone(), vec2.clone(), vec3.clone(), graph);
    Ok(outTpl)
}

fn addDirectedNumEdgeGraph(
    mut v: i32,
    mut e: i32,
    mut inTpl: &(i32, i32, ArcStr, GraphML::GraphInfo),
) -> Result<(i32, i32, ArcStr, GraphML::GraphInfo)> {
    let mut outTpl: (i32, i32, ArcStr, GraphML::GraphInfo);
    let mut id: i32;
    let mut r: i32;
    let mut graph: GraphML::GraphInfo;
    let mut arrow: (GraphML::ArrowType, GraphML::ArrowType);
    let mut text: ArcStr;
    let mut labels: metamodelica::List<GraphML::EdgeLabel>;
    (id, r, text, graph) = inTpl.clone();
    arrow = if (intEq(r, v)) {
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
        )
    } else {
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
        )
    };
    labels = if (intEq(r, v)) {
        list![GraphML::EdgeLabel {
            text: text.clone(),
            backgroundColor: Some(literal!("#0000FF")),
            fontSize: GraphML::FONTSIZE_STANDARD.clone()
        }]
    } else {
        metamodelica::nil()
    };
    (graph, _) = GraphML::addEdge(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("e"));
            __mm_s.push_str(&*intString(id));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("n"));
            __mm_s.push_str(&*intString(e));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("v"));
            __mm_s.push_str(&*intString(v));
            ArcStr::from(__mm_s)
        },
        arcstr::literal!(GraphML::COLOR_BLACK),
        openmodelica_codegen_graphml::GraphML::LineType::LINE,
        GraphML::LINEWIDTH_STANDARD.clone(),
        false,
        labels,
        arrow,
        metamodelica::nil(),
        graph,
    )?;
    outTpl = (id + 1, r, text, graph);
    Ok(outTpl)
}

fn addCompsGraph<'__b>(
    mut iComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut vars: &'__b BackendDAE::Variables,
    mut varcomp: metamodelica::Array<i32>,
    mut iN: i32,
    mut iGraph: (GraphML::GraphInfo, i32),
) -> Result<(GraphML::GraphInfo, i32)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iComps, iGraph.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(iGraph)
            },
            (Deref @ metamodelica::ListNode::Cons { head: comp, tail: rest }, (graphInfo, graph)) => {
                let mut vlst: metamodelica::List<i32>;
                let mut label: GraphML::NodeLabel;
                let mut varcomp1: metamodelica::Array<i32>;
                let mut text: ArcStr;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut graphInfo = (*graphInfo).clone();
                (_, vlst) = BackendDAETransform::getEquationAndSolvedVarIndxes(metamodelica::AsArg::as_arg(&comp))?;
                varcomp1 = List::fold1r(&vlst, &*(Arc::new(arrayUpdate.clone())), iN, varcomp.clone())?;
                varlst = List::map1r(vlst, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                text = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(iN)); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*stringDelimitList(List::mapMap(varlst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) }, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?, literal!("\n"))); ArcStr::from(__mm_s) };
                label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: text, backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
                (graphInfo, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(iN)); ArcStr::from(__mm_s) }, arcstr::literal!(GraphML::COLOR_GREEN), GraphML::BORDERWIDTH_STANDARD.clone(), list![label], openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                { (iComps, vars, varcomp, iN, iGraph) = (rest.clone(), vars, varcomp1.clone(), iN + 1, (graphInfo.clone(), graph.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn addCompsEdgesGraph<'__b>(
    mut iComps: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut varcomp: metamodelica::Array<i32>,
    mut iN: i32,
    mut id: i32,
    mut markarray: metamodelica::Array<i32>,
    mut mark: i32,
    mut iGraph: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    '__tco: loop {
        ::match_deref::match_deref! { match iComps {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iGraph)
            },
            Deref @ metamodelica::ListNode::Cons { head: comp, tail: rest } => {
                let mut elst: metamodelica::List<i32>;
                let mut vlst: metamodelica::List<i32>;
                let mut n: i32;
                let mut graph: GraphML::GraphInfo;
                (elst, vlst) = BackendDAETransform::getEquationAndSolvedVarIndxes(metamodelica::AsArg::as_arg(&comp))?;
                List::fold1r(&vlst, &*(Arc::new(arrayUpdate.clone())), mark, markarray.clone())?;
                vlst = getUsedVarsComp(&elst, m.clone(), markarray.clone(), mark)?;
                (n, graph) = addCompEdgesGraph(&vlst, varcomp.clone(), markarray.clone(), mark + 1, iN, id, &iGraph);
                { (iComps, m, varcomp, iN, id, markarray, mark, iGraph) = (rest, m.clone(), varcomp.clone(), iN + 1, n, markarray.clone(), mark + 2, graph); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getUsedVarsComp(
    mut iEqns: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut markarray: metamodelica::Array<i32>,
    mut mark: i32,
) -> Result<metamodelica::List<i32>> {
    let __ab_m = m.borrow();
    let mut oVars: metamodelica::List<i32> = metamodelica::nil();
    let mut vlst: metamodelica::List<i32>;
    for mut eq in &**iEqns {
        vlst = List::select1(
            (*metamodelica::index_checked(&__ab_m, eq.clone())?).clone(),
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            0,
        )?;
        vlst = List::select1r(
            vlst,
            (std::sync::Arc::new(isUnMarked)
                as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Array<i32>, i32), i32) -> Result<bool> + 'static>),
            (markarray.clone(), mark),
        )?;
        List::fold1r(&vlst, &*(Arc::new(arrayUpdate.clone())), mark, markarray.clone())?;
        oVars = listAppend(vlst, oVars);
    }
    Ok(oVars)
}

fn addCompEdgesGraph(
    mut iVars: &metamodelica::List<i32>,
    mut varcomp: metamodelica::Array<i32>,
    mut markarray: metamodelica::Array<i32>,
    mut mark: i32,
    mut iN: i32,
    mut id: i32,
    mut iGraph: &GraphML::GraphInfo,
) -> (i32, GraphML::GraphInfo) {
    let mut oN: i32;
    let mut oGraph: GraphML::GraphInfo;
    (oN, oGraph) = 'mc: {
        let __mc_input = &**iVars;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((id, iGraph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: v, tail: rest } => {
                    let mut n: i32;
                    let mut c: i32;
                    let mut graph: GraphML::GraphInfo;
                    c = ({let __elt = (*metamodelica::index_checked(&varcomp.borrow(), v.clone())?).clone(); __elt});
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&markarray.borrow(), c)?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(markarray.clone(), c, mark)?;
                    (graph, _) = GraphML::addEdge({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*intString(id)); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(c)); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(iN)); ArcStr::from(__mm_s) }, arcstr::literal!(GraphML::COLOR_BLACK), openmodelica_codegen_graphml::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), iGraph.clone())?;
                    (n, graph) = addCompEdgesGraph(metamodelica::AsArg::as_arg(&rest), varcomp.clone(), markarray.clone(), mark, iN, id + 1, &graph);
                    Ok((n, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut n: i32;
                    let mut graph: GraphML::GraphInfo;
                    (n, graph) = addCompEdgesGraph(metamodelica::AsArg::as_arg(&rest), varcomp.clone(), markarray.clone(), mark, iN, id, iGraph);
                    Ok((n, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oN, oGraph)
}

fn isUnMarked(mut ass: (metamodelica::Array<i32>, i32), mut indx: i32) -> Result<bool> {
    let mut b: bool;
    let mut arr: metamodelica::Array<i32>;
    let mut mark: i32;
    (arr, mark) = ass;
    b = !(intEq(
        ({
            let __elt = (*metamodelica::index_checked(&arr.borrow(), intAbs(indx))?).clone();
            __elt
        }),
        mark,
    ));
    Ok(b)
}
