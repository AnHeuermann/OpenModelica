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

use crate::Builtin;
use crate::Ceval;
use crate::ConnectUtil;
use crate::ConnectionGraph;
use crate::FGraph;
use crate::FGraphBuildEnv;
use crate::FNode;
use crate::FUnitCheck as UnitCheck;
use crate::HashSet;
use crate::InnerOuter;
use crate::InstBinding;
use crate::InstExtends;
use crate::InstFunction;
use crate::InstHashTable;
use crate::InstMeta;
use crate::InstSection;
use crate::InstStateMachineUtil;
use crate::InstUtil;
use crate::InstVar;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::Static;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTable5;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
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
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_inst::SCodeInstUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_script_util::UnitParserExt;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// public imports
// **
// These type aliases are introduced to make the code a little more readable.
// **
/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

pub type InstDims = metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;

type BasicTypeAttrTyper = std::sync::Arc<
    dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>>
        + 'static,
>;

// protected imports
// BTH
fn instantiateClass_dispatch(
    mut inCache: FCore::Cache,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut doSCodeDep: bool,
    mut relaxedFrontEnd: bool,
    mut clearCache: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDAElist: DAE::DAElist;
    (outCache, outEnv, outIH, outDAElist) = (::match_deref::match_deref! { match &((inProgram, inPath.clone())) {
        (cdecls @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::IDENT { .. }) => {
            let mut cache = inCache;
            let mut ih = inIH;
            let mut env: FCore::Graph;
            let mut dae2: DAE::DAElist;
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut cdecls = (*cdecls).clone();
            cache = FCore::setCacheClassName(cache, path.clone());
            if doSCodeDep {
                cdecls = InstUtil::scodeFlatten(cdecls.clone(), inPath)?;
                ExecStat::execStat(&(literal!("FrontEnd - scodeFlatten")))?;
            }
            (cache, env) = Builtin::initialGraph(cache)?;
            env = FGraphBuildEnv::mkProgramGraph(metamodelica::AsArg::as_arg(&cdecls), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, env)?;
            source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(&env)?)?;
            if Flags::isSet(Flags::GC_PROF.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*GCExt::profStatsStr(GCExt::getProfStats(), literal!("GC stats after pre-frontend work (building graphs):"), literal!("\n  "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            ExecStat::execStat(&(literal!("FrontEnd - mkProgramGraph")))?;
            (cache, env, ih, dae2) = instClassInProgram(cache, env, ih, metamodelica::AsArg::as_arg(&cdecls), metamodelica::AsArg::as_arg(&path), source, relaxedFrontEnd)?;
            if clearCache {
                InstHashTable::release()?;
            }
            (cache, env, ih, dae2)
        },
        (cdecls @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
            let mut cache = inCache;
            let mut ih = inIH;
            let mut env: FCore::Graph;
            let mut dae: DAE::DAElist;
            let mut n: ArcStr;
            let mut pathstr: ArcStr;
            let mut cdef: metamodelica::Ref<SCode::Element>;
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut daeElts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut cmt: Option<metamodelica::Ref<SCode::Comment>>;
            let mut cdecls = (*cdecls).clone();
            cache = FCore::setCacheClassName(cache, path.clone());
            if doSCodeDep {
                cdecls = InstUtil::scodeFlatten(cdecls.clone(), inPath)?;
                ExecStat::execStat(&(literal!("FrontEnd - scodeFlatten")))?;
            }
            pathstr = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            (cache, env) = Builtin::initialGraph(cache)?;
            env = FGraphBuildEnv::mkProgramGraph(metamodelica::AsArg::as_arg(&cdecls), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, env)?;
            let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&cache, &env, metamodelica::AsArg::as_arg(&path), Some(Absyn::dummyInfo.clone()))?) {
                (__pa0, __pa2 @ Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            n = metamodelica::Own::own(__pa1);
            cdef = metamodelica::Own::own(__pa2);
            env = metamodelica::Own::own(__pa3);
            checkInstanceRestriction(&cdef, path.clone(), relaxedFrontEnd)?;
            if Flags::isSet(Flags::GC_PROF.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*GCExt::profStatsStr(GCExt::getProfStats(), literal!("GC stats after pre-frontend work (building graphs):"), literal!("\n  "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            ExecStat::execStat(&(literal!("FrontEnd - mkProgramGraph")))?;
            (cache, env, ih, _, dae, _, _, _, _, _) = instClass(cache, env.clone(), ih, UnitAbsynBuilder::emptyInstStore(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), makeTopComponentPrefix(&env, &n), cdef.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
            dae = InstUtil::reEvaluateInitialIfEqns(cache.clone(), env.clone(), dae, true)?;
            source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(&env)?)?;
            daeElts = DAEUtil::daeElements(&dae);
            cmt = SCodeUtil::getElementComment(&cdef);
            dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::COMP { ident: pathstr, dAElist: daeElts, source: source, comment: cmt })] };
            if clearCache {
                InstHashTable::release()?;
            }
            (cache, env, ih, dae)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outDAElist))
}

pub fn instantiateClass(
    mut inCache: FCore::Cache,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut doSCodeDep: bool,
    mut relaxedFrontEnd: bool,
    mut clearCache: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = metamodelica::nil();
    let mut outDAElist: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    (outCache, outEnv, outIH, outDAElist) = 'mc: {
        let __mc_input = (inCache, inIH, inProgram, inPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Error::addMessage(Error::NO_CLASSES_LOADED.clone(), metamodelica::nil())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDAElist: DAE::DAElist = outDAElist.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    (outCache, outEnv, outIH, outDAElist) = instantiateClass_dispatch(cache.clone(), ih.clone(), cdecls.clone(), path.clone(), doSCodeDep, relaxedFrontEnd, clearCache)?;
                    outDAElist = UnitCheck::checkUnits(outDAElist.clone(), &(FCore::getFunctionTree(&outCache)))?;
                    Ok(((outCache.clone(), outEnv.clone(), outIH.clone(), outDAElist.clone()), outCache.clone(), outDAElist.clone(), outEnv.clone(), outIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDAElist = __wb1;
            outEnv = __wb2;
            outIH = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path) => {
                    let mut cname_str: ArcStr;
                    let mut stackOverflow: bool;
                    stackOverflow = setStackOverflowSignal(false);
                    cname_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); __mm_s.push_str(&*if (stackOverflow) {literal!(". The compiler got into Stack Overflow!")} else {literal!("")}); ArcStr::from(__mm_s) };
                    if !(Config::getGraphicsExpMode()?) {
                        Error::addMessage(Error::ERROR_FLATTENING.clone(), list![cname_str.clone()])?;
                    }
                    InstHashTable::release()?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDAElist))
}

pub(crate) fn instantiatePartialClass(
    mut inCache: FCore::Cache,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDAElist: DAE::DAElist;
    (outCache, outEnv, outIH, outDAElist) = 'mc: {
        let __mc_input = (inCache, inIH, inProgram, inPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Error::addMessage(Error::NO_CLASSES_LOADED.clone(), metamodelica::nil())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::IDENT { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut cdecls = (*cdecls).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(metamodelica::AsArg::as_arg(&cdecls), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, env.clone())?;
                    cdecls = List::map1(cdecls.clone(), &SCodeUtil::classSetPartial, openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(&env)?)?;
                    (cache, env_2, ih, dae) = instClassInProgram(cache.clone(), env_1.clone(), ih.clone(), metamodelica::AsArg::as_arg(&cdecls), metamodelica::AsArg::as_arg(&path), source.clone(), true)?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut n: ArcStr;
                    let mut pathstr: ArcStr;
                    let mut cdef: metamodelica::Ref<SCode::Element>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut daeElts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut cmt: Option<metamodelica::Ref<SCode::Comment>>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(metamodelica::AsArg::as_arg(&cdecls), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, env.clone())?;
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), &env_1, metamodelica::AsArg::as_arg(&path), Some(Absyn::dummyInfo.clone()))?) {
                        (__pa0, __pa2 @ Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    n = metamodelica::Own::own(__pa1);
                    cdef = metamodelica::Own::own(__pa2);
                    env_2 = metamodelica::Own::own(__pa3);
                    cdef = SCodeUtil::classSetPartial(cdef.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    (cache, env_2, ih, _, dae, _, _, _, _, _) = instClass(cache.clone(), env_2.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), makeTopComponentPrefix(&env_2, &n), cdef.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    pathstr = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(&env)?)?;
                    daeElts = DAEUtil::daeElements(&dae);
                    cmt = SCodeUtil::getElementComment(&cdef);
                    dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::COMP { ident: pathstr.clone(), dAElist: daeElts.clone(), source: source.clone(), comment: cmt.clone() })] };
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, path) => {
                    if !((!(Config::getGraphicsExpMode()?))) { return Err("guard") }
                    let mut cname_str: ArcStr;
                    cname_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::ERROR_FLATTENING.clone(), list![cname_str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDAElist))
}

fn makeTopComponentPrefix(mut inGraph: &FCore::Graph, mut inName: &ArcStr) -> DAE::Prefix {
    let mut outPrefix: DAE::Prefix;
    outPrefix = openmodelica_frontend_types::DAE::Prefix::NOPRE;
    outPrefix
}

fn instClassInProgram(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut relaxedFrontEnd: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    (outCache, outEnv, outIH, outDae) = 'mc: {
        let __mc_input = (&**inProgram, &**inPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), DAE::emptyDae().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Path::IDENT { name: Deref @ "" }) => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), DAE::emptyDae().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Path::IDENT { name }) => {
                    let mut cls: metamodelica::Ref<SCode::Element>;
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut ih: InstanceHierarchy;
                    let mut dae: DAE::DAElist;
                    let mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut cmt: Option<metamodelica::Ref<SCode::Comment>>;
                    cls = InstUtil::lookupTopLevelClass(name.clone(), inProgram, true)?;
                    (cache, env, ih, _, dae, _, _, _, _, _) = instClass(inCache.clone(), inEnv.clone(), inIH.clone(), UnitAbsynBuilder::emptyInstStore(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), makeTopComponentPrefix(&inEnv, metamodelica::AsArg::as_arg(&name)), cls.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    dae = InstUtil::reEvaluateInitialIfEqns(cache.clone(), env.clone(), dae.clone(), true)?;
                    elts = DAEUtil::daeElements(&dae);
                    cmt = SCodeUtil::getElementComment(&cls);
                    dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::COMP { ident: name.clone(), dAElist: elts.clone(), source: inSource.clone(), comment: cmt.clone() })] };
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone()))
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
                    Debug::trace(literal!("Inst.instClassInProgram failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDae))
}

pub fn instClass(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplicit: bool,
    mut inCallingScope: InstTypes::CallingScope,
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
    ClassInf::State,
    Option<SCode::Attributes>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut cache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outState: ClassInf::State;
    let mut optDerAttr: Option<SCode::Attributes>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (
        cache, outEnv, outIH, outStore, outDae, outSets, outType, outState, optDerAttr, outGraph,
    ) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv.clone(),
            inIH.clone(),
            inStore,
            inMod.clone(),
            inPrefix.clone(),
            inClass.clone(),
            inInstDims.clone(),
            inImplicit,
            inCallingScope,
            inGraph.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, store, _, _, Deref @ SCode::Element::CLASS { name: n, partialPrefix: SCode::Partial::PARTIAL { .. }, restriction: r, info, .. }, _, _, _, _) => {
                    let mut env: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut ci_state_1: ClassInf::State;
                    let mut dae: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy;
                    let mut cache = (*cache).clone();
                    let mut store = (*store).clone();
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    let false = (SCodeUtil::isFunctionRestriction(metamodelica::AsArg::as_arg(&r))) else { return Err("pattern mismatch") };
                    c = SCodeUtil::setClassPartialPrefix(openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, inClass.clone())?;
                    if !(Config::getGraphicsExpMode()?) {
                        Error::addSourceMessage(&(Error::INST_PARTIAL_CLASS_CHECK_MODEL_WARNING.clone()), list![n.clone()], metamodelica::AsArg::as_arg(&info))?;
                    }
                    (cache, env, ih, store, dae, csets, ty, ci_state_1, oDA, graph) = instClass(inCache.clone(), inEnv.clone(), inIH.clone(), store.clone(), inMod.clone(), inPrefix.clone(), c.clone(), inInstDims.clone(), inImplicit, inCallingScope, inGraph.clone(), inSets)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), ci_state_1.clone(), oDA.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, r#mod, pre, c @ Deref @ SCode::Element::CLASS { name: n, encapsulatedPrefix: encflag, restriction: r, partialPrefix, info, .. }, inst_dims, r#impl, callscope, graph) => {
                    let mut env_1: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut scopeName: ArcStr;
                    let mut strDepth: ArcStr;
                    let mut callscope_1: bool;
                    let mut isFn: bool;
                    let mut notIsPartial: bool;
                    let mut isPartialFn: bool;
                    let mut recursionDepthReached: bool;
                    let mut ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut dae1: DAE::DAElist;
                    let mut dae1_1: DAE::DAElist;
                    let mut dae: DAE::DAElist;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut bc_ty: Option<metamodelica::Ref<DAE::Type>>;
                    let mut fq_class: metamodelica::Ref<Absyn::Path>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut equalityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    recursionDepthReached = (((FGraph::currentScope(metamodelica::AsArg::as_arg(&env)))).len() as i32) < Global::recursionDepthLimit.clone();
                    if !(recursionDepthReached) {
                        scopeName = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                        strDepth = intString(Global::recursionDepthLimit.clone());
                        Error::addSourceMessage(&(Error::RECURSION_DEPTH_REACHED.clone()), list![strDepth.clone(), scopeName.clone()], metamodelica::AsArg::as_arg(&info))?;
                        return Err("fail");
                    }
                    isFn = SCodeUtil::isFunctionRestriction(metamodelica::AsArg::as_arg(&r));
                    notIsPartial = !(SCodeUtil::partialBool(partialPrefix.clone()));
                    isPartialFn = isFn && SCodeUtil::partialBool(partialPrefix.clone());
                    let true = (notIsPartial || isPartialFn) else { return Err("pattern mismatch") };
                    env_1 = FGraph::openScope(env.clone(), encflag.clone(), n.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&r)))?;
                    ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&r), FGraph::getGraphName(&env_1)?)?;
                    csets = ConnectUtil::newSet(pre.clone(), inSets.clone());
                    (cache, env_3, ih, store, dae1, csets, ci_state_1, tys, bc_ty, oDA, equalityConstraint, graph) = instClassIn(cache.clone(), env_1.clone(), ih.clone(), store.clone(), r#mod.clone(), pre.clone(), ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), csets.clone(), None)?;
                    csets = ConnectUtil::addSet(inSets.clone(), csets.clone())?;
                    (cache, fq_class) = makeFullyQualifiedIdent(cache.clone(), env.clone(), n.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    callscope_1 = InstUtil::isTopCall(callscope.clone());
                    dae1_1 = DAEUtil::addComponentType(dae1.clone(), fq_class.clone())?;
                    (csets, _, graph) = InnerOuter::retrieveOuterConnections(metamodelica::AsArg::as_arg(&cache), &env_3, metamodelica::AsArg::as_arg(&ih), metamodelica::AsArg::as_arg(&pre), csets.clone(), callscope_1, graph.clone())?;
                    dae = ConnectUtil::equations(callscope_1, csets.clone(), dae1_1.clone(), graph.clone(), &(AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fq_class.clone()), literal!("."), true, false)?))?;
                    ty = InstUtil::mktype(fq_class.clone(), ci_state_1.clone(), tys.clone(), bc_ty.clone(), equalityConstraint.clone(), c.clone(), &(InstUtil::extractComment(&dae.elementLst)?))?;
                    dae = InstUtil::updateDeducedUnits(callscope_1, metamodelica::AsArg::as_arg(&store), dae.clone())?;
                    ty = markDerivedRecordOutsideBindings(ty.clone(), metamodelica::AsArg::as_arg(&c))?;
                    ty = markTypesVarsOutsideBindings(ty.clone(), metamodelica::AsArg::as_arg(&r#mod))?;
                    ty = InstUtil::fixInstClassType(ty.clone(), isPartialFn)?;
                    Ok((cache.clone(), env_3.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), ci_state_1.clone(), oDA.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _, _, Deref @ SCode::Element::CLASS { name: n, partialPrefix: SCode::Partial::PARTIAL { .. }, info, .. }, _, false, _, _) => {
                    if !(Config::getGraphicsExpMode()?) {
                        Error::addSourceMessage(&(Error::INST_PARTIAL_CLASS.clone()), list![n.clone()], metamodelica::AsArg::as_arg(&info))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, _, Deref @ SCode::Element::CLASS { name: n, .. }, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instClass: ")); __mm_s.push_str(&*n); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        cache, outEnv, outIH, outStore, outDae, outSets, outType, outState, optDerAttr, outGraph,
    ))
}

fn instClassBasictype(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplicit: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inSets: DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    ClassInf::State,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outTypeVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outState: ClassInf::State;
    (
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outType,
        outTypeVars,
        outState,
    ) = (::match_deref::match_deref! { match &(inClass) {
        c @ Deref @ SCode::Element::CLASS { name: n, encapsulatedPrefix: encflag, restriction: r, .. } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut store = inStore;
            let mut r#mod = inMod;
            let mut pre = inPrefix;
            let mut inst_dims = inInstDims;
            let mut r#impl = inImplicit;
            let mut env_1: FCore::Graph;
            let mut env_3: FCore::Graph;
            let mut ci_state: ClassInf::State;
            let mut ci_state_1: ClassInf::State;
            let mut c_1: metamodelica::Ref<SCode::Element>;
            let mut dae1: DAE::DAElist;
            let mut dae1_1: DAE::DAElist;
            let mut dae: DAE::DAElist;
            let mut csets: DAE::Connect::Sets;
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let mut bc_ty: Option<metamodelica::Ref<DAE::Type>>;
            let mut fq_class: metamodelica::Ref<Absyn::Path>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            env_1 = FGraph::openScope(env, encflag.clone(), n.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&r)))?;
            ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&r), FGraph::getGraphName(&env_1)?)?;
            c_1 = SCodeUtil::classSetPartial(c.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
            (cache, env_3, ih, store, dae1, csets, ci_state_1, tys, bc_ty, _, _, _) = instClassIn(cache, env_1, ih, store, r#mod, pre, ci_state, c_1, openmodelica_frontend_types::SCode::Visibility::PUBLIC, inst_dims, r#impl, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), inSets, None)?;
            (cache, fq_class) = makeFullyQualifiedIdent(cache, env_3.clone(), n.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
            dae1_1 = DAEUtil::addComponentType(dae1, fq_class.clone())?;
            dae = dae1_1;
            ty = InstUtil::mktypeWithArrays(fq_class, ci_state_1.clone(), tys.clone(), bc_ty, c.clone(), &(InstUtil::extractComment(&dae.elementLst)?))?;
            (cache, env_3, ih, store, dae, csets, ty, tys, ci_state_1)
        },
        Deref @ SCode::Element::CLASS { .. } => {
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outType,
        outTypeVars,
        outState,
    ))
}

pub fn instClassIn(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inVisibility: SCode::Visibility,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut implicitInstantiation: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut instSingleCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    Option<metamodelica::Ref<DAE::Type>>,
    Option<SCode::Attributes>,
    Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outType: Option<metamodelica::Ref<DAE::Type>>;
    let mut optDerAttr: Option<SCode::Attributes>;
    let mut outEqualityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outVars,
        outType,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ) = 'mc: {
        let __mc_input = &*inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } => {
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut cache: FCore::Cache;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut equalityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy;
                    let mut store: UnitAbsyn::InstStore;
                    let true = (boolOr(AbsynUtil::isNotInnerOuter(io.clone()), AbsynUtil::isOnlyInner(io.clone()))) else { return Err("pattern mismatch") };
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inClass.clone(), inVisibility, inInstDims.clone(), implicitInstantiation, inCallingScope, inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: n, restriction: r, encapsulatedPrefix: encflag, prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } => {
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut cache: FCore::Cache;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut equalityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy;
                    let mut store: UnitAbsyn::InstStore;
                    let mut n = (*n).clone();
                    let true = (boolOr(AbsynUtil::isInnerOuter(io.clone()), AbsynUtil::isOnlyOuter(io.clone()))) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(FGraph::lastScopeRef(&inEnv)?)) {
                        Deref @ FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: __pa0 }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    n = metamodelica::Own::own(__pa0);
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    env = FGraph::openScope(env.clone(), encflag.clone(), n.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&r)))?;
                    ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&r), FGraph::getGraphName(&env)?)?;
                    let __pa1 = ::match_deref::match_deref! { match &(InnerOuter::lookupInnerVar(&inCache, &env, &inIH, inPrefix.clone(), n.clone(), io.clone())?) {
                        InnerOuter::InstInner { innerElement: Some(__pa1), .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    c = metamodelica::Own::own(__pa1);
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), env.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), ci_state.clone(), c.clone(), inVisibility, inInstDims.clone(), implicitInstantiation, inCallingScope, inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: n, prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } => {
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut cache: FCore::Cache;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut equalityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy;
                    let mut store: UnitAbsyn::InstStore;
                    let mut n = (*n).clone();
                    let true = (boolOr(AbsynUtil::isInnerOuter(io.clone()), AbsynUtil::isOnlyOuter(io.clone()))) else { return Err("pattern mismatch") };
                    n = FGraph::getInstanceOriginalName(&inEnv, n.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(InnerOuter::lookupInnerVar(&inCache, &inEnv, &inIH, inPrefix.clone(), n.clone(), io.clone())?) {
                        InnerOuter::InstInner { innerElement: Some(__pa0), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    c = metamodelica::Own::own(__pa0);
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), c.clone(), inVisibility, inInstDims.clone(), implicitInstantiation, inCallingScope, inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: n, prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, info, .. } => {
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut cache: FCore::Cache;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut equalityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut store: UnitAbsyn::InstStore;
                    let true = (boolOr(AbsynUtil::isInnerOuter(io.clone()), AbsynUtil::isOnlyOuter(io.clone()))) else { return Err("pattern mismatch") };
                    if !(Config::getGraphicsExpMode()?) {
                        s1 = n.clone();
                        s2 = AbsynUtil::innerOuterStr(io.clone());
                        Error::addSourceMessage(&(Error::MISSING_INNER_CLASS.clone()), list![s1.clone(), s2.clone(), literal!("")], metamodelica::AsArg::as_arg(&info))?;
                    }
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inClass.clone(), inVisibility, inInstDims.clone(), implicitInstantiation, inCallingScope, inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outVars,
        outType,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ))
}

