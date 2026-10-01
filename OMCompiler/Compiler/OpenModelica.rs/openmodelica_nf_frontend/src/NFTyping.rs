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

use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFConnection as Connection;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFModifier::ModTable;
use crate::NFModifier::Modifier;
use crate::NFOperator as Operator;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPackage as Package;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

pub mod TypingError {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum TypingError {
        NO_ERROR,
        OUT_OF_BOUNDS { upperBound: i32 },
        UNKNOWN_TYPE,
    }
    impl metamodelica::gc::MMTrace for TypingError {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                TypingError::NO_ERROR => Ok(()),
                TypingError::OUT_OF_BOUNDS { upperBound } => {
                    metamodelica::gc::MMTrace::mm_accept(upperBound, __mmv)?;
                    Ok(())
                }
                TypingError::UNKNOWN_TYPE => Ok(()),
            }
        }
    }
    impl TypingError {
        pub fn interned_NO_ERROR() -> metamodelica::Ref<TypingError> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypingError>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(TypingError::NO_ERROR));
            (*INTERNED).clone()
        }
        pub fn interned_UNKNOWN_TYPE() -> metamodelica::Ref<TypingError> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypingError>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(TypingError::UNKNOWN_TYPE));
            (*INTERNED).clone()
        }
    }
    pub fn interned_NO_ERROR() -> metamodelica::Ref<TypingError> {
        TypingError::interned_NO_ERROR()
    }
    pub fn interned_UNKNOWN_TYPE() -> metamodelica::Ref<TypingError> {
        TypingError::interned_UNKNOWN_TYPE()
    }
    impl Default for TypingError {
        fn default() -> Self {
            Self::NO_ERROR
        }
    }
    pub(crate) use self::TypingError::{NO_ERROR, OUT_OF_BOUNDS, UNKNOWN_TYPE};
    pub(crate) fn isError(mut error: &metamodelica::Ref<TypingError>) -> bool {
        let mut isError: bool;
        isError = (match &**error {
            NO_ERROR { .. } => false,
            _ => true,
        });
        isError
    }
}

// Used by typeDimension for catching cyclic dimension involving :
thread_local! { static __WHOLEDIM_CREF_TLS: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::NFExpression::CREF { ty: crate::NFType::interned_UNKNOWN(), cref: metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF { node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: literal!(":") }) }), subscripts: metamodelica::nil(), ty: crate::NFType::interned_UNKNOWN(), origin: ComponentRef::Origin::CREF.clone(), restCref: crate::NFComponentRef::interned_EMPTY() }) }); }
pub(crate) fn WHOLEDIM_CREF() -> metamodelica::Ref<Expression::NFExpression> {
    __WHOLEDIM_CREF_TLS.with(|__t| __t.clone())
}

pub fn typeClass(mut cls: metamodelica::Ref<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut next_context: i32;
    next_context = InstContext::set(context, InstContext::CLASS.clone());
    typeClassType(
        cls.clone(),
        &(Binding::EMPTY_BINDING().clone()),
        next_context,
        &(cls.clone()),
    )?;
    typeComponents(cls.clone(), next_context, false)?;
    execStat(&(literal!("NFTyping.typeComponents")))?;
    typeBindings(cls.clone(), next_context)?;
    execStat(&(literal!("NFTyping.typeBindings")))?;
    typeClassSections(cls, next_context)?;
    execStat(&(literal!("NFTyping.typeClassSections")))?;
    Ok(())
}

