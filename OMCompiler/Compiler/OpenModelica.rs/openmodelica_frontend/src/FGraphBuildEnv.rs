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
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::SCodeInstUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

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

pub type Scope = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub(crate) fn mkProgramGraph(
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inKind: Kind,
    mut graph: Graph,
) -> Result<Graph> {
    let mut graph: Graph = graph;
    let mut topRef: Ref;
    topRef = FGraph::top(&graph)?;
    for mut cls in &**inProgram {
        graph = mkClassGraph(cls.clone(), topRef.clone(), inKind, graph, true)?;
    }
    Ok(graph)
}

fn mkClassGraph(
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
    mut checkDuplicate: bool,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match &*inClass {
        SCode::Element::CLASS { .. } => {
            let mut g = inGraph;
            g = mkClassNode(
                inClass,
                openmodelica_frontend_types::DAE::Prefix::NOPRE,
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
                inParentRef,
                inKind,
                g,
                checkDuplicate,
            )?;
            g
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outGraph)
}

pub(crate) fn mkClassNode(
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
    mut checkDuplicate: bool,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut cls: metamodelica::Ref<SCode::Element>;
            let mut name: ArcStr;
            let mut n: Node;
            let mut nr: Ref;
            cls = SCodeInstUtil::expandEnumerationClass(inClass)?;
            let __pa0 = ::match_deref::match_deref! { match &(cls.clone()) {
                Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            (g, n) = FGraph::node(
                g,
                name.clone(),
                list![inParentRef.clone()],
                metamodelica::Ref::new(FCore::Data::CL {
                    e: cls,
                    pre: inPrefix,
                    r#mod: inMod,
                    kind: inKind,
                    status: openmodelica_frontend_dump::FCore::Status::CLS_UNTYPED,
                }),
            );
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &name, nr, checkDuplicate)?;
            g
        }
    });
    Ok(outGraph)
}

pub(crate) fn mkConstrainClass(
    mut inElement: &metamodelica::Ref<SCode::Element>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (&**inElement, inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) }, .. }, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), arcstr::literal!(FNode::ccNodeName), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::CC { cc: cc.clone() }));
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), &(arcstr::literal!(FNode::ccNodeName)), nr.clone(), false)?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) }, .. }, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), arcstr::literal!(FNode::ccNodeName), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::CC { cc: cc.clone() }));
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), &(arcstr::literal!(FNode::ccNodeName)), nr.clone(), false)?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGraph.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outGraph
}

