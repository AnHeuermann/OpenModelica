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

use crate::FGraph;
use crate::FNode;
use openmodelica_ast::Absyn;
use openmodelica_codegen_graphml::GraphML;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::FCore::RefTree;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

pub type Name = ArcStr;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = metamodelica::Ref<FCore::Node>;

pub type Data = metamodelica::Ref<FCore::Data>;

pub type Kind = FCore::Kind;

pub type Ref = Mutable::Mutable<metamodelica::Ref<FCore::Node>>;

pub type Refs = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type Children = metamodelica::Ref<FCore::RefTree::Tree>;

pub type Parents = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Graph = FCore::Graph;

pub type Type = metamodelica::Ref<DAE::Type>;

pub type Types = metamodelica::List<metamodelica::Ref<DAE::Type>>;

pub(crate) fn dumpGraph(mut inGraph: &Graph, mut fileName: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = fileName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (Flags::isSet(Flags::GRAPH_INST_GEN_GRAPH.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut g: i32;
            let mut gi: GraphML::GraphInfo;
            let mut nr: Ref;
            gi = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("G"), false, gi.clone())?;
            gi = metamodelica::Own::own(__pa0);
            g = metamodelica::Own::own(__pa1);
            nr = FGraph::top(inGraph)?;
            (gi, g) = addNodes((gi.clone(), g), list![nr.clone()])?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Dumping graph file: "));
                __mm_s.push_str(&*fileName);
                __mm_s.push_str(&*literal!(" ....\n"));
                ArcStr::from(__mm_s)
            });
            GraphML::dumpGraph(gi.clone(), fileName.clone())?;
            metamodelica::print(literal!("Dumped\n"));
            Ok(())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn addNodes(
    mut gin: (GraphML::GraphInfo, i32),
    mut inRefs: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
) -> Result<(GraphML::GraphInfo, i32)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((gin.clone(), inRefs)) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(gin)
            },
            (g, Deref @ metamodelica::ListNode::Cons { head: n, tail: rest }) if (!(FNode::isRefTop(n.clone())) && !(FNode::isRefUserDefined(n.clone())?)) => {
                { (gin, inRefs) = (g.clone(), rest.clone()); continue '__tco; }
            },
            (g, Deref @ metamodelica::ListNode::Cons { head: n, tail: rest }) => {
                let mut g = (*g).clone();
                g = addNode(&(g.clone()), &(FNode::fromRef(n.clone())))?;
                { (gin, inRefs) = (g.clone(), rest.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn addNode(mut gin: &(GraphML::GraphInfo, i32), mut node: &Node) -> Result<(GraphML::GraphInfo, i32)> {
    let mut gout: (GraphML::GraphInfo, i32);
    gout = (::match_deref::match_deref! { match &((gin, &**node)) {
        ((gi, i), Deref @ FCore::Node { parents: Deref @ metamodelica::ListNode::Nil, children: kids, .. }) => {
            let mut nds: ArcStr;
            let mut color: ArcStr;
            let mut labelText: ArcStr;
            let mut shape: GraphML::ShapeType;
            let mut nrefs: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node, true)?;
            labelText = nds;
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText, backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, color, GraphML::BORDERWIDTH_STANDARD.clone(), list![label], shape, None, metamodelica::nil(), i.clone(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(metamodelica::AsArg::as_arg(&kids), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs)?;
            (gi.clone(), i.clone())
        },
        ((gi, i), Deref @ FCore::Node { parents: Deref @ metamodelica::ListNode::Cons { head: nr, tail: _ }, children: kids, data: Deref @ FCore::Data::REF { target: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            let mut nds: ArcStr;
            let mut color: ArcStr;
            let mut labelText: ArcStr;
            let mut shape: GraphML::ShapeType;
            let mut nrefs: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node, true)?;
            labelText = nds;
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText, backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, color, GraphML::BORDERWIDTH_STANDARD.clone(), list![label], shape, None, metamodelica::nil(), i.clone(), gi.clone())?;
            (gi, _) = GraphML::addEdge({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("r")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(&(FNode::fromRef(MutableWeak::upgrade(nr.clone())?))))); ArcStr::from(__mm_s) }, arcstr::literal!(GraphML::COLOR_RED), openmodelica_codegen_graphml::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(metamodelica::AsArg::as_arg(&kids), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs)?;
            (gi.clone(), i.clone())
        },
        ((gi, i), Deref @ FCore::Node { parents: Deref @ metamodelica::ListNode::Cons { head: nr, tail: _ }, children: kids, data: Deref @ FCore::Data::REF { target: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }) => {
            let mut nds: ArcStr;
            let mut color: ArcStr;
            let mut labelText: ArcStr;
            let mut shape: GraphML::ShapeType;
            let mut nrefs: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node, true)?;
            labelText = nds;
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText, backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, color, GraphML::BORDERWIDTH_STANDARD.clone(), list![label], shape, None, metamodelica::nil(), i.clone(), gi.clone())?;
            (gi, _) = GraphML::addEdge({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("r")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(&(FNode::fromRef(MutableWeak::upgrade(nr.clone())?))))); ArcStr::from(__mm_s) }, arcstr::literal!(GraphML::COLOR_GREEN), openmodelica_codegen_graphml::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(metamodelica::AsArg::as_arg(&kids), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs)?;
            (gi.clone(), i.clone())
        },
        ((gi, i), Deref @ FCore::Node { parents: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, data: Deref @ FCore::Data::VR { .. }, .. }) => {
            (gi.clone(), i.clone())
        },
        ((gi, i), Deref @ FCore::Node { parents: Deref @ metamodelica::ListNode::Cons { head: nr, tail: _ }, children: kids, .. }) => {
            let mut nds: ArcStr;
            let mut color: ArcStr;
            let mut labelText: ArcStr;
            let mut shape: GraphML::ShapeType;
            let mut nrefs: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node, true)?;
            labelText = nds;
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: labelText, backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, color, GraphML::BORDERWIDTH_STANDARD.clone(), list![label], shape, None, metamodelica::nil(), i.clone(), gi.clone())?;
            (gi, _) = GraphML::addEdge({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node))); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(&(FNode::fromRef(MutableWeak::upgrade(nr.clone())?))))); ArcStr::from(__mm_s) }, arcstr::literal!(GraphML::COLOR_BLACK), openmodelica_codegen_graphml::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(metamodelica::AsArg::as_arg(&kids), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs)?;
            (gi.clone(), i.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(gout)
}

