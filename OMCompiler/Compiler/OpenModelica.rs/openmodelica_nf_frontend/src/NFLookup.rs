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

use crate::NFAttributes as Attributes;
use crate::NFBuiltin;
use crate::NFBuiltinFuncs;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFLookupState::LookupState;
use crate::NFModifier as Modifier;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::BackendInterface;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum MatchType {
    FOUND = 1,
    NOT_FOUND = 2,
    PARTIAL = 3,
}
impl PartialOrd for MatchType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MatchType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for MatchType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub fn lookupClassName(
    mut name: metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
    mut checkAccessViolations: bool,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut prefixes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    (node, state, prefixes) = lookupNameWithError(
        name.clone(),
        scope,
        context,
        &info,
        &(Error::LOOKUP_ERROR.clone()),
        checkAccessViolations,
    )?;
    LookupState::assertClass(&state, node.clone(), name, context, info)?;
    Ok((node, prefixes))
}

pub(crate) fn lookupBaseClassName(
    mut name: metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    if let Ok((__pa0, __pa1)) = lookupNames(&name, scope.clone(), context) {
        nodes = metamodelica::Own::own(__pa0);
        state = metamodelica::Own::own(__pa1);
    } else {
        Error::addSourceMessage(
            &(Error::LOOKUP_BASECLASS_ERROR.clone()),
            list![
                AbsynUtil::pathString(name.clone(), literal!("."), true, false)?,
                NFInstNode::InstNode::scopeName(&scope)?
            ],
            &info,
        )?;
        return Err("fail");
    }
    LookupState::assertClass(&state, (nodes).head().cloned()?, name, context, info)?;
    Ok(nodes)
}

