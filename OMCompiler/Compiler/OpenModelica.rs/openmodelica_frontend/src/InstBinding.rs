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
use crate::InnerOuter;
use crate::InstSection;
use crate::InstUtil;
use crate::Mod;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

pub type InstDims = metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;

thread_local! { static __stateSelectType_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("never"), literal!("avoid"), literal!("default"), literal!("prefer"), literal!("always")], literalVarLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("never"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(1), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("never"), literal!("avoid"), literal!("default"), literal!("prefer"), literal!("always")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("avoid"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(2), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("never"), literal!("avoid"), literal!("default"), literal!("prefer"), literal!("always")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("default"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(3), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("never"), literal!("avoid"), literal!("default"), literal!("prefer"), literal!("always")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("prefer"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(4), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("never"), literal!("avoid"), literal!("default"), literal!("prefer"), literal!("always")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("always"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(5), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("never"), literal!("avoid"), literal!("default"), literal!("prefer"), literal!("always")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], attributeLst: metamodelica::nil() }); }
pub(crate) fn stateSelectType() -> metamodelica::Ref<DAE::Type> {
    __stateSelectType_TLS.with(|__t| __t.clone())
}

thread_local! { static __uncertaintyType_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("given"), literal!("sought"), literal!("refine"), literal!("propagate")], literalVarLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("given"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(1), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("given"), literal!("sought"), literal!("refine"), literal!("propagate")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("sought"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(2), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("given"), literal!("sought"), literal!("refine"), literal!("propagate")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("refine"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(3), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("given"), literal!("sought"), literal!("refine"), literal!("propagate")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("propagate"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(4), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("given"), literal!("sought"), literal!("refine"), literal!("propagate")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], attributeLst: metamodelica::nil() }); }
pub(crate) fn uncertaintyType() -> metamodelica::Ref<DAE::Type> {
    __uncertaintyType_TLS.with(|__t| __t.clone())
}

thread_local! { static __distributionType_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Distribution") }) }, varLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("name"), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("params"), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: DAE::T_ARRAY_REAL_NODIM().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("paramNames"), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: DAE::T_ARRAY_STRING_NODIM().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub(crate) fn distributionType() -> metamodelica::Ref<DAE::Type> {
    __distributionType_TLS.with(|__t| __t.clone())
}

fn instBinding(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inVarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inIntegerLst: metamodelica::List<i32>,
    mut inString: ArcStr,
    mut useConstValue: bool,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpOption: Option<metamodelica::Ref<DAE::Exp>>;
    outExpExpOption = 'mc: {
        let __mc_input = (inMod, &**inVarLst, inType, inIntegerLst, inString);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, expected_type, Deref @ metamodelica::ListNode::Nil, bind_name) => {
                    let mut mod2: metamodelica::Ref<DAE::Mod>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut optVal: Option<metamodelica::Ref<Values::Value>>;
                    mod2 = Mod::lookupCompModification(metamodelica::AsArg::as_arg(&r#mod), bind_name.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Mod::modEquation(&mod2)) {
                        Some(DAE::EqMod::TYPED { modifierAsExp: __pa0, modifierAsValue: __pa1, properties: DAE::Properties::PROP { type_: __pa2, constFlag: _ }, modifierAsAbsynExp: _, .. }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    optVal = metamodelica::Own::own(__pa1);
                    ty2 = metamodelica::Own::own(__pa2);
                    (e_1, _) = Types::matchType(e.clone(), ty2.clone(), expected_type.clone(), true)?;
                    e_1 = InstUtil::checkUseConstValue(useConstValue, e_1.clone(), optVal.clone());
                    Ok(Some(e_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, etype, index_list, bind_name) => {
                    let mut mod2: metamodelica::Ref<DAE::Mod>;
                    let mut result: Option<metamodelica::Ref<DAE::Exp>>;
                    mod2 = Mod::lookupCompModification(metamodelica::AsArg::as_arg(&r#mod), bind_name.clone())?;
                    result = instBinding2(mod2.clone(), etype.clone(), metamodelica::AsArg::as_arg(&index_list), bind_name.clone(), useConstValue)?;
                    Ok(result.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, _, Deref @ metamodelica::ListNode::Nil, bind_name) => {
                    if '__try0: {
                        unwrap_break_err!(Mod::lookupCompModification(metamodelica::AsArg::as_arg(&r#mod), bind_name.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name, binding, .. }, tail: _ }, _, _, bind_name) => {
                    let true = (stringEq(&name, &bind_name)) else { return Err("pattern mismatch") };
                    Ok(DAEUtil::bindingExp(metamodelica::AsArg::as_arg(&binding))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, Deref @ metamodelica::ListNode::Cons { head: _, tail: varLst }, etype, index_list, bind_name) => {
                    Ok(instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), etype.clone(), index_list.clone(), bind_name.clone(), useConstValue)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpExpOption)
}

fn instBinding2(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inIntegerLst: &metamodelica::List<i32>,
    mut inString: ArcStr,
    mut useConstValue: bool,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpOption: Option<metamodelica::Ref<DAE::Exp>>;
    outExpExpOption = (::match_deref::match_deref! { match inIntegerLst {
        Deref @ metamodelica::ListNode::Cons { head: index, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut r#mod = inMod;
            let mut etype = inType;
            let mut mod2: metamodelica::Ref<DAE::Mod>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut ty2: metamodelica::Ref<DAE::Type>;
            let mut optVal: Option<metamodelica::Ref<Values::Value>>;
            mod2 = Mod::lookupIdxModification(&r#mod, metamodelica::Ref::new(DAE::Exp::ICONST { integer: index.clone() }))?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Mod::modEquation(&mod2)) {
                Some(DAE::EqMod::TYPED { modifierAsExp: __pa0, modifierAsValue: __pa1, properties: DAE::Properties::PROP { type_: __pa2, constFlag: _ }, modifierAsAbsynExp: _, .. }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            optVal = metamodelica::Own::own(__pa1);
            ty2 = metamodelica::Own::own(__pa2);
            (e_1, _) = Types::matchType(e, ty2, etype, true)?;
            e_1 = InstUtil::checkUseConstValue(useConstValue, e_1, optVal);
            Some(e_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: index, tail: res } => {
            let mut r#mod = inMod;
            let mut etype = inType;
            let mut bind_name = inString;
            let mut mod2: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
            let mut result: Option<metamodelica::Ref<DAE::Exp>> = None;
            result = 'mc: {
        let __mc_input = ();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut mod2: metamodelica::Ref<DAE::Mod> = mod2.clone();
            let mut result: Option<metamodelica::Ref<DAE::Exp>>;
            mod2 = Mod::lookupIdxModification(&r#mod, metamodelica::Ref::new(DAE::Exp::ICONST { integer: index.clone() }))?;
            result = instBinding2(mod2.clone(), etype.clone(), res, bind_name.clone(), useConstValue)?;
            Ok((result.clone(), mod2.clone()))
        })() { mod2 = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(None)
        })() { break 'mc __v; }
        return Err("matchcontinue: no arm matched")
    };
            result
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExpExpOption)
}

pub(crate) fn instStartBindingExp(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inExpectedType: &metamodelica::Ref<DAE::Type>,
    mut inVariability: SCode::Variability,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outStartValue: Option<metamodelica::Ref<DAE::Exp>>;
    if SCodeUtil::isConstant(inVariability) {
        outStartValue = None;
    } else {
        outStartValue = instBinding(
            inMod,
            &(metamodelica::nil()),
            Types::arrayElementType(inExpectedType),
            metamodelica::nil(),
            literal!("start"),
            false,
        )?;
    }
    Ok(outStartValue)
}

fn instStartOrigin(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inVarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inString: ArcStr,
) -> Result<Option<DAE::StartOrigin>> {
    let mut outStartOrigin: Option<DAE::StartOrigin>;
    outStartOrigin = 'mc: {
        let __mc_input = (inMod, &**inVarLst, inString);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, bind_name) => {
                    let mut mod2: metamodelica::Ref<DAE::Mod>;
                    mod2 = Mod::lookupCompModification(metamodelica::AsArg::as_arg(&r#mod), bind_name.clone())?;
                    ::match_deref::match_deref! { match &(Mod::modEquation(&mod2)) {
                        Some(_) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(Some(openmodelica_frontend_types::DAE::StartOrigin::BINDING_ORIGIN))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name, .. }, tail: _ }, bind_name) => {
                    let true = (stringEq(&name, &bind_name)) else { return Err("pattern mismatch") };
                    Ok(Some(openmodelica_frontend_types::DAE::StartOrigin::TYPE_ORIGIN))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, Deref @ metamodelica::ListNode::Cons { head: _, tail: varLst }, bind_name) => {
                    Ok(instStartOrigin(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), bind_name.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStartOrigin)
}

pub(crate) fn instDaeVariableAttributes(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inIntegerLst: metamodelica::List<i32>,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<DAE::VariableAttributes>>)> {
    let mut outCache: FCore::Cache;
    let mut outDAEVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    (outCache, outDAEVariableAttributesOption) = 'mc: {
        let __mc_input = (inCache, inMod, inType, inIntegerLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, r#mod, Deref @ DAE::Type::T_REAL { varLst }, index_list) => {
                    let mut quantity_str: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut unit_str: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut displayunit_str: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut nominal_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut fixed_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_bind_select: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_bind_uncertainty: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut min_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut max_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut start_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut startOrigin: Option<DAE::StartOrigin>;
                    let mut stateSelect_value: Option<DAE::StateSelect>;
                    let mut uncertainty_value: Option<DAE::Uncertainty>;
                    let mut distribution_value: Option<metamodelica::Ref<DAE::Distribution>>;
                    quantity_str = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), literal!("quantity"), false)?;
                    unit_str = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), literal!("unit"), false)?;
                    displayunit_str = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), literal!("displayUnit"), false)?;
                    min_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), literal!("min"), false)?;
                    max_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), literal!("max"), false)?;
                    start_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), literal!("start"), false)?;
                    fixed_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), literal!("fixed"), true)?;
                    nominal_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), literal!("nominal"), false)?;
                    exp_bind_select = instEnumerationBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), index_list.clone(), literal!("stateSelect"), stateSelectType().clone(), true)?;
                    stateSelect_value = InstUtil::getStateSelectFromExpOption(exp_bind_select.clone());
                    exp_bind_uncertainty = instEnumerationBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), index_list.clone(), literal!("uncertain"), uncertaintyType().clone(), true)?;
                    uncertainty_value = getUncertainFromExpOption(exp_bind_uncertainty.clone());
                    distribution_value = instDistributionBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), index_list.clone(), literal!("distribution"), false);
                    startOrigin = instStartOrigin(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), literal!("start"))?;
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity_str.clone(), unit: unit_str.clone(), displayUnit: displayunit_str.clone(), min: min_val.clone(), max: max_val.clone(), start: start_val.clone(), fixed: fixed_val.clone(), nominal: nominal_val.clone(), stateSelectOption: stateSelect_value.clone(), uncertainOption: uncertainty_value.clone(), distributionOption: distribution_value.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, r#mod, Deref @ DAE::Type::T_INTEGER { varLst }, index_list) => {
                    let mut quantity_str: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut fixed_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_bind_uncertainty: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut min_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut max_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut start_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut startOrigin: Option<DAE::StartOrigin>;
                    let mut uncertainty_value: Option<DAE::Uncertainty>;
                    let mut distribution_value: Option<metamodelica::Ref<DAE::Distribution>>;
                    quantity_str = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), literal!("quantity"), false)?;
                    min_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_INTEGER_DEFAULT().clone(), index_list.clone(), literal!("min"), false)?;
                    max_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_INTEGER_DEFAULT().clone(), index_list.clone(), literal!("max"), false)?;
                    start_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_INTEGER_DEFAULT().clone(), index_list.clone(), literal!("start"), false)?;
                    fixed_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), literal!("fixed"), true)?;
                    exp_bind_uncertainty = instEnumerationBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), index_list.clone(), literal!("uncertain"), uncertaintyType().clone(), true)?;
                    uncertainty_value = getUncertainFromExpOption(exp_bind_uncertainty.clone());
                    distribution_value = instDistributionBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), index_list.clone(), literal!("distribution"), false);
                    startOrigin = instStartOrigin(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), literal!("start"))?;
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity_str.clone(), min: min_val.clone(), max: max_val.clone(), start: start_val.clone(), fixed: fixed_val.clone(), uncertainOption: uncertainty_value.clone(), distributionOption: distribution_value.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, r#mod, tp @ Deref @ DAE::Type::T_BOOL { varLst }, index_list) => {
                    let mut quantity_str: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut fixed_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut start_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut startOrigin: Option<DAE::StartOrigin>;
                    quantity_str = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), literal!("quantity"), false)?;
                    start_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), tp.clone(), index_list.clone(), literal!("start"), false)?;
                    fixed_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), tp.clone(), index_list.clone(), literal!("fixed"), true)?;
                    startOrigin = instStartOrigin(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), literal!("start"))?;
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity_str.clone(), start: start_val.clone(), fixed: fixed_val.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Type::T_CLOCK { .. }, _) => {
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: None }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, r#mod, tp @ Deref @ DAE::Type::T_STRING { varLst }, index_list) => {
                    let mut quantity_str: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut fixed_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut start_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut startOrigin: Option<DAE::StartOrigin>;
                    quantity_str = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), tp.clone(), index_list.clone(), literal!("quantity"), false)?;
                    start_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), tp.clone(), index_list.clone(), literal!("start"), false)?;
                    fixed_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), literal!("fixed"), true)?;
                    startOrigin = instStartOrigin(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), literal!("start"))?;
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity_str.clone(), start: start_val.clone(), fixed: fixed_val.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, r#mod, enumtype @ Deref @ DAE::Type::T_ENUMERATION { attributeLst: varLst, .. }, index_list) => {
                    let mut quantity_str: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut fixed_val: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_bind_min: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_bind_max: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut exp_bind_start: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut startOrigin: Option<DAE::StartOrigin>;
                    quantity_str = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), literal!("quantity"), false)?;
                    exp_bind_min = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), enumtype.clone(), index_list.clone(), literal!("min"), false)?;
                    exp_bind_max = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), enumtype.clone(), index_list.clone(), literal!("max"), false)?;
                    exp_bind_start = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), enumtype.clone(), index_list.clone(), literal!("start"), false)?;
                    fixed_val = instBinding(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), literal!("fixed"), true)?;
                    startOrigin = instStartOrigin(r#mod.clone(), metamodelica::AsArg::as_arg(&varLst), literal!("start"))?;
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity_str.clone(), min: exp_bind_min.clone(), max: exp_bind_max.clone(), start: exp_bind_start.clone(), fixed: fixed_val.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _) => {
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outDAEVariableAttributesOption))
}