pub fn typeComponents(
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut preserveDerived: bool,
) -> Result<()> {
    let mut c: metamodelica::Ref<Class::NFClass> = NFInstNode::InstNode::getClass(cls.clone())?;
    let mut c2: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut con: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut de: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut rec_con: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let () = (::match_deref::match_deref! { match &(&*c) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::TYPE, .. } => (),
        Deref @ Class::INSTANCED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            if InstContext::inInstanceAPI(context) {
                let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range0 {
                    typeComponentTry(c.clone(), context)?;
                }
            } else {
                let __range1 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    typeComponent(c.clone(), context, true)?;
                }
            }
            let () = (::match_deref::match_deref! { match &(var_field!((*c).ty, Class::NFClass::INSTANCED_CLASS).clone()) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: __esc_rec_con, .. }, .. } => {
            rec_con = (*__esc_rec_con).clone();
            typeStructor(NFInstNode::InstNode::borrow(rec_con.clone())?)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Class::TYPED_DERIVED { baseClass: __c_baseClass, ty: __c_ty, .. } if (preserveDerived || Type::isArray(metamodelica::AsArg::as_arg(&__c_ty))) => {
            typeComponents(__c_baseClass.clone(), context, false)?;
            ()
        },
        Deref @ Class::TYPED_DERIVED { baseClass: __c_baseClass, restriction: __c_restriction, .. } => {
            typeComponents(__c_baseClass.clone(), context, false)?;
            if !(InstContext::inInstanceAPI(context)) {
                c2 = NFInstNode::InstNode::getClass(__c_baseClass.clone())?;
                c2 = Class::setRestriction(__c_restriction.clone(), c2)?;
                NFInstNode::InstNode::updateClass(c2, cls)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __esc_con, destructor: __esc_de }, .. }, .. } => {
            con = (*__esc_con).clone();
            de = (*__esc_de).clone();
            typeStructor(NFInstNode::InstNode::borrow(con.clone())?)?;
            typeStructor(NFInstNode::InstNode::borrow(de.clone())?)?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponents")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&cls)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeStructor(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
    let mut cache: metamodelica::Ref<CachedData::CachedData>;
    let mut fnl: metamodelica::List<metamodelica::Ref<Function::Function>>;
    let mut context: i32;
    cache = NFInstNode::InstNode::getFuncCache(&node)?;
    let () = (match &*cache {
        NFInstNode::CachedData::FUNCTION {
            funcs: __esc_fnl,
            typed: false,
            specialBuiltin: __cache_specialBuiltin,
        } => {
            fnl = (*__esc_fnl).clone();
            context = InstContext::set(InstContext::FUNCTION.clone(), InstContext::RELAXED.clone());
            fnl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
                for mut r#fn in (fnl.clone()).into_iter().cloned() {
                    let __x = Function::typeFunction(r#fn.clone(), context)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            fnl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
                for mut r#fn in (fnl.clone()).into_iter().cloned() {
                    let __x = OperatorOverloading::patchOperatorRecordConstructorBinding(r#fn.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            NFInstNode::InstNode::setFuncCache(
                node,
                metamodelica::Ref::new(CachedData::CachedData::FUNCTION {
                    funcs: fnl.clone(),
                    typed: true,
                    specialBuiltin: __cache_specialBuiltin.clone(),
                }),
            )?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub fn typeClassType(
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut componentBinding: &metamodelica::Ref<Binding::NFBinding>,
    mut context: i32,
    mut instanceNode: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut ty_cls: metamodelica::Ref<Class::NFClass>;
    let mut ty_node: metamodelica::Ref<InstNode::InstNode>;
    let mut node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut cls_ty: metamodelica::Ref<Type::NFType>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut is_expandable: bool;
    cls = NFInstNode::InstNode::getClass(clsNode.clone())?;
    ty = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::CONNECTOR { isExpandable: __esc_is_expandable }, elements: __cls_elements, .. } => {
            is_expandable = (*__esc_is_expandable).clone();
            ty = metamodelica::Ref::new(Type::NFType::COMPLEX { cls: NFInstNode::InstNode::identityCell(clsNode.clone()), complexTy: makeConnectorType(metamodelica::AsArg::as_arg(&__cls_elements), is_expandable.clone())? });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            NFInstNode::InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { ty: __esc_cls_ty @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: __esc_node, .. }, .. }, .. } => {
            cls_ty = (*__esc_cls_ty).clone();
            node = (*__esc_node).clone();
            ty_node = Type::complexNode(metamodelica::AsArg::as_arg(&cls_ty))?;
            ty = metamodelica::Ref::new(Type::NFType::COMPLEX { cls: NFInstNode::InstNode::identityCell(ty_node), complexTy: makeRecordType(node.clone())? });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            NFInstNode::InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTENDS_TYPE { baseClass: __esc_node }, .. }, .. } => {
            node = (*__esc_node).clone();
            ty = typeClassType(NFInstNode::InstNode::borrow(node.clone())?, componentBinding, context, instanceNode)?;
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            NFInstNode::InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::FUNCTION, .. } if (NFInstNode::InstNode::isComponent(instanceNode)?) => {
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(clsNode.clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r#fn = metamodelica::Own::own(__pa0);
            ty = metamodelica::Ref::new(Type::NFType::FUNCTION { r#fn: r#fn, fnType: Type::FunctionType::FUNCTIONAL_PARAMETER.clone() });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            NFInstNode::InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { ty: __cls_ty, .. } => __cls_ty.clone(),
        Deref @ Class::EXPANDED_DERIVED { baseClass: __cls_baseClass, restriction: __cls_restriction, .. } => {
            typeDimensions(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone(), clsNode.clone(), componentBinding.clone(), context, NFInstNode::InstNode::info(&clsNode))?;
            ty = typeClassType(__cls_baseClass.clone(), componentBinding, context, instanceNode)?;
            ty = Type::liftArrayLeftList(ty, &(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            ty_cls = metamodelica::Ref::new(Class::NFClass::TYPED_DERIVED { ty: ty.clone(), baseClass: __cls_baseClass.clone(), restriction: __cls_restriction.clone() });
            NFInstNode::InstNode::updateClass(ty_cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_BUILTIN { ty: __cls_ty, .. } => __cls_ty.clone(),
        Deref @ Class::TYPED_DERIVED { ty: __cls_ty, .. } => __cls_ty.clone(),
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeClassType")); __mm_s.push_str(&*literal!(" got noninstantiated class ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&clsNode)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub(crate) fn makeConnectorType(
    mut ctree: &metamodelica::Ref<ClassTree::ClassTree>,
    mut isExpandable: bool,
) -> Result<metamodelica::Ref<ComplexType::NFComplexType>> {
    let mut connectorTy: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut pots: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>> =
        metamodelica::nil();
    let mut flows: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>> =
        metamodelica::nil();
    let mut streams: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>> =
        metamodelica::nil();
    let mut exps: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>> =
        metamodelica::nil();
    let mut cty: i32;
    if isExpandable {
        for mut c in &*ClassTree::enumerateComponents(ctree)? {
            cty = Component::connectorType(
                &(NFInstNode::InstNode::component(&(NFInstNode::InstNode::resolveInner(c.clone())))?),
            );
            if intBitAnd(cty, ConnectorType::EXPANDABLE.clone()) > 0 {
                exps = metamodelica::cons(NFInstNode::InstNode::scopeRef(c.clone()), exps);
            } else {
                pots = metamodelica::cons(NFInstNode::InstNode::scopeRef(c.clone()), pots);
            }
        }
        connectorTy = metamodelica::Ref::new(ComplexType::NFComplexType::EXPANDABLE_CONNECTOR {
            potentiallyPresents: pots,
            expandableConnectors: exps,
        });
    } else {
        for mut c in &*ClassTree::enumerateComponents(ctree)? {
            cty = Component::connectorType(
                &(NFInstNode::InstNode::component(&(NFInstNode::InstNode::resolveInner(c.clone())))?),
            );
            if intBitAnd(cty, ConnectorType::FLOW.clone()) > 0 {
                flows = metamodelica::cons(NFInstNode::InstNode::scopeRef(c.clone()), flows);
            } else if intBitAnd(cty, ConnectorType::STREAM.clone()) > 0 {
                streams = metamodelica::cons(NFInstNode::InstNode::scopeRef(c.clone()), streams);
            } else if intBitAnd(cty, ConnectorType::POTENTIAL.clone()) > 0 {
                pots = metamodelica::cons(NFInstNode::InstNode::scopeRef(c.clone()), pots);
            } else {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Invalid connector type on component "));
                        __mm_s.push_str(&*NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&c))?);
                        ArcStr::from(__mm_s)
                    },
                    NFInstNode::InstNode::info(metamodelica::AsArg::as_arg(&c)),
                )?;
                return Err("fail");
            }
        }
        connectorTy = metamodelica::Ref::new(ComplexType::NFComplexType::CONNECTOR {
            potentials: pots,
            flows: flows,
            streams: streams.clone(),
        });
        if !((streams).is_empty()) {
            System::setHasStreamConnectors(true);
        }
    }
    Ok(connectorTy)
}

pub(crate) fn checkConnectorTypeBalance(mut component: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
    let mut pots: i32;
    let mut flows: i32;
    let mut streams: i32;
    let mut known_size: bool;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    comp = NFInstNode::InstNode::component(&component)?;
    if !(Prefixes::ConnectorType::isConnector(Component::connectorType(&comp))) {
        return Ok(());
    }
    parent = NFInstNode::InstNode::instanceParent(component.clone())?;
    if NFInstNode::InstNode::isComponent(&parent)?
        && Component::isConnector(&(NFInstNode::InstNode::component(&parent)?))
    {
        return Ok(());
    }
    (pots, flows, streams, known_size) = Component::countConnectorVars(&comp, true)?;
    if !(known_size) {
        return Ok(());
    }
    if pots != flows
        && !(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), literal!("unbalancedModel"))?)
    {
        Error::addStrictMessage(
            Error::UNBALANCED_CONNECTOR.clone(),
            list![
                NFInstNode::InstNode::name(&component)?,
                ArcStr::from(::std::format!("{}", pots)),
                ArcStr::from(::std::format!("{}", flows))
            ],
            &(NFInstNode::InstNode::info(&component)),
        )?;
    }
    if streams > 0 && flows != 1 {
        Error::addSourceMessage(
            &(Error::MISMATCHED_FLOW_IN_STREAM_CONNECTOR.clone()),
            list![
                NFInstNode::InstNode::name(&component)?,
                ArcStr::from(::std::format!("{}", flows))
            ],
            &(NFInstNode::InstNode::info(&component)),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn makeRecordType(
    mut constructor: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
) -> Result<metamodelica::Ref<ComplexType::NFComplexType>> {
    let mut recordTy: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut cache: metamodelica::Ref<CachedData::CachedData>;
    let mut r#fn: metamodelica::Ref<Function::Function> =
        <metamodelica::Ref<Function::Function> as ::std::default::Default>::default();
    let mut fields: metamodelica::Array<metamodelica::Ref<Record::Field::Field>> = Default::default();
    let mut indexMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>> =
        <metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>> as ::std::default::Default>::default();
    cache = NFInstNode::InstNode::getFuncCache(&(NFInstNode::InstNode::borrow(constructor.clone())?))?;
    recordTy = 'mc: {
        let __mc_input = &*cache;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFInstNode::CachedData::FUNCTION { .. } => {
                    let mut fields: metamodelica::Array<metamodelica::Ref<Record::Field::Field>> = fields.clone();
                    let mut r#fn: metamodelica::Ref<Function::Function> = r#fn.clone();
                    let mut indexMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>> = indexMap.clone();
                    r#fn = List::find(var_field!((*cache).funcs, CachedData::CachedData::FUNCTION), &move |__a0: metamodelica::Ref<Function::Function>| Function::isDefaultRecordConstructor(&__a0))?;
                    (fields, indexMap) = Record::collectRecordFields(NFInstNode::InstNode::fromHandle(&r#fn.node)?)?;
                    Ok((metamodelica::Ref::new(ComplexType::NFComplexType::RECORD { constructor: constructor.clone(), fields: fields.clone(), indexMap: indexMap.clone() }), fields.clone(), r#fn.clone(), indexMap.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            fields = __wb0;
            r#fn = __wb1;
            indexMap = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.makeRecordType")); __mm_s.push_str(&*literal!(" got record type without constructor")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(recordTy)
}

pub(crate) fn typeComponent(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut typeChildren: bool,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType> = metamodelica::Ref::new(Type::ANY);
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut c: metamodelica::Ref<Component::NFComponent>;
    let mut is_deleted: bool;
    let mut dims: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    if NFInstNode::InstNode::isEmpty(&component) || NFInstNode::InstNode::isOnlyOuter(&component)? {
        return Ok(ty);
    }
    node = NFInstNode::InstNode::resolveOuter(component.clone());
    c = NFInstNode::InstNode::component(&node)?;
    ty = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: __esc_dims, .. }, binding: __c_binding, classInst: __c_classInst, info: __c_info, .. } => {
            dims = (*__esc_dims).clone();
            typeDimensions(dims.clone(), node.clone(), __c_binding.clone(), context, __c_info.clone())?;
            if NFInstNode::InstNode::isEmpty(metamodelica::AsArg::as_arg(&__c_classInst)) {
                ty = crate::NFType::interned_UNKNOWN();
            } else {
                ty = typeClassType(__c_classInst.clone(), metamodelica::AsArg::as_arg(&__c_binding), context, &component)?;
            }
            ty = Type::liftArrayLeftList(ty, &(dims.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            if Binding::isBound(var_field!((*c).condition, Component::NFComponent::COMPONENT)) {
                assign_variant_field!(c => Component::NFComponent::COMPONENT; condition = typeComponentCondition(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone(), context, true)?);
                is_deleted = Expression::isFalse(&(Binding::getExp(var_field!((*c).condition, Component::NFComponent::COMPONENT))?));
            } else {
                is_deleted = false;
            }
            if typeChildren {
                assign_variant_field!(c => Component::NFComponent::COMPONENT;
                    ty = ty.clone(),
                    state = ComponentState::Typed.clone()
                );
                NFInstNode::InstNode::updateComponent(c.clone(), node.clone())?;
                if !(is_deleted) && !(NFInstNode::InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT))) {
                    checkComponentStreamAttribute(var_field!((*c).attributes, Component::NFComponent::COMPONENT).connectorType.clone(), &ty, &component)?;
                    typeComponents(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context, false)?;
                    checkConnectorTypeBalance(node)?;
                }
            }
            ty
        },
        Deref @ Component::COMPONENT { ty: __c_ty, .. } => __c_ty.clone(),
        Deref @ Component::ITERATOR { ty: __c_ty, .. } => __c_ty.clone(),
        Deref @ Component::ENUM_LITERAL { literal: Deref @ Expression::ENUM_LITERAL { ty: __esc_ty, .. }, .. } => {
            ty = (*__esc_ty).clone();
            ty.clone()
        },
        Deref @ Component::INVALID_COMPONENT { .. } => Component::getType(&c)?,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponent")); __mm_s.push_str(&*literal!(" got noninstantiated component ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&component)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub(crate) fn typeComponentTry(
    mut componentNode: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    ErrorExt::setCheckpoint(literal!("NFTyping.typeComponentTry"));
    if '__try0: {
        unwrap_break_err!(typeComponent(componentNode.clone(), context, true), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        comp = NFInstNode::InstNode::component(&componentNode)?;
        comp = metamodelica::Ref::new(Component::NFComponent::INVALID_COMPONENT {
            component: comp.clone(),
            errors: ErrorExt::printCheckpointMessagesStr(false),
        });
        NFInstNode::InstNode::updateComponent(comp.clone(), componentNode.clone())?;
    }
    ErrorExt::delCheckpoint(literal!("NFTyping.typeComponentTry"));
    Ok(())
}

pub(crate) fn checkComponentStreamAttribute(
    mut cty: i32,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    let mut ety: metamodelica::Ref<Type::NFType>;
    if Prefixes::ConnectorType::isFlowOrStream(cty) {
        ety = Type::arrayElementType(ty);
        if !(Type::isReal(&ety)? || Type::isComplex(&ety)) {
            Error::addSourceMessageAndFail(
                &(Error::NON_REAL_FLOW_OR_STREAM.clone()),
                list![
                    Prefixes::ConnectorType::toString(cty),
                    NFInstNode::InstNode::name(component)?
                ],
                &(NFInstNode::InstNode::info(component)),
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok(())
}

pub(crate) fn typeIterator(
    mut iterator: metamodelica::Ref<InstNode::InstNode>,
    mut range: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut structural: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut outRange: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity = Purity::PURE;
    let mut c: metamodelica::Ref<Component::NFComponent> = NFInstNode::InstNode::component(&iterator)?;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut info: SourceInfo;
    (outRange, ty, var) = (match &*c {
        Component::ITERATOR { info: __esc_info, .. } => {
            info = (*__esc_info).clone();
            (exp, ty, var, purity) = typeExp(
                range,
                InstContext::set(context, InstContext::ITERATION_RANGE.clone()),
                metamodelica::AsArg::as_arg(&info),
                false,
            )?;
            if structural
                && var > Variability::PARAMETER.clone()
                && (!(var == Variability::NON_STRUCTURAL_PARAMETER.clone())
                    || Flags::isSet(Flags::NF_SCALARIZE.clone())?)
            {
                Error::addSourceMessageAndFail(
                    &(Error::NON_PARAMETER_ITERATOR_RANGE.clone()),
                    list![Expression::toString(exp.clone())?],
                    metamodelica::AsArg::as_arg(&info),
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if !(Type::isVector(&ty)?) {
                Error::addSourceMessageAndFail(
                    &(Error::FOR_EXPRESSION_TYPE_ERROR.clone()),
                    list![Expression::toString(exp.clone())?, Type::toString(&ty)?],
                    metamodelica::AsArg::as_arg(&info),
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            c = metamodelica::Ref::new(Component::NFComponent::ITERATOR {
                ty: Type::arrayElementType(&ty),
                variability: var,
                info: info.clone(),
            });
            NFInstNode::InstNode::updateComponent(c, iterator)?;
            (exp, ty, var)
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTyping.typeIterator"));
                    __mm_s.push_str(&*literal!(" got non-iterator "));
                    __mm_s.push_str(&*NFInstNode::InstNode::name(&iterator)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((outRange, ty, var, purity))
}

pub(crate) fn typeDimensions(
    mut dimensions: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>,
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>> {
    let mut dimensions: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>> = dimensions;
    for mut i in 1..=metamodelica::arrayLength(dimensions.clone()) {
        typeDimension(
            dimensions.clone(),
            i,
            component.clone(),
            binding.clone(),
            context,
            info.clone(),
        )?;
    }
    Ok(dimensions)
}

pub(crate) fn typeDimension(
    mut dimensions: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>,
    mut index: i32,
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dimension: metamodelica::Ref<Dimension::NFDimension> = ({
        let __elt = (*metamodelica::index_checked(&dimensions.borrow(), index)?).clone();
        __elt
    });
    dimension = (match &*dimension {
        Dimension::UNTYPED { isProcessing: true, .. } => {
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            if InstContext::inFunction(context) {
                dim = crate::NFDimension::interned_UNKNOWN();
                metamodelica::arrayUpdate(dimensions.clone(), index, dim.clone())?;
            } else {
                dim = dimension;
            }
            dim
        }
        Dimension::UNTYPED {
            dimension: __dimension_dimension,
            ..
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            let mut var: Variability;
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut target: metamodelica::Ref<Ceval::EvalTarget::EvalTarget>;
            metamodelica::arrayUpdate(
                dimensions.clone(),
                index,
                metamodelica::Ref::new(Dimension::NFDimension::UNTYPED {
                    dimension: __dimension_dimension.clone(),
                    isProcessing: true,
                }),
            )?;
            (exp, ty, var, _) = typeExp(
                __dimension_dimension.clone(),
                InstContext::set(context, InstContext::DIMENSION.clone()),
                &info,
                false,
            )?;
            TypeCheck::checkDimensionType(exp.clone(), &ty, &info)?;
            if !(InstContext::inFunction(context)) {
                if var <= Variability::PARAMETER.clone() {
                    if InstContext::inRelaxed(context) {
                        exp = Ceval::tryEvalExp(exp, &(Ceval::noTarget().clone()));
                    } else {
                        target = Ceval::EvalTarget::new(
                            info,
                            context,
                            Some(metamodelica::Ref::new(Ceval::EvalTargetData {
                                component: component.clone(),
                                index: index,
                                exp: exp.clone(),
                            })),
                        );
                        exp = Ceval::tryEvalExpResizable(exp, &target)?;
                    }
                } else if !(var == Variability::NON_STRUCTURAL_PARAMETER.clone()) {
                    Error::addSourceMessage(
                        &(Error::DIMENSION_NOT_KNOWN.clone()),
                        list![Expression::toString(exp.clone())?],
                        &info,
                    )?;
                    return Err("fail");
                }
            } else {
                if var <= Variability::STRUCTURAL_PARAMETER.clone()
                    && !(Expression::contains(exp.clone(), &move |__a0: metamodelica::Ref<
                        Expression::NFExpression,
                    >| {
                        Expression::isFunctionInputCref(&__a0)
                    })?)
                {
                    exp = Ceval::tryEvalExp(exp, &(Ceval::noTarget().clone()));
                }
            }
            exp = subscriptDimExp(exp, component.clone())?;
            dim = Dimension::fromExp(exp, var)?;
            metamodelica::arrayUpdate(dimensions.clone(), index, dim.clone())?;
            dim
        }
        Dimension::UNKNOWN
            if (InstContext::inFunction(context)
                && (Binding::isUnbound(&binding) && NFInstNode::InstNode::isOutput(&component)
                    || !(NFInstNode::InstNode::isOutput(&component)))) =>
        {
            dimension
        }
        Dimension::UNKNOWN
            if (InstContext::inFunction(context)
                && Binding::hasExp(&binding)
                && Expression::contains(
                    Binding::getExp(&binding)?,
                    &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(Expression::isCref(&__a0))
                    },
                )?) =>
        {
            dimension
        }
        Dimension::UNKNOWN => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            let mut b: metamodelica::Ref<Binding::NFBinding>;
            let mut ty_err: metamodelica::Ref<TypingError::TypingError>;
            let mut parent_dims: i32;
            let mut target: metamodelica::Ref<Ceval::EvalTarget::EvalTarget>;
            b = binding.clone();
            parent_dims = 0;
            metamodelica::arrayUpdate(
                dimensions.clone(),
                index,
                metamodelica::Ref::new(Dimension::NFDimension::UNTYPED {
                    dimension: WHOLEDIM_CREF().clone(),
                    isProcessing: true,
                }),
            )?;
            if Binding::isUnbound(&binding) {
                (b, parent_dims) = getRecordElementBinding(component.clone(), context)?;
                if Binding::isUnbound(&b) {
                    parent_dims = 0;
                    b = Class::lookupAttributeBinding(
                        literal!("start"),
                        NFInstNode::InstNode::getClass(component.clone())?,
                    );
                    b = Binding::mapExp(
                        b,
                        (std::sync::Arc::new({
                            let __pe_b1 = component.clone();
                            move |__pe_a0| Expression::filterSplitIndices(__pe_a0, &__pe_b1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                    )?;
                }
            }
            (dim, ty_err) = (match &*b {
                Binding::UNBOUND if (!(InstContext::inRelaxed(context))) => {
                    Error::addSourceMessage(
                        &(Error::FAILURE_TO_DEDUCE_DIMS_NO_MOD.clone()),
                        list![
                            ArcStr::from(::std::format!("{}", index)),
                            NFInstNode::InstNode::name(&component)?
                        ],
                        &info,
                    )?;
                    return Err("fail");
                }
                Binding::UNTYPED_BINDING {
                    bindingExp: __b_bindingExp,
                    ..
                } => deduceDimensionFromExp(
                    __b_bindingExp.clone(),
                    None,
                    index,
                    parent_dims,
                    component.clone(),
                    context,
                    info.clone(),
                )?,
                Binding::TYPED_BINDING {
                    bindingExp: __b_bindingExp,
                    bindingType: __b_bindingType,
                    ..
                } => deduceDimensionFromExp(
                    __b_bindingExp.clone(),
                    Some(__b_bindingType.clone()),
                    index,
                    parent_dims,
                    component.clone(),
                    context,
                    info.clone(),
                )?,
                _ => (dimension, crate::NFTyping::TypingError::interned_NO_ERROR()),
            });
            let () = (match &*ty_err {
                TypingError::OUT_OF_BOUNDS { .. } if (!(InstContext::inRelaxed(context))) => {
                    Error::addSourceMessage(
                        &(Error::DIMENSION_DEDUCTION_FROM_BINDING_FAILURE.clone()),
                        list![
                            ArcStr::from(::std::format!("{}", index)),
                            NFInstNode::InstNode::name(&component)?,
                            Binding::toString(&b, &(literal!("")))?
                        ],
                        &info,
                    )?;
                    return Err("fail");
                }
                _ => (),
            });
            dim = (match &*dim {
                Dimension::EXP {
                    exp: __esc_exp,
                    var: __dim_var,
                } => {
                    exp = (*__esc_exp).clone();
                    Structural::markExp(metamodelica::AsArg::as_arg(&exp))?;
                    if InstContext::inRelaxed(context) {
                        exp = Ceval::tryEvalExp(exp.clone(), &(Ceval::noTarget().clone()));
                    } else {
                        target = Ceval::EvalTarget::new(
                            info,
                            context,
                            Some(metamodelica::Ref::new(Ceval::EvalTargetData {
                                component: component.clone(),
                                index: index,
                                exp: exp.clone(),
                            })),
                        );
                        exp = Ceval::evalExp(exp.clone(), &target)?;
                    }
                    exp = subscriptDimExp(exp.clone(), component.clone())?;
                    Dimension::fromExp(exp.clone(), __dim_var.clone())?
                }
                Dimension::UNKNOWN if (!(InstContext::inRelaxed(context))) => {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFTyping.typeDimension"));
                            __mm_s.push_str(&*literal!(" returned unknown dimension in a non-function context"));
                            ArcStr::from(__mm_s)
                        },
                        info,
                    )?;
                    return Err("fail");
                }
                _ => dim,
            });
            metamodelica::arrayUpdate(dimensions.clone(), index, dim.clone())?;
            dim
        }
        _ => dimension,
    });
    Ok(dimension)
}

pub(crate) fn deduceDimensionFromExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: Option<metamodelica::Ref<Type::NFType>>,
    mut index: i32,
    mut parentDims: i32,
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Dimension::NFDimension>,
    metamodelica::Ref<TypingError::TypingError>,
)> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut error: metamodelica::Ref<TypingError::TypingError>;
    let mut oe: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut dim_index: i32;
    dim_index = index + parentDims;
    if (ty).is_some() && !(Type::isConditionalArray(&(Util::getOption(ty.clone())?))) {
        (dim, error) = nthDimensionBoundsChecked(Util::getOption(ty)?, dim_index, 0);
        if Dimension::isUnknown(&dim) {
            (dim, oe, error) = typeExpDim(
                exp.clone(),
                dim_index,
                InstContext::set(context, InstContext::DIMENSION.clone()),
                info.clone(),
            )?;
        } else {
            oe = None;
        }
    } else {
        (dim, oe, error) = typeExpDim(
            exp.clone(),
            dim_index,
            InstContext::set(context, InstContext::DIMENSION.clone()),
            info.clone(),
        )?;
    }
    if Dimension::isUnknown(&dim) && !(TypingError::isError(&error)) {
        e = if ((oe).is_some()) { Util::getOption(oe)? } else { exp };
        if InstContext::inRelaxed(context) {
            e = Ceval::tryEvalExp(e, &(Ceval::noTarget().clone()));
        } else {
            e = Ceval::evalExp(
                e.clone(),
                &(Ceval::EvalTarget::new(
                    info,
                    context,
                    Some(metamodelica::Ref::new(Ceval::EvalTargetData {
                        component: component,
                        index: index,
                        exp: e,
                    })),
                )),
            )?;
        }
        (dim, error) = nthDimensionBoundsChecked(Expression::typeOf(e), dim_index, 0);
    }
    Ok((dim, error))
}

pub(crate) fn subscriptDimExp(
    mut dimExp: metamodelica::Ref<Expression::NFExpression>,
    mut component: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut dimExp: metamodelica::Ref<Expression::NFExpression> = dimExp;
    let mut exp_dims: i32;
    let mut parent_dims: i32;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    exp_dims = Expression::dimensionCount(dimExp.clone(), true)?;
    if exp_dims == 0 {
        return Ok(dimExp);
    }
    subs = metamodelica::nil();
    parent = NFInstNode::InstNode::instanceParent(component)?;
    while exp_dims > 0 && !(NFInstNode::InstNode::isEmpty(&parent)) {
        parent_dims = NFInstNode::InstNode::dimensionCount(&parent);
        for mut i in ({
            let __s = parent_dims;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            subs = metamodelica::cons(Subscript::makeSplitIndex(parent.clone(), i)?, subs);
            exp_dims = exp_dims - 1;
            if exp_dims == 0 {
                break;
            }
        }
        parent = NFInstNode::InstNode::instanceParent(parent)?;
    }
    dimExp = Expression::applySubscripts(&subs, dimExp, false)?;
    Ok(dimExp)
}

pub(crate) fn simplifyDimExp(
    mut dimExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut dimExp: metamodelica::Ref<Expression::NFExpression> = dimExp;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    dimExp = (match &*dimExp {
        Expression::ARRAY { .. } if (Expression::arrayAllEqual(dimExp.clone())) => {
            Expression::arrayFirstScalar(dimExp.clone())?
        }
        Expression::SUBSCRIPTED_EXP {
            split: true,
            exp: __dimExp_exp,
            ..
        } if (Expression::isArray(metamodelica::AsArg::as_arg(&__dimExp_exp))
            && Expression::arrayAllEqual(__dimExp_exp.clone())) =>
        {
            Expression::arrayFirstScalar(__dimExp_exp.clone())?
        }
        _ => dimExp.clone(),
    });
    Ok(dimExp)
}

pub(crate) fn makeDimension(
    mut dimExp: metamodelica::Ref<Expression::NFExpression>,
    mut unevaledExp: &metamodelica::Ref<Expression::NFExpression>,
    mut variability: Variability,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut outDimension: metamodelica::Ref<Dimension::NFDimension>;
    let mut exp: metamodelica::Ref<Expression::NFExpression> = dimExp;
    if Expression::isArray(&exp) {
        if Expression::arrayAllEqual(exp.clone()) {
            exp = Expression::arrayFirstScalar(exp)?;
        }
    }
    outDimension = Dimension::fromExp(exp, variability)?;
    Ok(outDimension)
}

pub(crate) fn getRecordElementBinding(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(metamodelica::Ref<Binding::NFBinding>, i32)> {
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut parentDims: i32 = 0;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut parent_binding: metamodelica::Ref<Binding::NFBinding>;
    parent = NFInstNode::InstNode::instanceParent(component.clone())?;
    if NFInstNode::InstNode::isComponent(&parent)? {
        comp = NFInstNode::InstNode::component(&parent)?;
        parent_binding = Component::getBinding(&comp);
        if Binding::isUnbound(&parent_binding) {
            (binding, parentDims) = getRecordElementBinding(parent, context)?;
        } else {
            binding = typeBinding(
                parent_binding.clone(),
                InstContext::set(context, InstContext::DIMENSION.clone()),
            )?;
            if !(referenceEq(&*(parent_binding), &*(&*binding))) {
                NFInstNode::InstNode::componentApply(parent, &Component::setBinding, binding.clone())?;
            }
        }
        parentDims = parentDims + Component::dimensionCount(&comp);
        if Binding::isBound(&binding) {
            binding = Binding::recordFieldBinding(&component, binding)?;
        }
    } else {
        binding = Binding::EMPTY_BINDING().clone();
    }
    Ok((binding, parentDims))
}

pub fn typeBindings(mut cls: metamodelica::Ref<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut c: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    c = NFInstNode::InstNode::getClass(cls.clone())?;
    let () = (::match_deref::match_deref! { match &(c) {
        Deref @ Class::INSTANCED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentBinding(c, context, true)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentBinding(c, context, true)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::TYPED_DERIVED { baseClass: __c_baseClass, .. } => {
            typeBindings(__c_baseClass.clone(), context)?;
            ()
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeBindings")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&cls)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeComponentBinding(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut typeChildren: bool,
) -> Result<()> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut c: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut name: ArcStr;
    let mut comp_var: Variability;
    let mut attrs: metamodelica::Ref<Attributes::NFAttributes>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    if NFInstNode::InstNode::isEmpty(&component) || NFInstNode::InstNode::isOnlyOuter(&component)? {
        return Ok(());
    }
    node = NFInstNode::InstNode::resolveOuter(component.clone());
    c = NFInstNode::InstNode::component(&node)?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { .. } if (Component::isDeleted(&c)? || Component::isInvalid(&c)) => (),
        Deref @ Component::COMPONENT { binding: Deref @ Binding::UNTYPED_BINDING { .. }, attributes: __esc_attrs, state: __c_state, .. } if (__c_state.clone() == ComponentState::Typed.clone()) => {
            attrs = (*__esc_attrs).clone();
            name = NFInstNode::InstNode::name(&component)?;
            binding = var_field!((*c).binding, Component::NFComponent::COMPONENT).clone();
            ErrorExt::setCheckpoint(literal!("NFTyping.typeComponentBinding"));
            match '__try0: {
                binding = unwrap_break_err!(typeBinding(binding.clone(), InstContext::set(context, InstContext::BINDING.clone())), '__try0);
                if !(InstContext::inAnnotation(context) && stringEq(&name, &(literal!("graphics"))) || NFInstNode::InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT))) {
                    binding = unwrap_break_err!(TypeCheck::matchBinding(binding.clone(), var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), name.clone(), node.clone(), context), '__try0);
                }
                comp_var = unwrap_break_err!(checkComponentBindingVariability(&node, &c, &binding, context), '__try0);
                if comp_var != attrs.variability.clone() {
                    assign_field!(attrs.variability = comp_var);
                    assign_variant_field!(c => Component::NFComponent::COMPONENT; attributes = attrs.clone());
                }
                Ok::<_, &'static str>((binding.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    binding = __try0_o0;
                }
                Err(_) => {
                    if Binding::isBound(var_field!((*c).condition, Component::NFComponent::COMPONENT)) || InstContext::inInstanceAPI(context) {
                        binding = metamodelica::Ref::new(Binding::NFBinding::INVALID_BINDING { binding: binding.clone(), errors: ErrorExt::getCheckpointMessages() });
                    } else {
                        ErrorExt::delCheckpoint(literal!("NFTyping.typeComponentBinding"));
                        return Err("fail");
                    }
                }
            }
            ErrorExt::delCheckpoint(literal!("NFTyping.typeComponentBinding"));
            assign_variant_field!(c => Component::NFComponent::COMPONENT;
                binding = binding,
                state = ComponentState::TypeChecked.clone()
            );
            NFInstNode::InstNode::updateComponent(c.clone(), node)?;
            if typeChildren && !(NFInstNode::InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT))) {
                typeBindings(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context)?;
            }
            ()
        },
        Deref @ Component::COMPONENT { state: __c_state, .. } if (__c_state.clone() >= ComponentState::Typed.clone()) => {
            if var_field!((*c).state, Component::NFComponent::COMPONENT).clone() == ComponentState::Typed.clone() {
                if Binding::isTyped(var_field!((*c).binding, Component::NFComponent::COMPONENT)) {
                    assign_variant_field!(c => Component::NFComponent::COMPONENT; binding = TypeCheck::matchBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), NFInstNode::InstNode::name(&component)?, node.clone(), context)?);
                    checkComponentBindingVariability(&component, &c, var_field!((*c).binding, Component::NFComponent::COMPONENT), context)?;
                }
                assign_variant_field!(c => Component::NFComponent::COMPONENT; state = ComponentState::TypeChecked.clone());
                NFInstNode::InstNode::updateComponent(c.clone(), node)?;
            }
            if typeChildren && !(NFInstNode::InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT))) {
                typeBindings(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context)?;
            }
            ()
        },
        Deref @ Component::COMPONENT { binding: Deref @ Binding::UNTYPED_BINDING { .. }, attributes: __esc_attrs, state: __c_state, .. } if (__c_state.clone() < ComponentState::Typed.clone()) => {
            attrs = (*__esc_attrs).clone();
            binding = typeBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), InstContext::set(context, InstContext::BINDING.clone()))?;
            comp_var = checkComponentBindingVariability(&component, &c, &binding, context)?;
            if comp_var != attrs.variability.clone() {
                assign_field!(attrs.variability = comp_var);
                assign_variant_field!(c => Component::NFComponent::COMPONENT; attributes = attrs.clone());
            }
            assign_variant_field!(c => Component::NFComponent::COMPONENT; binding = binding);
            NFInstNode::InstNode::updateComponent(c.clone(), node)?;
            ()
        },
        Deref @ Component::COMPONENT { .. } => (),
        Deref @ Component::ENUM_LITERAL { .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { modifier: Deref @ Modifier::NOMOD, .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { modifier: __c_modifier, ty: __c_ty } => {
            assign_variant_field!(c => Component::NFComponent::TYPE_ATTRIBUTE; modifier = typeTypeAttribute(__c_modifier.clone(), __c_ty.clone(), &component, context)?);
            NFInstNode::InstNode::updateComponent(c.clone(), node)?;
            ()
        },
        Deref @ Component::INVALID_COMPONENT { .. } => (),
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponentBinding")); __mm_s.push_str(&*literal!(" got invalid node ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&node)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkComponentBindingVariability(
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut component: &metamodelica::Ref<Component::NFComponent>,
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut context: i32,
) -> Result<Variability> {
    let mut var: Variability;
    let mut comp_eff_var: Variability;
    let mut bind_var: Variability;
    let mut bind_eff_var: Variability;
    var = Component::variability(component)?;
    comp_eff_var = Prefixes::effectiveVariability(var);
    bind_var = Binding::variability(binding)?;
    bind_eff_var = Prefixes::effectiveVariability(bind_var);
    if bind_eff_var > comp_eff_var && !(InstContext::inFunction(context)) {
        Error::addSourceMessage(
            &(Error::HIGHER_VARIABILITY_BINDING.clone()),
            list![
                NFInstNode::InstNode::name(node)?,
                Prefixes::variabilityString(comp_eff_var)?,
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("'"));
                    __mm_s.push_str(&*Binding::toString(
                        &(Component::getBinding(component)),
                        &(literal!("")),
                    )?);
                    __mm_s.push_str(&*literal!("'"));
                    ArcStr::from(__mm_s)
                },
                Prefixes::variabilityString(bind_eff_var)?
            ],
            &(Binding::getInfo(binding)),
        )?;
        if !(InstContext::inRelaxed(context)) {
            return Err("fail");
        }
    }
    if var == Variability::PARAMETER.clone() {
        if bind_var <= Variability::STRUCTURAL_PARAMETER.clone()
            && (NFInstNode::InstNode::isInheritedProtected(node)? || Component::isFinal(component)?)
        {
            var = Variability::STRUCTURAL_PARAMETER.clone();
        } else if bind_var == Variability::NON_STRUCTURAL_PARAMETER.clone() {
            var = Variability::NON_STRUCTURAL_PARAMETER.clone();
        }
    }
    Ok(var)
}

pub fn typeBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut context: i32,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    binding = (match &*binding.clone() {
        Binding::UNTYPED_BINDING {
            bindingExp: exp,
            confidence: __binding_confidence,
            eachType: __binding_eachType,
            info: __binding_info,
            source: __binding_source,
            ..
        } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut var: Variability;
            let mut purity: Purity;
            let mut info: SourceInfo;
            let mut exp = (*exp).clone();
            info = Binding::getInfo(&binding);
            (exp, ty, var, purity) = typeExp(exp.clone(), context, &info, false)?;
            metamodelica::Ref::new(Binding::NFBinding::TYPED_BINDING {
                bindingExp: exp.clone(),
                bindingType: ty,
                variability: var,
                purity: purity,
                eachType: __binding_eachType.clone(),
                evalState: Mutable::create(Binding::EvalState::NOT_EVALUATED.clone()),
                isFlattened: false,
                source: __binding_source.clone(),
                confidence: __binding_confidence.clone(),
                info: __binding_info.clone(),
            })
        }
        Binding::TYPED_BINDING { .. } => binding,
        Binding::UNBOUND => binding,
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTyping.typeBinding"));
                    __mm_s.push_str(&*literal!(" got uninstantiated binding"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(binding)
}

pub(crate) fn typeComponentCondition(
    mut condition: metamodelica::Ref<Binding::NFBinding>,
    mut context: i32,
    mut evaluate: bool,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut condition: metamodelica::Ref<Binding::NFBinding> = condition;
    condition = (match &*condition.clone() {
        Binding::UNTYPED_BINDING {
            bindingExp: exp,
            confidence: __condition_confidence,
            source: __condition_source,
            ..
        } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut var: Variability;
            let mut purity: Purity;
            let mut info: SourceInfo;
            let mut mk: MatchKind;
            let mut eval_state: Binding::EvalState;
            let mut next_context: i32;
            let mut exp = (*exp).clone();
            next_context = InstContext::set(context, InstContext::CONDITION.clone());
            info = Binding::getInfo(&condition);
            (exp, ty, var, purity) = typeExp(exp.clone(), next_context, &info, false)?;
            (exp, _, mk) = TypeCheck::matchTypes(
                ty.clone(),
                crate::NFType::interned_BOOLEAN(),
                exp.clone(),
                TypeCheck::DEFAULT_OPTIONS.clone(),
            )?;
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessage(
                    &(Error::IF_CONDITION_TYPE_ERROR.clone()),
                    list![Expression::toString(exp.clone())?, Type::toString(&ty)?],
                    &info,
                )?;
                return Err("fail");
            }
            if var > Variability::PARAMETER.clone() {
                Error::addSourceMessage(
                    &(Error::COMPONENT_CONDITION_VARIABILITY.clone()),
                    list![Expression::toString(exp.clone())?],
                    &info,
                )?;
                return Err("fail");
            }
            eval_state = Binding::EvalState::NOT_EVALUATED.clone();
            if evaluate {
                ErrorExt::setCheckpoint(literal!("NFTyping.typeComponentCondition"));
                if '__try0: {
                    exp = unwrap_break_err!(Ceval::evalExp(exp.clone(), &(Ceval::EvalTarget::new(info.clone(), next_context, None))), '__try0);
                    exp = unwrap_break_err!(simplifyDimExp(exp.clone()), '__try0);
                    eval_state = Binding::EvalState::EVALUATED.clone();
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
                ErrorExt::rollBack(literal!("NFTyping.typeComponentCondition"));
            }
            metamodelica::Ref::new(Binding::NFBinding::TYPED_BINDING {
                bindingExp: exp.clone(),
                bindingType: ty,
                variability: var,
                purity: purity,
                eachType: Binding::EachType::NOT_EACH.clone(),
                evalState: Mutable::create(eval_state),
                isFlattened: false,
                source: __condition_source.clone(),
                confidence: __condition_confidence.clone(),
                info: info,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(condition)
}

pub(crate) fn typeTypeAttribute(
    mut attribute: metamodelica::Ref<Modifier::Modifier>,
    mut attrType: metamodelica::Ref<Type::NFType>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Modifier::Modifier>> {
    let mut attribute: metamodelica::Ref<Modifier::Modifier> = attribute;
    let mut name: ArcStr;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    attribute = (::match_deref::match_deref! { match &(attribute.clone()) {
        Deref @ Modifier::MODIFIER { info: __attribute_info, name: __attribute_name, subModifiers: __attribute_subModifiers, .. } if (!(ModTable::isEmpty(metamodelica::AsArg::as_arg(&__attribute_subModifiers)))) => {
            name = { let mut __mm_s = String::new(); __mm_s.push_str(&*__attribute_name); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Util::tuple21(((ModTable::toList(metamodelica::AsArg::as_arg(&__attribute_subModifiers), metamodelica::nil()))).head().cloned()?)); ArcStr::from(__mm_s) };
            Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![name, Type::toString(&attrType)?], metamodelica::AsArg::as_arg(&__attribute_info))?;
            return Err("fail")
        },
        Deref @ Modifier::MODIFIER { binding: __attribute_binding, .. } if (Binding::isUnbound(metamodelica::AsArg::as_arg(&__attribute_binding))) => crate::NFModifier::Modifier::interned_NOMOD(),
        Deref @ Modifier::MODIFIER { binding: Deref @ Binding::TYPED_BINDING { .. }, .. } => attribute,
        Deref @ Modifier::MODIFIER { name: __esc_name, binding: __esc_binding, .. } => {
            name = (*__esc_name).clone();
            binding = (*__esc_binding).clone();
            if Binding::isBound(metamodelica::AsArg::as_arg(&binding)) {
                binding = typeBinding(binding.clone(), context)?;
                parent = NFInstNode::InstNode::parent(component)?;
                binding = TypeCheck::matchBinding(binding.clone(), attrType, name.clone(), parent, context)?;
                if Binding::variability(metamodelica::AsArg::as_arg(&binding))? >= Variability::DISCRETE.clone() && !(InstContext::inFunction(context)) {
                    Error::addSourceMessage(&(Error::HIGHER_VARIABILITY_BINDING.clone()), list![name.clone(), Prefixes::variabilityString(Variability::PARAMETER.clone())?, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Binding::toString(metamodelica::AsArg::as_arg(&binding), &(literal!("")))?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }, Prefixes::variabilityString(Binding::variability(metamodelica::AsArg::as_arg(&binding))?)?], &(Binding::getInfo(metamodelica::AsArg::as_arg(&binding))))?;
                    return Err("fail");
                }
                assign_variant_field!(attribute => Modifier::Modifier::MODIFIER; binding = binding.clone());
            }
            attribute
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(attribute)
}

pub fn typeExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
    mut retype: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    (exp, ty, variability, purity) = (match &*exp.clone() {
        Expression::INTEGER { .. } => (
            exp,
            crate::NFType::interned_INTEGER(),
            Variability::CONSTANT.clone(),
            Purity::PURE.clone(),
        ),
        Expression::REAL { .. } => (
            exp,
            crate::NFType::interned_REAL(),
            Variability::CONSTANT.clone(),
            Purity::PURE.clone(),
        ),
        Expression::STRING { .. } => (
            exp,
            crate::NFType::interned_STRING(),
            Variability::CONSTANT.clone(),
            Purity::PURE.clone(),
        ),
        Expression::BOOLEAN { .. } => (
            exp,
            crate::NFType::interned_BOOLEAN(),
            Variability::CONSTANT.clone(),
            Purity::PURE.clone(),
        ),
        Expression::ENUM_LITERAL { ty: __exp_ty, .. } => (
            exp,
            __exp_ty.clone(),
            Variability::CONSTANT.clone(),
            Purity::PURE.clone(),
        ),
        Expression::CREF { cref: __exp_cref, .. } => typeCrefExp(__exp_cref.clone(), context, info)?,
        Expression::TYPENAME { ty: __exp_ty } => {
            if !(InstContext::inValidTypenameScope(context)) {
                Error::addSourceMessage(
                    &(Error::INVALID_TYPENAME_USE.clone()),
                    list![Type::typenameString(
                        &(Type::arrayElementType(metamodelica::AsArg::as_arg(&__exp_ty)))
                    )?],
                    info,
                )?;
                return Err("fail");
            }
            (
                exp,
                __exp_ty.clone(),
                Variability::CONSTANT.clone(),
                Purity::PURE.clone(),
            )
        }
        Expression::ARRAY {
            literal: __exp_literal,
            ty: __exp_ty,
            ..
        } => typeArray(
            var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(),
            __exp_literal.clone(),
            metamodelica::AsArg::as_arg(&__exp_ty),
            context,
            info,
        )?,
        Expression::MATRIX {
            elements: __exp_elements,
        } => typeMatrix(metamodelica::AsArg::as_arg(&__exp_elements), context, info)?,
        Expression::RANGE { .. } => typeRange(exp, context, info)?,
        Expression::TUPLE {
            elements: __exp_elements,
            ..
        } => typeTuple(__exp_elements.clone(), context, info)?,
        Expression::SIZE { .. } => typeSize(exp, context, info.clone(), true)?,
        Expression::END => {
            Error::addSourceMessage(&(Error::END_ILLEGAL_USE_ERROR.clone()), metamodelica::nil(), info)?;
            return Err("fail");
        }
        Expression::BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut var1: Variability;
            let mut var2: Variability;
            let mut pur1: Purity;
            let mut pur2: Purity;
            let mut ty1: metamodelica::Ref<Type::NFType>;
            let mut ty2: metamodelica::Ref<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(__exp_exp1.clone(), next_context, info, false)?;
            (e2, ty2, var2, pur2) = typeExp(__exp_exp2.clone(), next_context, info, false)?;
            (exp, ty) = TypeCheck::checkBinaryOperation(
                e1,
                ty1,
                var1,
                __exp_operator.clone(),
                e2,
                ty2,
                var2,
                context,
                info,
                retype,
            )?;
            (
                exp,
                ty,
                Prefixes::variabilityMax(var1, var2),
                Prefixes::purityMin(pur1, pur2),
            )
        }
        Expression::UNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut var1: Variability;
            let mut pur1: Purity;
            let mut ty1: metamodelica::Ref<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(__exp_exp.clone(), next_context, info, false)?;
            (exp, ty) = TypeCheck::checkUnaryOperation(e1, ty1, var1, __exp_operator.clone(), context, info)?;
            (exp, ty, var1, pur1)
        }
        Expression::LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut var1: Variability;
            let mut var2: Variability;
            let mut pur1: Purity;
            let mut pur2: Purity;
            let mut ty1: metamodelica::Ref<Type::NFType>;
            let mut ty2: metamodelica::Ref<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(__exp_exp1.clone(), next_context, info, false)?;
            (e2, ty2, var2, pur2) = typeExp(__exp_exp2.clone(), next_context, info, false)?;
            (exp, ty) = TypeCheck::checkLogicalBinaryOperation(
                e1,
                ty1,
                var1,
                __exp_operator.clone(),
                e2,
                ty2,
                var2,
                context,
                info,
            )?;
            (
                exp,
                ty,
                Prefixes::variabilityMax(var1, var2),
                Prefixes::purityMin(pur1, pur2),
            )
        }
        Expression::LUNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut var1: Variability;
            let mut pur1: Purity;
            let mut ty1: metamodelica::Ref<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(__exp_exp.clone(), next_context, info, false)?;
            (exp, ty) = TypeCheck::checkLogicalUnaryOperation(e1, ty1, var1, __exp_operator.clone(), context, info)?;
            (exp, ty, var1, pur1)
        }
        Expression::RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            index: __exp_index,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut var1: Variability;
            let mut var2: Variability;
            let mut pur1: Purity;
            let mut pur2: Purity;
            let mut ty1: metamodelica::Ref<Type::NFType>;
            let mut ty2: metamodelica::Ref<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(__exp_exp1.clone(), next_context, info, false)?;
            (e2, ty2, var2, pur2) = typeExp(__exp_exp2.clone(), next_context, info, false)?;
            (exp, ty) = TypeCheck::checkRelationOperation(
                e1,
                ty1,
                var1,
                __exp_operator.clone(),
                e2,
                ty2,
                var2,
                __exp_index.clone(),
                context,
                info,
            )?;
            variability = Prefixes::variabilityMax(var1, var2);
            purity = Prefixes::purityMin(pur1, pur2);
            if !(InstContext::inNoEvent(context)) && variability == Variability::CONTINUOUS.clone() {
                variability = Variability::DISCRETE.clone();
            }
            (exp, ty, variability, purity)
        }
        Expression::IF { .. } => typeIfExpression(exp, context, info)?,
        Expression::RECORD { .. } => typeRecordExp(exp, context, info)?,
        Expression::CALL { .. } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut var1: Variability;
            let mut pur1: Purity;
            (e1, ty, var1, pur1) = Call::typeCall(exp, context, info.clone(), retype)?;
            if Type::isTuple(&ty) && !(InstContext::isSingleExpression(context)) {
                ty = Type::firstTupleType(&ty)?;
                e1 = Expression::tupleElement(e1, 1)?;
            }
            (e1, ty, var1, pur1)
        }
        Expression::CAST { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty, variability, purity) = typeExp(__exp_exp.clone(), next_context, info, retype)?;
            assign_variant_field!(exp => Expression::NFExpression::CAST;
                exp = e1,
                ty = Type::copyDims(ty, var_field!((*exp).ty, Expression::NFExpression::CAST).clone())
            );
            (
                exp.clone(),
                var_field!((*exp).ty, Expression::NFExpression::CAST).clone(),
                variability,
                purity,
            )
        }
        Expression::SUBSCRIPTED_EXP { .. } => typeSubscriptedExp(exp, context, info)?,
        Expression::MUTABLE { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = Mutable::access(__exp_exp.clone());
            (e1, ty, variability, purity) = typeExp(e1, context, info, retype)?;
            assign_variant_field!(exp => Expression::NFExpression::MUTABLE; exp = Mutable::create(e1));
            (exp, ty, variability, purity)
        }
        Expression::PARTIAL_FUNCTION_APPLICATION { .. } => Function::typePartialApplication(exp, context, info)?,
        Expression::FILENAME { .. } => (
            exp,
            crate::NFType::interned_STRING(),
            Variability::CONSTANT.clone(),
            Purity::PURE.clone(),
        ),
        Expression::MULTARY { .. } => typeExp(SimplifyExp::splitMultary(exp)?, context, info, retype)?,
        _ => (
            exp.clone(),
            Expression::typeOf(exp.clone()),
            Expression::variability(exp.clone())?,
            Expression::purity(exp)?,
        ),
    });
    if InstContext::inDiscreteScope(context) && variability == Variability::CONTINUOUS.clone() {
        variability = Variability::DISCRETE.clone();
    }
    Ok((exp, ty, variability, purity))
}

pub(crate) fn typeExpl(
    mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::List<metamodelica::Ref<Type::NFType>>,
    metamodelica::List<Variability>,
)> {
    let mut explTyped: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut tyl: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
    let mut varl: metamodelica::List<Variability> = metamodelica::nil();
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut var: Variability;
    let mut ty: metamodelica::Ref<Type::NFType>;
    for mut e in &*expl.reverse() {
        (exp, ty, var, _) = typeExp(e.clone(), context, info, false)?;
        explTyped = metamodelica::cons(exp, explTyped);
        tyl = metamodelica::cons(ty, tyl);
        varl = metamodelica::cons(var, varl);
    }
    Ok((explTyped, tyl, varl))
}

pub(crate) fn typeRecordExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty_elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut next_context: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::RECORD { path: __pa0, ty: __pa1, elements: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    elems = metamodelica::Own::own(__pa2);
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    for mut e in &*elems {
        let mut e = e.clone();
        (e, _, var, pur) = typeExp(e, context, info, false)?;
        variability = Prefixes::variabilityMax(var, variability);
        purity = Prefixes::purityMin(pur, purity);
        ty_elems = metamodelica::cons(e, ty_elems);
    }
    exp = Expression::makeRecord(path, ty.clone(), metamodelica::Dangerous::listReverseInPlace(ty_elems));
    Ok((exp, ty, variability, purity))
}

pub(crate) fn typeSubscriptedExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut expanded_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut fill_dims: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut split: bool;
    let mut subs_var: Variability;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { exp: __pa0, subscripts: __pa1, ty: __pa2, split: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    subs = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    split = metamodelica::Own::own(__pa3);
    if split {
        (expanded_subs, fill_dims) = expandProxySubscripts(&subs, context)?;
        (exp, ty, variability, purity) = typeSubscriptedExp2(&e, &expanded_subs, context, info)?;
        if !((fill_dims).is_empty()) {
            fill_dims = metamodelica::Dangerous::listReverseInPlace(fill_dims);
            ty = Type::liftArrayLeftList(
                ty,
                &({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
                    for mut d in (fill_dims.clone()).into_iter().cloned() {
                        let __x = Dimension::fromExp(d.clone(), Variability::CONSTANT.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            );
            exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
                call: Call::makeTypedCall(
                    NFBuiltinFuncs::FILL_FUNC().clone(),
                    metamodelica::cons(exp, fill_dims),
                    variability,
                    purity,
                    ty.clone(),
                ),
            });
        }
        if !((expanded_subs).is_empty()) {
            ty = Type::subscript(ty, &expanded_subs, false)?;
            if Type::isUnknown(&ty) {
                exp = metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP {
                    exp: exp,
                    subscripts: expanded_subs.clone(),
                    ty: ty.clone(),
                    split: true,
                });
            } else {
                exp = Expression::applySubscripts(&expanded_subs, exp, false)?;
            }
            if purity == Purity::PURE.clone() {
                purity = Subscript::purityList(&expanded_subs)?;
            }
            if variability != Variability::CONTINUOUS.clone() {
                variability = Prefixes::variabilityMax(variability, Subscript::variabilityList(&expanded_subs)?);
            }
        }
    } else {
        (e, ty, variability, purity) = typeExp(e, context, info, false)?;
        (subs, subs_var) = typeSubscripts(subs, ty.clone(), exp, context, info, true)?;
        ty = Type::subscript(ty, &subs, true)?;
        exp = metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP {
            exp: e,
            subscripts: subs,
            ty: ty.clone(),
            split: false,
        });
    }
    Ok((exp, ty, variability, purity))
}

pub(crate) fn expandProxySubscripts(
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut context: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
)> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
    let mut fillDimensions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut dim_count: i32;
    let mut cr_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    for mut s in &**subscripts {
        outSubscripts = (match &*s.clone() {
            Subscript::SPLIT_PROXY {
                origin: __s_origin,
                parent: __s_parent,
            } => {
                dim_count = NFInstNode::InstNode::dimensionCount(&(NFInstNode::InstNode::borrow(__s_parent.clone())?));
                for mut i in 1..=dim_count {
                    outSubscripts = metamodelica::cons(
                        Subscript::makeSplitIndex(NFInstNode::InstNode::borrow(__s_parent.clone())?, i)?,
                        outSubscripts,
                    );
                }
                if !(NFInstNode::InstNode::refEqual(
                    &(NFInstNode::InstNode::borrow(__s_origin.clone())?),
                    &(NFInstNode::InstNode::borrow(__s_parent.clone())?),
                )?) {
                    dim_count = dim_count
                        - NFInstNode::InstNode::dimensionCount(&(NFInstNode::InstNode::borrow(__s_origin.clone())?));
                    if dim_count > 0 {
                        ty = NFInstNode::InstNode::getType(NFInstNode::InstNode::borrow(__s_parent.clone())?)?;
                        cr_exp = Expression::fromCref(
                            ComponentRef::fromNode(
                                NFInstNode::InstNode::borrow(__s_parent.clone())?,
                                ty.clone(),
                                metamodelica::nil(),
                                ComponentRef::Origin::CREF.clone(),
                            )?,
                            false,
                        )?;
                        dims = Type::arrayDims(ty);
                        for mut i in 1..=dim_count {
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims) {
                                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            dim = metamodelica::Own::own(__pa0);
                            dims = metamodelica::Own::own(__pa1);
                            if Dimension::isKnown(&dim, true) {
                                fillDimensions = metamodelica::cons(Dimension::sizeExp(&dim)?, fillDimensions);
                            } else {
                                fillDimensions = metamodelica::cons(
                                    metamodelica::Ref::new(Expression::NFExpression::SIZE {
                                        exp: cr_exp.clone(),
                                        dimIndex: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                                            value: i,
                                        })),
                                    }),
                                    fillDimensions,
                                );
                            }
                        }
                    }
                }
                outSubscripts
            }
            _ => metamodelica::cons(s.clone(), outSubscripts),
        });
    }
    outSubscripts = List::trim(outSubscripts, &move |__a0: metamodelica::Ref<
        Subscript::NFSubscript,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Subscript::isWhole(&__a0))
    })?;
    outSubscripts = metamodelica::Dangerous::listReverseInPlace(outSubscripts);
    Ok((outSubscripts, fillDimensions))
}

pub(crate) fn typeSubscriptedExp2(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut splitSubs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut variability: Variability;
    let mut purity: Purity;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    (outExp, ty, variability, purity) = (match &**exp {
        Expression::ARRAY {
            literal: __exp_literal, ..
        } if (!((splitSubs).is_empty())
            && !(var_field!((**exp).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .is_empty())) =>
        {
            expl = metamodelica::nil();
            variability = Variability::CONSTANT.clone();
            purity = Purity::PURE.clone();
            let __range0 = var_field!((**exp).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut e in __range0 {
                (e, ty, variability, purity) = typeSubscriptedExp2(&e, &((splitSubs).rest()?), context, info)?;
                expl = metamodelica::cons(e, expl);
            }
            expl = metamodelica::Dangerous::listReverseInPlace(expl);
            ty = Type::liftArrayLeft(
                ty,
                &(Dimension::fromInteger(((expl).len() as i32), Prefixes::Variability::CONSTANT.clone())),
            );
            outExp = Expression::makeArray(
                ty.clone(),
                metamodelica::arrayFromVec(expl.into_iter().cloned().collect()),
                __exp_literal.clone(),
            );
            (outExp, ty, variability, purity)
        }
        _ => typeExp(exp.clone(), context, info, false)?,
    });
    Ok((outExp, ty, variability, purity))
}

pub(crate) fn typeExpDim(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut dimIndex: i32,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Dimension::NFDimension>,
    Option<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::Ref<TypingError::TypingError>,
)> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut typedExp: Option<metamodelica::Ref<Expression::NFExpression>> = None;
    let mut error: metamodelica::Ref<TypingError::TypingError>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut next_context: i32;
    ty = Expression::typeOf(exp.clone());
    if Type::isKnown(&ty) {
        (dim, error) = nthDimensionBoundsChecked(ty, dimIndex, 0);
        typedExp = Some(exp.clone());
        if !(Dimension::isUnknown(&dim)) {
            return Ok((dim, typedExp, error));
        }
    }
    next_context = InstContext::clearExpFlags(context);
    (dim, error) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::UNKNOWN, .. } => typeArrayDim(exp, dimIndex)?,
        Deref @ Expression::CREF { cref: __exp_cref, .. } => typeCrefDim(__exp_cref.clone(), dimIndex, next_context, &info)?,
        _ => {
            (e, ty, _, _) = typeExp(exp, next_context, &info, false)?;
            if Type::isTuple(&ty) {
                ty = Type::firstTupleType(&ty)?;
                e = Expression::tupleElement(e, 1)?;
            }
            if Type::isConditionalArray(&ty) {
                e = Expression::map(e, (std::sync::Arc::new({ let __pe_b1 = Ceval::EvalTarget::new(info.clone(), next_context, None); move |__pe_a0| evaluateArrayIf(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                (e, ty, _, _) = typeExp(e, next_context, &info, false)?;
            }
            typedExp = Some(e);
            nthDimensionBoundsChecked(ty, dimIndex, 0)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((dim, typedExp, error))
}

pub(crate) fn evaluateArrayIf(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<Ceval::EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = (match &*exp.clone() {
        Expression::IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ty: __exp_ty,
        } if (Type::isConditionalArray(metamodelica::AsArg::as_arg(&__exp_ty))) => {
            let mut cond: metamodelica::Ref<Expression::NFExpression>;
            cond = Ceval::evalExp(__exp_condition.clone(), target)?;
            if Expression::isTrue(&cond) {
                outExp = __exp_trueBranch.clone();
            } else if Expression::isFalse(&cond) {
                outExp = __exp_falseBranch.clone();
            } else {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFTyping.evaluateArrayIf"));
                        __mm_s.push_str(&*literal!(" failed on "));
                        __mm_s.push_str(&*Expression::toString(exp)?);
                        ArcStr::from(__mm_s)
                    },
                    Ceval::EvalTarget::getInfo(target),
                )?;
                return Err("fail");
            }
            outExp
        }
        _ => exp,
    });
    Ok(outExp)
}

pub(crate) fn typeArrayDim(
    mut arrayExp: metamodelica::Ref<Expression::NFExpression>,
    mut dimIndex: i32,
) -> Result<(
    metamodelica::Ref<Dimension::NFDimension>,
    metamodelica::Ref<TypingError::TypingError>,
)> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut error: metamodelica::Ref<TypingError::TypingError>;
    if dimIndex < 1 {
        dim = crate::NFDimension::interned_UNKNOWN();
        error = metamodelica::Ref::new(TypingError::TypingError::OUT_OF_BOUNDS {
            upperBound: Expression::dimensionCount(arrayExp, true)?,
        });
    } else {
        (dim, error) = typeArrayDim2(arrayExp, dimIndex, 0)?;
    }
    Ok((dim, error))
}

pub(crate) fn typeArrayDim2(
    mut arrayExp: metamodelica::Ref<Expression::NFExpression>,
    mut dimIndex: i32,
    mut dimCount: i32,
) -> Result<(
    metamodelica::Ref<Dimension::NFDimension>,
    metamodelica::Ref<TypingError::TypingError>,
)> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut error: metamodelica::Ref<TypingError::TypingError>;
    (dim, error) = (::match_deref::match_deref! { match &((arrayExp.clone(), dimIndex)) {
        (Deref @ Expression::ARRAY { .. }, 1) => (Dimension::fromExpArray(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone()), crate::NFTyping::TypingError::interned_NO_ERROR()),
        (Deref @ Expression::ARRAY { .. }, _) => typeArrayDim2(metamodelica::arrayGet(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), 1)?, dimIndex - 1, dimCount + 1)?,
        _ => {
            dim = crate::NFDimension::interned_UNKNOWN();
            error = metamodelica::Ref::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dimCount });
            (dim, error)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((dim, error))
}

pub(crate) fn typeCrefDim(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut dimIndex: i32,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Dimension::NFDimension>,
    metamodelica::Ref<TypingError::TypingError>,
)> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut error: metamodelica::Ref<TypingError::TypingError> = crate::NFTyping::TypingError::interned_NO_ERROR();
    let mut crl: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut index: i32;
    let mut dim_count: i32;
    let mut dim_total: i32 = 0;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut c: metamodelica::Ref<Component::NFComponent>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut dims: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>;
    if ComponentRef::hasSubscripts(&cref)? {
        (_, ty, _, _) = typeCref(cref, context, info)?;
        (dim, error) = nthDimensionBoundsChecked(ty, dimIndex, 0);
        return Ok((dim, error));
    }
    crl = ComponentRef::toListReverse(&cref, false, metamodelica::nil());
    index = dimIndex;
    for mut cr in &*crl {
        let () = (match &*cr.clone() {
            ComponentRef::CREF { subscripts: _, .. }
                if (NFInstNode::InstNode::isComponent(&(ComponentRef::node(metamodelica::AsArg::as_arg(&cr))?))?) =>
            {
                node = NFInstNode::InstNode::resolveOuter(ComponentRef::node(metamodelica::AsArg::as_arg(&cr))?);
                c = NFInstNode::InstNode::component(&node)?;
                if Class::hasDimensions(&(NFInstNode::InstNode::getClass(Component::classInstance(&c)?)?))? {
                    typeComponent(node.clone(), context, true)?;
                    c = NFInstNode::InstNode::component(&node)?;
                }
                dim_count = (::match_deref::match_deref! { match &(c) {
                    Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: __esc_dims, .. }, binding: __c_binding, info: __c_info, .. } => {
                        dims = (*__esc_dims).clone();
                        dim_count = metamodelica::arrayLength(dims.clone());
                        if index <= dim_count && index > 0 {
                            dim = typeDimension(dims.clone(), index, node.clone(), __c_binding.clone(), context, __c_info.clone())?;
                            checkCyclicDimension(&dim, &node, index, metamodelica::AsArg::as_arg(&__c_info))?;
                            return Ok((dim, error));
                        }
                        dim_count
                    },
                    Deref @ Component::COMPONENT { ty: __c_ty, .. } => {
                        dim_count = Type::dimensionCount(__c_ty.clone());
                        if index <= dim_count && index > 0 {
                            dim = Type::nthDimension(__c_ty.clone(), index)?;
                            return Ok((dim, error));
                        }
                        dim_count
                    },
                    _ => 0,
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                index = index - dim_count;
                dim_total = dim_total + dim_count;
                ()
            }
            _ => (),
        });
    }
    dim = crate::NFDimension::interned_UNKNOWN();
    error = metamodelica::Ref::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dim_total });
    Ok((dim, error))
}

pub(crate) fn checkCyclicDimension(
    mut dim: &metamodelica::Ref<Dimension::NFDimension>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
    mut index: i32,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (match &**dim {
        Dimension::UNTYPED {
            isProcessing: true,
            dimension: __dim_dimension,
        } => {
            Error::addSourceMessage(
                &(Error::CYCLIC_DIMENSIONS.clone()),
                list![
                    ArcStr::from(::std::format!("{}", index)),
                    NFInstNode::InstNode::name(component)?,
                    Expression::toString(__dim_dimension.clone())?
                ],
                info,
            )?;
            return Err("fail");
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn nthDimensionBoundsChecked(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut dimIndex: i32,
    mut offset: i32,
) -> (
    metamodelica::Ref<Dimension::NFDimension>,
    metamodelica::Ref<TypingError::TypingError>,
) {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut error: metamodelica::Ref<TypingError::TypingError>;
    let mut dim_size: i32 = Type::dimensionCount(ty.clone());
    let mut index: i32 = dimIndex + offset;
    if index < 1 || index > dim_size {
        dim = crate::NFDimension::interned_UNKNOWN();
        error = metamodelica::Ref::new(TypingError::TypingError::OUT_OF_BOUNDS {
            upperBound: dim_size - offset,
        });
    } else {
        match '__try0: {
            dim = unwrap_break_err!(Type::nthDimension(ty.clone(), index), '__try0);
            error = crate::NFTyping::TypingError::interned_NO_ERROR();
            Ok::<_, &'static str>((dim.clone(), error.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                dim = __try0_o0;
                error = __try0_o1;
            }
            Err(_) => {
                dim = crate::NFDimension::interned_UNKNOWN();
                error = crate::NFTyping::TypingError::interned_UNKNOWN_TYPE();
            }
        }
    }
    (dim, error)
}

pub(crate) fn typeCrefExp(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node_var: Variability;
    let mut subs_var: Variability;
    (cr, ty, node_var, subs_var) = typeCref(cref.clone(), context, info)?;
    exp = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: ty.clone(),
        cref: cr,
    });
    variability = Prefixes::variabilityMax(node_var, subs_var);
    purity = ComponentRef::purity(&cref)?;
    Ok((exp, ty, variability, purity))
}

pub(crate) fn typeCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Variability,
)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut nodeVariability: Variability;
    let mut subsVariability: Variability;
    if InstContext::inFunction(context) && ComponentRef::isTime(&cref)? {
        Error::addSourceMessage(&(Error::EXP_INVALID_IN_FUNCTION.clone()), list![literal!("time")], info)?;
        return Err("fail");
    }
    (cref, subsVariability) = typeCref2(cref, context, info, true)?;
    if ComponentRef::hasImplicitTrailingIndex(&cref) {
        cref = ComponentRef::fillSubscripts(cref);
    }
    ty = ComponentRef::getSubscriptedType(&cref, false)?;
    nodeVariability = ComponentRef::nodeVariability(&cref)?;
    Ok((cref, ty, nodeVariability, subsVariability))
}

pub(crate) fn typeCref2(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut context: i32,
    mut info: &SourceInfo,
    mut firstPart: bool,
) -> Result<(metamodelica::Ref<ComponentRef::NFComponentRef>, Variability)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut subsVariability: Variability;
    let mut cr_node: metamodelica::Ref<InstNode::InstNode>;
    (cref, subsVariability) = (match &*cref {
        ComponentRef::CREF {
            origin: ComponentRef::Origin::SCOPE,
            ..
        } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF;
                ty = NFInstNode::InstNode::getType(ComponentRef::node(&cref)?)?,
                restCref = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context, info, false)?.0
            );
            (cref.clone(), Variability::CONSTANT.clone())
        }
        ComponentRef::CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } if (NFInstNode::InstNode::isComponent(&(ComponentRef::node(&cref)?))?) => {
            let mut rest_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut node_ty: metamodelica::Ref<Type::NFType>;
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let mut subs_var: Variability;
            let mut rest_var: Variability;
            cr_node = ComponentRef::node(&cref)?;
            node_ty = typeComponent(
                cr_node.clone(),
                InstContext::nodeContext(&cr_node, context)?,
                firstPart || !(InstContext::inDimension(context)),
            )?;
            (subs, subs_var) = typeSubscripts(
                __cref_subscripts.clone(),
                node_ty.clone(),
                metamodelica::Ref::new(Expression::NFExpression::CREF {
                    ty: node_ty.clone(),
                    cref: cref.clone(),
                }),
                context,
                info,
                true,
            )?;
            (rest_cr, rest_var) = typeCref2(__cref_restCref.clone(), context, info, false)?;
            subsVariability = Prefixes::variabilityMax(subs_var, rest_var);
            (
                metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF {
                    node: __cref_node.clone(),
                    subscripts: subs,
                    ty: node_ty,
                    origin: __cref_origin.clone(),
                    restCref: rest_cr,
                }),
                subsVariability,
            )
        }
        ComponentRef::CREF { .. }
            if (NFInstNode::InstNode::isClass(&(ComponentRef::node(&cref)?))?
                && firstPart
                && NFInstNode::InstNode::isFunction(ComponentRef::node(&cref)?)?) =>
        {
            let mut r#fn: metamodelica::Ref<Function::Function>;
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(ComponentRef::node(&cref)?, InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r#fn = metamodelica::Own::own(__pa0);
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF;
                ty = metamodelica::Ref::new(Type::NFType::FUNCTION { r#fn: r#fn, fnType: Type::FunctionType::FUNCTION_REFERENCE.clone() }),
                restCref = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context, info, false)?.0
            );
            (cref.clone(), Variability::CONSTANT.clone())
        }
        ComponentRef::CREF { .. } if (NFInstNode::InstNode::isClass(&(ComponentRef::node(&cref)?))?) => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; ty = NFInstNode::InstNode::getType(ComponentRef::node(&cref)?)?);
            (cref.clone(), Variability::CONSTANT.clone())
        }
        ComponentRef::CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
            ..
        } if (NFInstNode::InstNode::isName(&(ComponentRef::node(&cref)?))) => {
            let mut rest_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut subs_var: Variability;
            let mut rest_var: Variability;
            (_, subs_var) = typeSubscripts(
                __cref_subscripts.clone(),
                __cref_ty.clone(),
                metamodelica::Ref::new(Expression::NFExpression::CREF {
                    ty: __cref_ty.clone(),
                    cref: cref.clone(),
                }),
                context,
                info,
                false,
            )?;
            (rest_cr, rest_var) = typeCref2(__cref_restCref.clone(), context, info, false)?;
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = rest_cr);
            subsVariability = Prefixes::variabilityMax(subs_var, rest_var);
            (cref.clone(), rest_var)
        }
        _ => (cref.clone(), Variability::CONSTANT.clone()),
    });
    Ok((cref, subsVariability))
}

pub(crate) fn typeSubscripts(
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut crefType: metamodelica::Ref<Type::NFType>,
    mut subscriptedExp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
    mut checkSubscripts: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    Variability,
)> {
    let mut typedSubs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut next_context: i32;
    let mut i: i32;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    let mut var: Variability;
    if (subscripts).is_empty() {
        typedSubs = subscripts;
        return Ok((typedSubs, variability));
    }
    dims = Type::arrayDims(crefType);
    typedSubs = metamodelica::nil();
    next_context = InstContext::set(context, InstContext::SUBSCRIPT.clone());
    i = 1;
    if ((subscripts).len() as i32) > ((dims).len() as i32) && checkSubscripts {
        Error::addSourceMessage(
            &(Error::WRONG_NUMBER_OF_SUBSCRIPTS.clone()),
            list![
                Expression::toString(subscriptedExp.clone())?,
                ArcStr::from(::std::format!("{}", ((subscripts).len() as i32))),
                ArcStr::from(::std::format!("{}", ((dims).len() as i32)))
            ],
            info,
        )?;
        return Err("fail");
    }
    for mut s in &*subscripts {
        if checkSubscripts {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            dim = metamodelica::Own::own(__pa0);
            dims = metamodelica::Own::own(__pa1);
        } else {
            dim = crate::NFDimension::interned_UNKNOWN();
        }
        (sub, var) = typeSubscript(s.clone(), &dim, &subscriptedExp, i, next_context, info, checkSubscripts)?;
        typedSubs = metamodelica::cons(sub.clone(), typedSubs);
        variability = Prefixes::variabilityMax(variability, var);
        i = i + 1;
        if var == Variability::PARAMETER.clone() {
            Structural::markSubscript(&sub)?;
        }
    }
    typedSubs = metamodelica::Dangerous::listReverseInPlace(typedSubs);
    Ok((typedSubs, variability))
}

pub(crate) fn typeSubscript(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
    mut subscriptedExp: &metamodelica::Ref<Expression::NFExpression>,
    mut index: i32,
    mut context: i32,
    mut info: &SourceInfo,
    mut checkSubscript: bool,
) -> Result<(metamodelica::Ref<Subscript::NFSubscript>, Variability)> {
    let mut outSubscript: metamodelica::Ref<Subscript::NFSubscript> = subscript.clone();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut e: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::NFExpression::EMPTY {
        ty: crate::NFType::interned_UNKNOWN(),
    });
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut matched_ty: metamodelica::Ref<Type::NFType>;
    (ty, variability) = (match &*subscript {
        Subscript::UNTYPED { exp: __subscript_exp } => {
            e = evaluateEnd(__subscript_exp.clone(), dimension, subscriptedExp, index, context, info)?;
            (e, ty, variability, _) = typeExp(e, context, info, false)?;
            if Type::isArray(&ty) && InstContext::inEquation(context) {
                Structural::markExp(&e)?;
                e = Ceval::tryEvalExp(e, &(Ceval::noTarget().clone()));
                ty = Expression::typeOf(e.clone());
            }
            if checkSubscript {
                (e, matched_ty) = checkSubscriptType(e, Type::arrayElementType(&ty), dimension, info)?;
            } else {
                matched_ty = ty.clone();
            }
            outSubscript = if (Type::isArray(&ty)) {
                metamodelica::Ref::new(Subscript::NFSubscript::SLICE { slice: e })
            } else {
                metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: e })
            };
            (matched_ty, variability)
        }
        Subscript::INDEX { index: __esc_e } => {
            e = (*__esc_e).clone();
            if checkSubscript {
                (e, ty) = checkSubscriptType(e.clone(), Expression::typeOf(e.clone()), dimension, info)?;
            } else {
                ty = Expression::typeOf(e.clone());
            }
            outSubscript = metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: e.clone() });
            (ty, Expression::variability(e.clone())?)
        }
        Subscript::SLICE { slice: __esc_e } => {
            e = (*__esc_e).clone();
            if checkSubscript {
                (e, ty) = checkSubscriptType(
                    e.clone(),
                    Type::unliftArray(Expression::typeOf(e.clone()))?,
                    dimension,
                    info,
                )?;
            } else {
                ty = Type::unliftArray(Expression::typeOf(e.clone()))?;
            }
            outSubscript = metamodelica::Ref::new(Subscript::NFSubscript::SLICE { slice: e.clone() });
            (ty, Expression::variability(e.clone())?)
        }
        Subscript::WHOLE => (crate::NFType::interned_UNKNOWN(), Dimension::variability(dimension)?),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTyping.typeSubscript"));
                    __mm_s.push_str(&*literal!(" got unknown subscript"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((outSubscript, variability))
}

pub(crate) fn checkSubscriptType(
    mut subscriptExp: metamodelica::Ref<Expression::NFExpression>,
    mut subscriptType: metamodelica::Ref<Type::NFType>,
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut subscriptExp: metamodelica::Ref<Expression::NFExpression> = subscriptExp;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut expected_ty: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    expected_ty = Dimension::subscriptType(dimension);
    (subscriptExp, outType, mk) = TypeCheck::matchTypes(
        subscriptType.clone(),
        expected_ty.clone(),
        subscriptExp,
        TypeCheck::ALLOW_UNKNOWN.clone(),
    )?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(
            &(Error::SUBSCRIPT_TYPE_MISMATCH.clone()),
            list![
                Expression::toString(subscriptExp.clone())?,
                Type::toString(&subscriptType)?,
                Type::toString(&expected_ty)?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok((subscriptExp, outType))
}

pub(crate) fn typeArray(
    mut elements: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
    mut isLiteral: bool,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut arrayExp: metamodelica::Ref<Expression::NFExpression>;
    let mut arrayType: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut expl2: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut ty1: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut ty3: metamodelica::Ref<Type::NFType>;
    let mut tys: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
    let mut mk: MatchKind;
    let mut array_len: i32;
    let mut idx: i32;
    let mut next_context: i32;
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    array_len = metamodelica::arrayLength(elements.clone());
    if array_len > 0 {
        (exp, ty1, variability, purity) =
            typeExp(metamodelica::arrayGet(elements.clone(), 1)?, next_context, info, false)?;
        expl = metamodelica::cons(exp, expl);
        tys = metamodelica::cons(ty1.clone(), tys);
        for mut i in 2..=array_len {
            (exp, ty2, var, pur) = typeExp(metamodelica::arrayGet(elements.clone(), i)?, next_context, info, false)?;
            variability = Prefixes::variabilityMax(var, variability);
            purity = Prefixes::purityMin(pur, purity);
            (_, ty3, mk) = TypeCheck::matchTypes(
                ty2.clone(),
                ty1.clone(),
                exp.clone(),
                TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone(),
            )?;
            if TypeCheck::isIncompatibleMatch(mk) {
                (_, ty3, mk) = TypeCheck::matchTypes(
                    ty1.clone(),
                    ty2.clone(),
                    exp.clone(),
                    TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone(),
                )?;
                if TypeCheck::isCompatibleMatch(mk) {
                    ty1 = ty3;
                }
            } else {
                ty1 = ty3;
            }
            expl = metamodelica::cons(exp, expl);
            tys = metamodelica::cons(ty2, tys);
        }
    } else {
        ty1 = Type::arrayElementType(ty);
    }
    idx = array_len;
    for mut e in &*expl {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty2 = metamodelica::Own::own(__pa0);
        tys = metamodelica::Own::own(__pa1);
        (exp, _, mk) = TypeCheck::matchTypes(
            ty2.clone(),
            ty1.clone(),
            e.clone(),
            TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone(),
        )?;
        expl2 = metamodelica::cons(exp.clone(), expl2);
        if !(InstContext::inAnnotation(context)) {
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessage(
                    &(Error::NF_ARRAY_TYPE_MISMATCH.clone()),
                    list![
                        ArcStr::from(::std::format!("{}", idx)),
                        Expression::toString(exp)?,
                        Type::toString(&ty2)?,
                        Type::toString(&ty1)?
                    ],
                    info,
                )?;
                return Err("fail");
            }
        }
        idx = idx - 1;
    }
    arrayType = Type::liftArrayLeft(ty1, &(Dimension::fromExpList(&expl2)));
    arrayExp = Expression::makeArray(
        arrayType.clone(),
        metamodelica::arrayFromVec(expl2.into_iter().cloned().collect()),
        isLiteral,
    );
    Ok((arrayExp, arrayType, variability, purity))
}

pub(crate) fn typeMatrix(
    mut elements: &metamodelica::List<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut arrayExp: metamodelica::Ref<Expression::NFExpression>;
    let mut arrayType: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut res: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut ty: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut tys: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
    let mut resTys: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
    let mut n: i32 = 2;
    let mut next_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    if ((elements).len() as i32) > 1 {
        for mut el in &**elements {
            (exp, ty, var, pur) = typeMatrixComma(metamodelica::AsArg::as_arg(&el), next_context, info)?;
            variability = Prefixes::variabilityMax(var, variability);
            purity = Prefixes::purityMin(pur, purity);
            expl = metamodelica::cons(exp, expl);
            tys = metamodelica::cons(ty.clone(), tys);
            n = std::cmp::max(n, Type::dimensionCount(ty));
        }
        for mut e in &*expl {
            let mut e = e.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            tys = metamodelica::Own::own(__pa1);
            (e, ty) = Expression::promote(e, ty, n)?;
            resTys = metamodelica::cons(ty, resTys);
            res = metamodelica::cons(e, res);
        }
        (arrayExp, arrayType) = BuiltinCall::makeCatExp(1, &res, resTys, variability, purity, info)?;
    } else {
        (arrayExp, arrayType, variability, purity) =
            typeMatrixComma(&((elements).head().cloned()?), next_context, info)?;
        if Type::dimensionCount(arrayType.clone()) < 2 {
            (arrayExp, arrayType) = Expression::promote(arrayExp, arrayType, n)?;
        }
    }
    Ok((arrayExp, arrayType, variability, purity))
}

pub(crate) fn typeMatrixComma(
    mut elements: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut arrayExp: metamodelica::Ref<Expression::NFExpression>;
    let mut arrayType: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut res: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut ty: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut ty3: metamodelica::Ref<Type::NFType>;
    let mut tys: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
    let mut tys2: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut n: i32 = 2;
    let mut pos: i32;
    let mut mk: MatchKind;
    Error::assertion(
        !((elements).is_empty()),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NFTyping.typeMatrixComma"));
            __mm_s.push_str(&*literal!(" expected non-empty arguments"));
            ArcStr::from(__mm_s)
        },
        &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")),
    )?;
    if ((elements).len() as i32) > 1 {
        for mut e in &**elements {
            (exp, ty1, var, pur) = typeExp(e.clone(), context, info, false)?;
            expl = metamodelica::cons(exp, expl);
            if Type::isEqual(&ty, &(crate::NFType::interned_UNKNOWN()))? {
                ty = ty1.clone();
            } else {
                (_, _, ty2, mk) = TypeCheck::matchExpressions(
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                    Type::arrayElementType(&ty1),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                    Type::arrayElementType(&ty),
                    TypeCheck::DEFAULT_OPTIONS.clone(),
                )?;
                if TypeCheck::isCompatibleMatch(mk) {
                    ty = ty2;
                }
            }
            tys = metamodelica::cons(ty1, tys);
            variability = Prefixes::variabilityMax(variability, var);
            purity = Prefixes::purityMin(purity, pur);
            n = std::cmp::max(n, Type::dimensionCount(ty.clone()));
        }
        tys2 = metamodelica::nil();
        res = metamodelica::nil();
        pos = n + 1;
        for mut e in &*expl {
            let mut e = e.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty1 = metamodelica::Own::own(__pa0);
            tys = metamodelica::Own::own(__pa1);
            pos = pos - 1;
            if Type::dimensionCount(ty1.clone()) != n {
                (e, ty1) = Expression::promote(e, ty1, n)?;
            }
            ty2 = Type::setArrayElementType(&ty1, &ty);
            (e, ty3, mk) = TypeCheck::matchTypes(ty1.clone(), ty2.clone(), e, TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessageAndFail(
                    &(Error::ARG_TYPE_MISMATCH.clone()),
                    list![
                        ArcStr::from(::std::format!("{}", pos)),
                        literal!("matrix constructor "),
                        literal!("arg"),
                        Expression::toString(e.clone())?,
                        Type::toString(&ty1)?,
                        Type::toString(&ty2)?
                    ],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            res = metamodelica::cons(e, res);
            tys2 = metamodelica::cons(ty3, tys2);
        }
        (arrayExp, arrayType) = BuiltinCall::makeCatExp(2, &res, tys2, variability, purity, info)?;
    } else {
        (arrayExp, arrayType, variability, _) = typeExp((elements).head().cloned()?, context, info, false)?;
    }
    Ok((arrayExp, arrayType, variability, purity))
}

pub(crate) fn typeRange(
    mut rangeExp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut rangeExp: metamodelica::Ref<Expression::NFExpression> = rangeExp;
    let mut rangeType: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut step_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut stop_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut start_ty: metamodelica::Ref<Type::NFType>;
    let mut step_ty: metamodelica::Ref<Type::NFType>;
    let mut stop_ty: metamodelica::Ref<Type::NFType>;
    let mut ostep_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut ostep_ty: Option<metamodelica::Ref<Type::NFType>>;
    let mut start_var: Variability;
    let mut step_var: Variability;
    let mut stop_var: Variability;
    let mut start_pur: Purity;
    let mut step_pur: Purity;
    let mut stop_pur: Purity;
    let mut ty_match: MatchKind;
    let mut next_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(rangeExp) {
        Deref @ Expression::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    start_exp = metamodelica::Own::own(__pa0);
    ostep_exp = metamodelica::Own::own(__pa1);
    stop_exp = metamodelica::Own::own(__pa2);
    (start_exp, start_ty, start_var, start_pur) = typeExp(start_exp, next_context, info, false)?;
    (stop_exp, stop_ty, stop_var, stop_pur) = typeExp(stop_exp, next_context, info, false)?;
    variability = Prefixes::variabilityMax(start_var, stop_var);
    purity = Prefixes::purityMin(start_pur, stop_pur);
    (start_exp, stop_exp, rangeType, ty_match) = TypeCheck::matchExpressions(
        start_exp,
        start_ty.clone(),
        stop_exp,
        stop_ty.clone(),
        TypeCheck::DEFAULT_OPTIONS.clone(),
    )?;
    if TypeCheck::isIncompatibleMatch(ty_match) {
        printRangeTypeError(start_exp.clone(), &start_ty, stop_exp.clone(), &stop_ty, info)?;
    }
    if (ostep_exp).is_some() {
        let __pa3 = ::match_deref::match_deref! { match &(ostep_exp) {
            Some(__pa3) => __pa3.clone(),
            _ => return Err("pattern mismatch"),
        } };
        step_exp = metamodelica::Own::own(__pa3);
        (step_exp, step_ty, step_var, step_pur) = typeExp(step_exp, next_context, info, false)?;
        variability = Prefixes::variabilityMax(step_var, variability);
        purity = Prefixes::purityMin(step_pur, purity);
        (start_exp, step_exp, rangeType, ty_match) = TypeCheck::matchExpressions(
            start_exp,
            start_ty.clone(),
            step_exp,
            step_ty.clone(),
            TypeCheck::DEFAULT_OPTIONS.clone(),
        )?;
        if TypeCheck::isIncompatibleMatch(ty_match) {
            printRangeTypeError(start_exp.clone(), &start_ty, step_exp.clone(), &step_ty, info)?;
        }
        (stop_exp, _, _) =
            TypeCheck::matchTypes_cast(stop_ty, rangeType.clone(), stop_exp, TypeCheck::DEFAULT_OPTIONS.clone())?;
        ostep_exp = Some(step_exp);
        ostep_ty = Some(step_ty);
    } else {
        ostep_exp = None;
        ostep_ty = None;
    }
    rangeType = TypeCheck::getRangeType(start_exp.clone(), ostep_exp.clone(), stop_exp.clone(), rangeType, info)?;
    rangeExp = metamodelica::Ref::new(Expression::NFExpression::RANGE {
        ty: rangeType.clone(),
        start: start_exp,
        step: ostep_exp,
        stop: stop_exp,
    });
    if variability <= Variability::PARAMETER.clone()
        && purity == Purity::PURE.clone()
        && !(InstContext::inFunction(context))
    {
        Structural::markExp(&rangeExp)?;
    }
    Ok((rangeExp, rangeType, variability, purity))
}

pub(crate) fn typeTuple(
    mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut tupleExp: metamodelica::Ref<Expression::NFExpression>;
    let mut tupleType: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity = Purity::PURE.clone();
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut tyl: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut valr: metamodelica::List<Variability>;
    let mut next_context: i32;
    if !(InstContext::onLHS(context)) || InstContext::inSubexpression(context) {
        Error::addSourceMessage(
            &(Error::RHS_TUPLE_EXPRESSION.clone()),
            list![Expression::toString(metamodelica::Ref::new(
                Expression::NFExpression::TUPLE {
                    ty: crate::NFType::interned_UNKNOWN(),
                    elements: elements.clone()
                }
            ))?],
            info,
        )?;
        return Err("fail");
    }
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    (expl, tyl, valr) = typeExpl(elements, next_context, info)?;
    tupleType = metamodelica::Ref::new(Type::NFType::TUPLE {
        types: tyl,
        names: None,
    });
    tupleExp = metamodelica::Ref::new(Expression::NFExpression::TUPLE {
        ty: tupleType.clone(),
        elements: expl.clone(),
    });
    if !(List::all(
        &expl,
        &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Expression::isCref(&__a0))
        },
    )?) {
        Error::addSourceMessage(
            &(Error::TUPLE_ASSIGN_CREFS_ONLY.clone()),
            list![Expression::toString(tupleExp.clone())?],
            info,
        )?;
        return Err("fail");
    }
    variability = if ((valr).is_empty()) {
        Variability::CONSTANT.clone()
    } else {
        (valr).head().cloned()?
    };
    Ok((tupleExp, tupleType, variability, purity))
}

pub(crate) fn printRangeTypeError(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut ty1: &metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut ty2: &metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<()> {
    Error::addSourceMessage(
        &(Error::RANGE_TYPE_MISMATCH.clone()),
        list![
            Expression::toString(exp1)?,
            Type::toString(ty1)?,
            Expression::toString(exp2)?,
            Type::toString(ty2)?
        ],
        info,
    )?;
    return Err("fail");
    Ok(())
}

pub(crate) fn typeSize(
    mut sizeExp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: SourceInfo,
    mut evaluate: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut sizeExp: metamodelica::Ref<Expression::NFExpression> = sizeExp;
    let mut sizeType: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut index: metamodelica::Ref<Expression::NFExpression>;
    let mut exp_ty: metamodelica::Ref<Type::NFType>;
    let mut index_ty: metamodelica::Ref<Type::NFType>;
    let mut ty_match: MatchKind;
    let mut iindex: i32;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut ty_err: metamodelica::Ref<TypingError::TypingError>;
    let mut oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut next_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let mut expl: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    (sizeExp, sizeType, variability, purity) = (::match_deref::match_deref! { match &(sizeExp.clone()) {
        Deref @ Expression::SIZE { exp: __esc_exp, dimIndex: Some(__esc_index) } => {
            exp = (*__esc_exp).clone();
            index = (*__esc_index).clone();
            (index, index_ty, variability, purity) = typeExp(index.clone(), next_context, &info, false)?;
            (index, _, ty_match) = TypeCheck::matchTypes(index_ty.clone(), crate::NFType::interned_INTEGER(), index.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(ty_match) {
                Error::addSourceMessage(&(Error::ARG_TYPE_MISMATCH.clone()), list![literal!("2"), literal!("size "), literal!("dim"), Expression::toString(index.clone())?, Type::toString(&index_ty)?, literal!("Integer")], &info)?;
                return Err("fail");
            }
            if variability <= Variability::STRUCTURAL_PARAMETER.clone() && purity == Purity::PURE.clone() {
                index = Ceval::evalExp(index.clone(), &(Ceval::noTarget().clone()))?;
                let __pa0 = ::match_deref::match_deref! { match &(index.clone()) {
                    Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                iindex = metamodelica::Own::own(__pa0);
                (dim, oexp, ty_err) = typeExpDim(exp.clone(), iindex, next_context, info.clone())?;
                checkSizeTypingError(&ty_err, exp.clone(), iindex, &info)?;
                if Dimension::isKnown(&dim, false) && evaluate {
                    exp = Dimension::sizeExp(&dim)?;
                } else {
                    if (oexp).is_some() {
                        let __pa1 = ::match_deref::match_deref! { match &(oexp) {
                            Some(__pa1) => __pa1.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        exp = metamodelica::Own::own(__pa1);
                    } else {
                        (exp, _, _, _) = typeExp(exp.clone(), next_context, &info, false)?;
                    }
                    exp = metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
                if !(InstContext::inFunction(context)) || Dimension::isKnown(&dim, false) {
                    variability = Variability::CONSTANT.clone();
                } else {
                    variability = Variability::DISCRETE.clone();
                    purity = Purity::IMPURE.clone();
                }
            } else {
                (exp, exp_ty, _, purity) = typeExp(var_field!((*sizeExp).exp, Expression::NFExpression::SIZE).clone(), next_context, &info, false)?;
                if !(Type::isArray(&exp_ty)) {
                    Error::addSourceMessage(&(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone()), list![literal!("size")], &info)?;
                    return Err("fail");
                }
                if Type::isEmptyArray(&exp_ty)? && !(InstContext::inFunction(context)) {
                    expl = Array::mapList(&(Type::arrayDims(exp_ty)), &move |__a0: metamodelica::Ref<Dimension::NFDimension>| Dimension::sizeExp(&__a0))?;
                    exp = Expression::makeExpArray(expl.clone(), crate::NFType::interned_INTEGER(), false);
                    exp = Expression::makeSubscriptedExp(list![Subscript::makeIndex(index.clone())?], exp.clone(), false)?;
                } else {
                    exp = metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
            }
            (exp.clone(), crate::NFType::interned_INTEGER(), variability, purity)
        },
        Deref @ Expression::SIZE { exp: __sizeExp_exp, .. } => {
            (exp, exp_ty, _, _) = typeExp(__sizeExp_exp.clone(), next_context, &info, false)?;
            sizeType = Type::sizeType(exp_ty);
            (metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: exp, dimIndex: None }), sizeType, Variability::PARAMETER.clone(), Purity::PURE.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((sizeExp, sizeType, variability, purity))
}

pub(crate) fn checkSizeTypingError(
    mut typingError: &metamodelica::Ref<TypingError::TypingError>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut index: i32,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (match &**typingError {
        TypingError::NO_ERROR { .. } => (),
        TypingError::OUT_OF_BOUNDS { upperBound: 0 } => {
            Error::addSourceMessage(
                &(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone()),
                list![literal!("size")],
                info,
            )?;
            return Err("fail");
        }
        TypingError::OUT_OF_BOUNDS {
            upperBound: __typingError_upperBound,
        } => {
            Error::addSourceMessage(
                &(Error::INVALID_SIZE_INDEX.clone()),
                list![
                    ArcStr::from(::std::format!("{}", index)),
                    Expression::toString(exp)?,
                    ArcStr::from(::std::format!("{}", __typingError_upperBound.clone()))
                ],
                info,
            )?;
            return Err("fail");
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn evaluateEnd(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut dim: &metamodelica::Ref<Dimension::NFDimension>,
    mut subscriptedExp: &metamodelica::Ref<Expression::NFExpression>,
    mut index: i32,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = (match &*exp {
        Expression::END => Dimension::endExp(dim, subscriptedExp, index)?,
        Expression::CREF { .. } => exp,
        _ => Expression::mapShallow(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = dim.clone();
                let __pe_b2 = subscriptedExp.clone();
                let __pe_b3 = index;
                let __pe_b4 = context;
                let __pe_b5 = info.clone();
                move |__pe_a0| evaluateEnd(__pe_a0, &__pe_b1, &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), &__pe_b5)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
    });
    Ok(outExp)
}

pub(crate) fn typeIfExpression(
    mut ifExp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut ifExp: metamodelica::Ref<Expression::NFExpression> = ifExp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut tb: metamodelica::Ref<Expression::NFExpression>;
    let mut fb: metamodelica::Ref<Expression::NFExpression>;
    let mut tb2: metamodelica::Ref<Expression::NFExpression>;
    let mut fb2: metamodelica::Ref<Expression::NFExpression>;
    let mut next_context: i32;
    let mut cond_ty: metamodelica::Ref<Type::NFType>;
    let mut tb_ty: metamodelica::Ref<Type::NFType>;
    let mut fb_ty: metamodelica::Ref<Type::NFType>;
    let mut cond_var: Variability;
    let mut tb_var: Variability;
    let mut fb_var: Variability;
    let mut cond_pur: Purity;
    let mut tb_pur: Purity;
    let mut fb_pur: Purity;
    let mut ty_match: MatchKind;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ifExp) {
        Deref @ Expression::IF { condition: __pa0, trueBranch: __pa1, falseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cond = metamodelica::Own::own(__pa0);
    tb = metamodelica::Own::own(__pa1);
    fb = metamodelica::Own::own(__pa2);
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    (cond, cond_ty, cond_var, cond_pur) = typeExp(cond, next_context, info, false)?;
    (cond, _, ty_match) = TypeCheck::matchTypes(
        cond_ty.clone(),
        crate::NFType::interned_BOOLEAN(),
        cond,
        TypeCheck::DEFAULT_OPTIONS.clone(),
    )?;
    if TypeCheck::isIncompatibleMatch(ty_match) {
        Error::addSourceMessage(
            &(Error::IF_CONDITION_TYPE_ERROR.clone()),
            list![Expression::toString(cond.clone())?, Type::toString(&cond_ty)?],
            info,
        )?;
        return Err("fail");
    }
    (tb, tb_ty, tb_var, tb_pur) = typeExp(tb, next_context, info, false)?;
    (fb, fb_ty, fb_var, fb_pur) = typeExp(fb, next_context, info, false)?;
    (tb2, fb2, ty, ty_match) = TypeCheck::matchIfBranches(
        tb.clone(),
        tb_ty.clone(),
        fb.clone(),
        fb_ty.clone(),
        context,
        TypeCheck::DEFAULT_OPTIONS.clone(),
    )?;
    if TypeCheck::isIncompatibleMatch(ty_match) {
        Error::addSourceMessage(
            &(Error::TYPE_MISMATCH_IF_EXP.clone()),
            list![
                literal!(""),
                Expression::toString(tb)?,
                Type::toString(&tb_ty)?,
                Expression::toString(fb)?,
                Type::toString(&fb_ty)?
            ],
            info,
        )?;
        return Err("fail");
    }
    if Expression::contains(
        tb2.clone(),
        &({
            let __pe_b1 = literal!("der");
            move |__pe_a0| Expression::isCallNamed(&__pe_a0, &__pe_b1)
        }),
    )? != Expression::contains(
        fb2.clone(),
        &({
            let __pe_b1 = literal!("der");
            move |__pe_a0| Expression::isCallNamed(&__pe_a0, &__pe_b1)
        }),
    )? && metamodelica::stringEq(
        &(Flags::getConfigString(Flags::EVALUATE_STRUCTURAL_PARAMETERS.clone())?),
        &(literal!("all")),
    ) {
        Structural::markExp(&cond)?;
    }
    ifExp = metamodelica::Ref::new(Expression::NFExpression::IF {
        ty: ty.clone(),
        condition: cond,
        trueBranch: tb2,
        falseBranch: fb2,
    });
    var = Prefixes::variabilityMax(cond_var, Prefixes::variabilityMax(tb_var, fb_var));
    purity = Prefixes::purityMin(cond_pur, Prefixes::purityMin(tb_pur, fb_pur));
    Ok((ifExp, ty, var, purity))
}

pub(crate) fn typeClassSections(mut classNode: metamodelica::Ref<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut typed_cls: metamodelica::Ref<Class::NFClass>;
    let mut components: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    let mut info: SourceInfo;
    let mut initial_context: i32;
    cls = NFInstNode::InstNode::getClass(classNode.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { ty: __cls_ty, .. } if (Type::isBasic(Type::arrayElementType(metamodelica::AsArg::as_arg(&__cls_ty)))) => (),
        Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_components, .. }, sections: __esc_sections, restriction: __cls_restriction, .. } => {
            components = (*__esc_components).clone();
            sections = (*__esc_sections).clone();
            sections = (match &*sections.clone() {
        Sections::SECTIONS { .. } => {
            initial_context = InstContext::set(context, InstContext::INITIAL.clone());
            Sections::map(sections.clone(), &({ let __pe_b1 = InstContext::set(context, InstContext::EQUATION.clone()); move |__pe_a0| typeEquation(__pe_a0, __pe_b1.clone()) }), &({ let __pe_b1 = InstContext::set(context, InstContext::ALGORITHM.clone()); move |__pe_a0| typeAlgorithm(__pe_a0, __pe_b1.clone()) }), &({ let __pe_b1 = InstContext::set(initial_context, InstContext::EQUATION.clone()); move |__pe_a0| typeEquation(__pe_a0, __pe_b1.clone()) }), &({ let __pe_b1 = InstContext::set(initial_context, InstContext::ALGORITHM.clone()); move |__pe_a0| typeAlgorithm(__pe_a0, __pe_b1.clone()) }))?
        },
        Sections::EXTERNAL { .. } => {
            Error::addSourceMessage(&(Error::TRANS_VIOLATION.clone()), list![NFInstNode::InstNode::name(&classNode)?, Restriction::toString(metamodelica::AsArg::as_arg(&__cls_restriction)), literal!("external declaration")], &(NFInstNode::InstNode::info(&classNode)))?;
            return Err("fail")
        },
        _ => sections.clone(),
    });
            typed_cls = Class::setSections(sections.clone(), cls)?;
            let __range0 = components.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentSections(&(NFInstNode::InstNode::resolveOuter(c)), context)?;
            }
            NFInstNode::InstNode::updateClass(typed_cls, classNode)?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::TYPED_DERIVED { baseClass: __cls_baseClass, .. } => {
            typeClassSections(__cls_baseClass.clone(), context)?;
            ()
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeClassSections")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&classNode)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeFunctionSections(
    mut classNode: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<()> {
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut typed_cls: metamodelica::Ref<Class::NFClass>;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    let mut info: SourceInfo;
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
    cls = NFInstNode::InstNode::getClass(classNode.clone())?;
    let () = (match &*cls {
        Class::INSTANCED_CLASS {
            sections: __esc_sections,
            ..
        } => {
            sections = (*__esc_sections).clone();
            sections = (::match_deref::match_deref! { match &(sections.clone()) {
                Deref @ Sections::SECTIONS { equations: Deref @ metamodelica::ListNode::Nil, initialEquations: Deref @ metamodelica::ListNode::Nil, algorithms: Deref @ metamodelica::ListNode::Cons { head: __esc_alg, tail: Deref @ metamodelica::ListNode::Nil }, initialAlgorithms: Deref @ metamodelica::ListNode::Nil } => {
                    alg = (*__esc_alg).clone();
                    assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = list![typeAlgorithm(alg.clone(), InstContext::set(context, InstContext::ALGORITHM.clone()))?]);
                    sections.clone()
                },
                Deref @ Sections::SECTIONS { .. } => {
                    Error::addSourceMessage(&(Error::MULTIPLE_SECTIONS_IN_FUNCTION.clone()), list![NFInstNode::InstNode::name(&classNode)?], &(NFInstNode::InstNode::info(&classNode)))?;
                    return Err("fail")
                },
                Deref @ Sections::EXTERNAL { explicit: true, args: __sections_args, .. } => {
                    info = NFInstNode::InstNode::info(&classNode);
                    assign_variant_field!(sections => Sections::NFSections::EXTERNAL;
                        args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (__sections_args.clone()).into_iter().cloned() {
                    let __x = typeExternalArg(arg.clone(), info.clone(), &classNode)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        outputRef = typeCref(var_field!((*sections).outputRef, Sections::NFSections::EXTERNAL).clone(), context, &info)?.0
                    );
                    checkExternalCallResult(var_field!((*sections).outputRef, Sections::NFSections::EXTERNAL), &info)?;
                    sections.clone()
                },
                Deref @ Sections::EXTERNAL { .. } => makeDefaultExternalCall(sections.clone(), &classNode)?,
                _ => sections.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            typed_cls = Class::setSections(sections.clone(), cls)?;
            NFInstNode::InstNode::updateClass(typed_cls, classNode)?;
            ()
        }
        Class::TYPED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => {
            typeFunctionSections(__cls_baseClass.clone(), context)?;
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTyping.typeFunctionSections"));
                    __mm_s.push_str(&*literal!(" got uninstantiated class "));
                    __mm_s.push_str(&*NFInstNode::InstNode::name(&classNode)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(())
}

pub(crate) fn typeExternalArg(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut info: SourceInfo,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outArg: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut index: metamodelica::Ref<Expression::NFExpression>;
    outArg = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::SIZE { dimIndex: Some(_), .. } => {
            (outArg, _, _, _) = typeSize(arg.clone(), InstContext::FUNCTION.clone(), info.clone(), false)?;
            let __pa0 = ::match_deref::match_deref! { match &(outArg.clone()) {
                Deref @ Expression::SIZE { dimIndex: Some(__pa0), .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            index = metamodelica::Own::own(__pa0);
            if !(Expression::isInteger(&index)) {
                Error::addSourceMessage(&(Error::EXTERNAL_ARG_NONCONSTANT_SIZE_INDEX.clone()), list![Expression::toString(arg)?], &info)?;
                return Err("fail");
            }
            outArg
        },
        _ => {
            (outArg, ty, var, _) = typeExp(arg.clone(), InstContext::FUNCTION.clone(), &info, false)?;
            Call::updateExternalRecordArgsInType(&ty)?;
            (match &*arg {
        Expression::CREF { .. } => outArg,
        _ => {
            if Type::isScalarBuiltin(ty)? && var == Variability::CONSTANT.clone() {
                outArg = Ceval::evalExp(outArg, &(Ceval::EvalTarget::new(info, InstContext::FUNCTION.clone(), None)))?;
            } else {
                Error::addSourceMessage(&(Error::EXTERNAL_ARG_WRONG_EXP.clone()), list![Expression::toString(outArg.clone())?], &info)?;
                return Err("fail");
            }
            outArg
        },
    })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub(crate) fn makeDefaultExternalCall(
    mut extDecl: metamodelica::Ref<Sections::NFSections>,
    mut fnNode: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Sections::NFSections>> {
    let mut extDecl: metamodelica::Ref<Sections::NFSections> = extDecl;
    extDecl = (match &*extDecl {
        Sections::EXTERNAL {
            language: __extDecl_language,
            ..
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut r#fn: metamodelica::Ref<Function::Function>;
            let mut single_output: bool;
            let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
            let mut comp: metamodelica::Ref<Component::NFComponent>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut out_cell: metamodelica::Ref<NFInstNode::NodeHandle>;
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            if metamodelica::stringEq(&__extDecl_language, &(literal!("builtin"))) {
                return Ok(extDecl);
            }
            let __pa0 = ::match_deref::match_deref! { match &(NFInstNode::InstNode::getFuncCache(fnNode)?) {
                Deref @ NFInstNode::CachedData::FUNCTION { funcs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r#fn = metamodelica::Own::own(__pa0);
            single_output = ((r#fn.outputs).len() as i32) == 1;
            if single_output && Type::isArray(&(Function::returnType(&r#fn))) {
                single_output = false;
                Error::addSourceMessage(
                    &(Error::EXT_FN_SINGLE_RETURN_ARRAY.clone()),
                    list![__extDecl_language.clone()],
                    &(NFInstNode::InstNode::info(fnNode)),
                )?;
            }
            if single_output {
                let __pa2 = ::match_deref::match_deref! { match &(r#fn.outputs.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                out_cell = metamodelica::Own::own(__pa2);
                node = NFInstNode::InstNode::fromHandle(&out_cell)?;
                ty = NFInstNode::InstNode::getType(node.clone())?;
                assign_variant_field!(extDecl => Sections::NFSections::EXTERNAL; outputRef = ComponentRef::fromNode(node, ty, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?);
            }
            comps = ClassTree::getComponents(
                &(Class::classTree(NFInstNode::InstNode::getClass(NFInstNode::InstNode::fromHandle(
                    &r#fn.node,
                )?)?)?),
            )?;
            if metamodelica::arrayLength(comps.clone()) > 0 {
                args = metamodelica::nil();
                let __range4 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range4 {
                    comp = NFInstNode::InstNode::component(&c)?;
                    if !(single_output) || Component::direction(&comp) != Direction::OUTPUT.clone() {
                        ty = Component::getType(&comp)?;
                        exp = metamodelica::Ref::new(Expression::NFExpression::CREF {
                            ty: ty.clone(),
                            cref: ComponentRef::fromNode(
                                c,
                                ty.clone(),
                                metamodelica::nil(),
                                ComponentRef::Origin::CREF.clone(),
                            )?,
                        });
                        args = metamodelica::cons(exp.clone(), args);
                        for mut i in 1..=Type::dimensionCount(ty) {
                            args = metamodelica::cons(
                                metamodelica::Ref::new(Expression::NFExpression::SIZE {
                                    exp: exp.clone(),
                                    dimIndex: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                                        value: i,
                                    })),
                                }),
                                args,
                            );
                        }
                    }
                }
                assign_variant_field!(extDecl => Sections::NFSections::EXTERNAL; args = args.reverse());
            }
            extDecl
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(extDecl)
}

pub(crate) fn checkExternalCallResult(
    mut result: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    if !(ComponentRef::isCref(result)) {
        return Ok(());
    }
    ty = ComponentRef::nodeType(result)?;
    if Type::isArray(&ty) {
        Error::addSourceMessage(
            &(Error::EXTERNAL_FUNCTION_RESULT_ARRAY_TYPE.clone()),
            list![Type::toString(&ty)?],
            info,
        )?;
        return Err("fail");
    }
    if ComponentRef::variability(result)? < Variability::DISCRETE.clone() {
        Error::addSourceMessage(
            &(Error::EXTERNAL_FUNCTION_RESULT_NOT_VAR.clone()),
            metamodelica::nil(),
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn typeComponentSections(
    mut component: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    if NFInstNode::InstNode::isEmpty(component) {
        return Ok(());
    }
    comp = NFInstNode::InstNode::component(component)?;
    if Component::isDeleted(&comp)? || NFInstNode::InstNode::isOnlyOuter(component)? {
        return Ok(());
    }
    let () = (match &*comp {
        Component::COMPONENT {
            classInst: __comp_classInst,
            state: __comp_state,
            ..
        } if (__comp_state.clone() >= ComponentState::TypeChecked.clone()) => {
            typeClassSections(__comp_classInst.clone(), context)?;
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTyping.typeComponentSections"));
                    __mm_s.push_str(&*literal!(" got uninstantiated component "));
                    __mm_s.push_str(&*NFInstNode::InstNode::name(component)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(())
}

pub fn typeEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut context: i32,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut eq: metamodelica::Ref<Equation::NFEquation> = eq;
    eq = (match &*eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scope: __eq_scope,
            source: __eq_source,
            ..
        } => typeEqualityEquation(
            __eq_lhs.clone(),
            __eq_rhs.clone(),
            context,
            __eq_scope.clone(),
            __eq_source.clone(),
        )?,
        Equation::CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scope: __eq_scope,
            source: __eq_source,
        } => typeConnect(
            __eq_lhs.clone(),
            __eq_rhs.clone(),
            context,
            __eq_scope.clone(),
            __eq_source.clone(),
        )?,
        Equation::FOR { .. } => typeForEquation(&eq, context)?,
        Equation::IF {
            branches: __eq_branches,
            scope: __eq_scope,
            source: __eq_source,
        } => typeIfEquation(
            metamodelica::AsArg::as_arg(&__eq_branches),
            context,
            __eq_scope.clone(),
            __eq_source.clone(),
        )?,
        Equation::WHEN {
            branches: __eq_branches,
            scope: __eq_scope,
            source: __eq_source,
        } => typeWhenEquation(
            metamodelica::AsArg::as_arg(&__eq_branches),
            context,
            __eq_scope.clone(),
            __eq_source.clone(),
        )?,
        Equation::ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(__eq_source.clone());
            (e1, e2, e3) = typeAssert(
                __eq_condition.clone(),
                __eq_message.clone(),
                __eq_level.clone(),
                context,
                &info,
            )?;
            metamodelica::Ref::new(Equation::NFEquation::ASSERT {
                condition: e1,
                message: e2,
                level: e3,
                scope: __eq_scope.clone(),
                source: __eq_source.clone(),
            })
        }
        Equation::TERMINATE {
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(__eq_source.clone());
            (e1, _) = typeOperatorArg(
                __eq_message.clone(),
                crate::NFType::interned_STRING(),
                context,
                literal!("terminate"),
                literal!("message"),
                1,
                &info,
            )?;
            metamodelica::Ref::new(Equation::NFEquation::TERMINATE {
                message: e1,
                scope: __eq_scope.clone(),
                source: __eq_source.clone(),
            })
        }
        Equation::REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            (e1, e2) = typeReinit(__eq_cref.clone(), __eq_reinitExp.clone(), context, __eq_source.clone())?;
            metamodelica::Ref::new(Equation::NFEquation::REINIT {
                cref: e1,
                reinitExp: e2,
                scope: __eq_scope.clone(),
                source: __eq_source.clone(),
            })
        }
        Equation::NORETCALL {
            exp: __eq_exp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            (e1, _, _, _) = typeExp(
                __eq_exp.clone(),
                context,
                &(ElementSource::getInfo(__eq_source.clone())),
                false,
            )?;
            metamodelica::Ref::new(Equation::NFEquation::NORETCALL {
                exp: e1,
                scope: __eq_scope.clone(),
                source: __eq_source.clone(),
            })
        }
        _ => eq,
    });
    Ok(eq)
}

pub(crate) fn typeConnect(
    mut lhsConn: metamodelica::Ref<Expression::NFExpression>,
    mut rhsConn: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut connEq: metamodelica::Ref<Equation::NFEquation>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut lhs_ty: metamodelica::Ref<Type::NFType>;
    let mut rhs_ty: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    let mut next_context: i32;
    let mut info: SourceInfo;
    let mut lhs_deleted: bool;
    let mut rhs_deleted: bool;
    info = ElementSource::getInfo(source.clone());
    if InstContext::inNonexpandable(context) {
        Error::addSourceMessage(
            &(Error::IN_NON_EVALUABLE_IF_OR_FOR.clone()),
            list![literal!("connect")],
            &info,
        )?;
        return Err("fail");
    }
    next_context = InstContext::set(context, InstContext::CONNECT.clone());
    (lhs, lhs_ty, lhs_deleted) = typeConnector(lhsConn.clone(), next_context, &info)?;
    (rhs, rhs_ty, rhs_deleted) = typeConnector(rhsConn.clone(), next_context, &info)?;
    if !(lhs_deleted || rhs_deleted)
        && !(Type::isExpandableConnector(&(Type::arrayElementType(&lhs_ty)))
            || Type::isExpandableConnector(&(Type::arrayElementType(&rhs_ty))))
    {
        (lhs, rhs, _, mk) = TypeCheck::matchExpressions(lhs, lhs_ty, rhs, rhs_ty, TypeCheck::ALLOW_UNKNOWN.clone())?;
        if TypeCheck::isIncompatibleMatch(mk) {
            Error::addSourceMessage(
                &(Error::CONNECT_TYPE_MISMATCH.clone()),
                list![Expression::toString(lhsConn)?, Expression::toString(rhsConn)?],
                &info,
            )?;
            return Err("fail");
        }
    }
    connEq = metamodelica::Ref::new(Equation::NFEquation::CONNECT {
        lhs: lhs,
        rhs: rhs,
        scope: scope,
        source: source,
    });
    Ok(connEq)
}

pub(crate) fn typeConnector(
    mut connExp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    bool,
)> {
    let mut connExp: metamodelica::Ref<Expression::NFExpression> = connExp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut deleted: bool;
    (connExp, ty, _, _) = typeExp(connExp, context, info, false)?;
    deleted = checkConnector(connExp.clone(), info)?;
    Ok((connExp, ty, deleted))
}

pub(crate) fn checkConnector(
    mut connExp: metamodelica::Ref<Expression::NFExpression>,
    mut info: &SourceInfo,
) -> Result<bool> {
    let mut deleted: bool = false;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let () = (::match_deref::match_deref! { match &(connExp.clone()) {
        Deref @ Expression::CREF { cref: __esc_cr @ Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF { .. }, .. }, .. } => {
            cr = (*__esc_cr).clone();
            if !(NFInstNode::InstNode::isConnector(&(ComponentRef::node(metamodelica::AsArg::as_arg(&cr))?))?) {
                Error::addSourceMessageAndFail(&(Error::INVALID_CONNECTOR_TYPE.clone()), list![ComponentRef::toString(metamodelica::AsArg::as_arg(&cr))?], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if !(checkConnectorForm(metamodelica::AsArg::as_arg(&cr), true)?) {
                Error::addSourceMessageAndFail(&(Error::INVALID_CONNECTOR_FORM.clone()), list![ComponentRef::toString(metamodelica::AsArg::as_arg(&cr))?], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if ComponentRef::subscriptsVariability(metamodelica::AsArg::as_arg(&cr), Prefixes::Variability::CONSTANT.clone())? > Variability::NON_STRUCTURAL_PARAMETER.clone() {
                subs = ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&cr))?;
                for mut sub in &*subs {
                    if Subscript::variability(metamodelica::AsArg::as_arg(&sub))? > Variability::NON_STRUCTURAL_PARAMETER.clone() {
                        Error::addSourceMessage(&(Error::CONNECTOR_NON_PARAMETER_SUBSCRIPT.clone()), list![Expression::toString(connExp.clone())?, Subscript::toString(metamodelica::AsArg::as_arg(&sub))?], info)?;
                        return Err("fail");
                    }
                }
            }
            deleted = ComponentRef::isDeleted(metamodelica::AsArg::as_arg(&cr))?;
            ()
        },
        _ => {
            Error::addSourceMessage(&(Error::INVALID_CONNECTOR_TYPE.clone()), list![Expression::toString(connExp)?], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(deleted)
}

pub(crate) fn checkConnectorForm<'__b>(
    mut cref: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut isConnector: bool,
) -> Result<bool> {
    '__tco: loop {
        match &**cref {
            ComponentRef::CREF {
                origin: ComponentRef::Origin::CREF { .. },
                ..
            } => {
                if (isConnector) {
                    {
                        (cref, isConnector) = (
                            var_field!((**cref).restCref, ComponentRef::NFComponentRef::CREF),
                            NFInstNode::InstNode::isConnector(&(ComponentRef::node(cref)?))?,
                        );
                        continue '__tco;
                    }
                } else {
                    return Ok(false);
                }
            }
            _ => return Ok(true),
        }
    }
}

pub(crate) fn checkLhsInWhen(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> bool {
    let mut isValid: bool;
    isValid = (match &**exp {
        Expression::CREF { .. } => true,
        Expression::TUPLE {
            elements: __exp_elements,
            ..
        } => {
            for mut e in &*__exp_elements.clone() {
                checkLhsInWhen(metamodelica::AsArg::as_arg(&e));
            }
            true
        }
        _ => false,
    });
    isValid
}

pub(crate) fn typeAssert(
    mut condition: metamodelica::Ref<Expression::NFExpression>,
    mut message: metamodelica::Ref<Expression::NFExpression>,
    mut level: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut condition: metamodelica::Ref<Expression::NFExpression> = condition;
    let mut message: metamodelica::Ref<Expression::NFExpression> = message;
    let mut level: metamodelica::Ref<Expression::NFExpression> = level;
    let mut next_context: i32;
    let mut level_var: Variability;
    next_context = InstContext::set(context, InstContext::ASSERT.clone());
    (condition, _) = typeOperatorArg(
        condition,
        crate::NFType::interned_BOOLEAN(),
        InstContext::set(next_context, InstContext::CONDITION.clone()),
        literal!("assert"),
        literal!("condition"),
        1,
        info,
    )?;
    (message, _) = typeOperatorArg(
        message,
        crate::NFType::interned_STRING(),
        next_context,
        literal!("assert"),
        literal!("message"),
        2,
        info,
    )?;
    (level, level_var) = typeOperatorArg(
        level,
        Builtin::ASSERTIONLEVEL_TYPE().clone(),
        next_context,
        literal!("assert"),
        literal!("level"),
        3,
        info,
    )?;
    if level_var > Variability::PARAMETER.clone() {
        Error::addSourceMessage(
            &(Error::FUNCTION_SLOT_VARIABILITY.clone()),
            list![
                literal!("level"),
                Expression::toString(level.clone())?,
                literal!("assert"),
                Prefixes::variabilityString(level_var)?,
                literal!("parameter")
            ],
            info,
        )?;
        return Err("fail");
    }
    Structural::markExp(&level)?;
    Ok((condition, message, level))
}

pub(crate) fn typeAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut context: i32,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(
        alg.statements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
            for mut s in (alg.statements.clone()).into_iter().cloned() {
                let __x = typeStatement(s.clone(), context)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(alg)
}

pub(crate) fn typeStatements(
    mut alg: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut context: i32,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut alg: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = alg;
    alg = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        for mut stmt in (alg).into_iter().cloned() {
            let __x = typeStatement(stmt.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(alg)
}

pub(crate) fn typeStatement(
    mut st: metamodelica::Ref<Statement::NFStatement>,
    mut context: i32,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut st: metamodelica::Ref<Statement::NFStatement> = st;
    st = (match &*st.clone() {
        Statement::ASSIGNMENT {
            lhs: __st_lhs,
            rhs: __st_rhs,
            source: __st_source,
            ..
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut ty1: metamodelica::Ref<Type::NFType>;
            let mut ty2: metamodelica::Ref<Type::NFType>;
            let mut mk: MatchKind;
            let mut info: SourceInfo;
            let mut var: Variability;
            info = ElementSource::getInfo(__st_source.clone());
            (e1, ty1, var, _) = typeExp(
                __st_lhs.clone(),
                InstContext::set(context, InstContext::LHS.clone()),
                &info,
                false,
            )?;
            (e2, ty2, _, _) = typeExp(
                __st_rhs.clone(),
                InstContext::set(context, InstContext::RHS.clone()),
                &info,
                false,
            )?;
            (e2, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), e2, TypeCheck::ALLOW_UNKNOWN.clone())?;
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessage(
                    &(Error::ASSIGN_TYPE_MISMATCH_ERROR.clone()),
                    list![
                        Expression::toString(e1.clone())?,
                        Expression::toString(e2.clone())?,
                        Type::toString(&ty1)?,
                        Type::toString(&ty2)?
                    ],
                    &info,
                )?;
                return Err("fail");
            }
            checkAssignment(&e1, &e2, var, context, &info)?;
            if Expression::isExternalCall(&e2)? {
                Call::updateExternalRecordArgs(&(Expression::tupleElements(e1.clone())))?;
            }
            metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                lhs: e1,
                rhs: e2,
                ty: ty1,
                source: __st_source.clone(),
            })
        }
        Statement::FOR {
            body: __st_body,
            forType: __st_forType,
            iterator: __st_iterator,
            range: __st_range,
            source: __st_source,
            sub_iters: __st_sub_iters,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut next_context: i32;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(__st_source.clone());
            if (__st_range).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(__st_range.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                e1 = metamodelica::Own::own(__pa0);
            } else {
                e1 = deduceIterationRangeStmt(&st, metamodelica::AsArg::as_arg(&__st_iterator), &info)?;
            }
            (e1, _, _, _) = typeIterator(__st_iterator.clone(), e1, context, false)?;
            next_context = InstContext::set(context, InstContext::FOR.clone());
            body = typeStatements(__st_body.clone(), next_context)?;
            metamodelica::Ref::new(Statement::NFStatement::FOR {
                iterator: __st_iterator.clone(),
                range: Some(e1),
                body: body,
                forType: __st_forType.clone(),
                source: __st_source.clone(),
                sub_iters: __st_sub_iters.clone(),
            })
        }
        Statement::IF {
            branches: __st_branches,
            source: __st_source,
        } => {
            let mut cond: metamodelica::Ref<Expression::NFExpression>;
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut sts1: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut tybrs: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            )>;
            let mut next_context: i32;
            let mut cond_context: i32;
            next_context = InstContext::set(context, InstContext::IF.clone());
            cond_context = InstContext::set(next_context, InstContext::CONDITION.clone());
            tybrs = ({
                let mut __acc: metamodelica::List<(
                    metamodelica::Ref<Expression::NFExpression>,
                    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
                )> = metamodelica::nil();
                for mut br in (__st_branches.clone()).into_iter().cloned() {
                    let __x = (::match_deref::match_deref! { match &(br.clone()) {
                        (__esc_cond, __esc_body) => {
                            cond = (*__esc_cond).clone();
                            body = (*__esc_body).clone();
                            (e1, _, _) = typeCondition(cond.clone(), cond_context, __st_source.clone(), &(Error::IF_CONDITION_TYPE_ERROR.clone()), false, false)?;
                            sts1 = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
                        for mut bst in (body.clone()).into_iter().cloned() {
                            let __x = typeStatement(bst.clone(), next_context)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                            (e1.clone(), sts1.clone())
                        },
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            metamodelica::Ref::new(Statement::NFStatement::IF {
                branches: tybrs,
                source: __st_source.clone(),
            })
        }
        Statement::WHEN {
            branches: __st_branches,
            source: __st_source,
        } => {
            let mut cond: metamodelica::Ref<Expression::NFExpression>;
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut sts1: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut tybrs: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            )>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::WHEN.clone());
            tybrs = ({
                let mut __acc: metamodelica::List<(
                    metamodelica::Ref<Expression::NFExpression>,
                    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
                )> = metamodelica::nil();
                for mut br in (__st_branches.clone()).into_iter().cloned() {
                    let __x = (::match_deref::match_deref! { match &(br.clone()) {
                        (__esc_cond, __esc_body) => {
                            cond = (*__esc_cond).clone();
                            body = (*__esc_body).clone();
                            (e1, _, _) = typeWhenCondition(cond.clone(), context, __st_source.clone(), false)?;
                            sts1 = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
                        for mut bst in (body.clone()).into_iter().cloned() {
                            let __x = typeStatement(bst.clone(), next_context)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                            (e1.clone(), sts1.clone())
                        },
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            metamodelica::Ref::new(Statement::NFStatement::WHEN {
                branches: tybrs,
                source: __st_source.clone(),
            })
        }
        Statement::ASSERT {
            condition: __st_condition,
            level: __st_level,
            message: __st_message,
            source: __st_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(__st_source.clone());
            (e1, e2, e3) = typeAssert(
                __st_condition.clone(),
                __st_message.clone(),
                __st_level.clone(),
                context,
                &info,
            )?;
            metamodelica::Ref::new(Statement::NFStatement::ASSERT {
                condition: e1,
                message: e2,
                level: e3,
                source: __st_source.clone(),
            })
        }
        Statement::TERMINATE {
            message: __st_message,
            source: __st_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(__st_source.clone());
            if InstContext::inFunction(context) {
                Error::addSourceMessage(
                    &(Error::EXP_INVALID_IN_FUNCTION.clone()),
                    list![literal!("terminate")],
                    &info,
                )?;
                return Err("fail");
            }
            (e1, _) = typeOperatorArg(
                __st_message.clone(),
                crate::NFType::interned_STRING(),
                context,
                literal!("terminate"),
                literal!("message"),
                1,
                &info,
            )?;
            metamodelica::Ref::new(Statement::NFStatement::TERMINATE {
                message: e1,
                source: __st_source.clone(),
            })
        }
        Statement::REINIT {
            cref: __st_cref,
            reinitExp: __st_reinitExp,
            source: __st_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            if InstContext::inFunction(context) {
                Error::addSourceMessage(
                    &(Error::EXP_INVALID_IN_FUNCTION.clone()),
                    list![literal!("reinit")],
                    &(ElementSource::getInfo(__st_source.clone())),
                )?;
                return Err("fail");
            }
            (e1, e2) = typeReinit(__st_cref.clone(), __st_reinitExp.clone(), context, __st_source.clone())?;
            metamodelica::Ref::new(Statement::NFStatement::REINIT {
                cref: e1,
                reinitExp: e2,
                source: __st_source.clone(),
            })
        }
        Statement::NORETCALL {
            exp: __st_exp,
            source: __st_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            (e1, _, _, _) = typeExp(
                __st_exp.clone(),
                context,
                &(ElementSource::getInfo(__st_source.clone())),
                false,
            )?;
            metamodelica::Ref::new(Statement::NFStatement::NORETCALL {
                exp: e1,
                source: __st_source.clone(),
            })
        }
        Statement::WHILE {
            body: __st_body,
            condition: __st_condition,
            source: __st_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut sts1: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            (e1, _, _) = typeCondition(
                __st_condition.clone(),
                context,
                __st_source.clone(),
                &(Error::WHILE_CONDITION_TYPE_ERROR.clone()),
                false,
                false,
            )?;
            sts1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
                for mut bst in (__st_body.clone()).into_iter().cloned() {
                    let __x = typeStatement(bst.clone(), context)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            metamodelica::Ref::new(Statement::NFStatement::WHILE {
                condition: e1,
                body: sts1,
                source: __st_source.clone(),
            })
        }
        Statement::FAILURE {
            body: __st_body,
            source: __st_source,
        } => {
            let mut sts1: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            sts1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
                for mut bst in (__st_body.clone()).into_iter().cloned() {
                    let __x = typeStatement(bst.clone(), context)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            metamodelica::Ref::new(Statement::NFStatement::FAILURE {
                body: sts1,
                source: __st_source.clone(),
            })
        }
        _ => st,
    });
    Ok(st)
}

pub(crate) fn checkAssignment(
    mut lhsExp: &metamodelica::Ref<Expression::NFExpression>,
    mut rhsExp: &metamodelica::Ref<Expression::NFExpression>,
    mut lhsVar: Variability,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<()> {
    if InstContext::inInstanceAPI(context) {
        return Ok(());
    }
    let () = (match &**lhsExp {
        Expression::TUPLE {
            elements: __lhsExp_elements,
            ..
        } => {
            let mut i: i32;
            i = 1;
            for mut e in &*__lhsExp_elements.clone() {
                checkAssignment(
                    metamodelica::AsArg::as_arg(&e),
                    &(Expression::tupleElement(rhsExp.clone(), i)?),
                    Expression::variability(e.clone())?,
                    context,
                    info,
                )?;
                i = i + 1;
            }
            ()
        }
        Expression::CREF {
            cref: __lhsExp_cref, ..
        } if (InstContext::inFunction(context)) => {
            if ComponentRef::isCref(metamodelica::AsArg::as_arg(&__lhsExp_cref))
                && NFInstNode::InstNode::isInput(&(ComponentRef::node(metamodelica::AsArg::as_arg(&__lhsExp_cref))?))
            {
                Error::addSourceMessage(
                    &(Error::ASSIGN_READONLY_ERROR.clone()),
                    list![
                        literal!("input"),
                        ComponentRef::toString(metamodelica::AsArg::as_arg(&__lhsExp_cref))?
                    ],
                    info,
                )?;
                return Err("fail");
            }
            ()
        }
        _ => {
            if lhsVar < Variability::DISCRETE.clone() {
                if lhsVar == Variability::CONSTANT.clone() {
                    Error::addSourceMessage(
                        &(Error::ASSIGN_CONSTANT_ERROR.clone()),
                        list![
                            Expression::toString(lhsExp.clone())?,
                            Expression::toString(rhsExp.clone())?
                        ],
                        info,
                    )?;
                    return Err("fail");
                } else if !(InstContext::inInitial(context)) {
                    Error::addSourceMessage(
                        &(Error::ASSIGN_PARAM_ERROR.clone()),
                        list![
                            Expression::toString(lhsExp.clone())?,
                            Expression::toString(rhsExp.clone())?
                        ],
                        info,
                    )?;
                    return Err("fail");
                }
            }
            ()
        }
    });
    Ok(())
}

pub(crate) fn typeEqualityEquation(
    mut lhsExp: metamodelica::Ref<Expression::NFExpression>,
    mut rhsExp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    let mut info: SourceInfo = ElementSource::getInfo(source.clone());
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    if InstContext::inWhen(context) && !(InstContext::inClocked(context)) {
        if checkLhsInWhen(&lhsExp) {
            Structural::markSubscriptsInExp(&lhsExp)?;
        } else {
            Error::addSourceMessage(
                &(Error::WHEN_EQ_LHS.clone()),
                list![Expression::toString(lhsExp.clone())?],
                &info,
            )?;
            return Err("fail");
        }
    }
    (e1, ty1, _, _) = typeExp(
        lhsExp.clone(),
        InstContext::set(context, InstContext::LHS.clone()),
        &info,
        false,
    )?;
    (e2, ty2, _, _) = typeExp(
        rhsExp.clone(),
        InstContext::set(context, InstContext::RHS.clone()),
        &info,
        false,
    )?;
    (e2, e1, ty, mk) =
        TypeCheck::matchExpressions(e2, ty2.clone(), e1, ty1.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(
            &(Error::EQUATION_TYPE_MISMATCH_ERROR.clone()),
            list![
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*Expression::toString(lhsExp)?);
                    __mm_s.push_str(&*literal!(" = "));
                    __mm_s.push_str(&*Expression::toString(rhsExp)?);
                    ArcStr::from(__mm_s)
                },
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*Type::toString(&ty1)?);
                    __mm_s.push_str(&*literal!(" = "));
                    __mm_s.push_str(&*Type::toString(&ty2)?);
                    ArcStr::from(__mm_s)
                }
            ],
            &info,
        )?;
        return Err("fail");
    }
    eq = metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
        lhs: e1.clone(),
        rhs: e2.clone(),
        ty: ty,
        scope: scope,
        source: source,
        scalarizeMode: Equation::ScalarizeMode::NO_PREFERENCE.clone(),
    });
    if Expression::isExternalCall(&e2)? {
        Call::updateExternalRecordArgs(&(Expression::tupleElements(e1)))?;
    }
    Ok(eq)
}

pub(crate) fn typeCondition(
    mut condition: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut errorMsg: &ErrorTypes::Message,
    mut allowVector: bool,
    mut allowClock: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
)> {
    let mut condition: metamodelica::Ref<Expression::NFExpression> = condition;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut info: SourceInfo;
    let mut ety: metamodelica::Ref<Type::NFType>;
    info = ElementSource::getInfo(source);
    (condition, ty, variability, _) = typeExp(condition, context, &info, false)?;
    if allowVector && Type::isArray(&ty) {
        ety = Type::unliftArray(ty.clone())?;
    } else {
        ety = ty.clone();
    }
    if !(Type::isBoolean(&ety) || allowClock && Type::isClock(&ety)?) {
        Error::addSourceMessage(
            errorMsg,
            list![Expression::toString(condition.clone())?, Type::toString(&ty)?],
            &info,
        )?;
        return Err("fail");
    }
    Ok((condition, ty, variability))
}

pub(crate) fn typeForEquation(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut context: i32,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut forEq: metamodelica::Ref<Equation::NFEquation>;
    let mut iterator: metamodelica::Ref<InstNode::InstNode>;
    let mut range: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut info: SourceInfo;
    let mut range_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut range_var: Variability;
    let mut next_context: i32;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*eq)) {
        Deref @ Equation::FOR { iterator: __pa0, range: __pa1, body: __pa2, scope: __pa3, source: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iterator = metamodelica::Own::own(__pa0);
    range = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    scope = metamodelica::Own::own(__pa3);
    src = metamodelica::Own::own(__pa4);
    if (range).is_some() {
        let __pa5 = ::match_deref::match_deref! { match &(range) {
            Some(__pa5) => __pa5.clone(),
            _ => return Err("pattern mismatch"),
        } };
        range_exp = metamodelica::Own::own(__pa5);
    } else {
        range_exp = deduceIterationRangeEq(eq, &iterator, &(ElementSource::getInfo(src.clone())))?;
    }
    (range_exp, _, range_var, _) = typeIterator(iterator.clone(), range_exp, context, true)?;
    next_context = InstContext::set(context, InstContext::FOR.clone());
    if range_var > Variability::NON_STRUCTURAL_PARAMETER.clone()
        || Structural::isExpressionNotFixed(&range_exp, false, 100)?
    {
        next_context = InstContext::set(context, InstContext::NONEXPANDABLE.clone());
    }
    body = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
        for mut e in (body).into_iter().cloned() {
            let __x = typeEquation(e.clone(), next_context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    forEq = metamodelica::Ref::new(Equation::NFEquation::FOR {
        iterator: iterator,
        range: Some(range_exp),
        body: body,
        scope: scope,
        source: src,
    });
    Ok(forEq)
}

pub(crate) fn typeIfEquation(
    mut branches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut context: i32,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut ifEq: metamodelica::Ref<Equation::NFEquation>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut accum_var: Variability = Variability::CONSTANT.clone();
    let mut var: Variability;
    let mut bl: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    let mut bl2: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    let mut next_context: i32 = InstContext::set(context, InstContext::IF.clone());
    let mut cond_context: i32 = InstContext::set(next_context, InstContext::CONDITION.clone());
    for mut b in &**branches {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: _, body: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cond = metamodelica::Own::own(__pa0);
        eql = metamodelica::Own::own(__pa1);
        (cond, _, var) = typeCondition(
            cond,
            cond_context,
            source.clone(),
            &(Error::IF_CONDITION_TYPE_ERROR.clone()),
            false,
            false,
        )?;
        if var > Variability::PARAMETER.clone() || Structural::isExpressionNotFixed(&cond, false, 100)? {
            next_context = InstContext::set(next_context, InstContext::NONEXPANDABLE.clone());
        } else if var == Variability::PARAMETER.clone()
            && (accum_var <= Variability::PARAMETER.clone()
                || Equation::containsList(
                    &eql,
                    &move |__a0: metamodelica::Ref<Equation::NFEquation>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(Equation::isConnection(&__a0))
                    },
                )?)
        {
            var = Variability::STRUCTURAL_PARAMETER.clone();
        }
        accum_var = Prefixes::variabilityMax(accum_var, var);
        bl = metamodelica::cons(
            metamodelica::Ref::new(Equation::Branch::Branch::BRANCH {
                condition: cond,
                conditionVar: var,
                body: eql,
            }),
            bl,
        );
    }
    for mut b in &*bl {
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa2, conditionVar: __pa3, body: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cond = metamodelica::Own::own(__pa2);
        var = metamodelica::Own::own(__pa3);
        eql = metamodelica::Own::own(__pa4);
        ErrorExt::setCheckpoint(literal!("NFTyping.typeIfEquation"));
        match '__try5: {
            eql = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut e in (eql.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(typeEquation(e.clone(), next_context), '__try5);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            bl2 = metamodelica::cons(Equation::makeBranch(cond.clone(), eql.clone(), var), bl2.clone());
            Ok::<_, &'static str>((bl2.clone(),))
        } {
            Ok((__try5_o0,)) => {
                bl2 = __try5_o0;
            }
            Err(_) => {
                bl2 = metamodelica::cons(
                    metamodelica::Ref::new(Equation::Branch::Branch::INVALID_BRANCH {
                        branch: Equation::makeBranch(cond.clone(), eql.clone(), var),
                        errors: ErrorExt::getCheckpointMessages(),
                    }),
                    bl2.clone(),
                );
            }
        }
        ErrorExt::delCheckpoint(literal!("NFTyping.typeIfEquation"));
    }
    ifEq = metamodelica::Ref::new(Equation::NFEquation::IF {
        branches: bl2,
        scope: scope,
        source: source,
    });
    Ok(ifEq)
}

pub(crate) fn typeWhenEquation(
    mut branches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut context: i32,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut whenEq: metamodelica::Ref<Equation::NFEquation>;
    let mut next_context: i32 = InstContext::set(context, InstContext::WHEN.clone());
    let mut accum_branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    for mut branch in &**branches {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: _, body: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cond = metamodelica::Own::own(__pa0);
        body = metamodelica::Own::own(__pa1);
        (cond, ty, var) = typeWhenCondition(cond, context, source.clone(), true)?;
        if Type::isClock(&ty)? {
            if ((branches).len() as i32) != 1 {
                if referenceEq(&*(branch.clone()), &*((branches).head().cloned()?)) {
                    Error::addSourceMessage(
                        &(Error::ELSE_WHEN_CLOCK.clone()),
                        metamodelica::nil(),
                        &(ElementSource::getInfo(source.clone())),
                    )?;
                } else {
                    Error::addSourceMessage(
                        &(Error::CLOCKED_WHEN_BRANCH.clone()),
                        metamodelica::nil(),
                        &(ElementSource::getInfo(source.clone())),
                    )?;
                }
                return Err("fail");
            } else {
                next_context = InstContext::set(context, InstContext::CLOCKED.clone());
            }
        }
        body = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut eq in (body).into_iter().cloned() {
                let __x = typeEquation(eq.clone(), next_context)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        accum_branches = metamodelica::cons(Equation::makeBranch(cond, body, var), accum_branches);
    }
    whenEq = metamodelica::Ref::new(Equation::NFEquation::WHEN {
        branches: metamodelica::Dangerous::listReverseInPlace(accum_branches),
        scope: scope,
        source: source,
    });
    Ok(whenEq)
}

pub(crate) fn typeWhenCondition(
    mut condition: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut allowClock: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
)> {
    let mut outCondition: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    if InstContext::inNonexpandable(context) {
        Error::addSourceMessage(
            &(Error::IN_NON_EVALUABLE_IF_OR_FOR.clone()),
            list![literal!("when")],
            &(ElementSource::getInfo(source.clone())),
        )?;
        return Err("fail");
    }
    (outCondition, ty, variability) = typeCondition(
        condition.clone(),
        context,
        source.clone(),
        &(Error::WHEN_CONDITION_TYPE_ERROR.clone()),
        true,
        allowClock,
    )?;
    if variability > Variability::IMPLICITLY_DISCRETE.clone() && !(Type::isClock(&ty)?) {
        Error::addSourceMessage(
            &(Error::NON_DISCRETE_WHEN_CONDITION.clone()),
            list![Expression::toString(condition.clone())?],
            &(ElementSource::getInfo(source.clone())),
        )?;
        return Err("fail");
    }
    if !(checkWhenInitial(&outCondition)?) {
        Error::addSourceMessage(
            &(Error::INITIAL_CALL_WARNING.clone()),
            list![Expression::toString(condition)?],
            &(ElementSource::getInfo(source)),
        )?;
    }
    Ok((outCondition, ty, variability))
}

pub(crate) fn checkWhenInitial(mut condition: &metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut invalid: bool;
    invalid = (match &**condition {
        Expression::ARRAY { .. } => {
            let __range0 = var_field!((**condition).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut e in __range0 {
                if checkWhenInitial(&e)? {
                    invalid = true;
                    return Ok(invalid);
                }
            }
            false
        }
        _ => {
            !(Expression::containsShallow(
                condition,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                    > = (std::sync::Arc::new({
                        let __pe_b1 = literal!("initial");
                        move |__pe_a0| Expression::isCallNamed(&__pe_a0, &__pe_b1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                        >);
                    move |__pe_a0| Expression::contains(__pe_a0, &*__pe_b1)
                }),
            )?)
        }
    });
    Ok(invalid)
}

pub(crate) fn typeOperatorArg(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut expectedType: metamodelica::Ref<Type::NFType>,
    mut context: i32,
    mut operatorName: ArcStr,
    mut argName: ArcStr,
    mut argIndex: i32,
    mut info: &SourceInfo,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, Variability)> {
    let mut arg: metamodelica::Ref<Expression::NFExpression> = arg;
    let mut var: Variability;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    (arg, ty, var, _) = typeExp(arg, context, info, false)?;
    (arg, _, mk) = TypeCheck::matchTypes(
        ty.clone(),
        expectedType.clone(),
        arg,
        TypeCheck::DEFAULT_OPTIONS.clone(),
    )?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                intString(argIndex),
                operatorName,
                argName,
                Expression::toString(arg.clone())?,
                Type::toString(&ty)?,
                Type::toString(&expectedType)?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok((arg, var))
}

pub(crate) fn typeReinit(
    mut crefExp: metamodelica::Ref<Expression::NFExpression>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut crefExp: metamodelica::Ref<Expression::NFExpression> = crefExp;
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut mk: MatchKind;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut info: SourceInfo;
    info = ElementSource::getInfo(source);
    (crefExp, ty1, _, _) = typeExp(crefExp, context, &info, false)?;
    (exp, ty2, _, _) = typeExp(exp, context, &info, false)?;
    cref = (match &*crefExp {
        Expression::CREF {
            cref: __crefExp_cref, ..
        } => {
            if ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__crefExp_cref)) {
                Error::addSourceMessage(
                    &(Error::ASSIGN_ITERATOR_ERROR.clone()),
                    list![ComponentRef::toString(metamodelica::AsArg::as_arg(&__crefExp_cref))?],
                    &info,
                )?;
                return Err("fail");
            }
            __crefExp_cref.clone()
        }
        _ => {
            Error::addSourceMessage(
                &(Error::REINIT_MUST_BE_VAR_OR_ARRAY.clone()),
                metamodelica::nil(),
                &info,
            )?;
            return Err("fail");
        }
    });
    if ComponentRef::nodeVariability(&cref)? < Variability::IMPLICITLY_DISCRETE.clone() {
        Error::addSourceMessage(
            &(Error::REINIT_MUST_BE_VAR.clone()),
            list![
                Expression::toString(crefExp.clone())?,
                Prefixes::variabilityString(ComponentRef::nodeVariability(&cref)?)?
            ],
            &info,
        )?;
        return Err("fail");
    }
    (_, _, mk) = TypeCheck::matchTypes(
        Type::arrayElementType(&ty1),
        crate::NFType::interned_REAL(),
        crefExp.clone(),
        TypeCheck::DEFAULT_OPTIONS.clone(),
    )?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(
            &(Error::REINIT_MUST_BE_REAL.clone()),
            list![
                Expression::toString(crefExp.clone())?,
                Type::toString(&(Type::arrayElementType(&ty1)))?
            ],
            &info,
        )?;
        return Err("fail");
    }
    (exp, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), exp, TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("2"),
                literal!("reinit"),
                literal!(""),
                Expression::toString(exp.clone())?,
                Type::toString(&ty2)?,
                Type::toString(&ty1)?
            ],
            &info,
        )?;
        return Err("fail");
    }
    Ok((crefExp, exp))
}

pub(crate) fn deduceIterationRangeEq(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut iterationRange: metamodelica::Ref<Expression::NFExpression>;
    let mut crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>;
    crefs = Equation::foldExp(
        eq,
        &({
            let __pe_b1 = iterator.clone();
            move |__pe_a0, __pe_a2| collectIteratorCrefs(__pe_a0, &__pe_b1, __pe_a2)
        }),
        metamodelica::nil(),
    )?;
    iterationRange = deduceIterationRange(&crefs, iterator, info)?;
    Ok(iterationRange)
}

pub(crate) fn deduceIterationRangeStmt(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut iterationRange: metamodelica::Ref<Expression::NFExpression>;
    let mut crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>;
    crefs = Statement::foldExp(
        stmt,
        &({
            let __pe_b1 = iterator.clone();
            move |__pe_a0, __pe_a2| collectIteratorCrefs(__pe_a0, &__pe_b1, __pe_a2)
        }),
        metamodelica::nil(),
    )?;
    iterationRange = deduceIterationRange(&crefs, iterator, info)?;
    Ok(iterationRange)
}

pub(crate) fn deduceIterationRangeExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut iterationRange: metamodelica::Ref<Expression::NFExpression>;
    let mut crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>;
    crefs = Expression::fold(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = iterator.clone();
            move |__pe_a0, __pe_a2| collectIteratorCrefs2(&__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
                    )
                        -> Result<metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>>
                    + 'static,
            >),
        metamodelica::nil(),
    )?;
    iterationRange = deduceIterationRange(&crefs, iterator, info)?;
    Ok(iterationRange)
}

pub(crate) fn deduceIterationRange(
    mut crefs: &metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut iterationRange: metamodelica::Ref<Expression::NFExpression>;
    let mut range_cr: (metamodelica::Ref<ComponentRef::NFComponentRef>, i32);
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut dim_index: i32;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut stop_exp: metamodelica::Ref<Expression::NFExpression>;
    if (crefs).is_empty() {
        Error::addSourceMessage(
            &(Error::IMPLICIT_ITERATOR_NOT_FOUND_IN_LOOP_BODY.clone()),
            list![NFInstNode::InstNode::name(iterator)?],
            info,
        )?;
        return Err("fail");
    }
    range_cr = List::reduce(
        crefs,
        &({
            let __pe_b2 = info.clone();
            move |__pe_a0, __pe_a1| deduceIterationRange2(&__pe_a0, __pe_a1, &__pe_b2)
        }),
    )?;
    (cr, dim_index) = range_cr;
    dim = Type::nthDimension(NFInstNode::InstNode::getType(ComponentRef::node(&cr)?)?, dim_index)?;
    start_exp = Dimension::lowerBoundExp(&dim)?;
    stop_exp = Dimension::endExp(
        &dim,
        &(metamodelica::Ref::new(Expression::NFExpression::CREF {
            ty: crate::NFType::interned_UNKNOWN(),
            cref: cr,
        })),
        dim_index,
    )?;
    iterationRange = metamodelica::Ref::new(Expression::NFExpression::RANGE {
        ty: crate::NFType::interned_UNKNOWN(),
        start: start_exp,
        step: None,
        stop: stop_exp,
    });
    Ok(iterationRange)
}

pub(crate) fn collectIteratorCrefs(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
) -> Result<metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>> {
    let mut crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)> = crefs;
    crefs = Expression::fold(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = iterator.clone();
            move |__pe_a0, __pe_a2| collectIteratorCrefs2(&__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
                    )
                        -> Result<metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>>
                    + 'static,
            >),
        crefs,
    )?;
    Ok(crefs)
}

pub(crate) fn collectIteratorCrefs2(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
) -> Result<metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>> {
    let mut crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)> = crefs;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut index: i32;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let () = (match &**exp {
        Expression::CREF { cref: __esc_cref, .. } => {
            cref = (*__esc_cref).clone();
            while ComponentRef::isCref(metamodelica::AsArg::as_arg(&cref)) {
                (cref, subs) = ComponentRef::stripSubscripts(cref.clone());
                index = 1;
                for mut sub in &*subs {
                    if Subscript::equalsIterator(metamodelica::AsArg::as_arg(&sub), iterator)? {
                        crefs = metamodelica::cons((cref.clone(), index), crefs);
                    }
                    index = index + 1;
                }
                cref = ComponentRef::rest(metamodelica::AsArg::as_arg(&cref))?;
            }
            ()
        }
        _ => (),
    });
    Ok(crefs)
}

pub(crate) fn deduceIterationRange2(
    mut range1: &(metamodelica::Ref<ComponentRef::NFComponentRef>, i32),
    mut range2: (metamodelica::Ref<ComponentRef::NFComponentRef>, i32),
    mut info: &SourceInfo,
) -> Result<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)> {
    let mut range: (metamodelica::Ref<ComponentRef::NFComponentRef>, i32) = range2.clone();
    let mut cref1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cref2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut index1: i32;
    let mut index2: i32;
    let mut node1: metamodelica::Ref<InstNode::InstNode>;
    let mut node2: metamodelica::Ref<InstNode::InstNode>;
    let mut dim1: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim2: metamodelica::Ref<Dimension::NFDimension>;
    (cref1, index1) = range1.clone();
    (cref2, index2) = range2;
    node1 = ComponentRef::node(&cref1)?;
    node2 = ComponentRef::node(&cref2)?;
    if index1 == index2 && NFInstNode::InstNode::refEqual(&node1, &node2)? {
        return Ok(range);
    }
    dim1 = Type::nthDimension(NFInstNode::InstNode::getType(node1.clone())?, index1)?;
    dim2 = Type::nthDimension(NFInstNode::InstNode::getType(node2.clone())?, index2)?;
    if !(Dimension::isEqualKnownSize(&dim1, &node1, index1, &dim2, &node2, index2)?) {
        Error::addSourceMessage(
            &(Error::INCOMPATIBLE_IMPLICIT_RANGES.clone()),
            list![
                ArcStr::from(::std::format!("{}", index1)),
                ComponentRef::toString(&cref1)?,
                ArcStr::from(::std::format!("{}", index2)),
                ComponentRef::toString(&cref2)?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok(range)
}