pub(crate) fn lookupComponent(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    match '__try0: {
        (foundCref, foundScope, state) = unwrap_break_err!(lookupCref(&cref, scope.clone(), context), '__try0);
        node = unwrap_break_err!(ComponentRef::node(&foundCref), '__try0);
        let false = (NFInstNode::InstNode::isName(&node)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        Ok::<_, &'static str>((foundCref.clone(), foundScope.clone(), node.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            foundCref = __try0_o0;
            foundScope = __try0_o1;
            node = __try0_o2;
            state = __try0_o3;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(
                &(Error::LOOKUP_VARIABLE_ERROR.clone()),
                list![
                    Dump::printComponentRefStr(&cref)?,
                    NFInstNode::InstNode::scopeName(&scope)?
                ],
                &info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    state = fixTypenameState(node.clone(), state, context)?;
    LookupState::assertComponent(&state, node, cref, context, info)?;
    Ok((foundCref, foundScope))
}

pub(crate) fn lookupConnector(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    if let Ok((__pa0, __pa1, __pa2)) = lookupCref(&cref, scope.clone(), context) {
        foundCref = metamodelica::Own::own(__pa0);
        foundScope = metamodelica::Own::own(__pa1);
        state = metamodelica::Own::own(__pa2);
    } else {
        Error::addSourceMessageAndFail(
            &(Error::LOOKUP_VARIABLE_ERROR.clone()),
            list![
                Dump::printComponentRefStr(&cref)?,
                NFInstNode::InstNode::scopeName(&scope)?
            ],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    node = ComponentRef::node(&foundCref)?;
    state = fixTypenameState(node.clone(), state, context)?;
    LookupState::assertComponent(&state, node, cref, context, info)?;
    Ok((foundCref, foundScope))
}

pub(crate) fn fixTypenameState(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut state: metamodelica::Ref<LookupState::LookupState>,
    mut context: i32,
) -> Result<metamodelica::Ref<LookupState::LookupState>> {
    let mut state: metamodelica::Ref<LookupState::LookupState> = state;
    let mut ty: metamodelica::Ref<Type::NFType>;
    if NFInstNode::InstNode::isClass(&component)? {
        ty = NFInstNode::InstNode::getType(Inst::expand(component, context)?)?;
        state = (match &*ty {
            Type::ENUMERATION { .. } => crate::NFLookupState::LookupState::interned_COMP(),
            Type::BOOLEAN => crate::NFLookupState::LookupState::interned_COMP(),
            _ => state,
        });
    }
    Ok(state)
}

pub(crate) fn lookupLocalComponent(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    (foundCref, foundScope, state) = lookupLocalCref(&cref, scope, context, &info)?;
    LookupState::assertComponent(&state, ComponentRef::node(&foundCref)?, cref, context, info)?;
    Ok((foundCref, foundScope))
}

pub(crate) fn lookupFunctionName(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    match '__try0: {
        (foundCref, foundScope, state) = unwrap_break_err!(lookupCref(&cref, scope.clone(), context), '__try0);
        node = unwrap_break_err!(ComponentRef::node(&foundCref), '__try0);
        let false = (NFInstNode::InstNode::isName(&node)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        Ok::<_, &'static str>((foundCref.clone(), foundScope.clone(), node.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            foundCref = __try0_o0;
            foundScope = __try0_o1;
            node = __try0_o2;
            state = __try0_o3;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(
                &(Error::LOOKUP_FUNCTION_ERROR.clone()),
                list![
                    Dump::printComponentRefStr(&cref)?,
                    NFInstNode::InstNode::scopeName(&scope)?
                ],
                &info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    (foundCref, state) = fixExternalObjectCall(node.clone(), foundCref, state)?;
    LookupState::assertFunction(&state, node, cref, context, info)?;
    Ok((foundCref, foundScope))
}

pub(crate) fn lookupFunctionNameSilent(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    (foundCref, foundScope, state) = lookupCref(cref, scope, context)?;
    node = ComponentRef::node(&foundCref)?;
    (foundCref, state) = fixExternalObjectCall(node.clone(), foundCref, state)?;
    let true = (LookupState::isFunction(&state, node)?) else {
        return Err("pattern mismatch");
    };
    Ok((foundCref, foundScope))
}

pub(crate) fn fixExternalObjectCall(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut state: metamodelica::Ref<LookupState::LookupState>,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut state: metamodelica::Ref<LookupState::LookupState> = state;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut constructor: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    if !(LookupState::isClass(&state)) {
        return Ok((cref, state));
    }
    Inst::expand(node.clone(), InstContext::NO_CONTEXT.clone())?;
    cls = NFInstNode::InstNode::getClass(node)?;
    let () = (::match_deref::match_deref! { match &(cls) {
        Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __esc_constructor, .. }, .. }, .. } => {
            constructor = (*__esc_constructor).clone();
            cref = ComponentRef::prefixCref(NFInstNode::InstNode::borrow(constructor.clone())?, crate::NFType::interned_UNKNOWN(), metamodelica::nil(), cref)?;
            state = crate::NFLookupState::LookupState::interned_FUNC();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, state))
}

pub(crate) fn lookupImport(
    mut name: metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut element: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    (element, state, _) = lookupNameWithError(
        name.clone(),
        NFInstNode::InstNode::topScope(scope)?,
        InstContext::NO_CONTEXT.clone(),
        &info,
        &(Error::LOOKUP_IMPORT_ERROR.clone()),
        true,
    )?;
    LookupState::assertImport(&state, element.clone(), name, info)?;
    Ok(element)
}

pub(crate) fn lookupCrefWithError(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
    mut errMsg: &ErrorTypes::Message,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    if let Ok((__pa0, __pa1, __pa2)) = lookupCref(cref, scope.clone(), context) {
        foundCref = metamodelica::Own::own(__pa0);
        foundScope = metamodelica::Own::own(__pa1);
        state = metamodelica::Own::own(__pa2);
    } else {
        Error::addSourceMessage(
            errMsg,
            list![
                Dump::printComponentRefStr(cref)?,
                NFInstNode::InstNode::scopeName(&scope)?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok((foundCref, foundScope, state))
}

pub(crate) fn lookupCref<'__b>(
    mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut in_enclosing: bool;
    let mut is_iterator: bool;
    (foundCref, foundScope, state) = (match &**cref {
        Absyn::ComponentRef::CREF_IDENT { .. } => {
            (_, foundCref, foundScope, in_enclosing, _, state) = lookupSimpleCref(
                var_field!((**cref).name, Absyn::ComponentRef::CREF_IDENT).clone(),
                var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(),
                scope,
                context,
            )?;
            state = LookupState::checkCrefVariability(&foundCref, in_enclosing, context, state)?;
            (foundCref, foundScope, state)
        }
        Absyn::ComponentRef::CREF_QUAL { .. } => {
            (node, foundCref, foundScope, in_enclosing, is_iterator, state) = lookupSimpleCref(
                var_field!((**cref).name, Absyn::ComponentRef::CREF_QUAL).clone(),
                var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(),
                scope,
                context,
            )?;
            (foundCref, foundScope, state) = lookupCrefInNode(
                var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL),
                node,
                is_iterator,
                foundCref,
                foundScope,
                state,
                context,
            )?;
            state = LookupState::checkCrefVariability(&foundCref, in_enclosing, context, state)?;
            (foundCref, foundScope, state)
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => lookupCref(
            var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED),
            NFInstNode::InstNode::topScope(scope)?,
            context,
        )?,
        Absyn::ComponentRef::WILD { .. } => (
            crate::NFComponentRef::interned_WILD(),
            scope,
            crate::NFLookupState::LookupState::interned_PREDEF_COMP(),
        ),
        Absyn::ComponentRef::ALLWILD { .. } => (
            crate::NFComponentRef::interned_WILD(),
            scope,
            crate::NFLookupState::LookupState::interned_PREDEF_COMP(),
        ),
    });
    Ok((foundCref, foundScope, state))
}

pub(crate) fn lookupLocalCref(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    let mut foundScope: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut state: metamodelica::Ref<LookupState::LookupState> = metamodelica::Ref::new(LookupState::BEGIN);
    let mut node: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut is_iterator: bool = false;
    (foundCref, foundScope, state) = 'mc: {
        let __mc_input = &**cref;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
                    let mut foundScope: metamodelica::Ref<InstNode::InstNode> = foundScope.clone();
                    let mut node: metamodelica::Ref<InstNode::InstNode> = node.clone();
                    let mut state: metamodelica::Ref<LookupState::LookupState> = state.clone();
                    (node, foundScope, _) = lookupLocalSimpleCref(var_field!((**cref).name, Absyn::ComponentRef::CREF_IDENT).clone(), scope.clone())?;
                    state = LookupState::nodeState(node.clone())?;
                    Ok(((ComponentRef::fromAbsyn(node.clone(), var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), crate::NFComponentRef::interned_EMPTY(), false)?, foundScope.clone(), state.clone()), foundScope.clone(), node.clone(), state.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            foundScope = __wb0;
            node = __wb1;
            state = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
                    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef> = foundCref.clone();
                    let mut foundScope: metamodelica::Ref<InstNode::InstNode> = foundScope.clone();
                    let mut is_iterator: bool = is_iterator.clone();
                    let mut node: metamodelica::Ref<InstNode::InstNode> = node.clone();
                    let mut state: metamodelica::Ref<LookupState::LookupState> = state.clone();
                    (node, foundScope, is_iterator) = lookupLocalSimpleCref(var_field!((**cref).name, Absyn::ComponentRef::CREF_QUAL).clone(), scope.clone())?;
                    state = LookupState::nodeState(node.clone())?;
                    foundCref = ComponentRef::fromAbsyn(node.clone(), var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), crate::NFComponentRef::interned_EMPTY(), is_iterator)?;
                    (foundCref, foundScope, state) = lookupCrefInNode(var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL), node.clone(), is_iterator, foundCref.clone(), foundScope.clone(), state.clone(), context)?;
                    Ok(((foundCref.clone(), foundScope.clone(), state.clone()), foundCref.clone(), foundScope.clone(), is_iterator.clone(), node.clone(), state.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            foundCref = __wb0;
            foundScope = __wb1;
            is_iterator = __wb2;
            node = __wb3;
            state = __wb4;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::LOOKUP_VARIABLE_ERROR.clone()), list![Dump::printComponentRefStr(cref)?, NFInstNode::InstNode::scopeName(&scope)?], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((foundCref, foundScope, state))
}

pub(crate) fn lookupInner(
    mut outerNode: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut innerNode: metamodelica::Ref<InstNode::InstNode>;
    let mut name: ArcStr = NFInstNode::InstNode::name(&outerNode)?;
    let mut cur_scope: metamodelica::Ref<InstNode::InstNode> = scope.clone();
    let mut prev_scope: metamodelica::Ref<InstNode::InstNode> = scope.clone();
    while !(NFInstNode::InstNode::isEmpty(&cur_scope)) {
        if '__try0: {
            innerNode = NFInstNode::InstNode::resolveOuter((unwrap_break_err!(Class::lookupElement(name.clone(), unwrap_break_err!(NFInstNode::InstNode::getClass(cur_scope.clone()), '__try0)), '__try0)).0);
            let true = (unwrap_break_err!(NFInstNode::InstNode::isInner(&innerNode), '__try0)) else { break '__try0 Err::<_, _>("pattern mismatch") };
            return Ok(innerNode);
            Ok::<(), &'static str>(())
        }.is_err() {
            if NFInstNode::InstNode::isRootClass(&cur_scope) {
                prev_scope = NFInstNode::InstNode::topScope(cur_scope.clone())?;
                cur_scope = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
            } else {
                prev_scope = cur_scope.clone();
                cur_scope = NFInstNode::InstNode::instanceParent(cur_scope.clone())?;
            }
        }
    }
    innerNode = generateInner(outerNode, &(NFInstNode::InstNode::topScope(prev_scope)?))?;
    Ok(innerNode)
}

pub fn lookupLocalSimpleName(
    mut name: ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut isImport: bool;
    (node, isImport) = Class::lookupElement(name, NFInstNode::InstNode::getClass(scope)?)?;
    node = NFInstNode::InstNode::resolveInner(node);
    Ok((node, isImport))
}

pub fn lookupSimpleName(
    mut name: ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut selfReference: bool = false;
    let mut cur_scope: metamodelica::Ref<InstNode::InstNode> = scope.clone();
    let mut require_builtin: bool = false;
    let mut loaded: bool = false;
    if InstContext::inAnnotation(context) {
        if '__try0: {
            (node, _) = unwrap_break_err!(lookupLocalSimpleName(name.clone(), unwrap_break_err!(NFInstNode::InstNode::annotationScope(scope.clone()), '__try0)), '__try0);
            return Ok((node, selfReference));
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    for mut i in 1..=Global::recursionDepthLimit.clone() {
        if '__try1: {
            (node, _) = unwrap_break_err!(lookupLocalSimpleName(name.clone(), cur_scope.clone()), '__try1);
            if require_builtin {
                let true = (NFInstNode::InstNode::isBuiltin(&node)) else {
                    break '__try1 Err::<_, _>("pattern mismatch");
                };
            }
            return Ok((node, selfReference));
            Ok::<(), &'static str>(())
        }
        .is_err()
        {
            if NFInstNode::InstNode::isEncapsulated(cur_scope.clone())? {
                cur_scope =
                    NFInstNode::InstNode::topScope(NFInstNode::InstNode::parentScope(cur_scope.clone(), false)?)?;
                require_builtin = true;
            } else if metamodelica::stringEq(&name, &(NFInstNode::InstNode::name(&cur_scope)?))
                && NFInstNode::InstNode::isClass(&cur_scope)?
                || metamodelica::stringEq(
                    &name,
                    &(NFInstNode::InstNode::name(&(NFInstNode::InstNode::classScope(cur_scope.clone())?))?),
                )
            {
                node = NFInstNode::InstNode::classScope(cur_scope.clone())?;
                selfReference = true;
                return Ok((node, selfReference));
            } else {
                if NFInstNode::InstNode::isTopScope(&cur_scope) && !(loaded) && !(require_builtin) {
                    loaded = true;
                    loadLibrary(name.clone(), cur_scope.clone());
                } else {
                    cur_scope = NFInstNode::InstNode::parentScope(cur_scope.clone(), false)?;
                }
            }
        }
    }
    Error::addMessage(
        Error::RECURSION_DEPTH_REACHED.clone(),
        list![
            ArcStr::from(::std::format!("{}", Global::recursionDepthLimit.clone())),
            NFInstNode::InstNode::name(&scope)?
        ],
    )?;
    return Err("fail");
    Ok((node, selfReference))
}

pub fn lookupSimpleNameRootPath(
    mut name: ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cur_scope: metamodelica::Ref<InstNode::InstNode> = scope.clone();
    let mut in_root_class: bool = true;
    if InstContext::inAnnotation(context) {
        if '__try0: {
            unwrap_break_err!(lookupLocalSimpleName(name.clone(), unwrap_break_err!(NFInstNode::InstNode::annotationScope(scope.clone()), '__try0)), '__try0);
            path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() });
            return Ok(path);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    for mut i in 1..=Global::recursionDepthLimit.clone() {
        if '__try1: {
            (node, _) = unwrap_break_err!(Class::lookupElement(name.clone(), unwrap_break_err!(NFInstNode::InstNode::getClass(cur_scope.clone()), '__try1)), '__try1);
            if in_root_class {
                path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() });
            } else {
                path = AbsynUtil::makeFullyQualified(unwrap_break_err!(NFInstNode::InstNode::fullPath(node.clone(), false), '__try1));
            }
            return Ok(path);
            Ok::<(), &'static str>(())
        }.is_err() {
            if NFInstNode::InstNode::isEncapsulated(cur_scope.clone())? {
                path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() });
                return Ok(path);
            } else if metamodelica::stringEq(&name, &(NFInstNode::InstNode::name(&cur_scope)?)) && NFInstNode::InstNode::isClass(&cur_scope)? {
                path = if (in_root_class) {metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() })} else {AbsynUtil::makeFullyQualified(NFInstNode::InstNode::fullPath(cur_scope.clone(), false)?)};
                return Ok(path);
            } else if NFInstNode::InstNode::isTopScope(&cur_scope) {
                path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() });
                return Ok(path);
            } else {
                if in_root_class && NFInstNode::InstNode::isRootClass(&cur_scope) {
                    in_root_class = false;
                }
                cur_scope = NFInstNode::InstNode::parentScope(cur_scope.clone(), false)?;
            }
        }
    }
    return Err("fail");
    Ok(path)
}

pub(crate) fn lookupNameWithError(
    mut name: metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
    mut errorType: &ErrorTypes::Message,
    mut checkAccessViolations: bool,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut prefixes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    if let Ok((__pa0, __pa1, __pa2)) = lookupName(&name, scope.clone(), context, checkAccessViolations) {
        node = metamodelica::Own::own(__pa0);
        state = metamodelica::Own::own(__pa1);
        prefixes = metamodelica::Own::own(__pa2);
    } else {
        Error::addSourceMessage(
            errorType,
            list![
                AbsynUtil::pathString(name.clone(), literal!("."), true, false)?,
                NFInstNode::InstNode::scopeName(&scope)?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok((node, state, prefixes))
}

pub fn lookupName<'__b>(
    mut name: &'__b metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut checkAccessViolations: bool,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut prefixes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut self_reference: bool;
    let mut prefix: metamodelica::Ref<InstNode::InstNode>;
    (node, state, prefixes) = (match &**name {
        Absyn::Path::IDENT { .. } => {
            (node, state, _) = lookupFirstIdent(var_field!((**name).name, Absyn::Path::IDENT).clone(), scope, context)?;
            (node, state, metamodelica::nil())
        }
        Absyn::Path::QUALIFIED { .. } => {
            (prefix, state, self_reference) = lookupFirstIdent(
                var_field!((**name).name, Absyn::Path::QUALIFIED).clone(),
                scope,
                context,
            )?;
            (node, state, prefixes) = lookupLocalName(
                var_field!((**name).path, Absyn::Path::QUALIFIED),
                prefix.clone(),
                state,
                context,
                checkAccessViolations,
                self_reference,
            )?;
            (node, state, metamodelica::cons(prefix, prefixes))
        }
        Absyn::Path::FULLYQUALIFIED { .. } => lookupName(
            var_field!((**name).path, Absyn::Path::FULLYQUALIFIED),
            NFInstNode::InstNode::topScope(scope)?,
            context,
            checkAccessViolations,
        )?,
    });
    Ok((node, state, prefixes))
}

pub(crate) fn lookupNames<'__b>(
    mut name: &'__b metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut self_reference: bool;
    (nodes, state) = (match &**name {
        Absyn::Path::IDENT { .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            (node, state, _) = lookupFirstIdent(var_field!((**name).name, Absyn::Path::IDENT).clone(), scope, context)?;
            (list![node], state)
        }
        Absyn::Path::QUALIFIED { .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            (node, state, self_reference) = lookupFirstIdent(
                var_field!((**name).name, Absyn::Path::QUALIFIED).clone(),
                scope,
                context,
            )?;
            lookupLocalNames(
                var_field!((**name).path, Absyn::Path::QUALIFIED),
                node.clone(),
                list![node],
                state,
                context,
                self_reference,
            )?
        }
        Absyn::Path::FULLYQUALIFIED { .. } => lookupNames(
            var_field!((**name).path, Absyn::Path::FULLYQUALIFIED),
            NFInstNode::InstNode::topScope(scope)?,
            context,
        )?,
    });
    Ok((nodes, state))
}

pub(crate) fn lookupFirstIdent(
    mut name: ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
    bool,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut selfReference: bool;
    match '__try0: {
        node = unwrap_break_err!(lookupSimpleBuiltinName(&name), '__try0);
        state = crate::NFLookupState::LookupState::interned_PREDEF_CLASS();
        selfReference = false;
        Ok::<_, &'static str>((node.clone(), selfReference.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            node = __try0_o0;
            selfReference = __try0_o1;
            state = __try0_o2;
        }
        Err(_) => {
            (node, selfReference) = lookupSimpleName(name.clone(), scope.clone(), context)?;
            state = LookupState::nodeState(node.clone())?;
        }
    }
    Ok((node, state, selfReference))
}

pub(crate) fn lookupLocalName(
    mut name: &metamodelica::Ref<Absyn::Path>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut state: metamodelica::Ref<LookupState::LookupState>,
    mut context: i32,
    mut checkAccessViolations: bool,
    mut selfReference: bool,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut state: metamodelica::Ref<LookupState::LookupState> = state;
    let mut prefixes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut is_import: bool;
    let mut prefix: metamodelica::Ref<InstNode::InstNode>;
    if !(NFInstNode::InstNode::isClass(&node)?) {
        state = crate::NFLookupState::LookupState::interned_COMP_CLASS();
        return Ok((node, state, prefixes));
    }
    if !(selfReference) {
        node = Inst::instPackage(node, context)?;
        if NFInstNode::InstNode::isPartial(&node)?
            && !(InstContext::inRelaxed(context) || InstContext::inRedeclared(context))
        {
            state = metamodelica::Ref::new(LookupState::LookupState::ERROR {
                errorState: crate::NFLookupState::LookupState::interned_PARTIAL_CLASS(),
            });
            return Ok((node, state, prefixes));
        }
    }
    let () = (match &**name {
        Absyn::Path::IDENT { name: __name_name } => {
            (node, is_import) = lookupLocalSimpleName(__name_name.clone(), node)?;
            if is_import {
                state = metamodelica::Ref::new(LookupState::LookupState::ERROR {
                    errorState: crate::NFLookupState::LookupState::interned_IMPORT(),
                });
            } else {
                state = LookupState::next(node.clone(), &state, context, checkAccessViolations)?;
            }
            ()
        }
        Absyn::Path::QUALIFIED {
            name: __name_name,
            path: __name_path,
        } => {
            (prefix, is_import) = lookupLocalSimpleName(__name_name.clone(), node)?;
            if is_import {
                node = prefix;
                state = metamodelica::Ref::new(LookupState::LookupState::ERROR {
                    errorState: crate::NFLookupState::LookupState::interned_IMPORT(),
                });
            } else {
                state = LookupState::next(prefix.clone(), &state, context, checkAccessViolations)?;
                (node, state, prefixes) = lookupLocalName(
                    metamodelica::AsArg::as_arg(&__name_path),
                    prefix.clone(),
                    state,
                    context,
                    checkAccessViolations,
                    false,
                )?;
                prefixes = metamodelica::cons(prefix, prefixes);
            }
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFLookup.lookupLocalName"));
                    __mm_s.push_str(&*literal!(" was called with an invalid path."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFLookup.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((node, state, prefixes))
}

pub(crate) fn lookupLocalNames<'__b>(
    mut name: &'__b metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut state: metamodelica::Ref<LookupState::LookupState>,
    mut context: i32,
    mut selfReference: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = nodes;
    let mut state: metamodelica::Ref<LookupState::LookupState> = state;
    let mut node: metamodelica::Ref<InstNode::InstNode> = scope.clone();
    if !(NFInstNode::InstNode::isClass(&scope)?) {
        state = crate::NFLookupState::LookupState::interned_COMP_CLASS();
        return Ok((nodes, state));
    }
    if !(selfReference) {
        node = Inst::instPackage(node, context)?;
        if NFInstNode::InstNode::isPartial(&node)?
            && !(InstContext::inRelaxed(context) || InstContext::inRedeclared(context))
            && !(metamodelica::stringEq(
                &(NFInstNode::InstNode::name(&node)?),
                &(literal!("PartialModelicaServices")),
            ))
        {
            state = metamodelica::Ref::new(LookupState::LookupState::ERROR {
                errorState: crate::NFLookupState::LookupState::interned_PARTIAL_CLASS(),
            });
            return Ok((nodes, state));
        }
    }
    (nodes, state) = (match &**name {
        Absyn::Path::IDENT { .. } => {
            (node, _) = lookupLocalSimpleName(var_field!((**name).name, Absyn::Path::IDENT).clone(), node)?;
            state = LookupState::next(node.clone(), &state, context, true)?;
            (metamodelica::cons(node, nodes), state)
        }
        Absyn::Path::QUALIFIED { .. } => {
            (node, _) = lookupLocalSimpleName(var_field!((**name).name, Absyn::Path::QUALIFIED).clone(), node)?;
            state = LookupState::next(node.clone(), &state, context, true)?;
            lookupLocalNames(
                var_field!((**name).path, Absyn::Path::QUALIFIED),
                node.clone(),
                metamodelica::cons(node, nodes),
                state,
                context,
                false,
            )?
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFLookup.lookupLocalNames"));
                    __mm_s.push_str(&*literal!(" was called with an invalid path."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFLookup.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((nodes, state))
}

pub(crate) fn lookupSimpleBuiltinName(mut name: &ArcStr) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut builtin: metamodelica::Ref<InstNode::InstNode>;
    builtin = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "Real" => NFBuiltin::REAL_NODE().clone(),
        Deref @ "Integer" => NFBuiltin::INTEGER_NODE().clone(),
        Deref @ "Boolean" => NFBuiltin::BOOLEAN_NODE().clone(),
        Deref @ "String" => NFBuiltin::STRING_NODE().clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(builtin)
}

pub(crate) fn lookupSimpleBuiltinCref(
    mut name: &ArcStr,
    mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    (node, cref, state) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "time" => (NFBuiltin::TIME().clone(), NFBuiltin::TIME_CREF().clone(), crate::NFLookupState::LookupState::interned_PREDEF_COMP()),
        Deref @ "Boolean" => (NFBuiltin::BOOLEAN_NODE().clone(), NFBuiltin::BOOLEAN_CREF().clone(), crate::NFLookupState::LookupState::interned_PREDEF_CLASS()),
        Deref @ "Integer" => (NFBuiltinFuncs::INTEGER_NODE().clone(), NFBuiltinFuncs::INTEGER_CREF().clone(), crate::NFLookupState::LookupState::interned_FUNC()),
        Deref @ "String" => (NFBuiltinFuncs::STRING_NODE().clone(), NFBuiltinFuncs::STRING_CREF().clone(), crate::NFLookupState::LookupState::interned_FUNC()),
        _ => return Err("match: no arm matched"),
    } });
    if !((subs).is_empty()) {
        cref = ComponentRef::setSubscripts(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut s in (subs).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Subscript::NFSubscript::RAW_SUBSCRIPT { subscript: s.clone() });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            cref,
        )?;
    }
    Ok((node, cref, state))
}

pub(crate) fn lookupSimpleCref(
    mut name: ArcStr,
    mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
    bool,
    bool,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode> = scope;
    let mut inEnclosingScope: bool = false;
    let mut isIterator: bool = false;
    let mut state: metamodelica::Ref<LookupState::LookupState>;
    let mut require_builtin: bool = false;
    let mut loaded: bool = false;
    match '__try0: {
        (node, cref, state) = unwrap_break_err!(lookupSimpleBuiltinCref(&name, subs.clone()), '__try0);
        foundScope = unwrap_break_err!(NFInstNode::InstNode::topScope(foundScope.clone()), '__try0);
        Ok::<_, &'static str>((cref.clone(), foundScope.clone(), node.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cref = __try0_o0;
            foundScope = __try0_o1;
            node = __try0_o2;
            state = __try0_o3;
        }
        Err(__try0_err) => {
            if InstContext::inAnnotation(context) {
                if '__try1: {
                    (node, foundScope, _) = unwrap_break_err!(lookupLocalSimpleCref(name.clone(), unwrap_break_err!(NFInstNode::InstNode::annotationScope(foundScope.clone()), '__try1)), '__try1);
                    state = unwrap_break_err!(LookupState::nodeState(node.clone()), '__try1);
                    cref = unwrap_break_err!(ComponentRef::fromAbsyn(node.clone(), subs.clone(), crate::NFComponentRef::interned_EMPTY(), false), '__try1);
                    return Ok((node, cref, foundScope, inEnclosingScope, isIterator, state));
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
            }
            for mut i in 1..=Global::recursionDepthLimit.clone() {
                if '__try2: {
                    (node, foundScope, isIterator) = unwrap_break_err!(lookupLocalSimpleCref(name.clone(), foundScope.clone()), '__try2);
                    if require_builtin {
                        let true = (NFInstNode::InstNode::isBuiltin(&node)) else { break '__try2 Err::<_, _>("pattern mismatch") };
                    }
                    state = unwrap_break_err!(LookupState::nodeState(node.clone()), '__try2);
                    cref = unwrap_break_err!(ComponentRef::fromAbsyn(node.clone(), subs.clone(), crate::NFComponentRef::interned_EMPTY(), isIterator), '__try2);
                    return Ok((node, cref, foundScope, inEnclosingScope, isIterator, state));
                    Ok::<(), &'static str>(())
                }.is_err() {
                    if NFInstNode::InstNode::isEncapsulated(foundScope.clone())? {
                        foundScope = NFInstNode::InstNode::topScope(NFInstNode::InstNode::parentScope(foundScope.clone(), false)?)?;
                        require_builtin = true;
                    } else {
                        if NFInstNode::InstNode::isTopScope(&foundScope) && !(loaded) && !(require_builtin) {
                            loaded = true;
                            loadLibrary(name.clone(), foundScope.clone());
                        } else {
                            inEnclosingScope = !(NFInstNode::InstNode::isImplicit(&foundScope));
                            foundScope = NFInstNode::InstNode::parentScope(foundScope.clone(), false)?;
                        }
                    }
                }
            }
            Error::addMessage(
                Error::RECURSION_DEPTH_REACHED.clone(),
                list![
                    ArcStr::from(::std::format!("{}", Global::recursionDepthLimit.clone())),
                    NFInstNode::InstNode::scopeName(&foundScope)?
                ],
            )?;
            return Err(__try0_err);
        }
    }
    Ok((node, cref, foundScope, inEnclosingScope, isIterator, state))
}

pub(crate) fn lookupLocalSimpleCref(
    mut name: ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<InstNode::InstNode>,
    bool,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode> = scope;
    let mut isIterator: bool = false;
    let mut is_import: bool;
    (node, is_import) = (match &*foundScope {
        NFInstNode::InstNode::IMPLICIT_SCOPE {
            locals: __foundScope_locals,
            ..
        } => (
            lookupIterator(&name, metamodelica::AsArg::as_arg(&__foundScope_locals))?,
            false,
        ),
        NFInstNode::InstNode::CLASS_NODE { .. } => {
            Class::lookupElement(name, NFInstNode::InstNode::getClass(foundScope.clone())?)?
        }
        NFInstNode::InstNode::COMPONENT_NODE { .. } => {
            Class::lookupElement(name, NFInstNode::InstNode::getClass(foundScope.clone())?)?
        }
        NFInstNode::InstNode::INNER_OUTER_NODE {
            innerNode: __foundScope_innerNode,
            ..
        } => Class::lookupElement(name, NFInstNode::InstNode::getClass(__foundScope_innerNode.clone())?)?,
        _ => return Err("match: no arm matched"),
    });
    if is_import {
        foundScope = NFInstNode::InstNode::parent(&node)?;
    } else if NFInstNode::InstNode::isInnerOuterNode(&node) {
        node = NFInstNode::InstNode::resolveInner(node);
        foundScope = NFInstNode::InstNode::parent(&node)?;
    } else {
        isIterator = NFInstNode::InstNode::isIterator(&node);
    }
    Ok((node, foundScope, isIterator))
}

pub(crate) fn lookupIterator(
    mut name: &ArcStr,
    mut iterators: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut iterator: metamodelica::Ref<InstNode::InstNode>;
    for mut i in &**iterators {
        if metamodelica::stringEq(&name, &(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&i))?)) {
            iterator = i.clone();
            return Ok(iterator);
        }
    }
    return Err("fail");
    Ok(iterator)
}

pub(crate) fn lookupCrefInNode<'__b>(
    mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut isIterator: bool,
    mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut foundScope: metamodelica::Ref<InstNode::InstNode>,
    mut state: metamodelica::Ref<LookupState::LookupState>,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<LookupState::LookupState>,
)> {
    let mut foundCref: metamodelica::Ref<ComponentRef::NFComponentRef> = foundCref;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode> = foundScope;
    let mut state: metamodelica::Ref<LookupState::LookupState> = state;
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut name: ArcStr;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut is_import: bool;
    let mut scope_is_class: bool;
    if LookupState::isError(&state) || InstContext::inConnect(context) && NFInstNode::InstNode::isEmpty(&node) {
        return Ok((foundCref, foundScope, state));
    }
    scope = node.clone();
    scope_is_class = NFInstNode::InstNode::isClass(&scope)?;
    if scope_is_class {
        scope = Inst::instPackage(node.clone(), context)?;
        if NFInstNode::InstNode::isPartial(&scope)?
            && !(InstContext::inRelaxed(context) || InstContext::inRedeclared(context))
        {
            state = metamodelica::Ref::new(LookupState::LookupState::ERROR {
                errorState: crate::NFLookupState::LookupState::interned_PARTIAL_CLASS(),
            });
            return Ok((foundCref, foundScope, state));
        }
    } else if NFInstNode::InstNode::isGeneratedInner(&scope)
        && Component::isDefinition(&(NFInstNode::InstNode::component(&scope)?))
    {
        Inst::instComponent(
            scope.clone(),
            &(Attributes::DEFAULT_ATTR().clone()),
            crate::NFModifier::Modifier::interned_NOMOD(),
            true,
            0,
            InstContext::CLASS.clone(),
            None,
            &(metamodelica::nil()),
        )?;
    }
    name = AbsynUtil::crefFirstIdent(cref)?;
    cls_node = NFInstNode::InstNode::classScope(scope)?;
    if NFInstNode::InstNode::isEmpty(&cls_node) {
        foundCref = ComponentRef::fromAbsynCref(cref, foundCref)?;
        return Ok((foundCref, foundScope, state));
    }
    cls = NFInstNode::InstNode::getClass(cls_node)?;
    if let Ok((__pa0, __pa1)) = Class::lookupElement(name.clone(), cls.clone()) {
        n = metamodelica::Own::own(__pa0);
        is_import = metamodelica::Own::own(__pa1);
    } else {
        let true = (NFInstNode::InstNode::isComponent(&node)?) else {
            return Err("pattern mismatch");
        };
        let true = (Class::isExpandableConnectorClass(&cls) || InstContext::inInstanceAPI(context)) else {
            return Err("pattern mismatch");
        };
        foundCref = ComponentRef::fromAbsynCref(cref, foundCref.clone())?;
        return Ok((foundCref, foundScope, state));
    }
    if is_import {
        state = metamodelica::Ref::new(LookupState::LookupState::ERROR {
            errorState: crate::NFLookupState::LookupState::interned_IMPORT(),
        });
        foundCref = ComponentRef::fromAbsyn(n, metamodelica::nil(), foundCref, false)?;
        return Ok((foundCref, foundScope, state));
    }
    (n, foundCref, foundScope) = resolveInnerCref(n, foundCref, foundScope)?;
    foundCref = ComponentRef::fromAbsyn(n.clone(), AbsynUtil::crefFirstSubs(cref)?, foundCref, isIterator)?;
    if scope_is_class && !(InstContext::inRelaxed(context)) && LookupState::isNonConstantComponent(&n)? {
        state = metamodelica::Ref::new(LookupState::LookupState::ERROR {
            errorState: crate::NFLookupState::LookupState::interned_NON_ENCAPSULATED(),
        });
        return Ok((foundCref, foundScope, state));
    } else {
        state = LookupState::next(n.clone(), &state, context, true)?;
    }
    (foundCref, foundScope, state) = (match &**cref {
        Absyn::ComponentRef::CREF_IDENT { .. } => (foundCref, foundScope, state),
        Absyn::ComponentRef::CREF_QUAL { .. } => lookupCrefInNode(
            var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL),
            n,
            isIterator,
            foundCref,
            foundScope,
            state,
            context,
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok((foundCref, foundScope, state))
}

pub(crate) fn resolveInnerCref(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut foundScope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut foundScope: metamodelica::Ref<InstNode::InstNode> = foundScope;
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    if NFInstNode::InstNode::isInnerOuterNode(&node) {
        node = NFInstNode::InstNode::resolveInner(node);
        scope = NFInstNode::InstNode::parent(&node)?;
        while !(ComponentRef::isEmpty(&cref)) {
            if referenceEq(&*(ComponentRef::node(&cref)?), &*(&*scope)) {
                break;
            } else {
                cref = ComponentRef::rest(&cref)?;
            }
        }
        if ComponentRef::isEmpty(&cref) {
            foundScope = scope;
        }
    }
    Ok((node, cref, foundScope))
}

pub(crate) fn generateInner(
    mut outerNode: metamodelica::Ref<InstNode::InstNode>,
    mut topScope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut innerNode: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut node_ty: metamodelica::Ref<InstNodeType>;
    let mut name: ArcStr;
    let mut inner_node_opt: Option<metamodelica::Ref<InstNode::InstNode>>;
    node_ty = NFInstNode::InstNode::nodeType(topScope)?;
    let () = (match &*node_ty {
        NFInstNode::InstNodeType::TOP_SCOPE {
            generatedInners: __node_ty_generatedInners,
            ..
        } => {
            name = NFInstNode::InstNode::name(&outerNode)?;
            inner_node_opt = UnorderedMap::get(name.clone(), __node_ty_generatedInners.clone())?;
            if (inner_node_opt).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(inner_node_opt) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                innerNode = metamodelica::Own::own(__pa0);
            } else {
                innerNode = makeInnerNode(outerNode)?;
                innerNode = NFInstNode::InstNode::setNodeType(
                    crate::NFInstNode::InstNodeType::interned_GENERATED_INNER(),
                    innerNode,
                )?;
                UnorderedMap::add(name, innerNode.clone(), __node_ty_generatedInners.clone())?;
            }
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFLookup.generateInner"));
                    __mm_s.push_str(&*literal!(" got invalid top node"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFLookup.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(innerNode)
}

pub(crate) fn makeInnerNode(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    node = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ NFInstNode::InstNode::CLASS_NODE { definition: def @ Deref @ SCode::Element::CLASS { prefixes: prefs, .. }, .. } => {
            let mut def = (*def).clone();
            let mut prefs = (*prefs).clone();
            assign_field!(prefs.innerOuter = openmodelica_ast::Absyn::InnerOuter::INNER);
            assign_variant_field!(def => SCode::Element::CLASS; prefixes = prefs.clone());
            assign_variant_field!(node => InstNode::InstNode::CLASS_NODE; definition = def.clone());
            node
        },
        Deref @ NFInstNode::InstNode::COMPONENT_NODE { .. } => {
            let mut def: metamodelica::Ref<SCode::Element>;
            let mut prefs: metamodelica::Ref<SCode::Prefixes>;
            let mut comp: metamodelica::Ref<Component::NFComponent>;
            comp = NFInstNode::InstNode::component(&node)?;
            (comp, def) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT_DEF { definition: __esc_def @ Deref @ SCode::Element::COMPONENT { prefixes: __esc_prefs, .. }, .. } => {
            def = (*__esc_def).clone();
            prefs = (*__esc_prefs).clone();
            assign_field!(prefs.innerOuter = openmodelica_ast::Absyn::InnerOuter::INNER);
            assign_variant_field!(def => SCode::Element::COMPONENT; prefixes = prefs.clone());
            assign_variant_field!(comp => Component::NFComponent::COMPONENT_DEF; definition = def.clone());
            (comp, def.clone())
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookup.makeInnerNode")); __mm_s.push_str(&*literal!(" got unknown component")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFLookup.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            NFInstNode::InstNode::replaceComponent(comp, NFInstNode::InstNode::setDefinition(def, node)?)?
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookup.makeInnerNode")); __mm_s.push_str(&*literal!(" got unknown node")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFLookup.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub(crate) fn loadLibrary(mut name: ArcStr, mut scope: metamodelica::Ref<InstNode::InstNode>) -> () {
    let mut version: ArcStr;
    ErrorExt::setCheckpoint(literal!("NFLookup.loadLibrary"));
    if '__try0: {
        let true = (unwrap_break_err!(Flags::getConfigBool(Flags::LOAD_MISSING_LIBRARIES.clone()), '__try0)) else { break '__try0 Err::<_, _>("pattern mismatch") };
        version = unwrap_break_err!(loadLibrary_work(name.clone(), scope.clone()), '__try0);
        unwrap_break_err!(Error::addMessage(Error::NOTIFY_IMPLICIT_LOAD.clone(), list![name.clone(), version.clone()]), '__try0);
        System::loadModelCallBack(name.clone());
        ErrorExt::delCheckpoint(literal!("NFLookup.loadLibrary"));
        Ok::<(), &'static str>(())
    }.is_err() {
        ErrorExt::rollBack(literal!("NFLookup.loadLibrary"));
    }
    ()
}

pub(crate) fn loadLibrary_work(mut name: ArcStr, mut scope: metamodelica::Ref<InstNode::InstNode>) -> Result<ArcStr> {
    let mut version: ArcStr = literal!("(default)");
    let mut modelica_path: ArcStr;
    let mut aprog: Absyn::Program;
    let mut scls: metamodelica::Ref<SCode::Element>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut lib_node: metamodelica::Ref<InstNode::InstNode>;
    let mut new_libs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    modelica_path = Settings::getModelicaPath(Testsuite::isRunning()?)?;
    let (__pa0, true) = (BackendInterface::appendLibrary(
        metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
        modelica_path,
    )?) else {
        return Err("pattern mismatch");
    };
    aprog = metamodelica::Own::own(__pa0);
    for mut c in &*aprog.classes.clone() {
        if '__try1: {
            unwrap_break_err!(lookupLocalSimpleName(AbsynUtil::getClassName(metamodelica::AsArg::as_arg(&c)), scope.clone()), '__try1);
            Ok::<(), &'static str>(())
        }.is_err() {
            scls = AbsynToSCode::translateClass(c.clone())?;
            lib_node = NFInstNode::InstNode::new(scls.clone(), scope.clone())?;
            new_libs = metamodelica::cons(lib_node.clone(), new_libs.clone());
            if metamodelica::stringEq(&name, &(SCodeUtil::getElementName(&scls)?)) {
                if '__try2: {
                    let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(SCodeUtil::lookupElementAnnotationBinding(&scls, &(literal!("version"))), '__try2)) {
                        Some(Deref @ Absyn::Exp::STRING { value: __pa3 }) => __pa3.clone(),
                        _ => break '__try2 Err::<_, _>("pattern mismatch"),
                    } };
                    version = metamodelica::Own::own(__pa3);
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
            }
        }
    }
    cls = NFInstNode::InstNode::getClass(scope.clone())?;
    cls = Class::classTreeApply(
        cls,
        &({
            let __pe_b0 = new_libs;
            move |__pe_a1| ClassTree::appendClasses(__pe_b0.clone(), __pe_a1)
        }),
    )?;
    NFInstNode::InstNode::updateClass(cls, scope)?;
    Ok(version)
}
