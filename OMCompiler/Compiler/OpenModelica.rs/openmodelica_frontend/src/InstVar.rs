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
use crate::HashSet;
use crate::InnerOuter;
use crate::Inst;
use crate::InstBinding;
use crate::InstDAE;
use crate::InstFunction;
use crate::InstSection;
use crate::InstUtil;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

pub type InstDims = metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;

pub(crate) fn instVar(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inState: ClassInf::State,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inIdent: ArcStr,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inIntegerLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImpl: bool,
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut info: &SourceInfo,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut componentDefinitionParentEnv: &FCore::Graph,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut io: Absyn::InnerOuter;
    if (::match_deref::match_deref! { match &(inIdent.clone()) {
        Deref @ "Integer" => true,
        Deref @ "Real" => true,
        Deref @ "Boolean" => true,
        Deref @ "String" => true,
        Deref @ "time" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
        Error::addSourceMessage(&(Error::RESERVED_IDENTIFIER.clone()), list![inIdent.clone()], info)?;
        return Err("fail");
    }
    io = SCodeUtil::prefixesInnerOuter(&inPrefixes);
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv,
            inIH,
            inStore,
            inState,
            inMod,
            inPrefix.clone(),
            inIdent,
            inClass,
            inAttributes,
            inPrefixes,
            inDimensionLst,
            inIntegerLst,
            inInstDims,
            inImpl,
            inComment,
            inGraph,
            inSets,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { name: typeName, .. }, attr, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut innerCompEnv: FCore::Graph;
                    let mut outerCompEnv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut outerDAE: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut fullName: ArcStr;
                    let mut typePath: metamodelica::Ref<Absyn::Path>;
                    let mut innerScope: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isOnlyInner(io)) else { return Err("pattern mismatch") };
                    (cache, innerCompEnv, ih, store, dae, csets, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    fullName = ComponentReferenceBasics::printComponentRefStr(&cref)?;
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), typeName.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    outerCompEnv = InnerOuter::switchInnerToOuterInGraph(innerCompEnv.clone(), cref.clone())?;
                    outerDAE = DAE::emptyDae().clone();
                    innerScope = FGraph::printGraphPathStr(componentDefinitionParentEnv);
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), metamodelica::AsArg::as_arg(&pre), io, &(InnerOuter::InstInner { innerPrefix: pre.clone(), name: n.clone(), io: io, fullName: fullName.clone(), typePath: typePath.clone(), scope: innerScope.clone(), instResult: Some(InnerOuter::InstResult { outCache: cache.clone(), outEnv: outerCompEnv.clone(), outStore: store.clone(), outDae: outerDAE.clone(), outSets: csets.clone(), outType: ty.clone(), outGraph: graph.clone() }), outers: metamodelica::nil(), innerElement: None }))?;
                    Ok((cache.clone(), innerCompEnv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s: ArcStr;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isOnlyOuter(io)) else { return Err("pattern mismatch") };
                    let false = (Mod::modEqual(metamodelica::AsArg::as_arg(&r#mod), &(openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))?) else { return Err("pattern mismatch") };
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    s1 = ComponentReferenceBasics::printComponentRefStr(&cref)?;
                    s2 = Mod::prettyPrintMod(metamodelica::AsArg::as_arg(&r#mod), 0)?;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::OUTER_MODIFICATION.clone()), list![s.clone()], info)?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info, graph.clone(), csets.clone(), componentDefinitionParentEnv)?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr @ SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, .. }, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isOnlyOuter(io)) else { return Err("pattern mismatch") };
                    let true = (Mod::modEqual(metamodelica::AsArg::as_arg(&r#mod), &(openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))?) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(InnerOuter::lookupInnerVar(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), pre.clone(), n.clone(), io)?) {
                        InnerOuter::InstInner { innerPrefix: _, name: _, io: _, fullName: _, typePath: _, scope: _, instResult: Some(InnerOuter::InstResult { outCache: __pa0, outEnv: __pa1, outStore: __pa2, outDae: _, outSets: _, outType: __pa3, outGraph: __pa4 }), outers: _, innerElement: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    compenv = metamodelica::Own::own(__pa1);
                    store = metamodelica::Own::own(__pa2);
                    ty = metamodelica::Own::own(__pa3);
                    graph = metamodelica::Own::own(__pa4);
                    topInstance = (ih).head().cloned()?;
                    let InnerOuter::TOP_INSTANCE { sm: __pa5, .. } = &topInstance;
                    sm = metamodelica::Own::own(__pa5);
                    let true = (BaseHashSet::currentSize(&sm) > 0) else { return Err("pattern mismatch") };
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), &sm)?) else { return Err("pattern mismatch") };
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((inCache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, _, r#mod, pre, n, _, _, _, _, _, _, _, _, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut outerDAE: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut innerPrefix: DAE::Prefix;
                    let mut crefOuter: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crefInner: metamodelica::Ref<DAE::ComponentRef>;
                    let mut outers: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut nInner: ArcStr;
                    let mut fullName: ArcStr;
                    let mut typePath: metamodelica::Ref<Absyn::Path>;
                    let mut innerScope: ArcStr;
                    let mut ioInner: Absyn::InnerOuter;
                    let mut instResult: Option<InnerOuter::InstResult>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isOnlyOuter(io)) else { return Err("pattern mismatch") };
                    let true = (Mod::modEqual(metamodelica::AsArg::as_arg(&r#mod), &(openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))?) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa12, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa13) = ::match_deref::match_deref! { match &(InnerOuter::lookupInnerVar(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), pre.clone(), n.clone(), io)?) {
                        InnerOuter::InstInner { innerPrefix: __pa0, name: __pa1, io: __pa2, fullName: __pa3, typePath: __pa4, scope: __pa5, instResult: __pa12 @ Some(InnerOuter::InstResult { outCache: __pa6, outEnv: __pa7, outStore: __pa8, outDae: __pa9, outSets: _, outType: __pa10, outGraph: __pa11 }), outers: __pa13, innerElement: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa12.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa13.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    innerPrefix = metamodelica::Own::own(__pa0);
                    nInner = metamodelica::Own::own(__pa1);
                    ioInner = metamodelica::Own::own(__pa2);
                    fullName = metamodelica::Own::own(__pa3);
                    typePath = metamodelica::Own::own(__pa4);
                    innerScope = metamodelica::Own::own(__pa5);
                    cache = metamodelica::Own::own(__pa6);
                    compenv = metamodelica::Own::own(__pa7);
                    store = metamodelica::Own::own(__pa8);
                    outerDAE = metamodelica::Own::own(__pa9);
                    ty = metamodelica::Own::own(__pa10);
                    graph = metamodelica::Own::own(__pa11);
                    instResult = metamodelica::Own::own(__pa12);
                    outers = metamodelica::Own::own(__pa13);
                    (cache, crefOuter) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    (cache, crefInner) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), innerPrefix.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    ih = InnerOuter::addOuterPrefixToIH(metamodelica::AsArg::as_arg(&ih), &crefOuter, crefInner.clone())?;
                    outers = List::unionElt(crefOuter.clone(), outers.clone());
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), &(innerPrefix.clone()), ioInner, &(InnerOuter::InstInner { innerPrefix: innerPrefix.clone(), name: nInner.clone(), io: ioInner, fullName: fullName.clone(), typePath: typePath.clone(), scope: innerScope.clone(), instResult: instResult.clone(), outers: outers.clone(), innerElement: None }))?;
                    outerDAE = DAE::emptyDae().clone();
                    Ok((inCache.clone(), compenv.clone(), ih.clone(), store.clone(), outerDAE.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut crefOuter: metamodelica::Ref<DAE::ComponentRef>;
                    let mut typeName: ArcStr;
                    let mut typePath: metamodelica::Ref<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isOnlyOuter(io)) else { return Err("pattern mismatch") };
                    let true = (Mod::modEqual(metamodelica::AsArg::as_arg(&r#mod), &(openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(InnerOuter::lookupInnerVar(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), pre.clone(), n.clone(), io)?) {
                        InnerOuter::InstInner { innerPrefix: _, name: _, io: _, fullName: _, typePath: __pa0, scope: _, instResult: None, outers: _, innerElement: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    typePath = metamodelica::Own::own(__pa0);
                    (cache, crefOuter) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    typeName = SCodeUtil::className(metamodelica::AsArg::as_arg(&cl))?;
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), typeName.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    if !(r#impl.clone() && listMember(pre.clone(), list![openmodelica_frontend_types::DAE::Prefix::NOPRE])) && !(Config::getGraphicsExpMode()?) {
                        s1 = ComponentReferenceBasics::printComponentRefStr(&crefOuter)?;
                        s2 = AbsynUtil::innerOuterStr(io);
                        s3 = InnerOuter::getExistingInnerDeclarations(metamodelica::AsArg::as_arg(&ih), componentDefinitionParentEnv)?;
                        s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(typePath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s1); ArcStr::from(__mm_s) };
                        Error::addSourceMessage(&(Error::MISSING_INNER_PREFIX.clone()), list![s1.clone(), s2.clone(), s3.clone()], info)?;
                    }
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut crefOuter: metamodelica::Ref<DAE::ComponentRef>;
                    let mut typeName: ArcStr;
                    let mut typePath: metamodelica::Ref<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isOnlyOuter(io)) else { return Err("pattern mismatch") };
                    let true = (Mod::modEqual(metamodelica::AsArg::as_arg(&r#mod), &(openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))?) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(InnerOuter::lookupInnerVar(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), pre.clone(), n.clone(), io), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (cache, crefOuter) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    typeName = SCodeUtil::className(metamodelica::AsArg::as_arg(&cl))?;
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), typeName.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    if !(r#impl.clone() && listMember(pre.clone(), list![openmodelica_frontend_types::DAE::Prefix::NOPRE])) && !(Config::getGraphicsExpMode()?) {
                        s1 = ComponentReferenceBasics::printComponentRefStr(&crefOuter)?;
                        s2 = AbsynUtil::innerOuterStr(io);
                        s3 = InnerOuter::getExistingInnerDeclarations(metamodelica::AsArg::as_arg(&ih), componentDefinitionParentEnv)?;
                        s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(typePath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s1); ArcStr::from(__mm_s) };
                        Error::addSourceMessage(&(Error::MISSING_INNER_PREFIX.clone()), list![s1.clone(), s2.clone(), s3.clone()], info)?;
                    }
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { name: typeName, .. }, attr @ SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, .. }, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut innerCompEnv: FCore::Graph;
                    let mut outerCompEnv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut innerDAE: DAE::DAElist;
                    let mut csetsInner: DAE::Connect::Sets;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut fullName: ArcStr;
                    let mut typePath: metamodelica::Ref<Absyn::Path>;
                    let mut innerScope: ArcStr;
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isInnerOuter(io)) else { return Err("pattern mismatch") };
                    topInstance = (ih).head().cloned()?;
                    let InnerOuter::TOP_INSTANCE { sm: __pa0, .. } = &topInstance;
                    sm = metamodelica::Own::own(__pa0);
                    let true = (BaseHashSet::currentSize(&sm) > 0) else { return Err("pattern mismatch") };
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), &sm)?) else { return Err("pattern mismatch") };
                    (cache, innerCompEnv, ih, store, dae, csetsInner, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    fullName = ComponentReferenceBasics::printComponentRefStr(&cref)?;
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), typeName.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    outerCompEnv = InnerOuter::switchInnerToOuterInGraph(innerCompEnv.clone(), cref.clone())?;
                    innerDAE = dae.clone();
                    innerScope = FGraph::printGraphPathStr(componentDefinitionParentEnv);
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), metamodelica::AsArg::as_arg(&pre), io, &(InnerOuter::InstInner { innerPrefix: pre.clone(), name: n.clone(), io: io, fullName: fullName.clone(), typePath: typePath.clone(), scope: innerScope.clone(), instResult: Some(InnerOuter::InstResult { outCache: cache.clone(), outEnv: outerCompEnv.clone(), outStore: store.clone(), outDae: innerDAE.clone(), outSets: csetsInner.clone(), outType: ty.clone(), outGraph: graph.clone() }), outers: metamodelica::nil(), innerElement: None }))?;
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csetsInner.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { name: typeName, .. }, attr, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut innerCompEnv: FCore::Graph;
                    let mut outerCompEnv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut outerDAE: DAE::DAElist;
                    let mut innerDAE: DAE::DAElist;
                    let mut csetsInner: DAE::Connect::Sets;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut fullName: ArcStr;
                    let mut typePath: metamodelica::Ref<Absyn::Path>;
                    let mut innerScope: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut pf = (*pf).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isInnerOuter(io)) else { return Err("pattern mismatch") };
                    (cache, innerCompEnv, ih, store, dae, csetsInner, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    fullName = ComponentReferenceBasics::printComponentRefStr(&cref)?;
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), typeName.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    outerCompEnv = InnerOuter::switchInnerToOuterInGraph(innerCompEnv.clone(), cref.clone())?;
                    innerDAE = dae.clone();
                    innerScope = FGraph::printGraphPathStr(componentDefinitionParentEnv);
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), metamodelica::AsArg::as_arg(&pre), io, &(InnerOuter::InstInner { innerPrefix: pre.clone(), name: n.clone(), io: io, fullName: fullName.clone(), typePath: typePath.clone(), scope: innerScope.clone(), instResult: Some(InnerOuter::InstResult { outCache: cache.clone(), outEnv: outerCompEnv.clone(), outStore: store.clone(), outDae: innerDAE.clone(), outSets: csetsInner.clone(), outType: ty.clone(), outGraph: graph.clone() }), outers: metamodelica::nil(), innerElement: None }))?;
                    pf = SCodeUtil::prefixesSetInnerOuter(pf.clone(), openmodelica_ast::Absyn::InnerOuter::OUTER);
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info, graph.clone(), csets.clone(), componentDefinitionParentEnv)?;
                    outerDAE = dae.clone();
                    dae = DAEUtil::joinDaes(&outerDAE, &innerDAE)?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csetsInner.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isNotInnerOuter(io)) else { return Err("pattern mismatch") };
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, r#mod, pre, n, cl, _, _, _, _, _, _, _, _, _) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstVar.instVar failed while instatiating variable: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cref)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Mod::prettyPrintMod(metamodelica::AsArg::as_arg(&r#mod), 0)?); __mm_s.push_str(&*literal!("\nin scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); __mm_s.push_str(&*literal!(" class:\n")); __mm_s.push_str(&*SCodeDump::unparseElementStr(cl.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn instVar_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inState: ClassInf::State,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inName: ArcStr,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inIndices: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImpl: bool,
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut inInfo: SourceInfo,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut type_mods: metamodelica::Ref<DAE::Mod>;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let mut attr: SCode::Attributes;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    match '__try0: {
        unwrap_break_err!(Error::updateCurrentComponent(inName.clone(), inInfo.clone(), (std::sync::Arc::new({ let __pe_b1 = inPrefix.clone(); move |__pe_a0| PrefixUtil::identAndPrefixToPath(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>)), '__try0);
        (outCache, dims, cls, type_mods) = unwrap_break_err!(InstUtil::getUsertypeDimensions(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone(), inImpl), '__try0);
        if (dims).is_empty() {
            dims = inDimensions.clone();
            cls = inClass.clone();
            r#mod = inMod.clone();
            attr = inAttributes.clone();
        } else {
            type_mods = liftUserTypeMod(type_mods.clone(), inDimensions.clone());
            dims = listAppend(inDimensions.clone(), dims.clone());
            r#mod = unwrap_break_err!(Mod::merge(inMod.clone(), type_mods.clone(), literal!(""), true), '__try0);
            attr = InstUtil::propagateClassPrefix(inAttributes.clone(), &inPrefix);
        }
        (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = unwrap_break_err!(instVar2(outCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), r#mod.clone(), inPrefix.clone(), inName.clone(), cls.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), inIndices.clone(), inInstDims.clone(), inImpl, inComment.clone(), inInfo.clone(), inGraph.clone(), inSets.clone()), '__try0);
        source = ElementSource::createElementSource(
            inInfo.clone(),
            unwrap_break_err!(FGraph::getScopePath(&inEnv), '__try0),
            &inPrefix,
            (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
        );
        (outCache, outDae) = addArrayVarEquation(
            outCache.clone(),
            inEnv.clone(),
            &outIH,
            &inState,
            outDae.clone(),
            outType.clone(),
            &r#mod,
            Types::variabilityToConst(SCodeUtil::attrVariability(&attr)),
            inPrefix.clone(),
            inName.clone(),
            source.clone(),
        );
        outCache = InstFunction::addRecordConstructorFunction(
            outCache.clone(),
            &inEnv,
            &(Types::arrayElementType(&outType)),
            &(SCodeUtil::elementInfo(&inClass)),
        );
        unwrap_break_err!(Error::clearCurrentComponent(), '__try0);
        Ok::<_, &'static str>((
            attr.clone(),
            cls.clone(),
            dims.clone(),
            r#mod.clone(),
            outCache.clone(),
            outDae.clone(),
            outEnv.clone(),
            outGraph.clone(),
            outIH.clone(),
            outSets.clone(),
            outStore.clone(),
            outType.clone(),
            source.clone(),
            type_mods.clone(),
        ))
    } {
        Ok((
            __try0_o0,
            __try0_o1,
            __try0_o2,
            __try0_o3,
            __try0_o4,
            __try0_o5,
            __try0_o6,
            __try0_o7,
            __try0_o8,
            __try0_o9,
            __try0_o10,
            __try0_o11,
            __try0_o12,
            __try0_o13,
        )) => {
            attr = __try0_o0;
            cls = __try0_o1;
            dims = __try0_o2;
            r#mod = __try0_o3;
            outCache = __try0_o4;
            outDae = __try0_o5;
            outEnv = __try0_o6;
            outGraph = __try0_o7;
            outIH = __try0_o8;
            outSets = __try0_o9;
            outStore = __try0_o10;
            outType = __try0_o11;
            source = __try0_o12;
            type_mods = __try0_o13;
        }
        Err(__try0_err) => {
            Error::clearCurrentComponent()?;
            return Err(__try0_err);
        }
    }
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn liftUserTypeMod(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = inMod;
    if (inDims).is_empty() {
        return outMod;
    }
    outMod = 'mc: {
        let __mc_input = outMod.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Mod::MOD { .. } => {
                            let mut outMod: metamodelica::Ref<DAE::Mod> = outMod.clone();
                            if !(SCodeUtil::eachBool(var_field!((*outMod).eachPrefix, DAE::Mod::MOD).clone())) {
                                assign_variant_field!(outMod => DAE::Mod::MOD;
                                    binding = liftUserTypeEqMod(var_field!((*outMod).binding, DAE::Mod::MOD).clone(), inDims.clone())?,
                                    subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
                for mut s in (var_field!((*outMod).subModLst, DAE::Mod::MOD).clone()).into_iter().cloned() {
                            let __x = liftUserTypeSubMod(s.clone(), inDims.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                                );
                            }
                            Ok((outMod.clone(), outMod.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outMod = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(outMod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outMod
}

fn liftUserTypeSubMod(
    mut inSubMod: metamodelica::Ref<DAE::SubMod>,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::Ref<DAE::SubMod> {
    let mut outSubMod: metamodelica::Ref<DAE::SubMod> = inSubMod;
    outSubMod = (match &*outSubMod {
        DAE::SubMod { .. } => {
            assign_field!(outSubMod.r#mod = liftUserTypeMod(outSubMod.r#mod.clone(), inDims));
            outSubMod
        }
    });
    outSubMod
}

fn liftUserTypeEqMod(
    mut inEqMod: Option<DAE::EqMod>,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<Option<DAE::EqMod>> {
    let mut outEqMod: Option<DAE::EqMod>;
    let mut eq: DAE::EqMod;
    let mut ty: metamodelica::Ref<DAE::Type>;
    if (inEqMod).is_none() {
        outEqMod = inEqMod;
        return Ok(outEqMod);
    }
    let __pa0 = ::match_deref::match_deref! { match &(inEqMod) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    eq = metamodelica::Own::own(__pa0);
    eq = (match eq.clone() {
        DAE::EqMod::TYPED { .. } => {
            let __owned_variant_modifierAsExp_0 =
                Expression::liftExpList(var_field!(eq.modifierAsExp, DAE::EqMod::TYPED).clone(), inDims.clone())?;
            let __owned_variant_modifierAsValue_1 = Util::applyOption1(
                var_field!(eq.modifierAsValue, DAE::EqMod::TYPED).clone(),
                &ValuesUtil::liftValueList,
                inDims.clone(),
            )?;
            if let DAE::EqMod::TYPED {
                modifierAsExp,
                modifierAsValue,
                ..
            } = &mut eq
            {
                *modifierAsExp = __owned_variant_modifierAsExp_0;
                *modifierAsValue = __owned_variant_modifierAsValue_1;
            } else {
                panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED");
            }
            ty = Types::getPropType(var_field!(eq.properties, DAE::EqMod::TYPED));
            let __owned_variant_properties_0 = Types::setPropType(
                var_field!(eq.properties, DAE::EqMod::TYPED),
                Types::liftArrayListDims(ty, inDims),
            );
            if let DAE::EqMod::TYPED { properties, .. } = &mut eq {
                *properties = __owned_variant_properties_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED");
            }
            eq
        }
        _ => eq,
    });
    outEqMod = Some(eq);
    Ok(outEqMod)
}

fn addArrayVarEquation(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inState: &ClassInf::State,
    mut inDae: DAE::DAElist,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut r#mod: &metamodelica::Ref<DAE::Mod>,
    mut r#const: DAE::Const,
    mut pre: DAE::Prefix,
    mut n: ArcStr,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> (FCore::Cache, DAE::DAElist) {
    let mut outCache: FCore::Cache;
    let mut outDae: DAE::DAElist;
    (outCache, outDae) = 'mc: {
        let __mc_input = (inDae.clone(), r#const);
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (Config::scalarizeBindings()?) else {
                return Err("pattern mismatch");
            };
            Ok((inCache.clone(), inDae.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::DAElist { elementLst: ref dae }, DAE::Const::C_VAR { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cache: FCore::Cache;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut eq: metamodelica::Ref<DAE::Element>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let false = (ClassInfUtil::isFunctionOrRecord(inState)) else {
                return Err("pattern mismatch");
            };
            ty = Types::simplifyType(inType.clone())?;
            let false = (Types::isExternalObject(&(Types::arrayElementType(&ty)))) else {
                return Err("pattern mismatch");
            };
            let false = (Types::isComplexType(&(Types::arrayElementType(&ty)))) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(TypesDump::getDimensions(&ty)) {
                __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dims = metamodelica::Own::own(__pa0);
            let __pa1 = ::match_deref::match_deref! { match &(InstBinding::makeVariableBinding(ty.clone(), r#mod, r#const.clone(), pre.clone(), n.clone())?) {
                Some(__pa1) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            exp = metamodelica::Own::own(__pa1);
            cr = ComponentReferenceBasics::makeCrefIdent(n.clone(), ty.clone(), metamodelica::nil());
            (cache, cr) = PrefixUtil::prefixCref(inCache.clone(), inEnv.clone(), inIH, pre.clone(), cr.clone())?;
            eq = metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION {
                dimension: dims.clone(),
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: cr.clone(),
                    ty: ty.clone(),
                }),
                array: exp.clone(),
                source: source.clone(),
            });
            Ok((
                cache.clone(),
                DAE::DAElist {
                    elementLst: metamodelica::cons(eq.clone(), dae.clone()),
                },
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((inCache.clone(), inDae.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outDae)
}

fn instVar2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inState: ClassInf::State,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inName: ArcStr,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImpl: bool,
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut inInfo: SourceInfo,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv.clone(),
            inIH.clone(),
            inStore.clone(),
            inState.clone(),
            inMod.clone(),
            inPrefix.clone(),
            inName.clone(),
            inClass.clone(),
            inAttributes.clone(),
            inPrefixes.clone(),
            inDimensions,
            inSubscripts.clone(),
            inInstDims.clone(),
            inImpl,
            inComment.clone(),
            inInfo.clone(),
            inGraph.clone(),
            inSets.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: None, .. }, pre, n, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, attr, pf, dims, _, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut env_1: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty_1: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty_2: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut dae_var_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                    let mut vis: SCode::Visibility;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut fin: SCode::Final;
                    let mut io: Absyn::InnerOuter;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (ClassInfUtil::isFunction(metamodelica::AsArg::as_arg(&ci_state))) else { return Err("pattern mismatch") };
                    InstUtil::checkFunctionVar(n.clone(), metamodelica::AsArg::as_arg(&attr), metamodelica::AsArg::as_arg(&pf), metamodelica::AsArg::as_arg(&info))?;
                    (cache, env_1, ih, store, _, csets, ty, _, _, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), cl.clone(), inst_dims.clone(), r#impl.clone(), openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, graph.clone(), metamodelica::AsArg::as_arg(&csets))?;
                    ty_1 = InstUtil::makeArrayType(metamodelica::AsArg::as_arg(&dims), ty.clone())?;
                    InstUtil::checkFunctionVarType(ty_1.clone(), metamodelica::AsArg::as_arg(&ci_state), n.clone(), metamodelica::AsArg::as_arg(&info))?;
                    (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), metamodelica::AsArg::as_arg(&env), r#mod.clone(), ty.clone(), metamodelica::nil())?;
                    ty_2 = Types::simplifyType(ty_1.clone())?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), ty_2.clone(), metamodelica::nil()))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(InstBinding::makeBinding(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&attr), metamodelica::AsArg::as_arg(&r#mod), ty_2.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&n), metamodelica::AsArg::as_arg(&info))?) {
                        (__pa0, Deref @ DAE::Binding::EQBOUND { exp: __pa1, evaluatedExp: _, constant_: _, source: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e = metamodelica::Own::own(__pa1);
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                    let __arc6 = pf.clone();
                    let SCode::PREFIXES { visibility: __pa3, finalPrefix: __pa4, innerOuter: __pa5, .. } = &*__arc6;
                    vis = metamodelica::Own::own(__pa3);
                    fin = metamodelica::Own::own(__pa4);
                    io = metamodelica::Own::own(__pa5);
                    dae = InstDAE::daeDeclare(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &env_1, cr.clone(), ci_state.clone(), ty.clone(), metamodelica::AsArg::as_arg(&attr), vis, Some(e.clone()), list![dims.clone()], None, dae_var_attr.clone(), Some(comment.clone()), io, fin, &source, true)?;
                    store = UnitAbsynBuilder::instAddStore(store.clone(), &ty, &cr);
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: Some(_), .. }, pre, n, cl, attr, pf, dims, _, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut p: DAE::Properties;
                    let mut env_1: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty_1: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty_2: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut dae_var_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                    let mut vis: SCode::Visibility;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut fin: SCode::Final;
                    let mut io: Absyn::InnerOuter;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (ClassInfUtil::isFunction(metamodelica::AsArg::as_arg(&ci_state))) else { return Err("pattern mismatch") };
                    InstUtil::checkFunctionVar(n.clone(), metamodelica::AsArg::as_arg(&attr), metamodelica::AsArg::as_arg(&pf), metamodelica::AsArg::as_arg(&info))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mod::modEquation(metamodelica::AsArg::as_arg(&r#mod))) {
                        Some(DAE::EqMod::TYPED { modifierAsExp: __pa0, modifierAsValue: _, properties: __pa1, modifierAsAbsynExp: _, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    p = metamodelica::Own::own(__pa1);
                    (cache, env_1, ih, store, _, csets, ty, _, _, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), cl.clone(), inst_dims.clone(), r#impl.clone(), openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, graph.clone(), metamodelica::AsArg::as_arg(&csets))?;
                    ty_1 = InstUtil::makeArrayType(metamodelica::AsArg::as_arg(&dims), ty.clone())?;
                    InstUtil::checkFunctionVarType(ty_1.clone(), metamodelica::AsArg::as_arg(&ci_state), n.clone(), metamodelica::AsArg::as_arg(&info))?;
                    (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), metamodelica::AsArg::as_arg(&env), r#mod.clone(), ty.clone(), metamodelica::nil())?;
                    (e_1, _) = Types::matchProp(e.clone(), &p, &(DAE::Properties::PROP { type_: ty_1.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }), true)?;
                    ty_2 = Types::simplifyType(ty_1.clone())?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), ty_2.clone(), metamodelica::nil()))?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                    let __arc5 = pf.clone();
                    let SCode::PREFIXES { visibility: __pa2, finalPrefix: __pa3, innerOuter: __pa4, .. } = &*__arc5;
                    vis = metamodelica::Own::own(__pa2);
                    fin = metamodelica::Own::own(__pa3);
                    io = metamodelica::Own::own(__pa4);
                    dae = InstDAE::daeDeclare(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &env_1, cr.clone(), ci_state.clone(), ty.clone(), metamodelica::AsArg::as_arg(&attr), vis, Some(e_1.clone()), list![dims.clone()], None, dae_var_attr.clone(), Some(comment.clone()), io, fin, &source, true)?;
                    store = UnitAbsynBuilder::instAddStore(store.clone(), &ty, &cr);
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { .. }, attr, pf, dims, _, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut env_1: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut arrty: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut dae: DAE::DAElist;
                    let mut dae_var_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                    let mut vis: SCode::Visibility;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut fin: SCode::Final;
                    let mut io: Absyn::InnerOuter;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut csets = (*csets).clone();
                    let true = (ClassInfUtil::isFunction(metamodelica::AsArg::as_arg(&ci_state))) else { return Err("pattern mismatch") };
                    InstUtil::checkFunctionVar(n.clone(), metamodelica::AsArg::as_arg(&attr), metamodelica::AsArg::as_arg(&pf), metamodelica::AsArg::as_arg(&info))?;
                    (cache, env_1, ih, store, _, csets, ty, _, _, _) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), r#mod.clone(), pre.clone(), cl.clone(), inst_dims.clone(), r#impl.clone(), openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), metamodelica::AsArg::as_arg(&csets))?;
                    arrty = InstUtil::makeArrayType(metamodelica::AsArg::as_arg(&dims), ty.clone())?;
                    InstUtil::checkFunctionVarType(arrty.clone(), metamodelica::AsArg::as_arg(&ci_state), n.clone(), metamodelica::AsArg::as_arg(&info))?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), arrty.clone(), metamodelica::nil()))?;
                    (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), metamodelica::AsArg::as_arg(&env), r#mod.clone(), ty.clone(), metamodelica::nil())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                    let __arc3 = pf.clone();
                    let SCode::PREFIXES { visibility: __pa0, finalPrefix: __pa1, innerOuter: __pa2, .. } = &*__arc3;
                    vis = metamodelica::Own::own(__pa0);
                    fin = metamodelica::Own::own(__pa1);
                    io = metamodelica::Own::own(__pa2);
                    dae = InstDAE::daeDeclare(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &env_1, cr.clone(), ci_state.clone(), ty.clone(), metamodelica::AsArg::as_arg(&attr), vis, None, list![dims.clone()], None, dae_var_attr.clone(), Some(comment.clone()), io, fin, &source, true)?;
                    store = UnitAbsynBuilder::instAddStore(store.clone(), &ty, &cr);
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), arrty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, Deref @ metamodelica::ListNode::Nil, _, _, _, _, _, _, _) => {
                    let mut env: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut cache: FCore::Cache;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy;
                    let mut store: UnitAbsyn::InstStore;
                    let false = (ClassInfUtil::isFunction(&inState)) else { return Err("pattern mismatch") };
                    (cache, env, ih, store, dae, csets, ty, graph) = instScalar(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), inMod.clone(), inPrefix.clone(), inName.clone(), inClass.clone(), inAttributes.clone(), inPrefixes.clone(), inSubscripts.clone(), inInstDims.clone(), inImpl, Some(inComment.clone()), inInfo.clone(), inGraph.clone(), &inSets)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }, pre, n, cl, attr, pf, Deref @ metamodelica::ListNode::Cons { head: dim @ Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims;
                    let mut compenv: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut dim2: metamodelica::Ref<DAE::Dimension>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(metamodelica::AsArg::as_arg(&ci_state))) else { return Err("pattern mismatch") };
                    dim2 = InstUtil::instWholeDimFromMod(metamodelica::AsArg::as_arg(&dim), metamodelica::AsArg::as_arg(&r#mod), n.clone(), metamodelica::AsArg::as_arg(&info))?;
                    inst_dims_1 = List::appendLastList(metamodelica::AsArg::as_arg(&inst_dims), list![dim2.clone()])?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instArray(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), &((cl.clone(), attr.clone())), pf.clone(), 1, dim2.clone(), dims.clone(), idxs.clone(), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::liftNonBasicTypes(ty.clone(), dim2.clone());
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }, pre, n, cl, attr, pf, Deref @ metamodelica::ListNode::Cons { head: dim @ Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims;
                    let mut compenv: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut dim2: metamodelica::Ref<DAE::Dimension>;
                    let mut dime2: metamodelica::Ref<DAE::Subscript>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let false = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(metamodelica::AsArg::as_arg(&ci_state))) else { return Err("pattern mismatch") };
                    dim2 = InstUtil::instWholeDimFromMod(metamodelica::AsArg::as_arg(&dim), metamodelica::AsArg::as_arg(&r#mod), n.clone(), metamodelica::AsArg::as_arg(&info))?;
                    inst_dims_1 = List::appendLastList(metamodelica::AsArg::as_arg(&inst_dims), list![dim2.clone()])?;
                    dime2 = Expression::dimensionSubscript(&dim2)?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), metamodelica::cons(dime2.clone(), idxs.clone()), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::liftNonBasicTypes(ty.clone(), dim2.clone());
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims;
                    let mut compenv: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(metamodelica::AsArg::as_arg(&ci_state))) else { return Err("pattern mismatch") };
                    inst_dims_1 = List::appendLastList(metamodelica::AsArg::as_arg(&inst_dims), list![dim.clone()])?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instArray(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), &((cl.clone(), attr.clone())), pf.clone(), 1, dim.clone(), dims.clone(), idxs.clone(), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::liftNonBasicTypes(ty.clone(), dim.clone());
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims;
                    let mut compenv: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut dime: metamodelica::Ref<DAE::Subscript>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let false = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(metamodelica::AsArg::as_arg(&ci_state))) else { return Err("pattern mismatch") };
                    inst_dims_1 = List::appendLastList(metamodelica::AsArg::as_arg(&inst_dims), list![dim.clone()])?;
                    dime = Expression::dimensionSubscript(metamodelica::AsArg::as_arg(&dim))?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), metamodelica::cons(dime.clone(), idxs.clone()), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, Deref @ DAE::Mod::NOMOD { .. }, _, n, _, _, _, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: _ }, _, _, _, _, info, _, _) => {
                    Error::addSourceMessage(&(Error::FAILURE_TO_DEDUCE_DIMS_NO_MOD.clone()), list![ArcStr::from(::std::format!("{}", ((inSubscripts).len() as i32) + 1)), n.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, r#mod, pre, n, _, _, _, _, _, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstVar.instVar2 failed: ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(metamodelica::AsArg::as_arg(&pre))?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*n); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*Mod::prettyPrintMod(metamodelica::AsArg::as_arg(&r#mod), 0)?); __mm_s.push_str(&*literal!(")\n  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

pub(crate) fn instScalar(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inState: ClassInf::State,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inName: ArcStr,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImpl: bool,
    mut inComment: Option<metamodelica::Ref<SCode::Comment>>,
    mut inInfo: SourceInfo,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: &DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = ({
        let mut implicitInstantiation: bool = false;
        let mut inStateAndClassNameIsEqual: bool = false;
        'mc: {
            let __mc_input = (
                inCache,
                inEnv.clone(),
                inIH,
                inStore,
                inMod.clone(),
                &*inClass,
                &inAttributes,
                &*inPrefixes,
                inSubscripts.clone(),
            );
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, ih, store, r#mod, Deref @ SCode::Element::CLASS { name: cls_name, restriction: res, .. }, SCode::Attributes { variability: vt, .. }, Deref @ SCode::Prefixes { visibility: vis, finalPrefix: fin, innerOuter: io, .. }, idxs) => {
                        let mut ci_state: ClassInf::State;
                        let mut csets: DAE::Connect::Sets;
                        let mut graph: ConnectionGraph::ConnectionGraph;
                        let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                        let mut dae: DAE::DAElist;
                        let mut dae1: DAE::DAElist;
                        let mut dae2: DAE::DAElist;
                        let mut source: metamodelica::Ref<DAE::ElementSource>;
                        let mut pre: DAE::Prefix;
                        let mut start: Option<metamodelica::Ref<DAE::Exp>>;
                        let mut ident_ty: metamodelica::Ref<DAE::Type>;
                        let mut ty: metamodelica::Ref<DAE::Type>;
                        let mut env_1: FCore::Graph;
                        let mut opt_binding: Option<metamodelica::Ref<DAE::Exp>>;
                        let mut dae_var_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                        let mut opt_attr: Option<SCode::Attributes>;
                        let mut attr: SCode::Attributes;
                        let mut classWithElementsRemoved: metamodelica::Ref<SCode::Element>;
                        let mut stateName: ArcStr;
                        let mut predims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                        let mut cache = (*cache).clone();
                        let mut ih = (*ih).clone();
                        let mut store = (*store).clone();
                        let mut r#mod = (*r#mod).clone();
                        let mut idxs = (*idxs).clone();
                        idxs = idxs.clone().reverse();
                        ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&res), metamodelica::Ref::new(Absyn::Path::IDENT { name: cls_name.clone() }))?;
                        predims = List::lastListOrEmpty(&inInstDims);
                        pre = PrefixUtil::prefixAdd(inName.clone(), predims.clone(), idxs.clone(), &inPrefix, vt.clone(), ci_state.clone(), inInfo.clone())?;
                        if Config::acceptMetaModelicaGrammar()? {
                            stateName = AbsynUtil::pathString(ClassInfUtil::getStateName(&inState), literal!(""), true, false)?;
                            inStateAndClassNameIsEqual = stringEqual(&stateName, &cls_name);
                            implicitInstantiation = SCodeUtil::isUniontype(&inClass) && SCodeUtil::isConstant(inAttributes.variability.clone()) && inStateAndClassNameIsEqual;
                            if implicitInstantiation {
                                classWithElementsRemoved = SCodeUtil::setClassDef(metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), inClass.clone())?;
                                (_, env_1, ih, store, dae1, csets, ty, _, opt_attr, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), inMod.clone(), pre.clone(), classWithElementsRemoved.clone(), inInstDims.clone(), inImpl, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, inGraph.clone(), inSets)?;
                            } else {
                                (cache, env_1, ih, store, dae1, csets, ty, _, opt_attr, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), inMod.clone(), pre.clone(), inClass.clone(), inInstDims.clone(), inImpl, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, inGraph.clone(), inSets)?;
                            }
                        } else {
                            (cache, env_1, ih, store, dae1, csets, ty, _, opt_attr, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), inMod.clone(), pre.clone(), inClass.clone(), inInstDims.clone(), inImpl, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, inGraph.clone(), inSets)?;
                        }
                        (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), &env_1, inMod.clone(), ty.clone(), metamodelica::nil())?;
                        attr = InstUtil::propagateAbSCDirection(vt.clone(), inAttributes.clone(), opt_attr.clone(), &inInfo)?;
                        attr = SCodeUtil::removeAttributeDimensions(attr.clone());
                        ident_ty = InstUtil::makeCrefBaseType(ty.clone(), &inInstDims)?;
                        cr = ComponentReferenceBasics::makeCrefIdent(inName.clone(), ident_ty.clone(), idxs.clone());
                        (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), inPrefix.clone(), cr.clone())?;
                        InstUtil::checkModificationOnOuter(metamodelica::AsArg::as_arg(&cache), &env_1, metamodelica::AsArg::as_arg(&ih), inPrefix.clone(), &inName, &cr, &inMod, vt.clone(), io.clone(), inImpl, &inInfo)?;
                        source = ElementSource::createElementSource(inInfo.clone(), FGraph::getScopePath(&env_1)?, &inPrefix, (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                        r#mod = if (!((inSubscripts).is_empty()) && !(SCodeUtil::isParameterOrConst(vt.clone())) && !(ClassInfUtil::isFunctionOrRecord(&inState)) && !(Types::isComplexType(&(Types::arrayElementType(&ty)))) && !(Types::isExternalObject(&(Types::arrayElementType(&ty)))) && !(Config::scalarizeBindings()?)) {openmodelica_frontend_types::DAE::Mod::interned_NOMOD()} else {inMod.clone()};
                        opt_binding = InstBinding::makeVariableBinding(ty.clone(), metamodelica::AsArg::as_arg(&r#mod), Types::variabilityToConst(vt.clone()), inPrefix.clone(), inName.clone())?;
                        start = InstBinding::instStartBindingExp(inMod.clone(), &ty, vt.clone())?;
                        if !(Flags::getConfigBool(Flags::USE_LOCAL_DIRECTION.clone())?) {
                            attr = stripVarAttrDirection(&cr, metamodelica::AsArg::as_arg(&ih), &inState, inPrefix.clone(), attr.clone());
                        }
                        dae1 = InstUtil::propagateAttributes(dae1.clone(), attr.clone(), inPrefixes.clone(), inInfo.clone())?;
                        dae2 = InstDAE::daeDeclare(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &env_1, cr.clone(), inState.clone(), ty.clone(), &attr, vis.clone(), opt_binding.clone(), inInstDims.clone(), start.clone(), dae_var_attr.clone(), inComment.clone(), io.clone(), fin.clone(), &source, false)?;
                        store = UnitAbsynBuilder::instAddStore(store.clone(), &ty, &cr);
                        dae = instScalar2(cr.clone(), ty.clone(), vt.clone(), &inMod, &dae2, dae1.clone(), source.clone(), inImpl)?;
                        Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
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
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instScalar failed on ")); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!(" in scope ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(&inPrefix)?); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(&inEnv)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                        Ok(return Err("fail"))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    });
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn stripVarAttrDirection(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut ih: &InstanceHierarchy,
    mut inState: &ClassInf::State,
    mut inPrefix: DAE::Prefix,
    mut inAttributes: SCode::Attributes,
) -> SCode::Attributes {
    let mut outAttributes: SCode::Attributes;
    outAttributes = 'mc: {
        let __mc_input = (&**inCref, inState, &inAttributes);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, .. }) => {
                    Ok(inAttributes.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, _) => {
                    Ok(inAttributes.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::CONNECTOR { .. }, _) => {
                    if !((ConnectUtil::faceEqual(ConnectUtil::componentFaceType(inCref)?, openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE))) { return Err("guard") }
                    Ok(inAttributes.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    topInstance = (ih).head().cloned()?;
                    let InnerOuter::TOP_INSTANCE { sm: __pa0, .. } = &topInstance;
                    sm = metamodelica::Own::own(__pa0);
                    let true = (BaseHashSet::currentSize(&sm) > 0) else { return Err("pattern mismatch") };
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), &sm)?) else { return Err("pattern mismatch") };
                    Ok(inAttributes.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(SCodeUtil::setAttributesDirection(inAttributes.clone(), openmodelica_ast::Absyn::Direction::BIDIR))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outAttributes
}

fn instScalar2(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inVariability: SCode::Variability,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inDae: &DAE::DAElist,
    mut inClassDae: DAE::DAElist,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inImpl: bool,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inType.clone(), inVariability, inMod.clone())) {
        (_, SCode::Variability::CONST { .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }) => {
            let mut dae: DAE::DAElist;
            dae = DAEUtil::joinDaes(&inClassDae, inDae)?;
            dae
        },
        (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, .. }), .. }) => {
            let mut dae: DAE::DAElist;
            dae = InstBinding::instModEquation(inCref, inType, inMod, inSource, inImpl)?;
            dae = InstUtil::moveBindings(dae, inClassDae)?;
            dae = DAEUtil::joinDaes(&dae, inDae)?;
            dae
        },
        (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, .. }, .. }), .. }) => {
            let mut dae: DAE::DAElist;
            dae = InstBinding::instModEquation(inCref, inType, inMod, inSource, inImpl)?;
            dae = InstUtil::moveBindings(dae, inClassDae)?;
            dae = DAEUtil::joinDaes(&dae, inDae)?;
            dae
        },
        (_, SCode::Variability::PARAM { .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }) => {
            let mut dae: DAE::DAElist;
            dae = InstBinding::instModEquation(inCref, inType, inMod, inSource, inImpl)?;
            dae = InstUtil::propagateBinding(inClassDae, dae)?;
            dae = DAEUtil::joinDaes(&dae, inDae)?;
            dae
        },
        _ => {
            let mut dae: DAE::DAElist;
            let mut cls_dae: DAE::DAElist;
            dae = if (Types::isComplexType(&inType)) {InstBinding::instModEquation(inCref, inType.clone(), inMod, inSource, inImpl)?} else {DAE::emptyDae().clone()};
            cls_dae = stripRecordDefaultBindingsFromDAE(inClassDae, &inType, &dae)?;
            dae = DAEUtil::joinDaes(&dae, inDae)?;
            dae = DAEUtil::joinDaes(&cls_dae, &dae)?;
            dae
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDae)
}

fn stripRecordDefaultBindingsFromDAE(
    mut inClassDAE: DAE::DAElist,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inEqDAE: &DAE::DAElist,
) -> Result<DAE::DAElist> {
    let mut outClassDAE: DAE::DAElist;
    outClassDAE = (::match_deref::match_deref! { match &((inClassDAE.clone(), inType.clone(), inEqDAE.clone())) {
        (DAE::DAElist { elementLst: els }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, DAE::DAElist { elementLst: eqs @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }) => {
            let mut els = (*els).clone();
            (els, _) = List::mapFold(metamodelica::AsArg::as_arg(&els), &stripRecordDefaultBindingsFromElement, eqs.clone())?;
            DAE::DAElist { elementLst: els.clone() }
        },
        _ => {
            inClassDAE
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outClassDAE)
}

fn stripRecordDefaultBindingsFromElement(
    mut inVar: metamodelica::Ref<DAE::Element>,
    mut inEqs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::Ref<DAE::Element>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outVar: metamodelica::Ref<DAE::Element>;
    let mut outEqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    (outVar, outEqs) = (::match_deref::match_deref! { match &((inVar.clone(), inEqs.clone())) {
        (Deref @ DAE::Element::VAR { componentRef: var_cr, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: eq_cr, .. }, .. }, tail: rest_eqs }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&var_cr), metamodelica::AsArg::as_arg(&eq_cr))?) => {
            (DAEUtil::setElementVarBinding(inVar, None), rest_eqs.clone())
        },
        (Deref @ DAE::Element::VAR { componentRef: var_cr, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: eq_cr, .. }, .. }, tail: _ }) if (ComponentReferenceBasics::crefPrefixOf(metamodelica::AsArg::as_arg(&eq_cr), metamodelica::AsArg::as_arg(&var_cr))?) => {
            (DAEUtil::setElementVarBinding(inVar, None), inEqs)
        },
        _ => {
            (inVar, inEqs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outEqs))
}

fn checkDimensionGreaterThanZero(
    mut inDim: &metamodelica::Ref<DAE::Dimension>,
    mut inPrefix: DAE::Prefix,
    mut inIdent: ArcStr,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (match &**inDim {
        DAE::Dimension::DIM_INTEGER {
            integer: __inDim_integer,
        } => {
            let mut dim_str: ArcStr;
            let mut cr_str: ArcStr;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            if __inDim_integer.clone() < 0 {
                dim_str = ExpressionBasics::dimensionString(inDim)?;
                cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: inIdent,
                    identType: DAE::T_REAL_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                });
                cr_str =
                    ComponentReferenceBasics::printComponentRefStr(&(PrefixUtil::prefixCrefNoContext(inPrefix, cr)?))?;
                Error::addSourceMessageAndFail(
                    &(Error::NEGATIVE_DIMENSION_INDEX.clone()),
                    list![dim_str, cr_str],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

fn checkArrayModDimSize(
    mut r#mod: &metamodelica::Ref<DAE::Mod>,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
    mut inPrefix: DAE::Prefix,
    mut inIdent: ArcStr,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (match &**r#mod {
        DAE::Mod::MOD {
            eachPrefix: SCode::Each::NOT_EACH { .. },
            subModLst: __mod_subModLst,
            ..
        } => {
            List::map4_0(
                metamodelica::AsArg::as_arg(&__mod_subModLst),
                &move |__a0: metamodelica::Ref<DAE::SubMod>,
                       __a1: metamodelica::Ref<DAE::Dimension>,
                       __a2: DAE::Prefix,
                       __a3: ArcStr,
                       __a4: SourceInfo| checkArraySubModDimSize(&__a0, __a1, &__a2, &__a3, __a4),
                inDimension,
                inPrefix,
                inIdent,
                inInfo,
            )?;
            ()
        }
        _ => (),
    });
    Ok(())
}

fn checkArraySubModDimSize(
    mut inSubMod: &metamodelica::Ref<DAE::SubMod>,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
    mut inPrefix: &DAE::Prefix,
    mut inIdent: &ArcStr,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inSubMod {
        Deref @ DAE::SubMod { ident: Deref @ "quantity", .. } => {
            ()
        },
        Deref @ DAE::SubMod { ident: name, r#mod: Deref @ DAE::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH { .. }, binding: eqmod, .. } } => {
            let mut name = (*name).clone();
            name = { let mut __mm_s = String::new(); __mm_s.push_str(&*inIdent); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
            let true = (checkArrayModBindingDimSize(eqmod.clone(), inDimension, inPrefix, metamodelica::AsArg::as_arg(&name), inInfo)) else { return Err("pattern mismatch") };
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkArrayModBindingDimSize(
    mut inBinding: Option<DAE::EqMod>,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
    mut inPrefix: &DAE::Prefix,
    mut inIdent: &ArcStr,
    mut inInfo: SourceInfo,
) -> bool {
    let mut outIsCorrect: bool;
    outIsCorrect = 'mc: {
        let __mc_input = inBinding;
        if let Ok(__v) = (|| -> Result<_> {
            let Some(DAE::EqMod::TYPED {
                modifierAsExp: ref exp,
                properties: DAE::Properties::PROP { type_: ref ty, .. },
                info: mut info,
                ..
            }) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut ty_dim: metamodelica::Ref<DAE::Dimension>;
            let mut dim_size1: i32;
            let mut dim_size2: i32;
            let mut exp_str: ArcStr;
            let mut exp_ty_str: ArcStr;
            let mut dims_str: ArcStr;
            let mut ty_dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            ty_dim = Types::getDimensionNth(&(ty.clone()), 1)?;
            dim_size1 = Expression::dimensionSize(&inDimension)?;
            dim_size2 = Expression::dimensionSize(&ty_dim)?;
            let true = (dim_size1 != dim_size2) else {
                return Err("pattern mismatch");
            };
            exp_str = ExpressionBasics::printExpStr(exp.clone())?;
            exp_ty_str = TypesDump::unparseType(ty.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(TypesDump::getDimensions(&(ty.clone()))) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty_dims = metamodelica::Own::own(__pa0);
            dims_str = ExpressionBasics::dimensionsString(metamodelica::cons(inDimension.clone(), ty_dims.clone()))?;
            Error::addMultiSourceMessage(
                &(Error::ARRAY_DIMENSION_MISMATCH.clone()),
                &(list![exp_str.clone(), exp_ty_str.clone(), dims_str.clone()]),
                &(metamodelica::cons(info.clone(), metamodelica::cons(inInfo.clone(), metamodelica::nil()))),
            )?;
            Ok(false)
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
    outIsCorrect
}

fn instArray(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inState: ClassInf::State,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inIdent: ArcStr,
    mut inElement: &(metamodelica::Ref<SCode::Element>, SCode::Attributes),
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inInteger: i32,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
    mut inDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inIntegerLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inBoolean: bool,
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut info: SourceInfo,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    checkDimensionGreaterThanZero(&inDimension, inPrefix.clone(), inIdent.clone(), &info)?;
    checkArrayModDimSize(
        &inMod,
        inDimension.clone(),
        inPrefix.clone(),
        inIdent.clone(),
        info.clone(),
    )?;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inStore,
            inState,
            inMod.clone(),
            inPrefix.clone(),
            inIdent.clone(),
            inElement,
            inPrefixes.clone(),
            inInteger,
            inDimension.clone(),
            inDimensionLst.clone(),
            inIntegerLst.clone(),
            inInstDims.clone(),
            inBoolean,
            inComment.clone(),
            inGraph,
            inSets.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ClassInf::State::FUNCTION { .. }, r#mod, pre, n, (cl, _), _, _, dim, _, _, inst_dims, _, _, graph, csets) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut p: DAE::Properties;
                    let mut env_1: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (Expression::dimensionUnknownOrExp(metamodelica::AsArg::as_arg(&dim))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mod::modEquation(metamodelica::AsArg::as_arg(&r#mod))) {
                        Some(DAE::EqMod::TYPED { modifierAsExp: __pa0, properties: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    p = metamodelica::Own::own(__pa1);
                    (cache, env_1, ih, store, _, _, ty, _, _, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), r#mod.clone(), pre.clone(), cl.clone(), inst_dims.clone(), true, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, graph.clone(), metamodelica::AsArg::as_arg(&csets))?;
                    ty_1 = Types::simplifyType(ty.clone())?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), ty_1.clone(), metamodelica::nil()))?;
                    (rhs, _) = Types::matchProp(e.clone(), &p, &(DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }), true)?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                    lhs = Expression::makeCrefExp(cr.clone(), ty_1.clone())?;
                    dae = InstSection::makeDaeEquation(lhs.clone(), rhs.clone(), source.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL)?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), inSets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, (cl, attr), pf, i, _, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut compenv: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut daeLst: DAE::DAElist;
                    let mut s: metamodelica::Ref<DAE::Subscript>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let false = (Expression::dimensionKnown(&inDimension)) else { return Err("pattern mismatch") };
                    e = metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() });
                    s = metamodelica::Ref::new(DAE::Subscript::INDEX { exp: e.clone() });
                    r#mod = Mod::lookupIdxModification(metamodelica::AsArg::as_arg(&r#mod), e.clone())?;
                    (cache, compenv, ih, store, daeLst, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), metamodelica::cons(s.clone(), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), daeLst.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, _, pre, n, (cl, attr), pf, _, Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut compenv: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut s: metamodelica::Ref<DAE::Subscript>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    ErrorExt::setCheckpoint(literal!("instArray Real[0]"));
                    e = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
                    s = metamodelica::Ref::new(DAE::Subscript::INDEX { exp: e.clone() });
                    r#mod = Mod::filterRedeclares(inMod.clone());
                    r#mod = Mod::lookupIdxModification(&r#mod, e.clone())?;
                    (cache, compenv, ih, store, _, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), metamodelica::cons(s.clone(), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ErrorExt::rollBack(literal!("instArray Real[0]"));
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, _, _, _, _, _, _, _) => {
                    ErrorExt::delCheckpoint(literal!("instArray Real[0]"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, _, _, _, _, _, _, Deref @ DAE::Dimension::DIM_INTEGER { integer: stop }, _, _, _, _, _, graph, csets) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    (cache, env, ih, store, dae, csets, ty, graph) = instArrayDimInteger(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), inMod.clone(), inPrefix.clone(), inIdent.clone(), inElement, inPrefixes.clone(), stop.clone(), inDimensionLst.clone(), inIntegerLst.clone(), inInstDims.clone(), inBoolean, inComment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, (cl, attr), pf, _, Deref @ DAE::Dimension::DIM_ENUM { .. }, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    Ok(instArrayDimEnum(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), metamodelica::AsArg::as_arg(&r#mod), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), &inDimension, dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, (cl, attr), pf, _, Deref @ DAE::Dimension::DIM_BOOLEAN { .. }, dims, idxs, inst_dims, r#impl, comment, graph, csets) => {
                    let mut env_1: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut mod_2: metamodelica::Ref<DAE::Mod>;
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut daeLst: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    mod_1 = Mod::lookupIdxModification(metamodelica::AsArg::as_arg(&r#mod), metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }))?;
                    mod_2 = Mod::lookupIdxModification(metamodelica::AsArg::as_arg(&r#mod), metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))?;
                    (cache, env_1, ih, store, dae1, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), mod_1.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }) }), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, _, ih, store, dae2, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), mod_2.clone(), pre.clone(), n.clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }) }), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    daeLst = DAEUtil::joinDaes(&dae1, &dae2)?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), daeLst.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, ci_state, r#mod, pre, n, _, _, i, _, _, idxs, _, _, _, _, _) => {
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    let mut str4: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(Mod::lookupIdxModification(metamodelica::AsArg::as_arg(&r#mod), metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() })), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    str1 = PrefixUtil::printPrefixStrIgnoreNoPre(PrefixUtil::prefixAdd(n.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::AsArg::as_arg(&pre), openmodelica_frontend_types::SCode::Variability::VAR, ci_state.clone(), info.clone())?)?;
                    str2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(idxs.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0))?, literal!(", "))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                    str3 = Mod::prettyPrintMod(metamodelica::AsArg::as_arg(&r#mod), 1)?;
                    str4 = { let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*n); __mm_s.push_str(&*str2); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*str3); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    str2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*str1); __mm_s.push_str(&*str2); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::MODIFICATION_INDEX_NOT_FOUND.clone()), list![str1.clone(), str4.clone(), str2.clone(), str3.clone()], &info)?;
                    Ok(return Err("fail"))
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instArray failed: ")); __mm_s.push_str(&*inIdent); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn instArrayDimInteger(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inState: ClassInf::State,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inName: ArcStr,
    mut inElement: &(metamodelica::Ref<SCode::Element>, SCode::Attributes),
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inDimensionSize: i32,
    mut inRestDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImpl: bool,
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut inInfo: SourceInfo,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH;
    let mut outStore: UnitAbsyn::InstStore = inStore;
    let mut outDae: DAE::DAElist = DAE::emptyDae().clone();
    let mut outSets: DAE::Connect::Sets = inSets;
    let mut outType: metamodelica::Ref<DAE::Type> = DAE::T_UNKNOWN_DEFAULT().clone();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph;
    let mut c: metamodelica::Ref<SCode::Element>;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let mut imod: metamodelica::Ref<DAE::Mod>;
    let mut cls_path: metamodelica::Ref<Absyn::Path>;
    let mut smod: metamodelica::Ref<SCode::Mod>;
    let mut attr: SCode::Attributes;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut s: metamodelica::Ref<DAE::Subscript>;
    let mut inst_dims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;
    let mut dae: DAE::DAElist;
    (cls, r#mod, attr, inst_dims) = (::match_deref::match_deref! { match &(inElement) {
        (__esc_c @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_cls_path, arrayDim: Some(_) }, modifications: __esc_smod, .. }, .. }, __esc_attr) => {
            c = (*__esc_c).clone();
            cls_path = (*__esc_cls_path).clone();
            smod = (*__esc_smod).clone();
            attr = (*__esc_attr).clone();
            (_, cls, _) = Lookup::lookupClass(&outCache, &outEnv, metamodelica::AsArg::as_arg(&cls_path), Some(var_field!((*c).info, SCode::Element::CLASS).clone()))?;
            smod = InstUtil::chainRedeclares(&inMod, smod.clone());
            (_, r#mod) = Mod::elabMod(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), smod.clone(), inImpl, Mod::ModScope::DERIVED { path: cls_path.clone() }, inInfo.clone())?;
            r#mod = Mod::merge(inMod, r#mod, literal!(""), true)?;
            (cls, r#mod, attr.clone(), metamodelica::nil())
        },
        (__esc_cls, __esc_attr) => {
            cls = (*__esc_cls).clone();
            attr = (*__esc_attr).clone();
            (cls.clone(), inMod, attr.clone(), inInstDims)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut i in ({
        let __s = inDimensionSize;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        e = metamodelica::Ref::new(DAE::Exp::ICONST { integer: i });
        imod = Mod::lookupIdxModification(&r#mod, e.clone())?;
        s = metamodelica::Ref::new(DAE::Subscript::INDEX { exp: e });
        (outCache, outEnv, outIH, outStore, dae, outSets, outType, outGraph) = instVar2(
            outCache,
            inEnv.clone(),
            outIH,
            outStore,
            inState.clone(),
            imod,
            inPrefix.clone(),
            inName.clone(),
            cls.clone(),
            attr.clone(),
            inPrefixes.clone(),
            inRestDimensions.clone(),
            metamodelica::cons(s, inSubscripts.clone()),
            inst_dims.clone(),
            inImpl,
            inComment.clone(),
            inInfo.clone(),
            outGraph,
            outSets,
        )?;
        outDae = DAEUtil::joinDaes(&dae, &outDae)?;
    }
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn instArrayDimEnum(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inState: ClassInf::State,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inName: ArcStr,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
    mut inRestDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImpl: bool,
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut inInfo: SourceInfo,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH;
    let mut outStore: UnitAbsyn::InstStore = inStore;
    let mut outDae: DAE::DAElist = DAE::emptyDae().clone();
    let mut outSets: DAE::Connect::Sets = inSets;
    let mut outType: metamodelica::Ref<DAE::Type> = DAE::T_UNKNOWN_DEFAULT().clone();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph;
    let mut enum_path: metamodelica::Ref<Absyn::Path>;
    let mut enum_lit_path: metamodelica::Ref<Absyn::Path>;
    let mut literals: metamodelica::List<ArcStr>;
    let mut i: i32 = 1;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let mut dae: DAE::DAElist;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inDimension)) {
        Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: __pa0, literals: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    enum_path = metamodelica::Own::own(__pa0);
    literals = metamodelica::Own::own(__pa1);
    for mut lit in &*literals {
        enum_lit_path = AbsynUtil::joinPaths(
            enum_path.clone(),
            metamodelica::Ref::new(Absyn::Path::IDENT { name: lit.clone() }),
        )?;
        e = metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
            name: enum_lit_path,
            index: i,
        });
        r#mod = Mod::lookupIdxModification(inMod, e.clone())?;
        i = i + 1;
        (outCache, outEnv, outIH, outStore, dae, outSets, outType, outGraph) = instVar2(
            outCache,
            inEnv.clone(),
            outIH,
            outStore,
            inState.clone(),
            r#mod,
            inPrefix.clone(),
            inName.clone(),
            inClass.clone(),
            inAttributes.clone(),
            inPrefixes.clone(),
            inRestDimensions.clone(),
            metamodelica::cons(
                metamodelica::Ref::new(DAE::Subscript::INDEX { exp: e }),
                inSubscripts.clone(),
            ),
            inInstDims.clone(),
            inImpl,
            inComment.clone(),
            inInfo.clone(),
            outGraph,
            outSets,
        )?;
        outDae = DAEUtil::joinDaes(&outDae, &dae)?;
    }
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}