pub(crate) fn mkModNode(
    mut inName: Name,
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inName, &*inMod, inGraph);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::NOMOD { .. }, g) => {
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, binding: None, .. }, g) => {
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, binding: b @ Some(_), .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), name.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::MO { m: inMod.clone() }));
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), metamodelica::AsArg::as_arg(&name), nr.clone(), false)?;
                    g = mkBindingNode(b.clone(), nr.clone(), inKind, g.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::MOD { subModLst: sm, binding: b, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), name.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::MO { m: inMod.clone() }));
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), metamodelica::AsArg::as_arg(&name), nr.clone(), false)?;
                    g = mkSubMods(sm.clone(), nr.clone(), inKind, g.clone())?;
                    g = mkBindingNode(b.clone(), nr.clone(), inKind, g.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::REDECL { element: e, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), name.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::MO { m: inMod.clone() }));
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), metamodelica::AsArg::as_arg(&name), nr.clone(), false)?;
                    g = mkElementNode(e.clone(), nr.clone(), inKind, g.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, _, g) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraphBuildEnv.mkModNode failed with: ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(" mod: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(g.clone())
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

pub(crate) fn mkSubMods(
    mut inSubMod: metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inSubMod, inGraph)) {
            (Deref @ metamodelica::ListNode::Nil, g) => {
                return Ok(g.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: id, r#mod: m }, tail: rest }, g) => {
                let mut g = (*g).clone();
                g = mkModNode(id.clone(), m.clone(), inParentRef.clone(), inKind, g.clone())?;
                { (inSubMod, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn mkBindingNode(
    mut inBinding: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &(inBinding) {
        None => {
            let mut g = inGraph;
            g
        },
        Some(e) => {
            let mut g = inGraph;
            g = mkExpressionNode(arcstr::literal!(FNode::bndNodeName), e.clone(), inParentRef, inKind, g)?;
            g
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

fn mkClassChildren(
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (&**inClassDef, inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: el, normalEquationLst: eqs, initialEquationLst: ieqs, normalAlgorithmLst: als, initialAlgorithmLst: ials, constraintLst, clsattrs, externalDecl }, g) => {
                    let mut g = (*g).clone();
                    g = List::fold2(metamodelica::AsArg::as_arg(&el), &mkElementNode, inParentRef.clone(), inKind, g.clone())?;
                    g = mkEqNode(arcstr::literal!(FNode::eqNodeName), eqs.clone(), inParentRef.clone(), inKind, g.clone())?;
                    g = mkEqNode(arcstr::literal!(FNode::ieqNodeName), ieqs.clone(), inParentRef.clone(), inKind, g.clone())?;
                    g = mkAlNode(arcstr::literal!(FNode::alNodeName), als.clone(), inParentRef.clone(), inKind, g.clone())?;
                    g = mkAlNode(arcstr::literal!(FNode::ialNodeName), ials.clone(), inParentRef.clone(), inKind, g.clone())?;
                    g = mkOptNode(arcstr::literal!(FNode::optNodeName), constraintLst.clone(), clsattrs.clone(), inParentRef.clone(), inKind, g.clone())?;
                    g = mkExternalNode(arcstr::literal!(FNode::edNodeName), externalDecl.clone(), inParentRef.clone(), inKind, g.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: cdef, modifications: m }, g) => {
                    let mut g = (*g).clone();
                    g = mkClassChildren(metamodelica::AsArg::as_arg(&cdef), inParentRef.clone(), inKind, g.clone());
                    g = mkModNode(arcstr::literal!(FNode::modNodeName), m.clone(), inParentRef.clone(), inKind, g.clone())?;
                    g = mkRefNode(arcstr::literal!(FNode::refNodeName), metamodelica::nil(), inParentRef.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::DERIVED { typeSpec: ts, modifications: m, .. }, g) => {
                    let mut nr: Ref;
                    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut g = (*g).clone();
                    nr = inParentRef.clone();
                    g = mkModNode(arcstr::literal!(FNode::modNodeName), m.clone(), nr.clone(), inKind, g.clone())?;
                    ad = AbsynUtil::typeSpecDimensions(metamodelica::AsArg::as_arg(&ts));
                    g = mkDimsNode(arcstr::literal!(FNode::tydimsNodeName), Some(ad.clone()), nr.clone(), inKind, g.clone())?;
                    g = mkRefNode(arcstr::literal!(FNode::refNodeName), metamodelica::nil(), nr.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::OVERLOAD { pathLst: _ }, g) => {
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PDER { functionPath: _, derivedVariables: _ }, g) => {
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGraph.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outGraph
}

pub(crate) fn mkElementNode(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match &*inElement.clone() {
        SCode::Element::COMPONENT { .. } => {
            let mut g = inGraph;
            g = mkCompNode(inElement, inParentRef, inKind, g)?;
            g
        }
        SCode::Element::CLASS { .. } => {
            let mut g = inGraph;
            g = mkClassNode(
                inElement,
                openmodelica_frontend_types::DAE::Prefix::NOPRE,
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
                inParentRef,
                inKind,
                g,
                false,
            )?;
            g
        }
        SCode::Element::EXTENDS {
            baseClassPath: p,
            modifications: m,
            ..
        } => {
            let mut g = inGraph;
            let mut name: ArcStr;
            let mut n: Node;
            let mut nr: Ref;
            name = FNode::mkExtendsName(p.clone())?;
            (g, n) = FGraph::node(
                g,
                name.clone(),
                list![inParentRef.clone()],
                metamodelica::Ref::new(FCore::Data::EX {
                    e: inElement,
                    r#mod: openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
                }),
            );
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &name, nr.clone(), false)?;
            g = mkModNode(arcstr::literal!(FNode::modNodeName), m.clone(), nr.clone(), inKind, g)?;
            g = mkRefNode(arcstr::literal!(FNode::refNodeName), metamodelica::nil(), nr, g)?;
            g
        }
        SCode::Element::IMPORT { .. } => {
            let mut g = inGraph;
            g = mkImportNode(&inElement, inParentRef, inKind, g)?;
            g
        }
        SCode::Element::DEFINEUNIT { .. } => {
            let mut g = inGraph;
            g = mkUnitsNode(inElement, inParentRef, inKind, g)?;
            g
        }
    });
    Ok(outGraph)
}

pub(crate) fn mkUnitsNode(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            r = FNode::child(inParentRef.clone(), arcstr::literal!(FNode::duNodeName))?;
            FNode::addDefinedUnitToRef(r.clone(), inElement.clone())?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut n: Node;
            let mut r: Ref;
            (g, n) = FGraph::node(
                g.clone(),
                arcstr::literal!(FNode::duNodeName),
                list![inParentRef.clone()],
                metamodelica::Ref::new(FCore::Data::DU {
                    els: list![inElement.clone()],
                }),
            );
            r = FNode::toRef(n.clone());
            FNode::addChildRef(
                inParentRef.clone(),
                &(arcstr::literal!(FNode::duNodeName)),
                r.clone(),
                false,
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

pub(crate) fn mkImportNode(
    mut inElement: &metamodelica::Ref<SCode::Element>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            r = FNode::child(inParentRef.clone(), arcstr::literal!(FNode::imNodeName))?;
            FNode::addImportToRef(r.clone(), inElement)?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut n: Node;
            let mut r: Ref;
            (g, n) = FGraph::node(
                g.clone(),
                arcstr::literal!(FNode::imNodeName),
                list![inParentRef.clone()],
                metamodelica::Ref::new(FCore::Data::IM {
                    i: FCore::emptyImportTable.clone(),
                }),
            );
            r = FNode::toRef(n.clone());
            FNode::addChildRef(
                inParentRef.clone(),
                &(arcstr::literal!(FNode::imNodeName)),
                r.clone(),
                false,
            )?;
            FNode::addImportToRef(r.clone(), inElement)?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

pub(crate) fn mkDimsNode(
    mut inName: Name,
    mut inArrayDims: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &(inArrayDims) {
        None => {
            let mut g = inGraph;
            g
        },
        Some(Deref @ metamodelica::ListNode::Nil) => {
            let mut g = inGraph;
            g
        },
        Some(a @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
            let mut g = inGraph;
            let mut n: Node;
            let mut nr: Ref;
            (g, n) = FGraph::node(g, inName.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::DIMS { name: inName.clone(), dims: a.clone() }));
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &inName, nr.clone(), false)?;
            g = mkDimsNode_helper(0, a.clone(), nr, inKind, g)?;
            g
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkDimsNode_helper(
    mut inStartWith: i32,
    mut inArrayDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inStartWith, inArrayDims, inGraph)) {
            (_, Deref @ metamodelica::ListNode::Nil, g) => {
                return Ok(g.clone())
            },
            (i, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: rest }, g) => {
                let mut name: Name;
                let mut g = (*g).clone();
                name = intString(i.clone());
                g = mkExpressionNode(name, openmodelica_ast::Absyn::Exp::interned_END(), inParentRef.clone(), inKind, g.clone())?;
                { (inStartWith, inArrayDims, inParentRef, inKind, inGraph) = (i.clone() + 1, rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            (i, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: rest }, g) => {
                let mut name: Name;
                let mut g = (*g).clone();
                name = intString(i.clone());
                g = mkExpressionNode(name, e.clone(), inParentRef.clone(), inKind, g.clone())?;
                { (inStartWith, inArrayDims, inParentRef, inKind, inGraph) = (i.clone() + 1, rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn mkCompNode(
    mut inComp: metamodelica::Ref<SCode::Element>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut name: ArcStr;
    let mut g: Graph;
    let mut n: Node;
    let mut nr: Ref;
    let mut m: metamodelica::Ref<SCode::Mod>;
    let mut cnd: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut ts: metamodelica::Ref<Absyn::TypeSpec>;
    let mut nd: Data;
    let mut i: metamodelica::Ref<DAE::Var>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, attributes: SCode::Attributes { arrayDims: __pa1, .. }, typeSpec: __pa2, modifications: __pa3, condition: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    ad = metamodelica::Own::own(__pa1);
    ts = metamodelica::Own::own(__pa2);
    m = metamodelica::Own::own(__pa3);
    cnd = metamodelica::Own::own(__pa4);
    (nd, i) = FNode::element2Data(inComp, inKind)?;
    (g, n) = FGraph::node(inGraph, name.clone(), list![inParentRef.clone()], nd);
    nr = FNode::toRef(n);
    FNode::addChildRef(inParentRef, &name, nr.clone(), false)?;
    g = mkInstNode(i, nr.clone(), g)?;
    g = mkRefNode(arcstr::literal!(FNode::refNodeName), metamodelica::nil(), nr, g)?;
    outGraph = g;
    Ok(outGraph)
}

pub(crate) fn mkInstNode(
    mut inVar: metamodelica::Ref<DAE::Var>,
    mut inParentRef: Ref,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut nr: Ref;
    let mut n: Node;
    let mut g: Graph;
    (g, n) = FGraph::node(
        inGraph,
        arcstr::literal!(FNode::itNodeName),
        list![inParentRef.clone()],
        metamodelica::Ref::new(FCore::Data::IT { i: inVar }),
    );
    nr = FNode::toRef(n);
    FNode::addChildRef(inParentRef, &(arcstr::literal!(FNode::itNodeName)), nr, false)?;
    outGraph = g;
    Ok(outGraph)
}

pub(crate) fn mkConditionNode(
    mut inCondition: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &(inCondition) {
        None => {
            let mut g = inGraph;
            g
        },
        Some(e) => {
            let mut g = inGraph;
            g = mkExpressionNode(arcstr::literal!(FNode::cndNodeName), e.clone(), inParentRef, inKind, g)?;
            g
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkExpressionNode(
    mut inName: Name,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inExp, inGraph)) {
        (e, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), inName.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::EXP { name: inName.clone(), e: e.clone() }));
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &inName, nr.clone(), false)?;
            g = analyseExp(e.clone(), nr, inKind, g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkCrefsNodes(
    mut inCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCrefs, inGraph)) {
            (Deref @ metamodelica::ListNode::Nil, g) => {
                return Ok(g.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest }, g) => {
                let mut g = (*g).clone();
                g = mkCrefNode(cr.clone(), inParentRef.clone(), inKind, g.clone())?;
                { (inCrefs, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn mkCrefNode(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut n: Node;
            let mut nr: Ref;
            let mut name: Name;
            name = Dump::printComponentRefStr(&inCref)?;
            (g, n) = FGraph::node(
                g,
                name.clone(),
                list![inParentRef.clone()],
                metamodelica::Ref::new(FCore::Data::CR { r: inCref.clone() }),
            );
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &name, nr.clone(), false)?;
            g = mkDimsNode(
                arcstr::literal!(FNode::subsNodeName),
                List::mkOption(AbsynUtil::getSubsFromCref(&inCref, true, true)?),
                nr,
                inKind,
                g,
            )?;
            g
        }
    });
    Ok(outGraph)
}

pub(crate) fn mkTypeNode(
    mut inTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inParentRef: Ref,
    mut inName: Name,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph =
        'mc: {
            let __mc_input = inGraph.clone();
            if let Ok(__v) = (|| -> Result<_> {
                let _ = __mc_input.clone() else { return Err("nomatch") };
                let mut nr: Ref;
                let mut pr: Ref;
                pr = FNode::child(inParentRef.clone(), arcstr::literal!(FNode::tyNodeName))?;
                nr = FNode::child(pr.clone(), inName.clone())?;
                FNode::addTypesToRef(nr.clone(), inTypes.clone())?;
                Ok(inGraph.clone())
            })() {
                break 'mc __v;
            }
            if let Ok(__v) =
                (|| -> Result<_> {
                    let mut g = __mc_input.clone() else {
                        return Err("nomatch");
                    };
                    let mut nr: Ref;
                    let mut pr: Ref;
                    let mut n: Node;
                    if '__try0: {
                unwrap_break_err!(FNode::child(inParentRef.clone(), arcstr::literal!(FNode::tyNodeName)), '__try0);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
                    (g, n) = FGraph::node(
                        g.clone(),
                        arcstr::literal!(FNode::tyNodeName),
                        list![inParentRef.clone()],
                        metamodelica::Ref::new(FCore::Data::ND { scopeType: None }),
                    );
                    pr = FNode::toRef(n.clone());
                    FNode::addChildRef(
                        inParentRef.clone(),
                        &(arcstr::literal!(FNode::tyNodeName)),
                        pr.clone(),
                        false,
                    )?;
                    (g, n) = FGraph::node(
                        g.clone(),
                        inName.clone(),
                        list![pr.clone()],
                        metamodelica::Ref::new(FCore::Data::FT { tys: inTypes.clone() }),
                    );
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(pr.clone(), &inName, nr.clone(), false)?;
                    Ok(g.clone())
                })()
            {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let mut g = __mc_input.clone() else {
                    return Err("nomatch");
                };
                let mut nr: Ref;
                let mut pr: Ref;
                let mut n: Node;
                pr = FNode::child(inParentRef.clone(), arcstr::literal!(FNode::tyNodeName))?;
                if '__try0: {
                    unwrap_break_err!(FNode::child(pr.clone(), inName.clone()), '__try0);
                    Ok::<(), &'static str>(())
                }
                .is_ok()
                {
                    return Err("failure(): body succeeded");
                }
                (g, n) = FGraph::node(
                    g.clone(),
                    inName.clone(),
                    list![pr.clone()],
                    metamodelica::Ref::new(FCore::Data::FT { tys: inTypes.clone() }),
                );
                nr = FNode::toRef(n.clone());
                FNode::addChildRef(pr.clone(), &inName, nr.clone(), false)?;
                Ok(g.clone())
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let _ = __mc_input.clone() else { return Err("nomatch") };
                let mut pr: Ref;
                pr = FGraph::top(&inGraph)?;
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("FGraphBuildEnv.mkTypeNode: Error making type node: "));
                    __mm_s.push_str(&*inName);
                    __mm_s.push_str(&*literal!(" in parent: "));
                    __mm_s.push_str(&*FNode::name(&(FNode::fromRef(pr.clone()))));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                Ok(inGraph.clone())
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
    Ok(outGraph)
}

pub(crate) fn mkEqNode(
    mut inName: Name,
    mut inEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &(inEqs.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut g = inGraph;
            g
        },
        _ => {
            let mut g = inGraph;
            let mut n: Node;
            let mut nr: Ref;
            (g, n) = FGraph::node(g, inName.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::EQ { name: inName.clone(), e: inEqs.clone() }));
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &inName, nr.clone(), false)?;
            g = List::fold2(&inEqs, &analyseEquation, nr, inKind, g)?;
            g
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkAlNode(
    mut inName: Name,
    mut inAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &(inAlgs.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut g = inGraph;
            g
        },
        _ => {
            let mut g = inGraph;
            let mut n: Node;
            let mut nr: Ref;
            (g, n) = FGraph::node(g, inName.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::AL { name: inName.clone(), a: inAlgs.clone() }));
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &inName, nr.clone(), false)?;
            g = List::fold2(&inAlgs, &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>, __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>, __a2: FCore::Kind, __a3: FCore::Graph| analyseAlgorithm(&__a0, __a1, __a2, __a3), nr, inKind, g)?;
            g
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkOptNode(
    mut inName: Name,
    mut inConstraintLst: metamodelica::List<SCode::ConstraintSection>,
    mut inClsAttrs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inConstraintLst.clone(), inClsAttrs.clone())) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut g = inGraph;
            g
        },
        (_, _) => {
            let mut g = inGraph;
            let mut n: Node;
            let mut nr: Ref;
            (g, n) = FGraph::node(g, inName.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::OT { constrainLst: inConstraintLst, clsAttrs: inClsAttrs }));
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &inName, nr, false)?;
            g
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkExternalNode(
    mut inName: Name,
    mut inExternalDeclOpt: Option<metamodelica::Ref<SCode::ExternalDecl>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &(inExternalDeclOpt) {
        None => {
            let mut g = inGraph;
            g
        },
        Some(ed @ Deref @ SCode::ExternalDecl { output_: ocr, args: exps, .. }) => {
            let mut g = inGraph;
            let mut n: Node;
            let mut nr: Ref;
            let mut oae: Option<metamodelica::Ref<Absyn::Exp>>;
            (g, n) = FGraph::node(g, inName.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::ED { ed: ed.clone() }));
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &inName, nr.clone(), false)?;
            oae = Util::applyOption(ocr.clone(), &fnptr!(AbsynUtil::crefExp, metamodelica::Ref<Absyn::ComponentRef>))?;
            g = mkCrefsFromExps(List::consOption(oae, exps.clone()), nr, inKind, g)?;
            g
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkCrefsFromExps(
    mut inExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExps, inGraph)) {
            (Deref @ metamodelica::ListNode::Nil, g) => {
                return Ok(g.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, g) => {
                let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut g = (*g).clone();
                crefs = AbsynUtil::getCrefFromExp(e.clone(), true, true)?;
                g = mkCrefsNodes(crefs, inParentRef.clone(), inKind, g.clone())?;
                { (inExps, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn analyseExp(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    (_, outGraph) = AbsynUtil::traverseExpBidir(
        inExp,
        (std::sync::Arc::new({
            let __pe_b1 = inRef;
            let __pe_b2 = inKind;
            move |__pe_a0, __pe_a3| analyseExpTraverserEnter(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        FCore::Graph,
                    ) -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)>
                    + 'static,
            >),
        (std::sync::Arc::new(fnptr!(
            analyseExpTraverserExit,
            metamodelica::Ref<Absyn::Exp>,
            FCore::Graph
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        FCore::Graph,
                    ) -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)>
                    + 'static,
            >),
        inGraph,
    )?;
    Ok(outGraph)
}

fn analyseOptExp(
    mut inExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &(inExp) {
        None => {
            let mut g = inGraph;
            g
        },
        Some(exp) => {
            let mut g = inGraph;
            g = analyseExp(exp.clone(), inRef, inKind, g)?;
            g
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

fn analyseExpTraverserEnter(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut r#ref: Ref,
    mut kind: Kind,
    mut graph: Graph,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Graph)> {
    let mut inExp: metamodelica::Ref<Absyn::Exp> = inExp;
    let mut graph: Graph = graph;
    graph = (::match_deref::match_deref! { match &(&*inExp) {
        Deref @ Absyn::Exp::CREF { componentRef: cref } => {
            analyseCref(cref.clone(), r#ref, kind, graph)?
        },
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators: iters, .. }, .. } => {
            addIterators(iters.clone(), r#ref, kind, graph)?
        },
        Deref @ Absyn::Exp::CALL { function_: cref, .. } => {
            analyseCref(cref.clone(), r#ref, kind, graph)?
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, .. } => {
            analyseCref(cref.clone(), r#ref, kind, graph)?
        },
        Deref @ Absyn::Exp::MATCHEXP { .. } => {
            addMatchScope(inExp.clone(), r#ref, kind, graph)?
        },
        _ => {
            graph
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((inExp, graph))
}

fn analyseCref(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match &*inCref {
        Absyn::ComponentRef::WILD { .. } => {
            let mut g = inGraph;
            g
        }
        _ => {
            let mut g = inGraph;
            g = mkCrefNode(inCref, inParentRef, inKind, g)?;
            g
        }
    });
    Ok(outGraph)
}

fn analyseExpTraverserExit(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut graph: Graph,
) -> (metamodelica::Ref<Absyn::Exp>, Graph) {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut graph: Graph = graph;
    (exp, graph)
}

fn analyseEquation(
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    (_, outGraph) = SCodeUtil::mapFoldEquations(
        inEquation,
        (std::sync::Arc::new({
            let __pe_b1 = inParentRef;
            let __pe_b2 = inKind;
            move |__pe_a0, __pe_a3| analyseEquationTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SCode::Equation>,
                        FCore::Graph,
                    ) -> Result<(metamodelica::Ref<SCode::Equation>, FCore::Graph)>
                    + 'static,
            >),
        inGraph,
    )?;
    Ok(outGraph)
}

fn analyseEquationTraverser(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut r#ref: Ref,
    mut kind: Kind,
    mut graph: Graph,
) -> Result<(metamodelica::Ref<SCode::Equation>, Graph)> {
    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let mut graph: Graph = graph;
    (eq, graph) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_FOR { index: iter_name, .. } => {
            graph = addIterators(list![metamodelica::Ref::new(Absyn::ForIterator { name: iter_name.clone(), guardExp: None, range: None })], r#ref.clone(), kind, graph)?;
            SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new({ let __pe_b2 = r#ref; let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, FCore::Graph) -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, .. } => {
            graph = analyseCref(cref1.clone(), r#ref.clone(), kind, graph)?;
            SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new({ let __pe_b2 = r#ref; let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, FCore::Graph) -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?
        },
        _ => {
            SCodeUtil::getEquationInfo(&eq);
            SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new({ let __pe_b2 = r#ref; let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, FCore::Graph) -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, graph))
}

fn traverseExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut graph: Graph,
    mut r#ref: Ref,
    mut kind: Kind,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Graph)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut graph: Graph = graph;
    (exp, graph) = AbsynUtil::traverseExpBidir(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = r#ref;
            let __pe_b2 = kind;
            move |__pe_a0, __pe_a3| analyseExpTraverserEnter(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        FCore::Graph,
                    ) -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)>
                    + 'static,
            >),
        (std::sync::Arc::new(fnptr!(
            analyseExpTraverserExit,
            metamodelica::Ref<Absyn::Exp>,
            FCore::Graph
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        FCore::Graph,
                    ) -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)>
                    + 'static,
            >),
        graph,
    )?;
    Ok((exp, graph))
}

fn analyseAlgorithm(
    mut inAlgorithm: &metamodelica::Ref<SCode::AlgorithmSection>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let __arc1 = &(*inAlgorithm);
    let SCode::ALGORITHM { statements: __pa0 } = &**__arc1;
    stmts = metamodelica::Own::own(__pa0);
    outGraph = List::fold2(&stmts, &analyseStatement, inParentRef, inKind, inGraph)?;
    Ok(outGraph)
}

fn analyseStatement(
    mut inStatement: metamodelica::Ref<SCode::Statement>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    (_, outGraph) = SCodeUtil::mapFoldStatements(
        inStatement,
        (std::sync::Arc::new({
            let __pe_b1 = inParentRef;
            let __pe_b2 = inKind;
            move |__pe_a0, __pe_a3| analyseStatementTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SCode::Statement>,
                        FCore::Graph,
                    ) -> Result<(metamodelica::Ref<SCode::Statement>, FCore::Graph)>
                    + 'static,
            >),
        inGraph,
    )?;
    Ok(outGraph)
}

fn analyseStatementTraverser(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut r#ref: Ref,
    mut kind: Kind,
    mut graph: Graph,
) -> Result<(metamodelica::Ref<SCode::Statement>, Graph)> {
    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let mut graph: Graph = graph;
    (stmt, graph) = (match &*stmt {
        SCode::Statement::ALG_FOR {
            index: __stmt_index, ..
        } => {
            graph = addIterators(
                list![metamodelica::Ref::new(Absyn::ForIterator {
                    name: __stmt_index.clone(),
                    guardExp: None,
                    range: None
                })],
                r#ref.clone(),
                kind,
                graph,
            )?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(
                stmt.clone(),
                (std::sync::Arc::new({
                    let __pe_b2 = r#ref;
                    let __pe_b3 = kind;
                    move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                FCore::Graph,
                            )
                                -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)>
                            + 'static,
                    >),
                graph,
            )?;
            (stmt, graph)
        }
        SCode::Statement::ALG_PARFOR {
            index: __stmt_index, ..
        } => {
            graph = addIterators(
                list![metamodelica::Ref::new(Absyn::ForIterator {
                    name: __stmt_index.clone(),
                    guardExp: None,
                    range: None
                })],
                r#ref.clone(),
                kind,
                graph,
            )?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(
                stmt.clone(),
                (std::sync::Arc::new({
                    let __pe_b2 = r#ref;
                    let __pe_b3 = kind;
                    move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                FCore::Graph,
                            )
                                -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)>
                            + 'static,
                    >),
                graph,
            )?;
            (stmt, graph)
        }
        _ => {
            SCodeUtil::getStatementInfo(&stmt)?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(
                stmt.clone(),
                (std::sync::Arc::new({
                    let __pe_b2 = r#ref;
                    let __pe_b3 = kind;
                    move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                FCore::Graph,
                            )
                                -> Result<(metamodelica::Ref<Absyn::Exp>, FCore::Graph)>
                            + 'static,
                    >),
                graph,
            )?;
            (stmt, graph)
        }
    });
    Ok((stmt, graph))
}

pub(crate) fn addIterators(
    mut inIterators: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut nr: Ref;
            nr = FNode::child(inParentRef.clone(), arcstr::literal!(FNode::forNodeName))?;
            FNode::addIteratorsToRef(nr.clone(), inIterators.clone())?;
            g = addIterators_helper(inIterators.clone(), nr.clone(), inKind, g.clone())?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut n: Node;
            let mut nr: Ref;
            (g, n) = FGraph::node(
                g.clone(),
                arcstr::literal!(FNode::forNodeName),
                list![inParentRef.clone()],
                metamodelica::Ref::new(FCore::Data::FS {
                    fis: inIterators.clone(),
                }),
            );
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(
                inParentRef.clone(),
                &(arcstr::literal!(FNode::forNodeName)),
                nr.clone(),
                false,
            )?;
            g = addIterators_helper(inIterators.clone(), nr.clone(), inKind, g.clone())?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

pub(crate) fn addIterators_helper(
    mut inIterators: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inIterators, inGraph)) {
            (Deref @ metamodelica::ListNode::Nil, g) => {
                return Ok(g.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: i @ Deref @ Absyn::ForIterator { name, .. }, tail: rest }, g) => {
                let mut n: Node;
                let mut nr: Ref;
                let mut g = (*g).clone();
                (g, n) = FGraph::node(g.clone(), name.clone(), list![inParentRef.clone()], metamodelica::Ref::new(FCore::Data::FI { fi: i.clone() }));
                nr = FNode::toRef(n);
                FNode::addChildRef(inParentRef.clone(), metamodelica::AsArg::as_arg(&name), nr, false)?;
                { (inIterators, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn addMatchScope(
    mut inMatchExp: metamodelica::Ref<Absyn::Exp>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut n: Node;
    let mut nr: Ref;
    let mut local_decls: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut g: Graph;
    (g, n) = FGraph::node(
        inGraph,
        arcstr::literal!(FNode::matchNodeName),
        list![inParentRef.clone()],
        metamodelica::Ref::new(FCore::Data::MS { e: inMatchExp.clone() }),
    );
    nr = FNode::toRef(n);
    FNode::addChildRef(
        inParentRef,
        &(arcstr::literal!(FNode::matchNodeName)),
        nr.clone(),
        false,
    )?;
    let __pa0 = ::match_deref::match_deref! { match &(inMatchExp) {
        Deref @ Absyn::Exp::MATCHEXP { localDecls: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    local_decls = metamodelica::Own::own(__pa0);
    outGraph = addMatchScope_helper(local_decls, nr, inKind, g)?;
    Ok(outGraph)
}

pub(crate) fn addMatchScope_helper(
    mut inElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inParentRef: Ref,
    mut inKind: Kind,
    mut inGraph: Graph,
) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElements, inGraph)) {
            (Deref @ metamodelica::ListNode::Nil, g) => {
                return Ok(g.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element }, tail: rest }, g) => {
                let mut el: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut g = (*g).clone();
                el = AbsynToSCode::translateElement(metamodelica::AsArg::as_arg(&element), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                g = List::fold2(&el, &mkElementNode, inParentRef.clone(), inKind, g.clone())?;
                { (inElements, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, g) => {
                let mut g = (*g).clone();
                { (inElements, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef, inKind, g.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn mkRefNode(
    mut inName: Name,
    mut inTargetScope: Scope,
    mut inParentRef: Ref,
    mut inGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut n: Node;
            let mut rn: Ref;
            (g, n) = FGraph::node(
                g,
                inName.clone(),
                list![inParentRef.clone()],
                metamodelica::Ref::new(FCore::Data::REF { target: inTargetScope }),
            );
            rn = FNode::toRef(n);
            FNode::addChildRef(inParentRef, &inName, rn, false)?;
            g
        }
    });
    Ok(outGraph)
}
