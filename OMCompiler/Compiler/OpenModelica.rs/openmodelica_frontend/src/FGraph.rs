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

use crate::FGraphBuildEnv;
use crate::FNode;
use crate::InnerOuter;
use crate::Mod;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::FCore::RefTree;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

// public imports
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

pub type Scope = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type Top = FCore::Top;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Status = FCore::Status;

pub const fn emptyGraph() -> FCore::Graph {
    FCore::Graph::EG {
        name: literal!("empty"),
    }
}

pub(crate) fn top(mut inGraph: &Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (match inGraph.clone() {
        FCore::Graph::G { .. } => var_field!(inGraph.top, FCore::Graph::G).node.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outRef)
}

pub(crate) fn extra(mut inGraph: &Graph) -> Result<Extra> {
    let mut outExtra: Extra;
    outExtra = (match inGraph.clone() {
        FCore::Graph::G { .. } => var_field!(inGraph.top, FCore::Graph::G).extra.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outExtra)
}

pub(crate) fn currentScope(mut inGraph: &Graph) -> Scope {
    let mut outScope: Scope;
    outScope = (match inGraph.clone() {
        FCore::Graph::G {
            scope: ref __esc_outScope,
            ..
        } => {
            outScope = __esc_outScope.clone();
            outScope.clone()
        }
        FCore::Graph::EG { name: _ } => metamodelica::nil(),
    });
    outScope
}

pub fn lastScopeRef(mut inGraph: &Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (currentScope(inGraph)).head().cloned()?;
    Ok(outRef)
}

pub(crate) fn setLastScopeRef(mut inRef: Ref, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph = inGraph;
    outGraph = (match outGraph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 =
                metamodelica::cons(inRef, (var_field!(outGraph.scope, FCore::Graph::G)).rest()?);
            if let FCore::Graph::G { scope, .. } = &mut outGraph {
                *scope = __owned_variant_scope_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G");
            }
            outGraph
        }
        _ => outGraph,
    });
    Ok(outGraph)
}

pub(crate) fn stripLastScopeRef(mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    let mut t: Top;
    let mut s: Scope;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inGraph) {
        FCore::Graph::G { top: __pa0, scope: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    t = metamodelica::Own::own(__pa0);
    outRef = metamodelica::Own::own(__pa1);
    s = metamodelica::Own::own(__pa2);
    outGraph = FCore::Graph::G { top: t, scope: s };
    Ok((outGraph, outRef))
}

pub(crate) fn topScope(mut inGraph: &Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        FCore::Graph::G { .. } => FCore::Graph::G {
            top: var_field!(inGraph.top, FCore::Graph::G).clone(),
            scope: list![var_field!(inGraph.top, FCore::Graph::G).node.clone()],
        },
        _ => return Err("match: no arm matched"),
    });
    Ok(outGraph)
}

pub fn empty() -> Graph {
    let mut outGraph: Graph;
    outGraph = emptyGraph().clone();
    outGraph
}

pub fn new(mut inGraphName: Name, mut inPath: metamodelica::Ref<Absyn::Path>) -> Graph {
    let mut outGraph: Graph;
    let mut n: Node;
    let mut s: Scope;
    let mut nr: Ref;
    let mut id: Id;
    let mut top: Top;
    id = System::tmpTickIndex(Global::fgraph_nextId.clone());
    n = FNode::new(
        arcstr::literal!(FNode::topNodeName),
        id,
        metamodelica::nil(),
        openmodelica_frontend_dump::FCore::Data::interned_TOP(),
    );
    nr = FNode::toRef(n);
    s = list![nr.clone()];
    top = FCore::Top {
        name: inGraphName,
        node: nr,
        extra: FCore::Extra { topModel: inPath },
    };
    outGraph = FCore::Graph::G { top: top, scope: s };
    outGraph
}

pub(crate) fn node(mut inGraph: Graph, mut inName: Name, mut inParents: Parents, mut inData: Data) -> (Graph, Node) {
    let mut outGraph: Graph;
    let mut outNode: Node;
    (outGraph, outNode) = (match inGraph {
        mut g => {
            let mut i: i32;
            let mut n: Node;
            i = System::tmpTickIndex(Global::fgraph_nextId.clone());
            n = FNode::new(inName, i, inParents, inData);
            (g, n)
        }
    });
    (outGraph, outNode)
}

pub(crate) fn clone(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        FCore::Graph::G {
            top: mut t,
            scope: ref s,
        } => {
            let mut g: Graph;
            let mut nt: Ref;
            let mut s = s.clone();
            nt = FNode::toRef(FNode::fromRef(t.node.clone()));
            (g, nt) = FNode::copyRef(nt, inGraph)?;
            s = List::map1r(s.clone(), &FNode::lookupRefFromRef, nt.clone())?;
            t = FCore::Top {
                name: t.name.clone(),
                node: nt,
                extra: t.extra.clone(),
            };
            g = FCore::Graph::G {
                top: t.clone(),
                scope: s.clone(),
            };
            g
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outGraph)
}