pub(crate) fn instClassIn2(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut ih: metamodelica::List<InnerOuter::TopInstance>,
    mut store: UnitAbsyn::InstStore,
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut prefix: DAE::Prefix,
    mut state: ClassInf::State,
    mut cls: metamodelica::Ref<SCode::Element>,
    mut visibility: SCode::Visibility,
    mut instDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut implicitInst: bool,
    mut callingScope: InstTypes::CallingScope,
    mut graph: ConnectionGraph::ConnectionGraph,
    mut sets: DAE::Connect::Sets,
    mut instSingleCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
    DAE::Connect::Sets,
    DAE::DAElist,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    Option<metamodelica::Ref<DAE::Type>>,
    Option<SCode::Attributes>,
    Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ih: metamodelica::List<InnerOuter::TopInstance> = ih;
    let mut store: UnitAbsyn::InstStore = store;
    let mut state: ClassInf::State = state;
    let mut graph: ConnectionGraph::ConnectionGraph = graph;
    let mut sets: DAE::Connect::Sets = sets;
    let mut dae: DAE::DAElist;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut ty: Option<metamodelica::Ref<DAE::Type>>;
    let mut optDerAttr: Option<SCode::Attributes>;
    let mut equalityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
    let mut cache_path: metamodelica::Ref<Absyn::Path>;
    let mut inputs: (
        metamodelica::Ref<DAE::Mod>,
        DAE::Prefix,
        DAE::Connect::Sets,
        ClassInf::State,
        metamodelica::Ref<SCode::Element>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
        bool,
        Option<metamodelica::Ref<DAE::ComponentRef>>,
        InstTypes::CallingScope,
    );
    let mut outputs: (
        FCore::Graph,
        DAE::DAElist,
        DAE::Connect::Sets,
        ClassInf::State,
        metamodelica::List<metamodelica::Ref<DAE::Var>>,
        Option<metamodelica::Ref<DAE::Type>>,
        Option<SCode::Attributes>,
        Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
        ConnectionGraph::ConnectionGraph,
    );
    let mut m: metamodelica::Ref<DAE::Mod>;
    let mut pre: DAE::Prefix;
    let mut csets: DAE::Connect::Sets;
    let mut st: ClassInf::State;
    let mut e: metamodelica::Ref<SCode::Element>;
    let mut dims: InstDims;
    let mut r#impl: bool;
    let mut scr: Option<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cs: InstTypes::CallingScope;
    let mut cached_graph: ConnectionGraph::ConnectionGraph;
    if SCodeUtil::isPackage(&cls) && SCodeUtil::isPartial(&cls) {
        (cache, env, ih, state, _) =
            partialInstClassIn(cache, env, ih, r#mod, prefix, state, cls, visibility, instDims, 0)?;
        dae = DAE::emptyDae().clone();
        vars = metamodelica::nil();
        ty = None;
        optDerAttr = None;
        equalityConstraint = None;
        return Ok((
            cache,
            env,
            ih,
            store,
            state,
            graph,
            sets,
            dae,
            vars,
            ty,
            optDerAttr,
            equalityConstraint,
        ));
    }
    cache_path = generateCachePath(&env, &cls, &prefix, callingScope)?;
    if Flags::isSet(Flags::CACHE.clone())? {
        if '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(InstHashTable::get(cache_path.clone()), '__try0)) {
                Deref @ metamodelica::ListNode::Cons { head: Some(InstHashTable::CachedInstItem::FUNC_instClassIn { inputs: __pa1, outputs: __pa2 }), tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            inputs = metamodelica::Own::own(__pa1);
            outputs = metamodelica::Own::own(__pa2);
            let (__pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(inputs.clone()) {
                (__pa4, __pa5, __pa6, __pa7, __pa8 @ Deref @ SCode::Element::CLASS { .. }, __pa9, __pa10, __pa11, __pa12) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            m = metamodelica::Own::own(__pa4);
            pre = metamodelica::Own::own(__pa5);
            csets = metamodelica::Own::own(__pa6);
            st = metamodelica::Own::own(__pa7);
            e = metamodelica::Own::own(__pa8);
            dims = metamodelica::Own::own(__pa9);
            r#impl = metamodelica::Own::own(__pa10);
            scr = metamodelica::Own::own(__pa11);
            cs = metamodelica::Own::own(__pa12);
            unwrap_break_err!(InstUtil::prefixEqualUnlessBasicType(prefix.clone(), pre.clone(), &cls), '__try0);
            if dims.clone() == instDims.clone() && r#impl == implicitInst && m.clone() == r#mod.clone() && csets.clone() == sets.clone() && st.clone() == state.clone() && e.clone() == cls.clone() && scr.clone() == instSingleCref.clone() && callingScopeCacheEq(cs, callingScope) {
                (env, dae, sets, state, vars, ty, optDerAttr, equalityConstraint, cached_graph) = outputs.clone();
                graph = unwrap_break_err!(ConnectionGraph::merge(graph.clone(), cached_graph.clone()), '__try0);
                unwrap_break_err!(showCacheInfo(&(literal!("Full Inst Hit: ")), cache_path.clone()), '__try0);
                return Ok((cache, env, ih, store, state, graph, sets, dae, vars, ty, optDerAttr, equalityConstraint));
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    match '__try14: {
        inputs = (
            r#mod.clone(),
            prefix.clone(),
            sets.clone(),
            state.clone(),
            cls.clone(),
            instDims.clone(),
            implicitInst,
            instSingleCref.clone(),
            callingScope,
        );
        (
            cache,
            env,
            ih,
            store,
            dae,
            sets,
            state,
            vars,
            ty,
            optDerAttr,
            equalityConstraint,
            graph,
        ) = unwrap_break_err!(instClassIn_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), r#mod.clone(), prefix.clone(), state.clone(), cls.clone(), visibility, instDims.clone(), implicitInst, callingScope, graph.clone(), sets.clone(), instSingleCref.clone()), '__try14);
        outputs = (
            env.clone(),
            dae.clone(),
            sets.clone(),
            state.clone(),
            vars.clone(),
            ty.clone(),
            optDerAttr.clone(),
            equalityConstraint.clone(),
            graph.clone(),
        );
        unwrap_break_err!(showCacheInfo(&(literal!("Full Inst Add: ")), cache_path.clone()), '__try14);
        InstHashTable::addToInstCache(
            cache_path.clone(),
            Some(InstHashTable::CachedInstItem::FUNC_instClassIn {
                inputs: inputs.clone(),
                outputs: outputs.clone(),
            }),
            None,
        );
        Ok::<_, &'static str>((
            cache.clone(),
            dae.clone(),
            env.clone(),
            equalityConstraint.clone(),
            graph.clone(),
            ih.clone(),
            inputs.clone(),
            optDerAttr.clone(),
            outputs.clone(),
            sets.clone(),
            state.clone(),
            store.clone(),
            ty.clone(),
            vars.clone(),
        ))
    } {
        Ok((
            __try14_o0,
            __try14_o1,
            __try14_o2,
            __try14_o3,
            __try14_o4,
            __try14_o5,
            __try14_o6,
            __try14_o7,
            __try14_o8,
            __try14_o9,
            __try14_o10,
            __try14_o11,
            __try14_o12,
            __try14_o13,
        )) => {
            cache = __try14_o0;
            dae = __try14_o1;
            env = __try14_o2;
            equalityConstraint = __try14_o3;
            graph = __try14_o4;
            ih = __try14_o5;
            inputs = __try14_o6;
            optDerAttr = __try14_o7;
            outputs = __try14_o8;
            sets = __try14_o9;
            state = __try14_o10;
            store = __try14_o11;
            ty = __try14_o12;
            vars = __try14_o13;
        }
        Err(__try14_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- Inst.instClassIn2 failed on class: "));
                __mm_s.push_str(&*SCodeUtil::elementName(&cls)?);
                __mm_s.push_str(&*literal!(" in environment: "));
                __mm_s.push_str(&*FGraph::printGraphPathStr(&env));
                ArcStr::from(__mm_s)
            })?;
            return Err(__try14_err);
        }
    }
    Ok((
        cache,
        env,
        ih,
        store,
        state,
        graph,
        sets,
        dae,
        vars,
        ty,
        optDerAttr,
        equalityConstraint,
    ))
}

fn markDerivedRecordOutsideBindings(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inClass: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut derMod: metamodelica::Ref<SCode::Mod>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    if !(SCodeUtil::isRecord(inClass)) || !(SCodeUtil::isDerivedClass(inClass)) {
        outType = inType;
        return Ok(outType);
    }
    derMod = SCodeUtil::getDerivedMod(inClass)?;
    if SCodeUtil::isEmptyMod(&derMod) {
        outType = inType;
        return Ok(outType);
    }
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(derMod.clone()) {
            Deref @ SCode::Mod::MOD { subModLst: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        submods = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((submods.clone(),))
    } {
        Ok((__try0_o0,)) => {
            submods = __try0_o0;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    "Unexpected Mod structure in collectAndFixDerivedComplexOutsideBindings."
                )],
            )?;
            return Err(__try0_err);
        }
    }
    outType = (match &*inType {
        DAE::Type::T_COMPLEX {
            complexClassType: __inType_complexClassType,
            equalityConstraint: __inType_equalityConstraint,
            usedExternally: __inType_usedExternally,
            varLst: __inType_varLst,
        } => {
            let mut tvars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            tvars = metamodelica::nil();
            for mut var in &*__inType_varLst.clone() {
                let mut var = var.clone();
                for mut submod in &*submods {
                    if varIsModifiedInDerivedMod(&var.name, metamodelica::AsArg::as_arg(&submod)) {
                        assign_field!(
                            var.bind_from_outside = true,
                            var.binding = markBindingFromDerivedRecordMods(var.binding.clone())
                        );
                        break;
                    }
                }
                tvars = metamodelica::cons(var, tvars);
            }
            tvars = tvars.reverse();
            metamodelica::Ref::new(DAE::Type::T_COMPLEX {
                complexClassType: __inType_complexClassType.clone(),
                varLst: tvars,
                equalityConstraint: __inType_equalityConstraint.clone(),
                usedExternally: __inType_usedExternally.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outType)
}

fn markBindingFromDerivedRecordMods(mut bind: metamodelica::Ref<DAE::Binding>) -> metamodelica::Ref<DAE::Binding> {
    let mut bind: metamodelica::Ref<DAE::Binding> = bind;
    let () = (match &*bind {
        DAE::Binding::EQBOUND { .. } => {
            assign_variant_field!(bind => DAE::Binding::EQBOUND; source = openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DERIVED_RECORD_DECL);
            ()
        }
        _ => (),
    });
    bind
}

fn varIsModifiedInDerivedMod(mut inName: &ArcStr, mut inSubmod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inSubmod {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::REDECL { .. }, .. } => false,
        Deref @ SCode::SubMod { .. } => stringEqual(&inSubmod.ident, &inName),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn markTypesVarsOutsideBindings(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type> = inType.clone();
    let mut submods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    if !(Types::isRecord(&inType)) {
        return Ok(outType);
    }
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*inMod)) {
            Deref @ DAE::Mod::MOD { subModLst: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        submods = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((submods.clone(),))
    } {
        Ok((__try0_o0,)) => {
            submods = __try0_o0;
        }
        Err(_) => {
            return Ok(outType);
        }
    }
    if (submods).is_empty() {
        return Ok(outType);
    }
    outType = (match &*inType {
        DAE::Type::T_COMPLEX {
            complexClassType: __inType_complexClassType,
            equalityConstraint: __inType_equalityConstraint,
            usedExternally: __inType_usedExternally,
            varLst: __inType_varLst,
        } => {
            let mut tvars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            tvars = metamodelica::nil();
            for mut var in &*__inType_varLst.clone() {
                let mut var = var.clone();
                for mut submod in &*submods {
                    if varIsModifiedInMod(&var.name, metamodelica::AsArg::as_arg(&submod)) {
                        assign_field!(var.bind_from_outside = true);
                        break;
                    }
                }
                tvars = metamodelica::cons(var, tvars);
            }
            tvars = tvars.reverse();
            metamodelica::Ref::new(DAE::Type::T_COMPLEX {
                complexClassType: __inType_complexClassType.clone(),
                varLst: tvars,
                equalityConstraint: __inType_equalityConstraint.clone(),
                usedExternally: __inType_usedExternally.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outType)
}

fn varIsModifiedInMod(mut inName: &ArcStr, mut inSubmod: &metamodelica::Ref<DAE::SubMod>) -> bool {
    let mut b: bool;
    b = (match &**inSubmod {
        DAE::SubMod { .. } => stringEqual(&inSubmod.ident, &inName),
    });
    b
}

fn callingScopeCacheEq(
    mut inCallingScope1: InstTypes::CallingScope,
    mut inCallingScope2: InstTypes::CallingScope,
) -> bool {
    let mut outIsEq: bool;
    outIsEq = (match (inCallingScope1, inCallingScope2) {
        (InstTypes::CallingScope::TYPE_CALL { .. }, InstTypes::CallingScope::TYPE_CALL { .. }) => true,
        (InstTypes::CallingScope::TYPE_CALL { .. }, _) => false,
        (_, InstTypes::CallingScope::TYPE_CALL { .. }) => false,
        _ => true,
    });
    outIsEq
}

pub(crate) fn instClassIn_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inVisibility: SCode::Visibility,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut implicitInstantiation: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut instSingleCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    Option<metamodelica::Ref<DAE::Type>>,
    Option<SCode::Attributes>,
    Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outTypesVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outTypesTypeOption: Option<metamodelica::Ref<DAE::Type>>;
    let mut optDerAttr: Option<SCode::Attributes>;
    let mut outEqualityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        outTypesTypeOption,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inStore,
            inMod,
            inPrefix,
            inState,
            inClass,
            inVisibility,
            inInstDims,
            implicitInstantiation,
            inCallingScope,
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::Element::CLASS { name: n, .. }, _, inst_dims, _, _, graph) => {
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut typer: BasicTypeAttrTyper;
                    ty = getBasicTypeType(metamodelica::AsArg::as_arg(&n))?;
                    typer = getBasicTypeAttrTyper(metamodelica::AsArg::as_arg(&n))?;
                    ty = liftNonExpType(ty.clone(), metamodelica::AsArg::as_arg(&inst_dims), Config::splitArrays()?);
                    tys = instBasicTypeAttributes(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&mods), ty.clone(), typer.clone(), metamodelica::AsArg::as_arg(&pre))?;
                    ty = Types::setTypeVars(ty.clone(), &tys)?;
                    bc = arrayBasictypeBaseclass(metamodelica::AsArg::as_arg(&inst_dims), ty.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), inSets.clone(), ci_state.clone(), tys.clone(), bc.clone(), None, None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, c @ Deref @ SCode::Element::CLASS { name: n, restriction: SCode::Restriction::R_ENUMERATION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, info, .. }, _, inst_dims, r#impl, callscope, graph) => {
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut env_1: FCore::Graph;
                    let mut ci_state_1: ClassInf::State;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut env_2: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut comp: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut names: metamodelica::List<ArcStr>;
                    let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut fq_class: metamodelica::Ref<Absyn::Path>;
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    names = SCodeUtil::componentNames(metamodelica::AsArg::as_arg(&c));
                    Types::checkEnumDuplicateLiterals(names.clone(), metamodelica::AsArg::as_arg(&info))?;
                    tys = instBasicTypeAttributes(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&mods), DAE::T_ENUMERATION_DEFAULT().clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: SourceInfo| getEnumAttributeType(__a0, __a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>> + 'static>), metamodelica::AsArg::as_arg(&pre))?;
                    ci_state_1 = ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::NEWDEF)?;
                    comp = InstUtil::addNomod(els.clone());
                    (cache, env_1, ih) = InstUtil::addComponentsToEnv(cache.clone(), env.clone(), ih.clone(), mods.clone(), pre.clone(), ci_state_1.clone(), &comp, r#impl.clone())?;
                    (cache, env_2, ih, store, _, csets, ci_state_1, tys1, graph, _) = instElementList(cache.clone(), env_1.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state_1.clone(), comp.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), true)?;
                    (cache, fq_class) = makeFullyQualifiedIdent(cache.clone(), env_2.clone(), n.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    eqConstraint = InstUtil::equalityConstraint(&env_2, metamodelica::AsArg::as_arg(&els), metamodelica::AsArg::as_arg(&info));
                    ty2 = metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: fq_class.clone(), names: names.clone(), literalVarLst: tys1.clone(), attributeLst: tys.clone() });
                    bc = arrayBasictypeBaseclass(metamodelica::AsArg::as_arg(&inst_dims), ty2.clone())?;
                    bc = if ((bc).is_some()) {bc.clone()} else {Some(ty2.clone())};
                    ty = InstUtil::mktype(fq_class.clone(), ci_state_1.clone(), tys1.clone(), bc.clone(), eqConstraint.clone(), c.clone(), &(SCode::noComment.clone()))?;
                    (cache, env_3) = InstUtil::updateEnumerationEnvironment(cache.clone(), env_2.clone(), &ty, metamodelica::AsArg::as_arg(&c), &ci_state_1);
                    tys2 = listAppend(tys.clone(), tys1.clone());
                    Ok((cache.clone(), env_3.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state_1.clone(), tys2.clone(), bc.clone(), None, None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, ih, store, mods, pre, ci_state, c @ Deref @ SCode::Element::CLASS { name: n, restriction: r, classDef: d, cmt: comment, info, partialPrefix, encapsulatedPrefix, .. }, vis, inst_dims, r#impl, callscope, graph) => {
                            let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                            let mut env_1: FCore::Graph;
                            let mut ci_state_1: ClassInf::State;
                            let mut csets: DAE::Connect::Sets;
                            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                            let mut oDA: Option<SCode::Attributes>;
                            let mut dae: DAE::DAElist;
                            let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                            let mut cache = (*cache).clone();
                            let mut ih = (*ih).clone();
                            let mut store = (*store).clone();
                            let mut graph = (*graph).clone();
                            ErrorExt::setCheckpoint(literal!("instClassParts"));
                            let false = (InstUtil::isBuiltInClass(metamodelica::AsArg::as_arg(&n))?) else { return Err("pattern mismatch") };
                            let () = (match r.clone() {
                SCode::Restriction::R_ENUMERATION { .. } => return Err("fail"),
                _ => (),
            });
                            (cache, env_1, ih, store, dae, csets, ci_state_1, tys, bc, oDA, eqConstraint, graph) = instClassdef(cache.clone(), env.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), metamodelica::AsArg::as_arg(&n), metamodelica::AsArg::as_arg(&d), r.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), metamodelica::AsArg::as_arg(&comment), metamodelica::AsArg::as_arg(&info))?;
                            dae = if (SCodeUtil::isFunction(metamodelica::AsArg::as_arg(&c)) && !(r#impl.clone())) {DAE::DAElist { elementLst: metamodelica::nil() }} else {dae.clone()};
                            ErrorExt::delCheckpoint(literal!("instClassParts"));
                            Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state_1.clone(), tys.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, _, _, ci_state, c @ Deref @ SCode::Element::CLASS { .. }, _, _, r#impl, _, graph) => {
                    let mut b: bool;
                    b = Flags::getConfigBool(Flags::CHECK_MODEL.clone())? && !(r#impl.clone()) && SCodeUtil::isFunction(metamodelica::AsArg::as_arg(&c));
                    if !(b) {
                        ErrorExt::delCheckpoint(literal!("instClassParts"));
                        return Err("fail");
                    } else {
                        ErrorExt::rollBack(literal!("instClassParts"));
                    }
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), inSets.clone(), ci_state.clone(), metamodelica::nil(), None, None, None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        outTypesTypeOption,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ))
}

fn liftNonExpType(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inInstDims: &InstDims,
    mut inSplitArrays: bool,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &((&**inInstDims, inSplitArrays)) {
        (Deref @ metamodelica::ListNode::Cons { head: dims, tail: _ }, false) => {
            Types::liftArrayListDims(inType, dims.clone())
        },
        _ => {
            inType
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

fn getBasicTypeType(mut inName: &ArcStr) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inName.clone()) {
        Deref @ "Real" => DAE::T_REAL_DEFAULT().clone(),
        Deref @ "Integer" => DAE::T_INTEGER_DEFAULT().clone(),
        Deref @ "String" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "Boolean" => DAE::T_BOOL_DEFAULT().clone(),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            DAE::T_CLOCK_DEFAULT().clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outType)
}

fn getBasicTypeAttrTyper(
    mut inName: &ArcStr,
) -> Result<
    Arc<
        dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>>
            + 'static,
    >,
> {
    let mut outTyper: BasicTypeAttrTyper;
    outTyper = (::match_deref::match_deref! { match &(inName.clone()) {
        Deref @ "Real" => (std::sync::Arc::new(move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: SourceInfo| getRealAttributeType(__a0, __a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>> + 'static>),
        Deref @ "Integer" => (std::sync::Arc::new(move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: SourceInfo| getIntAttributeType(__a0, __a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>> + 'static>),
        Deref @ "String" => (std::sync::Arc::new(move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: SourceInfo| getStringAttributeType(__a0, __a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>> + 'static>),
        Deref @ "Boolean" => (std::sync::Arc::new(move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: SourceInfo| getBoolAttributeType(__a0, __a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>> + 'static>),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: SourceInfo| getClockAttributeType(&__a0, &__a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>> + 'static>)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTyper)
}

fn getRealAttributeType(
    mut inAttrName: ArcStr,
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inAttrName.clone()) {
        Deref @ "quantity" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "unit" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "displayUnit" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "min" => inBaseType,
        Deref @ "max" => inBaseType,
        Deref @ "start" => inBaseType,
        Deref @ "fixed" => DAE::T_BOOL_DEFAULT().clone(),
        Deref @ "nominal" => inBaseType,
        Deref @ "stateSelect" => InstBinding::stateSelectType().clone(),
        Deref @ "uncertain" => InstBinding::uncertaintyType().clone(),
        Deref @ "distribution" => InstBinding::distributionType().clone(),
        _ => {
            Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![inAttrName, literal!("Real")], inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getIntAttributeType(
    mut inAttrName: ArcStr,
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inAttrName.clone()) {
        Deref @ "quantity" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "min" => inBaseType,
        Deref @ "max" => inBaseType,
        Deref @ "start" => inBaseType,
        Deref @ "fixed" => DAE::T_BOOL_DEFAULT().clone(),
        Deref @ "nominal" => inBaseType,
        Deref @ "uncertain" => InstBinding::uncertaintyType().clone(),
        Deref @ "distribution" => InstBinding::distributionType().clone(),
        _ => {
            Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![inAttrName, literal!("Integer")], inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getStringAttributeType(
    mut inAttrName: ArcStr,
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inAttrName.clone()) {
        Deref @ "quantity" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "start" => inBaseType,
        _ => {
            Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![inAttrName, literal!("String")], inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getBoolAttributeType(
    mut inAttrName: ArcStr,
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inAttrName.clone()) {
        Deref @ "quantity" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "start" => inBaseType,
        Deref @ "fixed" => DAE::T_BOOL_DEFAULT().clone(),
        _ => {
            Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![inAttrName, literal!("Boolean")], inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getClockAttributeType(
    mut inAttrName: &ArcStr,
    mut inBaseType: &metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match inInfo.clone() {
        _ => return Err("fail"),
    });
    Ok(outType)
}

fn getEnumAttributeType(
    mut inAttrName: ArcStr,
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inAttrName.clone()) {
        Deref @ "quantity" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "min" => inBaseType,
        Deref @ "max" => inBaseType,
        Deref @ "start" => inBaseType,
        Deref @ "fixed" => DAE::T_BOOL_DEFAULT().clone(),
        _ => {
            Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![inAttrName, literal!("enumeration(:)")], inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn instBasicTypeAttributes(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inTypeFunc: Arc<
        dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>>
            + 'static,
    >,
    mut inPrefix: &DAE::Prefix,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    outVars = (match &**inMod {
        DAE::Mod::MOD { subModLst: submods, .. } => List::map4(
            submods.clone(),
            &move |__a0: metamodelica::Ref<DAE::SubMod>,
                   __a1: FCore::Cache,
                   __a2: FCore::Graph,
                   __a3: metamodelica::Ref<DAE::Type>,
                   __a4: Arc<
                dyn ::std::ops::Fn(
                        ArcStr,
                        metamodelica::Ref<DAE::Type>,
                        SourceInfo,
                    ) -> Result<metamodelica::Ref<DAE::Type>>
                    + 'static,
            >| instBasicTypeAttributes2(&__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4)),
            inCache,
            inEnv,
            inBaseType,
            inTypeFunc.clone(),
        )?,
        DAE::Mod::NOMOD { .. } => metamodelica::nil(),
        DAE::Mod::REDECL { .. } => metamodelica::nil(),
    });
    Ok(outVars)
}

fn instBasicTypeAttributes2(
    mut inSubMod: &metamodelica::Ref<DAE::SubMod>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inTypeFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<metamodelica::Ref<DAE::Type>>,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = (::match_deref::match_deref! { match inSubMod {
        Deref @ DAE::SubMod { ident: name, r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, modifierAsValue: val, properties: p, .. }), info, .. } } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            ty = getRealAttributeType(name.clone(), inBaseType, metamodelica::AsArg::as_arg(&info))?;
            instBuiltinAttribute(inCache, inEnv, name.clone(), val.clone(), exp.clone(), ty, metamodelica::AsArg::as_arg(&p))?
        },
        Deref @ DAE::SubMod { ident: name, .. } => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instBasicTypeAttributes2 failed on ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVar)
}

fn instBuiltinAttribute(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut id: ArcStr,
    mut optVal: Option<metamodelica::Ref<Values::Value>>,
    mut bind: metamodelica::Ref<DAE::Exp>,
    mut inExpectedTp: metamodelica::Ref<DAE::Type>,
    mut bindProp: &DAE::Properties,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut var: metamodelica::Ref<DAE::Var>;
    var = 'mc: {
        let __mc_input = (inCache, inEnv, optVal, inExpectedTp, bindProp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(v), expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: c }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut bind1: metamodelica::Ref<DAE::Exp>;
                    let mut vbind: metamodelica::Ref<DAE::Exp>;
                    let mut v = (*v).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { return Err("pattern mismatch") };
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (vbind, _) = Types::matchType(ValuesUtil::valueExp(v.clone(), None)?, bindTp.clone(), expectedTp.clone(), true)?;
                    v = ValuesUtil::expValue(&vbind)?;
                    Ok(metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(v), expectedTp, DAE::Properties::PROP { type_: bindTp @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d, tail: Deref @ metamodelica::ListNode::Nil }, .. }, constFlag: c }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut bind1: metamodelica::Ref<DAE::Exp>;
                    let mut vbind: metamodelica::Ref<DAE::Exp>;
                    let mut v = (*v).clone();
                    let mut expectedTp = (*expectedTp).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { return Err("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    expectedTp = Types::liftArray(expectedTp.clone(), d.clone());
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (vbind, _) = Types::matchType(ValuesUtil::valueExp(v.clone(), None)?, bindTp.clone(), expectedTp.clone(), true)?;
                    v = ValuesUtil::expValue(&vbind)?;
                    Ok(metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: c }) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut bind1: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { return Err("pattern mismatch") };
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (cache, v) = Ceval::ceval(cache.clone(), env.clone(), bind1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    Ok(metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, expectedTp, DAE::Properties::PROP { type_: bindTp @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d, tail: Deref @ metamodelica::ListNode::Nil }, .. }, constFlag: c }) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut bind1: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    let mut expectedTp = (*expectedTp).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { return Err("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    expectedTp = Types::liftArray(expectedTp.clone(), d.clone());
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (cache, v) = Ceval::ceval(cache.clone(), env.clone(), bind1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    Ok(metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: c }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut bind1: metamodelica::Ref<DAE::Exp>;
                    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
                        let true = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { return Err("pattern mismatch") };
                    } else {
                        let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { return Err("pattern mismatch") };
                    }
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    Ok(metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, DAE::Properties::PROP { type_: _, constFlag: c }) => {
                    let mut s: ArcStr;
                    let true = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { return Err("pattern mismatch") };
                    s = ExpressionBasics::printExpStr(bind.clone())?;
                    Error::addMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![id.clone(), literal!("PARAM"), s.clone(), literal!("VAR")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: _ }) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("builtin attribute ")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" of type ")); __mm_s.push_str(&*TypesDump::unparseType(bindTp.clone())?); ArcStr::from(__mm_s) };
                    s2 = TypesDump::unparseType(expectedTp.clone())?;
                    Error::addMessage(Error::TYPE_ERROR.clone(), list![s1.clone(), s2.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(v), expectedTp, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("instBuiltinAttribute failed for: ")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" value binding: ")); __mm_s.push_str(&*ValuesDump::printValStr(metamodelica::AsArg::as_arg(&v))?); __mm_s.push_str(&*literal!(" binding: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(bind.clone())?); __mm_s.push_str(&*literal!(" expected type: ")); __mm_s.push_str(&*TypesDump::printTypeStr(expectedTp.clone())); __mm_s.push_str(&*literal!(" type props: ")); __mm_s.push_str(&*Types::printPropStr(bindProp)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, expectedTp, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("instBuiltinAttribute failed for: ")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" value binding: NONE()")); __mm_s.push_str(&*literal!(" binding: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(bind.clone())?); __mm_s.push_str(&*literal!(" expected type: ")); __mm_s.push_str(&*TypesDump::printTypeStr(expectedTp.clone())); __mm_s.push_str(&*literal!(" type props: ")); __mm_s.push_str(&*Types::printPropStr(bindProp)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(var)
}

fn arrayBasictypeBaseclass(
    mut inInstDims: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<Option<metamodelica::Ref<DAE::Type>>> {
    let mut outOptType: Option<metamodelica::Ref<DAE::Type>>;
    outOptType = (::match_deref::match_deref! { match inInstDims {
        Deref @ metamodelica::ListNode::Nil => {
            None
        },
        _ => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            dims = List::last(inInstDims)?;
            ty = Expression::liftArrayLeftList(inType, dims);
            Some(ty)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOptType)
}

pub fn partialInstClassIn(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut ih: metamodelica::List<InnerOuter::TopInstance>,
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut prefix: DAE::Prefix,
    mut state: ClassInf::State,
    mut cls: metamodelica::Ref<SCode::Element>,
    mut visibility: SCode::Visibility,
    mut instDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut numIter: i32,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ih: metamodelica::List<InnerOuter::TopInstance> = ih;
    let mut state: ClassInf::State = state;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut cache_path: metamodelica::Ref<Absyn::Path>;
    let mut inputs: (
        metamodelica::Ref<DAE::Mod>,
        DAE::Prefix,
        ClassInf::State,
        metamodelica::Ref<SCode::Element>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    );
    let mut outputs: (
        FCore::Graph,
        ClassInf::State,
        metamodelica::List<metamodelica::Ref<DAE::Var>>,
    );
    let mut m: metamodelica::Ref<DAE::Mod>;
    let mut pre: DAE::Prefix;
    let mut st: ClassInf::State;
    let mut e: metamodelica::Ref<SCode::Element>;
    let mut dims: InstDims;
    let mut partial_inst: bool;
    cache_path = generateCachePath(
        &env,
        &cls,
        &prefix,
        openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL,
    )?;
    if Flags::isSet(Flags::CACHE.clone())? {
        if '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(InstHashTable::get(cache_path.clone()), '__try0)) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Some(InstHashTable::CachedInstItem::FUNC_partialInstClassIn { inputs: __pa1, outputs: __pa2 }), tail: Deref @ metamodelica::ListNode::Nil } } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            inputs = metamodelica::Own::own(__pa1);
            outputs = metamodelica::Own::own(__pa2);
            let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(inputs.clone()) {
                (__pa4, __pa5, __pa6, __pa7 @ Deref @ SCode::Element::CLASS { .. }, __pa8) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            m = metamodelica::Own::own(__pa4);
            pre = metamodelica::Own::own(__pa5);
            st = metamodelica::Own::own(__pa6);
            e = metamodelica::Own::own(__pa7);
            dims = metamodelica::Own::own(__pa8);
            unwrap_break_err!(InstUtil::prefixEqualUnlessBasicType(pre.clone(), prefix.clone(), &cls), '__try0);
            if dims.clone() == instDims.clone() && m.clone() == r#mod.clone() && st.clone() == state.clone() && e.clone() == cls.clone() {
                (env, state, vars) = outputs.clone();
                unwrap_break_err!(showCacheInfo(&(literal!("Partial Inst Hit: ")), cache_path.clone()), '__try0);
                return Ok((cache, env, ih, state, vars));
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    if numIter >= Global::recursionDepthLimit.clone() {
        Error::addSourceMessage(
            &(Error::RECURSION_DEPTH_REACHED.clone()),
            list![
                ArcStr::from(::std::format!("{}", Global::recursionDepthLimit.clone())),
                FGraph::printGraphPathStr(&env)
            ],
            &(SCodeUtil::elementInfo(&cls)),
        )?;
        return Err("fail");
    }
    match '__try10: {
        partial_inst = System::getPartialInstantiation();
        System::setPartialInstantiation(true);
        inputs = (
            r#mod.clone(),
            prefix.clone(),
            state.clone(),
            cls.clone(),
            instDims.clone(),
        );
        (cache, env, ih, state, vars) = unwrap_break_err!(partialInstClassIn_dispatch(cache.clone(), env.clone(), ih.clone(), &r#mod, &prefix, state.clone(), &cls, visibility, &instDims, partial_inst, numIter + 1), '__try10);
        outputs = (env.clone(), state.clone(), vars.clone());
        unwrap_break_err!(showCacheInfo(&(literal!("Partial Inst Add: ")), cache_path.clone()), '__try10);
        InstHashTable::addToInstCache(
            cache_path.clone(),
            None,
            Some(InstHashTable::CachedInstItem::FUNC_partialInstClassIn {
                inputs: inputs.clone(),
                outputs: outputs.clone(),
            }),
        );
        Ok::<_, &'static str>((
            cache.clone(),
            env.clone(),
            ih.clone(),
            inputs.clone(),
            outputs.clone(),
            partial_inst.clone(),
            state.clone(),
            vars.clone(),
        ))
    } {
        Ok((__try10_o0, __try10_o1, __try10_o2, __try10_o3, __try10_o4, __try10_o5, __try10_o6, __try10_o7)) => {
            cache = __try10_o0;
            env = __try10_o1;
            ih = __try10_o2;
            inputs = __try10_o3;
            outputs = __try10_o4;
            partial_inst = __try10_o5;
            state = __try10_o6;
            vars = __try10_o7;
        }
        Err(__try10_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- Inst.partialInstClassIn failed on class: "));
                __mm_s.push_str(&*SCodeUtil::elementName(&cls)?);
                __mm_s.push_str(&*literal!(" in environment: "));
                __mm_s.push_str(&*FGraph::printGraphPathStr(&env));
                ArcStr::from(__mm_s)
            })?;
            return Err(__try10_err);
        }
    }
    Ok((cache, env, ih, state, vars))
}

fn partialInstClassIn_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inPrefix: &DAE::Prefix,
    mut inState: ClassInf::State,
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inVisibility: SCode::Visibility,
    mut inInstDims: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut partialInst: bool,
    mut numIter: i32,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH.clone();
    let mut outState: ClassInf::State = inState.clone();
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    let mut success: bool;
    success = 'mc: {
        let __mc_input = &**inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Real", .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Integer", .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "String", .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Boolean", .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. } => {
                    if !((Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())? == 33)) { return Err("guard") }
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = outVars.clone();
                    (outCache, outEnv, outIH, outState, outVars) = partialInstClassdef(&inCache, &inEnv, &inIH, inMod, inPrefix, &inState, inClass, var_field!((**inClass).classDef, SCode::Element::CLASS), inVisibility, inInstDims, numIter)?;
                    Ok((true, outCache.clone(), outEnv.clone(), outIH.clone(), outState.clone(), outVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outEnv = __wb1;
            outIH = __wb2;
            outState = __wb3;
            outVars = __wb4;
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
        return Err("matchcontinue: no arm matched");
    };
    System::setPartialInstantiation(partialInst);
    if !(success) {
        return Err("fail");
    }
    Ok((outCache, outEnv, outIH, outState, outVars))
}

pub(crate) fn instClassdef(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut store: UnitAbsyn::InstStore,
    mut inMod2: metamodelica::Ref<DAE::Mod>,
    mut inPrefix3: DAE::Prefix,
    mut inState5: ClassInf::State,
    mut className: &ArcStr,
    mut inClassDef6: &metamodelica::Ref<SCode::ClassDef>,
    mut inRestriction7: SCode::Restriction,
    mut inVisibility: SCode::Visibility,
    mut inPartialPrefix: SCode::Partial,
    mut inEncapsulatedPrefix: SCode::Encapsulated,
    mut inInstDims9: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplicit: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut instSingleCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut comment: &metamodelica::Ref<SCode::Comment>,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    Option<metamodelica::Ref<DAE::Type>>,
    Option<SCode::Attributes>,
    Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outTypesVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outTypesTypeOption: Option<metamodelica::Ref<DAE::Type>>;
    let mut optDerAttr: Option<SCode::Attributes>;
    let mut outEqualityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        outTypesTypeOption,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ) = instClassdef2(
        inCache,
        inEnv,
        inIH,
        store,
        inMod2,
        inPrefix3,
        inState5,
        className,
        inClassDef6,
        inRestriction7,
        inVisibility,
        inPartialPrefix,
        inEncapsulatedPrefix,
        inInstDims9,
        inImplicit,
        inCallingScope,
        inGraph,
        inSets,
        instSingleCref,
        comment,
        info,
        Mutable::create(false),
    )?;
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        outTypesTypeOption,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ))
}

fn instClassdefBasicType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod2: metamodelica::Ref<DAE::Mod>,
    mut inPrefix3: DAE::Prefix,
    mut inState5: ClassInf::State,
    mut className: ArcStr,
    mut inClassDef6: &metamodelica::Ref<SCode::ClassDef>,
    mut inRestriction7: &SCode::Restriction,
    mut inVisibility: SCode::Visibility,
    mut inInstDims9: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplicit: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut instSingleCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut info: SourceInfo,
    mut stopInst: Mutable::Mutable<bool>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    Option<metamodelica::Ref<DAE::Type>>,
    Option<SCode::Attributes>,
    Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outTypesVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outTypesTypeOption: Option<metamodelica::Ref<DAE::Type>>;
    let mut optDerAttr: Option<SCode::Attributes>;
    let mut outEqualityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        outTypesTypeOption,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inStore,
            inMod2,
            inPrefix3,
            inState5,
            &**inClassDef6,
            inInstDims9,
            inImplicit,
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::ClassDef::PARTS { elementLst: els, normalEquationLst: Deref @ metamodelica::ListNode::Nil, initialEquationLst: Deref @ metamodelica::ListNode::Nil, normalAlgorithmLst: Deref @ metamodelica::ListNode::Nil, initialAlgorithmLst: Deref @ metamodelica::ListNode::Nil, .. }, inst_dims, r#impl, graph) => {
                    let mut cdefelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut compelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut extendselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut cdefelts_1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut cdefelts_2: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut csets: DAE::Connect::Sets;
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut dae: DAE::DAElist;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut mods = (*mods).clone();
                    let mut graph = (*graph).clone();
                    ErrorExt::setCheckpoint(literal!("instClassdefBasicType1"));
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(InstUtil::splitElts(metamodelica::AsArg::as_arg(&els))?) {
                        (__pa0, Deref @ metamodelica::ListNode::Nil, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cdefelts = metamodelica::Own::own(__pa0);
                    extendselts = metamodelica::Own::own(__pa1);
                    compelts = metamodelica::Own::own(__pa2);
                    (cache, env1, ih) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), &cdefelts, r#impl.clone(), Some(mods.clone()), false)?;
                    cdefelts_1 = InstUtil::addNomod(cdefelts.clone());
                    env2 = env1.clone();
                    cdefelts_2 = cdefelts_1.clone();
                    (cache, env3, ih, store, dae1, csets, _, tys, graph, _) = instElementList(cache.clone(), env2.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), cdefelts_2.clone(), inst_dims.clone(), r#impl.clone(), openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, graph.clone(), inSets.clone(), true)?;
                    mods = Mod::removeFirstSubsRedecl(mods.clone());
                    ErrorExt::rollBack(literal!("instClassdefBasicType1"));
                    (cache, ih, store, dae2, bc, tys) = instBasictypeBaseclass(cache.clone(), env3.clone(), ih.clone(), store.clone(), &extendselts, &compelts, mods.clone(), inst_dims.clone(), className.clone(), info.clone(), stopInst.clone())?;
                    eqConstraint = InstUtil::equalityConstraint(&env3, metamodelica::AsArg::as_arg(&els), &info);
                    dae = DAEUtil::joinDaes(&dae1, &dae2)?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), None, eqConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, Deref @ SCode::ClassDef::PARTS { normalEquationLst: Deref @ metamodelica::ListNode::Nil, initialEquationLst: Deref @ metamodelica::ListNode::Nil, normalAlgorithmLst: Deref @ metamodelica::ListNode::Nil, initialAlgorithmLst: Deref @ metamodelica::ListNode::Nil, .. }, _, _, _) => {
                    let true = (ErrorExt::isTopCheckpoint(literal!("instClassdefBasicType1"))) else { return Err("pattern mismatch") };
                    ErrorExt::rollBack(literal!("instClassdefBasicType1"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        outTypesTypeOption,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ))
}

fn instClassdef2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod2: metamodelica::Ref<DAE::Mod>,
    mut inPrefix3: DAE::Prefix,
    mut inState5: ClassInf::State,
    mut className: &ArcStr,
    mut inClassDef6: &metamodelica::Ref<SCode::ClassDef>,
    mut inRestriction7: SCode::Restriction,
    mut inVisibility: SCode::Visibility,
    mut inPartialPrefix: SCode::Partial,
    mut inEncapsulatedPrefix: SCode::Encapsulated,
    mut inInstDims9: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplicit: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut instSingleCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut comment: &metamodelica::Ref<SCode::Comment>,
    mut info: &SourceInfo,
    mut stopInst: Mutable::Mutable<bool>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    Option<metamodelica::Ref<DAE::Type>>,
    Option<SCode::Attributes>,
    Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut outSets: DAE::Connect::Sets = <DAE::Connect::Sets as ::std::default::Default>::default();
    let mut outState: ClassInf::State = <ClassInf::State as ::std::default::Default>::default();
    let mut outTypesVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    let mut oty: Option<metamodelica::Ref<DAE::Type>> = None;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut outEqualityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut outGraph: ConnectionGraph::ConnectionGraph =
        <ConnectionGraph::ConnectionGraph as ::std::default::Default>::default();
    (
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        oty,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv.clone(),
            inIH,
            inStore,
            inMod2,
            inPrefix3,
            inState5,
            &**inClassDef6,
            inRestriction7,
            inVisibility,
            inPartialPrefix,
            inEncapsulatedPrefix,
            inInstDims9,
            inImplicit,
            inCallingScope,
            inGraph,
            inSets.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::ClassDef::PARTS { elementLst: els, normalEquationLst: Deref @ metamodelica::ListNode::Nil, initialEquationLst: Deref @ metamodelica::ListNode::Nil, normalAlgorithmLst: Deref @ metamodelica::ListNode::Nil, initialAlgorithmLst: Deref @ metamodelica::ListNode::Nil, .. }, re, vis, _, _, inst_dims, r#impl, _, graph, _) => {
                    let mut cdefelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut extendselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut extendsclasselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut compelts_2_elem: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut env1: FCore::Graph;
                    let mut extcomps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut csets: DAE::Connect::Sets;
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut fdae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut ci_state = (*ci_state).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let false = (openmodelica_frontend_types::SCode::Restriction::R_MODEL == re.clone()) else { return Err("pattern mismatch") };
                    let false = (openmodelica_frontend_types::SCode::Restriction::R_PACKAGE == re.clone()) else { return Err("pattern mismatch") };
                    let false = (SCodeUtil::isFunctionRestriction(metamodelica::AsArg::as_arg(&re))) else { return Err("pattern mismatch") };
                    let false = (SCode::Restriction::R_RECORD { isOperator: true } == re.clone()) else { return Err("pattern mismatch") };
                    let false = (SCode::Restriction::R_RECORD { isOperator: false } == re.clone()) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(InstUtil::splitElts(metamodelica::AsArg::as_arg(&els))?) {
                        (__pa0, __pa1, __pa2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Nil) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cdefelts = metamodelica::Own::own(__pa0);
                    extendsclasselts = metamodelica::Own::own(__pa1);
                    extendselts = metamodelica::Own::own(__pa2);
                    extendselts = SCodeInstUtil::addRedeclareAsElementsToExtends(&extendselts, List::select(els.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isRedeclareElement(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?)?;
                    (cache, env1, ih) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), &cdefelts, r#impl.clone(), Some(mods.clone()), false)?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(InstExtends::instExtendsAndClassExtendsList(cache.clone(), env1.clone(), ih.clone(), mods.clone(), pre.clone(), extendselts.clone(), &extendsclasselts, els.clone(), metamodelica::AsArg::as_arg(&ci_state), className.clone(), r#impl.clone(), false)?) {
                        (__pa3, _, _, _, __pa4, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    extcomps = metamodelica::Own::own(__pa4);
                    compelts_2_elem = List::map(extcomps.clone(), &fnptr!(Util::tuple21, _))?;
                    ::match_deref::match_deref! { match &(InstUtil::splitElts(&compelts_2_elem)?) {
                        (_, _, _, Deref @ metamodelica::ListNode::Nil) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (cache, env, ih, store, fdae, csets, ci_state, vars, bc, oDA, eqConstraint, graph) = instClassdefBasicType(cache.clone(), env.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), className.clone(), inClassDef6, metamodelica::AsArg::as_arg(&re), vis.clone(), inst_dims.clone(), r#impl.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), info.clone(), stopInst.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), fdae.clone(), csets.clone(), ci_state.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, _, ci_state, Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, _, _, _, _, _, r#impl, _, graph, _) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut ci_state = (*ci_state).clone();
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::isExternalObject(metamodelica::AsArg::as_arg(&els))) else { return Err("pattern mismatch") };
                    (cache, env, ih, dae, ci_state) = InstFunction::instantiateExternalObject(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&els), mods.clone(), r#impl.clone(), comment.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), inSets.clone(), ci_state.clone(), metamodelica::nil(), None, None, None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::ClassDef::PARTS { elementLst: els, normalEquationLst: eqs, initialEquationLst: initeqs, normalAlgorithmLst: alg, initialAlgorithmLst: initalg, constraintLst: constrs, clsattrs, externalDecl: ed }, re, _, _, _, inst_dims, r#impl, callscope, graph, csets) => {
                            let mut cdefelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                            let mut compelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                            let mut extendselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                            let mut extendsclasselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                            let mut compelts_2_elem: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                            let mut env1: FCore::Graph;
                            let mut env2: FCore::Graph;
                            let mut env3: FCore::Graph;
                            let mut env5: FCore::Graph;
                            let mut cdefelts_1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                            let mut extcomps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                            let mut compelts_1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                            let mut compelts_2: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                            let mut comp_cond: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                            let mut csets1: DAE::Connect::Sets;
                            let mut csets2: DAE::Connect::Sets;
                            let mut csets3: DAE::Connect::Sets;
                            let mut csets4: DAE::Connect::Sets;
                            let mut csets5: DAE::Connect::Sets;
                            let mut dae1: DAE::DAElist;
                            let mut dae2: DAE::DAElist;
                            let mut dae3: DAE::DAElist;
                            let mut dae4: DAE::DAElist;
                            let mut dae5: DAE::DAElist;
                            let mut dae6: DAE::DAElist;
                            let mut dae7: DAE::DAElist;
                            let mut dae8: DAE::DAElist;
                            let mut dae: DAE::DAElist;
                            let mut ci_state1: ClassInf::State;
                            let mut ci_state2: ClassInf::State;
                            let mut ci_state3: ClassInf::State;
                            let mut ci_state4: ClassInf::State;
                            let mut ci_state5: ClassInf::State;
                            let mut ci_state6: ClassInf::State;
                            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                            let mut emods: metamodelica::Ref<DAE::Mod>;
                            let mut checkMods: metamodelica::Ref<DAE::Mod>;
                            let mut eqs2: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                            let mut initeqs2: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                            let mut eqs_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                            let mut initeqs_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                            let mut alg2: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                            let mut initalg2: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                            let mut alg_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                            let mut initalg_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                            let mut comments: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                            let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                            let mut unrollForLoops: bool;
                            let mut zero_dims: bool;
                            let mut ty: metamodelica::Ref<DAE::Type>;
                            let mut elementSource: metamodelica::Ref<DAE::ElementSource>;
                            let mut smCompCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut smInitialCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut smCompToFlatSM: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                            let mut domainFieldsLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>)>;
                            let mut cache = (*cache).clone();
                            let mut ih = (*ih).clone();
                            let mut store = (*store).clone();
                            let mut mods = (*mods).clone();
                            let mut els = (*els).clone();
                            let mut graph = (*graph).clone();
                            let mut csets = (*csets).clone();
                            let mut oty: Option<metamodelica::Ref<DAE::Type>> = oty.clone();
                            let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                            let false = (SCodeUtil::isExternalObject(metamodelica::AsArg::as_arg(&els))) else { return Err("pattern mismatch") };
                            ci_state1 = ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::NEWDEF)?;
                            els = InstUtil::extractConstantPlusDeps(els.clone(), instSingleCref.clone(), metamodelica::nil(), className)?;
                            (cdefelts, extendsclasselts, extendselts, compelts) = InstUtil::splitElts(metamodelica::AsArg::as_arg(&els))?;
                            extendselts = SCodeInstUtil::addRedeclareAsElementsToExtends(&extendselts, List::select(els.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isRedeclareElement(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?)?;
                            (cache, env1, ih) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), &cdefelts, r#impl.clone(), Some(mods.clone()), FGraph::isEmptyScope(metamodelica::AsArg::as_arg(&env)))?;
                            (cache, env2, ih, emods, extcomps, eqs2, initeqs2, alg2, initalg2, comments) = InstExtends::instExtendsAndClassExtendsList(cache.clone(), env1.clone(), ih.clone(), mods.clone(), pre.clone(), extendselts.clone(), &extendsclasselts, els.clone(), metamodelica::AsArg::as_arg(&ci_state), className.clone(), r#impl.clone(), false)?;
                            compelts_1 = InstUtil::addNomod(compelts.clone());
                            cdefelts_1 = InstUtil::addNomod(cdefelts.clone());
                            compelts_1 = List::flatten(list![extcomps.clone(), compelts_1.clone(), cdefelts_1.clone()])?;
                            eqs_1 = joinExtEquations(eqs.clone(), eqs2.clone(), callscope.clone());
                            initeqs_1 = joinExtEquations(initeqs.clone(), initeqs2.clone(), callscope.clone());
                            alg_1 = joinExtAlgorithms(alg.clone(), alg2.clone(), callscope.clone());
                            initalg_1 = joinExtAlgorithms(initalg.clone(), initalg2.clone(), callscope.clone());
                            (compelts_1, eqs_1, initeqs_1, alg_1, initalg_1) = InstUtil::extractConstantPlusDepsTpl(compelts_1.clone(), instSingleCref.clone(), metamodelica::nil(), className, eqs_1.clone(), initeqs_1.clone(), alg_1.clone(), initalg_1.clone())?;
                            if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                                compelts_1 = InstUtil::addGhostCells(compelts_1.clone(), &eqs_1)?;
                            }
                            checkMods = Mod::merge(mods.clone(), emods.clone(), className.clone(), true)?;
                            mods = checkMods.clone();
                            (cache, env3, ih) = InstUtil::addComponentsToEnv(cache.clone(), env2.clone(), ih.clone(), mods.clone(), pre.clone(), ci_state.clone(), &compelts_1, r#impl.clone())?;
                            compelts_2_elem = List::map(compelts_1.clone(), &fnptr!(Util::tuple21, _))?;
                            InstUtil::matchModificationToComponents(&compelts_2_elem, checkMods.clone(), &(FGraph::printGraphPathStr(&env3)))?;
                            (comp_cond, compelts_1) = List::splitOnTrue(&compelts_1, &move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(InstUtil::componentHasCondition(&__a0)) })?;
                            compelts_2 = listAppend(compelts_1.clone(), comp_cond.clone());
                            (smCompCrefs, smInitialCrefs) = InstStateMachineUtil::getSMStatesInContext(eqs_1.clone(), pre.clone())?;
                            ih = List::fold(&smCompCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<InnerOuter::TopInstance>| InnerOuter::updateSMHierarchy(__a0, &__a1), ih.clone())?;
                            (cache, env5, ih, store, dae1, csets, ci_state2, vars, graph, domainFieldsLst) = instElementList(cache.clone(), env3.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state1.clone(), compelts_2.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), csets.clone(), true)?;
                            zero_dims = InstUtil::instDimsHasZeroDims(metamodelica::AsArg::as_arg(&inst_dims));
                            elementSource = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(&env3)?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                            csets1 = ConnectUtil::addConnectorVariablesFromDAE(zero_dims, &ci_state1, pre.clone(), &vars, info, elementSource.clone(), csets.clone())?;
                            (cache, eqs_1) = InstUtil::reorderConnectEquationsExpandable(cache.clone(), &env5, eqs_1.clone())?;
                            if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                                eqs_1 = List::fold1(&eqs_1, &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>)>, __a2: metamodelica::List<metamodelica::Ref<SCode::Equation>>| InstUtil::discretizePDE(__a0, &__a1, __a2), domainFieldsLst.clone(), metamodelica::nil())?;
                            }
                            (cache, env5, ih, dae2, csets2, ci_state3, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets1.clone(), ci_state2.clone(), &InstSection::instEquation, &eqs_1, r#impl.clone(), InstTypes::alwaysUnroll.clone(), graph.clone())?;
                            DAEUtil::verifyEquationsDAE(dae2.clone())?;
                            if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                                initeqs_1 = List::fold1(&initeqs_1, &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>)>, __a2: metamodelica::List<metamodelica::Ref<SCode::Equation>>| InstUtil::discretizePDE(__a0, &__a1, __a2), domainFieldsLst.clone(), metamodelica::nil())?;
                            }
                            (cache, env5, ih, dae3, csets3, ci_state4, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets2.clone(), ci_state3.clone(), &InstSection::instInitialEquation, &initeqs_1, r#impl.clone(), InstTypes::alwaysUnroll.clone(), graph.clone())?;
                            unrollForLoops = if (SCodeUtil::isFunctionRestriction(metamodelica::AsArg::as_arg(&re))) {InstTypes::neverUnroll.clone()} else {InstTypes::alwaysUnroll.clone()};
                            (cache, env5, ih, dae4, csets4, ci_state5, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets3.clone(), ci_state4.clone(), &move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<InnerOuter::TopInstance>, __a3: DAE::Prefix, __a4: DAE::Connect::Sets, __a5: ClassInf::State, __a6: metamodelica::Ref<SCode::AlgorithmSection>, __a7: bool, __a8: bool, __a9: ConnectionGraph::ConnectionGraph| InstSection::instAlgorithm(__a0, __a1, __a2, __a3, __a4, __a5, &__a6, __a7, __a8, __a9), &alg_1, r#impl.clone(), unrollForLoops, graph.clone())?;
                            (cache, env5, ih, dae5, csets5, ci_state6, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets4.clone(), ci_state5.clone(), &move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<InnerOuter::TopInstance>, __a3: DAE::Prefix, __a4: DAE::Connect::Sets, __a5: ClassInf::State, __a6: metamodelica::Ref<SCode::AlgorithmSection>, __a7: bool, __a8: bool, __a9: ConnectionGraph::ConnectionGraph| InstSection::instInitialAlgorithm(__a0, __a1, __a2, __a3, __a4, __a5, &__a6, __a7, __a8, __a9), &initalg_1, r#impl.clone(), unrollForLoops, graph.clone())?;
                            (cache, env5, dae6) = instClassAttributes(cache.clone(), env5.clone(), pre.clone(), clsattrs.clone(), r#impl.clone(), info)?;
                            (cache, env5, dae7, _) = instConstraints(metamodelica::AsArg::as_arg(&cache), &env5, metamodelica::AsArg::as_arg(&pre), &ci_state6, metamodelica::AsArg::as_arg(&constrs), r#impl.clone())?;
                            dae8 = instFunctionAnnotations(&(metamodelica::cons(comment.clone(), comments.clone())), &ci_state6);
                            smCompToFlatSM = InstStateMachineUtil::createSMNodeToFlatSMGroupTable(dae2.clone())?;
                            (dae1, dae2) = InstStateMachineUtil::wrapSMCompsInFlatSMs(ih.clone(), dae1.clone(), dae2.clone(), smCompToFlatSM.clone(), smInitialCrefs.clone())?;
                            dae = DAEUtil::joinDaeLst(&(list![dae1.clone(), dae2.clone(), dae3.clone(), dae4.clone(), dae5.clone(), dae6.clone(), dae7.clone(), dae8.clone()]))?;
                            csets5 = InnerOuter::changeInnerOuterInOuterConnect(csets5.clone())?;
                            eqConstraint = InstUtil::equalityConstraint(&env5, metamodelica::AsArg::as_arg(&els), info);
                            ci_state6 = if ((ed).is_some()) {ClassInfUtil::assertTrans(ci_state6.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_EXT_DECL, info)?} else {ci_state6.clone()};
                            (cache, oty) = InstMeta::fixUniontype(cache.clone(), env5.clone(), &ci_state6, inClassDef6)?;
                            let () = (::match_deref::match_deref! { match &(&oty) {
                Some(__esc_ty @ Deref @ DAE::Type::T_METAUNIONTYPE { typeVars: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }) => {
                            ty = (*__esc_ty).clone();
                            Error::addSourceMessage(&(Error::UNIONTYPE_MISSING_TYPEVARS.clone()), list![TypesDump::unparseType(ty.clone())?], info)?;
                            return Err("fail")
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            Ok(((cache.clone(), env5.clone(), ih.clone(), store.clone(), dae.clone(), csets5.clone(), ci_state6.clone(), vars.clone(), oty.clone(), None, eqConstraint.clone(), graph.clone()), oty.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            oty = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: ad }, modifications: r#mod, attributes: DA }, re, vis, _, _, inst_dims, r#impl, callscope, graph, _) => {
                    let mut env3: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut cenv_2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut csets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut ci_state2: ClassInf::State;
                    let mut new_ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut mods_1: metamodelica::Ref<DAE::Mod>;
                    let mut r: SCode::Restriction;
                    let mut enc2: SCode::Encapsulated;
                    let mut inst_dims_1: InstDims;
                    let mut cn2: ArcStr;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut eq: Option<DAE::EqMod>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), Some(info.clone()))?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3 @ SCode::Restriction::R_ENUMERATION { .. }, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cn2 = metamodelica::Own::own(__pa1);
                    enc2 = metamodelica::Own::own(__pa2);
                    r = metamodelica::Own::own(__pa3);
                    c = metamodelica::Own::own(__pa4);
                    cenv = metamodelica::Own::own(__pa5);
                    env3 = FGraph::openScope(cenv.clone(), enc2, cn2.clone(), Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE))?;
                    ci_state2 = ClassInfUtil::start(&r, FGraph::getGraphName(&env3)?)?;
                    new_ci_state = ClassInfUtil::start(&r, FGraph::getGraphName(&env3)?)?;
                    (cache, cenv_2, _, _, _, _, _, _, _, _, _, _) = instClassIn(cache.clone(), env3.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state2.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), false, callscope.clone(), ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), None)?;
                    (cache, mod_1) = Mod::elabMod(cache.clone(), cenv_2.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    mods_1 = Mod::merge(mods.clone(), mod_1.clone(), className.clone(), true)?;
                    eq = Mod::modEquation(&mods_1);
                    (cache, dims) = InstUtil::elabArraydimOpt(cache.clone(), cenv_2.clone(), metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!(""), subscripts: metamodelica::nil() }), cn.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, pre.clone(), info.clone(), inst_dims.clone())?;
                    inst_dims_1 = List::appendLastList(metamodelica::AsArg::as_arg(&inst_dims), dims.clone())?;
                    (cache, env_2, ih, store, dae, csets_1, ci_state_1, vars, bc, oDA, eqConstraint, graph) = instClassIn(cache.clone(), cenv_2.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), new_ci_state.clone(), c.clone(), vis.clone(), inst_dims_1.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone())?;
                    ClassInfUtil::assertValid(ci_state_1.clone(), re.clone(), info)?;
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), store.clone(), dae.clone(), csets_1.clone(), ci_state_1.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: ad }, modifications: r#mod, attributes: DA }, re, vis, _, _, inst_dims, r#impl, callscope, graph, _) => {
                    let mut cenv: FCore::Graph;
                    let mut cenv_2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut parentEnv: FCore::Graph;
                    let mut csets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut new_ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut mods_1: metamodelica::Ref<DAE::Mod>;
                    let mut r: SCode::Restriction;
                    let mut valid_connector: bool;
                    let mut enc2: SCode::Encapsulated;
                    let mut inst_dims_1: InstDims;
                    let mut cn2: ArcStr;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut eq: Option<DAE::EqMod>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), Some(info.clone()))?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cn2 = metamodelica::Own::own(__pa1);
                    enc2 = metamodelica::Own::own(__pa2);
                    r = metamodelica::Own::own(__pa3);
                    c = metamodelica::Own::own(__pa4);
                    cenv = metamodelica::Own::own(__pa5);
                    let true = (InstUtil::checkDerivedRestriction(re.clone(), r.clone(), cn2.clone())?) else { return Err("pattern mismatch") };
                    valid_connector = ConnectUtil::checkShortConnectorDef(metamodelica::AsArg::as_arg(&ci_state), metamodelica::AsArg::as_arg(&DA), info)?;
                    Mutable::update(stopInst.clone(), !(valid_connector));
                    let true = (valid_connector) else { return Err("pattern mismatch") };
                    cenv_2 = FGraph::openScope(cenv.clone(), enc2, cn2.clone(), FGraph::classInfToScopeType(metamodelica::AsArg::as_arg(&ci_state)))?;
                    new_ci_state = ClassInfUtil::start(&r, FGraph::getGraphName(&cenv_2)?)?;
                    r#mod = InstUtil::chainRedeclares(metamodelica::AsArg::as_arg(&mods), r#mod.clone());
                    (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    mods_1 = Mod::merge(mods.clone(), mod_1.clone(), className.clone(), true)?;
                    eq = Mod::modEquation(&mods_1);
                    (cache, dims) = InstUtil::elabArraydimOpt(cache.clone(), parentEnv.clone(), metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!(""), subscripts: metamodelica::nil() }), cn.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, pre.clone(), info.clone(), inst_dims.clone())?;
                    inst_dims_1 = List::appendLastList(metamodelica::AsArg::as_arg(&inst_dims), dims.clone())?;
                    (cache, env_2, ih, store, dae, csets_1, ci_state_1, vars, bc, oDA, eqConstraint, graph) = instClassIn(cache.clone(), cenv_2.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), new_ci_state.clone(), c.clone(), vis.clone(), inst_dims_1.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone())?;
                    ClassInfUtil::assertValid(ci_state_1.clone(), re.clone(), info)?;
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), store.clone(), dae.clone(), csets_1.clone(), ci_state_1.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: ad }, modifications: r#mod, attributes: DA }, re, vis, partialPrefix, encapsulatedPrefix, inst_dims, r#impl, callscope, graph, _) => {
                            let mut parentEnv: FCore::Graph;
                            let mut parentClassEnv: FCore::Graph;
                            let mut csets: DAE::Connect::Sets;
                            let mut dae: DAE::DAElist;
                            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                            let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                            let mut mod_1: metamodelica::Ref<DAE::Mod>;
                            let mut mods_1: metamodelica::Ref<DAE::Mod>;
                            let mut r: SCode::Restriction;
                            let mut cn2: ArcStr;
                            let mut classDefParent: metamodelica::Ref<SCode::ClassDef>;
                            let mut oDA: Option<SCode::Attributes>;
                            let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                            let mut cache = (*cache).clone();
                            let mut env = (*env).clone();
                            let mut ih = (*ih).clone();
                            let mut store = (*store).clone();
                            let mut ci_state = (*ci_state).clone();
                            let mut r#mod = (*r#mod).clone();
                            let mut graph = (*graph).clone();
                            let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                            let false = (re.clone() == openmodelica_frontend_types::SCode::Restriction::R_TYPE) else { return Err("pattern mismatch") };
                            let false = (re.clone() == openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION) else { return Err("pattern mismatch") };
                            let false = (re.clone() == openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION) else { return Err("pattern mismatch") };
                            let false = (SCodeUtil::isConnector(metamodelica::AsArg::as_arg(&re))) else { return Err("pattern mismatch") };
                            let true = (boolOr((ad.clone()).is_none(), ad.clone() == Some(metamodelica::nil()))) else { return Err("pattern mismatch") };
                            let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), Some(info.clone()))?) {
                                (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, restriction: __pa2, classDef: __pa3, .. }, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            cn2 = metamodelica::Own::own(__pa1);
                            r = metamodelica::Own::own(__pa2);
                            classDefParent = metamodelica::Own::own(__pa3);
                            parentClassEnv = metamodelica::Own::own(__pa4);
                            let false = (InstUtil::checkDerivedRestriction(re.clone(), r.clone(), cn2.clone())?) else { return Err("pattern mismatch") };
                            if (match r.clone() {
                SCode::Restriction::R_PACKAGE { .. } => false,
                _ => if (SCodeUtil::restrictionEqual(&r, metamodelica::AsArg::as_arg(&re))) {Mod::isInvariantMod(r#mod.clone())? && Mod::isInvariantDAEMod(metamodelica::AsArg::as_arg(&mods))?} else {false},
            }) {
                                r#mod = InstUtil::chainRedeclares(metamodelica::AsArg::as_arg(&mods), r#mod.clone());
                                (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                                (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), false, Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                                mods_1 = Mod::merge(mods.clone(), mod_1.clone(), className.clone(), true)?;
                                (cache, env, ih, store, dae, csets, ci_state, vars, bc, oDA, eqConstraint, graph) = instClassdef2(cache.clone(), parentClassEnv.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), ci_state.clone(), className, &classDefParent, re.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), comment, info, stopInst.clone())?;
                                oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                            } else {
                                r#mod = InstUtil::chainRedeclares(metamodelica::AsArg::as_arg(&mods), r#mod.clone());
                                (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                                (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), false, Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                                mods_1 = Mod::merge(mods.clone(), mod_1.clone(), className.clone(), true)?;
                                (cache, env, ih, store, dae, csets, ci_state, vars, bc, oDA, eqConstraint, graph) = instClassdef2(cache.clone(), env.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), ci_state.clone(), className, &(metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: list![metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: cn.clone(), visibility: vis.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), ann: None, info: info.clone() })], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None })), re.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), comment, info, stopInst.clone())?;
                                oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                            }
                            Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: ad }, modifications: r#mod, attributes: DA }, re, vis, _, _, inst_dims, r#impl, callscope, graph, _) => {
                    let mut cenv: FCore::Graph;
                    let mut cenv_2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut parentEnv: FCore::Graph;
                    let mut csets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut new_ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut mods_1: metamodelica::Ref<DAE::Mod>;
                    let mut r: SCode::Restriction;
                    let mut enc2: SCode::Encapsulated;
                    let mut inst_dims_1: InstDims;
                    let mut cn2: ArcStr;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut eq: Option<DAE::EqMod>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut eqConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), Some(info.clone()))?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cn2 = metamodelica::Own::own(__pa1);
                    enc2 = metamodelica::Own::own(__pa2);
                    r = metamodelica::Own::own(__pa3);
                    c = metamodelica::Own::own(__pa4);
                    cenv = metamodelica::Own::own(__pa5);
                    let false = (InstUtil::checkDerivedRestriction(re.clone(), r.clone(), cn2.clone())?) else { return Err("pattern mismatch") };
                    cenv_2 = FGraph::openScope(cenv.clone(), enc2, className.clone(), FGraph::classInfToScopeType(metamodelica::AsArg::as_arg(&ci_state)))?;
                    new_ci_state = ClassInfUtil::start(&r, FGraph::getGraphName(&cenv_2)?)?;
                    c = SCodeUtil::setClassName(className.clone(), c.clone())?;
                    r#mod = InstUtil::chainRedeclares(metamodelica::AsArg::as_arg(&mods), r#mod.clone());
                    (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    mods_1 = Mod::merge(mods.clone(), mod_1.clone(), className.clone(), true)?;
                    eq = Mod::modEquation(&mods_1);
                    (cache, dims) = InstUtil::elabArraydimOpt(cache.clone(), parentEnv.clone(), metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!(""), subscripts: metamodelica::nil() }), cn.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, pre.clone(), info.clone(), inst_dims.clone())?;
                    inst_dims_1 = List::appendLastList(metamodelica::AsArg::as_arg(&inst_dims), dims.clone())?;
                    (cache, env_2, ih, store, dae, csets_1, ci_state_1, vars, bc, oDA, eqConstraint, graph) = instClassIn(cache.clone(), cenv_2.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), new_ci_state.clone(), c.clone(), vis.clone(), inst_dims_1.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone())?;
                    ClassInfUtil::assertValid(ci_state_1.clone(), re.clone(), info)?;
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), store.clone(), dae.clone(), csets_1.clone(), ci_state_1.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, mods, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { .. }, modifications: r#mod, .. }, _, _, _, _, _, _, _, _, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::META_COMPLEX_TYPE_MOD.clone()), metamodelica::nil(), info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, typeSpecs: Deref @ metamodelica::ListNode::Cons { head: tSpec, tail: Deref @ metamodelica::ListNode::Nil }, arrayDim: None }, modifications: r#mod, attributes: DA }, _, _, _, _, inst_dims, r#impl, _, graph, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                    (cache, _, ih, tys, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), &(list![tSpec.clone()]), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), &inSets, info)?;
                    ty = (tys).head().cloned()?;
                    ty = Types::boxIfUnboxedType(ty.clone());
                    bc = Some(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_LIST { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Option" }, typeSpecs: Deref @ metamodelica::ListNode::Cons { head: tSpec, tail: Deref @ metamodelica::ListNode::Nil }, arrayDim: None }, modifications: r#mod, attributes: DA }, _, _, _, _, inst_dims, r#impl, _, graph, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(instClassDefHelper(cache.clone(), env.clone(), ih.clone(), &(list![tSpec.clone()]), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), &inSets, info)?) {
                        (__pa0, _, __pa1, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ih = metamodelica::Own::own(__pa1);
                    ty = metamodelica::Own::own(__pa2);
                    csets = metamodelica::Own::own(__pa3);
                    oDA = metamodelica::Own::own(__pa4);
                    ty = Types::boxIfUnboxedType(ty.clone());
                    bc = Some(metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: ty.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_OPTION { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tuple" }, typeSpecs: tSpecs, arrayDim: None }, modifications: r#mod, attributes: DA }, _, _, _, _, inst_dims, r#impl, _, graph, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                    (cache, _, ih, tys, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&tSpecs), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), &inSets, info)?;
                    tys = List::map(tys.clone(), &fnptr!(Types::boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    bc = Some(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_TUPLE { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, typeSpecs: Deref @ metamodelica::ListNode::Cons { head: tSpec, tail: Deref @ metamodelica::ListNode::Nil }, arrayDim: None }, modifications: r#mod, attributes: DA }, _, _, _, _, inst_dims, r#impl, _, graph, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(instClassDefHelper(cache.clone(), env.clone(), ih.clone(), &(list![tSpec.clone()]), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), &inSets, info)?) {
                        (__pa0, _, __pa1, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ih = metamodelica::Own::own(__pa1);
                    ty = metamodelica::Own::own(__pa2);
                    csets = metamodelica::Own::own(__pa3);
                    oDA = metamodelica::Own::own(__pa4);
                    ty = Types::boxIfUnboxedType(ty.clone());
                    bc = Some(metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: ty.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_ARRAY { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: className.clone() }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, typeSpecs: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Any" }, arrayDim: None }, tail: Deref @ metamodelica::ListNode::Nil }, arrayDim: None }, modifications: r#mod, attributes: DA }, _, _, _, _, inst_dims, r#impl, _, graph, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                    (cache, _, ih, _, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), &(metamodelica::nil()), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), &inSets, info)?;
                    bc = Some(metamodelica::Ref::new(DAE::Type::T_METAPOLYMORPHIC { name: className.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_POLYMORPHIC { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: className.clone() }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, mods, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. }, modifications: r#mod, .. }, _, _, _, _, _, _, _, _, _) => {
                    let true = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::META_POLYMORPHIC.clone()), list![className.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11)) =
            (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: r#str }, typeSpecs: tSpecs, arrayDim: None }, modifications: r#mod, attributes: DA }, re, vis, partialPrefix, encapsulatedPrefix, inst_dims, r#impl, _, graph, _) => {
                        let mut r#str = (*r#str).clone();
                        let mut optDerAttr: Option<SCode::Attributes> = optDerAttr.clone();
                        let mut oty: Option<metamodelica::Ref<DAE::Type>> = oty.clone();
                        let mut outCache: FCore::Cache = outCache.clone();
                        let mut outDae: DAE::DAElist = outDae.clone();
                        let mut outEnv: FCore::Graph = outEnv.clone();
                        let mut outEqualityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)> = outEqualityConstraint.clone();
                        let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                        let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                        let mut outSets: DAE::Connect::Sets = outSets.clone();
                        let mut outState: ClassInf::State = outState.clone();
                        let mut outStore: UnitAbsyn::InstStore = outStore.clone();
                        let mut outTypesVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>> = outTypesVarLst.clone();
                        r#str = Util::assoc(r#str.clone(), list![(literal!("List"), literal!("list")), (literal!("Tuple"), literal!("tuple")), (literal!("Array"), literal!("array"))])?;
                        (outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, oty, optDerAttr, outEqualityConstraint, outGraph) = instClassdef2(cache.clone(), env.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), className, &(metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TCOMPLEX { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: r#str.clone() }), typeSpecs: tSpecs.clone(), arrayDim: None }), modifications: r#mod.clone(), attributes: DA.clone() })), re.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), inCallingScope, graph.clone(), inSets.clone(), instSingleCref.clone(), comment, info, stopInst.clone())?;
                        Ok(((outCache.clone(), outEnv.clone(), outIH.clone(), outStore.clone(), outDae.clone(), outSets.clone(), outState.clone(), outTypesVarLst.clone(), oty.clone(), optDerAttr.clone(), outEqualityConstraint.clone(), outGraph.clone()), optDerAttr.clone(), oty.clone(), outCache.clone(), outDae.clone(), outEnv.clone(), outEqualityConstraint.clone(), outGraph.clone(), outIH.clone(), outSets.clone(), outState.clone(), outStore.clone(), outTypesVarLst.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })()
        {
            optDerAttr = __wb0;
            oty = __wb1;
            outCache = __wb2;
            outDae = __wb3;
            outEnv = __wb4;
            outEqualityConstraint = __wb5;
            outGraph = __wb6;
            outIH = __wb7;
            outSets = __wb8;
            outState = __wb9;
            outStore = __wb10;
            outTypesVarLst = __wb11;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, ih, store, mods, pre, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: cn, typeSpecs: tSpecs, arrayDim: None }, modifications: r#mod, attributes: DA }, _, _, _, _, inst_dims, r#impl, _, graph, _) => {
                            let mut cenv: FCore::Graph;
                            let mut csets: DAE::Connect::Sets;
                            let mut new_ci_state: ClassInf::State;
                            let mut bc: Option<metamodelica::Ref<DAE::Type>>;
                            let mut cn2: ArcStr;
                            let mut classDef: metamodelica::Ref<SCode::ClassDef>;
                            let mut fq_class: metamodelica::Ref<Absyn::Path>;
                            let mut oDA: Option<SCode::Attributes>;
                            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut ty: metamodelica::Ref<DAE::Type>;
                            let mut typeVars: metamodelica::List<ArcStr>;
                            let mut cache = (*cache).clone();
                            let mut ih = (*ih).clone();
                            let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                            let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                            let true = (Mod::emptyModOrEquality(metamodelica::AsArg::as_arg(&mods)) && SCodeUtil::emptyModOrEquality(metamodelica::AsArg::as_arg(&r#mod))) else { return Err("pattern mismatch") };
                            let false = (listMember(AbsynUtil::pathString(cn.clone(), literal!("."), true, false)?, list![literal!("tuple"), literal!("Tuple"), literal!("array"), literal!("Array"), literal!("Option"), literal!("list"), literal!("List")])) else { return Err("pattern mismatch") };
                            let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), Some(info.clone()))?) {
                                (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, restriction: SCode::Restriction::R_UNIONTYPE { typeVars: __pa2 }, classDef: __pa3, .. }, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            cn2 = metamodelica::Own::own(__pa1);
                            typeVars = metamodelica::Own::own(__pa2);
                            classDef = metamodelica::Own::own(__pa3);
                            cenv = metamodelica::Own::own(__pa4);
                            (cache, fq_class) = makeFullyQualifiedIdent(cache.clone(), cenv.clone(), cn2.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                            new_ci_state = ClassInf::State::META_UNIONTYPE { path: fq_class.clone(), typeVars: typeVars.clone() };
                            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(InstMeta::fixUniontype(cache.clone(), env.clone(), &new_ci_state, &classDef)?) {
                                (__pa6, Some(__pa7 @ Deref @ DAE::Type::T_METAUNIONTYPE { .. })) => (__pa6.clone(), __pa7.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa6);
                            ty = metamodelica::Own::own(__pa7);
                            (cache, _, ih, tys, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&tSpecs), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), &inSets, info)?;
                            tys = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut t in (tys.clone()).into_iter().cloned() {
                            let __x = Types::boxIfUnboxedType(t.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            if !(((tys).len() as i32) == ((typeVars).len() as i32)) {
                                Error::addSourceMessage(&(Error::UNIONTYPE_WRONG_NUM_TYPEVARS.clone()), list![AbsynUtil::pathString(fq_class.clone(), literal!("."), true, false)?, ArcStr::from(::std::format!("{}", ((typeVars).len() as i32))), ArcStr::from(::std::format!("{}", ((tys).len() as i32)))], info)?;
                                return Err("fail");
                            }
                            ty = Types::setTypeVariables(ty.clone(), tys.clone());
                            oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                            bc = Some(ty.clone());
                            Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), new_ci_state.clone(), metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: tSpec @ Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: Some(_), .. }, .. }, _, _, _, _, _, _, _, _, _) => {
                    let mut cns: ArcStr;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    cns = Dump::unparseTypeSpec(tSpec.clone())?;
                    Error::addSourceMessage(&(Error::META_INVALID_COMPLEX_TYPE.clone()), list![cns.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: tSpec @ Deref @ Absyn::TypeSpec::TCOMPLEX { path: cn, typeSpecs: tSpecs, .. }, .. }, _, _, _, _, _, _, _, _, _) => {
                    let mut cns: ArcStr;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (listMember((AbsynUtil::pathString(cn.clone(), literal!("."), true, false)?, ((tSpecs).len() as i32) == 1), list![(literal!("tuple"), false), (literal!("array"), true), (literal!("Option"), true), (literal!("list"), true)])) else { return Err("pattern mismatch") };
                    cns = Dump::unparseTypeSpec(tSpec.clone())?;
                    Error::addSourceMessage(&(Error::META_INVALID_COMPLEX_TYPE.clone()), list![cns.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, .. }, .. }, _, _, _, _, _, _, _, _, _) => {
                    let mut cns: ArcStr;
                    let mut scope_str: ArcStr;
                    let false = (Mutable::access(stopInst.clone())) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), None), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    cns = AbsynUtil::pathString(cn.clone(), literal!("."), true, false)?;
                    scope_str = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    Error::addSourceMessage(&(Error::LOOKUP_ERROR.clone()), list![cns.clone(), scope_str.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, .. }, .. }, _, _, _, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), None), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Debug::trace(literal!("- Inst.instClassdef DERIVED( "))?;
                    Debug::trace(AbsynUtil::pathString(cn.clone(), literal!("."), true, false)?)?;
                    Debug::trace(literal!(") lookup failed\n ENV:"))?;
                    Debug::trace(FGraph::printGraphStr(metamodelica::AsArg::as_arg(&env)))?;
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
                    let mut s: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("- Inst.instClassdef failed"))?;
                    s = FGraph::printGraphPathStr(&inEnv);
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  class :")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outTypesVarLst,
        oty,
        optDerAttr,
        outEqualityConstraint,
        outGraph,
    ))
}

fn joinExtEquations(
    mut inEq: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inExtEq: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inCallingScope: InstTypes::CallingScope,
) -> metamodelica::List<metamodelica::Ref<SCode::Equation>> {
    let mut outEq: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    outEq = (match inCallingScope {
        InstTypes::CallingScope::TYPE_CALL { .. } => metamodelica::nil(),
        _ => listAppend(inEq, inExtEq),
    });
    outEq
}

fn joinExtAlgorithms(
    mut inAlg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut inExtAlg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut inCallingScope: InstTypes::CallingScope,
) -> metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> {
    let mut outAlg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    outAlg = (match inCallingScope {
        InstTypes::CallingScope::TYPE_CALL { .. } => metamodelica::nil(),
        _ => listAppend(inAlg, inExtAlg),
    });
    outAlg
}

fn instClassDefHelper(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inSpecs: &metamodelica::List<metamodelica::Ref<Absyn::TypeSpec>>,
    mut inPre: DAE::Prefix,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImpl: bool,
    mut accTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inSets: &DAE::Connect::Sets,
    mut inInfo: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
    DAE::Connect::Sets,
    Option<SCode::Attributes>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outType: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut outSets: DAE::Connect::Sets;
    let mut outAttr: Option<SCode::Attributes>;
    (outCache, outEnv, outIH, outType, outSets, outAttr) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, &**inSpecs, inPre, inInstDims, inImpl, accTypes);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Nil, _, _, _, localAccTypes) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone().reverse(), inSets.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: _ }, tail: restTypeSpecs }, pre, dims, r#impl, localAccTypes) => {
                    let mut cenv: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut localAccTypes = (*localAccTypes).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), Some(inInfo.clone()))?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    cenv = metamodelica::Own::own(__pa2);
                    let false = (SCodeUtil::isFunction(&c)) else { return Err("pattern mismatch") };
                    (cache, cenv, ih, _, _, csets, ty, _, oDA, _) = instClass(cache.clone(), cenv.clone(), ih.clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), c.clone(), dims.clone(), r#impl.clone(), openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), inSets)?;
                    localAccTypes = metamodelica::cons(ty.clone(), localAccTypes.clone());
                    (cache, env, ih, localAccTypes, csets, _) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&restTypeSpecs), pre.clone(), dims.clone(), r#impl.clone(), localAccTypes.clone(), &csets, inInfo)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone(), csets.clone(), oDA.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: _ }, tail: restTypeSpecs }, pre, dims, r#impl, localAccTypes) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut localAccTypes = (*localAccTypes).clone();
                    (cache, ty, _) = Lookup::lookupType(cache.clone(), env.clone(), cn.clone(), None)?;
                    localAccTypes = metamodelica::cons(ty.clone(), localAccTypes.clone());
                    (cache, env, ih, localAccTypes, csets, _) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&restTypeSpecs), pre.clone(), dims.clone(), r#impl.clone(), localAccTypes.clone(), inSets, inInfo)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone(), csets.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: tSpec @ Deref @ Absyn::TypeSpec::TCOMPLEX { path: p, typeSpecs: _, arrayDim: _ }, tail: restTypeSpecs }, pre, dims, r#impl, localAccTypes) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut id: ArcStr;
                    let mut oDA: Option<SCode::Attributes>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut localAccTypes = (*localAccTypes).clone();
                    id = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    c = metamodelica::Ref::new(SCode::Element::CLASS { name: id.clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: tSpec.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() });
                    (cache, _, ih, _, _, csets, ty, _, oDA, _) = instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), c.clone(), dims.clone(), r#impl.clone(), openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), inSets)?;
                    localAccTypes = metamodelica::cons(ty.clone(), localAccTypes.clone());
                    (cache, env, ih, localAccTypes, csets, _) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&restTypeSpecs), pre.clone(), dims.clone(), r#impl.clone(), localAccTypes.clone(), &csets, inInfo)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone(), csets.clone(), oDA.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outType, outSets, outAttr))
}

