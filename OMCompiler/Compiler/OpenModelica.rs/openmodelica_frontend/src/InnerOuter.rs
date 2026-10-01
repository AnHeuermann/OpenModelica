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
use crate::ConnectionGraph;
use crate::FGraph;
use crate::FNode;
use crate::HashSet;
use crate::InstSection;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::UnitAbsyn;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub type Cache = FCore::Cache;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct InstResult {
    pub outCache: Cache,
    pub outEnv: FCore::Graph,
    pub outStore: UnitAbsyn::InstStore,
    pub outDae: DAE::DAElist,
    pub outSets: DAE::Connect::Sets,
    pub outType: metamodelica::Ref<DAE::Type>,
    pub outGraph: ConnectionGraph::ConnectionGraph,
}

impl metamodelica::gc::MMTrace for InstResult {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.outCache, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outEnv, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outStore, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outDae, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outSets, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outGraph, __mmv)?;
        Ok(())
    }
}
impl Default for InstResult {
    fn default() -> Self {
        Self {
            outCache: Default::default(),
            outEnv: Default::default(),
            outStore: Default::default(),
            outDae: Default::default(),
            outSets: Default::default(),
            outType: Default::default(),
            outGraph: Default::default(),
        }
    }
}

pub type INST_RESULT = InstResult;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct InstInner {
    /// the prefix of the inner. we need it to prefix the outer variables with it!
    pub innerPrefix: DAE::Prefix,
    pub name: ArcStr,
    pub io: Absyn::InnerOuter,
    /// full inner component name
    pub fullName: ArcStr,
    /// the type of the inner
    pub typePath: metamodelica::Ref<Absyn::Path>,
    /// the scope of the inner
    pub scope: ArcStr,
    pub instResult: Option<InstResult>,
    /// which outers are referencing this inner
    pub outers: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    /// class or component
    pub innerElement: Option<metamodelica::Ref<SCode::Element>>,
}

impl metamodelica::gc::MMTrace for InstInner {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.innerPrefix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.io, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fullName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.typePath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scope, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.instResult, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outers, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerElement, __mmv)?;
        Ok(())
    }
}
impl Default for InstInner {
    fn default() -> Self {
        Self {
            innerPrefix: Default::default(),
            name: Default::default(),
            io: Default::default(),
            fullName: Default::default(),
            typePath: Default::default(),
            scope: Default::default(),
            instResult: Default::default(),
            outers: Default::default(),
            innerElement: Default::default(),
        }
    }
}

pub type INST_INNER = InstInner;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct OuterPrefix {
    /// the prefix of this outer + component name
    pub outerComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    /// the coresponding prefix for this outer + component name
    pub innerComponentRef: metamodelica::Ref<DAE::ComponentRef>,
}

impl metamodelica::gc::MMTrace for OuterPrefix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.outerComponentRef, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerComponentRef, __mmv)?;
        Ok(())
    }
}
pub type OUTER = OuterPrefix;

pub type OuterPrefixes = metamodelica::List<OuterPrefix>;

thread_local! { static __emptyOuterPrefixes_TLS: metamodelica::List<OuterPrefix> = metamodelica::nil(); }
pub(crate) fn emptyOuterPrefixes() -> metamodelica::List<OuterPrefix> {
    __emptyOuterPrefixes_TLS.with(|__t| __t.clone())
}

/// the prefix + '.' + the component name
pub type Key = metamodelica::Ref<DAE::ComponentRef>;

/// the inputs of the instantiation function and the results
pub type Value = InstInner;

