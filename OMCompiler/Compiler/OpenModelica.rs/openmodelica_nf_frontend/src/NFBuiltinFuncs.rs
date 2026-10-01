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
use crate::NFBinding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionStatus;
use crate::NFFunction::Slot;
use crate::NFFunction::SlotEvalStatus;
use crate::NFFunction::SlotType;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFModifier::Modifier;
use crate::NFPrefixes::Visibility;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn::TypeSpec;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::SCode::Comment;
use openmodelica_frontend_types::SCode::Mod;
use openmodelica_util_datatypes_basic::Pointer;

pub(crate) static DUMMY_ELEMENT: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("$DummyFunction"),
            prefixes: SCode::defaultPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: openmodelica_ast::Absyn::FunctionPurity::NO_PURITY,
                },
            },
            classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                elementLst: metamodelica::nil(),
                normalEquationLst: metamodelica::nil(),
                initialEquationLst: metamodelica::nil(),
                normalAlgorithmLst: metamodelica::nil(),
                initialAlgorithmLst: metamodelica::nil(),
                constraintLst: metamodelica::nil(),
                clsattrs: metamodelica::nil(),
                externalDecl: None,
            }),
            cmt: metamodelica::Ref::new(Comment {
                annotation_: None,
                comment: None,
            }),
            info: Absyn::dummyInfo.clone(),
        })
    });

// Default Integer parameter.
thread_local! { static __INT_COMPONENT_TLS: metamodelica::Ref<Component::NFComponent> = metamodelica::Ref::new(Component::NFComponent::COMPONENT { classInst: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), ty: crate::NFType::interned_INTEGER(), binding: NFBinding::EMPTY_BINDING().clone(), condition: NFBinding::EMPTY_BINDING().clone(), attributes: Attributes::DEFAULT_ATTR().clone(), comment: SCode::noComment.clone(), state: ComponentState::TypeChecked.clone(), info: Absyn::dummyInfo.clone() }); }
pub(crate) fn INT_COMPONENT() -> metamodelica::Ref<Component::NFComponent> {
    __INT_COMPONENT_TLS.with(|__t| __t.clone())
}