fn instBasictypeBaseclass(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inSCodeElementLst2: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inSCodeElementLst3: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inMod4: metamodelica::Ref<DAE::Mod>,
    mut inInstDims5: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut className: ArcStr,
    mut info: SourceInfo,
    mut stopInst: Mutable::Mutable<bool>,
) -> Result<(
    FCore::Cache,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    Option<metamodelica::Ref<DAE::Type>>,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outTypesTypeOption: Option<metamodelica::Ref<DAE::Type>>;
    let mut outTypeVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    (outCache, outIH, outStore, outDae, outTypesTypeOption, outTypeVars) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inStore,
            &**inSCodeElementLst2,
            &**inSCodeElementLst3,
            inMod4,
            inInstDims5,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: path, modifications: r#mod, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, mods, inst_dims) => {
                    let mut m_1: metamodelica::Ref<DAE::Mod>;
                    let mut m_2: metamodelica::Ref<DAE::Mod>;
                    let mut cdef: metamodelica::Ref<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint(literal!("instBasictypeBaseclass"));
                    (cache, m_1) = Mod::elabModForBasicType(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::DERIVED { path: path.clone() }, info.clone())?;
                    m_2 = Mod::merge(mods.clone(), m_1.clone(), className.clone(), true)?;
                    (cache, cdef, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path), Some(info.clone()))?;
                    (cache, _, ih, store, dae, _, ty, tys, _) = instClassBasictype(cache.clone(), cenv.clone(), ih.clone(), store.clone(), m_2.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), inst_dims.clone(), false, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, Connect::emptySet().clone())?;
                    b1 = Types::basicType(&ty);
                    b2 = Types::arrayType(&ty);
                    b3 = Types::extendsBasicType(&ty);
                    let true = (boolOr(b1, boolOr(b2, b3))) else { return Err("pattern mismatch") };
                    ErrorExt::rollBack(literal!("instBasictypeBaseclass"));
                    Ok((cache.clone(), ih.clone(), store.clone(), dae.clone(), Some(ty.clone()), tys.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: path, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    rollbackCheck(path.clone());
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { .. }, tail: Deref @ metamodelica::ListNode::Nil }, _, mods, inst_dims) => {
                    let false = ((inSCodeElementLst3).is_empty()) else { return Err("pattern mismatch") };
                    ErrorExt::setCheckpoint(literal!("instBasictypeBaseclass2"));
                    instBasictypeBaseclass2(cache.clone(), env.clone(), ih.clone(), store.clone(), inSCodeElementLst2, inSCodeElementLst3, metamodelica::AsArg::as_arg(&mods), inst_dims.clone(), &className, info.clone(), stopInst.clone());
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outIH, outStore, outDae, outTypesTypeOption, outTypeVars))
}