fn instEnumerationBinding(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut varLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inIndices: metamodelica::List<i32>,
    mut inName: ArcStr,
    mut expected_type: metamodelica::Ref<DAE::Type>,
    mut useConstValue: bool,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outBinding: Option<metamodelica::Ref<DAE::Exp>>;
    if let Ok(__iflet0) = instBinding(
        inMod.clone(),
        varLst,
        expected_type.clone(),
        inIndices.clone(),
        inName.clone(),
        useConstValue,
    ) {
        outBinding = __iflet0;
    } else {
        Error::addMessage(
            Error::TYPE_ERROR.clone(),
            list![inName.clone(), literal!("enumeration type")],
        )?;
        return Err("fail");
    }
    Ok(outBinding)
}

fn instDistributionBinding(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut varLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inIntegerLst: metamodelica::List<i32>,
    mut inString: ArcStr,
    mut useConstValue: bool,
) -> Option<metamodelica::Ref<DAE::Distribution>> {
    let mut out: Option<metamodelica::Ref<DAE::Distribution>>;
    out = 'mc: {
        let __mc_input = (inMod, inIntegerLst, inString);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, index_list, bind_name) => {
                    let mut name: metamodelica::Ref<DAE::Exp>;
                    let mut params: metamodelica::Ref<DAE::Exp>;
                    let mut paramNames: metamodelica::Ref<DAE::Exp>;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(instBinding(r#mod.clone(), varLst, distributionType().clone(), index_list.clone(), bind_name.clone(), useConstValue)?) {
                        Some(Deref @ DAE::Exp::CALL { path: __pa0, expLst: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    name = metamodelica::Own::own(__pa1);
                    params = metamodelica::Own::own(__pa2);
                    paramNames = metamodelica::Own::own(__pa3);
                    let true = (AbsynUtil::pathEqual(&path, &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Distribution") })))) else { return Err("pattern mismatch") };
                    Ok(Some(metamodelica::Ref::new(DAE::Distribution { name: name.clone(), params: params.clone(), paramNames: paramNames.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, index_list, bind_name) => {
                    let mut name: metamodelica::Ref<DAE::Exp>;
                    let mut params: metamodelica::Ref<DAE::Exp>;
                    let mut paramNames: metamodelica::Ref<DAE::Exp>;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(instBinding(r#mod.clone(), varLst, distributionType().clone(), index_list.clone(), bind_name.clone(), useConstValue)?) {
                        Some(Deref @ DAE::Exp::RECORD { path: __pa0, exps: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    name = metamodelica::Own::own(__pa1);
                    params = metamodelica::Own::own(__pa2);
                    paramNames = metamodelica::Own::own(__pa3);
                    let true = (AbsynUtil::pathEqual(&path, &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Distribution") })))) else { return Err("pattern mismatch") };
                    Ok(Some(metamodelica::Ref::new(DAE::Distribution { name: name.clone(), params: params.clone(), paramNames: paramNames.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, index_list, bind_name) => {
                    let mut name: metamodelica::Ref<DAE::Exp>;
                    let mut params: metamodelica::Ref<DAE::Exp>;
                    let mut paramNames: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut paramDim: i32;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crName: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crParams: metamodelica::Ref<DAE::ComponentRef>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(instBinding(r#mod.clone(), varLst, distributionType().clone(), index_list.clone(), bind_name.clone(), useConstValue)?) {
                        Some(Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = metamodelica::Own::own(__pa0);
                    ty = metamodelica::Own::own(__pa1);
                    let true = (Types::isRecord(&ty)) else { return Err("pattern mismatch") };
                    let __pa2 = ::match_deref::match_deref! { match &(ty.clone()) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: __pa2 }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, tail: _ } }, .. } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    paramDim = metamodelica::Own::own(__pa2);
                    crName = ComponentReference::crefPrependIdent(&cr, &(literal!("name")), &(metamodelica::nil()), &(DAE::T_STRING_DEFAULT().clone()))?;
                    crParams = ComponentReference::crefPrependIdent(&cr, &(literal!("params")), &(metamodelica::nil()), &(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: paramDim })] })))?;
                    name = Expression::makeCrefExp(crName.clone(), DAE::T_STRING_DEFAULT().clone())?;
                    params = Expression::makeCrefExp(crParams.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: paramDim })] }))?;
                    paramNames = Expression::makeCrefExp(crParams.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: paramDim })] }))?;
                    Ok(Some(metamodelica::Ref::new(DAE::Distribution { name: name.clone(), params: params.clone(), paramNames: paramNames.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    out
}

fn getUncertainFromExpOption(mut expOption: Option<metamodelica::Ref<DAE::Exp>>) -> Option<DAE::Uncertainty> {
    let mut out: Option<DAE::Uncertainty>;
    out = (::match_deref::match_deref! { match &(expOption) {
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Uncertainty", path: Deref @ Absyn::Path::IDENT { name: Deref @ "given" } }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::GIVEN),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Uncertainty", path: Deref @ Absyn::Path::IDENT { name: Deref @ "sought" } }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::SOUGHT),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Uncertainty", path: Deref @ Absyn::Path::IDENT { name: Deref @ "refine" } }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::REFINE),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Uncertainty", path: Deref @ Absyn::Path::IDENT { name: Deref @ "propagate" } }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::PROPAGATE),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub(crate) fn instModEquation(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inImpl: bool,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = (&*inType, &**inMod);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: _, modifierAsValue: Some(_), properties: DAE::Properties::PROP { type_: _, constFlag: DAE::Const::C_CONST { .. } }, modifierAsAbsynExp: _, .. }), .. }) => {
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { properties: prop2, .. }), .. }) => {
                    ::match_deref::match_deref! { match &(Types::getPropType(metamodelica::AsArg::as_arg(&prop2))) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: prop2, modifierAsAbsynExp: aexp2, .. }), info, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut aexp1: metamodelica::Ref<Absyn::Exp>;
                    let mut scode: metamodelica::Ref<SCode::Equation>;
                    let mut acr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    t = Types::simplifyType(inType.clone())?;
                    lhs = Expression::makeCrefExp(inComponentRef.clone(), t.clone())?;
                    acr = ComponentReference::unelabCref(&inComponentRef)?;
                    aexp1 = metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: acr.clone() });
                    scode = metamodelica::Ref::new(SCode::Equation::EQ_EQUALS { expLeft: aexp1.clone(), expRight: aexp2.clone(), comment: SCode::noComment.clone(), info: info.clone() });
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::FLATTEN { scode: scode.clone(), dae: None }))?;
                    dae = InstSection::instEqEquation(lhs.clone(), DAE::Properties::PROP { type_: inType.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, e.clone(), prop2.clone(), source.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, inImpl, info.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::MOD { binding: None, .. }) => {
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::NOMOD { .. }) => {
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::REDECL { .. }) => {
                    Ok(DAE::emptyDae().clone())
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
                    Debug::trace(literal!("- InstBinding.instModEquation failed\n type: "))?;
                    Debug::trace(TypesDump::printTypeStr(inType.clone()))?;
                    Debug::trace(literal!("\n  cref: "))?;
                    Debug::trace(ComponentReferenceBasics::printComponentRefStr(&inComponentRef)?)?;
                    Debug::trace(literal!("\n mod:"))?;
                    Debug::traceln(Mod::printModStr(inMod)?)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDae)
}