thread_local! { static __INT_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("i"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(INT_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn INT_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __INT_PARAM_TLS.with(|__t| __t.clone())
}

// Default Real parameter.
thread_local! { static __REAL_COMPONENT_TLS: metamodelica::Ref<Component::NFComponent> = metamodelica::Ref::new(Component::NFComponent::COMPONENT { classInst: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), ty: crate::NFType::interned_REAL(), binding: NFBinding::EMPTY_BINDING().clone(), condition: NFBinding::EMPTY_BINDING().clone(), attributes: Attributes::DEFAULT_ATTR().clone(), comment: SCode::noComment.clone(), state: ComponentState::TypeChecked.clone(), info: Absyn::dummyInfo.clone() }); }
pub(crate) fn REAL_COMPONENT() -> metamodelica::Ref<Component::NFComponent> {
    __REAL_COMPONENT_TLS.with(|__t| __t.clone())
}

thread_local! { static __REAL_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("r"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(REAL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn REAL_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __REAL_PARAM_TLS.with(|__t| __t.clone())
}

// Default Boolean parameter.
thread_local! { static __BOOL_COMPONENT_TLS: metamodelica::Ref<Component::NFComponent> = metamodelica::Ref::new(Component::NFComponent::COMPONENT { classInst: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), ty: crate::NFType::interned_BOOLEAN(), binding: NFBinding::EMPTY_BINDING().clone(), condition: NFBinding::EMPTY_BINDING().clone(), attributes: Attributes::DEFAULT_ATTR().clone(), comment: SCode::noComment.clone(), state: ComponentState::TypeChecked.clone(), info: Absyn::dummyInfo.clone() }); }
pub(crate) fn BOOL_COMPONENT() -> metamodelica::Ref<Component::NFComponent> {
    __BOOL_COMPONENT_TLS.with(|__t| __t.clone())
}

thread_local! { static __BOOL_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("b"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(BOOL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn BOOL_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __BOOL_PARAM_TLS.with(|__t| __t.clone())
}

// Default String parameter.
thread_local! { static __STRING_COMPONENT_TLS: metamodelica::Ref<Component::NFComponent> = metamodelica::Ref::new(Component::NFComponent::COMPONENT { classInst: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), ty: crate::NFType::interned_STRING(), binding: NFBinding::EMPTY_BINDING().clone(), condition: NFBinding::EMPTY_BINDING().clone(), attributes: Attributes::DEFAULT_ATTR().clone(), comment: SCode::noComment.clone(), state: ComponentState::TypeChecked.clone(), info: Absyn::dummyInfo.clone() }); }
pub(crate) fn STRING_COMPONENT() -> metamodelica::Ref<Component::NFComponent> {
    __STRING_COMPONENT_TLS.with(|__t| __t.clone())
}

thread_local! { static __STRING_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("s"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(STRING_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn STRING_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __STRING_PARAM_TLS.with(|__t| __t.clone())
}

// Default enumeration(:) parameter.
thread_local! { static __ENUM_COMPONENT_TLS: metamodelica::Ref<Component::NFComponent> = metamodelica::Ref::new(Component::NFComponent::COMPONENT { classInst: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), ty: metamodelica::Ref::new(Type::NFType::ENUMERATION { typePath: metamodelica::Ref::new(Path::IDENT { name: literal!(":") }), literals: metamodelica::nil() }), binding: NFBinding::EMPTY_BINDING().clone(), condition: NFBinding::EMPTY_BINDING().clone(), attributes: Attributes::DEFAULT_ATTR().clone(), comment: SCode::noComment.clone(), state: ComponentState::TypeChecked.clone(), info: Absyn::dummyInfo.clone() }); }
pub(crate) fn ENUM_COMPONENT() -> metamodelica::Ref<Component::NFComponent> {
    __ENUM_COMPONENT_TLS.with(|__t| __t.clone())
}

thread_local! { static __ENUM_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("e"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(ENUM_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn ENUM_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __ENUM_PARAM_TLS.with(|__t| __t.clone())
}

// Integer(e)
thread_local! { static __EMPTY_NODE_CACHE_TLS: metamodelica::StaticArray<metamodelica::Ref<CachedData::CachedData>> = metamodelica::StaticArray::new(list![crate::NFInstNode::CachedData::interned_NO_CACHE(), crate::NFInstNode::CachedData::interned_NO_CACHE(), crate::NFInstNode::CachedData::interned_NO_CACHE()].into_iter().cloned().collect()); }
pub(crate) fn EMPTY_NODE_CACHE() -> metamodelica::StaticArray<metamodelica::Ref<CachedData::CachedData>> {
    __EMPTY_NODE_CACHE_TLS.with(|__t| __t.share())
}

thread_local! { static __INTEGER_DUMMY_NODE_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::CLASS_NODE { name: literal!("Integer"), definition: DUMMY_ELEMENT.clone(), visibility: Visibility::PUBLIC.clone(), cls: Pointer::createImmutable(crate::NFClass::interned_NOT_INSTANTIATED()), caches: EMPTY_NODE_CACHE().clone(), owner: None, identity: None, parentScope: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS() }); }
pub(crate) fn INTEGER_DUMMY_NODE() -> metamodelica::Ref<InstNode::InstNode> {
    __INTEGER_DUMMY_NODE_TLS.with(|__t| __t.clone())
}

thread_local! { static __INTEGER_FUNCTION_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("Integer") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INTEGER_DUMMY_NODE().clone() }), inputs: list![ENUM_PARAM().clone()], outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: ENUM_PARAM().clone(), ty: SlotType::POSITIONAL.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn INTEGER_FUNCTION() -> metamodelica::Ref<Function::Function> {
    __INTEGER_FUNCTION_TLS.with(|__t| __t.clone())
}

thread_local! { static __INTEGER_NODE_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::CLASS_NODE { name: literal!("IntegerFunc"), definition: DUMMY_ELEMENT.clone(), visibility: Visibility::PUBLIC.clone(), cls: Pointer::createImmutable(metamodelica::Ref::new(Class::NFClass::INSTANCED_CLASS { ty: crate::NFType::interned_UNKNOWN(), elements: crate::NFClassTree::ClassTree::interned_EMPTY_TREE(), sections: crate::NFSections::interned_EMPTY(), prefixes: Class::DEFAULT_PREFIXES.clone(), restriction: crate::NFRestriction::interned_FUNCTION() })), caches: metamodelica::arrayFromVec(list![metamodelica::Ref::new(CachedData::CachedData::FUNCTION { funcs: list![INTEGER_FUNCTION().clone()], typed: true, specialBuiltin: false }), crate::NFInstNode::CachedData::interned_NO_CACHE(), crate::NFInstNode::CachedData::interned_NO_CACHE()].into_iter().cloned().collect()), owner: None, identity: None, parentScope: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_BUILTIN_CLASS() }); }
pub(crate) fn INTEGER_NODE() -> metamodelica::Ref<InstNode::InstNode> {
    __INTEGER_NODE_TLS.with(|__t| __t.clone())
}

thread_local! { static __INTEGER_CREF_TLS: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF { node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INTEGER_NODE().clone() }), subscripts: metamodelica::nil(), ty: crate::NFType::interned_INTEGER(), origin: Origin::CREF.clone(), restCref: crate::NFComponentRef::interned_EMPTY() }); }
pub(crate) fn INTEGER_CREF() -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    __INTEGER_CREF_TLS.with(|__t| __t.clone())
}