fn rollbackCheck(mut p: metamodelica::Ref<Absyn::Path>) -> () {
    let () = 'mc: {
        let __mc_input = &*p;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut n: ArcStr;
                    n = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    let true = (InstUtil::isBuiltInClass(&n)?) else { return Err("pattern mismatch") };
                    ErrorExt::rollBack(literal!("instBasictypeBaseclass"));
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack(literal!("instBasictypeBaseclass"));
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn instBasictypeBaseclass2(
    mut inCache: FCore::Cache,
    mut inEnv1: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut store: UnitAbsyn::InstStore,
    mut inSCodeElementLst2: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inSCodeElementLst3: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inMod4: &metamodelica::Ref<DAE::Mod>,
    mut inInstDims5: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut className: &ArcStr,
    mut inInfo: SourceInfo,
    mut stopInst: Mutable::Mutable<bool>,
) -> () {
    let () = 'mc: {
        let __mc_input = (
            inCache,
            inEnv1,
            inIH,
            &**inSCodeElementLst2,
            &**inSCodeElementLst3,
            inInstDims5,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: path, modifications: r#mod, info, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, inst_dims) => {
                    let mut m_1: metamodelica::Ref<DAE::Mod>;
                    let mut cdef: metamodelica::Ref<SCode::Element>;
                    let mut cdef_1: metamodelica::Ref<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut classname: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, m_1) = Mod::elabModForBasicType(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::DERIVED { path: path.clone() }, inInfo.clone())?;
                    (cache, cdef, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path), Some(info.clone()))?;
                    cdef_1 = SCodeUtil::classSetPartial(cdef.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    (cache, _, ih, _, _, _, ty, _, _, _) = instClass(cache.clone(), cenv.clone(), ih.clone(), store.clone(), m_1.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef_1.clone(), inst_dims.clone(), false, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    b1 = Types::basicType(&ty);
                    b2 = Types::arrayType(&ty);
                    let true = (boolOr(b1, b2)) else { return Err("pattern mismatch") };
                    classname = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    ErrorExt::rollBack(literal!("instBasictypeBaseclass2"));
                    Error::addSourceMessage(&(Error::INHERIT_BASIC_WITH_COMPS.clone()), list![classname.clone()], &inInfo)?;
                    Mutable::update(stopInst.clone(), true);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack(literal!("instBasictypeBaseclass2"));
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn partialInstClassdef(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inPrefix: &DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
    mut inVisibility: SCode::Visibility,
    mut inInstDims: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut numIter: i32,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outState: ClassInf::State;
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    (outCache, outEnv, outIH, outState, outVars) = ({
        let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
        (::match_deref::match_deref! { match inClassDef {
            Deref @ SCode::ClassDef::PARTS { elementLst: __inClassDef_elementLst, .. } => {
                let mut partial_prefix: SCode::Partial;
                let mut class_name: ArcStr;
                let mut cdef_els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut class_ext_els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut extends_els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut emods: metamodelica::Ref<DAE::Mod>;
                let mut r#mod: metamodelica::Ref<DAE::Mod>;
                let mut ext_comps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                let mut const_els: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                partial_prefix = SCodeUtil::getClassPartialPrefix(inClass)?;
                partial_prefix = InstUtil::isPartial(partial_prefix, inMod);
                class_name = SCodeUtil::elementName(inClass)?;
                outState = ClassInfUtil::trans(inState.clone(), openmodelica_frontend_types::ClassInf::Event::NEWDEF)?;
                (cdef_els, class_ext_els, extends_els, _) = InstUtil::splitElts(metamodelica::AsArg::as_arg(&__inClassDef_elementLst))?;
                extends_els = SCodeInstUtil::addRedeclareAsElementsToExtends(&extends_els, List::select(__inClassDef_elementLst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isRedeclareElement(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?)?;
                (outCache, outEnv, outIH) = InstUtil::addClassdefsToEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), &cdef_els, true, Some(inMod.clone()), FGraph::isEmptyScope(inEnv))?;
                (outCache, outEnv, outIH, emods, ext_comps, _, _, _, _, _) = InstExtends::instExtendsAndClassExtendsList(outCache, outEnv, outIH, inMod.clone(), inPrefix.clone(), extends_els, &class_ext_els, __inClassDef_elementLst.clone(), inState, class_name.clone(), true, true)?;
                const_els = listAppend(ext_comps.clone(), InstUtil::addNomod(InstUtil::constantEls(__inClassDef_elementLst.clone())));
                r#mod = Mod::merge(inMod.clone(), emods, class_name, true)?;
                (cdef_els, ext_comps) = InstUtil::classdefElts2(&ext_comps, partial_prefix)?;
                (outCache, outEnv, outIH) = InstUtil::addClassdefsToEnv(outCache, outEnv, outIH, inPrefix.clone(), &cdef_els, true, Some(r#mod.clone()), false)?;
                (outCache, outEnv, outIH) = InstUtil::addComponentsToEnv(outCache, outEnv, outIH, r#mod.clone(), inPrefix.clone(), inState.clone(), &const_els, false)?;
                (outCache, outEnv, outIH, _, _, _, outState, outVars, _, _) = instElementList(outCache, outEnv, outIH, UnitAbsyn::noStore().clone(), r#mod, inPrefix.clone(), outState, const_els, inInstDims.clone(), true, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), false)?;
                (outCache, outEnv, outIH, outState, outVars)
            },
            Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: class_path, arrayDim: class_dims }, modifications: class_mod, .. } => {
                let mut class_name: ArcStr;
                let mut scope_str: ArcStr;
                let mut r#mod: metamodelica::Ref<DAE::Mod>;
                let mut smod: metamodelica::Ref<SCode::Mod>;
                let mut cls: metamodelica::Ref<SCode::Element>;
                let mut cdef: metamodelica::Ref<SCode::ClassDef>;
                let mut cenv: FCore::Graph;
                let mut parent_env: FCore::Graph;
                let mut der_re: SCode::Restriction;
                let mut parent_re: SCode::Restriction;
                let mut enc: SCode::Encapsulated;
                let mut info: SourceInfo;
                let mut eq: Option<DAE::EqMod>;
                let mut has_dims: bool;
                let mut is_basic_type: bool;
                let mut inst_dims: InstDims;
                let mut scope_ty: Option<FCore::ScopeType>;
                info = SCodeUtil::elementInfo(inClass);
                has_dims = !((class_dims).is_none() || class_dims.clone() == Some(metamodelica::nil()));
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(inCache, inEnv, metamodelica::AsArg::as_arg(&class_path), Some(info.clone()))) {
                    Ok((__pa0, __pa1 @ Deref @ SCode::Element::CLASS { .. }, __pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => {
                    class_name = AbsynUtil::pathString(class_path.clone(), literal!("."), true, false)?;
                    scope_str = FGraph::printGraphPathStr(inEnv);
                    Error::addSourceMessageAndFail(&(Error::LOOKUP_ERROR.clone()), list![class_name.clone(), scope_str.clone()], &info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    },
                } };
                outCache = metamodelica::Own::own(__pa0);
                cls = metamodelica::Own::own(__pa1);
                cenv = metamodelica::Own::own(__pa2);
                let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(cls.clone()) {
                    Deref @ SCode::Element::CLASS { name: __pa4, encapsulatedPrefix: __pa5, restriction: __pa6, .. } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                class_name = metamodelica::Own::own(__pa4);
                enc = metamodelica::Own::own(__pa5);
                der_re = metamodelica::Own::own(__pa6);
                parent_re = SCodeUtil::getClassRestriction(inClass)?;
                is_basic_type = InstUtil::checkDerivedRestriction(parent_re, der_re.clone(), class_name.clone())?;
                smod = InstUtil::chainRedeclares(inMod, class_mod.clone());
                (parent_env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                (outCache, r#mod) = Mod::elabMod(outCache, parent_env.clone(), inIH.clone(), inPrefix.clone(), smod, false, Mod::ModScope::DERIVED { path: class_path.clone() }, info.clone())?;
                r#mod = Mod::merge(inMod.clone(), r#mod, class_name.clone(), true)?;
                if has_dims && !(is_basic_type) {
                    cls = SCodeUtil::setClassName(class_name.clone(), cls)?;
                    eq = Mod::modEquation(&r#mod);
                    (outCache, dims) = InstUtil::elabArraydimOpt(outCache, parent_env, metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!(""), subscripts: metamodelica::nil() }), class_path.clone(), class_dims.clone(), eq, false, true, inPrefix.clone(), info.clone(), inInstDims.clone())?;
                    inst_dims = List::appendLastList(inInstDims, dims)?;
                } else {
                    inst_dims = inInstDims.clone();
                }
                if is_basic_type || has_dims {
                    scope_ty = if (is_basic_type) {FGraph::restrictionToScopeType(&der_re)} else {FGraph::classInfToScopeType(inState)};
                    cenv = FGraph::openScope(cenv, enc, class_name, scope_ty)?;
                    outState = ClassInfUtil::start(&der_re, FGraph::getGraphName(&cenv)?)?;
                    (outCache, outEnv, outIH, outState, outVars) = partialInstClassIn(outCache, cenv, inIH.clone(), r#mod, inPrefix.clone(), outState, cls.clone(), inVisibility, inst_dims, numIter)?;
                } else {
                    cdef = metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: list![metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: class_path.clone(), visibility: inVisibility, modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), ann: None, info: info })], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None });
                    (outCache, outEnv, outIH, outState, outVars) = partialInstClassdef(&outCache, inEnv, inIH, &r#mod, inPrefix, inState, inClass, &cdef, inVisibility, inInstDims, numIter)?;
                }
                if SCodeUtil::isPartial(&cls) {
                    outEnv = FGraph::makeScopePartial(inEnv.clone());
                }
                (outCache, outEnv, outIH, outState, outVars)
            },
            _ => return Err("match: no arm matched"),
        } })
    });
    Ok((outCache, outEnv, outIH, outState, outVars))
}

pub(crate) fn instElementList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplInst: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut inStopOnError: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    ConnectionGraph::ConnectionGraph,
    metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    )>,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH;
    let mut outStore: UnitAbsyn::InstStore = inStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets = inSets;
    let mut outState: ClassInf::State = inState.clone();
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph;
    let mut domainFieldsListOut: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    )> = metamodelica::nil();
    let mut el: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut cache: FCore::Cache;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut dae: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut varsl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Var>>> = metamodelica::nil();
    let mut dael: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>> = metamodelica::nil();
    let mut fieldDomOpt: Option<(
        metamodelica::Ref<Absyn::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>;
    let mut element_order: metamodelica::List<i32>;
    let mut el_arr: metamodelica::Array<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut var_arr: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Var>>>;
    let mut dae_arr: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
    let mut length: i32;
    cache = InstUtil::pushStructuralParameters(inCache);
    el = InstUtil::sortElementList(inElements.clone(), &(inEnv.clone()), FGraph::inFunctionScope(&inEnv))?;
    el = InstUtil::sortInnerFirstTplLstElementMod(el);
    if !(ClassInfUtil::isFunction(&inState)) {
        element_order = getSortedElementOrdering(&inElements, el.clone())?;
        el_arr = metamodelica::arrayFromVec(inElements.into_iter().cloned().collect());
        length = ((el).len() as i32);
        var_arr = arrayCreate(length, metamodelica::nil());
        dae_arr = arrayCreate(length, metamodelica::nil());
        for mut idx in &*element_order {
            (
                cache,
                outEnv,
                outIH,
                outStore,
                dae,
                outSets,
                outState,
                vars,
                outGraph,
                fieldDomOpt,
            ) = instElement2(
                cache,
                outEnv,
                outIH,
                outStore,
                inMod.clone(),
                inPrefix.clone(),
                outState,
                ({
                    let __elt = (*metamodelica::index_checked(&el_arr.borrow(), idx.clone())?).clone();
                    __elt
                }),
                inInstDims.clone(),
                inImplInst,
                inCallingScope,
                outGraph,
                outSets,
                inStopOnError,
            )?;
            metamodelica::arrayUpdate(var_arr.clone(), length - idx.clone() + 1, vars)?;
            metamodelica::arrayUpdate(dae_arr.clone(), length - idx.clone() + 1, dae)?;
            if intEq(
                Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                Flags::PDEMODELICA.clone(),
            ) {
                domainFieldsListOut = InstUtil::optAppendField(domainFieldsListOut, fieldDomOpt)?;
            }
        }
        outVars = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
            for mut lst in (var_arr.clone()).borrow().iter() {
                let __x = lst.clone();
                __acc = __x.append(&__acc);
            }
            __acc
        });
        outDae = DAE::DAElist {
            elementLst: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
                for mut lst in (dae_arr.clone()).borrow().iter() {
                    let __x = lst.clone();
                    __acc = __x.append(&__acc);
                }
                __acc
            }),
        };
        GCExt::free(var_arr.clone());
        GCExt::free(dae_arr.clone());
    } else {
        for mut e in &*el {
            (
                cache,
                outEnv,
                outIH,
                outStore,
                dae,
                outSets,
                outState,
                vars,
                outGraph,
                fieldDomOpt,
            ) = instElement2(
                cache,
                outEnv,
                outIH,
                outStore,
                inMod.clone(),
                inPrefix.clone(),
                outState,
                e.clone(),
                inInstDims.clone(),
                inImplInst,
                inCallingScope,
                outGraph,
                outSets,
                inStopOnError,
            )?;
            varsl = metamodelica::cons(vars, varsl);
            dael = metamodelica::cons(dae, dael);
        }
        outVars = List::flattenReverse(varsl)?;
        outDae = DAE::DAElist {
            elementLst: List::flattenReverse(dael)?,
        };
    }
    outCache = InstUtil::popStructuralParameters(cache, inPrefix)?;
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outVars,
        outGraph,
        domainFieldsListOut,
    ))
}