pub fn updateComp(
    mut inGraph: Graph,
    mut inVar: metamodelica::Ref<DAE::Var>,
    mut instStatus: &FCore::Status,
    mut inTargetGraph: &Graph,
) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inVar);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id;
                    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
                    let mut c: Children;
                    let mut e: metamodelica::Ref<SCode::Element>;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut k: Kind;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    r = FNode::child(pr.clone(), n.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                        Deref @ FCore::Node { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: Deref @ FCore::Data::CO { e: __pa4, r#mod: __pa5, kind: __pa6, status: _ } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    n = metamodelica::Own::own(__pa0);
                    id = metamodelica::Own::own(__pa1);
                    p = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    e = metamodelica::Own::own(__pa4);
                    m = metamodelica::Own::own(__pa5);
                    k = metamodelica::Own::own(__pa6);
                    r = FNode::updateRef(r.clone(), metamodelica::Ref::new(FCore::Node { name: n.clone(), id: id, parents: p.clone(), children: c.clone(), data: metamodelica::Ref::new(FCore::Data::CO { e: e.clone(), r#mod: m.clone(), kind: k, status: instStatus.clone() }) }));
                    r = updateSourceTargetScope(r.clone(), currentScope(inTargetGraph))?;
                    r = updateInstance(r.clone(), v.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    let true = (FNode::isImplicitRefName(pr.clone())) else { return Err("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateComp(g.clone(), v.clone(), instStatus, inTargetGraph);
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

pub(crate) fn updateSourceTargetScope(mut inRef: Ref, mut inTargetScope: Scope) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else {
                return Err("nomatch");
            };
            r = FNode::refRef(r.clone())?;
            r = FNode::updateRef(
                r.clone(),
                FNode::setData(
                    &(FNode::fromRef(r.clone())),
                    metamodelica::Ref::new(FCore::Data::REF {
                        target: inTargetScope.clone(),
                    }),
                ),
            );
            Ok(inRef.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else {
                return Err("nomatch");
            };
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "FNode.updateSourceTargetScope: node does not yet have a reference child: "
                ));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!(" target scope: "));
                __mm_s.push_str(&*FNode::scopeStr(inTargetScope.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(inRef.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRef)
}

pub(crate) fn updateInstance(mut inRef: Ref, mut inVar: metamodelica::Ref<DAE::Var>) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else {
                return Err("nomatch");
            };
            r = FNode::refInstance(r.clone())?;
            r = FNode::updateRef(
                r.clone(),
                FNode::setData(
                    &(FNode::fromRef(r.clone())),
                    metamodelica::Ref::new(FCore::Data::IT { i: inVar.clone() }),
                ),
            );
            Ok(inRef.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FGraph.updateInstance failed for node: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(inRef.clone())))?);
                __mm_s.push_str(&*literal!(" variable:"));
                __mm_s.push_str(&*TypesDump::printVarStr(&inVar));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRef)
}

fn updateVarAndMod(
    mut inGraph: Graph,
    mut inVar: metamodelica::Ref<DAE::Var>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut instStatus: &FCore::Status,
    mut inTargetGraph: &Graph,
) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inVar);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id;
                    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
                    let mut c: Children;
                    let mut e: metamodelica::Ref<SCode::Element>;
                    let mut k: Kind;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    r = FNode::child(pr.clone(), n.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                        Deref @ FCore::Node { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: Deref @ FCore::Data::CO { e: __pa4, r#mod: _, kind: __pa5, status: _ } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    n = metamodelica::Own::own(__pa0);
                    id = metamodelica::Own::own(__pa1);
                    p = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    e = metamodelica::Own::own(__pa4);
                    k = metamodelica::Own::own(__pa5);
                    r = FNode::updateRef(r.clone(), metamodelica::Ref::new(FCore::Node { name: n.clone(), id: id, parents: p.clone(), children: c.clone(), data: metamodelica::Ref::new(FCore::Data::CO { e: e.clone(), r#mod: inMod.clone(), kind: k, status: instStatus.clone() }) }));
                    r = updateSourceTargetScope(r.clone(), currentScope(inTargetGraph))?;
                    r = updateInstance(r.clone(), v.clone())?;
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    let true = (FNode::isImplicitRefName(pr.clone())) else { return Err("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateVarAndMod(g.clone(), v.clone(), inMod, instStatus, inTargetGraph);
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

pub(crate) fn updateClass(
    mut inGraph: Graph,
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inPrefix: &DAE::Prefix,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut instStatus: &FCore::Status,
    mut inTargetGraph: &Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph, inElement);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, e @ Deref @ SCode::Element::CLASS { name: n, .. }) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id;
                    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
                    let mut c: Children;
                    let mut k: Kind;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    r = FNode::child(pr.clone(), n.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                        Deref @ FCore::Node { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: Deref @ FCore::Data::CL { e: _, pre: _, r#mod: _, kind: __pa4, status: _ } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    n = metamodelica::Own::own(__pa0);
                    id = metamodelica::Own::own(__pa1);
                    p = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    k = metamodelica::Own::own(__pa4);
                    r = FNode::updateRef(r.clone(), metamodelica::Ref::new(FCore::Node { name: n.clone(), id: id, parents: p.clone(), children: c.clone(), data: metamodelica::Ref::new(FCore::Data::CL { e: e.clone(), pre: inPrefix.clone(), r#mod: inMod.clone(), kind: k, status: instStatus.clone() }) }));
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, e) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    let true = (FNode::isImplicitRefName(pr.clone())) else { return Err("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateClass(g.clone(), e.clone(), inPrefix, inMod, instStatus, inTargetGraph)?;
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

pub(crate) fn updateClassElement(
    mut inRef: Ref,
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut instStatus: FCore::Status,
    mut inTargetGraph: &Graph,
) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (::match_deref::match_deref! { match &(inElement) {
        e @ Deref @ SCode::Element::CLASS { name: n, .. } => {
            let mut r = inRef;
            let mut id: Id;
            let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
            let mut c: Children;
            let mut k: Kind;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                Deref @ FCore::Node { name: _, id: __pa0, parents: __pa1, children: __pa2, data: Deref @ FCore::Data::CL { e: _, pre: _, r#mod: _, kind: __pa3, status: _ } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            id = metamodelica::Own::own(__pa0);
            p = metamodelica::Own::own(__pa1);
            c = metamodelica::Own::own(__pa2);
            k = metamodelica::Own::own(__pa3);
            r = FNode::updateRef(r, metamodelica::Ref::new(FCore::Node { name: n.clone(), id: id, parents: p, children: c, data: metamodelica::Ref::new(FCore::Data::CL { e: e.clone(), pre: inPrefix, r#mod: inMod, kind: k, status: instStatus }) }));
            r
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outRef)
}

pub(crate) fn addForIterator(
    mut inGraph: Graph,
    mut name: ArcStr,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut binding: metamodelica::Ref<DAE::Binding>,
    mut variability: SCode::Variability,
    mut constOfForIteratorRange: Option<DAE::Const>,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            let mut c: metamodelica::Ref<SCode::Element>;
            let mut v: metamodelica::Ref<DAE::Var>;
            c = metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: name.clone(),
                prefixes: SCode::defaultPrefixes.clone(),
                attributes: SCode::Attributes {
                    arrayDims: metamodelica::nil(),
                    connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                    parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                    variability: openmodelica_frontend_types::SCode::Variability::CONST,
                    direction: openmodelica_ast::Absyn::Direction::BIDIR,
                    isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                },
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                    arrayDim: None,
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone(),
            });
            v = metamodelica::Ref::new(DAE::Var {
                name: name,
                attributes: metamodelica::Ref::new(DAE::Attributes {
                    connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
                    parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                    variability: variability,
                    direction: openmodelica_ast::Absyn::Direction::BIDIR,
                    innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
                    visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC,
                }),
                ty: ty,
                binding: binding,
                bind_from_outside: false,
                constOfForIteratorRange: constOfForIteratorRange,
            });
            r = lastScopeRef(&g)?;
            g = FGraphBuildEnv::mkCompNode(c, r, openmodelica_frontend_dump::FCore::Kind::BUILTIN, g)?;
            g = updateVarAndMod(
                g,
                v,
                &(openmodelica_frontend_types::DAE::Mod::interned_NOMOD()),
                &(openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED),
                &(empty()),
            );
            g
        }
    });
    Ok(outGraph)
}

pub(crate) fn printGraphPathStr(mut inGraph: &Graph) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Graph::G { scope: s @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. } => {
                    let mut r#str: ArcStr;
                    let mut s = (*s).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(s.clone().reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    s = metamodelica::Own::own(__pa0);
                    r#str = stringDelimitList(List::map(s.clone(), &fnptr!(FNode::refName, Mutable::Mutable<metamodelica::Ref<FCore::Node>>))?, literal!("."));
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("<global scope>"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub(crate) fn openNewScope(
    mut inGraph: Graph,
    mut encapsulatedPrefix: SCode::Encapsulated,
    mut inName: Option<ArcStr>,
    mut inScopeType: Option<FCore::ScopeType>,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, Some(mut n)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut no: Node;
            let mut r: Ref;
            let mut p: Ref;
            p = lastScopeRef(&(g.clone()))?;
            (g, no) = node(
                g.clone(),
                n.clone(),
                list![p.clone()],
                metamodelica::Ref::new(FCore::Data::ND {
                    scopeType: inScopeType.clone(),
                }),
            );
            r = FNode::toRef(no.clone());
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FGraph.openNewScope: failed to open new scope in scope: "));
                __mm_s.push_str(&*getGraphNameStr(&inGraph));
                __mm_s.push_str(&*literal!(" name: "));
                __mm_s.push_str(&*Util::getOptionOrDefault(inName.clone(), literal!("")));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

pub fn openScope(
    mut inGraph: Graph,
    mut encapsulatedPrefix: SCode::Encapsulated,
    mut inName: Name,
    mut inScopeType: Option<FCore::ScopeType>,
) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut p: Ref;
    p = lastScopeRef(&inGraph)?;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, mut n) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            r = FNode::child(p.clone(), n.clone())?;
            ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: _ }, .. } => (),
                _ => return Err("pattern mismatch"),
            } };
            FNode::addChildRef(p.clone(), &(n.clone()), r.clone(), false)?;
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, mut n) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            r = FNode::child(p.clone(), n.clone())?;
            r = FNode::copyRefNoUpdate(r.clone())?;
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, mut n) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut no: Node;
            let mut r: Ref;
            (g, no) = node(
                g.clone(),
                n.clone(),
                list![p.clone()],
                metamodelica::Ref::new(FCore::Data::ND {
                    scopeType: inScopeType.clone(),
                }),
            );
            r = FNode::toRef(no.clone());
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FGraph.openScope: failed to open new scope in scope: "));
                __mm_s.push_str(&*getGraphNameStr(&inGraph));
                __mm_s.push_str(&*literal!(" name: "));
                __mm_s.push_str(&*inName);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outGraph)
}

pub(crate) fn inForLoopScope(mut inGraph: &Graph) -> bool {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut name: ArcStr;
            name = FNode::refName((currentScope(inGraph)).head().cloned()?);
            let true = (stringEq(&name, &arcstr::literal!(FCore::forScopeName))) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res
}

pub(crate) fn inForOrParforIterLoopScope(mut inGraph: &Graph) -> bool {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut name: ArcStr;
            name = FNode::refName((currentScope(inGraph)).head().cloned()?);
            let true = (stringEq(&name, &arcstr::literal!(FCore::forIterScopeName))
                || stringEq(&name, &arcstr::literal!(FCore::parForIterScopeName)))
            else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res
}

pub fn getScopePath(mut inGraph: &Graph) -> Result<Option<metamodelica::Ref<Absyn::Path>>> {
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    outPath = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r: Ref;
            let __pa0 = ::match_deref::match_deref! { match &(currentScope(inGraph)) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            let true = (FNode::isRefTop(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(None)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = getGraphName(inGraph)?;
            Ok(Some(p.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPath)
}

pub(crate) fn getGraphNameStr(mut inGraph: &Graph) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(AbsynUtil::pathString(
                getGraphName(inGraph)?,
                literal!("."),
                true,
                false,
            )?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(literal!("."))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub fn getGraphName(mut inGraph: &Graph) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut p: metamodelica::Ref<Absyn::Path>;
    let mut s: Scope;
    let mut r: Ref;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(currentScope(inGraph)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    r = metamodelica::Own::own(__pa0);
    s = metamodelica::Own::own(__pa1);
    p = AbsynUtil::makeIdentPathFromString(FNode::refName(r));
    for mut r in &*s {
        let mut r = r.clone();
        p = metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: FNode::refName(r),
            path: p,
        });
    }
    let __pa2 = ::match_deref::match_deref! { match &(p) {
        Deref @ Absyn::Path::QUALIFIED { name: _, path: __pa2 } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outPath = metamodelica::Own::own(__pa2);
    Ok(outPath)
}

pub(crate) fn getGraphNameNoImplicitScopes(mut inGraph: &Graph) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut s: Scope;
    let __pa0 = ::match_deref::match_deref! { match &(currentScope(inGraph).reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    s = metamodelica::Own::own(__pa0);
    outPath = AbsynUtil::stringListPath(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut r#str in ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut n in (s).into_iter().cloned() {
                    let __x = FNode::refName(n.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
            .into_iter()
            .cloned()
            {
                if !(stringGet(&r#str, 1)? != 36) {
                    continue;
                }
                let __x = r#str.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )?;
    Ok(outPath)
}

pub(crate) fn pushScopeRef(mut graph: Graph, mut inRef: Ref) -> Result<Graph> {
    let mut graph: Graph = graph;
    let () = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = metamodelica::cons(inRef, var_field!(graph.scope, FCore::Graph::G).clone());
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G");
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(graph)
}

pub(crate) fn pushScope(mut graph: Graph, mut inScope: Scope) -> Result<Graph> {
    let mut graph: Graph = graph;
    let () = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = listAppend(inScope, var_field!(graph.scope, FCore::Graph::G).clone());
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G");
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(graph)
}

pub(crate) fn setScope(mut graph: Graph, mut inScope: Scope) -> Result<Graph> {
    let mut graph: Graph = graph;
    let () = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = inScope;
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G");
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(graph)
}

pub fn restrictionToScopeType(mut inRestriction: &SCode::Restriction) -> Option<FCore::ScopeType> {
    let mut outType: Option<FCore::ScopeType>;
    outType = (match inRestriction.clone() {
        SCode::Restriction::R_FUNCTION {
            functionRestriction: SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. },
        } => Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE),
        SCode::Restriction::R_FUNCTION {
            functionRestriction: SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. },
        } => Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE),
        SCode::Restriction::R_FUNCTION { functionRestriction: _ } => {
            Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE)
        }
        _ => Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE),
    });
    outType
}

pub(crate) fn scopeTypeToRestriction(mut inScopeType: FCore::ScopeType) -> SCode::Restriction {
    let mut outRestriction: SCode::Restriction;
    outRestriction = (match inScopeType {
        FCore::ScopeType::PARALLEL_SCOPE { .. } => SCode::Restriction::R_FUNCTION {
            functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_PARALLEL_FUNCTION,
        },
        FCore::ScopeType::FUNCTION_SCOPE { .. } => SCode::Restriction::R_FUNCTION {
            functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                purity: openmodelica_ast::Absyn::FunctionPurity::NO_PURITY,
            },
        },
        _ => openmodelica_frontend_types::SCode::Restriction::R_CLASS,
    });
    outRestriction
}

pub(crate) fn isTopScope(mut graph: &Graph) -> bool {
    let mut isTop: bool;
    isTop = 'mc: {
        let __mc_input = graph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (FNode::isRefTop(lastScopeRef(graph)?)) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isTop
}

pub(crate) fn crefStripGraphScopePrefix(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnv: &Graph,
    mut stripPartial: bool,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = stripPartial;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(inCref.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut env_path: metamodelica::Ref<Absyn::Path>;
            let mut cref1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut cref2: metamodelica::Ref<Absyn::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(getScopePath(inEnv)?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            env_path = metamodelica::Own::own(__pa0);
            cref1 = AbsynUtil::unqualifyCref(inCref.clone());
            env_path = AbsynUtil::makeNotFullyQualified(env_path.clone());
            cref2 = crefStripGraphScopePrefix2(&cref1, env_path.clone(), stripPartial)?;
            let false = (AbsynUtil::crefEqual(&cref1, &cref2)?) else {
                return Err("pattern mismatch");
            };
            Ok(cref2.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inCref.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn crefStripGraphScopePrefix2(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnvPath: metamodelica::Ref<Absyn::Path>,
    mut stripPartial: bool,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = (&**inCref, inEnvPath, stripPartial);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: cref }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: env_path }, _) => {
                    let true = (stringEqual(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(crefStripGraphScopePrefix2(metamodelica::AsArg::as_arg(&cref), env_path.clone(), stripPartial)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: cref }, Deref @ Absyn::Path::IDENT { name: id2 }, _) => {
                    let true = (stringEqual(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::ListNode::Nil, .. }, env_path, true) => {
                    let false = (stringEqual(&id1, &(AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&env_path))))) else { return Err("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCref)
}

pub(crate) fn pathStripGraphScopePrefix(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: &Graph,
    mut stripPartial: bool,
) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = stripPartial;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(inPath.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut env_path: metamodelica::Ref<Absyn::Path>;
            let mut path1: metamodelica::Ref<Absyn::Path>;
            let mut path2: metamodelica::Ref<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(getScopePath(inEnv)?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            env_path = metamodelica::Own::own(__pa0);
            path1 = AbsynUtil::makeNotFullyQualified(inPath.clone());
            env_path = AbsynUtil::makeNotFullyQualified(env_path.clone());
            path2 = pathStripGraphScopePrefix2(path1.clone(), env_path.clone(), stripPartial)?;
            let false = (AbsynUtil::pathEqual(&path1, &path2)) else {
                return Err("pattern mismatch");
            };
            Ok(path2.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inPath.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outPath
}

fn pathStripGraphScopePrefix2(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnvPath: metamodelica::Ref<Absyn::Path>,
    mut stripPartial: bool,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPath.clone(), inEnvPath, stripPartial)) {
            (Deref @ Absyn::Path::QUALIFIED { name: id1, path }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: env_path }, _) if (stringEqual(&id1, &id2)) => {
                { (inPath, inEnvPath, stripPartial) = (path.clone(), env_path.clone(), stripPartial); continue '__tco; }
            },
            (Deref @ Absyn::Path::QUALIFIED { name: id1, path }, Deref @ Absyn::Path::IDENT { name: id2 }, _) if (stringEqual(&id1, &id2)) => {
                return Ok(path.clone())
            },
            (Deref @ Absyn::Path::QUALIFIED { name: id1, .. }, env_path, true) if (!(stringEqual(&id1, &(AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&env_path)))))) => {
                return Ok(inPath)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn mkComponentNode(
    mut inGraph: Graph,
    mut inVar: metamodelica::Ref<DAE::Var>,
    mut inVarEl: metamodelica::Ref<SCode::Element>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut instStatus: Status,
    mut inCompGraph: Graph,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph, inVar, inVarEl, inMod, instStatus, inCompGraph);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Var { name: n, .. }, c, _, _, _) => {
                    let false = (stringEq(&n, &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&c))?))) else { return Err("pattern mismatch") };
                    Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.mkComponentNode: The component name: ")); __mm_s.push_str(&*SCodeUtil::elementName(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!(" is not the same as its DAE.TYPES_VAR: ")); __mm_s.push_str(&*n); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }, c, m, i, cg) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    let true = (stringEq(&n, &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&c))?))) else { return Err("pattern mismatch") };
                    r = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    g = FGraphBuildEnv::mkCompNode(c.clone(), r.clone(), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone())?;
                    g = updateVarAndMod(g.clone(), v.clone(), metamodelica::AsArg::as_arg(&m), metamodelica::AsArg::as_arg(&i), metamodelica::AsArg::as_arg(&cg));
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

pub(crate) fn mkClassNode(
    mut inGraph: Graph,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut checkDuplicate: bool,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph, &*inClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ SCode::Element::CLASS { name: n, .. }) => {
                    let mut r: Ref;
                    r = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    r = FNode::child(r.clone(), n.clone())?;
                    ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                        Deref @ FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: _ }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(g.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ SCode::Element::CLASS { .. }) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    r = lastScopeRef(metamodelica::AsArg::as_arg(&g))?;
                    g = FGraphBuildEnv::mkClassNode(inClass.clone(), inPrefix.clone(), inMod.clone(), r.clone(), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone(), checkDuplicate)?;
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

pub(crate) fn mkTypeNode(
    mut inGraph: Graph,
    mut inName: Name,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = lastScopeRef(&g)?;
            g = FGraphBuildEnv::mkTypeNode(list![inType], r, inName, g)?;
            g
        }
    });
    Ok(outGraph)
}

pub(crate) fn mkImportNode(mut inGraph: Graph, mut inImport: metamodelica::Ref<SCode::Element>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = lastScopeRef(&g)?;
            g = FGraphBuildEnv::mkElementNode(inImport, r, openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g)?;
            g
        }
    });
    Ok(outGraph)
}

pub(crate) fn mkDefunitNode(mut inGraph: Graph, mut inDu: metamodelica::Ref<SCode::Element>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = lastScopeRef(&g)?;
            g = FGraphBuildEnv::mkElementNode(inDu, r, openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g)?;
            g
        }
    });
    Ok(outGraph)
}

pub(crate) fn classInfToScopeType(mut inState: &ClassInf::State) -> Option<FCore::ScopeType> {
    let mut outType: Option<FCore::ScopeType>;
    outType = (match inState.clone() {
        ClassInf::State::FUNCTION { .. } => Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE),
        _ => Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE),
    });
    outType
}

pub(crate) fn isEmpty(mut inGraph: &Graph) -> bool {
    let mut b: bool;
    b = (match inGraph.clone() {
        FCore::Graph::EG { name: _ } => true,
        _ => false,
    });
    b
}

pub fn isNotEmpty(mut inGraph: &Graph) -> bool {
    let mut b: bool;
    b = !(isEmpty(inGraph));
    b
}

pub(crate) fn isEmptyScope(mut graph: &Graph) -> bool {
    let mut isEmpty: bool;
    match '__try0: {
        isEmpty = FCore::RefTree::isEmpty(
            &(FNode::children(&(FNode::fromRef(unwrap_break_err!(lastScopeRef(graph), '__try0))))),
        );
        Ok::<_, &'static str>((isEmpty.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isEmpty = __try0_o0;
        }
        Err(_) => {
            isEmpty = true;
        }
    }
    isEmpty
}

pub(crate) fn printGraphStr(mut inGraph: &Graph) -> ArcStr {
    let mut s: ArcStr;
    s = literal!("NOT IMPLEMENTED YET");
    s
}

pub(crate) fn inFunctionScope(mut inGraph: &Graph) -> bool {
    let mut inFunction: bool;
    inFunction = (match inGraph.clone() {
        FCore::Graph::G { scope: ref s, .. }
            if (checkScopeType(
                metamodelica::AsArg::as_arg(&s),
                Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE),
            ) || checkScopeType(
                metamodelica::AsArg::as_arg(&s),
                Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE),
            )) =>
        {
            true
        }
        _ => false,
    });
    inFunction
}

pub(crate) fn getScopeName(mut inGraph: &Graph) -> Result<Name> {
    let mut name: Name;
    name = (match inGraph.clone() {
        _ => {
            let mut r: Ref;
            r = lastScopeRef(inGraph)?;
            let false = (FNode::isRefTop(r.clone())) else {
                return Err("pattern mismatch");
            };
            name = FNode::refName(r);
            name
        }
    });
    Ok(name)
}

pub(crate) fn checkScopeType(mut inScope: &Scope, mut inScopeType: Option<FCore::ScopeType>) -> bool {
    let mut yes: bool;
    yes = 'mc: {
        let __mc_input = &**inScope;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let mut restr: SCode::Restriction;
                    let true = (FNode::isRefClass(r.clone())) else { return Err("pattern mismatch") };
                    restr = SCodeUtil::getClassRestriction(&(FNode::getElement(&(FNode::fromRef(r.clone())))?))?;
                    let true = (restrictionToScopeType(&restr) == inScopeType.clone()) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let mut st: Option<FCore::ScopeType>;
                    let __pa0 = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                        Deref @ FCore::Node { data: Deref @ FCore::Data::ND { scopeType: __pa0 }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    st = metamodelica::Own::own(__pa0);
                    let true = (st.clone() == inScopeType.clone()) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(checkScopeType(metamodelica::AsArg::as_arg(&rest), inScopeType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    yes
}

pub(crate) fn lastScopeRestriction(mut inGraph: Graph) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    let mut s: Scope;
    let FCore::G { scope: __pa0, .. } = (inGraph) else {
        return Err("pattern mismatch");
    };
    s = metamodelica::Own::own(__pa0);
    outRestriction = getScopeRestriction(&s)?;
    Ok(outRestriction)
}

pub(crate) fn getScopeRestriction(mut inScope: &Scope) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    outRestriction = 'mc: {
        let __mc_input = &**inScope;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    if !((FNode::isRefClass(r.clone()))) { return Err("guard") }
                    Ok(SCodeUtil::getClassRestriction(&(FNode::getElement(&(FNode::fromRef(r.clone())))?))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let mut st: FCore::ScopeType;
                    let __pa0 = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                        Deref @ FCore::Node { data: Deref @ FCore::Data::ND { scopeType: Some(__pa0) }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    st = metamodelica::Own::own(__pa0);
                    Ok(scopeTypeToRestriction(st))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(getScopeRestriction(&((inScope).rest()?))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRestriction)
}

pub(crate) fn getGraphPathNoImplicitScope(mut inGraph: &Graph) -> Option<metamodelica::Ref<Absyn::Path>> {
    let mut outAbsynPathOption: Option<metamodelica::Ref<Absyn::Path>>;
    outAbsynPathOption = getGraphPathNoImplicitScope_dispatch(&(currentScope(inGraph)));
    outAbsynPathOption
}

fn getGraphPathNoImplicitScope_dispatch(mut inScope: &Scope) -> Option<metamodelica::Ref<Absyn::Path>> {
    let mut outAbsynPathOption: Option<metamodelica::Ref<Absyn::Path>>;
    let mut opath: Option<metamodelica::Ref<Absyn::Path>> = None;
    outAbsynPathOption = 'mc: {
        let __mc_input = &**inScope;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: rest } => {
                    if !((!(FNode::isRefTop(r#ref.clone())))) { return Err("guard") }
                    let mut id: Name;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut opath: Option<metamodelica::Ref<Absyn::Path>> = opath.clone();
                    id = FNode::refName(r#ref.clone());
                    if isImplicitScope(id.clone()) {
                        opath = getGraphPathNoImplicitScope_dispatch(metamodelica::AsArg::as_arg(&rest));
                    } else {
                        opath = getGraphPathNoImplicitScope_dispatch(metamodelica::AsArg::as_arg(&rest));
                        if (opath).is_some() {
                            let __pa0 = ::match_deref::match_deref! { match &(opath.clone()) {
                                        Some(__pa0) => __pa0.clone(),
                                        _ => return Err("pattern mismatch"),
                            } };
                            path = metamodelica::Own::own(__pa0);
                            path_1 = AbsynUtil::joinPaths(path.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }))?;
                            opath = Some(path_1.clone());
                        } else {
                            opath = Some(metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }));
                        }
                    }
                    Ok((opath.clone(), opath.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            opath = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outAbsynPathOption
}

pub(crate) fn isImplicitScope(mut inName: Name) -> bool {
    let mut isImplicit: bool;
    isImplicit = FCore::isImplicitScope(inName);
    isImplicit
}

pub(crate) fn joinScopePath(
    mut inGraph: &Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut opath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut envPath: metamodelica::Ref<Absyn::Path>;
    opath = getScopePath(inGraph)?;
    if (opath).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(opath) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        envPath = metamodelica::Own::own(__pa0);
        outPath = AbsynUtil::joinPaths(envPath, inPath)?;
    } else {
        outPath = inPath;
    }
    Ok(outPath)
}

pub(crate) fn splitGraphScope(mut inGraph: &Graph) -> Result<(Graph, Scope)> {
    let mut outRealGraph: Graph;
    let mut outForScope: Scope;
    (outRealGraph, outForScope) = splitGraphScope_dispatch(inGraph, &(metamodelica::nil()))?;
    Ok((outRealGraph, outForScope))
}

pub(crate) fn splitGraphScope_dispatch(mut inGraph: &Graph, mut inAcc: &Scope) -> Result<(Graph, Scope)> {
    let mut outRealGraph: Graph;
    let mut outForScope: Scope;
    (outRealGraph, outForScope) = (::match_deref::match_deref! { match &(inGraph) {
        FCore::Graph::EG { name: _ } => {
            (inGraph.clone(), inAcc.clone().reverse())
        },
        FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. } => {
            let mut g: Graph;
            let mut s: Scope;
            if FNode::isImplicitRefName(r.clone()) {
                (g, _) = stripLastScopeRef(inGraph.clone())?;
                (g, s) = splitGraphScope_dispatch(&g, &(metamodelica::cons(r.clone(), inAcc.clone())))?;
            } else {
                g = inGraph.clone();
                s = inAcc.clone().reverse();
            }
            (g, s)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outRealGraph, outForScope))
}

pub(crate) fn getVariablesFromGraphScope(mut inGraph: &Graph) -> Result<metamodelica::List<ArcStr>> {
    let mut variables: metamodelica::List<ArcStr>;
    variables = (::match_deref::match_deref! { match &(inGraph) {
        FCore::Graph::EG { name: _ } => {
            metamodelica::nil()
        },
        FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Nil, .. } => {
            metamodelica::nil()
        },
        FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. } => {
            let mut lst: metamodelica::List<ArcStr>;
            lst = List::map(FNode::filter(r.clone(), (std::sync::Arc::new(fnptr!(FNode::isRefComponent, Mutable::Mutable<metamodelica::Ref<FCore::Node>>)) as std::sync::Arc<dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool> + 'static>))?, &fnptr!(FNode::refName, Mutable::Mutable<metamodelica::Ref<FCore::Node>>))?;
            lst
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(variables)
}

pub(crate) fn removeComponentsFromScope(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut r: Ref;
    let mut n: Node;
    r = lastScopeRef(&inGraph)?;
    r = FNode::copyRefNoUpdate(r)?;
    n = FNode::fromRef(r.clone());
    n = FNode::setChildren(&n, FCore::RefTree::new());
    r = FNode::updateRef(r, n);
    (outGraph, _) = stripLastScopeRef(inGraph)?;
    outGraph = pushScopeRef(outGraph, r)?;
    Ok(outGraph)
}

pub(crate) fn cloneLastScopeRef(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut r: Ref;
    (outGraph, r) = stripLastScopeRef(inGraph)?;
    r = FNode::copyRefNoUpdate(r)?;
    outGraph = pushScopeRef(outGraph, r)?;
    Ok(outGraph)
}

pub(crate) fn updateScope(mut inGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        _ => inGraph,
    });
    outGraph
}

pub(crate) fn mkVersionNode(
    mut inSourceEnv: &Graph,
    mut inSourceName: Name,
    mut inPrefix: &DAE::Prefix,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inTargetClassEnv: Graph,
    mut inTargetClass: metamodelica::Ref<SCode::Element>,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
) -> Result<(
    Graph,
    metamodelica::Ref<SCode::Element>,
    metamodelica::List<InnerOuter::TopInstance>,
)> {
    let mut outVersionedTargetClassEnv: Graph;
    let mut outVersionedTargetClass: metamodelica::Ref<SCode::Element>;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    (outVersionedTargetClassEnv, outVersionedTargetClass, outIH) = 'mc: {
        let __mc_input = &*inIH;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut gclass: Graph;
                    let mut classRef: Ref;
                    let mut sourceRef: Ref;
                    let mut targetClassParentRef: Ref;
                    let mut crefPrefix: DAE::Prefix;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut targetClassName: Name;
                    let mut newTargetClassName: Name;
                    let mut ih: metamodelica::List<InnerOuter::TopInstance>;
                    c = inTargetClass.clone();
                    gclass = inTargetClassEnv.clone();
                    targetClassName = SCodeUtil::elementName(&c)?;
                    (newTargetClassName, crefPrefix) = mkVersionName(inSourceEnv, inSourceName.clone(), inPrefix, &inMod, &inTargetClassEnv, &targetClassName)?;
                    sourceRef = FNode::child(lastScopeRef(inSourceEnv)?, inSourceName.clone())?;
                    targetClassParentRef = lastScopeRef(&inTargetClassEnv)?;
                    classRef = FNode::child(targetClassParentRef.clone(), targetClassName.clone())?;
                    classRef = FNode::copyRefNoUpdate(classRef.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(classRef.clone())) {
                        Deref @ FCore::Data::CL { e: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    c = metamodelica::Own::own(__pa0);
                    c = SCodeUtil::setClassName(newTargetClassName.clone(), c.clone())?;
                    classRef = updateClassElement(classRef.clone(), c.clone(), crefPrefix.clone(), inMod.clone(), FCore::Status::CLS_INSTANCE { instanceOf: targetClassName.clone() }, &(empty()))?;
                    FNode::addChildRef(targetClassParentRef.clone(), &newTargetClassName, classRef.clone(), false)?;
                    sourceRef = updateSourceTargetScope(sourceRef.clone(), metamodelica::cons(classRef.clone(), currentScope(&gclass)))?;
                    ih = inIH.clone();
                    Ok((gclass.clone(), c.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut targetClassName: Name;
                    let mut newTargetClassName: Name;
                    c = inTargetClass.clone();
                    targetClassName = SCodeUtil::elementName(&c)?;
                    (newTargetClassName, _) = mkVersionName(inSourceEnv, inSourceName.clone(), inPrefix, &inMod, &inTargetClassEnv, &targetClassName)?;
                    Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.mkVersionNode: failed to create version node:\n")); __mm_s.push_str(&*literal!("Instance: CL(")); __mm_s.push_str(&*getGraphNameStr(inSourceEnv)); __mm_s.push_str(&*literal!(").CO(")); __mm_s.push_str(&*inSourceName); __mm_s.push_str(&*literal!(").CL(")); __mm_s.push_str(&*getGraphNameStr(&inTargetClassEnv)); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*targetClassName); __mm_s.push_str(&*SCodeDump::printModStr(Mod::unelabMod(inMod.clone())?, SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(")\n\t")); __mm_s.push_str(&*newTargetClassName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVersionedTargetClassEnv, outVersionedTargetClass, outIH))
}

pub(crate) fn createVersionScope(
    mut inSourceEnv: &Graph,
    mut inSourceName: Name,
    mut inPrefix: &DAE::Prefix,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inTargetClassEnv: Graph,
    mut inTargetClass: metamodelica::Ref<SCode::Element>,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
) -> Result<(
    Graph,
    metamodelica::Ref<SCode::Element>,
    metamodelica::List<InnerOuter::TopInstance>,
)> {
    let mut outVersionedTargetClassEnv: Graph;
    let mut outVersionedTargetClass: metamodelica::Ref<SCode::Element>;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = metamodelica::nil();
    (outVersionedTargetClassEnv, outVersionedTargetClass, outIH) = 'mc: {
        let __mc_input = &*inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD { .. } => {
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Config::acceptMetaModelicaGrammar()? || isTargetClassBuiltin(&inTargetClassEnv, &inTargetClass) || inFunctionScope(inSourceEnv) || SCodeUtil::isOperatorRecord(&inTargetClass)) else { return Err("pattern mismatch") };
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (stringEq(&(AbsynUtil::pathFirstIdent(&(getGraphName(&inTargetClassEnv)?))), &(literal!("OpenModelica")))) else { return Err("pattern mismatch") };
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut gclass: Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    (gclass, c, outIH) = mkVersionNode(inSourceEnv, inSourceName.clone(), inPrefix, inMod.clone(), inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone())?;
                    Ok(((gclass.clone(), c.clone(), outIH.clone()), outIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outIH = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVersionedTargetClassEnv, outVersionedTargetClass, outIH))
}

pub(crate) fn isTargetClassBuiltin(mut inGraph: &Graph, mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut yes: bool = false;
    yes = 'mc: {
        let __mc_input = &**inClass;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r: Ref;
                    let mut yes: bool = yes.clone();
                    r = FNode::child(lastScopeRef(inGraph)?, SCodeUtil::elementName(inClass)?)?;
                    yes = FNode::isRefBasicType(r.clone()) || FNode::isRefBuiltin(r.clone());
                    Ok((yes, yes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            yes = __wb0;
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
    yes
}

pub(crate) fn mkVersionName(
    mut inSourceEnv: &Graph,
    mut inSourceName: Name,
    mut inPrefix: &DAE::Prefix,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inTargetClassEnv: &Graph,
    mut inTargetClassName: &Name,
) -> Result<(Name, DAE::Prefix)> {
    let mut outName: Name;
    let mut outCrefPrefix: DAE::Prefix;
    (outName, outCrefPrefix) = (match inTargetClassName.clone() {
        _ => {
            let mut crefPrefix: DAE::Prefix;
            let mut name: Name;
            crefPrefix = PrefixUtil::prefixAdd(
                inSourceName,
                metamodelica::nil(),
                metamodelica::nil(),
                inPrefix,
                openmodelica_frontend_types::SCode::Variability::CONST,
                ClassInf::State::UNKNOWN {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                },
                Absyn::dummyInfo.clone(),
            )?;
            name = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inTargetClassName);
                __mm_s.push_str(&*literal!("$"));
                __mm_s.push_str(&*AbsynUtil::pathString(
                    AbsynUtil::stringListPath(
                        AbsynUtil::pathToStringList(&(PrefixUtil::prefixToPath(&crefPrefix)?)).reverse(),
                    )?,
                    literal!("$"),
                    false,
                    false,
                )?);
                ArcStr::from(__mm_s)
            };
            (name, crefPrefix)
        }
    });
    Ok((outName, outCrefPrefix))
}

pub(crate) fn getClassPrefix(mut inEnv: &FCore::Graph, mut inClassName: Name) -> DAE::Prefix {
    let mut outPrefix: DAE::Prefix;
    outPrefix = 'mc: {
        let __mc_input = inClassName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut p: DAE::Prefix;
            let mut r: Ref;
            r = FNode::child(lastScopeRef(inEnv)?, inClassName.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CL { pre: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            Ok(p.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(openmodelica_frontend_types::DAE::Prefix::NOPRE)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outPrefix
}

pub(crate) fn isInstance(mut inEnv: &FCore::Graph, mut inName: ArcStr) -> bool {
    let mut yes: bool;
    yes = 'mc: {
        let __mc_input = inName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(FNode::refData(FNode::child(lastScopeRef(inEnv)?, inName.clone())?)) {
                Deref @ FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: _ }, .. } => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    yes
}

pub(crate) fn getInstanceOriginalName(mut inEnv: &FCore::Graph, mut inName: ArcStr) -> ArcStr {
    let mut outName: ArcStr = arcstr::literal!("");
    outName = 'mc: {
        let __mc_input = inName.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut outName: ArcStr = outName.clone();
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(FNode::child(lastScopeRef(inEnv)?, inName.clone())?)) {
                Deref @ FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: __pa0 }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            outName = metamodelica::Own::own(__pa0);
            Ok((outName.clone(), outName.clone()))
        })() {
            outName = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inName.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outName
}

pub(crate) fn graphPrefixOf(mut inPrefixEnv: &Graph, mut inEnv: &Graph) -> bool {
    let mut outIsPrefix: bool;
    outIsPrefix = graphPrefixOf2(&(currentScope(inPrefixEnv).reverse()), &(currentScope(inEnv).reverse()));
    outIsPrefix
}

pub(crate) fn graphPrefixOf2<'__b>(mut inPrefixEnv: &'__b Scope, mut inEnv: &'__b Scope) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match (inPrefixEnv, inEnv) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                return true
            },
            (Deref @ metamodelica::ListNode::Cons { head: r1, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: r2, tail: rest2 }) if (stringEq(&(FNode::refName(r1.clone())), &(FNode::refName(r2.clone())))) => {
                { (inPrefixEnv, inEnv) = (rest1, rest2); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn setStatus(
    mut inEnv: Graph,
    mut inName: Name,
    mut inStatus: metamodelica::Ref<FCore::Data>,
) -> Result<Graph> {
    let mut outEnv: Graph;
    outEnv = 'mc: {
        let __mc_input = inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut n: Node;
            let mut r#ref: Ref;
            let mut refParent: Ref;
            refParent = lastScopeRef(&(g.clone()))?;
            if FNode::refHasChild(refParent.clone(), inName.clone()) {
                r#ref = FNode::child(refParent.clone(), inName.clone())?;
                if FNode::refHasChild(r#ref.clone(), arcstr::literal!(FNode::statusNodeName)) {
                    r#ref = FNode::child(r#ref.clone(), arcstr::literal!(FNode::statusNodeName))?;
                    n = FNode::setData(&(FNode::fromRef(r#ref.clone())), inStatus.clone());
                    r#ref = FNode::updateRef(r#ref.clone(), n.clone());
                } else {
                    (g, n) = node(
                        g.clone(),
                        arcstr::literal!(FNode::statusNodeName),
                        list![r#ref.clone()],
                        inStatus.clone(),
                    );
                    FNode::addChildRef(
                        r#ref.clone(),
                        &(arcstr::literal!(FNode::statusNodeName)),
                        FNode::toRef(n.clone()),
                        false,
                    )?;
                }
            }
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FGraph.setStatus failed on: "));
                __mm_s.push_str(&*getGraphNameStr(&(g.clone())));
                __mm_s.push_str(&*literal!(" element: "));
                __mm_s.push_str(&*inName);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEnv)
}

pub(crate) fn getStatus(mut inEnv: Graph, mut inName: Name) -> Result<metamodelica::Ref<FCore::Data>> {
    let mut outStatus: metamodelica::Ref<FCore::Data>;
    outStatus = (match inEnv {
        mut g => {
            let mut r#ref: Ref;
            let mut refParent: Ref;
            let mut s: metamodelica::Ref<FCore::Data>;
            refParent = lastScopeRef(&g)?;
            let true = (FNode::refHasChild(refParent.clone(), inName.clone())) else {
                return Err("pattern mismatch");
            };
            r#ref = FNode::child(refParent, inName)?;
            let true = (FNode::refHasChild(r#ref.clone(), arcstr::literal!(FNode::statusNodeName))) else {
                return Err("pattern mismatch");
            };
            r#ref = FNode::child(r#ref, arcstr::literal!(FNode::statusNodeName))?;
            s = FNode::refData(r#ref);
            s
        }
        _ => return Err("fail"),
    });
    Ok(outStatus)
}

pub(crate) fn selectScope(mut inEnv: Graph, mut inPath: &metamodelica::Ref<Absyn::Path>) -> Result<Graph> {
    let mut outEnv: Graph;
    outEnv = (match &**inPath {
        _ => {
            let mut env: Graph;
            let mut pl: metamodelica::List<ArcStr>;
            let mut lp: i32;
            let mut le: i32;
            let mut diff: i32;
            let mut cs: Scope;
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = AbsynUtil::stripLast(inPath)?;
            let true = (AbsynUtil::pathPrefixOf(p.clone(), getGraphName(&inEnv)?)) else {
                return Err("pattern mismatch");
            };
            pl = AbsynUtil::pathToStringList(&p);
            lp = ((pl).len() as i32);
            cs = currentScope(&inEnv);
            le = ((cs).len() as i32) - 1;
            diff = le - lp;
            cs = List::stripN(cs, diff)?;
            env = setScope(inEnv, cs)?;
            env
        }
    });
    Ok(outEnv)
}

pub(crate) fn makeScopePartial(mut inEnv: Graph) -> Graph {
    let mut outEnv: Graph = inEnv.clone();
    let mut node: Node;
    let mut data: Data;
    let mut el: metamodelica::Ref<SCode::Element>;
    if '__try0: {
        node = FNode::fromRef(unwrap_break_err!(lastScopeRef(&inEnv), '__try0));
        node = (::match_deref::match_deref! { match &(node.clone()) {
            Deref @ FCore::Node { data: __esc_data @ Deref @ FCore::Data::CL { e: __esc_el, .. }, .. } => {
                data = (*__esc_data).clone();
                el = (*__esc_el).clone();
                el = SCodeUtil::makeClassPartial(el.clone());
                assign_variant_field!(data => FCore::Data::CL; e = el.clone());
                assign_field!(node.data = data.clone());
                node.clone()
            },
            _ => node.clone(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        outEnv = unwrap_break_err!(setLastScopeRef(FNode::toRef(node.clone()), outEnv.clone()), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    outEnv
}

pub(crate) fn isPartialScope(mut inEnv: &Graph) -> bool {
    let mut outIsPartial: bool;
    let mut el: metamodelica::Ref<SCode::Element>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(FNode::fromRef(unwrap_break_err!(lastScopeRef(inEnv), '__try0))) {
            Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        el = metamodelica::Own::own(__pa1);
        outIsPartial = SCodeUtil::isPartial(&el);
        Ok::<_, &'static str>((outIsPartial.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outIsPartial = __try0_o0;
        }
        Err(_) => {
            outIsPartial = false;
        }
    }
    outIsPartial
}