thread_local! { static __STRING_DUMMY_NODE_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::CLASS_NODE { name: literal!("String"), definition: DUMMY_ELEMENT.clone(), visibility: Visibility::PUBLIC.clone(), cls: Pointer::createImmutable(crate::NFClass::interned_NOT_INSTANTIATED()), caches: EMPTY_NODE_CACHE().clone(), owner: None, identity: None, parentScope: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS() }); }
pub(crate) fn STRING_DUMMY_NODE() -> metamodelica::Ref<InstNode::InstNode> {
    __STRING_DUMMY_NODE_TLS.with(|__t| __t.clone())
}

thread_local! { static __R_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("r"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(REAL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn R_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __R_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __I_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("i"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(INT_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn I_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __I_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __B_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("b"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(BOOL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn B_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __B_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __E_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("e"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(ENUM_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn E_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __E_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __SIGNIFICANT_DIGITS_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("significantDigits"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(INT_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn SIGNIFICANT_DIGITS_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __SIGNIFICANT_DIGITS_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __MINIMUM_LENGTH_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("minimumLength"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(INT_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn MINIMUM_LENGTH_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __MINIMUM_LENGTH_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __LEFT_JUSTIFIED_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("leftJustified"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(BOOL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn LEFT_JUSTIFIED_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __LEFT_JUSTIFIED_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __FORMAT_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("format"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(STRING_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn FORMAT_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __FORMAT_PARAM_TLS.with(|__t| __t.clone())
}

// String(r, significantDigits=d, minimumLength=0, leftJustified=true)
thread_local! { static __STRING_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("String") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_DUMMY_NODE().clone() }), inputs: list![REAL_PARAM().clone(), INT_PARAM().clone(), INT_PARAM().clone(), BOOL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: R_PARAM().clone(), ty: SlotType::POSITIONAL.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: SIGNIFICANT_DIGITS_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 6 })), arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: MINIMUM_LENGTH_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })), arg: None, index: 3, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: LEFT_JUSTIFIED_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })), arg: None, index: 4, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_STRING(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn STRING_REAL() -> metamodelica::Ref<Function::Function> {
    __STRING_REAL_TLS.with(|__t| __t.clone())
}

// String(r, format="-0.6g")
thread_local! { static __STRING_REAL_FORMAT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("String") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_DUMMY_NODE().clone() }), inputs: list![REAL_PARAM().clone(), STRING_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: R_PARAM().clone(), ty: SlotType::POSITIONAL.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: FORMAT_PARAM().clone(), ty: SlotType::NAMED.clone(), default: None, arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_STRING(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn STRING_REAL_FORMAT() -> metamodelica::Ref<Function::Function> {
    __STRING_REAL_FORMAT_TLS.with(|__t| __t.clone())
}

// String(i, minimumLength=0, leftJustified=true)
thread_local! { static __STRING_INT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("String") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_DUMMY_NODE().clone() }), inputs: list![INT_PARAM().clone(), INT_PARAM().clone(), BOOL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: I_PARAM().clone(), ty: SlotType::POSITIONAL.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: MINIMUM_LENGTH_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })), arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: LEFT_JUSTIFIED_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })), arg: None, index: 3, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_STRING(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn STRING_INT() -> metamodelica::Ref<Function::Function> {
    __STRING_INT_TLS.with(|__t| __t.clone())
}

// String(b, minimumLength=0, leftJustified=true)
thread_local! { static __STRING_BOOL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("String") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_DUMMY_NODE().clone() }), inputs: list![BOOL_PARAM().clone(), INT_PARAM().clone(), BOOL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: B_PARAM().clone(), ty: SlotType::POSITIONAL.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: MINIMUM_LENGTH_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })), arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: LEFT_JUSTIFIED_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })), arg: None, index: 3, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_STRING(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn STRING_BOOL() -> metamodelica::Ref<Function::Function> {
    __STRING_BOOL_TLS.with(|__t| __t.clone())
}

// String(e, minimumLength=0, leftJustified=true)
thread_local! { static __STRING_ENUM_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("String") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_DUMMY_NODE().clone() }), inputs: list![ENUM_PARAM().clone(), INT_PARAM().clone(), BOOL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: E_PARAM().clone(), ty: SlotType::POSITIONAL.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: MINIMUM_LENGTH_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })), arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: LEFT_JUSTIFIED_PARAM().clone(), ty: SlotType::NAMED.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })), arg: None, index: 3, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_STRING(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn STRING_ENUM() -> metamodelica::Ref<Function::Function> {
    __STRING_ENUM_TLS.with(|__t| __t.clone())
}

thread_local! { static __STRING_NODE_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::CLASS_NODE { name: literal!("String"), definition: DUMMY_ELEMENT.clone(), visibility: Visibility::PUBLIC.clone(), cls: Pointer::createImmutable(metamodelica::Ref::new(Class::NFClass::PARTIAL_BUILTIN { ty: crate::NFType::interned_STRING(), elements: crate::NFClassTree::ClassTree::interned_EMPTY_TREE(), modifier: crate::NFModifier::Modifier::interned_NOMOD(), prefixes: Class::DEFAULT_PREFIXES.clone(), restriction: crate::NFRestriction::interned_TYPE() })), caches: metamodelica::arrayFromVec(list![metamodelica::Ref::new(CachedData::CachedData::FUNCTION { funcs: list![STRING_ENUM().clone(), STRING_INT().clone(), STRING_BOOL().clone(), STRING_REAL().clone(), STRING_REAL_FORMAT().clone()], typed: true, specialBuiltin: true }), crate::NFInstNode::CachedData::interned_NO_CACHE(), crate::NFInstNode::CachedData::interned_NO_CACHE()].into_iter().cloned().collect()), owner: None, identity: None, parentScope: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_BUILTIN_CLASS() }); }
pub(crate) fn STRING_NODE() -> metamodelica::Ref<InstNode::InstNode> {
    __STRING_NODE_TLS.with(|__t| __t.clone())
}