fn getSortedElementOrdering(
    mut inElements: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inSortedElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> Result<metamodelica::List<i32>> {
    let mut outIndices: metamodelica::List<i32> = metamodelica::nil();
    let mut index_map: metamodelica::List<(metamodelica::Ref<SCode::Element>, i32)> = metamodelica::nil();
    let mut sorted_el: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut i: i32 = 1;
    for mut e in &**inElements {
        index_map = metamodelica::cons((Util::tuple21(e.clone()), i), index_map);
        i = i + 1;
    }
    index_map = index_map.reverse();
    sorted_el = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (inSortedElements).into_iter().cloned() {
            let __x = Util::tuple21(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut e in &*sorted_el {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(e.clone(), index_map, &move |__a0: metamodelica::Ref<SCode::Element>, __a1: (metamodelica::Ref<SCode::Element>, i32)| -> metamodelica::Result<_> { ::std::result::Result::Ok(getSortedElementOrdering_comp(&__a0, __a1)) })?) {
            (__pa0, Some((_, __pa1))) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        index_map = metamodelica::Own::own(__pa0);
        i = metamodelica::Own::own(__pa1);
        outIndices = metamodelica::cons(i, outIndices);
    }
    outIndices = outIndices.reverse();
    Ok(outIndices)
}

fn getSortedElementOrdering_comp(
    mut inElement1: &metamodelica::Ref<SCode::Element>,
    mut inElement2: (metamodelica::Ref<SCode::Element>, i32),
) -> bool {
    let mut outEqual: bool = SCodeUtil::elementNameEqual(inElement1, &(Util::tuple21(inElement2.clone())));
    outEqual
}

pub(crate) fn instElement2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inElement: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplicit: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
    mut inStopOnError: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    ConnectionGraph::ConnectionGraph,
    Option<(
        metamodelica::Ref<Absyn::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH.clone();
    let mut outStore: UnitAbsyn::InstStore = inStore;
    let mut outDae: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outState: ClassInf::State = inState;
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph;
    let mut outFieldDomOpt: Option<(
        metamodelica::Ref<Absyn::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )> = None;
    let mut elt: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);
    let mut is_deleted: bool;
    (is_deleted, outEnv, outCache) = isDeletedComponent(
        &inElement,
        inPrefix.clone(),
        inStopOnError,
        inEnv.clone(),
        inCache.clone(),
    )?;
    if is_deleted {
        return Ok((
            outCache,
            outEnv,
            outIH,
            outStore,
            outDae,
            outSets,
            outState,
            outVars,
            outGraph,
            outFieldDomOpt,
        ));
    }
    match '__try0: {
        ErrorExt::setCheckpoint(literal!("instElement2"));
        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(updateCompeltsMods(inCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), list![inElement.clone()], outState.clone(), inImplicit)) {
            (__pa1, __pa2, __pa3, Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa1);
        outEnv = metamodelica::Own::own(__pa2);
        outIH = metamodelica::Own::own(__pa3);
        elt = metamodelica::Own::own(__pa4);
        let (__pa6, __pa7, __pa8, __pa9, DAE::DAE { elementLst: __pa10 }, __pa11, __pa12, __pa13, __pa14, __pa15) = unwrap_break_err!(instElement(outCache.clone(), outEnv.clone(), outIH.clone(), outStore.clone(), inMod.clone(), inPrefix.clone(), outState.clone(), &elt, inInstDims.clone(), inImplicit, inCallingScope, outGraph.clone(), inSets.clone()), '__try0);
        outCache = metamodelica::Own::own(__pa6);
        outEnv = metamodelica::Own::own(__pa7);
        outIH = metamodelica::Own::own(__pa8);
        outStore = metamodelica::Own::own(__pa9);
        outDae = metamodelica::Own::own(__pa10);
        outSets = metamodelica::Own::own(__pa11);
        outState = metamodelica::Own::own(__pa12);
        outVars = metamodelica::Own::own(__pa13);
        outGraph = metamodelica::Own::own(__pa14);
        outFieldDomOpt = metamodelica::Own::own(__pa15);
        unwrap_break_err!(Error::clearCurrentComponent(), '__try0);
        ErrorExt::delCheckpoint(literal!("instElement2"));
        Ok::<_, &'static str>((
            elt.clone(),
            outCache.clone(),
            outDae.clone(),
            outEnv.clone(),
            outFieldDomOpt.clone(),
            outGraph.clone(),
            outIH.clone(),
            outSets.clone(),
            outState.clone(),
            outStore.clone(),
            outVars.clone(),
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
        )) => {
            elt = __try0_o0;
            outCache = __try0_o1;
            outDae = __try0_o2;
            outEnv = __try0_o3;
            outFieldDomOpt = __try0_o4;
            outGraph = __try0_o5;
            outIH = __try0_o6;
            outSets = __try0_o7;
            outState = __try0_o8;
            outStore = __try0_o9;
            outVars = __try0_o10;
        }
        Err(_) => {
            if inStopOnError {
                ErrorExt::delCheckpoint(literal!("instElement2"));
                return Err("fail");
            } else {
                ErrorExt::rollBack(literal!("instElement2"));
                outCache = inCache.clone();
                outEnv = inEnv.clone();
                outIH = inIH.clone();
                return Ok((
                    outCache,
                    outEnv,
                    outIH,
                    outStore,
                    outDae,
                    outSets,
                    outState,
                    outVars,
                    outGraph,
                    outFieldDomOpt,
                ));
            }
        }
    }
    Ok((
        outCache,
        outEnv,
        outIH,
        outStore,
        outDae,
        outSets,
        outState,
        outVars,
        outGraph,
        outFieldDomOpt,
    ))
}

fn isDeletedComponent(
    mut element: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut prefix: DAE::Prefix,
    mut stopOnError: bool,
    mut env: FCore::Graph,
    mut cache: FCore::Cache,
) -> Result<(bool, FCore::Graph, FCore::Cache)> {
    let mut isDeleted: bool;
    let mut env: FCore::Graph = env;
    let mut cache: FCore::Cache = cache;
    let mut el: metamodelica::Ref<SCode::Element>;
    let mut el_name: ArcStr;
    let mut info: SourceInfo;
    let mut cond_val_opt: Option<bool>;
    let mut cond_val: bool;
    let mut var: metamodelica::Ref<DAE::Var>;
    if InstUtil::componentHasCondition(element) {
        (el, _) = element.clone();
        (el_name, info) = InstUtil::extractCurrentName(&el)?;
        if SCodeUtil::isElementRedeclare(&el)? {
            Error::addSourceMessage(&(Error::REDECLARE_CONDITION.clone()), list![el_name.clone()], &info)?;
            return Err("fail");
        }
        (cond_val_opt, cache) = InstUtil::instElementCondExp(cache, env.clone(), &el, prefix, info);
        if (cond_val_opt).is_none() {
            if stopOnError {
                return Err("fail");
            } else {
                isDeleted = false;
                return Ok((isDeleted, env, cache));
            }
        }
        let __pa0 = ::match_deref::match_deref! { match &(cond_val_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cond_val = metamodelica::Own::own(__pa0);
        isDeleted = !(cond_val);
        if isDeleted == true {
            var = metamodelica::Ref::new(DAE::Var {
                name: el_name,
                attributes: DAE::dummyAttrVar().clone(),
                ty: DAE::T_UNKNOWN_DEFAULT().clone(),
                binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
                bind_from_outside: false,
                constOfForIteratorRange: None,
            });
            env = FGraph::updateComp(
                env,
                var,
                &(openmodelica_frontend_dump::FCore::Status::VAR_DELETED),
                &(FGraph::emptyGraph().clone()),
            );
        }
    } else {
        isDeleted = false;
    }
    Ok((isDeleted, env, cache))
}

pub(crate) fn instElement(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inUnitStore: UnitAbsyn::InstStore,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inElement: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inImplicit: bool,
    mut inCallingScope: InstTypes::CallingScope,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inSets: DAE::Connect::Sets,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    ConnectionGraph::ConnectionGraph,
    Option<(
        metamodelica::Ref<Absyn::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outUnitStore: UnitAbsyn::InstStore;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut outFieldDomOpt: Option<(
        metamodelica::Ref<Absyn::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )> = None;
    (
        outCache,
        outEnv,
        outIH,
        outUnitStore,
        outDae,
        outSets,
        outState,
        outVars,
        outGraph,
    ) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv.clone(),
            inIH.clone(),
            inUnitStore.clone(),
            inMod,
            inPrefix.clone(),
            inState.clone(),
            inElement,
            inInstDims,
            inImplicit,
            inGraph.clone(),
            inSets.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, (Deref @ SCode::Element::IMPORT { .. }, _), _, _, _, _) => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), inUnitStore.clone(), DAE::emptyDae().clone(), inSets.clone(), inState.clone(), metamodelica::nil(), inGraph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, (cls @ Deref @ SCode::Element::CLASS { .. }, cmod), _, _, _, _) => {
                    let mut env: FCore::Graph;
                    if !(Mod::isEmptyMod(metamodelica::AsArg::as_arg(&cmod))) {
                        env = FGraph::updateClass(inEnv.clone(), cls.clone(), &inPrefix, metamodelica::AsArg::as_arg(&cmod), &(openmodelica_frontend_dump::FCore::Status::CLS_UNTYPED), &(inEnv.clone()))?;
                    } else {
                        env = inEnv.clone();
                    }
                    Ok((inCache.clone(), env.clone(), inIH.clone(), inUnitStore.clone(), DAE::emptyDae().clone(), inSets.clone(), inState.clone(), metamodelica::nil(), inGraph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, (el @ Deref @ SCode::Element::COMPONENT { name, typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, cmod), inst_dims, r#impl, graph, csets) => {
                    let mut own_cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut dir: Absyn::Direction;
                    let mut info: SourceInfo;
                    let mut io: Absyn::InnerOuter;
                    let mut t: metamodelica::Ref<Absyn::Path>;
                    let mut ts: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut already_declared: bool;
                    let mut is_function_input: bool;
                    let mut graph_new: ConnectionGraph::ConnectionGraph;
                    let mut dae_attr: metamodelica::Ref<DAE::Attributes>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cref2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut dae: DAE::DAElist;
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut class_mod: metamodelica::Ref<DAE::Mod>;
                    let mut mm: metamodelica::Ref<DAE::Mod>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut var_class_mod: metamodelica::Ref<DAE::Mod>;
                    let mut m_1: metamodelica::Ref<DAE::Mod>;
                    let mut cls_mod: metamodelica::Ref<DAE::Mod>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut new_var: metamodelica::Ref<DAE::Var>;
                    let mut env2: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut comp_env: FCore::Graph;
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs3: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
                    let mut eq: Option<DAE::EqMod>;
                    let mut comment: metamodelica::Ref<SCode::Comment>;
                    let mut attr: SCode::Attributes;
                    let mut cls: metamodelica::Ref<SCode::Element>;
                    let mut comp: metamodelica::Ref<SCode::Element>;
                    let mut final_prefix: SCode::Final;
                    let mut m: metamodelica::Ref<SCode::Mod>;
                    let mut oldmod: metamodelica::Ref<SCode::Mod>;
                    let mut prefixes: metamodelica::Ref<SCode::Prefixes>;
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut isInSM: bool;
                    let mut elems: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut ci_state = (*ci_state).clone();
                    let mut name = (*name).clone();
                    let mut inst_dims = (*inst_dims).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let mut outFieldDomOpt: Option<(metamodelica::Ref<Absyn::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)> = outFieldDomOpt.clone();
                    let (__pa0, __pa3, __pa1, __pa2, __pa5, __pa4, __pa7, __pa6, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(el.clone()) {
                        Deref @ SCode::Element::COMPONENT { name: __pa0, prefixes: __pa3 @ Deref @ SCode::Prefixes { finalPrefix: __pa1, innerOuter: __pa2, .. }, attributes: __pa5 @ SCode::Attributes { arrayDims: __pa4, .. }, typeSpec: __pa7 @ Deref @ Absyn::TypeSpec::TPATH { path: __pa6, .. }, modifications: __pa8, comment: __pa9, condition: __pa10, info: __pa11 } => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa5.clone(), __pa4.clone(), __pa7.clone(), __pa6.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    final_prefix = metamodelica::Own::own(__pa1);
                    io = metamodelica::Own::own(__pa2);
                    prefixes = metamodelica::Own::own(__pa3);
                    ad = metamodelica::Own::own(__pa4);
                    attr = metamodelica::Own::own(__pa5);
                    t = metamodelica::Own::own(__pa6);
                    ts = metamodelica::Own::own(__pa7);
                    m = metamodelica::Own::own(__pa8);
                    comment = metamodelica::Own::own(__pa9);
                    cond = metamodelica::Own::own(__pa10);
                    info = metamodelica::Own::own(__pa11);
                    let true = (if (Config::acceptParModelicaGrammar()?) {InstUtil::checkParallelismWRTEnv(metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&name), &attr, &info)} else {true}) else { return Err("pattern mismatch") };
                    m = SCodeUtil::mergeModifiers(m.clone(), SCodeUtil::getConstrainedByModifiers(&prefixes));
                    if SCodeUtil::finalBool(final_prefix) {
                        m = InstUtil::traverseModAddFinal(m.clone())?;
                    }
                    comp = if (referenceEq(&*(var_field!((**el).modifications, SCode::Element::COMPONENT).clone()),&*(&*m))) {el.clone()} else {metamodelica::Ref::new(SCode::Element::COMPONENT { name: name.clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: ts.clone(), modifications: m.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() })};
                    oldmod = m.clone();
                    already_declared = InstUtil::checkMultiplyDeclared(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&mods), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&ci_state), (comp.clone(), cmod.clone()), metamodelica::AsArg::as_arg(&inst_dims), r#impl.clone())?;
                    m = InstUtil::chainRedeclares(metamodelica::AsArg::as_arg(&mods), m.clone());
                    m = SCodeInstUtil::expandEnumerationMod(m.clone())?;
                    m = InstUtil::traverseModAddDims(cache.clone(), env.clone(), pre.clone(), m.clone(), inst_dims.clone())?;
                    comp = if (referenceEq(&*(&*oldmod),&*(&*m))) {comp.clone()} else {metamodelica::Ref::new(SCode::Element::COMPONENT { name: name.clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: ts.clone(), modifications: m.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() })};
                    ci_state = ClassInfUtil::trans(ci_state.clone(), ClassInf::Event::FOUND_COMPONENT { name: name.clone() })?;
                    cref = ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), cref.clone())?;
                    class_mod = Mod::lookupModificationP(mods.clone(), &t)?;
                    mm = Mod::lookupCompModification(metamodelica::AsArg::as_arg(&mods), name.clone())?;
                    own_cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() });
                    crefs1 = InstUtil::getCrefFromMod(m.clone())?;
                    crefs2 = InstUtil::getCrefFromDim(&ad)?;
                    crefs3 = InstUtil::getCrefFromCond(cond.clone())?;
                    crefs = List::unionList(&(list![crefs1.clone(), crefs2.clone(), crefs3.clone()]))?;
                    (cache, env, ih, store, crefs) = removeSelfReferenceAndUpdate(cache.clone(), env.clone(), ih.clone(), store.clone(), crefs.clone(), own_cref.clone(), t.clone(), ci_state.clone(), attr.clone(), prefixes.clone(), r#impl.clone(), inst_dims.clone(), pre.clone(), metamodelica::AsArg::as_arg(&mods), m.clone(), info.clone())?;
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), mods.clone(), &crefs, ci_state.clone(), r#impl.clone());
                    (cache, class_mod) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), class_mod.clone(), r#impl.clone(), &info)?;
                    (cache, mm) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), mm.clone(), r#impl.clone(), &info)?;
                    (var_class_mod, class_mod) = modifyInstantiateClass(class_mod.clone(), t.clone())?;
                    (cache, m_1) = Mod::elabMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), m.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: name.clone() }, info.clone())?;
                    r#mod = Mod::merge(mm.clone(), class_mod.clone(), name.clone(), true)?;
                    r#mod = Mod::merge(r#mod.clone(), m_1.clone(), name.clone(), !(ClassInfUtil::isRecord(metamodelica::AsArg::as_arg(&ci_state))))?;
                    r#mod = Mod::merge(cmod.clone(), r#mod.clone(), name.clone(), true)?;
                    r#mod = Mod::merge(r#mod.clone(), var_class_mod.clone(), name.clone(), true)?;
                    let (__pa14, __pa15, __pa16, __pa17, __pa19, __pa18, __pa22, __pa20, __pa21, __pa23, __pa24, __pa25) = ::match_deref::match_deref! { match &(redeclareType(cache.clone(), env2.clone(), ih.clone(), r#mod.clone(), comp.clone(), pre.clone(), ci_state.clone(), r#impl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD())?) {
                        (__pa14, __pa15, __pa16, Deref @ SCode::Element::COMPONENT { name: __pa17, prefixes: __pa19 @ Deref @ SCode::Prefixes { innerOuter: __pa18, .. }, attributes: __pa22 @ SCode::Attributes { arrayDims: __pa20, direction: __pa21, .. }, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa23, arrayDim: _ }, modifications: _, comment: __pa24, condition: _, info: _ }, __pa25) => (__pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone(), __pa19.clone(), __pa18.clone(), __pa22.clone(), __pa20.clone(), __pa21.clone(), __pa23.clone(), __pa24.clone(), __pa25.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa14);
                    env2 = metamodelica::Own::own(__pa15);
                    ih = metamodelica::Own::own(__pa16);
                    name = metamodelica::Own::own(__pa17);
                    io = metamodelica::Own::own(__pa18);
                    prefixes = metamodelica::Own::own(__pa19);
                    ad = metamodelica::Own::own(__pa20);
                    dir = metamodelica::Own::own(__pa21);
                    attr = metamodelica::Own::own(__pa22);
                    t = metamodelica::Own::own(__pa23);
                    comment = metamodelica::Own::own(__pa24);
                    mod_1 = metamodelica::Own::own(__pa25);
                    (cache, cls, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), &env2, &t, Some(info.clone()))?;
                    cls_mod = Mod::getClassModifier(&cenv, SCodeUtil::className(&cls)?);
                    if !(Mod::isEmptyMod(&cls_mod)) {
                        if !((ad).is_empty()) {
                            cls_mod = Mod::addEachIfNeeded(cls_mod.clone(), &(list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })]))?;
                        }
                        mod_1 = Mod::merge(mod_1.clone(), cls_mod.clone(), name.clone(), true)?;
                    }
                    attr = SCodeUtil::mergeAttributesFromClass(attr.clone(), &cls)?;
                    inst_dims = List::appendElt(metamodelica::nil(), inst_dims.clone());
                    (cache, r#mod) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), &info)?;
                    (cache, mod_1) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), mod_1.clone(), r#impl.clone(), &info)?;
                    (r#mod, mod_1) = InstUtil::selectModifiers(r#mod.clone(), mod_1.clone(), &t);
                    eq = Mod::modEquation(&r#mod);
                    is_function_input = InstUtil::isFunctionInput(metamodelica::AsArg::as_arg(&ci_state), dir);
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), env2.clone(), own_cref.clone(), t.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, is_function_input, pre.clone(), info.clone(), inst_dims.clone())?;
                    if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                        (dims, mod_1, outFieldDomOpt) = InstUtil::elabField(&inCache, &inEnv, name.clone(), &attr, dims.clone(), mod_1.clone(), &info)?;
                    }
                    (cenv, cls, ih) = FGraph::createVersionScope(&env2, name.clone(), metamodelica::AsArg::as_arg(&pre), mod_1.clone(), cenv.clone(), cls.clone(), ih.clone())?;
                    (cache, cref2) = PrefixUtil::prefixCref(cache.clone(), cenv.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), cref.clone())?;
                    if !((ih).is_empty()) {
                        topInstance = (ih).head().cloned()?;
                        let InnerOuter::TOP_INSTANCE { sm: __pa27, .. } = &topInstance;
                        sm = metamodelica::Own::own(__pa27);
                        if BaseHashSet::has(cref2.clone(), &sm)? {
                            isInSM = true;
                        } else {
                            isInSM = false;
                        }
                    } else {
                        isInSM = false;
                    }
                    (cache, comp_env, ih, store, dae, csets, ty, graph_new) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), ci_state.clone(), mod_1.clone(), pre.clone(), name.clone(), cls.clone(), attr.clone(), prefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), r#impl.clone(), comment.clone(), &info, graph.clone(), csets.clone(), &env2)?;
                    if isInSM {
                        let DAE::DAE { elementLst: __pa28 } = &dae;
                        elems = metamodelica::Own::own(__pa28);
                        dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::SM_COMP { componentRef: cref2.clone(), dAElist: elems.clone() })] };
                    }
                    (cache, binding) = InstBinding::makeBinding(cache.clone(), &env2, &attr, &r#mod, ty.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&name), &info)?;
                    dae_attr = DAEUtil::translateSCodeAttrToDAEAttr(attr.clone(), &prefixes);
                    (ty, _) = Types::traverseType(ty.clone(), 1, &fnptr!(Types::setIsFunctionPointer, metamodelica::Ref<DAE::Type>, i32))?;
                    binding = removePrefixFromBinding(binding.clone(), pre.clone())?;
                    new_var = metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: dae_attr.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env2.clone(), new_var.clone(), &(openmodelica_frontend_dump::FCore::Status::VAR_DAE), &comp_env);
                    vars = if (already_declared) {metamodelica::nil()} else {list![new_var.clone()]};
                    dae = if (already_declared) {DAE::emptyDae().clone()} else {dae.clone()};
                    (_, ih, graph) = InnerOuter::handleInnerOuterEquations(io, DAE::emptyDae().clone(), ih.clone(), graph_new.clone(), graph.clone())?;
                    Ok(((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), vars.clone(), graph.clone()), outFieldDomOpt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outFieldDomOpt = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, (comp @ Deref @ SCode::Element::COMPONENT { name, prefixes: prefixes @ Deref @ SCode::Prefixes { finalPrefix: final_prefix, innerOuter: io, .. }, attributes: attr @ SCode::Attributes { arrayDims: ad, connectorType: ct, .. }, typeSpec: ts @ Deref @ Absyn::TypeSpec::TCOMPLEX { path: type_name, .. }, modifications: m, comment, condition: cond, info }, cmod), inst_dims, r#impl, graph, csets) => {
                    let mut own_cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut already_declared: bool;
                    let mut graph_new: ConnectionGraph::ConnectionGraph;
                    let mut dae_attr: metamodelica::Ref<DAE::Attributes>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut dae: DAE::DAElist;
                    let mut m_1: metamodelica::Ref<DAE::Mod>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut new_var: metamodelica::Ref<DAE::Var>;
                    let mut comp_env: FCore::Graph;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut cls: metamodelica::Ref<SCode::Element>;
                    let mut id: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut comp = (*comp).clone();
                    let mut m = (*m).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    if SCodeUtil::finalBool(final_prefix.clone()) {
                        m = InstUtil::traverseModAddFinal(m.clone())?;
                        comp = metamodelica::Ref::new(SCode::Element::COMPONENT { name: name.clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: ts.clone(), modifications: m.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() });
                    }
                    already_declared = InstUtil::checkMultiplyDeclared(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&mods), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&ci_state), (comp.clone(), cmod.clone()), metamodelica::AsArg::as_arg(&inst_dims), r#impl.clone())?;
                    cref = ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), cref.clone())?;
                    own_cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() });
                    (cache, m_1) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), m.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: name.clone() }, info.clone())?;
                    id = AbsynUtil::pathString(type_name.clone(), literal!("."), true, false)?;
                    cls = metamodelica::Ref::new(SCode::Element::CLASS { name: id.clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: ts.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), attributes: SCode::Attributes { arrayDims: ad.clone(), connectorType: ct.clone(), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } }), cmt: SCode::noComment.clone(), info: info.clone() });
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), env.clone(), own_cref.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Integer") }), ad.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    (cache, comp_env, ih, store, dae, csets, ty, graph_new) = InstVar::instVar(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), m_1.clone(), pre.clone(), name.clone(), cls.clone(), attr.clone(), prefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), r#impl.clone(), comment.clone(), metamodelica::AsArg::as_arg(&info), graph.clone(), csets.clone(), metamodelica::AsArg::as_arg(&env))?;
                    (cache, binding) = InstBinding::makeBinding(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&attr), &m_1, ty.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&name), metamodelica::AsArg::as_arg(&info))?;
                    dae_attr = DAEUtil::translateSCodeAttrToDAEAttr(attr.clone(), metamodelica::AsArg::as_arg(&prefixes));
                    (ty, _) = Types::traverseType(ty.clone(), 1, &fnptr!(Types::setIsFunctionPointer, metamodelica::Ref<DAE::Type>, i32))?;
                    new_var = metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: dae_attr.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), &(openmodelica_frontend_dump::FCore::Status::VAR_DAE), &comp_env);
                    vars = if (already_declared) {metamodelica::nil()} else {list![new_var.clone()]};
                    dae = if (already_declared) {DAE::emptyDae().clone()} else {dae.clone()};
                    (_, ih, graph) = InnerOuter::handleInnerOuterEquations(io.clone(), DAE::emptyDae().clone(), ih.clone(), graph_new.clone(), graph.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), vars.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, pre, ci_state, (Deref @ SCode::Element::COMPONENT { name, attributes: SCode::Attributes { variability: vt, .. }, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: t, arrayDim: _ }, info, .. }, _), _, _, _, _) => {
                    let mut ns: ArcStr;
                    let mut s: ArcStr;
                    let mut scope_str: ArcStr;
                    let mut pre = (*pre).clone();
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&t), None), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s = AbsynUtil::pathString(t.clone(), literal!("."), true, false)?;
                    scope_str = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    pre = PrefixUtil::prefixAdd(name.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::AsArg::as_arg(&pre), vt.clone(), ci_state.clone(), info.clone())?;
                    ns = PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?;
                    Error::addSourceMessage(&(Error::LOOKUP_ERROR_COMPNAME.clone()), list![s.clone(), scope_str.clone(), ns.clone()], metamodelica::AsArg::as_arg(&info))?;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Lookup class failed:")); __mm_s.push_str(&*AbsynUtil::pathString(t.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, _, _, (comp, _), _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instElement failed: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(comp.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outEnv,
        outIH,
        outUnitStore,
        outDae,
        outSets,
        outState,
        outVars,
        outGraph,
        outFieldDomOpt,
    ))
}