pub(crate) fn makeBinding(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inAttributes: &SCode::Attributes,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inPrefix: &DAE::Prefix,
    mut componentName: &ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Binding>)> {
    let mut outCache: FCore::Cache;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    (outCache, outBinding) = 'mc: {
        let __mc_input = (inCache.clone(), inAttributes, &**inMod, inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::NOMOD { .. }, _) => {
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut complex_vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut tpath: metamodelica::Ref<Absyn::Path>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::arrayElementType(&inType)) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, varLst: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    tpath = metamodelica::Own::own(__pa0);
                    complex_vars = metamodelica::Own::own(__pa1);
                    let true = (Types::allHaveBindings(&complex_vars)?) else { return Err("pattern mismatch") };
                    binding = makeRecordBinding(metamodelica::AsArg::as_arg(&cache), inEnv, tpath.clone(), &inType, &complex_vars, metamodelica::nil(), inInfo)?;
                    Ok((cache.clone(), binding.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::NOMOD { .. }, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::Binding::interned_UNBOUND()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Mod::REDECL { .. }, _) => {
                    Ok(makeBinding(inCache.clone(), inEnv, inAttributes, var_field!((**inMod).r#mod, DAE::Mod::REDECL), inType.clone(), inPrefix, componentName, inInfo)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, SCode::Attributes { variability: SCode::Variability::PARAM { .. }, .. }, Deref @ DAE::Mod::MOD { binding: None, .. }, tp) => {
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut startValueModification: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let true = (Types::getFixedVarAttributeParameterOrConstant(metamodelica::AsArg::as_arg(&tp))) else { return Err("pattern mismatch") };
                    startValueModification = Mod::lookupCompModification(inMod, literal!("start"))?;
                    let false = (Mod::isEmptyMod(&startValueModification)) else { return Err("pattern mismatch") };
                    (cache, binding) = makeBinding(cache.clone(), inEnv, inAttributes, &startValueModification, inType.clone(), inPrefix, componentName, inInfo)?;
                    binding = DAEUtil::setBindingSource(binding.clone(), openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_START_VALUE);
                    Ok((cache.clone(), binding.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { subModLst: sub_mods @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, _) => {
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut complex_vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut tpath: metamodelica::Ref<Absyn::Path>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::arrayElementType(&inType)) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, varLst: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    tpath = metamodelica::Own::own(__pa0);
                    complex_vars = metamodelica::Own::own(__pa1);
                    binding = makeRecordBinding(metamodelica::AsArg::as_arg(&cache), inEnv, tpath.clone(), &inType, &complex_vars, sub_mods.clone(), inInfo)?;
                    Ok((cache.clone(), binding.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { binding: None, .. }, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::Binding::interned_UNBOUND()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: Some(v), properties: prop, modifierAsAbsynExp: _, .. }), .. }, e_tp) => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_val_exp: metamodelica::Ref<DAE::Exp>;
                    let mut e_val: Option<metamodelica::Ref<Values::Value>>;
                    let mut c: DAE::Const;
                    let mut v = (*v).clone();
                    c = Types::propAllConst(prop.clone())?;
                    tp = Types::getPropType(metamodelica::AsArg::as_arg(&prop));
                    let false = (Types::equivtypes(tp.clone(), e_tp.clone())) else { return Err("pattern mismatch") };
                    e_val_exp = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    (e_1, _) = Types::matchType(e.clone(), tp.clone(), e_tp.clone(), false)?;
                    (e_1, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    (e_val_exp, _) = Types::matchType(e_val_exp.clone(), tp.clone(), e_tp.clone(), false)?;
                    (e_val_exp, _) = ExpressionSimplify::simplify(e_val_exp.clone())?;
                    v = Ceval::cevalSimple(e_val_exp.clone())?;
                    e_val = Some(v.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: e_1.clone(), evaluatedExp: e_val.clone(), constant_: c, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: e_val, properties: prop, modifierAsAbsynExp: _, .. }), .. }, e_tp) => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut c: DAE::Const;
                    c = Types::propAllConst(prop.clone())?;
                    tp = Types::getPropType(metamodelica::AsArg::as_arg(&prop));
                    (e_1, _) = Types::matchType(e.clone(), tp.clone(), e_tp.clone(), false)?;
                    (e_1, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: e_1.clone(), evaluatedExp: e_val.clone(), constant_: c, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: prop, modifierAsAbsynExp: _, .. }), info, .. }, tp) => {
                    let mut e_tp: metamodelica::Ref<DAE::Type>;
                    let mut e_tp_str: ArcStr;
                    let mut tp_str: ArcStr;
                    let mut e_str: ArcStr;
                    let mut e_str_1: ArcStr;
                    let mut r#str: ArcStr;
                    e_tp = Types::getPropType(metamodelica::AsArg::as_arg(&prop));
                    if '__try0: {
                        unwrap_break_err!(Types::matchType(e.clone(), e_tp.clone(), tp.clone(), false), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    e_tp_str = TypesDump::unparseTypeNoAttr(&e_tp)?;
                    tp_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&tp))?;
                    e_str = ExpressionBasics::printExpStr(e.clone())?;
                    e_str_1 = stringAppend(literal!("="), e_str.clone());
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(inPrefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*componentName); ArcStr::from(__mm_s) };
                    Types::typeErrorSanityCheck(e_tp_str.clone(), &tp_str, metamodelica::AsArg::as_arg(&info))?;
                    Error::addSourceMessage(&(Error::MODIFIER_TYPE_MISMATCH_ERROR.clone()), list![r#str.clone(), tp_str.clone(), e_str_1.clone(), e_tp_str.clone()], metamodelica::AsArg::as_arg(&info))?;
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.makeBinding failed on component:")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(inPrefix)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*componentName); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outBinding))
}

pub(crate) fn makeRecordBinding(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inRecordName: metamodelica::Ref<Absyn::Path>,
    mut inRecordType: &metamodelica::Ref<DAE::Type>,
    mut inRecordVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inMods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Binding>> {
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut accum_exps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut accum_vals: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut accum_names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut mods: metamodelica::List<metamodelica::Ref<DAE::SubMod>> = inMods;
    let mut opt_mod: Option<metamodelica::Ref<DAE::SubMod>>;
    let mut name: ArcStr = literal!("");
    let mut scope: ArcStr;
    let mut ty_str: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ety: metamodelica::Ref<DAE::Type>;
    let mut binding: metamodelica::Ref<DAE::Binding>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut val: metamodelica::Ref<Values::Value>;
    dims = TypesDump::getDimensions(inRecordType);
    match '__try0: {
        for mut var in &**inRecordVars {
            let __arc4 = var.clone();
            let DAE::TYPES_VAR {
                name: __pa1,
                ty: __pa2,
                binding: __pa3,
                ..
            } = &*__arc4;
            name = metamodelica::Own::own(__pa1);
            ty = metamodelica::Own::own(__pa2);
            binding = metamodelica::Own::own(__pa3);
            (mods, opt_mod) = unwrap_break_err!(List::deleteMemberOnTrue(name.clone(), mods.clone(), &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(InstUtil::isSubModNamed(&__a0, &__a1)) }), '__try0);
            if (opt_mod).is_some() {
                ty = Types::liftArrayListDims(ty.clone(), dims.clone());
                (exp, val) = unwrap_break_err!(makeRecordBinding3(opt_mod.clone(), ty.clone(), inInfo), '__try0);
            } else if DAEUtil::isBound(&binding) {
                let (__pa5, __pa6) = ::match_deref::match_deref! { match &(binding.clone()) {
                    Deref @ DAE::Binding::EQBOUND { exp: __pa5, evaluatedExp: Some(__pa6), .. } => (__pa5.clone(), __pa6.clone()),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa5);
                val = metamodelica::Own::own(__pa6);
            } else {
                ety = unwrap_break_err!(Types::simplifyType(ty.clone()), '__try0);
                ty = Types::liftArrayListDims(ty.clone(), dims.clone());
                scope = FGraph::printGraphPathStr(inEnv);
                ty_str = TypesDump::printTypeStr(ty.clone());
                exp = metamodelica::Ref::new(DAE::Exp::EMPTY {
                    scope: scope.clone(),
                    name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                        ident: name.clone(),
                        identType: ety.clone(),
                        subscriptLst: metamodelica::nil(),
                    }),
                    ty: ety.clone(),
                    tyStr: ty_str.clone(),
                });
                val = metamodelica::Ref::new(Values::Value::EMPTY {
                    scope: scope.clone(),
                    name: name.clone(),
                    ty: unwrap_break_err!(Types::typeToValue(&ty), '__try0),
                    tyStr: ty_str.clone(),
                });
            }
            accum_exps = metamodelica::cons(exp.clone(), accum_exps.clone());
            accum_vals = metamodelica::cons(val.clone(), accum_vals.clone());
            accum_names = metamodelica::cons(name.clone(), accum_names.clone());
        }
        ety = unwrap_break_err!(Types::simplifyType(Types::arrayElementType(inRecordType)), '__try0);
        exp = metamodelica::Ref::new(DAE::Exp::CALL {
            path: inRecordName.clone(),
            expLst: accum_exps.clone().reverse(),
            attr: metamodelica::Ref::new(DAE::CallAttributes {
                ty: ety.clone(),
                tuple_: false,
                builtin: false,
                isImpure: false,
                isFunctionPointerCall: false,
                inlineType: openmodelica_frontend_types::DAE::InlineType::NORM_INLINE,
                tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
                noReturn: DAE::NoReturn::RETURNS.clone(),
            }),
        });
        val = metamodelica::Ref::new(Values::Value::RECORD {
            record_: inRecordName.clone(),
            orderd: accum_vals.clone().reverse(),
            comp: accum_names.clone().reverse(),
            index: -1,
        });
        (exp, val) = unwrap_break_err!(InstUtil::liftRecordBinding(inRecordType, &exp, &val), '__try0);
        outBinding = metamodelica::Ref::new(DAE::Binding::EQBOUND {
            exp: exp.clone(),
            evaluatedExp: Some(val.clone()),
            constant_: openmodelica_frontend_types::DAE::Const::C_CONST,
            source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS,
        });
        Ok::<_, &'static str>((ety.clone(), exp.clone(), outBinding.clone(), val.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            ety = __try0_o0;
            exp = __try0_o1;
            outBinding = __try0_o2;
            val = __try0_o3;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- Inst.makeRecordBinding2 failed for "));
                    __mm_s.push_str(&*AbsynUtil::pathString(
                        inRecordName.clone(),
                        literal!("."),
                        true,
                        false,
                    )?);
                    __mm_s.push_str(&*literal!("."));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
            return Err(__try0_err);
        }
    }
    Ok(outBinding)
}

fn makeRecordBinding3(
    mut inSubMod: Option<metamodelica::Ref<DAE::SubMod>>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<Values::Value>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outExp, outValue) = 'mc: {
        let __mc_input = inSubMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { eachPrefix: SCode::Each::EACH { .. }, binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, modifierAsValue: Some(val), .. }), .. }, .. }) => {
                    Ok((exp.clone(), val.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH { .. }, binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, modifierAsValue: Some(val), properties: DAE::Properties::PROP { type_: ty, .. }, .. }), .. }, .. }) => {
                    let mut exp = (*exp).clone();
                    let mut ty = (*ty).clone();
                    (exp, ty) = Types::matchType(exp.clone(), ty.clone(), inType.clone(), true)?;
                    Ok((exp.clone(), val.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH { .. }, binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, modifierAsValue: None, properties: DAE::Properties::PROP { type_: ty, .. }, .. }), .. }, .. }) => {
                    let mut exp = (*exp).clone();
                    let mut ty = (*ty).clone();
                    (exp, ty) = Types::matchType(exp.clone(), ty.clone(), inType.clone(), true)?;
                    Ok((exp.clone(), metamodelica::Ref::new(Values::Value::OPTION { some: None })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { ident, r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, properties: DAE::Properties::PROP { type_: ty, .. }, .. }), .. } }) => {
                    let mut binding_str: ArcStr;
                    let mut expected_type_str: ArcStr;
                    let mut given_type_str: ArcStr;
                    binding_str = ExpressionBasics::printExpStr(exp.clone())?;
                    expected_type_str = TypesDump::unparseTypeNoAttr(&inType)?;
                    given_type_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&ty))?;
                    Types::typeErrorSanityCheck(given_type_str.clone(), &expected_type_str, inInfo)?;
                    Error::addSourceMessage(&(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone()), list![ident.clone(), binding_str.clone(), expected_type_str.clone(), given_type_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outValue))
}