pub(crate) fn graphml(mut node: &Node, mut escape: bool) -> Result<(ArcStr, GraphML::ShapeType, ArcStr)> {
    let mut color: ArcStr;
    let mut shape: GraphML::ShapeType;
    let mut nname: ArcStr;
    (color, shape, nname) = 'mc: {
        let __mc_input = &**node;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: Deref @ FCore::Data::CL { e, .. } } => {
                    let mut s: ArcStr;
                    let mut b: bool;
                    let true = (SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    b = FNode::isClassExtends(node);
                    s = if (b) {literal!("rdrpCE:")} else {literal!("rdrpC:")};
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_codegen_graphml::GraphML::ShapeType::HEXAGON, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: Deref @ FCore::Data::CL { e, .. } } => {
                    let mut s: ArcStr;
                    let mut b: bool;
                    let true = (SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    b = FNode::isClassExtends(node);
                    s = if (b) {literal!("rdCE:")} else {literal!("rdC:")};
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_codegen_graphml::GraphML::ShapeType::HEXAGON, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: Deref @ FCore::Data::CL { e, .. } } => {
                    let mut s: ArcStr;
                    let true = (SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rpC:")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: Deref @ FCore::Data::CO { e, .. } } => {
                    let mut s: ArcStr;
                    let true = (SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rdrpc:")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: Deref @ FCore::Data::CO { e, .. } } => {
                    let mut s: ArcStr;
                    let true = (SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rdc:")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: Deref @ FCore::Data::CO { e, .. } } => {
                    let mut s: ArcStr;
                    let true = (SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rpc:")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::CL { .. } } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_GRAY), openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::CO { .. } } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_WHITE), openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::EX { .. } } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_GREEN), openmodelica_codegen_graphml::GraphML::ShapeType::ROUNDRECTANGLE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::EXP { e: exp, .. } } => {
                    let mut s: ArcStr;
                    s = Dump::printExpStr(exp.clone())?;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*if (escape) {Util::escapeModelicaStringToXmlString(s.clone())?} else {Util::stringTrunc(s.clone(), 100)?}); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_PURPLE), openmodelica_codegen_graphml::GraphML::ShapeType::HEXAGON, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::DIMS { dims, .. } } => {
                    let mut s: ArcStr;
                    s = Dump::printArraydimStr(dims.clone())?;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*if (escape) {Util::escapeModelicaStringToXmlString(s.clone())?} else {Util::stringTrunc(s.clone(), 100)?}); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_PINK), openmodelica_codegen_graphml::GraphML::ShapeType::TRIANGLE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::CR { r } } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&r))?); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_PURPLE), openmodelica_codegen_graphml::GraphML::ShapeType::OCTAGON, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::ASSERT { message: s } } => {
                    let mut s = (*s).clone();
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_codegen_graphml::GraphML::ShapeType::PARALLELOGRAM, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::REF { target: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*literal!("UNRESOLVED")); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_codegen_graphml::GraphML::ShapeType::PARALLELOGRAM, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ Deref @ FCore::Data::REF { target: Deref @ metamodelica::ListNode::Cons { head: target, tail: _ } } } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(target.clone())))?); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_GREEN), openmodelica_codegen_graphml::GraphML::ShapeType::TRAPEZOID, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: _, children: _, data: nd } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(metamodelica::AsArg::as_arg(&nd))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node)); ArcStr::from(__mm_s) };
                    Ok((arcstr::literal!(GraphML::COLOR_BLUE), openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((color, shape, nname))
}