fn removePrefixFromBinding(
    mut inBind: metamodelica::Ref<DAE::Binding>,
    mut inPrefix: DAE::Prefix,
) -> Result<metamodelica::Ref<DAE::Binding>> {
    let mut outBind: metamodelica::Ref<DAE::Binding>;
    outBind = (::match_deref::match_deref! { match &((inBind.clone(), inPrefix)) {
        (bind @ Deref @ DAE::Binding::EQBOUND { .. }, pref @ DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { .. }, .. }) => {
            let mut bind = (*bind).clone();
            assign_variant_field!(bind => DAE::Binding::EQBOUND; exp = PrefixUtil::removeCompPrefixFromExps(var_field!((*bind).exp, DAE::Binding::EQBOUND).clone(), var_field!(pref.compPre, DAE::Prefix::PREFIX))?);
            bind.clone()
        },
        _ => {
            inBind
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBind)
}

fn updateCompeltsMods(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inComponents: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inState: ClassInf::State,
    mut inImplicit: bool,
) -> (
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = metamodelica::nil();
    let mut outComponents: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> =
        metamodelica::nil();
    (outCache, outEnv, outIH, outComponents) = 'mc: {
        let __mc_input = inImplicit;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut outCache: FCore::Cache = outCache.clone();
            let mut outComponents: metamodelica::List<(
                metamodelica::Ref<SCode::Element>,
                metamodelica::Ref<DAE::Mod>,
            )> = outComponents.clone();
            let mut outEnv: FCore::Graph = outEnv.clone();
            let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
            ErrorExt::setCheckpoint(literal!("updateCompeltsMods"));
            (outCache, outEnv, outIH, outComponents) = updateCompeltsMods_dispatch(
                inCache.clone(),
                inEnv.clone(),
                inIH.clone(),
                inPrefix.clone(),
                &inComponents,
                inState.clone(),
                inImplicit,
            )?;
            ErrorExt::rollBack(literal!("updateCompeltsMods"));
            Ok((
                (outCache.clone(), outEnv.clone(), outIH.clone(), outComponents.clone()),
                outCache.clone(),
                outComponents.clone(),
                outEnv.clone(),
                outIH.clone(),
            ))
        })() {
            outCache = __wb0;
            outComponents = __wb1;
            outEnv = __wb2;
            outIH = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ErrorExt::rollBack(literal!("updateCompeltsMods"));
            Ok((inCache.clone(), inEnv.clone(), inIH.clone(), inComponents.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outEnv, outIH, outComponents)
}

fn updateCompeltsMods_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inComponents: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inState: ClassInf::State,
    mut inImplicit: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outComponents: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    (outCache, outEnv, outIH, outComponents) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inPrefix, &**inComponents, inState, inImplicit);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::ListNode::Cons { head: elMod @ (_, Deref @ DAE::Mod::NOMOD { .. }), tail: xs }, ci_state, r#impl) => {
                    let mut res: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    (cache, env, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env.clone(), ih.clone(), pre.clone(), metamodelica::AsArg::as_arg(&xs), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), metamodelica::cons(elMod.clone(), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::ListNode::Cons { head: (comp, cmod @ Deref @ DAE::Mod::REDECL { element: redComp, .. }), tail: xs }, ci_state, r#impl) => {
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut umod: metamodelica::Ref<SCode::Mod>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut cmod_1: metamodelica::Ref<DAE::Mod>;
                    let mut cmod2: metamodelica::Ref<DAE::Mod>;
                    let mut ltmod: metamodelica::List<metamodelica::Ref<DAE::Mod>>;
                    let mut res: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut name: ArcStr;
                    let mut info: SourceInfo;
                    let mut fprefix: SCode::Final;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    info = SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&redComp));
                    umod = Mod::unelabMod(cmod.clone())?;
                    crefs = InstUtil::getCrefFromMod(umod.clone())?;
                    crefs_1 = InstUtil::getCrefFromCompDim(metamodelica::AsArg::as_arg(&comp));
                    crefs = List::unionOnTrue(&crefs, &crefs_1, &move |__a0: metamodelica::Ref<Absyn::ComponentRef>, __a1: metamodelica::Ref<Absyn::ComponentRef>| AbsynUtil::crefEqual(&__a0, &__a1))?;
                    name = SCodeUtil::elementName(metamodelica::AsArg::as_arg(&comp))?;
                    cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() });
                    ltmod = List::map1(crefs.clone(), &move |__a0: metamodelica::Ref<Absyn::ComponentRef>, __a1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>| InstUtil::getModsForDep(__a0, &__a1), xs.clone())?;
                    cmod2 = List::fold2r(&(metamodelica::cons(cmod.clone(), ltmod.clone())), &Mod::merge, name.clone(), true, openmodelica_frontend_types::DAE::Mod::interned_NOMOD())?;
                    let __arc1 = SCodeUtil::elementPrefixes(metamodelica::AsArg::as_arg(&comp))?;
                    let SCode::PREFIXES { finalPrefix: __pa0, .. } = &*__arc1;
                    fprefix = metamodelica::Own::own(__pa0);
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), cmod2.clone(), &crefs, ci_state.clone(), r#impl.clone());
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env2.clone(), ih.clone(), pre.clone(), metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: fprefix, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![metamodelica::Ref::new(DAE::SubMod { ident: name.clone(), r#mod: cmod.clone() })], binding: None, info: info.clone() }), &(list![cref.clone()]), ci_state.clone(), r#impl.clone());
                    (cache, cmod_1) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), cmod.clone(), r#impl.clone(), &info)?;
                    (cache, env3, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env2.clone(), ih.clone(), pre.clone(), metamodelica::AsArg::as_arg(&xs), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), metamodelica::cons((comp.clone(), cmod_1.clone()), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::ListNode::Cons { head: (comp, cmod @ Deref @ DAE::Mod::MOD { .. }), tail: xs }, ci_state, r#impl) => {
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut res: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut name: ArcStr;
                    let mut fprefix: SCode::Final;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let false = (Mod::isUntypedMod(metamodelica::AsArg::as_arg(&cmod))?) else { return Err("pattern mismatch") };
                    name = SCodeUtil::elementName(metamodelica::AsArg::as_arg(&comp))?;
                    cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() });
                    let __arc1 = SCodeUtil::elementPrefixes(metamodelica::AsArg::as_arg(&comp))?;
                    let SCode::PREFIXES { finalPrefix: __pa0, .. } = &*__arc1;
                    fprefix = metamodelica::Own::own(__pa0);
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: fprefix, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![metamodelica::Ref::new(DAE::SubMod { ident: name.clone(), r#mod: cmod.clone() })], binding: None, info: var_field!((**cmod).info, DAE::Mod::MOD).clone() }), &(list![cref.clone()]), ci_state.clone(), r#impl.clone());
                    (cache, env3, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env2.clone(), ih.clone(), pre.clone(), metamodelica::AsArg::as_arg(&xs), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), metamodelica::cons((comp.clone(), cmod.clone()), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::ListNode::Cons { head: (comp, cmod @ Deref @ DAE::Mod::MOD { .. }), tail: xs }, ci_state, r#impl) => {
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut umod: metamodelica::Ref<SCode::Mod>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut cmod_1: metamodelica::Ref<DAE::Mod>;
                    let mut cmod2: metamodelica::Ref<DAE::Mod>;
                    let mut ltmod: metamodelica::List<metamodelica::Ref<DAE::Mod>>;
                    let mut res: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut name: ArcStr;
                    let mut info: SourceInfo;
                    let mut fprefix: SCode::Final;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    info = SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&comp));
                    umod = Mod::unelabMod(cmod.clone())?;
                    crefs = InstUtil::getCrefFromMod(umod.clone())?;
                    crefs_1 = InstUtil::getCrefFromCompDim(metamodelica::AsArg::as_arg(&comp));
                    crefs = List::unionOnTrue(&crefs, &crefs_1, &move |__a0: metamodelica::Ref<Absyn::ComponentRef>, __a1: metamodelica::Ref<Absyn::ComponentRef>| AbsynUtil::crefEqual(&__a0, &__a1))?;
                    name = SCodeUtil::elementName(metamodelica::AsArg::as_arg(&comp))?;
                    cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() });
                    ltmod = List::map1(crefs.clone(), &move |__a0: metamodelica::Ref<Absyn::ComponentRef>, __a1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>| InstUtil::getModsForDep(__a0, &__a1), xs.clone())?;
                    cmod2 = List::fold2r(&ltmod, &Mod::merge, name.clone(), true, openmodelica_frontend_types::DAE::Mod::interned_NOMOD())?;
                    let __arc1 = SCodeUtil::elementPrefixes(metamodelica::AsArg::as_arg(&comp))?;
                    let SCode::PREFIXES { finalPrefix: __pa0, .. } = &*__arc1;
                    fprefix = metamodelica::Own::own(__pa0);
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), cmod2.clone(), &crefs, ci_state.clone(), r#impl.clone());
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env2.clone(), ih.clone(), pre.clone(), metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: fprefix, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![metamodelica::Ref::new(DAE::SubMod { ident: name.clone(), r#mod: cmod.clone() })], binding: None, info: var_field!((**cmod).info, DAE::Mod::MOD).clone() }), &(list![cref.clone()]), ci_state.clone(), r#impl.clone());
                    (cache, cmod_1) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), cmod.clone(), r#impl.clone(), &info)?;
                    (cache, env3, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env2.clone(), ih.clone(), pre.clone(), metamodelica::AsArg::as_arg(&xs), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), metamodelica::cons((comp.clone(), cmod_1.clone()), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outComponents))
}

pub(crate) fn redeclareType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inImpl: bool,
    mut inCmod: metamodelica::Ref<DAE::Mod>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::Ref<SCode::Element>,
    metamodelica::Ref<DAE::Mod>,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH.clone();
    let mut outElement: metamodelica::Ref<SCode::Element> = inElement.clone();
    let mut outMod: metamodelica::Ref<DAE::Mod> = openmodelica_frontend_types::DAE::Mod::interned_NOMOD();
    let mut redecl_el: metamodelica::Ref<SCode::Element>;
    let mut r#mod: metamodelica::Ref<SCode::Mod> = metamodelica::Ref::new(SCode::Mod::NOMOD);
    let mut redecl_mod: metamodelica::Ref<DAE::Mod>;
    let mut m: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
    let mut old_m: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
    let mut redecl_name: ArcStr;
    let mut name: ArcStr = arcstr::literal!("");
    let mut repl: metamodelica::Ref<SCode::Replaceable>;
    let mut cc: Option<metamodelica::Ref<SCode::ConstrainClass>> = None;
    let mut cc_comps: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> = metamodelica::nil();
    if !(Mod::isRedeclareMod(&inMod)) {
        outMod = Mod::merge(inMod, inCmod, literal!(""), true)?;
        return Ok((outCache, outEnv, outIH, outElement, outMod));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::REDECL { element: __pa0, r#mod: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    redecl_el = metamodelica::Own::own(__pa0);
    redecl_mod = metamodelica::Own::own(__pa1);
    redecl_name = SCodeUtil::elementName(&redecl_el)?;
    (outElement, outMod) = 'mc: {
        let __mc_input = (&*redecl_el, &*inElement);
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: repl, .. }, .. }) => {
                            let mut cc_comps: metamodelica::List<metamodelica::Ref<SCode::Element>> = cc_comps.clone();
                            let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> = crefs.clone();
                            let mut m: metamodelica::Ref<DAE::Mod> = m.clone();
                            let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod.clone();
                            let mut old_m: metamodelica::Ref<DAE::Mod> = old_m.clone();
                            let mut outCache: FCore::Cache = outCache.clone();
                            let mut outElement: metamodelica::Ref<SCode::Element> = outElement.clone();
                            let mut outEnv: FCore::Graph = outEnv.clone();
                            let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                            let mut redecl_mod: metamodelica::Ref<DAE::Mod> = redecl_mod.clone();
                            let true = (metamodelica::stringEq(&redecl_name, &var_field!((*inElement).name, SCode::Element::COMPONENT))) else { return Err("pattern mismatch") };
                            r#mod = InstUtil::chainRedeclares(&inMod, var_field!((*redecl_el).modifications, SCode::Element::COMPONENT).clone());
                            crefs = InstUtil::getCrefFromMod(r#mod.clone())?;
                            (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), &crefs, inState.clone(), inImpl);
                            (outCache, m) = Mod::elabMod(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), r#mod.clone(), inImpl, Mod::ModScope::COMPONENT { name: redecl_name.clone() }, var_field!((*redecl_el).info, SCode::Element::COMPONENT).clone())?;
                            (outCache, old_m) = Mod::elabMod(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), var_field!((*inElement).modifications, SCode::Element::COMPONENT).clone(), inImpl, Mod::ModScope::COMPONENT { name: var_field!((*inElement).name, SCode::Element::COMPONENT).clone() }, var_field!((*inElement).info, SCode::Element::COMPONENT).clone())?;
                            m = (::match_deref::match_deref! { match &(repl.clone()) {
                Deref @ SCode::Replaceable::REPLACEABLE { cc: __esc_cc @ Some(_) } => {
                            cc = (*__esc_cc).clone();
                            cc_comps = InstUtil::extractConstrainingComps(cc.clone(), &inEnv, &inPrefix)?;
                            redecl_mod = InstUtil::keepConstrainingTypeModifersOnly(redecl_mod.clone(), cc_comps.clone())?;
                            old_m = InstUtil::keepConstrainingTypeModifersOnly(old_m.clone(), cc_comps.clone())?;
                            m = Mod::merge(m.clone(), redecl_mod.clone(), redecl_name.clone(), true)?;
                            m = Mod::merge(m.clone(), old_m.clone(), redecl_name.clone(), true)?;
                            m = Mod::merge(m.clone(), inCmod.clone(), redecl_name.clone(), true)?;
                            m.clone()
                },
                _ => {
                            m = Mod::merge(redecl_mod.clone(), m.clone(), redecl_name.clone(), true)?;
                            m = Mod::merge(m.clone(), old_m.clone(), redecl_name.clone(), true)?;
                            m = Mod::merge(inCmod.clone(), m.clone(), redecl_name.clone(), true)?;
                            m.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            (outCache, outElement) = propagateRedeclCompAttr(outCache.clone(), &outEnv, &inElement, redecl_el.clone())?;
                            outElement = SCodeUtil::setComponentMod(outElement.clone(), r#mod.clone())?;
                            Ok(((outElement.clone(), m.clone()), cc_comps.clone(), crefs.clone(), m.clone(), r#mod.clone(), old_m.clone(), outCache.clone(), outElement.clone(), outEnv.clone(), outIH.clone(), redecl_mod.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            cc_comps = __wb0;
            crefs = __wb1;
            m = __wb2;
            r#mod = __wb3;
            old_m = __wb4;
            outCache = __wb5;
            outElement = __wb6;
            outEnv = __wb7;
            outIH = __wb8;
            redecl_mod = __wb9;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    let true = (metamodelica::stringEq(&redecl_name, &var_field!((*inElement).name, SCode::Element::CLASS))) else { return Err("pattern mismatch") };
                    (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), &(list![metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: var_field!((*inElement).name, SCode::Element::CLASS).clone(), subscripts: metamodelica::nil() })]), inState.clone(), inImpl);
                    Ok(((inElement.clone(), redecl_mod.clone()), outCache.clone(), outEnv.clone(), outIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outEnv = __wb1;
            outIH = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
                    let mut name: ArcStr = name.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    name = AbsynUtil::typeSpecPathString(var_field!((*inElement).typeSpec, SCode::Element::COMPONENT))?;
                    let true = (metamodelica::stringEq(&redecl_name, &name)) else { return Err("pattern mismatch") };
                    (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), &(list![metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() })]), inState.clone(), inImpl);
                    Ok(((inElement.clone(), redecl_mod.clone()), name.clone(), outCache.clone(), outEnv.clone(), outIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            outCache = __wb1;
            outEnv = __wb2;
            outIH = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
                    let mut name: ArcStr = name.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    name = AbsynUtil::pathFirstIdent(&(AbsynUtil::typeSpecPath(var_field!((*inElement).typeSpec, SCode::Element::COMPONENT))));
                    let true = (metamodelica::stringEq(&redecl_name, &name)) else { return Err("pattern mismatch") };
                    (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), &(list![metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() })]), inState.clone(), inImpl);
                    Ok(((inElement.clone(), redecl_mod.clone()), name.clone(), outCache.clone(), outEnv.clone(), outIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            outCache = __wb1;
            outEnv = __wb2;
            outIH = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inElement.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outElement, outMod))
}

fn propagateRedeclCompAttr(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inOldComponent: &metamodelica::Ref<SCode::Element>,
    mut inNewComponent: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, metamodelica::Ref<SCode::Element>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outComponent: metamodelica::Ref<SCode::Element>;
    let mut is_array: bool = false;
    if SCodeUtil::isArrayComponent(inOldComponent) && !(SCodeUtil::isArrayComponent(&inNewComponent)) {
        (outCache, is_array) = Lookup::isArrayType(outCache, inEnv, &(SCodeUtil::getElementTypePath(&inNewComponent)?));
    }
    outComponent = SCodeUtil::propagateAttributesVar(inOldComponent, inNewComponent, is_array)?;
    Ok((outCache, outComponent))
}

fn updateComponentsInEnv(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut pre: DAE::Prefix,
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut crefs: &metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut ci_state: ClassInf::State,
    mut r#impl: bool,
) -> (FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>) {
    let mut outCache: FCore::Cache = cache.clone();
    let mut outEnv: FCore::Graph = env.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH.clone();
    ErrorExt::setCheckpoint(literal!("updateComponentsInEnv__"));
    if '__try0: {
        (outCache, outEnv, outIH, _) = updateComponentsInEnv2(
            cache.clone(),
            env.clone(),
            inIH.clone(),
            pre.clone(),
            r#mod.clone(),
            crefs,
            ci_state.clone(),
            r#impl,
            None,
            None,
        );
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    ErrorExt::rollBack(literal!("updateComponentsInEnv__"));
    (outCache, outEnv, outIH)
}

fn getUpdatedCompsHashTable(
    mut optHT: Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    )>,
) -> (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Absyn::ComponentRef>,
                    metamodelica::Ref<Absyn::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
) {
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable5::FuncHashCref,
            HashTable5::FuncCrefEqual,
            HashTable5::FuncCrefStr,
            HashTable5::FuncExpStr,
        ),
    );
    ht = (match optHT {
        Some(mut __esc_ht) => {
            ht = __esc_ht.clone();
            ht
        }
        _ => HashTable5::emptyHashTableSized(BaseHashTable::lowBucketSize.clone()),
    });
    ht
}