thread_local! { static __STRING_CREF_TLS: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF { node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_NODE().clone() }), subscripts: metamodelica::nil(), ty: crate::NFType::interned_INTEGER(), origin: Origin::CREF.clone(), restCref: crate::NFComponentRef::interned_EMPTY() }); }
pub(crate) fn STRING_CREF() -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    __STRING_CREF_TLS.with(|__t| __t.clone())
}

// TODO: Sort these functions ...
thread_local! { static __COS_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("cos") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn COS_REAL() -> metamodelica::Ref<Function::Function> {
    __COS_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __SIN_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("sin") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn SIN_REAL() -> metamodelica::Ref<Function::Function> {
    __SIN_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __TAN_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("tan") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn TAN_REAL() -> metamodelica::Ref<Function::Function> {
    __TAN_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ACOS_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("acos") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ACOS_REAL() -> metamodelica::Ref<Function::Function> {
    __ACOS_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ASIN_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("asin") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ASIN_REAL() -> metamodelica::Ref<Function::Function> {
    __ASIN_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ATAN_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("atan") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ATAN_REAL() -> metamodelica::Ref<Function::Function> {
    __ATAN_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __COSH_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("cosh") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn COSH_REAL() -> metamodelica::Ref<Function::Function> {
    __COSH_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __SINH_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("sinh") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn SINH_REAL() -> metamodelica::Ref<Function::Function> {
    __SINH_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __TANH_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("tanh") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn TANH_REAL() -> metamodelica::Ref<Function::Function> {
    __TANH_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ACOSH_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("acosh") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ACOSH_REAL() -> metamodelica::Ref<Function::Function> {
    __ACOSH_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ASINH_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("asinh") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ASINH_REAL() -> metamodelica::Ref<Function::Function> {
    __ASINH_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ATANH_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("atanh") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ATANH_REAL() -> metamodelica::Ref<Function::Function> {
    __ATANH_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __EXP_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("exp") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn EXP_REAL() -> metamodelica::Ref<Function::Function> {
    __EXP_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __LOG_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("log") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn LOG_REAL() -> metamodelica::Ref<Function::Function> {
    __LOG_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __LOG10_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("log10") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn LOG10_REAL() -> metamodelica::Ref<Function::Function> {
    __LOG10_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ABS_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("abs") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn ABS_REAL() -> metamodelica::Ref<Function::Function> {
    __ABS_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __SIGN_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("sign") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn SIGN() -> metamodelica::Ref<Function::Function> {
    __SIGN_TLS.with(|__t| __t.clone())
}

thread_local! { static __MAX_INT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("max") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![INT_PARAM().clone(), INT_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn MAX_INT() -> metamodelica::Ref<Function::Function> {
    __MAX_INT_TLS.with(|__t| __t.clone())
}

thread_local! { static __MAX_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("max") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone(), REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn MAX_REAL() -> metamodelica::Ref<Function::Function> {
    __MAX_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __MIN_INT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("min") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![INT_PARAM().clone(), INT_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn MIN_INT() -> metamodelica::Ref<Function::Function> {
    __MIN_INT_TLS.with(|__t| __t.clone())
}

thread_local! { static __MIN_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("min") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone(), REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn MIN_REAL() -> metamodelica::Ref<Function::Function> {
    __MIN_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ARG_MIN_ARR_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("argmin") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ARG_MIN_ARR_REAL() -> metamodelica::Ref<Function::Function> {
    __ARG_MIN_ARR_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ARG_MAX_ARR_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("argmax") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn ARG_MAX_ARR_REAL() -> metamodelica::Ref<Function::Function> {
    __ARG_MAX_ARR_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __DIV_INT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("div") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![INT_PARAM().clone(), INT_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn DIV_INT() -> metamodelica::Ref<Function::Function> {
    __DIV_INT_TLS.with(|__t| __t.clone())
}

thread_local! { static __DIV_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("div") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone(), REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn DIV_REAL() -> metamodelica::Ref<Function::Function> {
    __DIV_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __FLOOR_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("floor") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn FLOOR() -> metamodelica::Ref<Function::Function> {
    __FLOOR_TLS.with(|__t| __t.clone())
}

thread_local! { static __INTEGER_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("integer") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn INTEGER_REAL() -> metamodelica::Ref<Function::Function> {
    __INTEGER_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __INTEGER_ENUM_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("Integer") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![ENUM_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn INTEGER_ENUM() -> metamodelica::Ref<Function::Function> {
    __INTEGER_ENUM_TLS.with(|__t| __t.clone())
}

thread_local! { static __POSITIVE_MAX_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("$OMC$PositiveMax") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone(), REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn POSITIVE_MAX_REAL() -> metamodelica::Ref<Function::Function> {
    __POSITIVE_MAX_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __INSTREAM_DIV_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("$OMC$inStreamDiv") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone(), REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn INSTREAM_DIV_REAL() -> metamodelica::Ref<Function::Function> {
    __INSTREAM_DIV_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __IN_STREAM_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("inStream") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn IN_STREAM() -> metamodelica::Ref<Function::Function> {
    __IN_STREAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __PROMOTE_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("promote") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn PROMOTE() -> metamodelica::Ref<Function::Function> {
    __PROMOTE_TLS.with(|__t| __t.clone())
}

thread_local! { static __CAT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("cat") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn CAT() -> metamodelica::Ref<Function::Function> {
    __CAT_TLS.with(|__t| __t.clone())
}

thread_local! { static __ARRAY_FUNC_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("array") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn ARRAY_FUNC() -> metamodelica::Ref<Function::Function> {
    __ARRAY_FUNC_TLS.with(|__t| __t.clone())
}

thread_local! { static __FILL_FUNC_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("fill") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn FILL_FUNC() -> metamodelica::Ref<Function::Function> {
    __FILL_FUNC_TLS.with(|__t| __t.clone())
}

thread_local! { static __SMOOTH_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("smooth") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn SMOOTH() -> metamodelica::Ref<Function::Function> {
    __SMOOTH_TLS.with(|__t| __t.clone())
}

thread_local! { static __NO_EVENT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("noEvent") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn NO_EVENT() -> metamodelica::Ref<Function::Function> {
    __NO_EVENT_TLS.with(|__t| __t.clone())
}

thread_local! { static __INITIAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("initial") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_BOOLEAN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn INITIAL() -> metamodelica::Ref<Function::Function> {
    __INITIAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __PRE_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("pre") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn PRE() -> metamodelica::Ref<Function::Function> {
    __PRE_TLS.with(|__t| __t.clone())
}

thread_local! { static __PREVIOUS_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("previous") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn PREVIOUS() -> metamodelica::Ref<Function::Function> {
    __PREVIOUS_TLS.with(|__t| __t.clone())
}

thread_local! { static __MAX_INT_ARR_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("max") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![INT_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn MAX_INT_ARR() -> metamodelica::Ref<Function::Function> {
    __MAX_INT_ARR_TLS.with(|__t| __t.clone())
}

thread_local! { static __SUM_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("sum") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn SUM() -> metamodelica::Ref<Function::Function> {
    __SUM_TLS.with(|__t| __t.clone())
}

thread_local! { static __FMU_LOAD_RESOURCE_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("OpenModelica_fmuLoadResource") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![STRING_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: STRING_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_STRING(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn FMU_LOAD_RESOURCE() -> metamodelica::Ref<Function::Function> {
    __FMU_LOAD_RESOURCE_TLS.with(|__t| __t.clone())
}

thread_local! { static __SAMPLE_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::QUALIFIED { name: literal!("OMC_NO_CLOCK"), path: metamodelica::Ref::new(Path::IDENT { name: literal!("sample") }) }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone(), REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_BOOLEAN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn SAMPLE() -> metamodelica::Ref<Function::Function> {
    __SAMPLE_TLS.with(|__t| __t.clone())
}

thread_local! { static __SAMPLE_CLOCKED_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("sample") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone(), CLOCK_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn SAMPLE_CLOCKED() -> metamodelica::Ref<Function::Function> {
    __SAMPLE_CLOCKED_TLS.with(|__t| __t.clone())
}

thread_local! { static __TRANSPOSE_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("transpose") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: metamodelica::nil(), outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn TRANSPOSE() -> metamodelica::Ref<Function::Function> {
    __TRANSPOSE_TLS.with(|__t| __t.clone())
}

thread_local! { static __CLOCK_COMPONENT_TLS: metamodelica::Ref<Component::NFComponent> = metamodelica::Ref::new(Component::NFComponent::COMPONENT { classInst: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), ty: crate::NFType::interned_CLOCK(), binding: NFBinding::EMPTY_BINDING().clone(), condition: NFBinding::EMPTY_BINDING().clone(), attributes: Attributes::DEFAULT_ATTR().clone(), comment: SCode::noComment.clone(), state: ComponentState::TypeChecked.clone(), info: Absyn::dummyInfo.clone() }); }
pub(crate) fn CLOCK_COMPONENT() -> metamodelica::Ref<Component::NFComponent> {
    __CLOCK_COMPONENT_TLS.with(|__t| __t.clone())
}

thread_local! { static __CLOCK_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("s"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(CLOCK_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn CLOCK_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __CLOCK_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __CLOCK_DUMMY_NODE_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::CLASS_NODE { name: literal!("Clock"), definition: DUMMY_ELEMENT.clone(), visibility: Visibility::PUBLIC.clone(), cls: Pointer::createImmutable(crate::NFClass::interned_NOT_INSTANTIATED()), caches: EMPTY_NODE_CACHE().clone(), owner: None, identity: None, parentScope: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS() }); }
pub(crate) fn CLOCK_DUMMY_NODE() -> metamodelica::Ref<InstNode::InstNode> {
    __CLOCK_DUMMY_NODE_TLS.with(|__t| __t.clone())
}

// Clock() - inferred clock
thread_local! { static __CLOCK_INFERRED_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("Clock") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_DUMMY_NODE().clone() }), inputs: metamodelica::nil(), outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_CLOCK(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn CLOCK_INFERRED() -> metamodelica::Ref<Function::Function> {
    __CLOCK_INFERRED_TLS.with(|__t| __t.clone())
}

thread_local! { static __INTERVAL_COUNTER_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("intervalCounter"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(INT_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn INTERVAL_COUNTER_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __INTERVAL_COUNTER_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __RESOLUTION_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("resolution"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(INT_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn RESOLUTION_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __RESOLUTION_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __INTERVAL_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("interval"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(REAL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn INTERVAL_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __INTERVAL_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __CONDITION_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("condition"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(BOOL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn CONDITION_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __CONDITION_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __START_INTERVAL_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("startInterval"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(REAL_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn START_INTERVAL_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __START_INTERVAL_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __C_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("c"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(CLOCK_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn C_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __C_PARAM_TLS.with(|__t| __t.clone())
}

thread_local! { static __SOLVER_METHOD_PARAM_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::COMPONENT_NODE { name: literal!("solverMethod"), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::createImmutable(STRING_COMPONENT().clone()), owner: None, identity: None, parent: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP() }); }
pub(crate) fn SOLVER_METHOD_PARAM() -> metamodelica::Ref<InstNode::InstNode> {
    __SOLVER_METHOD_PARAM_TLS.with(|__t| __t.clone())
}

// Clock(intervalCounter, resolution = 1) - clock with Integer interval
thread_local! { static __CLOCK_INT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("Clock") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_DUMMY_NODE().clone() }), inputs: list![INT_PARAM().clone(), INT_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: INTERVAL_COUNTER_PARAM().clone(), ty: SlotType::GENERIC.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: RESOLUTION_PARAM().clone(), ty: SlotType::GENERIC.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })), arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_CLOCK(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn CLOCK_INT() -> metamodelica::Ref<Function::Function> {
    __CLOCK_INT_TLS.with(|__t| __t.clone())
}

// Clock(interval) - clock with Real interval
thread_local! { static __CLOCK_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("Clock") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_DUMMY_NODE().clone() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: INTERVAL_PARAM().clone(), ty: SlotType::GENERIC.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_CLOCK(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn CLOCK_REAL() -> metamodelica::Ref<Function::Function> {
    __CLOCK_REAL_TLS.with(|__t| __t.clone())
}

// Clock(condition, startInterval = 0.0) - Event clock, triggered by zero-crossing events
thread_local! { static __CLOCK_BOOL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("Clock") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_DUMMY_NODE().clone() }), inputs: list![BOOL_PARAM().clone(), REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: CONDITION_PARAM().clone(), ty: SlotType::GENERIC.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: START_INTERVAL_PARAM().clone(), ty: SlotType::GENERIC.clone(), default: Some(metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) })), arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_CLOCK(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn CLOCK_BOOL() -> metamodelica::Ref<Function::Function> {
    __CLOCK_BOOL_TLS.with(|__t| __t.clone())
}

// Clock(c, solverMethod) - Solver clock
thread_local! { static __CLOCK_SOLVER_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("Clock") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_DUMMY_NODE().clone() }), inputs: list![CLOCK_PARAM().clone(), STRING_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: list![metamodelica::Ref::new(Slot::Slot { node: C_PARAM().clone(), ty: SlotType::GENERIC.clone(), default: None, arg: None, index: 1, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() }), metamodelica::Ref::new(Slot::Slot { node: SOLVER_METHOD_PARAM().clone(), ty: SlotType::GENERIC.clone(), default: None, arg: None, index: 2, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() })], returnType: crate::NFType::interned_CLOCK(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub(crate) fn CLOCK_SOLVER() -> metamodelica::Ref<Function::Function> {
    __CLOCK_SOLVER_TLS.with(|__t| __t.clone())
}

thread_local! { static __CLOCK_NODE_TLS: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::InstNode::CLASS_NODE { name: literal!("Clock"), definition: DUMMY_ELEMENT.clone(), visibility: Visibility::PUBLIC.clone(), cls: Pointer::createImmutable(metamodelica::Ref::new(Class::NFClass::PARTIAL_BUILTIN { ty: crate::NFType::interned_CLOCK(), elements: crate::NFClassTree::ClassTree::interned_EMPTY_TREE(), modifier: crate::NFModifier::Modifier::interned_NOMOD(), prefixes: Class::DEFAULT_PREFIXES.clone(), restriction: crate::NFRestriction::interned_TYPE() })), caches: metamodelica::arrayFromVec(list![metamodelica::Ref::new(CachedData::CachedData::FUNCTION { funcs: list![CLOCK_INFERRED().clone(), CLOCK_INT().clone(), CLOCK_REAL().clone(), CLOCK_BOOL().clone(), CLOCK_SOLVER().clone()], typed: true, specialBuiltin: true }), crate::NFInstNode::CachedData::interned_NO_CACHE(), crate::NFInstNode::CachedData::interned_NO_CACHE()].into_iter().cloned().collect()), owner: None, identity: None, parentScope: NFInstNode::NO_SCOPE().clone(), nodeType: crate::NFInstNode::InstNodeType::interned_BUILTIN_CLASS() }); }
pub(crate) fn CLOCK_NODE() -> metamodelica::Ref<InstNode::InstNode> {
    __CLOCK_NODE_TLS.with(|__t| __t.clone())
}

thread_local! { static __CLOCK_CREF_TLS: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF { node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_NODE().clone() }), subscripts: metamodelica::nil(), ty: crate::NFType::interned_INTEGER(), origin: Origin::CREF.clone(), restCref: crate::NFComponentRef::interned_EMPTY() }); }
pub(crate) fn CLOCK_CREF() -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    __CLOCK_CREF_TLS.with(|__t| __t.clone())
}

thread_local! { static __GET_PART_REAL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("$getPart") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![REAL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: REAL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_REAL(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn GET_PART_REAL() -> metamodelica::Ref<Function::Function> {
    __GET_PART_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __GET_PART_INT_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("$getPart") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![INT_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: INT_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_INTEGER(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn GET_PART_INT() -> metamodelica::Ref<Function::Function> {
    __GET_PART_INT_TLS.with(|__t| __t.clone())
}

thread_local! { static __GET_PART_BOOL_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("$getPart") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![BOOL_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: BOOL_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_BOOLEAN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn GET_PART_BOOL() -> metamodelica::Ref<Function::Function> {
    __GET_PART_BOOL_TLS.with(|__t| __t.clone())
}

thread_local! { static __GET_PART_CLOCK_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("$getPart") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![CLOCK_PARAM().clone()], outputs: list![metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: CLOCK_PARAM().clone() })], locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_BOOLEAN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn GET_PART_CLOCK() -> metamodelica::Ref<Function::Function> {
    __GET_PART_CLOCK_TLS.with(|__t| __t.clone())
}

thread_local! { static __CLOCK_FIRE_TLS: metamodelica::Ref<Function::Function> = metamodelica::Ref::new(Function::Function { path: metamodelica::Ref::new(Path::IDENT { name: literal!("$_clkfire") }), node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: crate::NFInstNode::InstNode::interned_EMPTY_NODE() }), inputs: list![INT_PARAM().clone()], outputs: metamodelica::nil(), locals: metamodelica::nil(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: crate::NFType::interned_UNKNOWN(), attributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::createImmutable(FunctionStatus::BUILTIN.clone()), callCounter: Pointer::createImmutable(0) }); }
pub fn CLOCK_FIRE() -> metamodelica::Ref<Function::Function> {
    __CLOCK_FIRE_TLS.with(|__t| __t.clone())
}

pub(crate) static BASE_MODELICA_POSITIVE_MAX_SIMPLE: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("$OMC$PositiveMax"),
            prefixes: SCode::defaultPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: openmodelica_ast::Absyn::FunctionPurity::NO_PURITY,
                },
            },
            classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                elementLst: list![
                    metamodelica::Ref::new(SCode::Element::COMPONENT {
                        name: literal!("flowValue"),
                        prefixes: SCode::defaultPrefixes.clone(),
                        attributes: SCode::defaultInputAttr.clone(),
                        typeSpec: metamodelica::Ref::new(TypeSpec::TPATH {
                            path: metamodelica::Ref::new(Path::IDENT { name: literal!("Real") }),
                            arrayDim: None
                        }),
                        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                        comment: SCode::noComment.clone(),
                        condition: None,
                        info: Absyn::dummyInfo.clone()
                    }),
                    metamodelica::Ref::new(SCode::Element::COMPONENT {
                        name: literal!("eps"),
                        prefixes: SCode::defaultPrefixes.clone(),
                        attributes: SCode::defaultInputAttr.clone(),
                        typeSpec: metamodelica::Ref::new(TypeSpec::TPATH {
                            path: metamodelica::Ref::new(Path::IDENT { name: literal!("Real") }),
                            arrayDim: None
                        }),
                        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                        comment: SCode::noComment.clone(),
                        condition: None,
                        info: Absyn::dummyInfo.clone()
                    }),
                    metamodelica::Ref::new(SCode::Element::COMPONENT {
                        name: literal!("positiveMax"),
                        prefixes: SCode::defaultPrefixes.clone(),
                        attributes: SCode::defaultOutputAttr.clone(),
                        typeSpec: metamodelica::Ref::new(TypeSpec::TPATH {
                            path: metamodelica::Ref::new(Path::IDENT { name: literal!("Real") }),
                            arrayDim: None
                        }),
                        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                        comment: SCode::noComment.clone(),
                        condition: None,
                        info: Absyn::dummyInfo.clone()
                    })
                ],
                normalEquationLst: metamodelica::nil(),
                initialEquationLst: metamodelica::nil(),
                normalAlgorithmLst: list![metamodelica::Ref::new(SCode::AlgorithmSection {
                    statements: list![metamodelica::Ref::new(SCode::Statement::ALG_ASSIGN {
                        assignComponent: metamodelica::Ref::new(Absyn::Exp::CREF {
                            componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                                name: literal!("positiveMax"),
                                subscripts: metamodelica::nil()
                            })
                        }),
                        value: metamodelica::Ref::new(Absyn::Exp::CALL {
                            function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                                name: literal!("max"),
                                subscripts: metamodelica::nil()
                            }),
                            functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
                                args: list![
                                    metamodelica::Ref::new(Absyn::Exp::CREF {
                                        componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                                            name: literal!("flowValue"),
                                            subscripts: metamodelica::nil()
                                        })
                                    }),
                                    metamodelica::Ref::new(Absyn::Exp::CREF {
                                        componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                                            name: literal!("eps"),
                                            subscripts: metamodelica::nil()
                                        })
                                    })
                                ],
                                argNames: metamodelica::nil()
                            }),
                            typeVars: metamodelica::nil()
                        }),
                        comment: SCode::noComment.clone(),
                        info: Absyn::dummyInfo.clone()
                    })]
                })],
                initialAlgorithmLst: metamodelica::nil(),
                constraintLst: metamodelica::nil(),
                clsattrs: metamodelica::nil(),
                externalDecl: None,
            }),
            cmt: SCode::noComment.clone(),
            info: Absyn::dummyInfo.clone(),
        })
    });