/// a top instance is an instance of a model thar resides at top level
#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct TopInstance {
    /// top model path
    pub path: Option<metamodelica::Ref<Absyn::Path>>,
    /// hash table with fully qualified components
    pub ht: InstHierarchyHashTable,
    /// the outer prefixes help us prefix the outer components with the correct prefix of inner component directly
    pub outerPrefixes: OuterPrefixes,
    /// Set of synchronous SM states (fully qualified components)
    pub sm: (
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

impl metamodelica::gc::MMTrace for TopInstance {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.path, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ht, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outerPrefixes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sm, __mmv)?;
        Ok(())
    }
}
impl PartialEq for TopInstance {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.ht == other.ht
            && self.outerPrefixes == other.outerPrefixes
            && (match ((&self.sm), (&other.sm)) {
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
impl Eq for TopInstance {}
impl PartialOrd for TopInstance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TopInstance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path
            .cmp(&other.path)
            .then_with(|| self.ht.cmp(&other.ht))
            .then_with(|| self.outerPrefixes.cmp(&other.outerPrefixes))
            .then_with(|| {
                (match ((&self.sm), (&other.sm)) {
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
impl std::fmt::Debug for TopInstance {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("TopInstance");
        __ds.field("path", &self.path);
        __ds.field("ht", &self.ht);
        __ds.field("outerPrefixes", &self.outerPrefixes);
        __ds.field("sm", &format_args!("<dyn-fn-container@{:p}>", (&self.sm) as *const _));
        __ds.finish()
    }
}

pub type TOP_INSTANCE = TopInstance;

pub type InstHierarchy = metamodelica::List<TopInstance>;

thread_local! { static __emptyInstHierarchy_TLS: metamodelica::List<TopInstance> = metamodelica::nil(); }
pub fn emptyInstHierarchy() -> metamodelica::List<TopInstance> {
    __emptyInstHierarchy_TLS.with(|__t| __t.clone())
}

pub(crate) fn handleInnerOuterEquations(
    mut io: Absyn::InnerOuter,
    mut inDae: DAE::DAElist,
    mut inIH: InstHierarchy,
    mut inGraphNew: ConnectionGraph::ConnectionGraph,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(DAE::DAElist, InstHierarchy, ConnectionGraph::ConnectionGraph)> {
    let mut odae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut outIH: InstHierarchy;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (odae, outIH, outGraph) = 'mc: {
        let __mc_input = (io, inDae, inIH, inGraphNew, inGraph);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::OUTER { .. }, dae, ih, _, graph) => {
                    let mut odae: DAE::DAElist = odae.clone();
                    (odae, _) = DAEUtil::splitDAEIntoVarsAndEquations(dae.clone())?;
                    Ok(((odae.clone(), ih.clone(), graph.clone()), odae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            odae = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::INNER_OUTER { .. }, dae, ih, _, graph) => {
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut dae = (*dae).clone();
                    (dae1, dae2) = DAEUtil::splitDAEIntoVarsAndEquations(dae.clone())?;
                    dae2 = DAEUtil::nameUniqueOuterVars(dae2.clone())?;
                    dae = DAEUtil::joinDaes(&dae1, &dae2)?;
                    Ok((dae.clone(), ih.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::INNER { .. }, dae, ih, graphNew, _) => {
                    Ok((dae.clone(), ih.clone(), graphNew.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::NOT_INNER_OUTER { .. }, dae, ih, graphNew, _) => {
                    Ok((dae.clone(), ih.clone(), graphNew.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("- InnerOuter.handleInnerOuterEquations failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((odae, outIH, outGraph))
}

pub(crate) fn changeInnerOuterInOuterConnect(mut sets: DAE::Connect::Sets) -> Result<DAE::Connect::Sets> {
    let mut sets: DAE::Connect::Sets = sets;
    sets.outerConnects = List::map(
        sets.outerConnects.clone(),
        &fnptr!(changeInnerOuterInOuterConnect2, DAE::Connect::OuterConnect),
    )?;
    Ok(sets)
}

fn changeInnerOuterInOuterConnect2(mut inOC: DAE::Connect::OuterConnect) -> DAE::Connect::OuterConnect {
    let mut outOC: DAE::Connect::OuterConnect;
    outOC = 'mc: {
        let __mc_input = inOC.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Connect::OuterConnect {
                scope: mut scope,
                cr1: mut cr1,
                io1: mut io1,
                f1: mut f1,
                cr2: mut cr2,
                io2: mut io2,
                f2: mut f2,
                source: mut source,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut ncr1: metamodelica::Ref<DAE::ComponentRef>;
            let (_, true) = (innerOuterBooleans(io1.clone())) else {
                return Err("pattern mismatch");
            };
            ncr1 = PrefixUtil::prefixToCref(scope.clone())?;
            let false = (ComponentReferenceBasics::crefFirstCrefLastCrefEqual(ncr1.clone(), &(cr1.clone()))?) else {
                return Err("pattern mismatch");
            };
            Ok(DAE::Connect::OuterConnect {
                scope: scope.clone(),
                cr1: cr1.clone(),
                io1: openmodelica_ast::Absyn::InnerOuter::INNER,
                f1: f1.clone(),
                cr2: cr2.clone(),
                io2: io2.clone(),
                f2: f2.clone(),
                source: source.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Connect::OuterConnect {
                scope: mut scope,
                cr1: mut cr1,
                io1: mut io1,
                f1: mut f1,
                cr2: mut cr2,
                io2: mut io2,
                f2: mut f2,
                source: mut source,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut ncr2: metamodelica::Ref<DAE::ComponentRef>;
            let (_, true) = (innerOuterBooleans(io2.clone())) else {
                return Err("pattern mismatch");
            };
            ncr2 = PrefixUtil::prefixToCref(scope.clone())?;
            let false = (ComponentReferenceBasics::crefFirstCrefLastCrefEqual(ncr2.clone(), &(cr2.clone()))?) else {
                return Err("pattern mismatch");
            };
            Ok(DAE::Connect::OuterConnect {
                scope: scope.clone(),
                cr1: cr1.clone(),
                io1: io1.clone(),
                f1: f1.clone(),
                cr2: cr2.clone(),
                io2: openmodelica_ast::Absyn::InnerOuter::INNER,
                f2: f2.clone(),
                source: source.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inOC.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outOC
}

pub(crate) fn retrieveOuterConnections(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &InstHierarchy,
    mut inPrefix: &DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inTopCall: bool,
    mut inCGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    DAE::Connect::Sets,
    metamodelica::List<DAE::Connect::OuterConnect>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outSets: DAE::Connect::Sets;
    let mut outInnerOuterConnects: metamodelica::List<DAE::Connect::OuterConnect>;
    let mut outCGraph: ConnectionGraph::ConnectionGraph;
    let mut oc: metamodelica::List<DAE::Connect::OuterConnect>;
    let Connect::SETS {
        outerConnects: __pa0, ..
    } = &inSets;
    oc = metamodelica::Own::own(__pa0);
    (oc, outSets, outInnerOuterConnects, outCGraph) =
        retrieveOuterConnections2(inCache, inEnv, inIH, inPrefix, &oc, inSets, inTopCall, inCGraph)?;
    outSets.outerConnects = oc;
    Ok((outSets, outInnerOuterConnects, outCGraph))
}

fn removeInnerPrefixFromCref(
    mut inPrefix: DAE::Prefix,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = inPrefix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Prefix::NOPRE { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(inCref.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut crefPrefix: metamodelica::Ref<DAE::ComponentRef>;
            let mut crOuter: metamodelica::Ref<DAE::ComponentRef>;
            crefPrefix = PrefixUtil::prefixToCref(inPrefix.clone())?;
            crOuter = ComponentReference::crefStripPrefix(&inCref, &crefPrefix)?;
            Ok(crOuter.clone())
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

fn retrieveOuterConnections2(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &InstHierarchy,
    mut inPrefix: &DAE::Prefix,
    mut inOuterConnects: &metamodelica::List<DAE::Connect::OuterConnect>,
    mut inSets: DAE::Connect::Sets,
    mut inTopCall: bool,
    mut inCGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    metamodelica::List<DAE::Connect::OuterConnect>,
    DAE::Connect::Sets,
    metamodelica::List<DAE::Connect::OuterConnect>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outOuterConnects: metamodelica::List<DAE::Connect::OuterConnect>;
    let mut outSets: DAE::Connect::Sets;
    let mut outInnerOuterConnects: metamodelica::List<DAE::Connect::OuterConnect>;
    let mut outCGraph: ConnectionGraph::ConnectionGraph;
    (outOuterConnects, outSets, outInnerOuterConnects, outCGraph) = 'mc: {
        let __mc_input = (&**inOuterConnects, inSets.clone(), inTopCall, inCGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                    Ok((inOuterConnects.clone(), inSets.clone(), metamodelica::nil(), inCGraph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: DAE::Connect::OuterConnect { scope, cr1, io1, f1, cr2, io2, f2, source: source @ Deref @ DAE::ElementSource { info, .. } }, tail: rest_oc }, sets, _, graph) => {
                    let mut ioc: metamodelica::List<DAE::Connect::OuterConnect>;
                    let mut inner1: bool;
                    let mut outer1: bool;
                    let mut added: bool;
                    let mut cr1 = (*cr1).clone();
                    let mut cr2 = (*cr2).clone();
                    let mut rest_oc = (*rest_oc).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (inner1, outer1) = lookupVarInnerOuterAttr(inCache.clone(), inEnv.clone(), inIH, cr1.clone(), cr2.clone())?;
                    let true = (inner1) else { return Err("pattern mismatch") };
                    let false = (outer1) else { return Err("pattern mismatch") };
                    cr1 = removeInnerPrefixFromCref(inPrefix.clone(), cr1.clone());
                    cr2 = removeInnerPrefixFromCref(inPrefix.clone(), cr2.clone());
                    (sets, added) = ConnectUtil::addOuterConnectToSets(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2), io1.clone(), io2.clone(), f1.clone(), f2.clone(), sets.clone(), metamodelica::AsArg::as_arg(&info))?;
                    (sets, graph) = addOuterConnectIfEmpty(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), sets.clone(), added, cr1.clone(), io1.clone(), f1.clone(), cr2.clone(), io2.clone(), f2.clone(), metamodelica::AsArg::as_arg(&info), graph.clone())?;
                    (rest_oc, sets, ioc, graph) = retrieveOuterConnections2(inCache, inEnv, inIH, inPrefix, metamodelica::AsArg::as_arg(&rest_oc), sets.clone(), inTopCall, graph.clone())?;
                    rest_oc = if (outer1) {metamodelica::cons(DAE::Connect::OuterConnect { scope: scope.clone(), cr1: cr1.clone(), io1: io1.clone(), f1: f1.clone(), cr2: cr2.clone(), io2: io2.clone(), f2: f2.clone(), source: source.clone() }, rest_oc.clone())} else {rest_oc.clone()};
                    Ok((rest_oc.clone(), sets.clone(), ioc.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: DAE::Connect::OuterConnect { scope: _, cr1, io1, f1, cr2, io2, f2, source: Deref @ DAE::ElementSource { info, .. } }, tail: rest_oc }, sets, true, graph) => {
                    let mut ioc: metamodelica::List<DAE::Connect::OuterConnect>;
                    let mut inner1: bool;
                    let mut inner2: bool;
                    let mut outer1: bool;
                    let mut outer2: bool;
                    let mut added: bool;
                    let mut io1 = (*io1).clone();
                    let mut io2 = (*io2).clone();
                    let mut rest_oc = (*rest_oc).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (inner1, outer1) = innerOuterBooleans(io1.clone());
                    (inner2, outer2) = innerOuterBooleans(io2.clone());
                    let true = (boolOr(inner1, inner2)) else { return Err("pattern mismatch") };
                    let false = (boolOr(outer1, outer2)) else { return Err("pattern mismatch") };
                    io1 = convertInnerOuterInnerToOuter(io1.clone());
                    io2 = convertInnerOuterInnerToOuter(io2.clone());
                    (sets, added) = ConnectUtil::addOuterConnectToSets(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2), io1.clone(), io2.clone(), f1.clone(), f2.clone(), sets.clone(), metamodelica::AsArg::as_arg(&info))?;
                    (sets, graph) = addOuterConnectIfEmpty(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), sets.clone(), added, cr1.clone(), io1.clone(), f1.clone(), cr2.clone(), io2.clone(), f2.clone(), metamodelica::AsArg::as_arg(&info), graph.clone())?;
                    (rest_oc, sets, ioc, graph) = retrieveOuterConnections2(inCache, inEnv, inIH, inPrefix, metamodelica::AsArg::as_arg(&rest_oc), sets.clone(), true, graph.clone())?;
                    Ok((rest_oc.clone(), sets.clone(), ioc.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: oc, tail: rest_oc }, sets, _, graph) => {
                    let mut ioc: metamodelica::List<DAE::Connect::OuterConnect>;
                    let mut rest_oc = (*rest_oc).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (rest_oc, sets, ioc, graph) = retrieveOuterConnections2(inCache, inEnv, inIH, inPrefix, metamodelica::AsArg::as_arg(&rest_oc), sets.clone(), inTopCall, graph.clone())?;
                    Ok((metamodelica::cons(oc.clone(), rest_oc.clone()), sets.clone(), ioc.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outOuterConnects, outSets, outInnerOuterConnects, outCGraph))
}

fn convertInnerOuterInnerToOuter(mut io: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut oio: Absyn::InnerOuter;
    oio = (match io {
        Absyn::InnerOuter::INNER { .. } => openmodelica_ast::Absyn::InnerOuter::OUTER,
        _ => io,
    });
    oio
}

fn addOuterConnectIfEmpty(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: InstHierarchy,
    mut pre: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut added: bool,
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut iio1: Absyn::InnerOuter,
    mut f1: DAE::Connect::Face,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut iio2: Absyn::InnerOuter,
    mut f2: DAE::Connect::Face,
    mut info: &SourceInfo,
    mut inCGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(DAE::Connect::Sets, ConnectionGraph::ConnectionGraph)> {
    let mut outSets: DAE::Connect::Sets;
    let mut outCGraph: ConnectionGraph::ConnectionGraph;
    (outSets, outCGraph) = (match (inSets.clone(), added) {
        (_, true) => (inSets, inCGraph),
        (
            DAE::Connect::Sets {
                sets: mut sets,
                setCount: mut sc,
                connections: ref cl,
                outerConnects: ref oc,
            },
            false,
        ) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut io1 = iio1;
            let mut io2 = iio2;
            let mut graph = inCGraph.clone();
            let mut vt1: SCode::Variability;
            let mut vt2: SCode::Variability;
            let mut t1: metamodelica::Ref<DAE::Type>;
            let mut t2: metamodelica::Ref<DAE::Type>;
            let mut ct: metamodelica::Ref<DAE::ConnectorType>;
            let mut sets = sets.clone();
            let mut cl = cl.clone();
            let (__pa0, __t4, __pa3, _, _, _, _, _, _) = Lookup::lookupVar(cache, env.clone(), cr1.clone())?;
            let __arc5 = __t4.clone();
            let DAE::ATTR {
                connectorType: __pa1,
                variability: __pa2,
                ..
            } = &*__arc5;
            cache = metamodelica::Own::own(__pa0);
            ct = metamodelica::Own::own(__pa1);
            vt1 = metamodelica::Own::own(__pa2);
            t1 = metamodelica::Own::own(__pa3);
            let (__pa6, __t9, __pa8, _, _, _, _, _, _) = Lookup::lookupVar(cache, env.clone(), cr2.clone())?;
            let __arc10 = __t9.clone();
            let DAE::ATTR { variability: __pa7, .. } = &*__arc10;
            cache = metamodelica::Own::own(__pa6);
            vt2 = metamodelica::Own::own(__pa7);
            t2 = metamodelica::Own::own(__pa8);
            io1 = removeOuter(io1);
            io2 = removeOuter(io2);
            let (
                __pa11,
                __pa12,
                __pa13,
                Connect::SETS {
                    sets: __pa14,
                    setCount: __pa15,
                    connections: __pa16,
                    ..
                },
                _,
                __pa17,
            ) = InstSection::connectComponents(
                cache,
                env,
                ih,
                DAE::Connect::Sets {
                    sets: sets.clone(),
                    setCount: sc.clone(),
                    connections: cl.clone(),
                    outerConnects: metamodelica::nil(),
                },
                pre,
                cr1,
                f1,
                t1,
                vt1,
                cr2,
                f2,
                t2,
                vt2,
                ct,
                io1,
                io2,
                graph,
                info,
            )?;
            cache = metamodelica::Own::own(__pa11);
            env = metamodelica::Own::own(__pa12);
            ih = metamodelica::Own::own(__pa13);
            sets = metamodelica::Own::own(__pa14);
            sc = metamodelica::Own::own(__pa15);
            cl = metamodelica::Own::own(__pa16);
            graph = metamodelica::Own::own(__pa17);
            (
                DAE::Connect::Sets {
                    sets: sets.clone(),
                    setCount: sc.clone(),
                    connections: cl.clone(),
                    outerConnects: oc.clone(),
                },
                graph,
            )
        }
        _ => return Err("fail"),
    });
    Ok((outSets, outCGraph))
}

fn removeOuter(mut io: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut outIo: Absyn::InnerOuter;
    outIo = (match io {
        Absyn::InnerOuter::OUTER { .. } => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        Absyn::InnerOuter::INNER { .. } => openmodelica_ast::Absyn::InnerOuter::INNER,
        Absyn::InnerOuter::INNER_OUTER { .. } => openmodelica_ast::Absyn::InnerOuter::INNER,
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
    });
    outIo
}

fn lookupVarInnerOuterAttr(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut inIH: &InstHierarchy,
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(bool, bool)> {
    let mut isInner: bool = false;
    let mut isOuter: bool = false;
    (isInner, isOuter) = 'mc: {
        let __mc_input = &*cr2;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut io1: Absyn::InnerOuter;
                    let mut io2: Absyn::InnerOuter;
                    let mut isInner1: bool;
                    let mut isInner2: bool;
                    let mut isOuter1: bool;
                    let mut isOuter2: bool;
                    let mut isInner: bool = isInner.clone();
                    let mut isOuter: bool = isOuter.clone();
                    ErrorExt::setCheckpoint(literal!("lookupVarInnerOuterAttr"));
                    let (_, __t1, _, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr1.clone())?;
                    let __arc2 = __t1.clone();
                    let DAE::ATTR { innerOuter: __pa0, .. } = &*__arc2;
                    io1 = metamodelica::Own::own(__pa0);
                    let (_, __t4, _, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr2.clone())?;
                    let __arc5 = __t4.clone();
                    let DAE::ATTR { innerOuter: __pa3, .. } = &*__arc5;
                    io2 = metamodelica::Own::own(__pa3);
                    (isInner1, isOuter1) = innerOuterBooleans(io1);
                    (isInner2, isOuter2) = innerOuterBooleans(io2);
                    isInner = isInner1 || isInner2;
                    isOuter = isOuter1 || isOuter2;
                    ErrorExt::rollBack(literal!("lookupVarInnerOuterAttr"));
                    Ok(((isInner, isOuter), isInner.clone(), isOuter.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            isInner = __wb0;
            isOuter = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut io: Absyn::InnerOuter;
                    let mut isInner: bool = isInner.clone();
                    let mut isOuter: bool = isOuter.clone();
                    let (_, __t1, _, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr1.clone())?;
                    let __arc2 = __t1.clone();
                    let DAE::ATTR { innerOuter: __pa0, .. } = &*__arc2;
                    io = metamodelica::Own::own(__pa0);
                    (isInner, isOuter) = innerOuterBooleans(io);
                    ErrorExt::rollBack(literal!("lookupVarInnerOuterAttr"));
                    Ok(((isInner, isOuter), isInner.clone(), isOuter.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            isInner = __wb0;
            isOuter = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut io: Absyn::InnerOuter;
                    let mut isInner: bool = isInner.clone();
                    let mut isOuter: bool = isOuter.clone();
                    let (_, __t1, _, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr2.clone())?;
                    let __arc2 = __t1.clone();
                    let DAE::ATTR { innerOuter: __pa0, .. } = &*__arc2;
                    io = metamodelica::Own::own(__pa0);
                    (isInner, isOuter) = innerOuterBooleans(io);
                    ErrorExt::rollBack(literal!("lookupVarInnerOuterAttr"));
                    Ok(((isInner, isOuter), isInner.clone(), isOuter.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            isInner = __wb0;
            isOuter = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack(literal!("lookupVarInnerOuterAttr"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((isInner, isOuter))
}

fn innerOuterBooleans(mut io: Absyn::InnerOuter) -> (bool, bool) {
    let mut inner1: bool;
    let mut outer1: bool;
    (inner1, outer1) = (match io {
        Absyn::InnerOuter::INNER { .. } => (true, false),
        Absyn::InnerOuter::OUTER { .. } => (false, true),
        Absyn::InnerOuter::INNER_OUTER { .. } => (true, true),
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => (false, false),
    });
    (inner1, outer1)
}

pub(crate) fn outerConnection(mut io1: Absyn::InnerOuter, mut io2: Absyn::InnerOuter) -> bool {
    let mut isOuter: bool;
    isOuter = (match (io1, io2) {
        (Absyn::InnerOuter::OUTER { .. }, _) => true,
        (_, Absyn::InnerOuter::OUTER { .. }) => true,
        (Absyn::InnerOuter::INNER_OUTER { .. }, _) => true,
        (_, Absyn::InnerOuter::INNER_OUTER { .. }) => true,
        _ => false,
    });
    isOuter
}

fn lookupInnerInIH(
    mut inTIH: &TopInstance,
    mut inPrefix: DAE::Prefix,
    mut inComponentIdent: ArcStr,
) -> Result<InstInner> {
    let mut outInstInner: InstInner;
    outInstInner = 'mc: {
        let __mc_input = (inTIH, inPrefix.clone(), inComponentIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { .. }, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, .. }, _) => {
                    Ok(lookupInnerInIH(inTIH, openmodelica_frontend_types::DAE::Prefix::NOPRE, inComponentIdent.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { .. }, DAE::Prefix::NOPRE { .. }, name) => {
                    Ok(emptyInstInner(openmodelica_frontend_types::DAE::Prefix::NOPRE, name.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { path: _, ht, outerPrefixes: _, sm: _ }, _, name) => {
                    let mut prefix: DAE::Prefix;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut instInner: InstInner;
                    prefix = PrefixUtil::prefixStripLast(&inPrefix)?;
                    (_, cref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), &(emptyInstHierarchy().clone()), prefix.clone(), ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    instInner = get(&cref, metamodelica::AsArg::as_arg(&ht))?;
                    Ok(instInner.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { path: _, ht, outerPrefixes: _, sm: _ }, _, name) => {
                    let mut prefix: DAE::Prefix;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut instInner: InstInner;
                    prefix = PrefixUtil::prefixStripLast(&inPrefix)?;
                    (_, cref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), &(emptyInstHierarchy().clone()), prefix.clone(), ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    if '__try0: {
                        unwrap_break_err!(get(&cref, metamodelica::AsArg::as_arg(&ht)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    instInner = lookupInnerInIH(inTIH, prefix.clone(), name.clone())?;
                    Ok(instInner.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { .. }, prefix, name) => {
                    Ok(emptyInstInner(prefix.clone(), name.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outInstInner)
}

pub(crate) fn modificationOnOuter(
    mut cache: &FCore::Cache,
    mut env: &FCore::Graph,
    mut ih: &InstHierarchy,
    mut prefix: &DAE::Prefix,
    mut componentName: &ArcStr,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut io: Absyn::InnerOuter,
    mut r#impl: bool,
    mut inInfo: &SourceInfo,
) -> bool {
    let mut modd: bool;
    modd = 'mc: {
        let __mc_input = (&**inMod, io);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { .. }, Absyn::InnerOuter::OUTER { .. }) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s: ArcStr;
                    s1 = ComponentReferenceBasics::printComponentRefStr(cr)?;
                    s2 = Mod::prettyPrintMod(inMod, 0)?;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::OUTER_MODIFICATION.clone()), list![s.clone()], inInfo)?;
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
    modd
}

pub(crate) fn switchInnerToOuterInGraph(
    mut inEnv: FCore::Graph,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = (::match_deref::match_deref! { match &(inEnv.clone()) {
        FCore::Graph::EG { name: _ } => {
            inEnv
        },
        FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Nil, .. } => {
            inEnv
        },
        _ => {
            let mut cr = inCr;
            let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
            let mut n: metamodelica::Ref<FCore::Node>;
            r = FGraph::lastScopeRef(&inEnv)?;
            n = FNode::fromRef(r.clone());
            n = switchInnerToOuterInNode(n, &cr)?;
            r = FNode::updateRef(r, n);
            inEnv
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn switchInnerToOuterInNode(
    mut inNode: metamodelica::Ref<FCore::Node>,
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<FCore::Node>> {
    let mut outNode: metamodelica::Ref<FCore::Node> = inNode;
    let () = (match &*outNode {
        FCore::Node { .. } => {
            assign_field!(
                outNode.children = FCore::RefTree::map(
                    outNode.children.clone(),
                    &({
                        let __pe_b1 = inCr.clone();
                        move |__pe_a0, __pe_a2| switchInnerToOuterInChild(&__pe_a0, &__pe_b1, __pe_a2)
                    })
                )?
            );
            ()
        }
        _ => (),
    });
    Ok(outNode)
}

fn switchInnerToOuterInChild(
    mut name: &ArcStr,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut inRef: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
) -> Result<Mutable::Mutable<metamodelica::Ref<FCore::Node>>> {
    let mut r#ref: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
    let mut n: metamodelica::Ref<FCore::Node>;
    n = FNode::fromRef(inRef.clone());
    n = switchInnerToOuterInChildrenValue(n, cr)?;
    r#ref = FNode::updateRef(inRef, n);
    Ok(r#ref)
}

fn switchInnerToOuterInChildrenValue(
    mut inNode: metamodelica::Ref<FCore::Node>,
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<FCore::Node>> {
    let mut outNode: metamodelica::Ref<FCore::Node>;
    outNode = 'mc: {
        let __mc_input = inNode.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                node => {
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut name: ArcStr;
                    let mut attributes: metamodelica::Ref<DAE::Attributes>;
                    let mut visibility: SCode::Visibility;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut bndsrc: bool;
                    let mut ct: metamodelica::Ref<DAE::ConnectorType>;
                    let mut parallelism: SCode::Parallelism;
                    let mut variability: SCode::Variability;
                    let mut direction: Absyn::Direction;
                    let mut cnstForRange: Option<DAE::Const>;
                    r = FNode::childFromNode(metamodelica::AsArg::as_arg(&node), arcstr::literal!(FNode::itNodeName))?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                        Deref @ FCore::Data::IT { i: Deref @ DAE::Var { name: __pa0, attributes: __pa1, ty: __pa2, binding: __pa3, bind_from_outside: __pa4, constOfForIteratorRange: __pa5 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    attributes = metamodelica::Own::own(__pa1);
                    ty = metamodelica::Own::own(__pa2);
                    binding = metamodelica::Own::own(__pa3);
                    bndsrc = metamodelica::Own::own(__pa4);
                    cnstForRange = metamodelica::Own::own(__pa5);
                    let (__pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(attributes.clone()) {
                        Deref @ DAE::Attributes { connectorType: __pa7, parallelism: __pa8, variability: __pa9, direction: __pa10, innerOuter: Absyn::InnerOuter::INNER { .. }, visibility: __pa11 } => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ct = metamodelica::Own::own(__pa7);
                    parallelism = metamodelica::Own::own(__pa8);
                    variability = metamodelica::Own::own(__pa9);
                    direction = metamodelica::Own::own(__pa10);
                    visibility = metamodelica::Own::own(__pa11);
                    attributes = metamodelica::Ref::new(DAE::Attributes { connectorType: ct.clone(), parallelism: parallelism, variability: variability, direction: direction, innerOuter: openmodelica_ast::Absyn::InnerOuter::OUTER, visibility: visibility });
                    r = FNode::updateRef(r.clone(), FNode::setData(&(FNode::fromRef(r.clone())), metamodelica::Ref::new(FCore::Data::IT { i: metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: attributes.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: bndsrc, constOfForIteratorRange: cnstForRange.clone() }) })));
                    Ok(node.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                node => {
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut name: ArcStr;
                    let mut attributes: metamodelica::Ref<DAE::Attributes>;
                    let mut visibility: SCode::Visibility;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut bndsrc: bool;
                    let mut ct: metamodelica::Ref<DAE::ConnectorType>;
                    let mut parallelism: SCode::Parallelism;
                    let mut variability: SCode::Variability;
                    let mut direction: Absyn::Direction;
                    let mut cnstForRange: Option<DAE::Const>;
                    r = FNode::childFromNode(metamodelica::AsArg::as_arg(&node), arcstr::literal!(FNode::itNodeName))?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                        Deref @ FCore::Data::IT { i: Deref @ DAE::Var { name: __pa0, attributes: __pa1, ty: __pa2, binding: __pa3, bind_from_outside: __pa4, constOfForIteratorRange: __pa5 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    attributes = metamodelica::Own::own(__pa1);
                    ty = metamodelica::Own::own(__pa2);
                    binding = metamodelica::Own::own(__pa3);
                    bndsrc = metamodelica::Own::own(__pa4);
                    cnstForRange = metamodelica::Own::own(__pa5);
                    let (__pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(attributes.clone()) {
                        Deref @ DAE::Attributes { connectorType: __pa7, parallelism: __pa8, variability: __pa9, direction: __pa10, innerOuter: Absyn::InnerOuter::INNER_OUTER { .. }, visibility: __pa11 } => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ct = metamodelica::Own::own(__pa7);
                    parallelism = metamodelica::Own::own(__pa8);
                    variability = metamodelica::Own::own(__pa9);
                    direction = metamodelica::Own::own(__pa10);
                    visibility = metamodelica::Own::own(__pa11);
                    attributes = metamodelica::Ref::new(DAE::Attributes { connectorType: ct.clone(), parallelism: parallelism, variability: variability, direction: direction, innerOuter: openmodelica_ast::Absyn::InnerOuter::OUTER, visibility: visibility });
                    r = FNode::updateRef(r.clone(), FNode::setData(&(FNode::fromRef(r.clone())), metamodelica::Ref::new(FCore::Data::IT { i: metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: attributes.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: bndsrc, constOfForIteratorRange: cnstForRange.clone() }) })));
                    Ok(node.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNode.clone())
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

// /////////////////////////////////////////////////
// / instance hieararchy for inner/outer
// / add furher functions before this
// /////////////////////////////////////////////////
fn emptyInstInner(mut innerPrefix: DAE::Prefix, mut name: ArcStr) -> InstInner {
    let mut outInstInner: InstInner;
    outInstInner = InstInner {
        innerPrefix: innerPrefix,
        name: name,
        io: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        fullName: literal!(""),
        typePath: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
        scope: literal!(""),
        instResult: None,
        outers: metamodelica::nil(),
        innerElement: None,
    };
    outInstInner
}

pub(crate) fn lookupInnerVar(
    mut inCache: &Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &InstHierarchy,
    mut inPrefix: DAE::Prefix,
    mut inIdent: ArcStr,
    mut io: Absyn::InnerOuter,
) -> Result<InstInner> {
    let mut outInstInner: InstInner;
    outInstInner = 'mc: {
        let __mc_input = (&**inIH, inPrefix, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: tih, tail: _ }, pre, n) => {
                    let mut instInner: InstInner;
                    instInner = lookupInnerInIH(metamodelica::AsArg::as_arg(&tih), pre.clone(), n.clone())?;
                    Ok(instInner.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, pre, n) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InnerOuter.lookupInnerVar failed on component: ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(metamodelica::AsArg::as_arg(&pre))?); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*n); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outInstInner)
}

pub(crate) fn updateInstHierarchy<'__b>(
    mut inIH: InstHierarchy,
    mut inPrefix: &'__b DAE::Prefix,
    mut inInnerOuter: Absyn::InnerOuter,
    mut inInstInner: &'__b InstInner,
) -> Result<InstHierarchy> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inIH, inInstInner.clone())) {
            (Deref @ metamodelica::ListNode::Nil, InstInner { .. }) => {
                let mut tih: TopInstance;
                let mut ih: InstHierarchy;
                let mut ht: InstHierarchyHashTable;
                let mut sm: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                ht = emptyInstHierarchyHashTable();
                sm = HashSet::emptyHashSet();
                tih = TopInstance { path: None, ht: ht, outerPrefixes: emptyOuterPrefixes().clone(), sm: sm };
                { (inIH, inPrefix, inInnerOuter, inInstInner) = (list![tih], inPrefix, inInnerOuter, inInstInner); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: TopInstance { path: pathOpt, ht, outerPrefixes, sm }, tail: restIH }, InstInner { name, .. }) => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                let mut ht = (*ht).clone();
                cref_ = ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                (_, cref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), &(emptyInstHierarchy().clone()), inPrefix.clone(), cref_)?;
                ht = add((cref, inInstInner.clone()), metamodelica::AsArg::as_arg(&ht))?;
                return Ok(metamodelica::cons(TopInstance { path: pathOpt.clone(), ht: ht.clone(), outerPrefixes: outerPrefixes.clone(), sm: sm.clone() }, restIH.clone()))
            },
            (_, InstInner { .. }) => {
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn updateSMHierarchy(
    mut smState: metamodelica::Ref<DAE::ComponentRef>,
    mut inIH: &InstHierarchy,
) -> Result<InstHierarchy> {
    let mut outIH: InstHierarchy;
    outIH = (::match_deref::match_deref! { match &((smState.clone(), inIH.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            let mut tih: TopInstance;
            let mut ih: InstHierarchy;
            let mut ht: InstHierarchyHashTable;
            let mut sm: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut sm2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            ht = emptyInstHierarchyHashTable();
            sm = HashSet::emptyHashSet();
            sm2 = BaseHashSet::add(smState, &sm)?;
            tih = TopInstance { path: None, ht: ht, outerPrefixes: emptyOuterPrefixes().clone(), sm: sm2 };
            ih = list![tih];
            ih
        },
        (cref, Deref @ metamodelica::ListNode::Cons { head: TopInstance { path: pathOpt, ht, outerPrefixes, sm }, tail: restIH }) => {
            let mut sm = (*sm).clone();
            sm = BaseHashSet::add(cref.clone(), &(sm.clone()))?;
            metamodelica::cons(TopInstance { path: pathOpt.clone(), ht: ht.clone(), outerPrefixes: outerPrefixes.clone(), sm: sm.clone() }, restIH.clone())
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, _) => {
            let true = (Flags::isSet(Flags::INSTANCE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InnerOuter.updateSMHierarchy failure for: ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outIH)
}

pub(crate) fn addClassIfInner(
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inPrefix: DAE::Prefix,
    mut inScope: &FCore::Graph,
    mut inIH: InstHierarchy,
) -> InstHierarchy {
    let mut outIH: InstHierarchy = metamodelica::nil();
    outIH = 'mc: {
        let __mc_input = &*inClass;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } => {
                    let mut scopeName: ArcStr;
                    let mut outIH: metamodelica::List<TopInstance> = outIH.clone();
                    let true = (AbsynUtil::isInner(io.clone())) else { return Err("pattern mismatch") };
                    scopeName = FGraph::getGraphNameStr(inScope);
                    outIH = updateInstHierarchy(inIH.clone(), &(inPrefix.clone()), io.clone(), &(InstInner { innerPrefix: inPrefix.clone(), name: name.clone(), io: io.clone(), fullName: name.clone(), typePath: metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), scope: scopeName.clone(), instResult: None, outers: metamodelica::nil(), innerElement: Some(inClass.clone()) }))?;
                    Ok((outIH.clone(), outIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outIH = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inIH.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outIH
}

pub(crate) fn addOuterPrefixToIH(
    mut inIH: &InstHierarchy,
    mut inOuterComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInnerComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<InstHierarchy> {
    let mut outIH: InstHierarchy;
    outIH = 'mc: {
        let __mc_input = &**inIH;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut tih: TopInstance;
                    let mut ih: InstHierarchy;
                    let mut ht: InstHierarchyHashTable;
                    let mut sm: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = emptyInstHierarchyHashTable();
                    sm = HashSet::emptyHashSet();
                    tih = TopInstance { path: None, ht: ht.clone(), outerPrefixes: list![OuterPrefix { outerComponentRef: ComponentReference::crefStripSubs(inOuterComponentRef)?, innerComponentRef: inInnerComponentRef.clone() }], sm: sm.clone() };
                    ih = list![tih.clone()];
                    Ok(ih.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: TopInstance { path: pathOpt, ht, outerPrefixes, sm }, tail: restIH } => {
                    let mut outerPrefixes = (*outerPrefixes).clone();
                    outerPrefixes = List::unionElt(OuterPrefix { outerComponentRef: ComponentReference::crefStripSubs(inOuterComponentRef)?, innerComponentRef: inInnerComponentRef.clone() }, outerPrefixes.clone());
                    Ok(metamodelica::cons(TopInstance { path: pathOpt.clone(), ht: ht.clone(), outerPrefixes: outerPrefixes.clone(), sm: sm.clone() }, restIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InnerOuter.addOuterPrefix failed to add: outer cref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inOuterComponentRef)?); __mm_s.push_str(&*literal!(" refers to inner cref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inInnerComponentRef)?); __mm_s.push_str(&*literal!(" to IH")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outIH)
}

pub(crate) fn prefixOuterCrefWithTheInnerPrefix(
    mut inIH: &InstHierarchy,
    mut inOuterComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inPrefix: DAE::Prefix,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outInnerComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outInnerComponentRef = (::match_deref::match_deref! { match inIH {
        Deref @ metamodelica::ListNode::Nil => {
            return Err("fail")
        },
        Deref @ metamodelica::ListNode::Cons { head: TopInstance { path: _, ht: _, outerPrefixes: outerPrefixes @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, sm: _ }, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut outerCrefPrefix: metamodelica::Ref<DAE::ComponentRef>;
            let mut fullCref: metamodelica::Ref<DAE::ComponentRef>;
            let mut innerCref: metamodelica::Ref<DAE::ComponentRef>;
            let mut innerCrefPrefix: metamodelica::Ref<DAE::ComponentRef>;
            (_, fullCref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), &(emptyInstHierarchy().clone()), inPrefix, inOuterComponentRef.clone())?;
            (outerCrefPrefix, innerCrefPrefix) = searchForInnerPrefix(&fullCref, &inOuterComponentRef, metamodelica::AsArg::as_arg(&outerPrefixes))?;
            innerCref = changeOuterReferenceToInnerReference(fullCref, outerCrefPrefix, innerCrefPrefix)?;
            innerCref
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInnerComponentRef)
}

fn changeOuterReferenceToInnerReference(
    mut inFullCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inOuterCrefPrefix: metamodelica::Ref<DAE::ComponentRef>,
    mut inInnerCrefPrefix: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outInnerCref: metamodelica::Ref<DAE::ComponentRef>;
    outInnerCref = (::match_deref::match_deref! { match &((inFullCref, inOuterCrefPrefix, inInnerCrefPrefix)) {
        (ifull, ocp, icp) => {
            let mut ic: metamodelica::Ref<DAE::ComponentRef>;
            let mut eifull: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut eocp: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut eicp: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut epre: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut erest: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut esuffix: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            eifull = ComponentReference::explode(metamodelica::AsArg::as_arg(&ifull))?;
            eicp = ComponentReference::explode(metamodelica::AsArg::as_arg(&icp))?;
            (eocp, esuffix) = List::split(eifull, ComponentReference::identifierCount(metamodelica::AsArg::as_arg(&ocp)))?;
            (epre, erest) = List::splitEqualPrefix(eocp, eicp.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefFirstIdentEqual(&__a0, &__a1), &(metamodelica::nil()))?;
            (_, eicp) = List::splitEqualPrefix(eicp, epre.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefFirstIdentEqual(&__a0, &__a1), &(metamodelica::nil()))?;
            (erest, _) = List::splitEqualPrefix(erest.reverse(), eicp.reverse(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefFirstIdentEqual(&__a0, &__a1), &(metamodelica::nil()))?;
            erest = List::append_reverse(&erest, esuffix);
            eifull = listAppend(epre, erest);
            ic = ComponentReference::implode(eifull)?;
            ic
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInnerCref)
}

fn searchForInnerPrefix(
    mut fullCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inOuterCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut outerPrefixes: &OuterPrefixes,
) -> Result<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    let mut outerCrefPrefix: metamodelica::Ref<DAE::ComponentRef>;
    let mut innerCrefPrefix: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut b1: bool = false;
    let mut b2: bool = false;
    for mut op in &**outerPrefixes {
        let OuterPrefix {
            outerComponentRef: __pa0,
            ..
        } = &op;
        outerCrefPrefix = metamodelica::Own::own(__pa0);
        b1 = ComponentReferenceBasics::crefPrefixOfIgnoreSubscripts(&outerCrefPrefix, fullCref);
        if !(b1) {
            cr = ComponentReference::crefStripLastIdent(&outerCrefPrefix)?;
            b2 = metamodelica::stringEq(
                &(ComponentReferenceBasics::crefLastIdent(&outerCrefPrefix)?),
                &(ComponentReferenceBasics::crefFirstIdent(inOuterCref)?),
            ) && ComponentReferenceBasics::crefPrefixOfIgnoreSubscripts(&cr, fullCref);
        }
        if b1 || b2 {
            let OuterPrefix {
                innerComponentRef: __pa1,
                ..
            } = &op;
            innerCrefPrefix = metamodelica::Own::own(__pa1);
            return Ok((outerCrefPrefix, innerCrefPrefix));
        }
    }
    return Err("fail");
    Ok((outerCrefPrefix, innerCrefPrefix))
}

fn printInnerDefStr(mut inInstInner: &InstInner) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (match inInstInner.clone() {
        InstInner {
            innerPrefix: _,
            name: _,
            io: _,
            fullName: mut fullName,
            typePath: mut typePath,
            scope: mut scope,
            instResult: _,
            outers: mut outers,
            innerElement: _,
        } => {
            let mut r#str: ArcStr;
            let mut strOuters: ArcStr;
            let mut outers = outers.clone();
            outers = List::uniqueOnTrue(metamodelica::AsArg::as_arg(&outers), &move |__a0: metamodelica::Ref<
                DAE::ComponentRef,
            >,
                                                                                     __a1: metamodelica::Ref<
                DAE::ComponentRef,
            >| {
                ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1)
            })?;
            strOuters = if ((outers).is_empty()) {
                literal!("")
            } else {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" Referenced by 'outer' components: {"));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(outers.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                            ComponentReferenceBasics::printComponentRefStr(&__a0)
                        })?,
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!("}"));
                    ArcStr::from(__mm_s)
                }
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*AbsynUtil::pathString(typePath.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*fullName);
                __mm_s.push_str(&*literal!("; defined in scope: "));
                __mm_s.push_str(&*scope);
                __mm_s.push_str(&*literal!("."));
                __mm_s.push_str(&*strOuters);
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    Ok(outStr)
}

pub(crate) fn getExistingInnerDeclarations(mut inIH: &InstHierarchy, mut inEnv: &FCore::Graph) -> Result<ArcStr> {
    let mut innerDeclarations: ArcStr;
    innerDeclarations = (::match_deref::match_deref! { match inIH {
        Deref @ metamodelica::ListNode::Nil => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("There are no 'inner' components defined in the model in any of the parent scopes of 'outer' component's scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }
        },
        Deref @ metamodelica::ListNode::Cons { head: TopInstance { path: _, ht, outerPrefixes: _, sm: _ }, tail: _ } => {
            let mut inners: metamodelica::List<InstInner>;
            let mut r#str: ArcStr;
            inners = getInnersFromInstHierarchyHashTable(metamodelica::AsArg::as_arg(&ht))?;
            r#str = stringDelimitList(List::map(inners, &move |__a0: InstInner| printInnerDefStr(&__a0))?, literal!("\n    "));
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(innerDeclarations)
}

fn getInnersFromInstHierarchyHashTable(mut t: &InstHierarchyHashTable) -> Result<metamodelica::List<InstInner>> {
    let mut inners: metamodelica::List<InstInner>;
    inners = List::map(hashTableList(t)?, &move |__a0: (
        metamodelica::Ref<DAE::ComponentRef>,
        InstInner,
    )|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(getValue(&__a0))
    })?;
    Ok(inners)
}

fn getValue(mut tpl: &(metamodelica::Ref<DAE::ComponentRef>, InstInner)) -> InstInner {
    let mut v: InstInner;
    v = (::match_deref::match_deref! { match &(tpl) {
        (_, __esc_v) => {
            v = (*__esc_v).clone();
            v.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    v
}

// ///////////////////////////////////////////////////////////////
// hash table implementation for InnerOuter instance hierarchy //
// ///////////////////////////////////////////////////////////////
fn hashFunc(mut k: &Key) -> Result<i32> {
    let mut res: i32;
    res = stringHashDjb2(&(ComponentReferenceBasics::printComponentRefStr(k)?));
    Ok(res)
}

fn keyEqual(mut key1: &Key, mut key2: &Key) -> Result<bool> {
    let mut res: bool;
    res = ComponentReferenceBasics::crefEqualNoStringCompare(key1, key2)?;
    Ok(res)
}

fn dumpInstHierarchyHashTable(mut t: &InstHierarchyHashTable) -> Result<()> {
    metamodelica::print(literal!("InstHierarchyHashTable:\n"));
    metamodelica::print(stringDelimitList(
        List::map(hashTableList(t)?, &move |__a0: (
            metamodelica::Ref<DAE::ComponentRef>,
            InstInner,
        )| dumpTuple(&__a0))?,
        literal!("\n"),
    ));
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn dumpTuple(mut tpl: &(metamodelica::Ref<DAE::ComponentRef>, InstInner)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &(tpl) {
        (k, _) => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*ComponentReference::crefStr(metamodelica::AsArg::as_arg(&k))?); __mm_s.push_str(&*literal!(" opaque InstInner for now, implement printing. ")); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

/* end of InstHierarchyHashTable instance specific code */
/* Generic hashtable code below!! */
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct InstHierarchyHashTable {
    /// hashtable to translate Key to array indx
    pub hashTable: metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    /// Array of values
    pub valueArr: ValueArray,
    /// bucket size
    pub bucketSize: i32,
    /// number of entries in hashtable
    pub numberOfEntries: i32,
}

impl metamodelica::gc::MMTrace for InstHierarchyHashTable {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.hashTable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.valueArr, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.bucketSize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfEntries, __mmv)?;
        Ok(())
    }
}
impl Default for InstHierarchyHashTable {
    fn default() -> Self {
        Self {
            hashTable: Default::default(),
            valueArr: Default::default(),
            bucketSize: Default::default(),
            numberOfEntries: Default::default(),
        }
    }
}

pub type HASHTABLE = InstHierarchyHashTable;

/// array of values are expandable, to amortize the
/// cost of adding elements in a more efficient manner
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ValueArray {
    /// number of elements in hashtable
    pub numberOfElements: i32,
    /// array of values
    pub valueArray: metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>>,
}

impl metamodelica::gc::MMTrace for ValueArray {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfElements, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.valueArray, __mmv)?;
        Ok(())
    }
}
impl Default for ValueArray {
    fn default() -> Self {
        Self {
            numberOfElements: Default::default(),
            valueArray: Default::default(),
        }
    }
}

pub type VALUE_ARRAY = ValueArray;

fn emptyInstHierarchyHashTable() -> InstHierarchyHashTable {
    let mut hashTable: InstHierarchyHashTable;
    let mut arr: metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>;
    let mut emptyarr: metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>>;
    arr = arrayCreate(1000, metamodelica::nil());
    emptyarr = arrayCreate(100, None);
    hashTable = InstHierarchyHashTable {
        hashTable: arr.clone(),
        valueArr: ValueArray {
            numberOfElements: 0,
            valueArray: emptyarr.clone(),
        },
        bucketSize: 1000,
        numberOfEntries: 0,
    };
    hashTable
}

fn add(
    mut entry: (metamodelica::Ref<DAE::ComponentRef>, InstInner),
    mut hashTable: &InstHierarchyHashTable,
) -> Result<InstHierarchyHashTable> {
    let mut outHashTable: InstHierarchyHashTable;
    outHashTable = 'mc: {
        let __mc_input = (entry, hashTable);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ (key, _), InstHierarchyHashTable { hashTable: hashvec, valueArr: varr, bucketSize: bsize, numberOfEntries: _ }) => {
                    let mut hval: i32;
                    let mut indx: i32;
                    let mut newpos: i32;
                    let mut n_1: i32;
                    let mut varr_1: ValueArray;
                    let mut indexes: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
                    let mut hashvec_1: metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>;
                    if '__try0: {
                        unwrap_break_err!(get(metamodelica::AsArg::as_arg(&key), hashTable), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    hval = hashFunc(metamodelica::AsArg::as_arg(&key))?;
                    indx = intMod(hval, bsize.clone());
                    newpos = valueArrayLength(metamodelica::AsArg::as_arg(&varr));
                    varr_1 = valueArrayAdd(metamodelica::AsArg::as_arg(&varr), v.clone())?;
                    indexes = ({let __elt = (*metamodelica::index_checked(&hashvec.borrow(), indx + 1)?).clone(); __elt});
                    hashvec_1 = metamodelica::arrayUpdate(hashvec.clone(), indx + 1, metamodelica::cons((key.clone(), newpos), indexes.clone()))?;
                    n_1 = valueArrayLength(&varr_1);
                    Ok(InstHierarchyHashTable { hashTable: hashvec_1.clone(), valueArr: varr_1.clone(), bucketSize: bsize.clone(), numberOfEntries: n_1 })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (newv @ (key, _), InstHierarchyHashTable { hashTable: hashvec, valueArr: varr, bucketSize: bsize, numberOfEntries: n }) => {
                    let mut indx: i32;
                    let mut varr_1: ValueArray;
                    (_, indx) = get1(metamodelica::AsArg::as_arg(&key), hashTable)?;
                    varr_1 = valueArraySetnth(varr.clone(), indx, newv.clone())?;
                    Ok(InstHierarchyHashTable { hashTable: hashvec.clone(), valueArr: varr_1.clone(), bucketSize: bsize.clone(), numberOfEntries: n.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("- InnerOuter.add failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outHashTable)
}

pub(crate) fn get(mut key: &Key, mut hashTable: &InstHierarchyHashTable) -> Result<Value> {
    let mut value: Value;
    (value, _) = get1(key, hashTable)?;
    Ok(value)
}

fn get1(mut key: &Key, mut hashTable: &InstHierarchyHashTable) -> Result<(Value, i32)> {
    let mut value: Value;
    let mut indx: i32;
    (value, indx) = (match hashTable.clone() {
        InstHierarchyHashTable {
            hashTable: mut hashvec,
            valueArr: mut varr,
            bucketSize: mut bsize,
            numberOfEntries: _,
        } => {
            let mut hval: i32;
            let mut hashindx: i32;
            let mut indexes: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
            let mut v: Value;
            let mut k: Key;
            hval = hashFunc(key)?;
            hashindx = intMod(hval, bsize.clone());
            indexes = ({
                let __elt = (*metamodelica::index_checked(&hashvec.borrow(), hashindx + 1)?).clone();
                __elt
            });
            indx = get2(key, &indexes)?;
            (k, v) = valueArrayNth(metamodelica::AsArg::as_arg(&varr), indx)?;
            let true = (keyEqual(&k, key)?) else {
                return Err("pattern mismatch");
            };
            (v, indx)
        }
    });
    Ok((value, indx))
}

fn get2(
    mut key: &Key,
    mut keyIndices: &metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
) -> Result<i32> {
    let mut index: i32 = 0;
    index = 'mc: {
        let __mc_input = &**keyIndices;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (key2, index), tail: _ } => {
                    let true = (keyEqual(key, metamodelica::AsArg::as_arg(&key2))?) else { return Err("pattern mismatch") };
                    Ok(index.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut index: i32 = index.clone();
                    index = get2(key, metamodelica::AsArg::as_arg(&xs))?;
                    Ok((index, index.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            index = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(index)
}

fn hashTableList(
    mut hashTable: &InstHierarchyHashTable,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>> {
    let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>;
    tplLst = (match hashTable.clone() {
        InstHierarchyHashTable { valueArr: mut varr, .. } => {
            tplLst = valueArrayList(metamodelica::AsArg::as_arg(&varr))?;
            tplLst
        }
    });
    Ok(tplLst)
}

fn valueArrayList(
    mut valueArray: &ValueArray,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>> {
    let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>;
    tplLst = 'mc: {
        let __mc_input = valueArray.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray {
                numberOfElements: 0, ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray {
                numberOfElements: 1,
                valueArray: mut arr,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut elt: (metamodelica::Ref<DAE::ComponentRef>, InstInner);
            let __pa0 = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&arr.borrow(), 0 + 1)?).clone(); __elt})) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            elt = metamodelica::Own::own(__pa0);
            Ok(list![elt.clone()])
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray {
                numberOfElements: mut n,
                valueArray: mut arr,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut lastpos: i32;
            let mut lst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>;
            lastpos = n.clone() - 1;
            lst = valueArrayList2(arr.clone(), 0, lastpos)?;
            Ok(lst.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(tplLst)
}

fn valueArrayList2(
    mut inVarOptionArray1: metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>>,
    mut inInteger2: i32,
    mut inInteger3: i32,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>> {
    let mut outVarLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>;
    outVarLst = 'mc: {
        let __mc_input = (inVarOptionArray1.clone(), inInteger2, inInteger3);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut arr, mut pos, mut lastpos) = __mc_input.clone() else {
                return Err("nomatch");
            };
            if !(pos.clone() == lastpos.clone()) {
                return Err("guard");
            }
            let mut v: (metamodelica::Ref<DAE::ComponentRef>, InstInner);
            let __pa0 = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&arr.borrow(), pos.clone() + 1)?).clone(); __elt})) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v = metamodelica::Own::own(__pa0);
            Ok(list![v.clone()])
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut arr, mut pos, mut lastpos) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut v: (metamodelica::Ref<DAE::ComponentRef>, InstInner);
            let mut pos_1: i32;
            let mut res: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>;
            pos_1 = pos.clone() + 1;
            let __pa0 = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&arr.borrow(), pos.clone() + 1)?).clone(); __elt})) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v = metamodelica::Own::own(__pa0);
            res = valueArrayList2(arr.clone(), pos_1, lastpos.clone())?;
            Ok(metamodelica::cons(v.clone(), res.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut arr, mut pos, mut lastpos) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut pos_1: i32;
            let mut res: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>;
            pos_1 = pos.clone() + 1;
            ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&arr.borrow(), pos.clone() + 1)?).clone(); __elt})) {
                None => (),
                _ => return Err("pattern mismatch"),
            } };
            res = valueArrayList2(arr.clone(), pos_1, lastpos.clone())?;
            Ok(res.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarLst)
}

fn valueArrayLength(mut valueArray: &ValueArray) -> i32 {
    let mut size: i32;
    size = (match valueArray.clone() {
        ValueArray {
            numberOfElements: mut __esc_size,
            ..
        } => {
            size = __esc_size.clone();
            size
        }
    });
    size
}

fn valueArrayAdd(
    mut valueArray: &ValueArray,
    mut entry: (metamodelica::Ref<DAE::ComponentRef>, InstInner),
) -> Result<ValueArray> {
    let mut outValueArray: ValueArray;
    outValueArray = 'mc: {
        let __mc_input = valueArray.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray {
                numberOfElements: mut n,
                valueArray: mut arr,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            if !(n.clone() < metamodelica::arrayLength(arr.clone())) {
                return Err("guard");
            }
            let mut n_1: i32;
            let mut arr_1: metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>>;
            n_1 = n.clone() + 1;
            arr_1 = metamodelica::arrayUpdate(arr.clone(), n.clone() + 1, Some(entry.clone()))?;
            Ok(ValueArray {
                numberOfElements: n_1,
                valueArray: arr_1.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray {
                numberOfElements: mut n,
                valueArray: mut arr,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            if !(n.clone() < metamodelica::arrayLength(arr.clone())) {
                return Err("guard");
            }
            let mut n_1: i32;
            let mut size: i32;
            let mut expandsize: i32;
            let mut expandsize_1: i32;
            let mut arr_1: metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>>;
            let mut arr_2: metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, InstInner)>>;
            let mut rsize: metamodelica::Real;
            let mut rexpandsize: metamodelica::Real;
            size = metamodelica::arrayLength(arr.clone());
            rsize = intReal(size);
            rexpandsize = rsize * metamodelica::OrderedFloat(0.4_f64);
            expandsize = ((rexpandsize).0.floor() as i32);
            expandsize_1 = intMax(expandsize, 1);
            arr_1 = Array::expand(expandsize_1, arr.clone(), None)?;
            n_1 = n.clone() + 1;
            arr_2 = metamodelica::arrayUpdate(arr_1.clone(), n.clone() + 1, Some(entry.clone()))?;
            Ok(ValueArray {
                numberOfElements: n_1,
                valueArray: arr_2.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("-InstHierarchyHashTable.valueArrayAdd failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValueArray)
}

fn valueArraySetnth(
    mut valueArray: ValueArray,
    mut pos: i32,
    mut entry: (metamodelica::Ref<DAE::ComponentRef>, InstInner),
) -> Result<ValueArray> {
    let mut outValueArray: ValueArray;
    outValueArray = 'mc: {
        let __mc_input = valueArray.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray {
                numberOfElements: _,
                valueArray: mut arr,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            if !(pos < metamodelica::arrayLength(arr.clone())) {
                return Err("guard");
            }
            metamodelica::arrayUpdate(arr.clone(), pos + 1, Some(entry.clone()))?;
            Ok(valueArray.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("-InstHierarchyHashTable.valueArraySetnth failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValueArray)
}

fn valueArrayClearnth(mut valueArray: ValueArray, mut pos: i32) -> Result<ValueArray> {
    let mut outValueArray: ValueArray;
    outValueArray = 'mc: {
        let __mc_input = valueArray.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray {
                numberOfElements: _,
                valueArray: mut arr,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            if !(pos < metamodelica::arrayLength(arr.clone())) {
                return Err("guard");
            }
            metamodelica::arrayUpdate(arr.clone(), pos + 1, None)?;
            Ok(valueArray.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("-InstHierarchyHashTable.valueArrayClearnth failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValueArray)
}

fn valueArrayNth(mut valueArray: &ValueArray, mut pos: i32) -> Result<(Key, Value)> {
    let mut key: Key;
    let mut value: Value;
    (key, value) = (match valueArray.clone() {
        ValueArray {
            numberOfElements: mut n,
            valueArray: mut arr,
        } => {
            let mut k: Key;
            let mut v: Value;
            let true = (pos < n.clone()) else {
                return Err("pattern mismatch");
            };
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&arr.borrow(), pos + 1)?).clone(); __elt})) {
                Some((__pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            k = metamodelica::Own::own(__pa0);
            v = metamodelica::Own::own(__pa1);
            (k, v)
        }
    });
    Ok((key, value))
}