fn updateComponentInEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut pre: DAE::Prefix,
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inCIState: ClassInf::State,
    mut r#impl: bool,
    mut inUpdatedComps: Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    )>,
    mut currentCref: Option<metamodelica::Ref<Absyn::ComponentRef>>,
) -> (
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    )>,
) {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outUpdatedComps: Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable5::FuncHashCref,
            HashTable5::FuncCrefEqual,
            HashTable5::FuncCrefStr,
            HashTable5::FuncExpStr,
        ),
    )>;
    (outCache, outEnv, outIH, outUpdatedComps) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), r#mod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { name, prefixes: prefixes @ Deref @ SCode::Prefixes { visibility, .. }, attributes, modifications: smod, info, .. }, .. }) => {
                    let mut n: ArcStr;
                    let mut id: ArcStr;
                    let mut ct: SCode::ConnectorType;
                    let mut io: Absyn::InnerOuter;
                    let mut attr: SCode::Attributes;
                    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut prl1: SCode::Parallelism;
                    let mut var1: SCode::Variability;
                    let mut dir: Absyn::Direction;
                    let mut t: metamodelica::Ref<Absyn::Path>;
                    let mut m: metamodelica::Ref<SCode::Mod>;
                    let mut cmod: metamodelica::Ref<DAE::Mod>;
                    let mut mods: metamodelica::Ref<DAE::Mod>;
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs3: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs_2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
                    let mut pf: metamodelica::Ref<SCode::Prefixes>;
                    let mut daeMod: metamodelica::Ref<DAE::Mod>;
                    let mut idENV: FCore::Graph;
                    let mut updatedComps: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
                    let mut ci_state: ClassInf::State;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut info = (*info).clone();
                    id = AbsynUtil::crefFirstIdent(&cref)?;
                    let true = (stringEq(&id, &name)) else { return Err("pattern mismatch") };
                    let false = (smod.clone() == openmodelica_frontend_types::SCode::Mod::interned_NOMOD()) else { return Err("pattern mismatch") };
                    let (__pa0, __t1, _, _, _, _) = Lookup::lookupIdentLocal(cache.clone(), metamodelica::AsArg::as_arg(&env), name.clone())?;
                    let __arc2 = __t1.clone();
                    let DAE::TYPES_VAR { name: _, attributes: _, ty: _, binding: _, bind_from_outside: _, .. } = &*__arc2;
                    cache = metamodelica::Own::own(__pa0);
                    (cache, daeMod) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl, Mod::ModScope::COMPONENT { name: name.clone() }, info.clone())?;
                    mods = daeMod.clone();
                    attr = attributes.clone();
                    m = smod.clone();
                    cmod = openmodelica_frontend_types::DAE::Mod::interned_NOMOD();
                    pf = prefixes.clone();
                    io = SCodeUtil::prefixesInnerOuter(&pf);
                    let SCode::ATTR { arrayDims: __pa3, connectorType: __pa4, parallelism: __pa5, variability: __pa6, direction: __pa7, .. } = &attr;
                    ad = metamodelica::Own::own(__pa3);
                    ct = metamodelica::Own::own(__pa4);
                    prl1 = metamodelica::Own::own(__pa5);
                    var1 = metamodelica::Own::own(__pa6);
                    dir = metamodelica::Own::own(__pa7);
                    let (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), metamodelica::AsArg::as_arg(&env), id.clone())?) {
                        (__pa8, _, Deref @ SCode::Element::COMPONENT { name: __pa9, prefixes: _, attributes: _, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa10, arrayDim: _ }, modifications: _, comment: _, condition: __pa11, info: __pa12 }, _, _, __pa13) => (__pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa8);
                    n = metamodelica::Own::own(__pa9);
                    t = metamodelica::Own::own(__pa10);
                    cond = metamodelica::Own::own(__pa11);
                    info = metamodelica::Own::own(__pa12);
                    idENV = metamodelica::Own::own(__pa13);
                    ci_state = InstUtil::updateClassInfState(cache.clone(), idENV.clone(), metamodelica::AsArg::as_arg(&env), inCIState.clone());
                    (cache, cl, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &t, None)?;
                    updatedComps = getUpdatedCompsHashTable(inUpdatedComps.clone());
                    (mods, cmod, m) = InstUtil::noModForUpdatedComponents(var1, &updatedComps, cref.clone(), mods.clone(), cmod.clone(), m.clone());
                    crefs = InstUtil::getCrefFromMod(m.clone())?;
                    crefs2 = InstUtil::getCrefFromDim(&ad)?;
                    crefs3 = InstUtil::getCrefFromCond(cond.clone())?;
                    crefs_1 = listAppend(crefs.clone(), listAppend(crefs2.clone(), crefs3.clone()));
                    crefs_2 = InstUtil::removeCrefFromCrefs(crefs_1.clone(), cref.clone())?;
                    updatedComps = BaseHashTable::add((cref.clone(), 0), updatedComps.clone())?;
                    let (__pa15, __pa16, __pa17, __pa18) = ::match_deref::match_deref! { match &(updateComponentsInEnv2(cache.clone(), env.clone(), ih.clone(), pre.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), &crefs_2, ci_state.clone(), r#impl, Some(updatedComps.clone()), Some(cref.clone()))) {
                        (__pa15, __pa16, __pa17, Some(__pa18)) => (__pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa15);
                    env2 = metamodelica::Own::own(__pa16);
                    ih = metamodelica::Own::own(__pa17);
                    updatedComps = metamodelica::Own::own(__pa18);
                    (cache, env_1, ih, updatedComps) = updateComponentInEnv2(cache.clone(), env2.clone(), cenv.clone(), ih.clone(), pre.clone(), t.clone(), n.clone(), ad.clone(), cl.clone(), attr.clone(), pf.clone(), metamodelica::Ref::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct, None), parallelism: prl1, variability: var1, direction: dir, innerOuter: io, visibility: visibility.clone() }), info.clone(), m.clone(), cmod.clone(), mods.clone(), cref.clone(), ci_state.clone(), r#impl, updatedComps.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), Some(updatedComps.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { name, .. }, .. }) => {
                    let mut id: ArcStr;
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut updatedComps: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
                    let mut env = (*env).clone();
                    id = AbsynUtil::crefFirstIdent(&cref)?;
                    let true = (stringEq(&name, &id)) else { return Err("pattern mismatch") };
                    (cl, _) = Lookup::lookupClassLocal(env.clone(), name.clone())?;
                    env = FGraph::updateClass(env.clone(), SCodeUtil::mergeWithOriginal(var_field!((*r#mod).element, DAE::Mod::REDECL).clone(), &cl), &pre, &(r#mod.clone()), &(openmodelica_frontend_dump::FCore::Status::CLS_UNTYPED), metamodelica::AsArg::as_arg(&env))?;
                    updatedComps = getUpdatedCompsHashTable(inUpdatedComps.clone());
                    updatedComps = BaseHashTable::add((cref.clone(), 0), updatedComps.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), Some(updatedComps.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _) => {
                    let mut id: ArcStr;
                    let mut is: FCore::Status;
                    let mut cache = (*cache).clone();
                    id = AbsynUtil::crefFirstIdent(&cref)?;
                    (cache, _, _, _, is, _) = Lookup::lookupIdent(cache.clone(), metamodelica::AsArg::as_arg(&env), id.clone())?;
                    let true = (FCore::isTyped(&is)) else { return Err("pattern mismatch") };
                    Ok((cache.clone(), env.clone(), ih.clone(), inUpdatedComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, mods) => {
                    let mut n: ArcStr;
                    let mut id: ArcStr;
                    let mut ct: SCode::ConnectorType;
                    let mut io: Absyn::InnerOuter;
                    let mut attr: SCode::Attributes;
                    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut prl1: SCode::Parallelism;
                    let mut var1: SCode::Variability;
                    let mut dir: Absyn::Direction;
                    let mut t: metamodelica::Ref<Absyn::Path>;
                    let mut m: metamodelica::Ref<SCode::Mod>;
                    let mut cmod: metamodelica::Ref<DAE::Mod>;
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs_2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
                    let mut info: SourceInfo;
                    let mut pf: metamodelica::Ref<SCode::Prefixes>;
                    let mut visibility: SCode::Visibility;
                    let mut idENV: FCore::Graph;
                    let mut updatedComps: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
                    let mut ci_state: ClassInf::State;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut mods = (*mods).clone();
                    id = AbsynUtil::crefFirstIdent(&cref)?;
                    let (__pa0, __pa1, __pa4, __pa2, __pa3, __pa10, __pa5, __pa6, __pa7, __pa8, __pa9, __pa11, __pa12, __pa13, __pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), metamodelica::AsArg::as_arg(&env), id.clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { name: __pa1, prefixes: __pa4 @ Deref @ SCode::Prefixes { innerOuter: __pa2, visibility: __pa3, .. }, attributes: __pa10 @ SCode::Attributes { arrayDims: __pa5, connectorType: __pa6, parallelism: __pa7, variability: __pa8, direction: __pa9, .. }, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa11, arrayDim: _ }, modifications: __pa12, comment: _, condition: __pa13, info: __pa14 }, __pa15, _, __pa16) => (__pa0.clone(), __pa1.clone(), __pa4.clone(), __pa2.clone(), __pa3.clone(), __pa10.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    n = metamodelica::Own::own(__pa1);
                    io = metamodelica::Own::own(__pa2);
                    visibility = metamodelica::Own::own(__pa3);
                    pf = metamodelica::Own::own(__pa4);
                    ad = metamodelica::Own::own(__pa5);
                    ct = metamodelica::Own::own(__pa6);
                    prl1 = metamodelica::Own::own(__pa7);
                    var1 = metamodelica::Own::own(__pa8);
                    dir = metamodelica::Own::own(__pa9);
                    attr = metamodelica::Own::own(__pa10);
                    t = metamodelica::Own::own(__pa11);
                    m = metamodelica::Own::own(__pa12);
                    cond = metamodelica::Own::own(__pa13);
                    info = metamodelica::Own::own(__pa14);
                    cmod = metamodelica::Own::own(__pa15);
                    idENV = metamodelica::Own::own(__pa16);
                    ci_state = InstUtil::updateClassInfState(cache.clone(), idENV.clone(), metamodelica::AsArg::as_arg(&env), inCIState.clone());
                    (cache, cl, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &t, None)?;
                    updatedComps = getUpdatedCompsHashTable(inUpdatedComps.clone());
                    (mods, cmod, m) = InstUtil::noModForUpdatedComponents(var1, &updatedComps, cref.clone(), mods.clone(), cmod.clone(), m.clone());
                    crefs = List::flatten(list![InstUtil::getCrefFromMod(m.clone())?, InstUtil::getCrefFromDim(&ad)?, InstUtil::getCrefFromCond(cond.clone())?, Mod::getUntypedCrefs(&cmod)])?;
                    crefs_2 = InstUtil::removeCrefFromCrefs(crefs.clone(), cref.clone())?;
                    crefs_2 = InstUtil::removeOptCrefFromCrefs(crefs_2.clone(), currentCref.clone())?;
                    updatedComps = BaseHashTable::add((cref.clone(), 0), updatedComps.clone())?;
                    let (__pa18, __pa19, __pa20, __pa21) = ::match_deref::match_deref! { match &(updateComponentsInEnv2(cache.clone(), env.clone(), ih.clone(), pre.clone(), mods.clone(), &crefs_2, ci_state.clone(), r#impl, Some(updatedComps.clone()), Some(cref.clone()))) {
                        (__pa18, __pa19, __pa20, Some(__pa21)) => (__pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa18);
                    env2 = metamodelica::Own::own(__pa19);
                    ih = metamodelica::Own::own(__pa20);
                    updatedComps = metamodelica::Own::own(__pa21);
                    (cache, env_1, ih, updatedComps) = updateComponentInEnv2(cache.clone(), env2.clone(), cenv.clone(), ih.clone(), pre.clone(), t.clone(), n.clone(), ad.clone(), cl.clone(), attr.clone(), pf.clone(), metamodelica::Ref::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct, None), parallelism: prl1, variability: var1, direction: dir, innerOuter: io, visibility: visibility }), info.clone(), m.clone(), cmod.clone(), mods.clone(), cref.clone(), ci_state.clone(), r#impl, updatedComps.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), Some(updatedComps.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), inUpdatedComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.updateComponentInEnv failed, cref = ")); __mm_s.push_str(&*Dump::printComponentRefStr(&cref)?); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" mods: ")); __mm_s.push_str(&*Mod::printModStr(&r#mod)?); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" prefix: ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(&pre)?); ArcStr::from(__mm_s) })?;
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
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), inUpdatedComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outEnv, outIH, outUpdatedComps)
}

fn updateComponentInEnv2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut cenv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut pre: DAE::Prefix,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut name: ArcStr,
    mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut cl: metamodelica::Ref<SCode::Element>,
    mut attr: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut dattr: metamodelica::Ref<DAE::Attributes>,
    mut info: SourceInfo,
    mut m: metamodelica::Ref<SCode::Mod>,
    mut cmod: metamodelica::Ref<DAE::Mod>,
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut ci_state: ClassInf::State,
    mut r#impl: bool,
    mut inUpdatedComps: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outUpdatedComps: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable5::FuncHashCref,
            HashTable5::FuncCrefEqual,
            HashTable5::FuncCrefStr,
            HashTable5::FuncExpStr,
        ),
    );
    match '__try0: {
        ErrorExt::setCheckpoint(literal!("Inst.updateComponentInEnv2"));
        (outCache, outEnv, outIH, outUpdatedComps) = unwrap_break_err!(updateComponentInEnv2_dispatch(inCache.clone(), inEnv.clone(), cenv.clone(), inIH.clone(), pre.clone(), path.clone(), name.clone(), ad.clone(), cl.clone(), attr.clone(), inPrefixes.clone(), dattr.clone(), info.clone(), m.clone(), cmod.clone(), r#mod.clone(), cref.clone(), ci_state.clone(), r#impl, inUpdatedComps.clone()), '__try0);
        ErrorExt::delCheckpoint(literal!("Inst.updateComponentInEnv2"));
        Ok::<_, &'static str>((outCache.clone(), outEnv.clone(), outIH.clone(), outUpdatedComps.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            outCache = __try0_o0;
            outEnv = __try0_o1;
            outIH = __try0_o2;
            outUpdatedComps = __try0_o3;
        }
        Err(__try0_err) => {
            ErrorExt::rollBack(literal!("Inst.updateComponentInEnv2"));
            return Err(__try0_err);
        }
    }
    Ok((outCache, outEnv, outIH, outUpdatedComps))
}

fn updateComponentInEnv2_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inClsEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inName: ArcStr,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inAttr: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inDAttr: metamodelica::Ref<DAE::Attributes>,
    mut inInfo: SourceInfo,
    mut inSMod: metamodelica::Ref<SCode::Mod>,
    mut inClsMod: metamodelica::Ref<DAE::Mod>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inState: ClassInf::State,
    mut inImpl: bool,
    mut inUpdatedComps: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outEnv: FCore::Graph = inEnv;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH;
    let mut outUpdatedComps: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable5::FuncHashCref,
            HashTable5::FuncCrefEqual,
            HashTable5::FuncCrefStr,
            HashTable5::FuncExpStr,
        ),
    ) = inUpdatedComps.clone();
    let mut smod: metamodelica::Ref<SCode::Mod>;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let mut mod1: metamodelica::Ref<DAE::Mod>;
    let mut mod2: metamodelica::Ref<DAE::Mod>;
    let mut class_mod: metamodelica::Ref<DAE::Mod>;
    let mut eq: Option<DAE::EqMod>;
    let mut own_cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut cls_env: FCore::Graph;
    let mut comp_env: FCore::Graph;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut binding: metamodelica::Ref<DAE::Binding>;
    let mut var: metamodelica::Ref<DAE::Var>;
    if '__try0: {
        let 1 = (unwrap_break_err!(BaseHashTable::get(inCref.clone(), &inUpdatedComps), '__try0)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        smod = SCodeUtil::mergeModifiers(inSMod.clone(), SCodeUtil::getConstrainedByModifiers(&inPrefixes));
        (outCache, mod1) = updateComponentInEnv3(
            outCache.clone(),
            outEnv.clone(),
            outIH.clone(),
            smod.clone(),
            inImpl,
            Mod::ModScope::COMPONENT { name: inName.clone() },
            inInfo.clone(),
        )?;
        class_mod = Mod::lookupModificationP(inMod.clone(), &inPath)?;
        mod2 = Mod::merge(class_mod.clone(), mod1.clone(), inName.clone(), true)?;
        mod2 = Mod::merge(inClsMod.clone(), mod2.clone(), inName.clone(), true)?;
        (outCache, mod2) = Mod::updateMod(
            outCache.clone(),
            outEnv.clone(),
            outIH.clone(),
            openmodelica_frontend_types::DAE::Prefix::NOPRE,
            mod2.clone(),
            inImpl,
            &inInfo,
        )?;
        r#mod = if (InstUtil::redeclareBasicType(&inClsMod)) {
            mod1.clone()
        } else {
            mod2.clone()
        };
        eq = Mod::modEquation(&r#mod);
        own_cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
            name: inName.clone(),
            subscripts: metamodelica::nil(),
        });
        (outCache, dims) = InstUtil::elabArraydim(
            outCache.clone(),
            outEnv.clone(),
            own_cref.clone(),
            inPath.clone(),
            inSubscripts.clone(),
            eq.clone(),
            inImpl,
            true,
            false,
            inPrefix.clone(),
            inInfo.clone(),
            metamodelica::nil(),
        )?;
        (cls_env, cls, outIH) = FGraph::createVersionScope(
            &outEnv,
            inName.clone(),
            &inPrefix,
            r#mod.clone(),
            inClsEnv.clone(),
            inClass.clone(),
            outIH.clone(),
        )?;
        (outCache, comp_env, outIH, _, _, _, ty, _) = InstVar::instVar(
            outCache.clone(),
            cls_env.clone(),
            outIH.clone(),
            UnitAbsyn::noStore().clone(),
            inState.clone(),
            r#mod.clone(),
            inPrefix.clone(),
            inName.clone(),
            cls.clone(),
            inAttr.clone(),
            inPrefixes.clone(),
            dims.clone(),
            metamodelica::nil(),
            metamodelica::nil(),
            inImpl,
            SCode::noComment.clone(),
            &inInfo,
            ConnectionGraph::EMPTY().clone(),
            Connect::emptySet().clone(),
            &outEnv,
        )?;
        (outCache, binding) = InstBinding::makeBinding(
            outCache.clone(),
            &outEnv,
            &inAttr,
            &r#mod,
            ty.clone(),
            &inPrefix,
            &inName,
            &inInfo,
        )?;
        var = metamodelica::Ref::new(DAE::Var {
            name: inName.clone(),
            attributes: inDAttr.clone(),
            ty: ty.clone(),
            binding: binding.clone(),
            bind_from_outside: false,
            constOfForIteratorRange: None,
        });
        outEnv = FGraph::updateComp(
            outEnv.clone(),
            var.clone(),
            &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED),
            &comp_env,
        );
        outUpdatedComps = BaseHashTable::add((inCref.clone(), 1), outUpdatedComps.clone())?;
    }
    Ok((outCache, outEnv, outIH, outUpdatedComps))
}

fn updateComponentInEnv3(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inImpl: bool,
    mut inModScope: Mod::ModScope,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Mod>)> {
    let mut outCache: FCore::Cache;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    (outCache, outMod) = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r#mod: metamodelica::Ref<DAE::Mod>;
            let mut cache: FCore::Cache;
            ErrorExt::setCheckpoint(literal!("updateComponentInEnv3"));
            (cache, r#mod) = Mod::elabMod(
                inCache.clone(),
                inEnv.clone(),
                inIH.clone(),
                openmodelica_frontend_types::DAE::Prefix::NOPRE,
                inMod.clone(),
                inImpl,
                inModScope.clone(),
                inInfo.clone(),
            )?;
            ErrorExt::rollBack(literal!("updateComponentInEnv3"));
            Ok((cache.clone(), r#mod.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ErrorExt::rollBack(literal!("updateComponentInEnv3"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outMod))
}

pub fn makeEnvFromProgram(
    mut prog: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut env_1: FCore::Graph;
    let mut env: FCore::Graph;
    let mut cache: FCore::Cache;
    (cache, env) = Builtin::initialGraph(FCore::emptyCache())?;
    env_1 = FGraphBuildEnv::mkProgramGraph(prog, openmodelica_frontend_dump::FCore::Kind::USERDEFINED, env)?;
    outCache = cache;
    Ok((outCache, env_1))
}

pub fn makeFullyQualified(
    mut cache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::Path>)> {
    let mut cache: FCore::Cache = cache;
    let mut path: metamodelica::Ref<Absyn::Path> = path;
    (cache, path) = (match &*path.clone() {
        Absyn::Path::IDENT { name: __path_name } => {
            (cache, path) = makeFullyQualifiedIdent(cache, inEnv, __path_name.clone(), path)?;
            (cache, path)
        }
        Absyn::Path::FULLYQUALIFIED { .. } => (cache, path),
        Absyn::Path::QUALIFIED { .. } => {
            (cache, path) = makeFullyQualifiedFromQual(cache, inEnv, path);
            (cache, path)
        }
    });
    Ok((cache, path))
}

fn makeFullyQualifiedFromQual(
    mut cache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> (FCore::Cache, metamodelica::Ref<Absyn::Path>) {
    let mut cache: FCore::Cache = cache;
    let mut path: metamodelica::Ref<Absyn::Path> = path;
    (cache, path) = 'mc: {
        let __mc_input = path.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env_1: FCore::Graph;
                    let mut path_2: metamodelica::Ref<Absyn::Path>;
                    let mut name: ArcStr;
                    let mut cache: FCore::Cache = cache.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&cache, &inEnv, &path, None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    name = metamodelica::Own::own(__pa1);
                    env_1 = metamodelica::Own::own(__pa2);
                    path_2 = makeFullyQualified2(&env_1, name.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    Ok(((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())), cache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env: FCore::Graph;
                    let mut path3: metamodelica::Ref<Absyn::Path>;
                    let mut crPath: metamodelica::Ref<DAE::ComponentRef>;
                    let mut name: ArcStr;
                    let mut cache: FCore::Cache = cache.clone();
                    crPath = ComponentReference::pathToCref(&path);
                    (cache, _, _, _, _, _, env, _, name) = Lookup::lookupVarInternal(cache.clone(), &inEnv, crPath.clone(), openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN)?;
                    path3 = makeFullyQualified2(&env, name.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    Ok(((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())), cache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env: FCore::Graph;
                    let mut path3: metamodelica::Ref<Absyn::Path>;
                    let mut crPath: metamodelica::Ref<DAE::ComponentRef>;
                    let mut name: ArcStr;
                    let mut cache: FCore::Cache = cache.clone();
                    crPath = ComponentReference::pathToCref(&path);
                    (cache, env, _, _, _, _, _, _, name) = Lookup::lookupVarInPackages(cache.clone(), inEnv.clone(), crPath.clone(), metamodelica::nil(), Mutable::create(false))?;
                    path3 = makeFullyQualified2(&env, name.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    Ok(((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())), cache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((cache.clone(), path.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (cache, path)
}

pub(crate) fn makeFullyQualifiedIdent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ident: ArcStr,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::Path>)> {
    let mut outCache: FCore::Cache;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut isKnownBuiltin: bool;
    (outPath, isKnownBuiltin) = makeFullyQualifiedIdentCheckBuiltin(&ident)?;
    if isKnownBuiltin {
        outCache = inCache;
        return Ok((outCache, outPath));
    }
    (outCache, outPath) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv, ident.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut env_1: FCore::Graph;
            let mut path_2: metamodelica::Ref<Absyn::Path>;
            let mut name: ArcStr;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClassIdent(cache.clone(), env.clone(), &ident, None)?) {
                (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            name = metamodelica::Own::own(__pa1);
            env_1 = metamodelica::Own::own(__pa2);
            path_2 = makeFullyQualified2(
                &env_1,
                name.clone(),
                metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
            )?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, mut s) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut path_2: metamodelica::Ref<Absyn::Path>;
            let mut name: ArcStr;
            let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
            r = FGraph::lastScopeRef(&(env.clone()))?;
            let false = (FNode::isRefTop(r.clone())) else {
                return Err("pattern mismatch");
            };
            name = FNode::refName(r.clone());
            let true = (metamodelica::stringEq(&name, &s)) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(&(env.clone()))?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            path_2 = metamodelica::Own::own(__pa0);
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, mut s) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut env_1: FCore::Graph;
            let mut path_2: metamodelica::Ref<Absyn::Path>;
            (cache, _, env_1) = Lookup::lookupTypeIdent(cache.clone(), env.clone(), s.clone(), None)?;
            path_2 = makeFullyQualified2(&env_1, s.clone(), inPath.clone())?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut path3: metamodelica::Ref<Absyn::Path>;
            let mut name: ArcStr;
            (cache, _, _, _, _, _, env, _, name) = Lookup::lookupVarInternalIdent(
                cache.clone(),
                &(env.clone()),
                &ident,
                &(metamodelica::nil()),
                openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN,
            )?;
            path3 = makeFullyQualified2(
                &(env.clone()),
                name.clone(),
                metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
            )?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut path3: metamodelica::Ref<Absyn::Path>;
            let mut name: ArcStr;
            (cache, env, _, _, _, _, _, _, name) = Lookup::lookupVarInPackagesIdent(
                cache.clone(),
                env.clone(),
                &ident,
                &(metamodelica::nil()),
                metamodelica::nil(),
                Mutable::create(false),
            )?;
            path3 = makeFullyQualified2(
                &(env.clone()),
                name.clone(),
                metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
            )?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((
                inCache.clone(),
                (::match_deref::match_deref! { match &(&*inPath) {
                    Deref @ Absyn::Path::IDENT { name: Deref @ "" } => metamodelica::Ref::new(Absyn::Path::IDENT { name: ident.clone() }),
                    _ => inPath.clone(),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } }),
            ))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outPath))
}

fn makeFullyQualifiedIdentCheckBuiltin(mut ident: &ArcStr) -> Result<(metamodelica::Ref<Absyn::Path>, bool)> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut isKnownBuiltin: bool = true;
    path = (::match_deref::match_deref! { match &(ident.clone()) {
        Deref @ "Boolean" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Boolean") }) }),
        Deref @ "Integer" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Integer") }) }),
        Deref @ "Real" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") }) }),
        Deref @ "String" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("String") }) }),
        Deref @ "EnumType" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("EnumType") }) }),
        Deref @ "assert" => metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("assert") }),
        Deref @ "reinit" => metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("reinit") }),
        Deref @ "smooth" => metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("smooth") }),
        Deref @ "list" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("list") })
        },
        Deref @ "Option" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Option") })
        },
        Deref @ "tuple" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("tuple") })
        },
        Deref @ "polymorphic" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("polymorphic") })
        },
        Deref @ "array" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("array") })
        },
        _ => {
            isKnownBuiltin = false;
            metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((path, isKnownBuiltin))
}

