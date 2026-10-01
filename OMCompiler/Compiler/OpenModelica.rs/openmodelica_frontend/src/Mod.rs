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

use crate::Ceval;
use crate::FGraph;
use crate::FNode;
use crate::InnerOuter;
use crate::Inst;
use crate::InstUtil;
use crate::Lookup;
use crate::PrefixUtil;
use crate::Static;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::SCodeInstUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

/// Used to know where a modifier came from, for error reporting.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ModScope {
    COMPONENT { name: ArcStr },
    EXTENDS { path: metamodelica::Ref<Absyn::Path> },
    DERIVED { path: metamodelica::Ref<Absyn::Path> },
}
impl metamodelica::gc::MMTrace for ModScope {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ModScope::COMPONENT { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            ModScope::EXTENDS { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            ModScope::DERIVED { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::ModScope::{COMPONENT, DERIVED, EXTENDS};

/// used for error reporting
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum FullMod {
    /// the fully qualified cref and the mod, only used for redeclare
    MOD {
        cref: metamodelica::Ref<DAE::ComponentRef>,
        r#mod: metamodelica::Ref<DAE::Mod>,
    },
    /// the fully qualified cref and the sub mod for all other mods
    SUB_MOD {
        cref: metamodelica::Ref<DAE::ComponentRef>,
        subMod: metamodelica::Ref<DAE::SubMod>,
    },
}
impl metamodelica::gc::MMTrace for FullMod {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FullMod::MOD { cref, r#mod } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#mod, __mmv)?;
                Ok(())
            }
            FullMod::SUB_MOD { cref, subMod } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subMod, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for FullMod {
    fn default() -> Self {
        Self::MOD {
            cref: Default::default(),
            r#mod: Default::default(),
        }
    }
}
pub(crate) use self::FullMod::{MOD, SUB_MOD};

pub type SubMod = metamodelica::Ref<DAE::SubMod>;

pub type EqMod = DAE::EqMod;

pub fn elabMod(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inBoolean: bool,
    mut inModScope: ModScope,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Mod>)> {
    let mut outCache: FCore::Cache;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    r#mod = SCodeInstUtil::expandEnumerationMod(inMod)?;
    (outCache, outMod) = (::match_deref::match_deref! { match &(r#mod) {
        Deref @ SCode::Mod::NOMOD { .. } => {
            let mut cache = inCache;
            (cache, openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
        },
        Deref @ SCode::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: None, info, .. } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut pre = inPrefix;
            let mut r#impl = inBoolean;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            (cache, subs_1) = elabSubmods(cache, &env, &ih, &pre, metamodelica::AsArg::as_arg(&subs), r#impl, inModScope, metamodelica::AsArg::as_arg(&info))?;
            (cache, metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1, binding: None, info: info.clone() }))
        },
        Deref @ SCode::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: Some(e), info, .. } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut pre = inPrefix;
            let mut r#impl = inBoolean;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut e_val: Option<metamodelica::Ref<Values::Value>>;
            (cache, subs_1) = elabSubmods(cache, &env, &ih, &pre, metamodelica::AsArg::as_arg(&subs), r#impl, inModScope, metamodelica::AsArg::as_arg(&info))?;
            (cache, e_1, prop) = Static::elabExp(cache, env.clone(), e.clone(), r#impl, Config::splitArrays()?, pre.clone(), info.clone())?;
            (e_1, prop) = Expression::tupleHead(e_1, prop)?;
            (cache, e_1, prop) = Ceval::cevalIfConstant(cache, env.clone(), e_1, prop, r#impl, info.clone())?;
            (e_val, cache) = elabModValue(cache, env.clone(), e_1.clone(), prop.clone(), r#impl, info.clone())?;
            (cache, e_2) = PrefixUtil::prefixExp(cache, &env, &ih, e_1, &pre)?;
            (cache, metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1, binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_2, modifierAsValue: e_val, properties: prop, modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() }))
        },
        Deref @ SCode::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: Some(e), info, .. } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut pre = inPrefix;
            let mut r#impl = inBoolean;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            (cache, subs_1) = elabSubmods(cache, &env, &ih, &pre, metamodelica::AsArg::as_arg(&subs), r#impl, inModScope, metamodelica::AsArg::as_arg(&info))?;
            (cache, metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1, binding: Some(DAE::EqMod::UNTYPED { exp: e.clone() }), info: info.clone() }))
        },
        Deref @ SCode::Mod::REDECL { finalPrefix, eachPrefix: each_, element: elem } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut pre = inPrefix;
            let mut r#impl = inBoolean;
            let mut info = inInfo;
            let mut dm: metamodelica::Ref<DAE::Mod>;
            let mut elem = (*elem).clone();
            (elem, dm) = elabModRedeclareElement(cache.clone(), env, ih, pre, finalPrefix.clone(), elem.clone(), r#impl, inModScope, info)?;
            (cache, metamodelica::Ref::new(DAE::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), element: elem.clone(), r#mod: dm }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outMod))
}

pub(crate) fn isInvariantMod(mut r#mod: metamodelica::Ref<SCode::Mod>) -> Result<bool> {
    let mut b: bool;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut mods: metamodelica::Ref<SCode::Mod>;
    b = (::match_deref::match_deref! { match &(&*r#mod) {
        Deref @ SCode::Mod::NOMOD { .. } => true,
        Deref @ SCode::Mod::MOD { binding: None, subModLst: __mod_subModLst, .. } => {
            b = (::match_deref::match_deref! { match &(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            (_, b) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isInvariantExpNoTraverse, metamodelica::Ref<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, bool) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)> + 'static>), true)?;
            b
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(b) {
                return Ok(b);
            }
            for mut sm in &*__mod_subModLst.clone() {
                if !(isInvariantMod(sm.r#mod.clone())?) {
                    b = false;
                    return Ok(b);
                }
            }
            true
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { modifications: __esc_mods, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::FULLYQUALIFIED { .. }, arrayDim: None }, .. }, .. } => {
            mods = (*__esc_mods).clone();
            isInvariantMod(mods.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isInvariantDAEMod(mut r#mod: &metamodelica::Ref<DAE::Mod>) -> Result<bool> {
    let mut b: bool;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut mods: metamodelica::Ref<SCode::Mod>;
    b = (::match_deref::match_deref! { match r#mod {
        Deref @ DAE::Mod::NOMOD { .. } => true,
        Deref @ DAE::Mod::MOD { binding: None, subModLst: __mod_subModLst, .. } => {
            b = (match var_field!((**r#mod).binding, DAE::Mod::MOD).clone() {
        Some(DAE::EqMod::TYPED { modifierAsExp: ref __esc_e, .. }) => {
            e = __esc_e.clone();
            (_, b) = Expression::traverseExpBottomUp(e.clone(), &fnptr!(Expression::isInvariantExpNoTraverse, metamodelica::Ref<DAE::Exp>, bool), true)?;
            b
        },
        Some(DAE::EqMod::UNTYPED { exp: mut __esc_exp }) => {
            exp = __esc_exp.clone();
            (_, b) = AbsynUtil::traverseExp(exp.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isInvariantExpNoTraverse, metamodelica::Ref<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, bool) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)> + 'static>), true)?;
            b
        },
        _ => true,
    });
            if !(b) {
                return Ok(b);
            }
            for mut sm in &*__mod_subModLst.clone() {
                if !(isInvariantDAEMod(&sm.r#mod)?) {
                    b = false;
                    return Ok(b);
                }
            }
            true
        },
        Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { modifications: __esc_mods, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::FULLYQUALIFIED { .. }, arrayDim: None }, .. }, .. } => {
            mods = (*__esc_mods).clone();
            isInvariantMod(mods.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn elabModForBasicType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inBoolean: bool,
    mut inModScope: ModScope,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Mod>)> {
    let mut outCache: FCore::Cache;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    checkIfModsAreBasicTypeMods(&inMod)?;
    (outCache, outMod) = elabMod(inCache, inEnv, inIH, inPrefix, inMod, inBoolean, inModScope, info)?;
    Ok((outCache, outMod))
}

fn checkIfModsAreBasicTypeMods(mut r#mod: &metamodelica::Ref<SCode::Mod>) -> Result<()> {
    let () = (match &**r#mod {
        SCode::Mod::NOMOD { .. } => (),
        SCode::Mod::MOD { subModLst: subs, .. } => {
            checkIfSubmodsAreBasicTypeMods(subs)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn checkIfSubmodsAreBasicTypeMods(mut inSubs: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inSubs {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident, .. }, tail: subs } => {
            let true = (ClassInfUtil::isBasicTypeComponentName(ident.clone())) else { return Err("pattern mismatch") };
            checkIfSubmodsAreBasicTypeMods(subs)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn elabModRedeclareElement(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut finalPrefix: SCode::Final,
    mut inElt: metamodelica::Ref<SCode::Element>,
    mut r#impl: bool,
    mut inModScope: ModScope,
    mut info: SourceInfo,
) -> Result<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    (outElement, outMod) = 'mc: {
        let __mc_input = inElt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: cn, prefixes: prefixes @ Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: redecl, finalPrefix: fi, innerOuter: io, replaceablePrefix: repl }, encapsulatedPrefix: enc, partialPrefix: p, restriction: restr, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: tp, modifications: r#mod, attributes: attr1 }, cmt, info: i } => {
                    let mut cache: FCore::Cache;
                    let mut tp1: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut emod: metamodelica::Ref<DAE::Mod>;
                    let mut r#mod = (*r#mod).clone();
                    r#mod = SCodeUtil::mergeModifiers(r#mod.clone(), SCodeUtil::getConstrainedByModifiers(metamodelica::AsArg::as_arg(&prefixes)));
                    (cache, emod) = elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#mod.clone(), r#impl, inModScope.clone(), info.clone())?;
                    (_, tp1) = elabModQualifyTypespec(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#impl, info.clone(), cn.clone(), metamodelica::AsArg::as_arg(&tp))?;
                    r#mod = unelabMod(emod.clone())?;
                    Ok((metamodelica::Ref::new(SCode::Element::CLASS { name: cn.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: fi.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), encapsulatedPrefix: enc.clone(), partialPrefix: p.clone(), restriction: restr.clone(), classDef: metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: tp1.clone(), modifications: r#mod.clone(), attributes: attr1.clone() }), cmt: cmt.clone(), info: i.clone() }), emod.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, .. } => {
                    Ok((inElt.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
                    Ok((inElt.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { name: compname, prefixes: prefixes @ Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: redecl, finalPrefix: fi, innerOuter: io, replaceablePrefix: repl }, attributes: attr, typeSpec: tp, modifications: r#mod, comment: cmt, condition: cond, info: i } => {
                    let mut cache: FCore::Cache;
                    let mut tp1: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut emod: metamodelica::Ref<DAE::Mod>;
                    let mut r#mod = (*r#mod).clone();
                    r#mod = SCodeUtil::mergeModifiers(r#mod.clone(), SCodeUtil::getConstrainedByModifiers(metamodelica::AsArg::as_arg(&prefixes)));
                    (cache, emod) = elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#mod.clone(), r#impl, inModScope.clone(), info.clone())?;
                    (_, tp1) = elabModQualifyTypespec(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#impl, info.clone(), compname.clone(), metamodelica::AsArg::as_arg(&tp))?;
                    r#mod = unelabMod(emod.clone())?;
                    Ok((metamodelica::Ref::new(SCode::Element::COMPONENT { name: compname.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: fi.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: attr.clone(), typeSpec: tp1.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: i.clone() }), emod.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                element => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unhandled element redeclare (we keep it as it is!): ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(element.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok((element.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outElement, outMod))
}

fn elabModQualifyTypespec(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut r#impl: bool,
    mut info: SourceInfo,
    mut name: ArcStr,
    mut tp: &metamodelica::Ref<Absyn::TypeSpec>,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::TypeSpec>)> {
    let mut outCache: FCore::Cache;
    let mut outTp: metamodelica::Ref<Absyn::TypeSpec>;
    (outCache, outTp) = (::match_deref::match_deref! { match tp {
        Deref @ Absyn::TypeSpec::TPATH { path: p, arrayDim: None } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut p1: metamodelica::Ref<Absyn::Path>;
            (cache, p1) = Inst::makeFullyQualified(cache, env, p.clone())?;
            (cache, metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: p1, arrayDim: None }))
        },
        Deref @ Absyn::TypeSpec::TPATH { path: p, arrayDim: Some(dims) } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut pre = inPrefix;
            let mut p1: metamodelica::Ref<Absyn::Path>;
            let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
            let mut edims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut dims = (*dims).clone();
            cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name, subscripts: metamodelica::nil() });
            (cache, edims) = InstUtil::elabArraydim(cache, env.clone(), cref, p.clone(), dims.clone(), None, r#impl, true, false, pre.clone(), info, metamodelica::nil())?;
            (cache, edims) = PrefixUtil::prefixDimensions(&cache, &env, &ih, &pre, &edims);
            dims = List::map(edims, &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::unelabDimension(&__a0))?;
            (cache, p1) = Inst::makeFullyQualified(cache, env, p.clone())?;
            (cache, metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: p1, arrayDim: Some(dims.clone()) }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outTp))
}

fn elabModValue(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProp: DAE::Properties,
    mut inImpl: bool,
    mut inInfo: SourceInfo,
) -> Result<(Option<metamodelica::Ref<Values::Value>>, FCore::Cache)> {
    let mut outValue: Option<metamodelica::Ref<Values::Value>> = None;
    let mut outCache: FCore::Cache = inCache.clone();
    let mut err_count: i32;
    let mut msg: Absyn::Msg;
    let mut c: DAE::Const;
    let mut v: metamodelica::Ref<Values::Value>;
    c = Types::propAllConst(inProp)?;
    if !(Types::constIsVariable(c)) {
        msg = AbsynUtil::optMsg(Types::constIsConst(c) && !(inImpl), inInfo);
        err_count = Error::getNumErrorMessages();
        if '__try0: {
            (_, v) = unwrap_break_err!(Ceval::ceval(inCache.clone(), inEnv.clone(), inExp.clone(), false, msg.clone(), 0), '__try0);
            if ValuesUtil::isRecord(&v) {
                v = unwrap_break_err!(ValuesUtil::typeConvertRecord(v.clone(), &(unwrap_break_err!(Expression::r#typeof(inExp.clone()), '__try0))), '__try0);
            }
            outValue = Some(v.clone());
            Ok::<(), &'static str>(())
        }.is_err() {
            if err_count != Error::getNumErrorMessages() && !(Expression::containsAnyCall(inExp.clone())?) {
                return Err("fail");
            }
        }
    }
    Ok((outValue, outCache))
}

pub(crate) fn unelabMod(mut inMod: metamodelica::Ref<DAE::Mod>) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = 'mc: {
        let __mc_input = inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD { .. } => {
                    Ok(openmodelica_frontend_types::SCode::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: None, info } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    subs_1 = unelabSubmods(subs.clone())?;
                    Ok(metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: None, comment: None, info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: Some(DAE::EqMod::UNTYPED { exp: e }), info } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    subs_1 = unelabSubmods(subs.clone())?;
                    Ok(metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(e.clone()), comment: None, info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: Some(DAE::EqMod::TYPED { modifierAsValue: Some(v), .. }), info } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    let mut e_1: metamodelica::Ref<Absyn::Exp>;
                    subs_1 = unelabSubmods(subs.clone())?;
                    e_1 = Expression::unelabExp(&(ValuesUtil::valueExp(v.clone(), None)?))?;
                    Ok(metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(e_1.clone()), comment: None, info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: Some(DAE::EqMod::TYPED { modifierAsExp: _, modifierAsValue: _, properties: _, modifierAsAbsynExp: absynExp, .. }), info } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    let mut e_1: metamodelica::Ref<Absyn::Exp>;
                    subs_1 = unelabSubmods(subs.clone())?;
                    e_1 = absynExp.clone();
                    Ok(metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(e_1.clone()), comment: None, info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { finalPrefix, eachPrefix: each_, element: elem, .. } => {
                    Ok(metamodelica::Ref::new(SCode::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), element: elem.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                r#mod => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mod.elabUntypedMod failed: ")); __mm_s.push_str(&*printModStr(metamodelica::AsArg::as_arg(&r#mod))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

fn unelabSubmods(
    mut inTypesSubModLst: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSCodeSubModLst: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    outSCodeSubModLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
        for mut x in (inTypesSubModLst).into_iter().cloned() {
            let __x = (match &*x.clone() {
                DAE::SubMod { ident: i, r#mod: m } => {
                    let mut m_1: metamodelica::Ref<SCode::Mod>;
                    m_1 = unelabMod(m.clone())?;
                    metamodelica::Ref::new(SCode::SubMod {
                        ident: i.clone(),
                        r#mod: m_1.clone(),
                    })
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outSCodeSubModLst)
}

fn unelabSubscript(
    mut inIntegerLst: metamodelica::List<i32>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Subscript>> {
    let mut outSCodeSubscriptLst: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outSCodeSubscriptLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
        for mut i in (inIntegerLst).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
                subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i.clone() }),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outSCodeSubscriptLst
}

pub(crate) fn updateMod(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inBoolean: bool,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Mod>)> {
    let mut outCache: FCore::Cache;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    (outCache, outMod) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inPrefix, inMod, inBoolean);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, Deref @ DAE::Mod::NOMOD { .. }, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, m @ Deref @ DAE::Mod::REDECL { .. }, _) => {
                    Ok((cache.clone(), m.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: subs, binding: Some(DAE::EqMod::UNTYPED { exp: e }), info }, r#impl) => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut e_val: Option<metamodelica::Ref<Values::Value>>;
                    let mut cache = (*cache).clone();
                    (cache, subs_1) = updateSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), metamodelica::AsArg::as_arg(&info))?;
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (e_val, cache) = elabModValue(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), e_1.clone(), metamodelica::AsArg::as_arg(&pre))?;
                    if Flags::isSet(Flags::UPDMOD.clone())? {
                        Debug::trace(literal!("Updated mod: "))?;
                        Debug::traceln(printModStr(&(metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_2.clone(), modifierAsValue: None, properties: prop.clone(), modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() })))?)?;
                    }
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_2.clone(), modifierAsValue: e_val.clone(), properties: prop.clone(), modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: subs, binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_1, modifierAsValue: e_val, properties: p, modifierAsAbsynExp: e, .. }), info }, r#impl) => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
                    let mut cache = (*cache).clone();
                    (cache, subs_1) = updateSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_1.clone(), modifierAsValue: e_val.clone(), properties: p.clone(), modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: subs, binding: None, info }, r#impl) => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
                    let mut cache = (*cache).clone();
                    (cache, subs_1) = updateSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: None, info: info.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, m, _) => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    r#str = printModStr(metamodelica::AsArg::as_arg(&m))?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Mod.updateMod failed mod: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outMod))
}

fn updateSubmods(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inTypesSubModLst: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inBoolean: bool,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::SubMod>>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outTypesSubModLst: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    outTypesSubModLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
        for mut x in (inTypesSubModLst).into_iter().cloned() {
            let __x = (match &*x.clone() {
                DAE::SubMod { ident: i, r#mod: m } => {
                    let mut m_1: metamodelica::Ref<DAE::Mod>;
                    (outCache, m_1) = updateMod(
                        outCache.clone(),
                        inEnv.clone(),
                        inIH.clone(),
                        inPrefix.clone(),
                        m.clone(),
                        inBoolean,
                        info,
                    )?;
                    metamodelica::Ref::new(DAE::SubMod {
                        ident: i.clone(),
                        r#mod: m_1.clone(),
                    })
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((outCache, outTypesSubModLst))
}

pub(crate) fn elabUntypedMod(
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inModScope: ModScope,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = &*inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::NOMOD { .. } => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: None, info, .. } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
                    subs_1 = elabUntypedSubmods(metamodelica::AsArg::as_arg(&subs), inModScope.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: None, info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix, eachPrefix: each_, subModLst: subs, binding: Some(e), info, .. } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
                    subs_1 = elabUntypedSubmods(metamodelica::AsArg::as_arg(&subs), inModScope.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::UNTYPED { exp: e.clone() }), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { finalPrefix, eachPrefix: each_, element: elem } => {
                    Ok(metamodelica::Ref::new(DAE::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), element: elem.clone(), r#mod: openmodelica_frontend_types::DAE::Mod::interned_NOMOD() }))
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
                    metamodelica::print(literal!("- elab_untyped_mod "));
                    s = SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?;
                    metamodelica::print(s.clone());
                    metamodelica::print(literal!(" failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

fn elabSubmods(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &DAE::Prefix,
    mut inSCodeSubModLst: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inBoolean: bool,
    mut inModScope: ModScope,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::SubMod>>)> {
    let mut outCache: FCore::Cache;
    let mut outTypesSubModLst: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    submods = compactSubMods(inSCodeSubModLst, inModScope)?;
    (outCache, outTypesSubModLst) = elabSubmods2(
        inCache,
        inEnv,
        inIH,
        inPrefix,
        submods,
        inBoolean,
        info,
        metamodelica::nil(),
    )?;
    Ok((outCache, outTypesSubModLst))
}

fn elabSubmods2<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: &'__b FCore::Graph,
    mut inIH: &'__b metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &'__b DAE::Prefix,
    mut inSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inImpl: bool,
    mut inInfo: &'__b SourceInfo,
    mut inAccumMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::SubMod>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache.clone(), inSubMods)) {
            (cache, Deref @ metamodelica::ListNode::Cons { head: smod, tail: rest_smods }) => {
                let mut dmod: metamodelica::Ref<DAE::SubMod>;
                let mut accum_mods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
                let mut cache = (*cache).clone();
                (cache, dmod) = elabSubmod(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), metamodelica::AsArg::as_arg(&smod), inImpl, inInfo.clone())?;
                { (inCache, inEnv, inIH, inPrefix, inSubMods, inImpl, inInfo, inAccumMods) = (cache.clone(), inEnv, inIH, inPrefix, rest_smods.clone(), inImpl, inInfo, metamodelica::cons(dmod, inAccumMods)); continue '__tco; }
            },
            _ => {
                return Ok((inCache, inAccumMods.reverse()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn compactSubMods(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inModScope: ModScope,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    submods = List::fold2(
        inSubMods,
        &move |__a0: metamodelica::Ref<SCode::SubMod>,
               __a1: ModScope,
               __a2: metamodelica::List<ArcStr>,
               __a3: metamodelica::List<metamodelica::Ref<SCode::SubMod>>| {
            compactSubMod(__a0, &__a1, &__a2, __a3)
        },
        inModScope,
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    outSubMods = submods.reverse();
    Ok(outSubMods)
}

fn compactSubMod(
    mut inSubMod: metamodelica::Ref<SCode::SubMod>,
    mut inModScope: &ModScope,
    mut inName: &metamodelica::List<ArcStr>,
    mut inAccumMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut found: bool;
    (submods, found) = List::findMap(
        inAccumMods,
        &({
            let __pe_b1 = inSubMod.clone();
            let __pe_b2 = inModScope.clone();
            let __pe_b3 = inName.clone();
            move |__pe_a0| compactSubMod2(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    outSubMods = List::consOnTrue(!(found), inSubMod, submods);
    Ok(outSubMods)
}

fn compactSubMod2(
    mut inExistingMod: metamodelica::Ref<SCode::SubMod>,
    mut inNewMod: &metamodelica::Ref<SCode::SubMod>,
    mut inModScope: ModScope,
    mut inName: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::Ref<SCode::SubMod>, bool)> {
    let mut outMod: metamodelica::Ref<SCode::SubMod>;
    let mut outFound: bool;
    (outMod, outFound) = (::match_deref::match_deref! { match &((inExistingMod.clone(), inNewMod.clone())) {
        (Deref @ SCode::SubMod { ident: name1, .. }, Deref @ SCode::SubMod { ident: name2, .. }) if (!(stringEqual(&name1, &name2))) => {
            (inExistingMod, false)
        },
        (Deref @ SCode::SubMod { ident: name1, .. }, _) => {
            let mut submod: metamodelica::Ref<SCode::SubMod>;
            submod = mergeSubModsInSameScope(&inExistingMod, inNewMod, metamodelica::cons(name1.clone(), inName), inModScope)?;
            (submod, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outMod, outFound))
}

fn mergeSubModsInSameScope(
    mut inMod1: &metamodelica::Ref<SCode::SubMod>,
    mut inMod2: &metamodelica::Ref<SCode::SubMod>,
    mut inElementName: metamodelica::List<ArcStr>,
    mut inModScope: ModScope,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut outMod: metamodelica::Ref<SCode::SubMod>;
    let mut scope: ArcStr;
    let mut name: ArcStr;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    let mut mod1: metamodelica::Ref<SCode::Mod> = inMod1.r#mod.clone();
    let mut mod2: metamodelica::Ref<SCode::Mod> = inMod2.r#mod.clone();
    outMod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ SCode::Mod::MOD { .. }, Deref @ SCode::Mod::MOD { binding: None, .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD), &move |__a0: metamodelica::Ref<SCode::SubMod>, __a1: ModScope, __a2: metamodelica::List<ArcStr>, __a3: metamodelica::List<metamodelica::Ref<SCode::SubMod>>| compactSubMod(__a0, &__a1, &__a2, __a3), inModScope, inElementName, var_field!((*mod2).subModLst, SCode::Mod::MOD).clone())?;
            metamodelica::Ref::new(SCode::SubMod { ident: inMod1.ident.clone(), r#mod: metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod1).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod1).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods, binding: var_field!((*mod1).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod1).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod1).info, SCode::Mod::MOD).clone() }) })
        },
        (Deref @ SCode::Mod::MOD { binding: None, .. }, Deref @ SCode::Mod::MOD { .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD), &move |__a0: metamodelica::Ref<SCode::SubMod>, __a1: ModScope, __a2: metamodelica::List<ArcStr>, __a3: metamodelica::List<metamodelica::Ref<SCode::SubMod>>| compactSubMod(__a0, &__a1, &__a2, __a3), inModScope, inElementName, var_field!((*mod2).subModLst, SCode::Mod::MOD).clone())?;
            metamodelica::Ref::new(SCode::SubMod { ident: inMod2.ident.clone(), r#mod: metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod2).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod2).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods, binding: var_field!((*mod2).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod2).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod2).info, SCode::Mod::MOD).clone() }) })
        },
        _ => {
            info1 = SCodeUtil::getModifierInfo(&mod1);
            info2 = SCodeUtil::getModifierInfo(&mod2);
            scope = printModScope(&inModScope)?;
            name = stringDelimitList(inElementName.reverse(), literal!("."));
            Error::addMultiSourceMessage(&(Error::DUPLICATE_MODIFICATIONS.clone()), &(list![name, scope]), &(list![info2, info1]))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn printModScope(mut inModScope: &ModScope) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inModScope.clone() {
        ModScope::COMPONENT { name: mut name } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*name);
            ArcStr::from(__mm_s)
        }
        ModScope::EXTENDS { path: mut path } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("extends "));
            __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
            ArcStr::from(__mm_s)
        }
        ModScope::DERIVED { path: mut path } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("inherited class "));
            __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
            ArcStr::from(__mm_s)
        }
    });
    Ok(outString)
}

fn elabSubmod(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSubMod: &metamodelica::Ref<SCode::SubMod>,
    mut inBoolean: bool,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::SubMod>)> {
    let mut outCache: FCore::Cache;
    let mut outSubMod: metamodelica::Ref<DAE::SubMod>;
    let mut smod: metamodelica::Ref<SCode::Mod>;
    let mut dmod: metamodelica::Ref<DAE::Mod>;
    let mut i: ArcStr;
    let __arc2 = &(*inSubMod);
    let SCode::NAMEMOD {
        ident: __pa0,
        r#mod: __pa1,
    } = &**__arc2;
    i = metamodelica::Own::own(__pa0);
    smod = metamodelica::Own::own(__pa1);
    (outCache, dmod) = elabMod(
        inCache,
        inEnv,
        inIH,
        inPrefix,
        smod,
        inBoolean,
        ModScope::COMPONENT { name: i.clone() },
        info,
    )?;
    outSubMod = metamodelica::Ref::new(DAE::SubMod { ident: i, r#mod: dmod });
    Ok((outCache, outSubMod))
}

fn elabUntypedSubmods(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inModScope: ModScope,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    submods = compactSubMods(inSubMods, inModScope)?;
    outSubMods = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
        for mut m in (submods.reverse()).into_iter().cloned() {
            let __x = elabUntypedSubmod(&(m.clone()))?;
            __acc = __x.append(&__acc);
        }
        __acc
    });
    Ok(outSubMods)
}

fn elabUntypedSubmod(
    mut inSubMod: &metamodelica::Ref<SCode::SubMod>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::SubMod>>> {
    let mut outTypesSubModLst: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    outTypesSubModLst = (match &**inSubMod {
        SCode::SubMod { ident: i, r#mod: m } => {
            let mut m_1: metamodelica::Ref<DAE::Mod>;
            m_1 = elabUntypedMod(m.clone(), ModScope::COMPONENT { name: literal!("") })?;
            list![metamodelica::Ref::new(DAE::SubMod {
                ident: i.clone(),
                r#mod: m_1
            })]
        }
    });
    Ok(outTypesSubModLst)
}

// - Lookup
pub(crate) fn lookupModificationP(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = (inMod, &**inPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ Absyn::Path::IDENT { name: n }) => {
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    r#mod = lookupCompModification(metamodelica::AsArg::as_arg(&m), n.clone())?;
                    Ok(r#mod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ Absyn::Path::FULLYQUALIFIED { path: p }) => {
                    Ok(lookupModificationP(m.clone(), metamodelica::AsArg::as_arg(&p))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ Absyn::Path::QUALIFIED { name: n, path: p }) => {
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    r#mod = lookupCompModification(metamodelica::AsArg::as_arg(&m), n.clone())?;
                    mod_1 = lookupModificationP(r#mod.clone(), metamodelica::AsArg::as_arg(&p))?;
                    Ok(mod_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf(literal!("- Mod.lookupModificationP failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

pub(crate) fn lookupCompModification(
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (match &**inMod {
        DAE::Mod::MOD {
            finalPrefix: f,
            eachPrefix: e,
            subModLst: subs,
            binding: eqMod,
            info,
        } => {
            let mut n = inIdent;
            let mut mod1: metamodelica::Ref<DAE::Mod>;
            let mut mod2: metamodelica::Ref<DAE::Mod>;
            mod1 = lookupCompModification2(subs, n.clone());
            mod2 = lookupComplexCompModification(eqMod.clone(), &n, f.clone(), e.clone(), info.clone());
            checkDuplicateModifications(mod1, mod2, n)?
        }
        _ => openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
    });
    Ok(outMod)
}

pub(crate) fn getModifs(
    mut inMods: &metamodelica::Ref<DAE::Mod>,
    mut inName: ArcStr,
    mut inSMod: &metamodelica::Ref<SCode::Mod>,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = &**inSMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    m = lookupCompModification(inMods, inName.clone())?;
                    m = mergeModifiers(inMods, m.clone(), inSMod);
                    Ok(m.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    m = mergeModifiers(inMods, openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), inSMod);
                    Ok(m.clone())
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

fn mergeModifiers(
    mut inMods: &metamodelica::Ref<DAE::Mod>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inSMod: &metamodelica::Ref<SCode::Mod>,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (match &**inSMod {
        SCode::Mod::MOD {
            finalPrefix: f,
            eachPrefix: e,
            subModLst: sl,
            binding: _,
            comment: _,
            ..
        } => {
            let mut m: metamodelica::Ref<DAE::Mod>;
            m = mergeSubMods(inMods, &inMod, f.clone(), e.clone(), sl);
            m
        }
        _ => inMod,
    });
    outMod
}

fn mergeSubMods(
    mut inMods: &metamodelica::Ref<DAE::Mod>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut f: SCode::Final,
    mut e: SCode::Each,
    mut inSMods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = &**inSMods;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inMod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: _ } }), info, .. } }, tail: rest } => {
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    m = lookupCompModification(inMods, id.clone())?;
                    m = metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: list![metamodelica::Ref::new(DAE::SubMod { ident: n.clone(), r#mod: m.clone() })], binding: None, info: info.clone() });
                    m = merge(inMod.clone(), m.clone(), literal!(""), true)?;
                    m = mergeSubMods(inMods, &m, f, e, metamodelica::AsArg::as_arg(&rest));
                    Ok(m.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    m = mergeSubMods(inMods, inMod, f, e, metamodelica::AsArg::as_arg(&rest));
                    Ok(m.clone())
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

pub(crate) fn lookupCompModificationFromEqu(
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (match &**inMod {
        DAE::Mod::NOMOD { .. } => openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        DAE::Mod::REDECL { .. } => openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        DAE::Mod::MOD {
            finalPrefix: f,
            eachPrefix: e,
            subModLst: subs,
            binding: eqMod,
            info,
        } => {
            let mut n = inIdent;
            let mut r#mod: metamodelica::Ref<DAE::Mod>;
            let mut mod1: metamodelica::Ref<DAE::Mod>;
            let mut mod2: metamodelica::Ref<DAE::Mod>;
            mod1 = lookupCompModification2(subs, n.clone());
            mod2 = lookupComplexCompModification(eqMod.clone(), &n, f.clone(), e.clone(), info.clone());
            r#mod = selectEqMod(mod1, mod2, n)?;
            r#mod
        }
    });
    Ok(outMod)
}

fn selectEqMod(
    mut subMod: metamodelica::Ref<DAE::Mod>,
    mut eqMod: metamodelica::Ref<DAE::Mod>,
    mut n: ArcStr,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    r#mod = (match &*eqMod {
        DAE::Mod::NOMOD { .. } => subMod,
        DAE::Mod::MOD {
            binding: Some(DAE::EqMod::TYPED { .. }),
            ..
        } => eqMod,
        _ => {
            r#mod = checkDuplicateModifications(subMod, eqMod, n)?;
            r#mod
        }
    });
    Ok(r#mod)
}

fn lookupComplexCompModification(
    mut inEqMod: Option<DAE::EqMod>,
    mut inName: &ArcStr,
    mut inFinal: SCode::Final,
    mut inEach: SCode::Each,
    mut inInfo: SourceInfo,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = openmodelica_frontend_types::DAE::Mod::interned_NOMOD();
    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut names: metamodelica::List<ArcStr>;
    let mut v: metamodelica::Ref<Values::Value>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut ae: metamodelica::Ref<Absyn::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut eq_mod: DAE::EqMod;
    let mut info: SourceInfo;
    if '__try0: {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inEqMod.clone()) {
            Some(DAE::EqMod::TYPED { modifierAsValue: Some(Deref @ Values::Value::RECORD { orderd: __pa1, comp: __pa2, index: (-1), .. }), info: __pa3, .. }) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        values = metamodelica::Own::own(__pa1);
        names = metamodelica::Own::own(__pa2);
        info = metamodelica::Own::own(__pa3);
        for mut name in &*names {
            let mut name = name.clone();
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(values.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            v = metamodelica::Own::own(__pa4);
            values = metamodelica::Own::own(__pa5);
            if metamodelica::stringEq(&name, &inName) {
                e = unwrap_break_err!(ValuesUtil::valueExp(v.clone(), None), '__try0);
                ae = unwrap_break_err!(Expression::unelabExp(&e), '__try0);
                ty = unwrap_break_err!(Types::complicateType(unwrap_break_err!(Expression::r#typeof(e.clone()), '__try0)), '__try0);
                eq_mod = DAE::EqMod::TYPED { modifierAsExp: e.clone(), modifierAsValue: Some(v.clone()), properties: DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }, modifierAsAbsynExp: ae.clone(), info: info.clone() };
                outMod = metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: inFinal, eachPrefix: inEach, subModLst: metamodelica::nil(), binding: Some(eq_mod.clone()), info: inInfo.clone() });
                break;
            }
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    outMod
}

fn checkDuplicateModifications(
    mut mod1: metamodelica::Ref<DAE::Mod>,
    mut mod2: metamodelica::Ref<DAE::Mod>,
    mut n: ArcStr,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ DAE::Mod::NOMOD { .. }, _) => {
            mod2
        },
        (_, Deref @ DAE::Mod::NOMOD { .. }) => {
            mod1
        },
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            mergeRedeclareWithBinding(mod1, mod2)?
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::REDECL { .. }) => {
            mergeRedeclareWithBinding(mod2, mod1)?
        },
        (Deref @ DAE::Mod::MOD { binding: None, .. }, Deref @ DAE::Mod::MOD { .. }) => {
            let mut submods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            submods = checkDuplicateModifications2(var_field!((*mod1).subModLst, DAE::Mod::MOD).clone(), var_field!((*mod2).subModLst, DAE::Mod::MOD).clone(), n)?;
            metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: var_field!((*mod2).finalPrefix, DAE::Mod::MOD).clone(), eachPrefix: var_field!((*mod2).eachPrefix, DAE::Mod::MOD).clone(), subModLst: submods, binding: var_field!((*mod2).binding, DAE::Mod::MOD).clone(), info: var_field!((*mod2).info, DAE::Mod::MOD).clone() })
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { binding: None, .. }) => {
            let mut submods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            submods = checkDuplicateModifications2(var_field!((*mod1).subModLst, DAE::Mod::MOD).clone(), var_field!((*mod2).subModLst, DAE::Mod::MOD).clone(), n)?;
            metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: var_field!((*mod1).finalPrefix, DAE::Mod::MOD).clone(), eachPrefix: var_field!((*mod1).eachPrefix, DAE::Mod::MOD).clone(), subModLst: submods, binding: var_field!((*mod1).binding, DAE::Mod::MOD).clone(), info: var_field!((*mod1).info, DAE::Mod::MOD).clone() })
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            Error::addMultiSourceMessage(&(Error::DUPLICATE_MODIFICATIONS.clone()), &(list![n, literal!("")]), &(list![getModInfo(&mod1), getModInfo(&mod2)]))?;
            mod2
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outMod)
}

fn checkDuplicateModifications2(
    mut inSubMods1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inSubMods2: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inName: ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    let mut submods: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = inSubMods2.clone();
    let mut osubmod: Option<metamodelica::Ref<DAE::SubMod>>;
    let mut submod: metamodelica::Ref<DAE::SubMod>;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    for mut s in &*inSubMods1 {
        (submods, osubmod) = List::deleteMemberOnTrue(
            subModName(metamodelica::AsArg::as_arg(&s)),
            submods,
            &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::SubMod>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isSubModNamed(&__a0, &__a1))
            },
        )?;
        if (osubmod).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(osubmod) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            submod = metamodelica::Own::own(__pa0);
            info1 = subModInfo(metamodelica::AsArg::as_arg(&s));
            info2 = subModInfo(&submod);
            Error::addMultiSourceMessage(
                &(Error::MULTIPLE_MODIFIER.clone()),
                &(list![inName.clone()]),
                &(list![info1, info2]),
            )?;
        }
    }
    outSubMods = listAppend(inSubMods1, inSubMods2);
    Ok(outSubMods)
}

fn mergeRedeclareWithBinding(
    mut inRedeclare: metamodelica::Ref<DAE::Mod>,
    mut inBinding: metamodelica::Ref<DAE::Mod>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = inRedeclare;
    outMod = (::match_deref::match_deref! { match &((outMod.clone(), inBinding.clone())) {
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, binding: Some(_), .. }) => {
            assign_variant_field!(outMod => DAE::Mod::REDECL; r#mod = merge(inBinding, var_field!((*outMod).r#mod, DAE::Mod::REDECL).clone(), literal!(""), true)?);
            outMod
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outMod)
}

fn modEqualNoPrefix(
    mut mod1: &metamodelica::Ref<DAE::Mod>,
    mut mod2: metamodelica::Ref<DAE::Mod>,
) -> Result<(metamodelica::Ref<DAE::Mod>, bool)> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut equal: bool;
    (outMod, equal) = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            let true = (subModsEqual(var_field!((**mod1).subModLst, DAE::Mod::MOD), var_field!((*mod2).subModLst, DAE::Mod::MOD))) else { return Err("pattern mismatch") };
            let true = (eqModEqual(var_field!((**mod1).binding, DAE::Mod::MOD).clone(), var_field!((*mod2).binding, DAE::Mod::MOD).clone())) else { return Err("pattern mismatch") };
            (mod2, true)
        },
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::REDECL { .. }) => {
            let true = (SCodeUtil::elementEqual(var_field!((**mod1).element, DAE::Mod::REDECL), var_field!((*mod2).element, DAE::Mod::REDECL))) else { return Err("pattern mismatch") };
            (mod2, true)
        },
        (Deref @ DAE::Mod::NOMOD { .. }, Deref @ DAE::Mod::NOMOD { .. }) => (openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), true),
        _ => (mod2, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outMod, equal))
}

fn lookupNamedSubMod(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<DAE::SubMod>> {
    let mut outSubMod: metamodelica::Ref<DAE::SubMod>;
    outSubMod = List::getMemberOnTrue(inIdent, inSubMods, &move |__a0: ArcStr,
                                                                 __a1: metamodelica::Ref<DAE::SubMod>|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(isSubModNamed(&__a0, &__a1))
    })?;
    Ok(outSubMod)
}

fn isSubModNamed(mut inIdent: &ArcStr, mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> bool {
    let mut outIsNamed: bool;
    let mut ident: ArcStr;
    let __arc1 = &(*inSubMod);
    let DAE::NAMEMOD { ident: __pa0, .. } = &**__arc1;
    ident = metamodelica::Own::own(__pa0);
    outIsNamed = stringEq(&inIdent, &ident);
    outIsNamed
}

pub(crate) fn printSubsStr(
    mut inSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut addParan: bool,
) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(
        List::map(inSubMods, &move |__a0: metamodelica::Ref<DAE::SubMod>| {
            prettyPrintSubmod(&__a0)
        })?,
        literal!(", "),
    );
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*if (addParan) { literal!("(") } else { literal!("") });
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*if (addParan) { literal!(")") } else { literal!("") });
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

fn lookupCompModification2(
    mut inSubModLst: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inIdent: ArcStr,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = &**inSubModLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let __arc1 = lookupNamedSubMod(inSubModLst, inIdent.clone())?;
                    let DAE::NAMEMOD { r#mod: __pa0, .. } = &*__arc1;
                    r#mod = metamodelica::Own::own(__pa0);
                    Ok(r#mod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
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

pub(crate) fn lookupIdxModification(
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inIndex: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = &**inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD { .. } => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { .. } => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { .. } => {
                    let mut mod1: metamodelica::Ref<DAE::Mod>;
                    let mut mod2: metamodelica::Ref<DAE::Mod>;
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
                    let mut eq: Option<DAE::EqMod>;
                    (mod1, subs) = lookupIdxModification2(var_field!((**inMod).subModLst, DAE::Mod::MOD), inIndex.clone())?;
                    mod2 = metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: var_field!((**inMod).finalPrefix, DAE::Mod::MOD).clone(), eachPrefix: var_field!((**inMod).eachPrefix, DAE::Mod::MOD).clone(), subModLst: subs.clone(), binding: None, info: var_field!((**inMod).info, DAE::Mod::MOD).clone() });
                    mod2 = merge(mod2.clone(), mod1.clone(), literal!(""), true)?;
                    eq = indexEqmod(var_field!((**inMod).binding, DAE::Mod::MOD).clone(), list![inIndex.clone()], var_field!((**inMod).info, DAE::Mod::MOD))?;
                    mod1 = metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: var_field!((**inMod).eachPrefix, DAE::Mod::MOD).clone(), subModLst: metamodelica::nil(), binding: eq.clone(), info: var_field!((**inMod).info, DAE::Mod::MOD).clone() });
                    mod2 = merge(mod2.clone(), mod1.clone(), literal!(""), true)?;
                    Ok(mod2.clone())
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
                    Debug::trace(literal!("- Mod.lookupIdxModification("))?;
                    Debug::trace(printModStr(inMod)?)?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inIndex.clone())?); __mm_s.push_str(&*literal!(") failed")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

fn lookupIdxModification2(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inIndex: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
)> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = openmodelica_frontend_types::DAE::Mod::interned_NOMOD();
    let mut outSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let mut name: ArcStr;
    for mut submod in &**inSubMods {
        let __arc2 = submod.clone();
        let DAE::NAMEMOD {
            ident: __pa0,
            r#mod: __pa1,
        } = &*__arc2;
        name = metamodelica::Own::own(__pa0);
        r#mod = metamodelica::Own::own(__pa1);
        r#mod = lookupIdxModification3(r#mod, inIndex.clone())?;
        if !(isNoMod(&r#mod)) {
            outSubMods = metamodelica::cons(
                metamodelica::Ref::new(DAE::SubMod {
                    ident: name,
                    r#mod: r#mod,
                }),
                outSubMods,
            );
        }
    }
    outSubMods = outSubMods.reverse();
    Ok((outMod, outSubMods))
}

fn lookupIdxModification3(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inIndex: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (match &*inMod {
        DAE::Mod::NOMOD { .. } => openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        DAE::Mod::REDECL { .. } => inMod,
        DAE::Mod::MOD {
            eachPrefix: SCode::Each::NOT_EACH { .. },
            binding: __inMod_binding,
            finalPrefix: __inMod_finalPrefix,
            info: __inMod_info,
            subModLst: __inMod_subModLst,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            let mut eq: Option<DAE::EqMod>;
            (_, subs) = lookupIdxModification2(metamodelica::AsArg::as_arg(&__inMod_subModLst), inIndex.clone())?;
            eq = indexEqmod(
                __inMod_binding.clone(),
                list![inIndex],
                metamodelica::AsArg::as_arg(&__inMod_info),
            )?;
            metamodelica::Ref::new(DAE::Mod::MOD {
                finalPrefix: __inMod_finalPrefix.clone(),
                eachPrefix: var_field!((*inMod).eachPrefix, DAE::Mod::MOD).clone(),
                subModLst: subs,
                binding: eq,
                info: __inMod_info.clone(),
            })
        }
        DAE::Mod::MOD {
            eachPrefix: SCode::Each::EACH { .. },
            ..
        } => inMod,
        _ => return Err("match: no arm matched"),
    });
    Ok(outMod)
}

fn indexEqmod(
    mut inBinding: Option<DAE::EqMod>,
    mut inIndices: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inInfo: &SourceInfo,
) -> Result<Option<DAE::EqMod>> {
    let mut outBinding: Option<DAE::EqMod> = inBinding.clone();
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut oval: Option<metamodelica::Ref<Values::Value>>;
    let mut val: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut aexp: metamodelica::Ref<Absyn::Exp>;
    let mut eq: DAE::EqMod;
    let mut info: SourceInfo;
    if (inBinding).is_none() || (inIndices).is_empty() {
        return Ok(outBinding);
    }
    let __pa0 = ::match_deref::match_deref! { match &(inBinding) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    eq = metamodelica::Own::own(__pa0);
    outBinding = 'mc: {
        let __mc_input = &eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::EqMod::TYPED { modifierAsValue: Some(Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. }), .. } => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::EqMod::TYPED { modifierAsExp: exp, modifierAsValue: oval, properties: DAE::Properties::PROP { type_: ty, constFlag: c }, modifierAsAbsynExp: aexp, info } => {
                    let mut exp = (*exp).clone();
                    let mut oval = (*oval).clone();
                    let mut ty = (*ty).clone();
                    let mut val: metamodelica::Ref<Values::Value> = val.clone();
                    for mut i in &*inIndices {
                        if !(Types::isArray(metamodelica::AsArg::as_arg(&ty))) {
                            Error::addSourceMessage(&(Error::MODIFIER_NON_ARRAY_TYPE_WARNING.clone()), list![ExpressionBasics::printExpStr(exp.clone())?], inInfo)?;
                            return Ok((outBinding.clone(), val.clone()));
                        }
                        ty = Types::unliftArray(metamodelica::AsArg::as_arg(&ty))?;
                        (exp, _) = ExpressionSimplify::simplify1(Expression::makeASUB(exp.clone(), list![i.clone()])?)?;
                    }
                    if (oval).is_some() {
                        let __pa0 = ::match_deref::match_deref! { match &(oval.clone()) {
                            Some(__pa0) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        val = metamodelica::Own::own(__pa0);
                        for mut i in &*inIndices {
                            val = ValuesUtil::nthArrayelt(&val, ExpressionBasics::expArrayIndex(metamodelica::AsArg::as_arg(&i))?)?;
                        }
                        oval = Some(val.clone());
                    }
                    Ok((Some(DAE::EqMod::TYPED { modifierAsExp: exp.clone(), modifierAsValue: oval.clone(), properties: DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() }, modifierAsAbsynExp: aexp.clone(), info: info.clone() }), val.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            val = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Mod.indexEqmod failed for mod:\n ")); __mm_s.push_str(&*TypesDump::unparseEqMod(&eq)?); __mm_s.push_str(&*literal!("\n indices: ")); __mm_s.push_str(&*ExpressionDump::printExpListStr(inIndices.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outBinding)
}

pub(crate) fn merge(
    mut inModOuter: metamodelica::Ref<DAE::Mod>,
    mut inModInner: metamodelica::Ref<DAE::Mod>,
    mut inElementName: ArcStr,
    mut inCheckFinal: bool,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut mod_str: ArcStr;
    if isEmptyMod(&inModOuter) {
        outMod = inModInner;
    } else if isEmptyMod(&inModInner) {
        outMod = inModOuter;
    } else if inCheckFinal
        && isFinalMod(&inModInner)
        && !(merge_isEqual(&inModOuter, &inModInner))
        && !(isRedeclareMod(&inModOuter))
    {
        mod_str = unparseModStr(&inModOuter)?;
        Error::addMultiSourceMessage(
            &(Error::FINAL_COMPONENT_OVERRIDE.clone()),
            &(list![inElementName, mod_str]),
            &(list![getModInfo(&inModInner), getModInfo(&inModOuter)]),
        )?;
        return Err("fail");
    } else {
        outMod = doMerge(inModOuter, inModInner, inCheckFinal)?;
    }
    Ok(outMod)
}

fn merge_isEqual(mut inMod1: &metamodelica::Ref<DAE::Mod>, mut inMod2: &metamodelica::Ref<DAE::Mod>) -> bool {
    let mut outIsEqual: bool;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    if referenceEq(&*(&**inMod1), &*(&**inMod2)) {
        outIsEqual = true;
    } else {
        info1 = getModInfo(inMod1);
        info2 = getModInfo(inMod2);
        outIsEqual = !(Util::sourceInfoIsEmpty(&info1) || Util::sourceInfoIsEmpty(&info2))
            && Util::sourceInfoIsEqual(&info1, &info2);
    }
    outIsEqual
}

pub(crate) fn isFinalMod(mut inMod1: &metamodelica::Ref<DAE::Mod>) -> bool {
    let mut outMod: bool;
    outMod = (::match_deref::match_deref! { match inMod1 {
        Deref @ DAE::Mod::MOD { finalPrefix: SCode::Final::FINAL { .. }, .. } => true,
        Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { finalPrefix: SCode::Final::FINAL { .. }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

fn doMerge(
    mut inModOuter: metamodelica::Ref<DAE::Mod>,
    mut inModInner: metamodelica::Ref<DAE::Mod>,
    mut inCheckFinal: bool,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = inModOuter.clone();
    outMod = (::match_deref::match_deref! { match &((outMod.clone(), inModInner.clone())) {
        (Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { .. }, .. }, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: None }, .. }, .. }, .. }) => {
            inModOuter
        },
        (Deref @ DAE::Mod::REDECL { element: el1 @ Deref @ SCode::Element::COMPONENT { .. }, r#mod: emod1, .. }, Deref @ DAE::Mod::REDECL { element: el2 @ Deref @ SCode::Element::COMPONENT { .. }, r#mod: emod2, .. }) => {
            let mut smod1: metamodelica::Ref<SCode::Mod>;
            let mut smod2: metamodelica::Ref<SCode::Mod>;
            let mut emod: metamodelica::Ref<DAE::Mod>;
            let mut dmod1: metamodelica::Ref<DAE::Mod>;
            let mut dmod2: metamodelica::Ref<DAE::Mod>;
            let mut dmod: metamodelica::Ref<DAE::Mod>;
            let mut el1 = (*el1).clone();
            smod1 = SCodeUtil::getConstrainedByModifiers(var_field!((*el1).prefixes, SCode::Element::COMPONENT));
            smod1 = SCodeUtil::mergeModifiers(var_field!((*el1).modifications, SCode::Element::COMPONENT).clone(), smod1);
            dmod1 = elabUntypedMod(smod1, ModScope::COMPONENT { name: var_field!((*el1).name, SCode::Element::COMPONENT).clone() })?;
            smod2 = SCodeUtil::getConstrainedByModifiers(var_field!((**el2).prefixes, SCode::Element::COMPONENT));
            smod2 = SCodeUtil::mergeModifiers(var_field!((**el2).modifications, SCode::Element::COMPONENT).clone(), smod2);
            dmod2 = elabUntypedMod(smod2, ModScope::COMPONENT { name: var_field!((**el2).name, SCode::Element::COMPONENT).clone() })?;
            dmod = merge(dmod1, dmod2, var_field!((*el1).name, SCode::Element::COMPONENT).clone(), inCheckFinal)?;
            emod = merge(emod1.clone(), emod2.clone(), var_field!((*el1).name, SCode::Element::COMPONENT).clone(), inCheckFinal)?;
            assign_variant_field!(el1 => SCode::Element::COMPONENT;
                modifications = unelabMod(dmod)?,
                prefixes = SCodeUtil::propagatePrefixes(var_field!((**el2).prefixes, SCode::Element::COMPONENT), var_field!((*el1).prefixes, SCode::Element::COMPONENT).clone())?,
                attributes = SCodeUtil::propagateAttributes(var_field!((**el2).attributes, SCode::Element::COMPONENT).clone(), var_field!((*el1).attributes, SCode::Element::COMPONENT).clone(), false)
            );
            assign_variant_field!(outMod => DAE::Mod::REDECL;
                element = el1.clone(),
                r#mod = emod
            );
            outMod
        },
        (Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { .. }, .. }, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: None }, .. }, .. }, .. }) => {
            inModOuter
        },
        (Deref @ DAE::Mod::REDECL { element: el1 @ Deref @ SCode::Element::CLASS { .. }, r#mod: emod1, .. }, Deref @ DAE::Mod::REDECL { element: el2 @ Deref @ SCode::Element::CLASS { .. }, r#mod: emod2, .. }) => {
            let mut smod1: metamodelica::Ref<SCode::Mod>;
            let mut smod2: metamodelica::Ref<SCode::Mod>;
            let mut emod: metamodelica::Ref<DAE::Mod>;
            let mut dmod1: metamodelica::Ref<DAE::Mod>;
            let mut dmod2: metamodelica::Ref<DAE::Mod>;
            let mut res: SCode::Restriction;
            let mut info: SourceInfo;
            let mut el1 = (*el1).clone();
            let mut emod1 = (*emod1).clone();
            let mut emod2 = (*emod2).clone();
            smod1 = SCodeUtil::getConstrainedByModifiers(var_field!((*el1).prefixes, SCode::Element::CLASS));
            dmod1 = elabUntypedMod(smod1, ModScope::COMPONENT { name: var_field!((*el1).name, SCode::Element::CLASS).clone() })?;
            emod1 = merge(emod1.clone(), dmod1, var_field!((*el1).name, SCode::Element::CLASS).clone(), inCheckFinal)?;
            smod2 = SCodeUtil::getConstrainedByModifiers(var_field!((**el2).prefixes, SCode::Element::CLASS));
            dmod2 = elabUntypedMod(smod2, ModScope::COMPONENT { name: var_field!((**el2).name, SCode::Element::CLASS).clone() })?;
            emod2 = merge(emod2.clone(), dmod2, var_field!((*el1).name, SCode::Element::CLASS).clone(), inCheckFinal)?;
            emod = merge(emod1.clone(), emod2.clone(), var_field!((*el1).name, SCode::Element::CLASS).clone(), inCheckFinal)?;
            assign_variant_field!(el1 => SCode::Element::CLASS; prefixes = SCodeUtil::propagatePrefixes(var_field!((**el2).prefixes, SCode::Element::CLASS), var_field!((**el2).prefixes, SCode::Element::CLASS).clone())?);
            (res, info) = SCodeUtil::checkSameRestriction(var_field!((*el1).restriction, SCode::Element::CLASS).clone(), var_field!((**el2).restriction, SCode::Element::CLASS), var_field!((*el1).info, SCode::Element::CLASS).clone(), var_field!((**el2).info, SCode::Element::CLASS));
            assign_variant_field!(el1 => SCode::Element::CLASS;
                restriction = res,
                info = info
            );
            assign_variant_field!(outMod => DAE::Mod::REDECL;
                element = el1.clone(),
                r#mod = emod
            );
            outMod
        },
        (Deref @ DAE::Mod::REDECL { element: el1, r#mod: emod, .. }, Deref @ DAE::Mod::MOD { .. }) => {
            let mut emod = (*emod).clone();
            emod = merge(emod.clone(), inModInner, literal!(""), inCheckFinal)?;
            assign_variant_field!(outMod => DAE::Mod::REDECL;
                element = el1.clone(),
                r#mod = emod.clone()
            );
            outMod
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::REDECL { element: el2, r#mod: emod, .. }) => {
            let mut emod = (*emod).clone();
            emod = merge(inModOuter, emod.clone(), literal!(""), inCheckFinal)?;
            metamodelica::Ref::new(DAE::Mod::REDECL { finalPrefix: var_field!((*inModInner).finalPrefix, DAE::Mod::REDECL).clone(), eachPrefix: var_field!((*inModInner).eachPrefix, DAE::Mod::REDECL).clone(), element: el2.clone(), r#mod: emod.clone() })
        },
        (Deref @ DAE::Mod::MOD { binding: Some(eqmod @ DAE::EqMod::TYPED { modifierAsValue: Some(val @ Deref @ Values::Value::RECORD { .. }), .. }), subModLst: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ DAE::Mod::MOD { binding: None, subModLst: submods @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }) => {
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut names: metamodelica::List<ArcStr>;
            let mut name: ArcStr;
            let mut submod: metamodelica::Ref<DAE::SubMod>;
            let mut eqmod = (*eqmod).clone();
            let mut val = (*val).clone();
            let mut submods = (*submods).clone();
            names = var_field!((*val).comp, Values::Value::RECORD).clone();
            vals = metamodelica::nil();
            for mut v in &*var_field!((*val).orderd, Values::Value::RECORD).clone() {
                let mut v = v.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(names) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                name = metamodelica::Own::own(__pa0);
                names = metamodelica::Own::own(__pa1);
                if ValuesUtil::isEmpty(&v) {
                    if '__try2: {
                        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue(name.clone(), submods.clone(), &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isSubModNamed(&__a0, &__a1)) }), '__try2)) {
                            (__pa3, Some(__pa4)) => (__pa3.clone(), __pa4.clone()),
                            _ => break '__try2 Err::<_, _>("pattern mismatch"),
                        } };
                        submods = metamodelica::Own::own(__pa3);
                        submod = metamodelica::Own::own(__pa4);
                        v = unwrap_break_err!(subModValue(&submod), '__try2);
                        Ok::<(), &'static str>(())
                    }.is_err() {
                    }
                }
                vals = metamodelica::cons(v, vals);
            }
            assign_variant_field!(val => Values::Value::RECORD; orderd = vals.reverse());
            let __owned_variant_modifierAsValue_0 = Some(val.clone());
            if let DAE::EqMod::TYPED { modifierAsValue, .. } = &mut eqmod {
                *modifierAsValue = __owned_variant_modifierAsValue_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED"); }
            assign_variant_field!(outMod => DAE::Mod::MOD;
                binding = Some(eqmod.clone()),
                subModLst = stripSubModBindings(var_field!((*inModInner).subModLst, DAE::Mod::MOD))
            );
            outMod
        },
        (Deref @ DAE::Mod::MOD { binding: None, subModLst: submods @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, Deref @ DAE::Mod::MOD { binding: Some(eqmod @ DAE::EqMod::TYPED { modifierAsValue: Some(val @ Deref @ Values::Value::RECORD { .. }), .. }), subModLst: Deref @ metamodelica::ListNode::Nil, .. }) => {
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut names: metamodelica::List<ArcStr>;
            let mut name: ArcStr;
            let mut submod: metamodelica::Ref<DAE::SubMod>;
            let mut submods = (*submods).clone();
            let mut eqmod = (*eqmod).clone();
            let mut val = (*val).clone();
            names = var_field!((*val).comp, Values::Value::RECORD).clone();
            vals = metamodelica::nil();
            for mut v in &*var_field!((*val).orderd, Values::Value::RECORD).clone() {
                let mut v = v.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(names) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                name = metamodelica::Own::own(__pa0);
                names = metamodelica::Own::own(__pa1);
                if '__try2: {
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue(name.clone(), submods.clone(), &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isSubModNamed(&__a0, &__a1)) }), '__try2)) {
                        (__pa3, Some(__pa4)) => (__pa3.clone(), __pa4.clone()),
                        _ => break '__try2 Err::<_, _>("pattern mismatch"),
                    } };
                    submods = metamodelica::Own::own(__pa3);
                    submod = metamodelica::Own::own(__pa4);
                    v = unwrap_break_err!(subModValue(&submod), '__try2);
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
                vals = metamodelica::cons(v, vals);
            }
            assign_variant_field!(val => Values::Value::RECORD; orderd = vals.reverse());
            let __owned_variant_modifierAsValue_0 = Some(val.clone());
            if let DAE::EqMod::TYPED { modifierAsValue, .. } = &mut eqmod {
                *modifierAsValue = __owned_variant_modifierAsValue_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED"); }
            assign_variant_field!(outMod => DAE::Mod::MOD;
                binding = Some(eqmod.clone()),
                subModLst = stripSubModBindings(var_field!((*outMod).subModLst, DAE::Mod::MOD))
            );
            outMod
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            assign_variant_field!(outMod => DAE::Mod::MOD;
                subModLst = mergeSubs(var_field!((*outMod).subModLst, DAE::Mod::MOD).clone(), var_field!((*inModInner).subModLst, DAE::Mod::MOD).clone(), inCheckFinal)?,
                binding = mergeEq(var_field!((*outMod).binding, DAE::Mod::MOD).clone(), var_field!((*inModInner).binding, DAE::Mod::MOD).clone())
            );
            outMod
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outMod)
}

fn mergeSubs(
    mut inSubMods1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inSubMods2: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inCheckFinal: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
    let mut submods2: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    let mut name: ArcStr;
    let mut m1: metamodelica::Ref<DAE::Mod>;
    let mut m2: metamodelica::Ref<DAE::Mod>;
    let mut osm2: Option<metamodelica::Ref<DAE::SubMod>>;
    let mut sm2: metamodelica::Ref<DAE::SubMod>;
    if (inSubMods1).is_empty() {
        outSubMods = inSubMods2;
    } else if (inSubMods2).is_empty() {
        outSubMods = inSubMods1;
    } else {
        submods2 = inSubMods2;
        for mut sm1 in &*inSubMods1 {
            let mut sm1 = sm1.clone();
            (submods2, osm2) = List::deleteMemberOnTrue(
                subModName(&sm1),
                submods2,
                &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::SubMod>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(subModIsNamed(&__a0, &__a1))
                },
            )?;
            if (osm2).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(osm2) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                sm2 = metamodelica::Own::own(__pa0);
                let __arc3 = sm1;
                let DAE::NAMEMOD {
                    ident: __pa1,
                    r#mod: __pa2,
                } = &*__arc3;
                name = metamodelica::Own::own(__pa1);
                m1 = metamodelica::Own::own(__pa2);
                let __arc5 = sm2;
                let DAE::NAMEMOD { r#mod: __pa4, .. } = &*__arc5;
                m2 = metamodelica::Own::own(__pa4);
                m1 = merge(m1, m2, name.clone(), inCheckFinal)?;
                sm1 = metamodelica::Ref::new(DAE::SubMod { ident: name, r#mod: m1 });
            }
            outSubMods = metamodelica::cons(sm1, outSubMods);
        }
        outSubMods = List::append_reverse(&outSubMods, submods2);
    }
    Ok(outSubMods)
}

fn mergeEq(mut inOuterEq: Option<DAE::EqMod>, mut inInnerEq: Option<DAE::EqMod>) -> Option<DAE::EqMod> {
    let mut outEqMod: Option<DAE::EqMod> = if ((inOuterEq).is_some()) {
        inOuterEq.clone()
    } else {
        inInnerEq.clone()
    };
    outEqMod
}

pub(crate) fn modEquation(mut inMod: &metamodelica::Ref<DAE::Mod>) -> Option<DAE::EqMod> {
    let mut outEqMod: Option<DAE::EqMod>;
    outEqMod = (match &**inMod {
        DAE::Mod::NOMOD { .. } => None,
        DAE::Mod::REDECL { .. } => None,
        DAE::Mod::MOD {
            binding: __inMod_binding,
            ..
        } => __inMod_binding.clone(),
    });
    outEqMod
}

fn modSubsetOrEqualOrNonOverlap(
    mut mod1: &metamodelica::Ref<DAE::Mod>,
    mut mod2: &metamodelica::Ref<DAE::Mod>,
) -> bool {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (mod1, mod2) {
        (Deref @ DAE::Mod::MOD { finalPrefix: f1, eachPrefix: _, subModLst: _, binding: None, info: _ }, Deref @ DAE::Mod::MOD { finalPrefix: f2, eachPrefix: SCode::Each::NOT_EACH { .. }, subModLst: Deref @ metamodelica::ListNode::Nil, binding: Some(_), info: _ }) if (SCodeUtil::finalEqual(f1.clone(), f2.clone())) => {
            true
        },
        (Deref @ DAE::Mod::MOD { binding: eqmod1, .. }, Deref @ DAE::Mod::MOD { finalPrefix: _, eachPrefix: SCode::Each::NOT_EACH { .. }, subModLst: Deref @ metamodelica::ListNode::Nil, binding: eqmod2, info: _ }) if (eqModSubsetOrEqual(eqmod1.clone(), eqmod2.clone())) => {
            true
        },
        (Deref @ DAE::Mod::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submods1, binding: eqmod1, info: _ }, Deref @ DAE::Mod::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submods2, binding: eqmod2, info: _ }) if (SCodeUtil::finalEqual(f1.clone(), f2.clone()) && SCodeUtil::eachEqual(each1.clone(), each2.clone()) && subModsEqual(submods1, submods2) && eqModSubsetOrEqual(eqmod1.clone(), eqmod2.clone())) => {
            true
        },
        (Deref @ DAE::Mod::REDECL { finalPrefix: f1, eachPrefix: each1, .. }, Deref @ DAE::Mod::REDECL { finalPrefix: f2, eachPrefix: each2, .. }) if (SCodeUtil::finalEqual(f1.clone(), f2.clone()) && SCodeUtil::eachEqual(each1.clone(), each2.clone()) && SCodeUtil::elementEqual(var_field!((**mod1).element, DAE::Mod::REDECL), var_field!((**mod2).element, DAE::Mod::REDECL))) => {
            true
        },
        (Deref @ DAE::Mod::NOMOD { .. }, Deref @ DAE::Mod::NOMOD { .. }) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

fn eqModSubsetOrEqual(mut eqMod1: Option<DAE::EqMod>, mut eqMod2: Option<DAE::EqMod>) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (eqMod1.clone(), eqMod2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (None, None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, Some(_)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::TYPED { .. }), Some(DAE::EqMod::TYPED { .. })) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (eqModEqual(eqMod1.clone(), eqMod2.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Some(DAE::EqMod::TYPED {
                    modifierAsAbsynExp: ref aexp1,
                    ..
                }),
                Some(DAE::EqMod::UNTYPED { exp: ref aexp2 }),
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }),
                Some(DAE::EqMod::TYPED {
                    modifierAsAbsynExp: ref aexp2,
                    ..
                }),
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }), Some(DAE::EqMod::UNTYPED { exp: ref aexp2 })) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else {
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
    equal
}

fn subModsSubsetOrEqual(
    mut subModLst1: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut subModLst2: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (&**subModLst1, &**subModLst2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id1, r#mod: mod1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id2, r#mod: mod2 }, tail: rest2 }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    let true = (modEqual(metamodelica::AsArg::as_arg(&mod1), metamodelica::AsArg::as_arg(&mod2))?) else { return Err("pattern mismatch") };
                    let true = (subModsEqual(metamodelica::AsArg::as_arg(&rest1), metamodelica::AsArg::as_arg(&rest2))) else { return Err("pattern mismatch") };
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
    equal
}

pub(crate) fn modEqual(mut mod1: &metamodelica::Ref<DAE::Mod>, mut mod2: &metamodelica::Ref<DAE::Mod>) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (mod1, mod2) {
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => SCodeUtil::finalEqual(var_field!((**mod1).finalPrefix, DAE::Mod::MOD).clone(), var_field!((**mod2).finalPrefix, DAE::Mod::MOD).clone()) && SCodeUtil::eachEqual(var_field!((**mod1).eachPrefix, DAE::Mod::MOD).clone(), var_field!((**mod2).eachPrefix, DAE::Mod::MOD).clone()) && List::isEqualOnTrue(var_field!((**mod1).subModLst, DAE::Mod::MOD).clone(), var_field!((**mod2).subModLst, DAE::Mod::MOD).clone(), &move |__a0: metamodelica::Ref<DAE::SubMod>, __a1: metamodelica::Ref<DAE::SubMod>| subModEqual(&__a0, &__a1))? && eqModEqual(var_field!((**mod1).binding, DAE::Mod::MOD).clone(), var_field!((**mod2).binding, DAE::Mod::MOD).clone()),
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::REDECL { .. }) => SCodeUtil::finalEqual(var_field!((**mod1).finalPrefix, DAE::Mod::REDECL).clone(), var_field!((**mod2).finalPrefix, DAE::Mod::REDECL).clone()) && SCodeUtil::eachEqual(var_field!((**mod1).eachPrefix, DAE::Mod::REDECL).clone(), var_field!((**mod2).eachPrefix, DAE::Mod::REDECL).clone()) && SCodeUtil::elementEqual(var_field!((**mod1).element, DAE::Mod::REDECL), var_field!((**mod2).element, DAE::Mod::REDECL)),
        (Deref @ DAE::Mod::NOMOD { .. }, Deref @ DAE::Mod::NOMOD { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

fn subModsEqual(
    mut inSubModLst1: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inSubModLst2: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (&**inSubModLst1, &**inSubModLst2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id1, r#mod: mod1 }, tail: subModLst1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id2, r#mod: mod2 }, tail: subModLst2 }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    let true = (modEqual(metamodelica::AsArg::as_arg(&mod1), metamodelica::AsArg::as_arg(&mod2))?) else { return Err("pattern mismatch") };
                    let true = (subModsEqual(metamodelica::AsArg::as_arg(&subModLst1), metamodelica::AsArg::as_arg(&subModLst2))) else { return Err("pattern mismatch") };
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
    equal
}

pub(crate) fn subModEqual(
    mut subMod1: &metamodelica::Ref<DAE::SubMod>,
    mut subMod2: &metamodelica::Ref<DAE::SubMod>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (subMod1, subMod2) {
        (Deref @ DAE::SubMod { ident: id1, r#mod: mod1 }, Deref @ DAE::SubMod { ident: id2, r#mod: mod2 }) if (stringEq(&id1, &id2) && modEqual(mod1, mod2)?) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

fn valEqual(
    mut inV1: Option<metamodelica::Ref<Values::Value>>,
    mut inV2: Option<metamodelica::Ref<Values::Value>>,
    mut equal: bool,
) -> Result<bool> {
    let mut bEq: bool;
    bEq = (::match_deref::match_deref! { match &((inV1, inV2, equal)) {
        (_, _, true) => {
            true
        },
        (None, None, _) => {
            equal
        },
        (Some(v1), Some(v2), false) => {
            bEq = ExpressionBasics::expEqual(&(ValuesUtil::valueExp(v1.clone(), None)?), ValuesUtil::valueExp(v2.clone(), None)?)?;
            bEq
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(bEq)
}

fn eqModEqual(mut eqMod1: Option<DAE::EqMod>, mut eqMod2: Option<DAE::EqMod>) -> bool {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (eqMod1, eqMod2);
        if let Ok(__v) = (|| -> Result<_> {
            let (None, None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (
                Some(DAE::EqMod::TYPED {
                    modifierAsExp: ref exp1,
                    modifierAsValue: mut v1,
                    ..
                }),
                Some(DAE::EqMod::TYPED {
                    modifierAsExp: ref exp2,
                    modifierAsValue: mut v2,
                    ..
                }),
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut equal: bool = equal.clone();
            equal = ExpressionBasics::expEqual(&(exp1.clone()), exp2.clone())?;
            let true = (valEqual(v1.clone(), v2.clone(), equal)?) else {
                return Err("pattern mismatch");
            };
            Ok((true, equal.clone()))
        })() {
            equal = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Some(DAE::EqMod::TYPED {
                    modifierAsAbsynExp: ref aexp1,
                    ..
                }),
                Some(DAE::EqMod::UNTYPED { exp: ref aexp2 }),
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }),
                Some(DAE::EqMod::TYPED {
                    modifierAsAbsynExp: ref aexp2,
                    ..
                }),
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }), Some(DAE::EqMod::UNTYPED { exp: ref aexp2 })) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else {
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
    equal
}

pub(crate) fn printModStr(mut inMod: &metamodelica::Ref<DAE::Mod>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD { .. } => {
                    Ok(literal!("()"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { finalPrefix, eachPrefix, .. } => {
                    let mut prefix: ArcStr;
                    let mut r#str: ArcStr;
                    let mut res: ArcStr;
                    prefix = { let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::finalStr(finalPrefix.clone())); __mm_s.push_str(&*SCodeDump::eachStr(eachPrefix.clone())); ArcStr::from(__mm_s) };
                    r#str = SCodeDump::unparseElementStr(var_field!((**inMod).element, DAE::Mod::REDECL).clone(), SCodeDump::defaultOptions.clone())?;
                    res = stringAppendList(list![literal!("("), prefix.clone(), r#str.clone(), literal!(")")]);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix, subModLst: subs, binding: eq, .. } => {
                    let mut prefix: ArcStr;
                    let mut r#str: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s1: metamodelica::List<ArcStr>;
                    prefix = { let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::finalStr(finalPrefix.clone())); __mm_s.push_str(&*SCodeDump::eachStr(eachPrefix.clone())); ArcStr::from(__mm_s) };
                    s1 = printSubs1Str(metamodelica::AsArg::as_arg(&subs))?;
                    s1_1 = stringDelimitList(s1.clone(), literal!(", "));
                    s1_1 = if (!((subs).is_empty())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" {")); __mm_s.push_str(&*s1_1); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }} else {s1_1.clone()};
                    s2 = printEqmodStr(eq.clone());
                    r#str = stringAppendList(list![prefix.clone(), s1_1.clone(), s2.clone()]);
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
                    metamodelica::print(literal!(" failure in printModStr \n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

pub(crate) fn printMod(mut m: &metamodelica::Ref<DAE::Mod>) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = printModStr(m)?;
    Print::printBuf(r#str)?;
    Ok(())
}

pub(crate) fn prettyPrintMod(mut m: &metamodelica::Ref<DAE::Mod>, mut depth: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = 'mc: {
        let __mc_input = &**m;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { subModLst: subs, binding: None, .. } => {
                    Ok(prettyPrintSubs(metamodelica::AsArg::as_arg(&subs), depth)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix: fp, binding: Some(eq), .. } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (SCodeUtil::finalBool(fp.clone())) {literal!("final ")} else {literal!("")}); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*TypesDump::unparseEqMod(metamodelica::AsArg::as_arg(&eq))?); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { .. } => {
                    Ok(SCodeDump::unparseElementStr(var_field!((**m).element, DAE::Mod::REDECL).clone(), SCodeDump::defaultOptions.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD { .. } => {
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
                    metamodelica::print(literal!(" failed prettyPrintMod\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(r#str)
}

fn prettyPrintSubs(mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>, mut depth: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match inSubs {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod: Deref @ DAE::Mod::REDECL { .. } }, tail: _ } => {
            let mut s2: ArcStr;
            s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" redeclare(")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!("), class or component ")); __mm_s.push_str(&*id); ArcStr::from(__mm_s) };
            s2
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod: m }, tail: _ } => {
            let mut s2: ArcStr;
            s2 = prettyPrintMod(metamodelica::AsArg::as_arg(&m), depth + 1)?;
            s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*id); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!("), class or component ")); __mm_s.push_str(&*id); ArcStr::from(__mm_s) };
            s2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

pub(crate) fn prettyPrintSubmod(mut inSub: &metamodelica::Ref<DAE::SubMod>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match inSub {
        Deref @ DAE::SubMod { ident: id, r#mod: m @ Deref @ DAE::Mod::REDECL { .. } } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = SCodeDump::unparseElementStr(var_field!((**m).element, DAE::Mod::REDECL).clone(), SCodeDump::defaultOptions.clone())?;
            s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!("(redeclare ")); __mm_s.push_str(&*if (SCodeUtil::eachBool(var_field!((**m).eachPrefix, DAE::Mod::REDECL).clone())) {literal!("each ")} else {literal!("")}); __mm_s.push_str(&*if (SCodeUtil::finalBool(var_field!((**m).finalPrefix, DAE::Mod::REDECL).clone())) {literal!("final ")} else {literal!("")}); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            s2
        },
        Deref @ DAE::SubMod { ident: id, r#mod: m } => {
            let mut s2: ArcStr;
            s2 = prettyPrintMod(m, 0)?;
            s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*id); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
            s2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

pub(crate) fn printSubs1Str(
    mut inTypesSubModLst: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = (::match_deref::match_deref! { match inTypesSubModLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
            let mut s1: ArcStr;
            let mut res: metamodelica::List<ArcStr>;
            s1 = printSubStr(metamodelica::AsArg::as_arg(&x))?;
            res = printSubs1Str(xs)?;
            metamodelica::cons(s1, res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStringLst)
}

fn printSubStr(mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inSubMod {
        DAE::SubMod { ident: n, r#mod } => {
            let mut mod_str: ArcStr;
            let mut res: ArcStr;
            mod_str = printModStr(r#mod)?;
            res = stringAppend(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*n);
                    __mm_s.push_str(&*literal!(" "));
                    ArcStr::from(__mm_s)
                },
                mod_str,
            );
            res
        }
    });
    Ok(outString)
}

fn printEqmodStr(mut inTypesEqModOption: Option<DAE::EqMod>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inTypesEqModOption;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: Some(e_val), properties: prop, modifierAsAbsynExp: _, .. }) => {
                    let mut r#str: ArcStr;
                    let mut str2: ArcStr;
                    let mut e_val_str: ArcStr;
                    let mut res: ArcStr;
                    r#str = ExpressionBasics::printExpStr(e.clone())?;
                    str2 = Types::printPropStr(metamodelica::AsArg::as_arg(&prop))?;
                    e_val_str = ValuesDump::valString(metamodelica::AsArg::as_arg(&e_val))?;
                    res = stringAppendList(list![literal!(" = (typed)"), r#str.clone(), literal!(" "), str2.clone(), literal!(", value: "), e_val_str.clone()]);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: None, properties: prop, modifierAsAbsynExp: _, .. }) => {
                    let mut r#str: ArcStr;
                    let mut str2: ArcStr;
                    let mut res: ArcStr;
                    r#str = ExpressionBasics::printExpStr(e.clone())?;
                    str2 = Types::printPropStr(metamodelica::AsArg::as_arg(&prop))?;
                    res = stringAppendList(list![literal!(" = (typed)"), r#str.clone(), literal!(", type:\n"), str2.clone()]);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(DAE::EqMod::UNTYPED { exp: ae }) => {
                    let mut r#str: ArcStr;
                    let mut res: ArcStr;
                    r#str = Dump::printExpStr(ae.clone())?;
                    res = stringAppend(literal!(" =(untyped) "), r#str.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: ArcStr;
                    res = literal!("---Mod.printEqmodStr FAILED---");
                    Ok(res.clone())
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

pub(crate) fn renameTopLevelNamedSubMod(
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut oldIdent: &ArcStr,
    mut newIdent: ArcStr,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = r#mod.clone();
    outMod = (match &*outMod {
        DAE::Mod::MOD {
            subModLst: __outMod_subModLst,
            ..
        } => {
            assign_variant_field!(outMod => DAE::Mod::MOD; subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
                for mut s in (__outMod_subModLst.clone()).into_iter().cloned() {
                    let __x = renameNamedSubMod(s.clone(), oldIdent, newIdent.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            outMod
        }
        _ => r#mod,
    });
    outMod
}

pub(crate) fn renameNamedSubMod(
    mut submod: metamodelica::Ref<DAE::SubMod>,
    mut oldIdent: &ArcStr,
    mut newIdent: ArcStr,
) -> metamodelica::Ref<DAE::SubMod> {
    let mut outMod: metamodelica::Ref<DAE::SubMod>;
    outMod = (match &*submod {
        DAE::SubMod { ident: id, r#mod } if (stringEq(&id, &oldIdent)) => metamodelica::Ref::new(DAE::SubMod {
            ident: newIdent,
            r#mod: r#mod.clone(),
        }),
        _ => submod,
    });
    outMod
}

pub(crate) fn emptyModOrEquality(mut r#mod: &metamodelica::Ref<DAE::Mod>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match r#mod {
        Deref @ DAE::Mod::NOMOD { .. } => true,
        Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn intStringDot(mut i: i32) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(i));
        __mm_s.push_str(&*literal!("."));
        ArcStr::from(__mm_s)
    };
    r#str
}

fn isPrefixOf(mut indexSubMod: &(ArcStr, metamodelica::Ref<DAE::SubMod>), mut idx: ArcStr) -> bool {
    let mut isPrefix: bool;
    isPrefix = 'mc: {
        let __mc_input = indexSubMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, _) => {
                    let mut len1: i32;
                    let mut len2: i32;
                    len1 = ((i).len() as i32);
                    len2 = ((idx).len() as i32);
                    let true = (0 == System::strncmp(i.clone(), idx.clone(), intMin(len1, len2))) else { return Err("pattern mismatch") };
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
    isPrefix
}

fn getFullModsFromMod(
    mut inTopCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
) -> Result<metamodelica::List<FullMod>> {
    let mut outFullMods: metamodelica::List<FullMod>;
    outFullMods = (match &*inMod {
        DAE::Mod::NOMOD { .. } => metamodelica::nil(),
        DAE::Mod::MOD {
            subModLst: __inMod_subModLst,
            ..
        } => getFullModsFromSubMods(inTopCref, metamodelica::AsArg::as_arg(&__inMod_subModLst))?,
        DAE::Mod::REDECL { .. } => list![getFullModFromModRedeclare(inTopCref, inMod)?],
    });
    Ok(outFullMods)
}

fn getFullModFromModRedeclare(
    mut inTopCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inRedeclare: metamodelica::Ref<DAE::Mod>,
) -> Result<FullMod> {
    let mut outFullMod: FullMod;
    let mut el: metamodelica::Ref<SCode::Element>;
    let mut id: ArcStr;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(inRedeclare.clone()) {
        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    el = metamodelica::Own::own(__pa0);
    id = SCodeUtil::elementName(&el)?;
    cref = ComponentReferenceBasics::makeCrefIdent(id, DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    cref = ComponentReference::joinCrefs(inTopCref, cref)?;
    outFullMod = FullMod::MOD {
        cref: cref,
        r#mod: inRedeclare,
    };
    Ok(outFullMod)
}

fn getFullModsFromSubMods(
    mut inTopCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inSubMods: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> Result<metamodelica::List<FullMod>> {
    let mut outFullMods: metamodelica::List<FullMod>;
    outFullMods = (::match_deref::match_deref! { match inSubMods {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: subMod @ Deref @ DAE::SubMod { ident: id, r#mod }, tail: rest } => {
            let mut fullMods1: metamodelica::List<FullMod>;
            let mut fullMods2: metamodelica::List<FullMod>;
            let mut fullMods: metamodelica::List<FullMod>;
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            cref = ComponentReference::joinCrefs(inTopCref, ComponentReferenceBasics::makeCrefIdent(id.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
            fullMods1 = getFullModsFromMod(&cref, r#mod.clone())?;
            fullMods2 = getFullModsFromSubMods(inTopCref, rest)?;
            fullMods = listAppend(if ((fullMods1).is_empty()) {metamodelica::cons(FullMod::SUB_MOD { cref: cref, subMod: subMod.clone() }, fullMods1)} else {fullMods1}, fullMods2);
            fullMods
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFullMods)
}

fn fullModCrefsEqual(mut inFullMod1: &FullMod, mut inFullMod2: &FullMod) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (match (inFullMod1.clone(), inFullMod2.clone()) {
        (
            FullMod::MOD {
                cref: ref cr1,
                r#mod: _,
            },
            FullMod::MOD {
                cref: ref cr2,
                r#mod: _,
            },
        ) => ComponentReferenceBasics::crefEqualNoStringCompare(
            metamodelica::AsArg::as_arg(&cr1),
            metamodelica::AsArg::as_arg(&cr2),
        )?,
        (
            FullMod::SUB_MOD {
                cref: ref cr1,
                subMod: _,
            },
            FullMod::SUB_MOD {
                cref: ref cr2,
                subMod: _,
            },
        ) => ComponentReferenceBasics::crefEqualNoStringCompare(
            metamodelica::AsArg::as_arg(&cr1),
            metamodelica::AsArg::as_arg(&cr2),
        )?,
        (
            FullMod::MOD {
                cref: ref cr1,
                r#mod: _,
            },
            FullMod::SUB_MOD {
                cref: ref cr2,
                subMod: _,
            },
        ) => ComponentReferenceBasics::crefEqualNoStringCompare(
            metamodelica::AsArg::as_arg(&cr1),
            metamodelica::AsArg::as_arg(&cr2),
        )?,
        (
            FullMod::SUB_MOD {
                cref: ref cr1,
                subMod: _,
            },
            FullMod::MOD {
                cref: ref cr2,
                r#mod: _,
            },
        ) => ComponentReferenceBasics::crefEqualNoStringCompare(
            metamodelica::AsArg::as_arg(&cr1),
            metamodelica::AsArg::as_arg(&cr2),
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok(isEqual)
}

fn prettyPrintFullMod(mut inFullMod: &FullMod, mut inDepth: i32) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (match inFullMod.clone() {
        FullMod::MOD {
            cref: ref cr,
            r#mod: mut r#mod,
        } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                    metamodelica::AsArg::as_arg(&cr),
                )?);
                __mm_s.push_str(&*literal!(": "));
                __mm_s.push_str(&*prettyPrintMod(metamodelica::AsArg::as_arg(&r#mod), inDepth)?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        FullMod::SUB_MOD {
            cref: ref cr,
            subMod: mut subMod,
        } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                    metamodelica::AsArg::as_arg(&cr),
                )?);
                __mm_s.push_str(&*literal!(": "));
                __mm_s.push_str(&*prettyPrintSubmod(metamodelica::AsArg::as_arg(&subMod))?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    Ok(outStr)
}

pub fn getUnelabedSubMod(
    mut inMod: &metamodelica::Ref<SCode::Mod>,
    mut inIdent: &ArcStr,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outSubMod: metamodelica::Ref<SCode::Mod>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inMod)) {
        Deref @ SCode::Mod::MOD { subModLst: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    submods = metamodelica::Own::own(__pa0);
    outSubMod = getUnelabedSubMod2(&submods, inIdent)?;
    Ok(outSubMod)
}

fn getUnelabedSubMod2<'__b>(
    mut inSubMods: &'__b metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inIdent: &'__b ArcStr,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inSubMods {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: id, r#mod: m }, tail: _ } if (stringEqual(&id, &inIdent)) => {
                return Ok(m.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_mods } => {
                { (inSubMods, inIdent) = (rest_mods, inIdent); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isUntypedMod(mut inMod: &metamodelica::Ref<DAE::Mod>) -> Result<bool> {
    let mut outIsUntyped: bool;
    outIsUntyped = (match &**inMod {
        DAE::Mod::MOD {
            binding: Some(DAE::EqMod::UNTYPED { .. }),
            ..
        } => true,
        DAE::Mod::MOD {
            subModLst: __inMod_subModLst,
            ..
        } => List::any(
            metamodelica::AsArg::as_arg(&__inMod_subModLst),
            &move |__a0: metamodelica::Ref<DAE::SubMod>| isUntypedSubMod(&__a0),
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok(outIsUntyped)
}

fn isUntypedSubMod(mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> Result<bool> {
    let mut outIsUntyped: bool = isUntypedMod(&inSubMod.r#mod)?;
    Ok(outIsUntyped)
}

pub(crate) fn getUntypedCrefs(
    mut inMod: &metamodelica::Ref<DAE::Mod>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outCrefs = 'mc: {
        let __mc_input = &**inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::UNTYPED { exp }), .. } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    crefs = AbsynUtil::getCrefFromExp(exp.clone(), true, true)?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { subModLst: submods, .. } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    crefs = List::fold(metamodelica::AsArg::as_arg(&submods), &move |__a0: metamodelica::Ref<DAE::SubMod>, __a1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getUntypedCrefFromSubMod(&__a0, __a1)) }, metamodelica::nil())?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCrefs
}

fn getUntypedCrefFromSubMod(
    mut inSubMod: &metamodelica::Ref<DAE::SubMod>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outCrefs = (match &**inSubMod {
        DAE::SubMod { r#mod, .. } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            crefs = getUntypedCrefs(r#mod);
            listAppend(crefs, inCrefs)
        }
    });
    outCrefs
}

// moved from Types!
pub(crate) fn stripSubmod(mut inMod: metamodelica::Ref<DAE::Mod>) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = inMod;
    outMod = (match &*outMod {
        DAE::Mod::MOD { .. } => {
            assign_variant_field!(outMod => DAE::Mod::MOD; subModLst = metamodelica::nil());
            outMod
        }
        _ => outMod,
    });
    outMod
}

pub(crate) fn removeFirstSubsRedecl(mut inMod: metamodelica::Ref<DAE::Mod>) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: Deref @ metamodelica::ListNode::Nil, binding: eq, info } => {
                    Ok(metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: metamodelica::nil(), binding: eq.clone(), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { subModLst: subs, binding: None, .. } => {
                    ::match_deref::match_deref! { match &(removeRedecl(metamodelica::AsArg::as_arg(&subs))) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: subs, binding: eq, info } => {
                    let mut subs = (*subs).clone();
                    subs = removeRedecl(metamodelica::AsArg::as_arg(&subs));
                    Ok(metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs.clone(), binding: eq.clone(), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                m => {
                    Ok(m.clone())
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

fn removeRedecl<'__b>(
    mut isubs: &'__b metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> metamodelica::List<metamodelica::Ref<DAE::SubMod>> {
    let mut osubs: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    osubs = (::match_deref::match_deref! { match isubs {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: _, r#mod: Deref @ DAE::Mod::REDECL { .. } }, tail: subs } => {
            removeRedecl(subs)
        },
        Deref @ metamodelica::ListNode::Cons { head: sm, tail: subs } => {
            osubs = removeRedecl(subs);
            metamodelica::cons(sm.clone(), osubs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    osubs
}

pub(crate) fn removeModList<'__b>(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut remStrings: &'__b metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    '__tco: loop {
        let mut s: ArcStr;
        ::match_deref::match_deref! { match remStrings {
            Deref @ metamodelica::ListNode::Nil => return Ok(inMod),
            Deref @ metamodelica::ListNode::Cons { head: __esc_s, tail: _ } => {
                s = (*__esc_s).clone();
                { (inMod, remStrings) = (removeMod(inMod, metamodelica::AsArg::as_arg(&s))?, remStrings); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn removeMod(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut componentModified: &ArcStr,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (match &*inMod {
        DAE::Mod::NOMOD { .. } => openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        DAE::Mod::REDECL {
            element: __inMod_element,
            ..
        } => {
            if (metamodelica::stringEq(
                &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&__inMod_element))?),
                &componentModified,
            )) {
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD()
            } else {
                inMod
            }
        }
        DAE::Mod::MOD {
            finalPrefix: f,
            eachPrefix: e,
            subModLst: subs,
            binding: oem,
            info,
        } => {
            let mut subs = (*subs).clone();
            subs = removeModInSubs(metamodelica::AsArg::as_arg(&subs), componentModified)?;
            outMod = metamodelica::Ref::new(DAE::Mod::MOD {
                finalPrefix: f.clone(),
                eachPrefix: e.clone(),
                subModLst: subs.clone(),
                binding: oem.clone(),
                info: info.clone(),
            });
            outMod
        }
    });
    Ok(outMod)
}

fn removeModInSubs(
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut componentName: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::SubMod>>> {
    let mut outsubs: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    outsubs = (::match_deref::match_deref! { match inSubs {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: s1, r#mod: m1 }, tail: subs } => {
            let mut subs1: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            let mut subs2: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            subs1 = if (stringEq(&s1, &componentName)) {metamodelica::nil()} else {list![metamodelica::Ref::new(DAE::SubMod { ident: s1.clone(), r#mod: m1.clone() })]};
            subs2 = removeModInSubs(subs, componentName)?;
            outsubs = listAppend(subs1, subs2);
            outsubs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outsubs)
}

pub(crate) fn addEachIfNeeded(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inDimensions: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = 'mc: {
        let __mc_input = (&*inMod, &**inDimensions);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inMod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::NOMOD { .. }, _) => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::REDECL { finalPrefix, eachPrefix: _, element: el, r#mod }, _) => {
                    Ok(metamodelica::Ref::new(DAE::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, element: el.clone(), r#mod: r#mod.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: SCode::Each::EACH { .. }, subModLst: subs, binding: eq, info }, _) => {
                    Ok(metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, subModLst: subs.clone(), binding: eq.clone(), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix, subModLst: subs, binding: eq, info }, _) => {
                    let mut subs = (*subs).clone();
                    subs = addEachToSubsIfNeeded(metamodelica::AsArg::as_arg(&subs), inDimensions)?;
                    Ok(metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: eachPrefix.clone(), subModLst: subs.clone(), binding: eq.clone(), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mod.addEachIfNeeded failed on: ")); __mm_s.push_str(&*printModStr(&inMod)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

pub(crate) fn addEachOneLevel(mut inMod: &metamodelica::Ref<DAE::Mod>) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (match &**inMod {
        DAE::Mod::NOMOD { .. } => openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        DAE::Mod::REDECL {
            finalPrefix,
            eachPrefix: _,
            element: el,
            r#mod,
        } => metamodelica::Ref::new(DAE::Mod::REDECL {
            finalPrefix: finalPrefix.clone(),
            eachPrefix: openmodelica_frontend_types::SCode::Each::EACH,
            element: el.clone(),
            r#mod: r#mod.clone(),
        }),
        DAE::Mod::MOD {
            finalPrefix,
            eachPrefix: _,
            subModLst: subs,
            binding: eq,
            info,
        } => metamodelica::Ref::new(DAE::Mod::MOD {
            finalPrefix: finalPrefix.clone(),
            eachPrefix: openmodelica_frontend_types::SCode::Each::EACH,
            subModLst: subs.clone(),
            binding: eq.clone(),
            info: info.clone(),
        }),
        _ => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Mod.addEachOneLevel failed on: "));
                __mm_s.push_str(&*printModStr(inMod)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            return Err("fail");
        }
    });
    Ok(outMod)
}

pub(crate) fn addEachToSubsIfNeeded(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inDimensions: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    outSubMods = (::match_deref::match_deref! { match (inSubMods, inDimensions) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            inSubMods.clone()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod: m }, tail: rest }, _) => {
            let mut m = (*m).clone();
            let mut rest = (*rest).clone();
            m = addEachOneLevel(metamodelica::AsArg::as_arg(&m))?;
            rest = addEachToSubsIfNeeded(metamodelica::AsArg::as_arg(&rest), inDimensions)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::SubMod { ident: id.clone(), r#mod: m.clone() }), rest.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outSubMods)
}

pub(crate) fn isEmptyMod(mut inMod: &metamodelica::Ref<DAE::Mod>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (::match_deref::match_deref! { match inMod {
        Deref @ DAE::Mod::NOMOD { .. } => true,
        Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, binding: None, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub(crate) fn isNoMod(mut inMod: &metamodelica::Ref<DAE::Mod>) -> bool {
    let mut outIsNoMod: bool;
    outIsNoMod = (match &**inMod {
        DAE::Mod::NOMOD { .. } => true,
        _ => false,
    });
    outIsNoMod
}

pub(crate) fn getModInfo(mut inMod: &metamodelica::Ref<DAE::Mod>) -> SourceInfo {
    let mut outInfo: SourceInfo;
    outInfo = (match &**inMod {
        DAE::Mod::MOD { info: __inMod_info, .. } => __inMod_info.clone(),
        DAE::Mod::REDECL {
            element: __inMod_element,
            ..
        } => SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&__inMod_element)),
        _ => Absyn::dummyInfo.clone(),
    });
    outInfo
}

pub(crate) fn isRedeclareMod(mut inMod: &metamodelica::Ref<DAE::Mod>) -> bool {
    let mut yes: bool;
    yes = (match &**inMod {
        DAE::Mod::REDECL { .. } => true,
        _ => false,
    });
    yes
}

pub(crate) fn getClassModifier(mut inEnv: &FCore::Graph, mut inName: ArcStr) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut n: metamodelica::Ref<FCore::Node> = <metamodelica::Ref<FCore::Node> as ::std::default::Default>::default();
    let mut r#mod: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = inName.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r#mod: metamodelica::Ref<DAE::Mod> = r#mod.clone();
            let mut n: metamodelica::Ref<FCore::Node> = n.clone();
            n = FNode::fromRef(FNode::child(FGraph::lastScopeRef(inEnv)?, inName.clone())?);
            if !(FNode::isInstance(&(FNode::fromRef(FGraph::lastScopeRef(inEnv)?)))) {
                let __pa0 = ::match_deref::match_deref! { match &(n.clone()) {
                    Deref @ FCore::Node { data: Deref @ FCore::Data::CL { r#mod: __pa0, .. }, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                r#mod = metamodelica::Own::own(__pa0);
                r#mod = removeMod(r#mod.clone(), &inName)?;
            } else {
                r#mod = openmodelica_frontend_types::DAE::Mod::interned_NOMOD();
            }
            Ok((r#mod.clone(), r#mod.clone(), n.clone()))
        })() {
            r#mod = __wb0;
            n = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outMod
}

fn subModValue(mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let __pa0 = ::match_deref::match_deref! { match &((*inSubMod)) {
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsValue: Some(__pa0), .. }), .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outValue = metamodelica::Own::own(__pa0);
    Ok(outValue)
}

fn subModName(mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> ArcStr {
    let mut outName: ArcStr;
    let __arc1 = &(*inSubMod);
    let DAE::NAMEMOD { ident: __pa0, .. } = &**__arc1;
    outName = metamodelica::Own::own(__pa0);
    outName
}

fn subModIsNamed(mut inName: &ArcStr, mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> bool {
    let mut outNameEq: bool;
    outNameEq = metamodelica::stringEq(&inName, &(subModName(inSubMod)));
    outNameEq
}

fn subModInfo(mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> SourceInfo {
    let mut outInfo: SourceInfo;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let __arc1 = &(*inSubMod);
    let DAE::NAMEMOD { r#mod: __pa0, .. } = &**__arc1;
    r#mod = metamodelica::Own::own(__pa0);
    outInfo = getModInfo(&r#mod);
    outInfo
}

fn setEqMod(mut inEqMod: Option<DAE::EqMod>, mut inMod: metamodelica::Ref<DAE::Mod>) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = inMod;
    outMod = (match &*outMod {
        DAE::Mod::MOD { .. } => {
            assign_variant_field!(outMod => DAE::Mod::MOD; binding = inEqMod);
            outMod
        }
        _ => outMod,
    });
    outMod
}

fn stripSubModBindings(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> metamodelica::List<metamodelica::Ref<DAE::SubMod>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
    let mut id: ArcStr;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    for mut submod in &**inSubMods {
        let __arc2 = submod.clone();
        let DAE::NAMEMOD {
            ident: __pa0,
            r#mod: __pa1,
        } = &*__arc2;
        id = metamodelica::Own::own(__pa0);
        r#mod = metamodelica::Own::own(__pa1);
        r#mod = setEqMod(None, r#mod);
        if !(isEmptyMod(&r#mod)) {
            outSubMods = metamodelica::cons(
                metamodelica::Ref::new(DAE::SubMod {
                    ident: id,
                    r#mod: r#mod,
                }),
                outSubMods,
            );
        }
    }
    outSubMods = outSubMods.reverse();
    outSubMods
}

pub(crate) fn filterRedeclares(mut inMod: metamodelica::Ref<DAE::Mod>) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod> = inMod;
    outMod = (match &*outMod {
        DAE::Mod::MOD {
            subModLst: __outMod_subModLst,
            ..
        } => {
            assign_variant_field!(outMod => DAE::Mod::MOD;
                subModLst = filterRedeclaresSubMods(metamodelica::AsArg::as_arg(&__outMod_subModLst)),
                binding = None
            );
            if ((var_field!((*outMod).subModLst, DAE::Mod::MOD)).is_empty()) {
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD()
            } else {
                outMod
            }
        }
        _ => outMod,
    });
    outMod
}

fn filterRedeclaresSubMods(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> metamodelica::List<metamodelica::Ref<DAE::SubMod>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = metamodelica::nil();
    let mut id: ArcStr;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    for mut submod in &**inSubMods {
        let __arc2 = submod.clone();
        let DAE::NAMEMOD {
            ident: __pa0,
            r#mod: __pa1,
        } = &*__arc2;
        id = metamodelica::Own::own(__pa0);
        r#mod = metamodelica::Own::own(__pa1);
        r#mod = filterRedeclares(r#mod);
        if isRedeclareMod(&r#mod) {
            outSubMods = metamodelica::cons(
                metamodelica::Ref::new(DAE::SubMod {
                    ident: id,
                    r#mod: r#mod,
                }),
                outSubMods,
            );
        }
    }
    outSubMods = outSubMods.reverse();
    outSubMods
}

pub(crate) fn unparseModStr(mut inMod: &metamodelica::Ref<DAE::Mod>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inMod {
        DAE::Mod::NOMOD { .. } => {
            literal!("")
        }
        DAE::Mod::MOD {
            binding: __inMod_binding,
            eachPrefix: __inMod_eachPrefix,
            finalPrefix: __inMod_finalPrefix,
            subModLst: __inMod_subModLst,
            ..
        } => {
            let mut final_str: ArcStr;
            let mut each_str: ArcStr;
            let mut sub_str: ArcStr;
            let mut binding_str: ArcStr;
            final_str = if (SCodeUtil::finalBool(__inMod_finalPrefix.clone())) {
                literal!("final ")
            } else {
                literal!("")
            };
            each_str = if (SCodeUtil::eachBool(__inMod_eachPrefix.clone())) {
                literal!("each ")
            } else {
                literal!("")
            };
            sub_str = List::toStringCustom(
                __inMod_subModLst.clone(),
                &move |__a0: metamodelica::Ref<DAE::SubMod>| unparseSubModStr(&__a0),
                literal!(""),
                literal!("("),
                literal!(", "),
                literal!(")"),
                false,
                0,
            )?;
            binding_str = unparseBindingStr(__inMod_binding.clone())?;
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*final_str);
                __mm_s.push_str(&*each_str);
                __mm_s.push_str(&*sub_str);
                __mm_s.push_str(&*binding_str);
                ArcStr::from(__mm_s)
            }
        }
        DAE::Mod::REDECL {
            eachPrefix: __inMod_eachPrefix,
            element: __inMod_element,
            finalPrefix: __inMod_finalPrefix,
            ..
        } => {
            let mut final_str: ArcStr;
            let mut each_str: ArcStr;
            let mut el_str: ArcStr;
            final_str = if (SCodeUtil::finalBool(__inMod_finalPrefix.clone())) {
                literal!("final ")
            } else {
                literal!("")
            };
            each_str = if (SCodeUtil::eachBool(__inMod_eachPrefix.clone())) {
                literal!("each ")
            } else {
                literal!("")
            };
            el_str = SCodeDump::unparseElementStr(__inMod_element.clone(), SCodeDump::defaultOptions.clone())?;
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*final_str);
                __mm_s.push_str(&*each_str);
                __mm_s.push_str(&*literal!("redeclare "));
                __mm_s.push_str(&*el_str);
                ArcStr::from(__mm_s)
            }
        }
    });
    Ok(outString)
}

fn unparseSubModStr(mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inSubMod {
        DAE::SubMod { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*inSubMod.ident);
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*unparseModStr(&inSubMod.r#mod)?);
            ArcStr::from(__mm_s)
        }
    });
    Ok(outString)
}

fn unparseBindingStr(mut inBinding: Option<DAE::EqMod>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inBinding {
        None => {
            literal!("")
        }
        Some(DAE::EqMod::TYPED {
            modifierAsAbsynExp: ref exp,
            ..
        }) => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*Dump::printExpStr(exp.clone())?);
            ArcStr::from(__mm_s)
        }
        Some(DAE::EqMod::UNTYPED { exp: mut exp }) => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*Dump::printExpStr(exp.clone())?);
            ArcStr::from(__mm_s)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}