pub(crate) fn makeVariableBinding(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inConst: DAE::Const,
    mut inPrefix: DAE::Prefix,
    mut inName: ArcStr,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outBinding: Option<metamodelica::Ref<DAE::Exp>>;
    let mut oeq_mod: Option<DAE::EqMod> = Mod::modEquation(inMod);
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut p: DAE::Properties;
    let mut info: SourceInfo;
    let mut c: DAE::Const;
    let mut e_str: ArcStr;
    let mut et_str: ArcStr;
    let mut bt_str: ArcStr;
    if (oeq_mod).is_none() {
        outBinding = None;
        return Ok(outBinding);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(oeq_mod) {
        Some(DAE::EqMod::TYPED { modifierAsExp: __pa0, properties: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    p = metamodelica::Own::own(__pa1);
    if Types::isExternalObject(&inType) {
        outBinding = Some(e);
    } else if Types::isEmptyArray(&(Types::getPropType(&p))) {
        outBinding = None;
    } else {
        info = Mod::getModInfo(inMod);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Types::matchProp(e.clone(), &p, &(DAE::Properties::PROP { type_: inType.clone(), constFlag: inConst }), true)) {
            Ok((__pa2, DAE::Properties::PROP { constFlag: __pa3, .. })) => (__pa2.clone(), __pa3.clone()),
            _ => {
            e_str = ExpressionBasics::printExpStr(e.clone())?;
            et_str = TypesDump::unparseTypeNoAttr(&inType)?;
            bt_str = TypesDump::unparseTypeNoAttr(&(Types::getPropType(&p)))?;
            Types::typeErrorSanityCheck(et_str.clone(), &bt_str, &info)?;
            Error::addSourceMessageAndFail(&(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone()), list![inName.clone(), e_str.clone(), et_str.clone(), bt_str.clone()], &info)?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            },
        } };
        e2 = metamodelica::Own::own(__pa2);
        c = metamodelica::Own::own(__pa3);
        InstUtil::checkHigherVariability(inConst, c, inPrefix, inName, e, &info)?;
        outBinding = Some(e2);
    }
    Ok(outBinding)
}