pub(crate) fn instList<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut instFunc: &dyn ::std::ops::Fn(
        FCore::Cache,
        FCore::Graph,
        metamodelica::List<InnerOuter::TopInstance>,
        DAE::Prefix,
        DAE::Connect::Sets,
        ClassInf::State,
        Type_a,
        bool,
        bool,
        ConnectionGraph::ConnectionGraph,
    ) -> Result<(
        FCore::Cache,
        FCore::Graph,
        metamodelica::List<InnerOuter::TopInstance>,
        DAE::DAElist,
        DAE::Connect::Sets,
        ClassInf::State,
        ConnectionGraph::ConnectionGraph,
    )>,
    mut inTypeALst: &metamodelica::List<Type_a>,
    mut inImplicit: bool,
    mut unrollForLoops: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    pub type InstFunc<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<InnerOuter::TopInstance>,
                DAE::Prefix,
                DAE::Connect::Sets,
                ClassInf::State,
                Type_a,
                bool,
                bool,
                ConnectionGraph::ConnectionGraph,
            ) -> Result<(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<InnerOuter::TopInstance>,
                DAE::DAElist,
                DAE::Connect::Sets,
                ClassInf::State,
                ConnectionGraph::ConnectionGraph,
            )> + 'static,
    >;

    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = (::match_deref::match_deref! { match inTypeALst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut csets = inSets;
            let mut ci_state = inState;
            let mut graph = inGraph;
            (cache, env, ih, DAE::emptyDae().clone(), csets, ci_state, graph)
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: es } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut pre = inPrefix;
            let mut csets = inSets;
            let mut ci_state = inState;
            let mut r#impl = inImplicit;
            let mut graph = inGraph;
            let mut env_1: FCore::Graph;
            let mut env_2: FCore::Graph;
            let mut csets_1: DAE::Connect::Sets;
            let mut csets_2: DAE::Connect::Sets;
            let mut ci_state_1: ClassInf::State;
            let mut ci_state_2: ClassInf::State;
            let mut dae1: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut dae: DAE::DAElist;
            (cache, env_1, ih, dae1, csets_1, ci_state_1, graph) = instFunc(cache, env, ih, pre.clone(), csets, ci_state, e.clone(), r#impl, unrollForLoops, graph)?;
            (cache, env_2, ih, dae2, csets_2, ci_state_2, graph) = instList(cache, env_1, ih, pre, csets_1, ci_state_1, instFunc, es, r#impl, unrollForLoops, graph)?;
            dae = DAEUtil::joinDaes(&dae1, &dae2)?;
            (cache, env_2, ih, dae, csets_2, ci_state_2, graph)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn instConstraints(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPrefix: &DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inConstraints: &metamodelica::List<SCode::ConstraintSection>,
    mut inImpl: bool,
) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist, ClassInf::State)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outDae: DAE::DAElist;
    let mut outState: ClassInf::State;
    (outCache, outEnv, outDae, outState) = (::match_deref::match_deref! { match inConstraints {
        Deref @ metamodelica::ListNode::Nil => {
            (inCache.clone(), inEnv.clone(), DAE::emptyDae().clone(), inState.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: constr, tail: rest } => {
            let mut env1: FCore::Graph;
            let mut env2: FCore::Graph;
            let mut constraints_1: DAE::DAElist;
            let mut constraints_2: DAE::DAElist;
            let mut ci_state: ClassInf::State;
            let mut cache: FCore::Cache;
            let mut dae: DAE::DAElist;
            (cache, env1, constraints_1, ci_state) = InstSection::instConstraint(inCache.clone(), inEnv.clone(), inPrefix.clone(), inState.clone(), metamodelica::AsArg::as_arg(&constr), inImpl)?;
            (cache, env2, constraints_2, ci_state) = instConstraints(&cache, &env1, inPrefix, &ci_state, rest, inImpl)?;
            dae = DAEUtil::joinDaes(&constraints_1, &constraints_2)?;
            (cache, env2, dae, ci_state)
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("- Inst.instConstraints failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outDae, outState))
}

fn instClassAttributes(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPrefix: DAE::Prefix,
    mut inAttrs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outDae: DAE::DAElist;
    (outCache, outEnv, outDae) = (::match_deref::match_deref! { match &(inAttrs.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache.clone();
            let mut env = inEnv.clone();
            (cache, env, DAE::emptyDae().clone())
        },
        _ => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut clsAttrs: DAE::DAElist;
            let mut dae: DAE::DAElist;
            clsAttrs = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: metamodelica::Ref::new(DAE::ClassAttributes { objetiveE: None, objectiveIntegrandE: None, startTimeE: None, finalTimeE: None }) })] };
            (cache, env, dae) = instClassAttributes2(inCache, inEnv, inPrefix, inAttrs, inImplicit, inInfo, clsAttrs)?;
            (cache, env, dae)
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("- Inst.instClassAttributes failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outDae))
}

fn instClassAttributes2<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPrefix: DAE::Prefix,
    mut inAttrs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inInfo: &'__b SourceInfo,
    mut inClsAttrs: DAE::DAElist,
) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, inPrefix, inAttrs, inImplicit, inClsAttrs)) {
            (cache, env, _, Deref @ metamodelica::ListNode::Nil, _, clsAttrs) => {
                return Ok((cache.clone(), env.clone(), clsAttrs.clone()))
            },
            (cache, env, pre, Deref @ metamodelica::ListNode::Cons { head: na, tail: rest }, r#impl, clsAttrs) => {
                let mut env_2: FCore::Graph;
                let mut attrName: ArcStr;
                let mut attrExp: metamodelica::Ref<Absyn::Exp>;
                let mut outExp: metamodelica::Ref<DAE::Exp>;
                let mut cache = (*cache).clone();
                let mut clsAttrs = (*clsAttrs).clone();
                let __arc2 = na.clone();
                let Absyn::NAMEDARG { argName: __pa0, argValue: __pa1 } = &*__arc2;
                attrName = metamodelica::Own::own(__pa0);
                attrExp = metamodelica::Own::own(__pa1);
                (cache, outExp, _) = Static::elabExp(cache.clone(), env.clone(), attrExp, r#impl.clone(), false, pre.clone(), inInfo.clone())?;
                clsAttrs = insertClassAttribute(clsAttrs.clone(), &attrName, outExp)?;
                { (inCache, inEnv, inPrefix, inAttrs, inImplicit, inInfo, inClsAttrs) = (cache.clone(), env.clone(), pre.clone(), rest.clone(), r#impl.clone(), inInfo, clsAttrs.clone()); continue '__tco; }
            },
            _ => {
                Error::addMessage(Error::OPTIMICA_ERROR.clone(), list![literal!("Class Attributes allowed only for Optimization classes.")])?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn insertClassAttribute(
    mut inAttrs: DAE::DAElist,
    mut attrName: &ArcStr,
    mut inAttrExp: metamodelica::Ref<DAE::Exp>,
) -> Result<DAE::DAElist> {
    let mut outAttrs: DAE::DAElist;
    outAttrs = (::match_deref::match_deref! { match &(attrName.clone()) {
        Deref @ "objective" => {
            let mut attrs = inAttrs;
            let mut startTimeE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut finalTimeE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut objectiveIntegrandE: Option<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(attrs) {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::CLASS_ATTRIBUTES { classAttrs: Deref @ DAE::ClassAttributes { objetiveE: _, objectiveIntegrandE: __pa0, startTimeE: __pa1, finalTimeE: __pa2 } }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            objectiveIntegrandE = metamodelica::Own::own(__pa0);
            startTimeE = metamodelica::Own::own(__pa1);
            finalTimeE = metamodelica::Own::own(__pa2);
            attrs = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: metamodelica::Ref::new(DAE::ClassAttributes { objetiveE: Some(inAttrExp), objectiveIntegrandE: objectiveIntegrandE, startTimeE: startTimeE, finalTimeE: finalTimeE }) })] };
            attrs
        },
        Deref @ "objectiveIntegrand" => {
            let mut attrs = inAttrs;
            let mut objectiveE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut startTimeE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut finalTimeE: Option<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(attrs) {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::CLASS_ATTRIBUTES { classAttrs: Deref @ DAE::ClassAttributes { objetiveE: __pa0, objectiveIntegrandE: _, startTimeE: __pa1, finalTimeE: __pa2 } }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            objectiveE = metamodelica::Own::own(__pa0);
            startTimeE = metamodelica::Own::own(__pa1);
            finalTimeE = metamodelica::Own::own(__pa2);
            attrs = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: metamodelica::Ref::new(DAE::ClassAttributes { objetiveE: objectiveE, objectiveIntegrandE: Some(inAttrExp), startTimeE: startTimeE, finalTimeE: finalTimeE }) })] };
            attrs
        },
        Deref @ "startTime" => {
            let mut attrs = inAttrs;
            let mut objectiveE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut finalTimeE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut objectiveIntegrandE: Option<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(attrs) {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::CLASS_ATTRIBUTES { classAttrs: Deref @ DAE::ClassAttributes { objetiveE: __pa0, objectiveIntegrandE: __pa1, startTimeE: _, finalTimeE: __pa2 } }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            objectiveE = metamodelica::Own::own(__pa0);
            objectiveIntegrandE = metamodelica::Own::own(__pa1);
            finalTimeE = metamodelica::Own::own(__pa2);
            attrs = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: metamodelica::Ref::new(DAE::ClassAttributes { objetiveE: objectiveE, objectiveIntegrandE: objectiveIntegrandE, startTimeE: Some(inAttrExp), finalTimeE: finalTimeE }) })] };
            attrs
        },
        Deref @ "finalTime" => {
            let mut attrs = inAttrs;
            let mut objectiveE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut startTimeE: Option<metamodelica::Ref<DAE::Exp>>;
            let mut objectiveIntegrandE: Option<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(attrs) {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::CLASS_ATTRIBUTES { classAttrs: Deref @ DAE::ClassAttributes { objetiveE: __pa0, objectiveIntegrandE: __pa1, startTimeE: __pa2, finalTimeE: _ } }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            objectiveE = metamodelica::Own::own(__pa0);
            objectiveIntegrandE = metamodelica::Own::own(__pa1);
            startTimeE = metamodelica::Own::own(__pa2);
            attrs = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: metamodelica::Ref::new(DAE::ClassAttributes { objetiveE: objectiveE, objectiveIntegrandE: objectiveIntegrandE, startTimeE: startTimeE, finalTimeE: Some(inAttrExp) }) })] };
            attrs
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("- Inst.insertClassAttribute failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAttrs)
}

pub(crate) fn instantiateBoschClass(
    mut inCache: FCore::Cache,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDAElist: DAE::DAElist;
    (outCache, outEnv, outIH, outDAElist) = 'mc: {
        let __mc_input = (inCache, inIH, inProgram, inPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Error::addMessage(Error::NO_CLASSES_LOADED.clone(), metamodelica::nil())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::IDENT { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(metamodelica::AsArg::as_arg(&cdecls), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, env.clone())?;
                    (cache, env_2, ih, dae) = instBoschClassInProgram(cache.clone(), env_1.clone(), ih.clone(), metamodelica::AsArg::as_arg(&cdecls), path.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut cdef: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(metamodelica::AsArg::as_arg(&cdecls), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, env.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), &env_1, metamodelica::AsArg::as_arg(&path), Some(Absyn::dummyInfo.clone()))?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cdef = metamodelica::Own::own(__pa1);
                    env_2 = metamodelica::Own::own(__pa2);
                    (cache, env_2, ih, _, dae, _, _, _, _, _) = instClass(cache.clone(), env_2.clone(), ih.clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, path) => {
                    let mut cname_str: ArcStr;
                    cname_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::ERROR_FLATTENING.clone(), list![cname_str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDAElist))
}

fn instBoschClassInProgram(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    (outCache, outEnv, outIH, outDae) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, &**inProgram, inPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: c @ Deref @ SCode::Element::CLASS { name: name1, .. }, tail: _ }, Deref @ Absyn::Path::IDENT { name: name2 }) => {
                    let mut dae: DAE::DAElist;
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (stringEq(&name1, &name2)) else { return Err("pattern mismatch") };
                    (cache, env_1, ih, _, dae, _, _, _, _, _) = instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { name: name1, .. }, tail: cs }, path @ Deref @ Absyn::Path::IDENT { name: name2 }) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let false = (stringEq(&name1, &name2)) else { return Err("pattern mismatch") };
                    (cache, env, ih, dae) = instBoschClassInProgram(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&cs), path.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDae))
}

fn modifyInstantiateClass(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<(metamodelica::Ref<DAE::Mod>, metamodelica::Ref<DAE::Mod>)> {
    let mut omod1: metamodelica::Ref<DAE::Mod>;
    let mut omod2: metamodelica::Ref<DAE::Mod>;
    (omod1, omod2) = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { name: id, .. }, .. } => {
            if (metamodelica::stringEq(&id, &(AbsynUtil::pathString(path, literal!("."), true, false)?))) {(inMod, openmodelica_frontend_types::DAE::Mod::interned_NOMOD())} else {(openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), inMod)}
        },
        _ => {
            (openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), inMod)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((omod1, omod2))
}

fn removeSelfReferenceAndUpdate(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inStore: UnitAbsyn::InstStore,
    mut inRefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut inRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inState: ClassInf::State,
    mut iattr: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut r#impl: bool,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut pre: DAE::Prefix,
    mut mods: &metamodelica::Ref<DAE::Mod>,
    mut scodeMod: metamodelica::Ref<SCode::Mod>,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    UnitAbsyn::InstStore,
    metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outStore: UnitAbsyn::InstStore;
    let mut o1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    (outCache, outEnv, outIH, outStore, o1) = 'mc: {
        let __mc_input = (
            inCache, inEnv, inIH, inStore, inRefs, inRef, inPath, inState, iattr, inInstDims,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1, _, _, _, _) => {
                    let mut cl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut i1: i32;
                    let mut i2: i32;
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    i1 = ((cl2).len() as i32);
                    i2 = ((cl1).len() as i32);
                    let true = (i1 == i2) else { return Err("pattern mismatch") };
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { arrayDims: ad, connectorType: ct, parallelism: prl1, variability: var1, direction: dir, .. }, inst_dims) => {
                    let mut cl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut vis: SCode::Visibility;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut new_var: metamodelica::Ref<DAE::Var>;
                    let mut io: Absyn::InnerOuter;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut smod: metamodelica::Ref<SCode::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint(literal!("Inst.removeSelfReferenceAndUpdate"));
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&sty), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl, true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    smod = SCodeInstUtil::removeSelfReferenceFromMod(scodeMod.clone(), c1.clone())?;
                    (cache, m) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl, Mod::ModScope::COMPONENT { name: n.clone() }, info.clone())?;
                    (cenv, c, ih) = FGraph::createVersionScope(metamodelica::AsArg::as_arg(&env), n.clone(), &pre, m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), n.clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), &info, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), metamodelica::AsArg::as_arg(&env))?;
                    io = SCodeUtil::prefixesInnerOuter(&inPrefixes);
                    vis = SCodeUtil::prefixesVisibility(&inPrefixes);
                    new_var = metamodelica::Ref::new(DAE::Var { name: n.clone(), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io, visibility: vis }), ty: ty.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED), &compenv);
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { arrayDims: ad, connectorType: ct, parallelism: prl1, variability: var1, direction: dir, .. }, inst_dims) => {
                    let mut cl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut vis: SCode::Visibility;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut new_var: metamodelica::Ref<DAE::Var>;
                    let mut io: Absyn::InnerOuter;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut smod: metamodelica::Ref<SCode::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint(literal!("Inst.removeSelfReferenceAndUpdate"));
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&sty), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl, true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    smod = SCodeInstUtil::removeNonConstantBindingsKeepRedeclares(scodeMod.clone(), false)?;
                    (cache, m) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl, Mod::ModScope::COMPONENT { name: n.clone() }, info.clone())?;
                    (cenv, c, ih) = FGraph::createVersionScope(metamodelica::AsArg::as_arg(&env), n.clone(), &pre, m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), n.clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), &info, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), metamodelica::AsArg::as_arg(&env))?;
                    io = SCodeUtil::prefixesInnerOuter(&inPrefixes);
                    vis = SCodeUtil::prefixesVisibility(&inPrefixes);
                    new_var = metamodelica::Ref::new(DAE::Var { name: n.clone(), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io, visibility: vis }), ty: ty.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED), &compenv);
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { arrayDims: ad, connectorType: ct, parallelism: prl1, variability: var1, direction: dir, .. }, inst_dims) => {
                    let mut cl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut vis: SCode::Visibility;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut new_var: metamodelica::Ref<DAE::Var>;
                    let mut io: Absyn::InnerOuter;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut smod: metamodelica::Ref<SCode::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint(literal!("Inst.removeSelfReferenceAndUpdate"));
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&sty), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl, true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    smod = SCodeInstUtil::removeNonConstantBindingsKeepRedeclares(scodeMod.clone(), true)?;
                    (cache, m) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl, Mod::ModScope::COMPONENT { name: n.clone() }, info.clone())?;
                    (cenv, c, ih) = FGraph::createVersionScope(metamodelica::AsArg::as_arg(&env), n.clone(), &pre, m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), n.clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), &info, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), metamodelica::AsArg::as_arg(&env))?;
                    io = SCodeUtil::prefixesInnerOuter(&inPrefixes);
                    vis = SCodeUtil::prefixesVisibility(&inPrefixes);
                    new_var = metamodelica::Ref::new(DAE::Var { name: n.clone(), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io, visibility: vis }), ty: ty.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED), &compenv);
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { arrayDims: ad, connectorType: ct, parallelism: prl1, variability: var1, direction: dir, .. }, inst_dims) => {
                    let mut cl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut vis: SCode::Visibility;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut new_var: metamodelica::Ref<DAE::Var>;
                    let mut io: Absyn::InnerOuter;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint(literal!("Inst.removeSelfReferenceAndUpdate"));
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&sty), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl, true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    m = openmodelica_frontend_types::DAE::Mod::interned_NOMOD();
                    (cenv, c, ih) = FGraph::createVersionScope(metamodelica::AsArg::as_arg(&env), n.clone(), &pre, m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), n.clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), &info, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), metamodelica::AsArg::as_arg(&env))?;
                    io = SCodeUtil::prefixesInnerOuter(&inPrefixes);
                    vis = SCodeUtil::prefixesVisibility(&inPrefixes);
                    new_var = metamodelica::Ref::new(DAE::Var { name: n.clone(), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io, visibility: vis }), ty: ty.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED), &compenv);
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack(literal!("Inst.removeSelfReferenceAndUpdate"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1, _, _, _, _) => {
                    let mut cl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outStore, o1))
}

fn updateComponentsInEnv2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut pre: DAE::Prefix,
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut crefs: &metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut ci_state: ClassInf::State,
    mut r#impl: bool,
    mut inUpdatedComps: Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    )>,
    mut currentCref: Option<metamodelica::Ref<Absyn::ComponentRef>>,
) -> (
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    )>,
) {
    let mut outCache: FCore::Cache = inCache;
    let mut outEnv: FCore::Graph = inEnv;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH;
    let mut outUpdatedComps: Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable5::FuncHashCref,
            HashTable5::FuncCrefEqual,
            HashTable5::FuncCrefStr,
            HashTable5::FuncExpStr,
        ),
    )> = inUpdatedComps;
    let mut name: ArcStr;
    let mut binding: metamodelica::Ref<DAE::Binding>;
    for mut cr in &**crefs {
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(cr.clone()) {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa1, subscripts: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa1);
            let (_, __t3, _, _, _, _) = unwrap_break_err!(Lookup::lookupIdentLocal(outCache.clone(), &outEnv, name.clone()), '__try0);
            let __arc4 = __t3.clone();
            let DAE::TYPES_VAR { binding: __pa2, .. } = &*__arc4;
            binding = metamodelica::Own::own(__pa2);
            let true = (DAEUtil::isBound(&binding)) else { break '__try0 Err::<_, _>("pattern mismatch") };
            Ok::<(), &'static str>(())
        }.is_err() {
            (outCache, outEnv, outIH, outUpdatedComps) = updateComponentInEnv(outCache.clone(), outEnv.clone(), outIH.clone(), pre.clone(), r#mod.clone(), cr.clone(), ci_state.clone(), r#impl, outUpdatedComps.clone(), currentCref.clone());
        }
    }
    (outCache, outEnv, outIH, outUpdatedComps)
}

fn makeFullyQualified2(
    mut env: &FCore::Graph,
    mut name: ArcStr,
    mut cachedPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut scope: metamodelica::Ref<Absyn::Path>;
    let mut oscope: Option<metamodelica::Ref<Absyn::Path>>;
    oscope = FGraph::getScopePath(env)?;
    if (oscope).is_none() {
        path = makeFullyQualified2Builtin(name, cachedPath);
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(oscope) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        scope = metamodelica::Own::own(__pa0);
        path = AbsynUtil::joinPaths(
            scope,
            (::match_deref::match_deref! { match &(cachedPath.clone()) {
                Deref @ Absyn::Path::IDENT { name: Deref @ "" } => metamodelica::Ref::new(Absyn::Path::IDENT { name: name }),
                _ => cachedPath,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }),
        )?;
    }
    Ok(path)
}

fn makeFullyQualified2Builtin(
    mut ident: ArcStr,
    mut cachedPath: metamodelica::Ref<Absyn::Path>,
) -> metamodelica::Ref<Absyn::Path> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = (::match_deref::match_deref! { match &(ident.clone()) {
        Deref @ "abs" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("abs") }) }),
        Deref @ "acos" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("acos") }) }),
        Deref @ "activeState" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("activeState") }) }),
        Deref @ "actualStream" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("actualStream") }) }),
        Deref @ "asin" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("asin") }) }),
        Deref @ "atan" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("atan") }) }),
        Deref @ "atan2" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("atan2") }) }),
        Deref @ "backSample" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("backSample") }) }),
        Deref @ "cardinality" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cardinality") }) }),
        Deref @ "cat" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cat") }) }),
        Deref @ "ceil" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ceil") }) }),
        Deref @ "change" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("change") }) }),
        Deref @ "classDirectory" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("classDirectory") }) }),
        Deref @ "cos" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cos") }) }),
        Deref @ "cosh" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cosh") }) }),
        Deref @ "cross" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cross") }) }),
        Deref @ "delay" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("delay") }) }),
        Deref @ "der" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }) }),
        Deref @ "diagonal" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("diagonal") }) }),
        Deref @ "div" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("div") }) }),
        Deref @ "edge" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("edge") }) }),
        Deref @ "exp" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("exp") }) }),
        Deref @ "fill" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("fill") }) }),
        Deref @ "firstTick" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("firstTick") }) }),
        Deref @ "floor" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("floor") }) }),
        Deref @ "getInstanceName" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("getInstanceName") }) }),
        Deref @ "hold" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("hold") }) }),
        Deref @ "homotopy" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("homotopy") }) }),
        Deref @ "identity" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("identity") }) }),
        Deref @ "inStream" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("inStream") }) }),
        Deref @ "initial" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("initial") }) }),
        Deref @ "initialState" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("initialState") }) }),
        Deref @ "integer" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("integer") }) }),
        Deref @ "interval" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("interval") }) }),
        Deref @ "intAbs" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("intAbs") }) }),
        Deref @ "linspace" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("linspace") }) }),
        Deref @ "log" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("log") }) }),
        Deref @ "log10" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("log10") }) }),
        Deref @ "matrix" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("matrix") }) }),
        Deref @ "max" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("max") }) }),
        Deref @ "min" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("min") }) }),
        Deref @ "mod" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("mod") }) }),
        Deref @ "ndims" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ndims") }) }),
        Deref @ "noClock" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("noClock") }) }),
        Deref @ "noEvent" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("noEvent") }) }),
        Deref @ "ones" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ones") }) }),
        Deref @ "outerProduct" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("outerProduct") }) }),
        Deref @ "pre" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("pre") }) }),
        Deref @ "previous" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("previous") }) }),
        Deref @ "print" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("print") }) }),
        Deref @ "product" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("product") }) }),
        Deref @ "realAbs" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("realAbs") }) }),
        Deref @ "rem" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("rem") }) }),
        Deref @ "rooted" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("rooted") }) }),
        Deref @ "sample" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }) }),
        Deref @ "scalar" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("scalar") }) }),
        Deref @ "semilinear" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("semilinear") }) }),
        Deref @ "shiftSample" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("shiftSample") }) }),
        Deref @ "sign" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sign") }) }),
        Deref @ "sin" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sin") }) }),
        Deref @ "sinh" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sinh") }) }),
        Deref @ "size" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("size") }) }),
        Deref @ "skew" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("skew") }) }),
        Deref @ "smooth" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("smooth") }) }),
        Deref @ "spatialDistribution" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("spatialDistribution") }) }),
        Deref @ "sqrt" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sqrt") }) }),
        Deref @ "subSample" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("subSample") }) }),
        Deref @ "symmetric" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("symmetric") }) }),
        Deref @ "tan" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("tan") }) }),
        Deref @ "tanh" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("tanh") }) }),
        Deref @ "terminal" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("terminal") }) }),
        Deref @ "ticksInState" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ticksInState") }) }),
        Deref @ "timeInState" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("timeInState") }) }),
        Deref @ "transition" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("transition") }) }),
        Deref @ "transpose" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("transpose") }) }),
        Deref @ "vector" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("vector") }) }),
        Deref @ "zeros" => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("zeros") }) }),
        _ => (::match_deref::match_deref! { match &(cachedPath.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "" } => metamodelica::Ref::new(Absyn::Path::IDENT { name: ident }),
        _ => cachedPath,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    path
}

pub(crate) fn getCachedInstance(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut name: ArcStr,
    mut r#ref: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut cache_path: metamodelica::Ref<Absyn::Path>;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut prefix: DAE::Prefix;
    let mut prefix2: DAE::Prefix;
    let mut env2: FCore::Graph;
    let mut enc: SCode::Encapsulated;
    let mut res: SCode::Restriction;
    let mut inputs: (
        metamodelica::Ref<DAE::Mod>,
        DAE::Prefix,
        DAE::Connect::Sets,
        ClassInf::State,
        metamodelica::Ref<SCode::Element>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
        bool,
        Option<metamodelica::Ref<DAE::ComponentRef>>,
        InstTypes::CallingScope,
    );
    let true = (Flags::isSet(Flags::CACHE.clone())?) else {
        return Err("pattern mismatch");
    };
    let (__pa2, __pa0, __pa1, __pa3) = ::match_deref::match_deref! { match &(FNode::refData(r#ref.clone())) {
        Deref @ FCore::Data::CL { e: __pa2 @ Deref @ SCode::Element::CLASS { encapsulatedPrefix: __pa0, restriction: __pa1, .. }, pre: __pa3, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    enc = metamodelica::Own::own(__pa0);
    res = metamodelica::Own::own(__pa1);
    cls = metamodelica::Own::own(__pa2);
    prefix = metamodelica::Own::own(__pa3);
    env2 = FGraph::openScope(env.clone(), enc, name, FGraph::restrictionToScopeType(&res))?;
    match '__try5: {
        cache_path = unwrap_break_err!(generateCachePath(&env2, &cls, &prefix, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL), '__try5);
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(unwrap_break_err!(InstHashTable::get(cache_path.clone()), '__try5)) {
            Deref @ metamodelica::ListNode::Cons { head: Some(InstHashTable::CachedInstItem::FUNC_instClassIn { inputs: __pa6, outputs: (__pa7, _, _, _, _, _, _, _, _) }), tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>("pattern mismatch"),
        } };
        inputs = metamodelica::Own::own(__pa6);
        env = metamodelica::Own::own(__pa7);
        (_, prefix2, _, _, _, _, _, _, _) = inputs.clone();
        let true = (PrefixUtil::isPrefix(&prefix) && PrefixUtil::isPrefix(&prefix2)) else {
            break '__try5 Err::<_, _>("pattern mismatch");
        };
        Ok::<_, &'static str>((env.clone(),))
    } {
        Ok((__try5_o0,)) => {
            env = __try5_o0;
        }
        Err(_) => {
            env = FGraph::pushScopeRef(env.clone(), r#ref.clone())?;
        }
    }
    Ok((cache, env))
}

fn generateCachePath(
    mut env: &FCore::Graph,
    mut cls: &metamodelica::Ref<SCode::Element>,
    mut prefix: &DAE::Prefix,
    mut callScope: InstTypes::CallingScope,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut cachePath: metamodelica::Ref<Absyn::Path>;
    let mut name: ArcStr;
    name = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*InstTypes::callingScopeStr(callScope));
        __mm_s.push_str(&*literal!("$"));
        __mm_s.push_str(&*SCodeDump::restrString(&(SCodeUtil::getClassRestriction(cls)?))?);
        __mm_s.push_str(&*literal!("$"));
        __mm_s.push_str(&*generatePrefixStr(prefix));
        __mm_s.push_str(&*literal!("$"));
        ArcStr::from(__mm_s)
    };
    cachePath = AbsynUtil::joinPaths(
        metamodelica::Ref::new(Absyn::Path::IDENT { name: name }),
        FGraph::getGraphName(env)?,
    )?;
    Ok(cachePath)
}

pub(crate) fn generatePrefixStr(mut inPrefix: &DAE::Prefix) -> ArcStr {
    let mut r#str: ArcStr;
    match '__try0: {
        r#str = unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(PrefixUtil::prefixToPath(inPrefix), '__try0), literal!("$"), false, true), '__try0);
        Ok::<_, &'static str>((r#str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            r#str = __try0_o0;
        }
        Err(_) => {
            r#str = literal!("");
        }
    }
    r#str
}

fn showCacheInfo(mut inMsg: &ArcStr, mut inPath: metamodelica::Ref<Absyn::Path>) -> Result<()> {
    if Flags::isSet(Flags::SHOW_INST_CACHE_INFO.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*inMsg);
            __mm_s.push_str(&*AbsynUtil::pathString(inPath, literal!("."), true, false)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn instFunctionAnnotations(
    mut comments: &metamodelica::List<metamodelica::Ref<SCode::Comment>>,
    mut state: &ClassInf::State,
) -> DAE::DAElist {
    let mut dae: DAE::DAElist = DAE::emptyDae().clone();
    let mut comment: Option<ArcStr> = None;
    let mut r#mod: metamodelica::Ref<SCode::Mod> = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
    let mut mod2: metamodelica::Ref<SCode::Mod>;
    if !(ClassInfUtil::isFunction(state)) {
        return dae;
    }
    for mut cmt in &**comments {
        if (comment).is_none() {
            comment = cmt.comment.clone();
        }
        r#mod = (::match_deref::match_deref! { match &(cmt.clone()) {
            Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: __esc_mod2 }), .. } => {
                mod2 = (*__esc_mod2).clone();
                SCodeUtil::mergeModifiers(mod2.clone(), r#mod)
            },
            _ => r#mod,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    dae = (match &*r#mod {
        SCode::Mod::NOMOD { .. } => {
            if ((comment).is_none()) {
                dae
            } else {
                DAE::DAElist {
                    elementLst: list![metamodelica::Ref::new(DAE::Element::COMMENT {
                        cmt: metamodelica::Ref::new(SCode::Comment {
                            annotation_: None,
                            comment: comment
                        })
                    })],
                }
            }
        }
        _ => DAE::DAElist {
            elementLst: list![metamodelica::Ref::new(DAE::Element::COMMENT {
                cmt: metamodelica::Ref::new(SCode::Comment {
                    annotation_: Some(metamodelica::Ref::new(SCode::Annotation { modification: r#mod })),
                    comment: comment
                })
            })],
        },
    });
    dae
}

pub(crate) fn instClassType(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut classElem: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::Ref<DAE::Type>)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ty: metamodelica::Ref<DAE::Type>;
    (cache, env, _, _, _, _, ty, _, _, _) = instClass(
        cache,
        env,
        InnerOuter::emptyInstHierarchy().clone(),
        UnitAbsyn::noStore().clone(),
        openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        openmodelica_frontend_types::DAE::Prefix::NOPRE,
        classElem,
        metamodelica::nil(),
        true,
        openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL,
        ConnectionGraph::EMPTY().clone(),
        &(Connect::emptySet().clone()),
    )?;
    Ok((cache, env, ty))
}

fn checkInstanceRestriction(
    mut cdef: &metamodelica::Ref<SCode::Element>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut relaxedFrontEnd: bool,
) -> Result<()> {
    if !(relaxedFrontEnd) && (SCodeUtil::isFunction(cdef) || SCodeUtil::isPackage(cdef)) {
        Error::addSourceMessage(
            &(Error::INST_INVALID_RESTRICTION.clone()),
            list![
                AbsynUtil::pathString(path, literal!("."), true, false)?,
                SCodeDump::restrString(&(SCodeUtil::getClassRestriction(cdef)?))?
            ],
            &(SCodeUtil::elementInfo(cdef)),
        )?;
        return Err("fail");
    }
    Ok(())
}
