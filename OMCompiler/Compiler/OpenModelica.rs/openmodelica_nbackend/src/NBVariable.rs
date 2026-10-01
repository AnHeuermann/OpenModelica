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

use crate::NBAdjacency::Mapping;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::Iterator;
use crate::NBSlice as Slice;
use crate::NBackendDAE as BackendDAE;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_nf_frontend::NFAttributes as Attributes;
use openmodelica_nf_frontend::NFBackendExtension as BackendExtension;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::OptimizerExpression;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::TearingSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltin;
use openmodelica_nf_frontend::NFCeval as Ceval;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Global;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

//OF Imports
//NF Imports
// Backend Imports
//Util Imports
// mainly used for mapping purposes
pub type VariablePointer = Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;

pub type VarSlice = metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>;

// ==========================================================================
//               Single Variable constants and functions
// ==========================================================================
thread_local! { static __DUMMY_VARIABLE_TLS: metamodelica::Ref<Variable::NFVariable> = metamodelica::Ref::new(Variable::NFVariable { name: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(), ty: openmodelica_nf_frontend::NFType::interned_ANY(), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() }); }
pub fn DUMMY_VARIABLE() -> metamodelica::Ref<Variable::NFVariable> {
    __DUMMY_VARIABLE_TLS.with(|__t| __t.clone())
}

thread_local! { static __SUBST_VARIABLE_TLS: metamodelica::Ref<Variable::NFVariable> = metamodelica::Ref::new(Variable::NFVariable { name: NFBuiltin::SUBST_CREF().clone(), ty: openmodelica_nf_frontend::NFType::interned_ANY(), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() }); }
pub(crate) fn SUBST_VARIABLE() -> metamodelica::Ref<Variable::NFVariable> {
    __SUBST_VARIABLE_TLS.with(|__t| __t.clone())
}

thread_local! { static __TIME_VARIABLE_TLS: metamodelica::Ref<Variable::NFVariable> = metamodelica::Ref::new(Variable::NFVariable { name: NFBuiltin::TIME_CREF().clone(), ty: openmodelica_nf_frontend::NFType::interned_REAL(), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: metamodelica::Ref::new(BackendInfo::BackendInfo { varKind: openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_TIME(), attributes: BackendExtension::EMPTY_VAR_ATTR_REAL().clone(), annotations: BackendExtension::EMPTY_ANNOTATIONS.clone(), var_pre: None, var_seed: None, var_pder_res: None, var_pder_tmp: None, var_start: None, parent: None }) }); }
pub(crate) fn TIME_VARIABLE() -> metamodelica::Ref<Variable::NFVariable> {
    __TIME_VARIABLE_TLS.with(|__t| __t.clone())
}

pub(crate) const DERIVATIVE_STR: &'static str = "$DER";

pub(crate) const DUMMY_DERIVATIVE_STR: &'static str = "$dDER";

pub(crate) const PARTIAL_DERIVATIVE_STR: &'static str = "$pDER";

pub(crate) const FUNCTION_DERIVATIVE_STR: &'static str = "$fDER";

pub(crate) const FUNCTION_STR: &'static str = "$FUN";

pub(crate) const PREVIOUS_STR: &'static str = "$PRE";

pub(crate) const AUXILIARY_STR: &'static str = "$AUX";

pub(crate) const STATE_ALIAS_STR: &'static str = "$STA";

pub(crate) const DUMMY_ALIAS_STR: &'static str = "$DUM";

pub(crate) const START_STR: &'static str = "$START";

pub(crate) const RESIDUAL_STR: &'static str = "$RES";

pub(crate) const TEMPORARY_STR: &'static str = "$TMP";

pub(crate) const SEED_STR: &'static str = "$SEED";

pub(crate) const TIME_EVENT_STR: &'static str = "$TEV";

pub(crate) const STATE_EVENT_STR: &'static str = "$SEV";

pub(crate) const WHEN_CONDITION_STR: &'static str = "$WC";

pub(crate) const CLOCK_STR: &'static str = "$CLK";

pub(crate) fn toString(mut var: &metamodelica::Ref<Variable::NFVariable>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut attr: ArcStr;
    attr = BackendExtension::VariableAttributes::toString(&var.backendinfo.attributes)?;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*BackendExtension::VariableKind::toString(&var.backendinfo.varKind));
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(Variable::size(var, true)?));
        __mm_s.push_str(&*literal!(") "));
        __mm_s.push_str(&*Variable::toString(var, literal!(""), false)?);
        __mm_s.push_str(&*if (metamodelica::stringEq(&attr, &(literal!("")))) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*attr);
                ArcStr::from(__mm_s)
            }
        });
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn pointerToString(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = toString(&(Pointer::access(var_ptr.clone())), literal!(""))?;
    Ok(r#str)
}

pub(crate) fn nameString(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = ComponentRef::toString(&(getVarName(var_ptr.clone())))?;
    Ok(r#str)
}

pub fn hash(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> {
    let mut i: i32 = Variable::hash(&(Pointer::access(var_ptr.clone())))?;
    Ok(i)
}

pub fn equalName(
    mut var_ptr1: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut var_ptr2: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<bool> {
    let mut b: bool = Variable::equalName(
        &(Pointer::access(var_ptr1.clone())),
        &(Pointer::access(var_ptr2.clone())),
    )?;
    Ok(b)
}

pub(crate) fn size(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut resize: bool,
) -> Result<i32> {
    let mut s: i32 = Variable::size(&(Pointer::access(var_ptr.clone())), resize)?;
    Ok(s)
}

pub(crate) fn applyToType(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>,
) -> Result<()> {
    pub type typeFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
    >;

    let mut new: metamodelica::Ref<Variable::NFVariable>;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    new = Variable::applyToType(var.clone(), func)?;
    if !(referenceEq(&*(var), &*(&*new))) {
        Pointer::update(var_ptr, new);
    }
    Ok(())
}

pub(crate) fn fromCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut attr: metamodelica::Ref<Attributes::NFAttributes>,
    mut binding: metamodelica::Ref<Binding::NFBinding>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut variable: metamodelica::Ref<Variable::NFVariable>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut class_node: metamodelica::Ref<InstNode::InstNode>;
    let mut child_nodes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut vis: Prefixes::Visibility;
    let mut info: SourceInfo;
    let mut children: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    node = ComponentRef::node(&cref)?;
    ty = ComponentRef::getSubscriptedType(&cref, true)?;
    vis = InstNode::visibility(&node);
    info = InstNode::info(&node);
    if !(Type::isExternalObject(&ty)) {
        children = (::match_deref::match_deref! { match &(Type::arrayElementType(&ty)) {
            __esc_elem_ty @ Deref @ Type::COMPLEX { .. } => {
                elem_ty = (*__esc_elem_ty).clone();
                class_node = Type::complexNode(metamodelica::AsArg::as_arg(&elem_ty))?;
                child_nodes = Class::getComponents(InstNode::getClass(class_node)?)?;
                children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut c in (child_nodes.clone()).borrow().iter() {
                let __x = fromCref(ComponentRef::prefixCref(c.clone(), InstNode::getType(c.clone())?, metamodelica::nil(), cref.clone())?, Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                children
            },
            _ => metamodelica::nil(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    variable = metamodelica::Ref::new(Variable::NFVariable {
        name: cref,
        ty: ty,
        binding: binding,
        visibility: vis,
        attributes: attr,
        typeAttributes: metamodelica::nil(),
        children: children,
        comment: SCode::noComment.clone(),
        info: info,
        backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone(),
    });
    Ok(variable)
}

pub(crate) fn makeVarPtr(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = name;
    let mut created: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    var_ptr = Pointer::create(var.clone());
    created = openmodelica_nf_frontend::Globals::nbCreatedVars.with(|__root| __root.borrow().clone());
    {
        let __v = metamodelica::cons(var_ptr.clone(), created);
        openmodelica_nf_frontend::Globals::nbCreatedVars.with(|__root| *__root.borrow_mut() = __v)
    };
    name = BackendDAE::lowerComponentReferenceInstNode(name, var_ptr.clone())?;
    assign_field!(var.name = name.clone());
    Pointer::update(var_ptr.clone(), var);
    Ok((var_ptr, name))
}

pub(crate) fn connectPartners(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut par_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<BackendInfo::BackendInfo>,
        Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> Result<metamodelica::Ref<BackendInfo::BackendInfo>>,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut par: metamodelica::Ref<Variable::NFVariable> = Pointer::access(par_ptr.clone());
    assign_field!(var.backendinfo = func(var.backendinfo.clone(), Some(par_ptr.clone()))?);
    assign_field!(par.backendinfo = func(par.backendinfo.clone(), Some(var_ptr.clone()))?);
    Pointer::update(var_ptr, var);
    Pointer::update(par_ptr, par);
    Ok(())
}

pub(crate) fn removePartner(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<BackendInfo::BackendInfo>,
        Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> Result<metamodelica::Ref<BackendInfo::BackendInfo>>,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.backendinfo = func(var.backendinfo.clone(), None)?);
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn getVar(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    var = Pointer::access(getVarPointer(cref, info)?);
    Ok(var)
}

// The following functions provide layers of protection. Whenever accessing names or pointers use these!
pub(crate) fn getVarPointer(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut info: SourceInfo,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    var = (match &**cref {
        ComponentRef::CREF { .. } if (InstNode::isVar(&(ComponentRef::node(cref)?))) => {
            PointerWeak::upgrade(InstNode::varPointer(&(ComponentRef::node(cref)?))?)?
        }
        ComponentRef::CREF { .. } if (InstNode::isName(&(ComponentRef::node(cref)?))) => {
            Pointer::create(DUMMY_VARIABLE().clone())
        }
        ComponentRef::WILD => Pointer::create(DUMMY_VARIABLE().clone()),
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBVariable.getVarPointer"));
                    __mm_s.push_str(&*literal!(" failed for "));
                    __mm_s.push_str(&*ComponentRef::toString(cref)?);
                    __mm_s.push_str(&*literal!(
                        ", because of wrong InstNode (not VAR_NODE). Show lowering errors with -d=failtrace."
                    ));
                    ArcStr::from(__mm_s)
                },
                info,
            )?;
            return Err("fail");
        }
    });
    Ok(var)
}

pub(crate) fn getVarName(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    name = var.name.clone();
    name
}

pub(crate) fn setVarName(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.name = name);
    Pointer::update(var_ptr.clone(), var);
    var_ptr
}

pub(crate) fn subIdxName(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(
        var.name = ComponentRef::rename(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::firstName(&var.name, false)?);
                __mm_s.push_str(&*literal!("_"));
                __mm_s.push_str(&*intString(Pointer::access(index)));
                ArcStr::from(__mm_s)
            },
            var.name.clone()
        )?
    );
    var_ptr = Pointer::create(var);
    Ok(var_ptr)
}

pub(crate) fn getVarKind(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> metamodelica::Ref<VariableKind::VariableKind> {
    let mut kind: metamodelica::Ref<VariableKind::VariableKind>;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    kind = BackendExtension::BackendInfo::getVarKind(&var.backendinfo);
    kind
}

pub(crate) fn toExpression(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> =
        Expression::fromCref(getVarName(var_ptr.clone()), false)?;
    Ok(exp)
}

pub type checkVar = std::sync::Arc<
    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static,
>;

pub(crate) fn isArray(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = Type::isArray(&var.ty);
    b
}

pub(crate) fn getDimensions(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    var = Pointer::access(var_ptr);
    dims = Type::arrayDims(var.ty.clone());
    dims
}

pub(crate) fn isEmpty(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = ComponentRef::isEmpty(&var.name);
    b
}

pub(crate) fn isForcedState(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::STATE { natural, .. } => !(natural.clone()),
        _ => false,
    });
    b
}

pub(crate) fn isState(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::STATE { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isStateDerivative(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::STATE_DER { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isAlgebraic(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::ALGEBRAIC => true,
        _ => false,
    });
    b
}

pub(crate) fn isStart(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::START { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isExtObj(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::EXTOBJ { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isTime(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::TIME => true,
        _ => false,
    });
    b
}

pub(crate) fn isContinuous(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut staticAsContinuous: bool,
) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::DISCRETE_STATE => false,
        BackendExtension::VariableKind::DISCRETE => false,
        BackendExtension::VariableKind::PREVIOUS => false,
        BackendExtension::VariableKind::CONSTANT => false,
        BackendExtension::VariableKind::ITERATOR => false,
        BackendExtension::VariableKind::EXTOBJ { .. } => false,
        BackendExtension::VariableKind::PARAMETER { .. } => staticAsContinuous && Type::isContinuous(var.ty.clone())?,
        BackendExtension::VariableKind::RECORD { .. } => List::all(
            &(getRecordChildren(var_ptr)?),
            &({
                let __pe_b1 = staticAsContinuous;
                move |__pe_a0| isContinuous(__pe_a0, __pe_b1.clone())
            }),
        )?,
        _ => true,
    });
    Ok(b)
}

pub(crate) fn isDiscontinuous(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut staticAsContinuous: bool,
) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isContinuous(var_ptr, staticAsContinuous)?);
    Ok(b)
}

pub(crate) fn isContinuousRecordAware(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut staticAsContinuous: bool,
) -> Result<bool> {
    '__tco: loop {
        let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
        match getParent(var_ptr.clone()) {
            Some(mut parent) => {
                (var_ptr, staticAsContinuous) = (parent, staticAsContinuous);
                continue '__tco;
            }
            _ => return Ok(isContinuous(var_ptr, staticAsContinuous)?),
        }
    }
}

pub(crate) fn isDiscreteState(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::DISCRETE_STATE => true,
        _ => false,
    });
    b
}

pub(crate) fn isDiscrete(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::DISCRETE => true,
        _ => false,
    });
    b
}

pub(crate) fn isPrevious(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::PREVIOUS => true,
        _ => false,
    });
    b
}

pub(crate) fn isRecord(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::RECORD { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isKnownRecord(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::RECORD {
            max_var: variability, ..
        } if (variability.clone() < Prefixes::Variability::DISCRETE.clone()) => true,
        _ => false,
    });
    b
}

pub(crate) fn isUnknownRecord(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::RECORD {
            min_var: variability, ..
        } if (variability.clone() > Prefixes::Variability::NON_STRUCTURAL_PARAMETER.clone()) => true,
        _ => false,
    });
    b
}

pub(crate) fn isConstRecord(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::RECORD {
            max_var: variability, ..
        } if (variability.clone() == Prefixes::Variability::CONSTANT.clone()) => true,
        _ => false,
    });
    b
}

pub(crate) fn isClock(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::CLOCK => true,
        _ => false,
    });
    b
}

pub(crate) fn isClocked(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::CLOCKED => true,
        _ => false,
    });
    b
}

pub(crate) fn isClockOrClocked(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::CLOCK => true,
        BackendExtension::VariableKind::CLOCKED => true,
        _ => false,
    });
    b
}

pub(crate) fn isIterator(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::ITERATOR => true,
        _ => false,
    });
    b
}

pub(crate) fn isPDer(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::JAC_VAR => true,
        BackendExtension::VariableKind::JAC_TMP_VAR => true,
        _ => false,
    });
    b
}

pub(crate) fn hasTearingSelect(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut compareTS: TearingSelect,
    mut func: &dyn ::std::ops::Fn(i32, i32) -> Result<bool>,
) -> Result<bool> {
    pub type compare = std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>;

    let mut b: bool = func(((getTearingSelect(varPointer.clone())?) as i32), ((compareTS) as i32))?;
    Ok(b)
}

pub type getVarPartner = std::sync::Arc<
    dyn ::std::ops::Fn(
            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        ) -> Result<(
            Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
            ArcStr,
        )> + 'static,
>;

pub(crate) fn getVarPre(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> (
    Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ArcStr,
) {
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = literal!("pre variable");
    partner = BackendExtension::BackendInfo::strengthen(var.backendinfo.var_pre.clone());
    (partner, partnerName)
}

pub(crate) fn getVarSeed(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> (
    Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ArcStr,
) {
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = literal!("seed variable");
    partner = BackendExtension::BackendInfo::strengthen(var.backendinfo.var_seed.clone());
    (partner, partnerName)
}

pub(crate) fn getVarPDer(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut isTmp: bool,
) -> (
    Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ArcStr,
) {
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    if isTmp {
        partnerName = literal!("partial derivative (temp)");
        partner = BackendExtension::BackendInfo::strengthen(var.backendinfo.var_pder_tmp.clone());
    } else {
        partnerName = literal!("partial derivative (result)");
        partner = BackendExtension::BackendInfo::strengthen(var.backendinfo.var_pder_res.clone());
    }
    (partner, partnerName)
}

pub(crate) fn getVarDer(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> (
    Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ArcStr,
) {
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = literal!("derivative");
    partner = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::STATE {
            derivative: partner_weak,
            ..
        } => BackendExtension::BackendInfo::strengthen(partner_weak.clone()),
        _ => None,
    });
    (partner, partnerName)
}

pub(crate) fn getVarState(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<(
    Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ArcStr,
)> {
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = literal!("state");
    partner = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::STATE_DER { state: p, .. } => Some(PointerWeak::upgrade(p.clone())?),
        _ => None,
    });
    Ok((partner, partnerName))
}

pub(crate) fn getVarDummyDer(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<(
    Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ArcStr,
)> {
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = literal!("dummy derivative");
    partner = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::DUMMY_STATE { dummy_der: p } => Some(PointerWeak::upgrade(p.clone())?),
        _ => None,
    });
    Ok((partner, partnerName))
}

pub(crate) fn getVarStart(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> (
    Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ArcStr,
) {
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = literal!("start");
    partner = BackendExtension::BackendInfo::strengthen(var.backendinfo.var_start.clone());
    (partner, partnerName)
}

pub(crate) fn getPartnerCref(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(
        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    ) -> Result<(
        Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        ArcStr,
    )>,
    mut scalarized: bool,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut partner_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut partner: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    (partner, partnerName) = func(getVarPointer(
        cref,
        metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"),
    )?)?;
    if (partner).is_some() {
        partner_cref = getVarName(partner.ok_or("pattern mismatch")?);
        if !(scalarized) {
            partner_cref = ComponentRef::copySubscripts(cref, partner_cref)?;
        }
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBVariable.getPartnerCref"));
                __mm_s.push_str(&*literal!(" failed because "));
                __mm_s.push_str(&*ComponentRef::toString(cref)?);
                __mm_s.push_str(&*literal!(" has no corresponding "));
                __mm_s.push_str(&*partnerName);
                __mm_s.push_str(&*literal!("."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok(partner_cref)
}

pub(crate) fn hasStartAttr(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (BackendExtension::VariableAttributes::getStartAttribute(&var.backendinfo.attributes)?).is_some();
    Ok(b)
}

pub(crate) fn hasPre(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isPrevious(var_ptr.clone())) && ((getVarPre(var_ptr)).0).is_some();
    b
}

pub(crate) fn isJacobianResultVar(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match (getVarPDer(var_ptr.clone(), false)).0 {
        Some(mut der_var) => isJacobianResultVarPDer(der_var),
        _ => {
            let mut der_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            (match (getVarPDer(var_ptr, true)).0 {
                Some(mut __esc_der_var) => {
                    der_var = __esc_der_var.clone();
                    isJacobianResultVarPDer(der_var)
                }
                _ => false,
            })
        }
    });
    b
}

pub(crate) fn isJacobianResultVarPDer(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::JAC_VAR => true,
        _ => false,
    });
    b
}

pub(crate) fn isDummyState(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::DUMMY_STATE { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isDummyDer(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::DUMMY_DER { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isParamOrConst(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::PARAMETER { .. } => true,
        BackendExtension::VariableKind::CONSTANT => true,
        BackendExtension::VariableKind::RECORD { .. } => isKnownRecord(var_ptr),
        _ => false,
    });
    b
}

pub(crate) fn isConst(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::CONSTANT => true,
        BackendExtension::VariableKind::RECORD { .. } => isConstRecord(var_ptr),
        _ => false,
    });
    b
}

pub(crate) fn isKnown(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::PARAMETER { .. } => true,
        BackendExtension::VariableKind::CONSTANT => true,
        BackendExtension::VariableKind::STATE { .. } => true,
        BackendExtension::VariableKind::RECORD { .. } => isKnownRecord(var_ptr),
        _ => false,
    });
    b
}

pub(crate) fn isOptimizable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::PARAMETER { .. }, annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizable: true, .. }, .. } => true,
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizable: true, .. }, .. } if (isInput(var_ptr.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isStateOrOptimizable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = isState(var_ptr.clone()) || isOptimizable(var_ptr);
    b
}

pub(crate) fn isInitialTime(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::INITIAL_TIME.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isFinalTime(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::FINAL_TIME.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isLagrange(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::LAGRANGE.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isMayer(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::MAYER.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isPathConstraint(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::PATH_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isFinalConstraint(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::FINAL_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isInitialConstraint(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::INITIAL_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isLfgFunction(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    if isStateDerivative(var_ptr) {
        b = true;
        return b;
    }
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::LAGRANGE.clone() || optExp.clone() == OptimizerExpression::PATH_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isMrfFunction(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::MAYER.clone() || optExp.clone() == OptimizerExpression::FINAL_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isLfgVariable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isFinalTime(var_ptr.clone()) || isInitialTime(var_ptr));
    b
}

pub(crate) fn isMrfVariable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isInitialTime(var_ptr));
    b
}

pub(crate) fn isR0Variable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isFinalTime(var_ptr));
    b
}

pub(crate) fn isResizable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = List::any(&(Type::arrayDims(var.ty.clone())), &move |__a0: metamodelica::Ref<
        Dimension::NFDimension,
    >|
          -> metamodelica::Result<
        _,
    > {
        ::std::result::Result::Ok(Dimension::isResizable(&__a0))
    })?;
    Ok(b)
}

pub(crate) fn isResizableParameter(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::PARAMETER { .. }, annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { resizable: true, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn updateResizableParameter(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut val: Option<metamodelica::Ref<Expression::NFExpression>> =
        UnorderedMap::get(var.name.clone(), optimal_values.clone())?;
    let () = (::match_deref::match_deref! { match &((val, var.backendinfo.clone())) {
        (Some(Deref @ Expression::INTEGER { value: i }), Deref @ BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::PARAMETER { .. }, annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { resizable: true, .. }, .. }) => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::PARAMETER; resize_value = Some(i.clone()));
            setVarKind(var_ptr, varKind.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn getResizableValue(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> {
    let mut val: i32;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    val = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::PARAMETER { resize_value: Some(__esc_val) }, .. } => {
            val = (*__esc_val).clone();
            val.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getResizableValue")); __mm_s.push_str(&*literal!(" failed because following variable is not a resizable parameter: ")); __mm_s.push_str(&*toString(&var, literal!(""))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(val)
}

pub(crate) fn isResidual(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::RESIDUAL_VAR => true,
        _ => false,
    });
    b
}

pub(crate) fn isSeed(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::SEED_VAR => true,
        _ => false,
    });
    b
}

pub(crate) fn isInput(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = var.attributes.direction.clone() == Prefixes::Direction::INPUT.clone();
    b
}

pub(crate) fn isOutput(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = var.attributes.direction.clone() == Prefixes::Direction::OUTPUT.clone();
    b
}

pub(crate) fn isFixed(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.attributes.clone()) {
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_REAL { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&fixed))?)?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_INT { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&fixed))?)?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&fixed))?)?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_STRING { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&fixed))?)?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&fixed))?)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isFixable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::STATE { .. } => !(isFixed(var_ptr)?),
        BackendExtension::VariableKind::DISCRETE_STATE => !(isFixed(var_ptr.clone())?) || hasPre(var_ptr),
        BackendExtension::VariableKind::PARAMETER { .. } => !(isFixed(var_ptr)?),
        BackendExtension::VariableKind::PREVIOUS => true,
        _ => false,
    });
    Ok(b)
}

pub(crate) fn isStateSelect(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut stateSelect: StateSelect,
) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = BackendExtension::VariableAttributes::getStateSelect(&var.backendinfo.attributes) == stateSelect;
    b
}

pub(crate) fn setVariableAttributes(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut variableAttributes: metamodelica::Ref<VariableAttributes::VariableAttributes>,
) -> metamodelica::Ref<Variable::NFVariable> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut backendinfo: metamodelica::Ref<BackendInfo::BackendInfo> = var.backendinfo.clone();
    assign_field!(backendinfo.attributes = variableAttributes);
    assign_field!(var.backendinfo = backendinfo);
    var
}

pub(crate) fn setMin(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut min_val: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut overwrite: bool,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setMin(variableAttributes.clone(), min_val, overwrite)?);
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn setMax(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut max_val: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut overwrite: bool,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setMax(variableAttributes.clone(), max_val, overwrite)?);
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn setStartAttribute(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut start_val: metamodelica::Ref<Expression::NFExpression>,
    mut overwrite: bool,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setStartAttribute(variableAttributes.clone(), start_val, overwrite)?);
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn setStateSelect(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut stateSelect_val: StateSelect,
    mut overwrite: bool,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setStateSelect(variableAttributes.clone(), stateSelect_val, overwrite));
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn setTearingSelect(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut tearingSelect_val: TearingSelect,
    mut overwrite: bool,
) -> metamodelica::Ref<Variable::NFVariable> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setTearingSelect(variableAttributes.clone(), tearingSelect_val, overwrite));
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => {
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    var
}

pub(crate) fn getTearingSelect(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<TearingSelect> {
    let mut tearingSelect_val: TearingSelect;
    tearingSelect_val = (::match_deref::match_deref! { match &(Pointer::access(varPointer.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            BackendExtension::VariableAttributes::getTearingSelect(metamodelica::AsArg::as_arg(&variableAttributes))
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getTearingSelect")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*pointerToString(varPointer)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tearingSelect_val)
}

pub(crate) fn setVarKind(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut varKind: metamodelica::Ref<VariableKind::VariableKind>,
) -> () {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    var = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind));
    Pointer::update(varPointer, var);
    ()
}

pub(crate) fn setParent(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut parent: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> {
    let mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = varPointer;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setParent(var.backendinfo.clone(), parent));
    Pointer::update(varPointer.clone(), var);
    varPointer
}

pub(crate) fn getParent(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut parent: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(varPointer.clone());
    parent = BackendExtension::BackendInfo::strengthen(var.backendinfo.parent.clone());
    parent
}

pub(crate) fn isDummyVariable(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.backendinfo.varKind.clone() {
        BackendExtension::VariableKind::FRONTEND_DUMMY => true,
        _ => false,
    });
    b
}

pub(crate) fn isArtificial(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith(ComponentRef::firstName(&(getVarName(var_ptr)), false)?, literal!("$"));
    Ok(b)
}

pub(crate) fn isFunctionAlias(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith(
        ComponentRef::firstName(&(getVarName(var_ptr)), false)?,
        arcstr::literal!(FUNCTION_STR),
    );
    Ok(b)
}

pub(crate) fn isClockAlias(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith(
        ComponentRef::firstName(&(getVarName(var_ptr)), false)?,
        arcstr::literal!(CLOCK_STR),
    );
    Ok(b)
}

pub(crate) fn createTimeVar() -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var: metamodelica::Ref<Variable::NFVariable> = TIME_VARIABLE().clone();
    (var_ptr, _) = makeVarPtr(var.clone(), var.name.clone())?;
    Ok(var_ptr)
}

pub(crate) fn setStateDerivativeVar(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut derivative: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> () {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    var = Pointer::access(varPointer.clone());
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setVarKind(
            var.backendinfo.clone(),
            metamodelica::Ref::new(VariableKind::VariableKind::STATE {
                index: 1,
                derivative: BackendExtension::BackendInfo::weaken(Some(derivative)),
                natural: true
            })
        )
    );
    Pointer::update(varPointer, var);
    ()
}

pub(crate) fn setStateDerKind(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut statePointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> () {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    var = Pointer::access(varPointer.clone());
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setVarKind(
            var.backendinfo.clone(),
            metamodelica::Ref::new(VariableKind::VariableKind::STATE_DER {
                state: PointerWeak::downgrade(statePointer),
                alias: None
            })
        )
    );
    Pointer::update(varPointer, var);
    ()
}

pub(crate) fn makeAlgStateVar(mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> () {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    if isAlgebraic(varPointer.clone()) {
        var = Pointer::access(varPointer.clone());
        assign_field!(
            var.backendinfo = BackendExtension::BackendInfo::setVarKind(
                var.backendinfo.clone(),
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_ALG_STATE()
            )
        );
        Pointer::update(varPointer, var);
    }
    ()
}

pub(crate) fn makeDerVar(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut scalarized: bool,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut der_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut state_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = if (scalarized) {
        cref.clone()
    } else {
        ComponentRef::stripSubscriptsAll(&cref)
    };
    let () = ({
        let mut dummy_ptr: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>> =
            PointerWeak::downgrade(Pointer::createImmutable(DUMMY_VARIABLE().clone()));
        (match &*(ComponentRef::node(&state_cref)?) {
            InstNode::VAR_NODE { .. } => {
                let mut derNode: metamodelica::Ref<InstNode::InstNode>;
                let mut state: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut var: metamodelica::Ref<Variable::NFVariable>;
                state = getVarPointer(&state_cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
                derNode = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
                    name: arcstr::literal!(DERIVATIVE_STR),
                    varPointer: dummy_ptr,
                });
                der_cref = ComponentRef::append(
                    state_cref.clone(),
                    &(ComponentRef::fromNode(
                        derNode,
                        ComponentRef::scalarType(&state_cref)?,
                        metamodelica::nil(),
                        ComponentRef::Origin::CREF.clone(),
                    )?),
                )?;
                var = fromCref(
                    ComponentRef::stripSubscriptsAll(&der_cref),
                    Variable::attributes(&(Pointer::access(state.clone()))),
                    Binding::EMPTY_BINDING().clone(),
                )?;
                assign_field!(
                    var.backendinfo = BackendExtension::BackendInfo::setVarKind(
                        var.backendinfo.clone(),
                        metamodelica::Ref::new(VariableKind::VariableKind::STATE_DER {
                            state: PointerWeak::downgrade(state),
                            alias: None
                        })
                    )
                );
                (var_ptr, der_cref) = makeVarPtr(var, der_cref)?;
                if !(scalarized) {
                    der_cref = ComponentRef::copySubscripts(&cref, der_cref)?;
                }
                ()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBVariable.makeDerVar"));
                        __mm_s.push_str(&*literal!(" failed for "));
                        __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    });
    Ok((der_cref, var_ptr))
}

pub(crate) fn hasDerVar(mut state_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(Pointer::access(state_var)) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::STATE { derivative: Some(_), .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn addRecordChild(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut child: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::cons(PointerWeak::downgrade(child), var_field!((*varKind).children, VariableKind::VariableKind::RECORD).clone()));
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.addRecordChild")); __mm_s.push_str(&*literal!(" failed adding ")); __mm_s.push_str(&*ComponentRef::toString(&(getVarName(child)))?); __mm_s.push_str(&*literal!(" as a child to ")); __mm_s.push_str(&*ComponentRef::toString(&(getVarName(var_ptr.clone())))?); __mm_s.push_str(&*literal!(" because it is not a record.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn setRecordChildren(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = ({
        let mut __acc: metamodelica::List<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut c in (children).into_iter().cloned() {
            let __x = PointerWeak::downgrade(c.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setRecordChildren")); __mm_s.push_str(&*literal!(" failed adding new children to ")); __mm_s.push_str(&*ComponentRef::toString(&(getVarName(var_ptr.clone())))?); __mm_s.push_str(&*literal!(" because it is not a record.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn getRecordChildren(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    children = (::match_deref::match_deref! { match &(Pointer::access(var)) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut c in (var_field!((**varKind).children, VariableKind::VariableKind::RECORD).clone()).into_iter().cloned() {
            let __x = PointerWeak::upgrade(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(children)
}

pub(crate) fn getRecordChildrenCells(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> metamodelica::List<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>> {
    let mut children: metamodelica::List<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>;
    children = (::match_deref::match_deref! { match &(Pointer::access(var)) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            var_field!((**varKind).children, VariableKind::VariableKind::RECORD).clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    children
}

pub(crate) fn getRecordChildrenOrSelf(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        getRecordChildren(var.clone())?;
    children = if ((children).is_empty()) { list![var] } else { children };
    Ok(children)
}

pub(crate) fn getRecordChildrenCref(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut children: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut arg_children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    subscripts = ComponentRef::subscriptsAllFlat(cref)?;
    arg_children = getRecordChildren(getVarPointer(
        cref,
        metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"),
    )?)?;
    children = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut child in (arg_children).into_iter().cloned() {
            let __x = ComponentRef::mergeSubscripts(subscripts.clone(), getVarName(child.clone()), true, true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(children)
}

pub(crate) fn getRecordChildrenCrefOrSelf(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut children: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        getRecordChildrenCref(&cref)?;
    children = if ((children).is_empty()) { list![cref] } else { children };
    Ok(children)
}

pub(crate) fn setRecordVariability(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut variability: Prefixes::Variability,
) -> () {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let _ = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD;
                min_var = variability,
                max_var = variability
            );
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
            Pointer::update(var_ptr, var);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub(crate) fn makeDummyState(
    mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut derivative: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(varPointer.clone());
    assign_field!(
        var.backendinfo = (match &*(BackendExtension::BackendInfo::getVarKind(&var.backendinfo)) {
            BackendExtension::VariableKind::STATE {
                derivative: Some(derivative_weak),
                ..
            } => {
                let mut der_var: metamodelica::Ref<Variable::NFVariable>;
                derivative = PointerWeak::upgrade(derivative_weak.clone())?;
                der_var = Pointer::access(derivative.clone());
                assign_field!(
                    der_var.backendinfo = BackendExtension::BackendInfo::setVarKind(
                        der_var.backendinfo.clone(),
                        metamodelica::Ref::new(VariableKind::VariableKind::DUMMY_DER {
                            dummy_state: PointerWeak::downgrade(varPointer.clone())
                        })
                    )
                );
                assign_field!(
                    der_var.backendinfo = BackendExtension::BackendInfo::setStateSelect(
                        der_var.backendinfo.clone(),
                        BackendExtension::StateSelect::AVOID.clone(),
                        false
                    )
                );
                Pointer::update(derivative.clone(), der_var);
                BackendExtension::BackendInfo::setVarKind(
                    var.backendinfo.clone(),
                    metamodelica::Ref::new(VariableKind::VariableKind::DUMMY_STATE {
                        dummy_der: PointerWeak::downgrade(derivative.clone()),
                    }),
                )
            }
            BackendExtension::VariableKind::DUMMY_STATE {
                dummy_der: derivative_weak,
            } => {
                derivative = PointerWeak::upgrade(derivative_weak.clone())?;
                var.backendinfo.clone()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBVariable.makeDummyState"));
                        __mm_s.push_str(&*literal!(" failed for "));
                        __mm_s.push_str(&*ComponentRef::toString(&(getVarName(varPointer.clone())))?);
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    );
    Pointer::update(varPointer, var);
    Ok(derivative)
}

pub(crate) fn makeDiscreteStateVar(mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> () {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(varPointer.clone());
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setVarKind(
            var.backendinfo.clone(),
            openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE_STATE()
        )
    );
    Pointer::update(varPointer, var);
    ()
}

pub(crate) fn makePreVar(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut pre_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    let mut pre_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(&cref)?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut pre: metamodelica::Ref<Variable::NFVariable>;
            let mut qual = (*qual).clone();
            var_ptr = getVarPointer(&cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(PREVIOUS_STR));
            pre_cref = ComponentRef::append(cref.clone(), &(ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(&cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?))?;
            pre = fromCref(pre_cref.clone(), Variable::attributes(&(Pointer::access(var_ptr.clone()))), Binding::EMPTY_BINDING().clone())?;
            assign_field!(pre.backendinfo = BackendExtension::BackendInfo::setVarKind(pre.backendinfo.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_PREVIOUS()));
            (pre_ptr, pre_cref) = makeVarPtr(pre, pre_cref)?;
            connectPartners(var_ptr, pre_ptr.clone(), &fnptr!(BackendExtension::BackendInfo::setVarPre, metamodelica::Ref<BackendInfo::BackendInfo>, Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>))?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makePreVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(&cref)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((pre_cref, pre_ptr))
}

pub(crate) fn makeSeedVar(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut name: &ArcStr,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(&cref)?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut ovar: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            let mut var: metamodelica::Ref<Variable::NFVariable>;
            let mut varKind: metamodelica::Ref<VariableKind::VariableKind>;
            let mut original_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut qual = (*qual).clone();
            original_cref = cref.clone();
            old_var_ptr = getVarPointer(&cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            if ComponentRef::hasSubscripts(&original_cref)? {
                ovar = None;
            } else {
                (ovar, _) = getVarSeed(old_var_ptr.clone());
                if (ovar).is_some() && !metamodelica::stringEq(&(ComponentRef::firstName(&(ComponentRef::last(&(getVarName(ovar.clone().ok_or("pattern mismatch")?)))), false)?), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(SEED_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) })) {
                    ovar = None;
                }
            }
            if (ovar).is_some() {
                var_ptr = ovar.ok_or("pattern mismatch")?;
                cref = getVarName(var_ptr.clone());
            } else {
                assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(SEED_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) });
                cref = ComponentRef::append(cref.clone(), &(ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(&cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?))?;
                var = fromCref(cref.clone(), Attributes::IMPL_DISCRETE_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
                varKind = (::match_deref::match_deref! { match &(getVarKind(old_var_ptr.clone())) {
        __esc_varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. } => {
            varKind = (*__esc_varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::nil());
            varKind.clone()
        },
        _ => openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_SEED_VAR(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind));
                (var_ptr, cref) = makeVarPtr(var, cref)?;
                if !(ComponentRef::hasSubscripts(&original_cref)?) {
                    connectPartners(old_var_ptr, var_ptr.clone(), &fnptr!(BackendExtension::BackendInfo::setVarSeed, metamodelica::Ref<BackendInfo::BackendInfo>, Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>))?;
                }
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeSeedVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(&cref)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, var_ptr))
}

pub(crate) fn makePDerVar(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut name: &ArcStr,
    mut isTmp: bool,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(&cref)?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut res_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut ovar: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            let mut varKind: metamodelica::Ref<VariableKind::VariableKind>;
            let mut var: metamodelica::Ref<Variable::NFVariable>;
            let mut qual = (*qual).clone();
            res_ptr = getVarPointer(&cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            (ovar, _) = getVarPDer(res_ptr.clone(), isTmp);
            if (ovar).is_some() && !metamodelica::stringEq(&(ComponentRef::firstName(&(ComponentRef::last(&(getVarName(ovar.clone().ok_or("pattern mismatch")?)))), false)?), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(PARTIAL_DERIVATIVE_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) })) {
                ovar = None;
            }
            if (ovar).is_some() {
                var_ptr = ovar.ok_or("pattern mismatch")?;
                cref = getVarName(var_ptr.clone());
            } else {
                assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(PARTIAL_DERIVATIVE_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) });
                cref = ComponentRef::append(cref.clone(), &(ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(&cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?))?;
                var = fromCref(cref.clone(), Variable::attributes(&(Pointer::access(res_ptr.clone()))), Binding::EMPTY_BINDING().clone())?;
                varKind = (::match_deref::match_deref! { match &(getVarKind(res_ptr.clone())) {
        __esc_varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. } => {
            varKind = (*__esc_varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::nil());
            varKind.clone()
        },
        _ => if (isTmp) {openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_JAC_TMP_VAR()} else {openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_JAC_VAR()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind));
                (var_ptr, cref) = makeVarPtr(var, cref)?;
                connectPartners(res_ptr, var_ptr.clone(), &({ let __pe_b2 = isTmp; move |__pe_a0, __pe_a1| Ok(BackendExtension::BackendInfo::setVarPDer(__pe_a0, __pe_a1, __pe_b2.clone())) }))?;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makePDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(&cref)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, var_ptr))
}

pub(crate) fn makeFDerVar(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: Deref @ ComponentRef::EMPTY, .. } => (::match_deref::match_deref! { match &(ComponentRef::node(&cref)?) {
        qual @ Deref @ InstNode::COMPONENT_NODE { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => InstNode::InstNode::COMPONENT_NODE; name = BackendUtil::makeFDerString(ComponentRef::toString(&cref)?, None)?);
            ComponentRef::fromOwnedNode(InstNode::reidentify(qual.clone()), ComponentRef::nodeType(&cref)?)
        },
        qual @ Deref @ InstNode::CLASS_NODE { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => InstNode::InstNode::CLASS_NODE; name = BackendUtil::makeFDerString(ComponentRef::toString(&cref)?, None)?);
            ComponentRef::fromOwnedNode(InstNode::reidentify(qual.clone()), ComponentRef::nodeType(&cref)?)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeFDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(&cref)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ ComponentRef::CREF { restCref: __cref_restCref, .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = makeFDerVar(__cref_restCref.clone())?);
            cref
        },
        _ => cref,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub(crate) fn makeStartVar(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut start_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    (start_cref, var_ptr) = (::match_deref::match_deref! { match &(ComponentRef::node(cref)?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut var: metamodelica::Ref<Variable::NFVariable>;
            let mut old_var: metamodelica::Ref<Variable::NFVariable>;
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            (start_cref, var_ptr) = (match (getVarStart(old_var_ptr.clone())).0 {
        Some(mut __esc_var_ptr) => {
            var_ptr = __esc_var_ptr.clone();
            (getVarName(var_ptr.clone()), var_ptr)
        },
        _ => {
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(START_STR));
            start_cref = ComponentRef::append(ComponentRef::stripSubscriptsAll(cref), &(ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?))?;
            var = fromCref(start_cref.clone(), Variable::attributes(&(getVar(cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?)), Binding::EMPTY_BINDING().clone())?;
            if isRecord(old_var_ptr.clone()) {
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), metamodelica::Ref::new(VariableKind::VariableKind::RECORD { children: metamodelica::nil(), min_var: Prefixes::Variability::PARAMETER.clone(), max_var: Prefixes::Variability::CONTINUOUS.clone() })));
            } else {
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), metamodelica::Ref::new(VariableKind::VariableKind::START { original: PointerWeak::downgrade(old_var_ptr.clone()) })));
            }
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarStart(var.backendinfo.clone(), Some(old_var_ptr.clone())));
            (var_ptr, start_cref) = makeVarPtr(var, start_cref)?;
            old_var = Pointer::access(old_var_ptr.clone());
            assign_field!(old_var.backendinfo = BackendExtension::BackendInfo::setVarStart(old_var.backendinfo.clone(), Some(var_ptr.clone())));
            Pointer::update(old_var_ptr, old_var);
            (start_cref, var_ptr)
        },
    });
            start_cref = ComponentRef::copySubscripts(cref, start_cref)?;
            (start_cref, var_ptr)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeStartVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start_cref, var_ptr))
}

pub(crate) fn makeResidualVar(
    mut name: &ArcStr,
    mut uniqueIndex: i32,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    node = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*arcstr::literal!(RESIDUAL_STR));
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(uniqueIndex));
            ArcStr::from(__mm_s)
        },
        varPointer: PointerWeak::downgrade(Pointer::createImmutable(DUMMY_VARIABLE().clone())),
    });
    cref = ComponentRef::fromNode(node, ty, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?;
    var = fromCref(
        cref.clone(),
        Attributes::DEFAULT_ATTR().clone(),
        Binding::EMPTY_BINDING().clone(),
    )?;
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setVarKind(
            var.backendinfo.clone(),
            openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_RESIDUAL_VAR()
        )
    );
    (var_ptr, cref) = makeVarPtr(var, cref)?;
    Ok((var_ptr, cref))
}

pub(crate) fn makeEventVar(
    mut name: &ArcStr,
    mut uniqueIndex: i32,
    mut var_ty: metamodelica::Ref<Type::NFType>,
    mut iterator: &metamodelica::Ref<Iterator::Iterator>,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut iter_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    iter_subs = BEquation::Iterator::normalizedSubscripts(
        iterator,
        UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                      __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        ),
    )?;
    if (iter_subs).is_empty() {
        ty = var_ty;
    } else {
        ty = Type::liftArrayLeftList(var_ty, &(BEquation::Iterator::dimensions(iterator)?));
    }
    node = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(uniqueIndex));
            ArcStr::from(__mm_s)
        },
        varPointer: PointerWeak::downgrade(Pointer::createImmutable(DUMMY_VARIABLE().clone())),
    });
    cref = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF {
        node: ComponentRef::storeNode(node.clone(), false)?,
        subscripts: iter_subs,
        ty: ty.clone(),
        origin: ComponentRef::Origin::CREF.clone(),
        restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
    });
    var_cref = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF {
        node: ComponentRef::storeNode(node, false)?,
        subscripts: metamodelica::nil(),
        ty: ty,
        origin: ComponentRef::Origin::CREF.clone(),
        restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
    });
    var = fromCref(
        var_cref,
        Attributes::IMPL_DISCRETE_ATTR().clone(),
        Binding::EMPTY_BINDING().clone(),
    )?;
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setVarKind(
            var.backendinfo.clone(),
            openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE()
        )
    );
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setHideResult(var.backendinfo.clone(), true));
    (var_ptr, cref) = makeVarPtr(var, cref)?;
    Ok((var_ptr, cref))
}

pub(crate) fn makeAuxVar(
    mut name: &ArcStr,
    mut uniqueIndex: i32,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut makeParam: bool,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    fn updateBackendInfo(
        mut var: metamodelica::Ref<Variable::NFVariable>,
        mut makeParam: bool,
    ) -> Result<metamodelica::Ref<Variable::NFVariable>> {
        let mut var: metamodelica::Ref<Variable::NFVariable> = var;
        assign_field!(
            var.backendinfo = BackendExtension::BackendInfo::setVarKind(
                var.backendinfo.clone(),
                BackendExtension::VariableKind::fromType(Variable::typeOf(&var), makeParam)?
            )
        );
        assign_field!(var.backendinfo = BackendExtension::BackendInfo::setHideResult(var.backendinfo.clone(), true));
        Ok(var)
    }

    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    node = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(uniqueIndex));
            ArcStr::from(__mm_s)
        },
        varPointer: PointerWeak::downgrade(Pointer::createImmutable(DUMMY_VARIABLE().clone())),
    });
    cref = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF {
        node: ComponentRef::storeNode(node, false)?,
        subscripts: metamodelica::nil(),
        ty: ty,
        origin: ComponentRef::Origin::CREF.clone(),
        restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
    });
    var = fromCref(
        cref.clone(),
        Attributes::DEFAULT_ATTR().clone(),
        Binding::EMPTY_BINDING().clone(),
    )?;
    var = updateBackendInfo(var, makeParam)?;
    assign_field!(
        var.children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut child in (var.children.clone()).into_iter().cloned() {
                let __x = updateBackendInfo(child.clone(), makeParam)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    (var_ptr, cref) = makeVarPtr(var, cref)?;
    Ok((var_ptr, cref))
}

pub(crate) fn makeAuxStateVar(
    mut uniqueIndex: i32,
    mut binding: Option<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut der_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut der_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut bnd: metamodelica::Ref<Expression::NFExpression>;
    node = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*arcstr::literal!(AUXILIARY_STR));
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(uniqueIndex));
            ArcStr::from(__mm_s)
        },
        varPointer: PointerWeak::downgrade(Pointer::createImmutable(DUMMY_VARIABLE().clone())),
    });
    cref = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF {
        node: ComponentRef::storeNode(node, false)?,
        subscripts: metamodelica::nil(),
        ty: openmodelica_nf_frontend::NFType::interned_REAL(),
        origin: ComponentRef::Origin::CREF.clone(),
        restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
    });
    if (binding).is_some() {
        bnd = binding.ok_or("pattern mismatch")?;
        var = fromCref(
            cref.clone(),
            Attributes::DEFAULT_ATTR().clone(),
            Binding::makeFlat(
                bnd.clone(),
                Expression::variability(bnd)?,
                Binding::Source::BINDING.clone(),
                Binding::NO_CONFIDENCE.clone(),
            ),
        )?;
    } else {
        var = fromCref(
            cref.clone(),
            Attributes::DEFAULT_ATTR().clone(),
            Binding::EMPTY_BINDING().clone(),
        )?;
    }
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setStateSelect(
            var.backendinfo.clone(),
            BackendExtension::StateSelect::AVOID.clone(),
            false
        )
    );
    (var_ptr, cref) = makeVarPtr(var, cref)?;
    (der_cref, der_var) = makeDerVar(cref.clone(), false)?;
    setStateDerivativeVar(var_ptr.clone(), der_var.clone());
    Ok((var_ptr, cref, der_var, der_cref))
}

pub(crate) fn makeTmpVar(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut tmp_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(&cref)?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut var: metamodelica::Ref<Variable::NFVariable>;
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(&cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(TEMPORARY_STR));
            tmp_cref = ComponentRef::append(cref.clone(), &(ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(&cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?))?;
            var = fromCref(tmp_cref.clone(), Variable::attributes(&(getVar(&cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?)), Binding::EMPTY_BINDING().clone())?;
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), getVarKind(old_var_ptr)));
            (var_ptr, tmp_cref) = makeVarPtr(var, tmp_cref)?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeTmpVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(&cref)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tmp_cref)
}

pub(crate) fn makeClockVar(
    mut uniqueIndex: i32,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    node = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*arcstr::literal!(CLOCK_STR));
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(uniqueIndex));
            ArcStr::from(__mm_s)
        },
        varPointer: PointerWeak::downgrade(Pointer::createImmutable(DUMMY_VARIABLE().clone())),
    });
    cref = metamodelica::Ref::new(ComponentRef::NFComponentRef::CREF {
        node: ComponentRef::storeNode(node, false)?,
        subscripts: metamodelica::nil(),
        ty: ty,
        origin: ComponentRef::Origin::CREF.clone(),
        restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
    });
    var = fromCref(
        cref.clone(),
        Attributes::DEFAULT_ATTR().clone(),
        Binding::EMPTY_BINDING().clone(),
    )?;
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setVarKind(
            var.backendinfo.clone(),
            openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCK()
        )
    );
    (var_ptr, cref) = makeVarPtr(var, cref)?;
    Ok((var_ptr, cref))
}

pub(crate) fn getBindingVariability(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<Prefixes::Variability> {
    let mut variability: Prefixes::Variability;
    variability = (::match_deref::match_deref! { match &(Pointer::access(var_ptr)) {
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::TYPED_BINDING { variability: tmp, .. }, .. } => {
            tmp.clone()
        },
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::FLAT_BINDING { variability: tmp, .. }, .. } => {
            tmp.clone()
        },
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::UNBOUND, .. } => {
            Prefixes::Variability::CONTINUOUS.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getBindingVariability")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(variability)
}

pub(crate) fn hasEvaluableBinding(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<bool> {
    fn isEvaluable(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
        let mut b: bool;
        let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
        b = Expression::isLiteralXML(exp.clone())?;
        if !(b) {
            (_, new_exp) = BEquation::Iterator::extract(
                exp,
                UnorderedSet::new(
                    (std::sync::Arc::new(hash)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                                + 'static,
                        >),
                    (std::sync::Arc::new(equalName)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                    13,
                ),
                UnorderedMap::new(
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>| {
                            Dimension::hashList(&__a0)
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
                                ) -> Result<i32>
                                + 'static,
                        >),
                    (std::sync::Arc::new({
                        let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(
                            move |__a0: metamodelica::Ref<Dimension::NFDimension>,
                                  __a1: metamodelica::Ref<Dimension::NFDimension>| {
                                Dimension::isEqual(&__a0, &__a1)
                            },
                        )
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Dimension::NFDimension>,
                                        metamodelica::Ref<Dimension::NFDimension>,
                                    ) -> Result<bool>
                                    + 'static,
                            >);
                        move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, &*__pe_b2)
                    }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>),
                    1,
                ),
            )?;
            new_exp = SimplifyExp::simplifyDump(
                new_exp,
                true,
                &(literal!("NBVariable.hasEvaluableBinding.isEvaluable")),
                &(literal!("")),
            )?;
            b = Expression::isLiteralXML(Ceval::tryEvalExp(new_exp, &(Ceval::noTarget().clone())))?;
        }
        Ok(b)
    }

    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut binding: metamodelica::Ref<Expression::NFExpression>;
    if isBound(var_ptr) {
        binding = Binding::getExp(&var.binding)?;
        b = isEvaluable(binding)?;
    } else {
        b = false;
    }
    Ok(b)
}

pub(crate) fn mapExp(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut funcExp: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
    mut mapFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut opt_start: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut binding: metamodelica::Ref<Expression::NFExpression>;
    let mut new_binding: metamodelica::Ref<Expression::NFExpression>;
    let mut start: metamodelica::Ref<Expression::NFExpression>;
    let mut new_start: metamodelica::Ref<Expression::NFExpression>;
    let mut changed: bool = false;
    if isBound(var_ptr.clone()) {
        binding = Binding::getExp(&var.binding)?;
        new_binding = mapFunc(binding.clone(), funcExp.clone())?;
        if !(referenceEq(&*(binding), &*(&*new_binding))) {
            assign_field!(var.binding = Binding::setExp(new_binding, var.binding.clone())?);
            changed = true;
        }
    }
    opt_start = getStartAttribute(var_ptr.clone())?;
    if (opt_start).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(opt_start) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        start = metamodelica::Own::own(__pa0);
        new_start = mapFunc(start.clone(), funcExp.clone())?;
        if !(referenceEq(&*(start), &*(&*new_start))) {
            var = setStartAttribute(var, new_start, true)?;
            changed = true;
        }
    }
    if changed {
        Pointer::update(var_ptr, var);
    }
    Ok(())
}

pub(crate) fn setFixed(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut b: bool,
    mut overwrite: bool,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    var = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendInfo::BACKEND_INFO { .. }, .. } => {
            let mut binfo = (*binfo).clone();
            assign_field!(binfo.attributes = BackendExtension::VariableAttributes::setFixed(binfo.attributes.clone(), var.ty.clone(), b, overwrite)?);
            assign_field!(var.backendinfo = binfo.clone());
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setFixed")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr.clone(), var);
    Ok(var_ptr)
}

pub(crate) fn setBindingAsStart(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut overwrite: bool,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    var = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendInfo::BACKEND_INFO { .. }, .. } => {
            let mut start: metamodelica::Ref<Expression::NFExpression>;
            let mut binfo = (*binfo).clone();
            start = Binding::getExp(&var.binding)?;
            assign_field!(binfo.attributes = BackendExtension::VariableAttributes::setStartAttribute(binfo.attributes.clone(), start, overwrite)?);
            assign_field!(var.backendinfo = binfo.clone());
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setBindingAsStart")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn setBindingAsStartAndFix(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut b: bool,
    mut overwrite: bool,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    setBindingAsStart(var_ptr.clone(), overwrite)?;
    var_ptr = setFixed(var_ptr, b, false)?;
    Ok(var_ptr)
}

pub(crate) fn getStartAttribute(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut start: Option<metamodelica::Ref<Expression::NFExpression>> =
        BackendExtension::VariableAttributes::getStartAttribute(
            &(Variable::getVariableAttributes(&(Pointer::access(var_ptr.clone())))),
        )?;
    Ok(start)
}

pub(crate) fn hasNonTrivialAliasBinding(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut binding: metamodelica::Ref<Expression::NFExpression> = Binding::getExp(&var.binding)?;
    b = !(Expression::isTrivialCref(&binding))
        && checkExpMap(
            binding,
            (std::sync::Arc::new(fnptr!(
                isTimeDependent,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >),
            &(metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo")),
        )?;
    Ok(b)
}

pub(crate) fn hasConstOrParamAliasBinding(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<bool> {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(checkExpMap(
        Binding::getExp(&var.binding)?,
        (std::sync::Arc::new(fnptr!(
            isTimeDependent,
            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static,
            >),
        &(metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo")),
    )?);
    Ok(b)
}

pub(crate) fn isTimeDependent(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = BackendExtension::VariableKind::isTimeDependent(&var.backendinfo.varKind);
    b
}

pub(crate) fn isBound(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match &*var.binding.clone() {
        Binding::TYPED_BINDING { .. } => true,
        Binding::UNTYPED_BINDING { .. } => true,
        Binding::FLAT_BINDING { .. } => true,
        _ => false,
    });
    b
}

// ==========================================================================
//                        Other type wrappers
//
// ==========================================================================
pub(crate) fn checkExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
    mut info: SourceInfo,
) -> Result<bool> {
    let mut b: bool;
    b = (match &**exp {
        Expression::CREF { cref, .. } => func(getVarPointer(cref, info)?)?,
        _ => false,
    });
    Ok(b)
}

pub(crate) fn checkExpMap(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static,
    >,
    mut info: &SourceInfo,
) -> Result<bool> {
    pub(crate) fn checkExpTraverse(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
        mut info: SourceInfo,
        mut b: bool,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut b: bool = b;
        if !(b) {
            b = checkExp(&exp, func, info)?;
        }
        Ok((exp, b))
    }

    let mut b: bool;
    (_, b) = Expression::mapFold(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = func.clone();
            let __pe_b2 = info.clone();
            move |__pe_a0, __pe_a3| checkExpTraverse(__pe_a0, &*__pe_b1, __pe_b2.clone(), __pe_a3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        bool,
                    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)>
                    + 'static,
            >),
        false,
    )?;
    Ok(b)
}

pub(crate) fn checkCref(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
    mut info: SourceInfo,
) -> Result<bool> {
    let mut b: bool = func(getVarPointer(cref, info.clone())?)?;
    Ok(b)
}

// ==========================================================================
//                        Variable Array Stuff
//    All variable arrays are pointer arrays to avoid duplicates
// ==========================================================================
pub mod VariablePointers {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct VariablePointers {
        /// Map for cref->index
        pub map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        /// Array of variable pointers
        pub varArr: metamodelica::Ref<
            ExpandableArray::ExpandableArray<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        >,
        /// true if the variables are scalarized
        pub scalarized: bool,
    }

    impl metamodelica::gc::MMTrace for VariablePointers {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.varArr, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.scalarized, __mmv)?;
            Ok(())
        }
    }
    impl Default for VariablePointers {
        fn default() -> Self {
            Self {
                map: Default::default(),
                varArr: Default::default(),
                scalarized: Default::default(),
            }
        }
    }

    pub type VARIABLE_POINTERS = VariablePointers;

    pub(crate) fn toString(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut r#str: ArcStr,
        mut mapping_opt: Option<metamodelica::Array<(i32, i32)>>,
        mut printEmpty: bool,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut numberOfElements: i32 = size(variables);
        let mut length: i32;
        let mut scal_start: i32;
        let mut index: ArcStr;
        let mut useMapping: bool = (mapping_opt).is_some();
        let mut mapping: metamodelica::Array<(i32, i32)> =
            metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
        if useMapping {
            length = 15;
            mapping = mapping_opt.ok_or("pattern mismatch")?;
        } else {
            length = 10;
        }
        if printEmpty || numberOfElements > 0 {
            r#str = StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!(" Variables ("));
                    __mm_s.push_str(&*intString(numberOfElements));
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*intString(scalarSize(variables, true)?));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            for mut i in 1..=numberOfElements {
                if useMapping {
                    (scal_start, _) = ({
                        let __elt = (*metamodelica::index_checked(&mapping.borrow(), i)?).clone();
                        __elt
                    });
                    index = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("("));
                        __mm_s.push_str(&*intString(i));
                        __mm_s.push_str(&*literal!("|"));
                        __mm_s.push_str(&*intString(scal_start));
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    };
                } else {
                    index = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("("));
                        __mm_s.push_str(&*intString(i));
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    };
                }
                index = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*index);
                    __mm_s.push_str(&*StringUtil::repeat(literal!(" "), length - ((index).len() as i32))?);
                    ArcStr::from(__mm_s)
                };
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*super::toString(
                        &(Pointer::access(ExpandableArray::get(i, variables.varArr.clone())?)),
                        index,
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
            }
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = literal!("");
        }
        Ok(r#str)
    }

    pub(crate) fn map(
        mut variables: metamodelica::Ref<VariablePointers>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Variable::NFVariable>) -> Result<metamodelica::Ref<Variable::NFVariable>>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Variable::NFVariable>,
                ) -> Result<metamodelica::Ref<Variable::NFVariable>>
                + 'static,
        >;

        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        let mut new_var: metamodelica::Ref<Variable::NFVariable>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i, variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i, variables.varArr.clone())?;
                var = Pointer::access(var_ptr.clone());
                new_var = func(var.clone())?;
                if !(referenceEq(&*(var), &*(&*new_var))) {
                    Pointer::update(var_ptr, new_var);
                }
            }
        }
        Ok(variables)
    }

    pub(crate) fn mapPtr(
        mut variables: metamodelica::Ref<VariablePointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<()>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<()> + 'static,
        >;

        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i, variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i, variables.varArr.clone())?;
                func(var_ptr)?;
            }
        }
        Ok(variables)
    }

    pub(crate) fn mapRemovePtr(
        mut variables: metamodelica::Ref<VariablePointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static,
        >;

        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i, variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i, variables.varArr.clone())?;
                if func(var_ptr.clone())? {
                    variables = remove(var_ptr, variables)?;
                }
            }
        }
        variables = compress(variables)?;
        Ok(variables)
    }

    pub(crate) fn empty(mut size: i32, mut scalarized: bool) -> metamodelica::Ref<VariablePointers> {
        let mut variables: metamodelica::Ref<VariablePointers>;
        let mut arr_size: i32;
        let mut bucketSize: i32;
        let mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >;
        arr_size = std::cmp::max(size, BaseHashTable::lowBucketSize.clone());
        bucketSize = Util::nextPrime(arr_size);
        if scalarized {
            map = UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                          __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                bucketSize,
            );
        } else {
            map = UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hashStrip(&__a0)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                          __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::isEqualStrip(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                bucketSize,
            );
        }
        variables = metamodelica::Ref::new(VariablePointers {
            map: map,
            varArr: ExpandableArray::new(arr_size, Pointer::create(DUMMY_VARIABLE().clone())),
            scalarized: scalarized,
        });
        variables
    }

    pub(crate) fn clone(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut shallow: bool,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut new: metamodelica::Ref<VariablePointers>;
        if shallow {
            new = fromList(&(toList(variables)?), false)?;
        } else {
            new = fromList(
                &({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                        metamodelica::nil();
                    for mut eqn in (toList(variables)?).into_iter().cloned() {
                        let __x = Pointer::create(Pointer::access(eqn.clone()));
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                false,
            )?;
        }
        Ok(new)
    }

    pub(crate) fn size(mut variables: &metamodelica::Ref<VariablePointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getNumberOfElements(variables.varArr.clone());
        sz
    }

    pub(crate) fn lastUsedIndex(mut variables: &metamodelica::Ref<VariablePointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getLastUsedIndex(variables.varArr.clone());
        sz
    }

    pub(crate) fn scalarSize(mut variables: &metamodelica::Ref<VariablePointers>, mut resize: bool) -> Result<i32> {
        let mut sz: i32 = 0;
        for mut var_ptr in &*toList(variables)? {
            sz = sz + super::size(var_ptr.clone(), resize)?;
        }
        Ok(sz)
    }

    pub(crate) fn toList(
        mut variables: &metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
        let mut var_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
        var_lst = ExpandableArray::toList(variables.varArr.clone())?;
        Ok(var_lst)
    }

    pub(crate) fn fromList(
        mut var_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut scalarized: bool,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers>;
        variables = empty(((var_lst).len() as i32), scalarized);
        variables = addList(var_lst, variables)?;
        Ok(variables)
    }

    pub(crate) fn addList(
        mut var_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        variables = List::fold(var_lst, &(move |__pe_a0, __pe_a1| add(__pe_a0, __pe_a1)), variables)?;
        Ok(variables)
    }

    pub(crate) fn removeList(
        mut var_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        variables = List::fold(var_lst, &(move |__pe_a0, __pe_a1| remove(__pe_a0, __pe_a1)), variables)?;
        variables = compress(variables)?;
        Ok(variables)
    }

    pub(crate) fn removeCheck(
        mut variables: metamodelica::Ref<VariablePointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
        vars = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut var in (toList(&variables)?).into_iter().cloned() {
                if !(!(func(var.clone())?)) {
                    continue;
                }
                let __x = var.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        variables = fromList(&vars, false)?;
        Ok(variables)
    }

    pub(crate) fn add(
        mut varPointer: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        let mut index: i32;
        var = Pointer::access(varPointer.clone());
        let () = (match UnorderedMap::get(var.name.clone(), variables.map.clone())? {
            Some(mut index)
                if (index > 0
                    && (variables.scalarized.clone()
                        || ComponentRef::isEqual(
                            &var.name,
                            &(getVarName(ExpandableArray::get(index, variables.varArr.clone())?)),
                        )?)) =>
            {
                ExpandableArray::update(index, varPointer, variables.varArr.clone())?;
                ()
            }
            _ => {
                (_, index) = ExpandableArray::add(varPointer, variables.varArr.clone())?;
                UnorderedMap::add(var.name.clone(), index, variables.map.clone())?;
                ()
            }
        });
        Ok(variables)
    }

    pub(crate) fn remove(
        mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        let mut index: i32;
        var = Pointer::access(var_ptr);
        let () = (match UnorderedMap::get(var.name.clone(), variables.map.clone())? {
            Some(mut index) if (index > 0) => {
                ExpandableArray::delete(index, variables.varArr.clone())?;
                UnorderedMap::add(var.name.clone(), -1, variables.map.clone())?;
                ()
            }
            _ => (),
        });
        Ok(variables)
    }

    pub(crate) fn setVarAt(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut idx: i32,
        mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    ) -> Result<()> {
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        ExpandableArray::set(idx, var_ptr.clone(), variables.varArr.clone())?;
        var = Pointer::access(var_ptr);
        UnorderedMap::add(var.name.clone(), idx, variables.map.clone())?;
        Ok(())
    }

    pub(crate) fn getVarAt(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut idx: i32,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
        let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        var = ExpandableArray::get(idx, variables.varArr.clone())?;
        Ok(var)
    }

    pub(crate) fn getVarSafe(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut info: Option<SourceInfo>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
        let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        let mut index: i32;
        var_ptr = (match UnorderedMap::get(cref.clone(), variables.map.clone())? {
            Some(mut index) if (index > 0) => ExpandableArray::get(index, variables.varArr.clone())?,
            _ => {
                if (info).is_some() {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBVariable.VariablePointers.getVarSafe"));
                            __mm_s.push_str(&*literal!(" failed for "));
                            __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                            ArcStr::from(__mm_s)
                        },
                        info.ok_or("pattern mismatch")?,
                    )?;
                }
                return Err("fail");
            }
        });
        Ok(var_ptr)
    }

    pub(crate) fn getVarIndex(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> Result<i32> {
        let mut index: i32 = UnorderedMap::getOrDefault(cref.clone(), variables.map.clone(), -1)?;
        Ok(index)
    }

    pub(crate) fn contains(
        mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut variables: &metamodelica::Ref<VariablePointers>,
    ) -> Result<bool> {
        let mut b: bool = containsCref(getVarName(var.clone()), variables)?;
        Ok(b)
    }

    pub(crate) fn containsCref(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut variables: &metamodelica::Ref<VariablePointers>,
    ) -> Result<bool> {
        let mut b: bool = getVarIndex(variables, cref.clone())? > 0;
        Ok(b)
    }

    pub(crate) fn getVarNames(
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut acc: Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
            Pointer::create(metamodelica::nil());
        mapPtr(
            variables,
            &({
                let __pe_b1 = acc.clone();
                move |__pe_a0| Ok(getVarNameTraverse(__pe_a0, __pe_b1.clone()))
            }),
        )?;
        names = Pointer::access(acc).reverse();
        Ok(names)
    }

    pub(crate) fn getScalarVarNames(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut resize: bool,
    ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        for mut var_ptr in &*toList(variables)? {
            var = Pointer::access(var_ptr.clone());
            if Type::isArray(&var.ty) {
                for mut cr in &*ComponentRef::scalarizeAll(ComponentRef::stripSubscriptsAll(&var.name), resize)? {
                    if Type::isComplex(&(ComponentRef::nodeType(metamodelica::AsArg::as_arg(&cr))?)) {
                        names = listAppend(ComponentRef::getRecordChildren(cr.clone())?, names);
                    } else {
                        names = metamodelica::cons(cr.clone(), names);
                    }
                }
            } else {
                names = metamodelica::cons(var.name.clone(), names);
            }
        }
        Ok(names)
    }

    pub(crate) fn getMarkedVars(
        mut variables: &metamodelica::Ref<VariablePointers>,
        mut marks: metamodelica::Array<bool>,
    ) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
        let mut marked_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
        let mut indices: metamodelica::List<i32> = BackendUtil::findTrueIndices(marks.clone())?;
        if metamodelica::arrayLength(marks.clone()) == size(variables) {
            marked_vars = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                    metamodelica::nil();
                for mut index in (indices).into_iter().cloned() {
                    let __x = getVarAt(variables, index.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBVariable.VariablePointers.getMarkedVars"));
                    __mm_s.push_str(&*literal!(" failed because the number var marks ("));
                    __mm_s.push_str(&*intString(metamodelica::arrayLength(marks.clone())));
                    __mm_s.push_str(&*literal!(") is not equal to the number of variables ("));
                    __mm_s.push_str(&*intString(size(variables)));
                    __mm_s.push_str(&*literal!(")."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(marked_vars)
    }

    pub(crate) fn compress(
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut i in ({
            let __s = ExpandableArray::getLastUsedIndex(variables.varArr.clone());
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            if ExpandableArray::occupied(i, variables.varArr.clone()) {
                vars = metamodelica::cons(ExpandableArray::get(i, variables.varArr.clone())?, vars);
            }
        }
        variables = fromList(&vars, false)?;
        Ok(variables)
    }

    pub(crate) fn sort(
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut size: i32;
        let mut hash_lst: metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>)>;
        let mut hash_lst_ptr: Pointer::Pointer<
            metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>)>,
        > = Pointer::create(metamodelica::nil());
        let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        size = ExpandableArray::getNumberOfElements(variables.varArr.clone());
        mapPtr(
            variables.clone(),
            &({
                let __pe_b1 = ((metamodelica::OrderedFloat((size) as f64)
                    * (metamodelica::OrderedFloat((size) as f64)).ln())
                .0
                .floor() as i32);
                let __pe_b2 = hash_lst_ptr.clone();
                move |__pe_a0| createSortHashTpl(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?;
        hash_lst = List::sort(
            Pointer::access(hash_lst_ptr),
            std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)),
        )?;
        variables = empty(size, variables.scalarized.clone());
        for mut tpl in &*hash_lst {
            (_, var_ptr) = tpl.clone();
            variables = add(var_ptr, variables)?;
        }
        Ok(variables)
    }

    pub(crate) fn scalarize(
        mut variables: metamodelica::Ref<VariablePointers>,
    ) -> Result<metamodelica::Ref<VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers> = variables;
        let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
        let mut flattened: bool;
        (vars, flattened) = scalarizeList(&(toList(&variables)?))?;
        if flattened {
            variables = fromList(&vars, true)?;
        }
        Ok(variables)
    }

    pub(crate) fn scalarizeList(
        mut vars: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> Result<(
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        bool,
    )> {
        let mut new_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        let mut flattened: bool = false;
        let mut scalar_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
        let mut element_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        for mut var_ptr in &**vars {
            var = Pointer::access(var_ptr.clone());
            if Type::isArray(&var.ty) {
                flattened = true;
                scalar_vars = Scalarize::scalarizeBackendVariable(&var, metamodelica::nil())?;
            } else {
                scalar_vars = list![Pointer::access(var_ptr.clone())];
            }
            for mut var in &*scalar_vars {
                let mut var = var.clone();
                if Type::isComplex(&var.ty) {
                    flattened = true;
                    element_vars = Scalarize::scalarizeComplexVariable(var, metamodelica::nil())?;
                    for mut elem_var in &*element_vars.reverse() {
                        new_vars = metamodelica::cons(Pointer::create(elem_var.clone()), new_vars);
                    }
                } else {
                    new_vars = metamodelica::cons(Pointer::create(var), new_vars);
                }
            }
        }
        new_vars = new_vars.reverse();
        Ok((new_vars, flattened))
    }

    pub(crate) fn varSlice(
        mut vars: &metamodelica::Ref<VariablePointers>,
        mut scal: i32,
        mut arr: i32,
        mut mapping: &metamodelica::Ref<Mapping::Mapping>,
        mut resize: bool,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        let mut start: i32;
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut sizes: metamodelica::List<i32>;
        let mut vals: metamodelica::List<i32>;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        (start, _) = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), arr)?).clone();
            __elt
        });
        var = getVarAt(vars, arr)?;
        let __arc2 = Pointer::access(var);
        let Variable::VARIABLE {
            name: __pa0, ty: __pa1, ..
        } = &*__arc2;
        cref = metamodelica::Own::own(__pa0);
        ty = metamodelica::Own::own(__pa1);
        dims = Type::arrayDims(ty);
        sizes = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut dim in (dims.clone()).into_iter().cloned() {
                let __x = Dimension::size(&(dim.clone()), resize)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        vals = Slice::indexToLocation(scal - start, sizes).reverse();
        subs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            let __thr_src0 = dims;
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = vals;
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(dim), Some(val)) => {
                        let __x = Subscript::nth(&(dim.clone()), val.clone() + 1)?;
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        });
        cref = ComponentRef::mergeSubscripts(subs, cref, true, true, false)?;
        Ok(cref)
    }

    fn createSortHashTpl(
        mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut r#mod: i32,
        mut hash_lst_ptr: Pointer::Pointer<
            metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>)>,
        >,
    ) -> Result<()> {
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        let mut hash: i32;
        var = Pointer::access(var_ptr.clone());
        hash = stringHashDjb2Mod(&(BackendExtension::BackendInfo::toString(&var.backendinfo)?), r#mod);
        Pointer::update(
            hash_lst_ptr.clone(),
            metamodelica::cons((hash, var_ptr), Pointer::access(hash_lst_ptr)),
        );
        Ok(())
    }
}

// ==========================================================================
//                        Variable Data
//    All variable arrays are pointer arrays to avoid duplicates
// ==========================================================================
pub mod VarData {
    use super::*;
    /// All variable arrays are pointer subsets of an array of variables indicated
    ///    by preceding comment. Used to traverse all variables of a special kind.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum VarData {
        /// Only to be used for simulation systems.
        VAR_DATA_SIM {
            /// use when trying to create unique variables
            uniqueIndex: Pointer::Pointer<i32>,
            /// All variables
            variables: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// All state derivatives, algebraic variables,
            ///                                          discrete variables
            unknowns: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Parameters, constants, states
            knowns: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// All initial unknowns (unknowns + states + previous + parameters(non const binding))
            initials: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal with 1 or -1 coefficient
            aliasVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal with gain * alias + offset function
            nonTrivialAlias: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// State derivatives (der(x) -> $DER.x)
            derivatives: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Algebraic variables
            algebraics: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Discrete variables
            discretes: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Discrete state variables
            discrete_states: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Clocked state variables
            clocked_states: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Previous variables (pre(d) -> $PRE.d)
            previous: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// clock variables
            clocks: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// States
            states: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Top level inputs
            top_level_inputs: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Resizable Parameters
            resizables: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Parameters
            parameters: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Constants
            constants: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Records
            records: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// External Objects
            external_objects: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// artificial variables to have pointers on crefs
            artificials: metamodelica::Ref<VariablePointers::VariablePointers>,
            state_order: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                >,
            >,
        },
        /// Only to be used for Jacobians.
        VAR_DATA_JAC {
            /// All jacobian variables
            variables: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// All result and temporary vars
            unknowns: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal
            aliasVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Differentiation variables z where J = dF/dz
            diffVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// All occurring unknowns for linearity analysis
            dependencies: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Result variable depending on current seed
            ///                                          ($RES.[jacname].[eq_idx])
            resultVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Temporary variables (inner partial derivatives)
            ///                                          dy/dz with y!=z for all y and z
            ///                                          ($TMP.[jacname].y)
            tmpVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Seed variables representing a generic derivative
            ///                                          dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED.[jacname].x)
            seedVars: metamodelica::Ref<VariablePointers::VariablePointers>,
        },
        /// Only to be used for Hessians.
        VAR_DATA_HES {
            /// All hessian variables
            variables: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// All state derivatives, algebraic variables,
            ///                                          discrete variables
            unknowns: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Parameters, constants
            knowns: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal
            aliasVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Differentiation variables z where J = dF/dz
            diffVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// All occurring unknowns for linearity analysis
            dependencies: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Result variable depending on current seed
            ///                                          ($RES.[jacname].[eq_idx])
            resultVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Temporary variables (inner partial derivatives)
            ///                                          dy/dz with y!=z for all y and z
            ///                                          ($TMP.[jacname].y)
            tmpVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Seed variables representing a generic derivative
            ///                                          dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED.[jacname].x)
            seedVars: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Second seed variables representing a generic
            ///                                          derivative dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED2.[jacname].x)
            seedVars2: metamodelica::Ref<VariablePointers::VariablePointers>,
            /// Lambda variables for optimization
            lambdaVars: Option<metamodelica::Ref<VariablePointers::VariablePointers>>,
        },
        VAR_DATA_EMPTY,
    }
    impl metamodelica::gc::MMTrace for VarData {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                VarData::VAR_DATA_SIM {
                    uniqueIndex,
                    variables,
                    unknowns,
                    knowns,
                    initials,
                    auxiliaries,
                    aliasVars,
                    nonTrivialAlias,
                    derivatives,
                    algebraics,
                    discretes,
                    discrete_states,
                    clocked_states,
                    previous,
                    clocks,
                    states,
                    top_level_inputs,
                    resizables,
                    parameters,
                    constants,
                    records,
                    external_objects,
                    artificials,
                    state_order,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(uniqueIndex, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(unknowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(knowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(initials, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(nonTrivialAlias, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(derivatives, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(algebraics, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(discretes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(discrete_states, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(clocked_states, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(previous, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(clocks, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(states, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(top_level_inputs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(resizables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(parameters, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(constants, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(records, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(external_objects, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(artificials, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(state_order, __mmv)?;
                    Ok(())
                }
                VarData::VAR_DATA_JAC {
                    variables,
                    unknowns,
                    auxiliaries,
                    aliasVars,
                    diffVars,
                    dependencies,
                    resultVars,
                    tmpVars,
                    seedVars,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(unknowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(diffVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(dependencies, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(resultVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(tmpVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(seedVars, __mmv)?;
                    Ok(())
                }
                VarData::VAR_DATA_HES {
                    variables,
                    unknowns,
                    knowns,
                    auxiliaries,
                    aliasVars,
                    diffVars,
                    dependencies,
                    resultVars,
                    tmpVars,
                    seedVars,
                    seedVars2,
                    lambdaVars,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(unknowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(knowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(diffVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(dependencies, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(resultVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(tmpVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(seedVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(seedVars2, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lambdaVars, __mmv)?;
                    Ok(())
                }
                VarData::VAR_DATA_EMPTY => Ok(()),
            }
        }
    }
    impl VarData {
        pub fn interned_VAR_DATA_EMPTY() -> metamodelica::Ref<VarData> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VarData> = metamodelica::Ref::new(VarData::VAR_DATA_EMPTY);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_VAR_DATA_EMPTY() -> metamodelica::Ref<VarData> {
        VarData::interned_VAR_DATA_EMPTY()
    }
    impl Default for VarData {
        fn default() -> Self {
            Self::VAR_DATA_EMPTY
        }
    }
    pub use self::VarData::{VAR_DATA_EMPTY, VAR_DATA_HES, VAR_DATA_JAC, VAR_DATA_SIM};
    pub(crate) fn size(mut varData: &metamodelica::Ref<VarData>) -> Result<i32> {
        let mut s: i32;
        s = (match &**varData {
            VAR_DATA_SIM {
                unknowns: __varData_unknowns,
                ..
            } => VariablePointers::size(metamodelica::AsArg::as_arg(&__varData_unknowns)),
            VAR_DATA_JAC {
                unknowns: __varData_unknowns,
                ..
            } => VariablePointers::size(metamodelica::AsArg::as_arg(&__varData_unknowns)),
            VAR_DATA_HES {
                unknowns: __varData_unknowns,
                ..
            } => VariablePointers::size(metamodelica::AsArg::as_arg(&__varData_unknowns)),
            _ => return Err("match: no arm matched"),
        });
        Ok(s)
    }

    pub(crate) fn scalarSize(mut varData: &metamodelica::Ref<VarData>, mut resize: bool) -> Result<i32> {
        let mut s: i32;
        s = (match &**varData {
            VAR_DATA_SIM {
                unknowns: __varData_unknowns,
                ..
            } => VariablePointers::scalarSize(metamodelica::AsArg::as_arg(&__varData_unknowns), resize)?,
            VAR_DATA_JAC {
                unknowns: __varData_unknowns,
                ..
            } => VariablePointers::scalarSize(metamodelica::AsArg::as_arg(&__varData_unknowns), resize)?,
            VAR_DATA_HES {
                unknowns: __varData_unknowns,
                ..
            } => VariablePointers::scalarSize(metamodelica::AsArg::as_arg(&__varData_unknowns), resize)?,
            _ => return Err("match: no arm matched"),
        });
        Ok(s)
    }

    pub(crate) fn toString(mut varData: &metamodelica::Ref<VarData>, mut level: i32) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = if (level == 0) {
            (match &**varData {
                VAR_DATA_SIM {
                    variables: __varData_variables,
                    ..
                } => VariablePointers::toString(
                    metamodelica::AsArg::as_arg(&__varData_variables),
                    literal!("Simulation"),
                    None,
                    true,
                )?,
                VAR_DATA_JAC {
                    variables: __varData_variables,
                    ..
                } => VariablePointers::toString(
                    metamodelica::AsArg::as_arg(&__varData_variables),
                    literal!("Jacobian"),
                    None,
                    true,
                )?,
                VAR_DATA_HES {
                    variables: __varData_variables,
                    ..
                } => VariablePointers::toString(
                    metamodelica::AsArg::as_arg(&__varData_variables),
                    literal!("Hessian"),
                    None,
                    true,
                )?,
                VAR_DATA_EMPTY { .. } => literal!("Empty variable Data!\n"),
                _ => return Err("fail"),
            })
        } else if (level == 1) {
            toStringVerbose(varData, false)?
        } else {
            toStringVerbose(varData, true)?
        };
        Ok(r#str)
    }

    pub(crate) fn toStringVerbose(mut varData: &metamodelica::Ref<VarData>, mut full: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = ({
            let mut tmp: ArcStr = literal!("");
            (match &**varData {
                VAR_DATA_SIM {
                    algebraics: __varData_algebraics,
                    aliasVars: __varData_aliasVars,
                    artificials: __varData_artificials,
                    auxiliaries: __varData_auxiliaries,
                    clocked_states: __varData_clocked_states,
                    clocks: __varData_clocks,
                    constants: __varData_constants,
                    derivatives: __varData_derivatives,
                    discrete_states: __varData_discrete_states,
                    discretes: __varData_discretes,
                    external_objects: __varData_external_objects,
                    knowns: __varData_knowns,
                    parameters: __varData_parameters,
                    previous: __varData_previous,
                    records: __varData_records,
                    resizables: __varData_resizables,
                    states: __varData_states,
                    top_level_inputs: __varData_top_level_inputs,
                    unknowns: __varData_unknowns,
                    ..
                } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Variable Data Simulation (scalar unknowns: "));
                        __mm_s.push_str(&*intString(VariablePointers::scalarSize(
                            metamodelica::AsArg::as_arg(&__varData_unknowns),
                            true,
                        )?));
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_2(&tmp)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                    if !(full) {
                        tmp = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_unknowns),
                                literal!("Unknown"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_states),
                                literal!("Local Known"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_knowns),
                                literal!("Global Known"),
                                None,
                                false,
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    } else {
                        tmp = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_states),
                                literal!("State"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_derivatives),
                                literal!("Derivative"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_algebraics),
                                literal!("Algebraic"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_discretes),
                                literal!("Discrete"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_discrete_states),
                                literal!("Discrete State"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_clocked_states),
                                literal!("Clocked State"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_previous),
                                literal!("Previous"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_clocks),
                                literal!("Clock"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_top_level_inputs),
                                literal!("Top Level Input"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_resizables),
                                literal!("Resizable Parameter"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_parameters),
                                literal!("Parameter"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_constants),
                                literal!("Constant"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_records),
                                literal!("Record"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_external_objects),
                                literal!("External Object"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_artificials),
                                literal!("Artificial"),
                                None,
                                false,
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    }
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_auxiliaries),
                            literal!("Auxiliary"),
                            None,
                            false,
                        )?);
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_aliasVars),
                            literal!("Alias"),
                            None,
                            false,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp
                }
                VAR_DATA_JAC {
                    aliasVars: __varData_aliasVars,
                    auxiliaries: __varData_auxiliaries,
                    dependencies: __varData_dependencies,
                    diffVars: __varData_diffVars,
                    resultVars: __varData_resultVars,
                    seedVars: __varData_seedVars,
                    tmpVars: __varData_tmpVars,
                    unknowns: __varData_unknowns,
                    ..
                } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_unknowns),
                            literal!("Partial Derivative"),
                            None,
                            false,
                        )?);
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_seedVars),
                            literal!("Seed"),
                            None,
                            false,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    if full {
                        tmp = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_diffVars),
                                literal!("Differentiation"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_resultVars),
                                literal!("Residual"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_tmpVars),
                                literal!("Inner"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_dependencies),
                                literal!("Dependencies"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_auxiliaries),
                                literal!("Auxiliary"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_aliasVars),
                                literal!("Alias"),
                                None,
                                false,
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    }
                    tmp
                }
                VAR_DATA_HES {
                    aliasVars: __varData_aliasVars,
                    auxiliaries: __varData_auxiliaries,
                    dependencies: __varData_dependencies,
                    diffVars: __varData_diffVars,
                    knowns: __varData_knowns,
                    lambdaVars: __varData_lambdaVars,
                    resultVars: __varData_resultVars,
                    seedVars: __varData_seedVars,
                    seedVars2: __varData_seedVars2,
                    tmpVars: __varData_tmpVars,
                    unknowns: __varData_unknowns,
                    ..
                } => {
                    let mut lambdaVars: metamodelica::Ref<VariablePointers::VariablePointers>;
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_2(&(literal!("Variable Data Hessian")))?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_unknowns),
                            literal!("Unknown"),
                            None,
                            false,
                        )?);
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_knowns),
                            literal!("Known"),
                            None,
                            false,
                        )?);
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_auxiliaries),
                            literal!("Auxiliary"),
                            None,
                            false,
                        )?);
                        __mm_s.push_str(&*VariablePointers::toString(
                            metamodelica::AsArg::as_arg(&__varData_aliasVars),
                            literal!("Alias"),
                            None,
                            false,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    if full {
                        tmp = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_diffVars),
                                literal!("Differentiation"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_dependencies),
                                literal!("Dependencies"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_resultVars),
                                literal!("Result"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_tmpVars),
                                literal!("Temporary"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_seedVars),
                                literal!("First Seed"),
                                None,
                                false,
                            )?);
                            __mm_s.push_str(&*VariablePointers::toString(
                                metamodelica::AsArg::as_arg(&__varData_seedVars2),
                                literal!("Second Seed"),
                                None,
                                false,
                            )?);
                            ArcStr::from(__mm_s)
                        };
                        if (__varData_lambdaVars).is_some() {
                            let __pa0 = ::match_deref::match_deref! { match &(__varData_lambdaVars.clone()) {
                                Some(__pa0) => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            lambdaVars = metamodelica::Own::own(__pa0);
                            tmp = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*tmp);
                                __mm_s.push_str(&*VariablePointers::toString(
                                    &lambdaVars,
                                    literal!("Lagrangian Lambda"),
                                    None,
                                    false,
                                )?);
                                ArcStr::from(__mm_s)
                            };
                        }
                    }
                    tmp
                }
                _ => return Err("fail"),
            })
        });
        Ok(r#str)
    }

    pub(crate) fn getVariables(
        mut varData: &metamodelica::Ref<VarData>,
    ) -> Result<metamodelica::Ref<VariablePointers::VariablePointers>> {
        let mut variables: metamodelica::Ref<VariablePointers::VariablePointers>;
        variables = (match &**varData {
            VAR_DATA_SIM {
                variables: __varData_variables,
                ..
            } => __varData_variables.clone(),
            VAR_DATA_JAC {
                variables: __varData_variables,
                ..
            } => __varData_variables.clone(),
            VAR_DATA_HES {
                variables: __varData_variables,
                ..
            } => __varData_variables.clone(),
            _ => return Err("fail"),
        });
        Ok(variables)
    }

    pub(crate) fn setVariables(
        mut varData: metamodelica::Ref<VarData>,
        mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    ) -> Result<metamodelica::Ref<VarData>> {
        let mut varData: metamodelica::Ref<VarData> = varData;
        varData = (match &*varData {
            VAR_DATA_SIM { .. } => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM; variables = variables);
                varData
            }
            VAR_DATA_JAC { .. } => {
                assign_variant_field!(varData => VarData::VAR_DATA_JAC; variables = variables);
                varData
            }
            VAR_DATA_HES { .. } => {
                assign_variant_field!(varData => VarData::VAR_DATA_HES; variables = variables);
                varData
            }
            _ => return Err("fail"),
        });
        Ok(varData)
    }

    pub(crate) fn getUniqueIndex(mut varData: &metamodelica::Ref<VarData>) -> Result<Pointer::Pointer<i32>> {
        let mut uniqueIndex: Pointer::Pointer<i32>;
        uniqueIndex = (match &**varData {
            VAR_DATA_SIM {
                uniqueIndex: __varData_uniqueIndex,
                ..
            } => __varData_uniqueIndex.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBVariable.VarData.getUniqueIndex"));
                        __mm_s.push_str(&*literal!(" failed because of incorrect record type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(uniqueIndex)
    }

    pub(crate) fn getStateOrder(
        mut varData: &metamodelica::Ref<VarData>,
    ) -> Result<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
    > {
        let mut state_order: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >;
        state_order = (match &**varData {
            VAR_DATA_SIM {
                state_order: __varData_state_order,
                ..
            } => __varData_state_order.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBVariable.VarData.getStateOrder"));
                        __mm_s.push_str(&*literal!(" failed because of incorrect record type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(state_order)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
    #[repr(i32)]
    pub enum VarType {
        STATE = 1,
        STATE_DER = 2,
        ALGEBRAIC = 3,
        DISCRETE = 4,
        DISC_STATE = 5,
        PREVIOUS = 6,
        START = 7,
        PARAMETER = 8,
        ITERATOR = 9,
        RECORD = 10,
        CLOCK = 11,
    }
    impl PartialOrd for VarType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for VarType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (*self as i32).cmp(&(*other as i32))
        }
    }
    impl metamodelica::gc::MMTrace for VarType {
        fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            Ok(())
        }
    }

    pub(crate) fn addTypedList(
        mut varData: metamodelica::Ref<VarData>,
        mut var_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut varType: VarType,
    ) -> Result<metamodelica::Ref<VarData>> {
        let mut varData: metamodelica::Ref<VarData> = varData;
        if (var_lst).is_empty() {
            return Ok(varData);
        }
        varData = (::match_deref::match_deref! { match &((varData.clone(), varType)) {
            (Deref @ VAR_DATA_SIM { .. }, VarType::STATE { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    knowns = VariablePointers::addList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?,
                    states = VariablePointers::addList(var_lst, var_field!((*varData).states, VarData::VAR_DATA_SIM).clone())?,
                    initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?,
                    unknowns = VariablePointers::removeList(var_lst, var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                    algebraics = VariablePointers::removeList(var_lst, var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone())?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::STATE_DER) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    unknowns = VariablePointers::addList(var_lst, var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                    derivatives = VariablePointers::addList(var_lst, var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone())?,
                    initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::ALGEBRAIC) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    unknowns = VariablePointers::addList(var_lst, var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                    algebraics = VariablePointers::addList(var_lst, var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone())?,
                    initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?,
                    states = VariablePointers::removeList(var_lst, var_field!((*varData).states, VarData::VAR_DATA_SIM).clone())?,
                    derivatives = VariablePointers::removeList(var_lst, var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone())?,
                    knowns = VariablePointers::removeList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::DISCRETE) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    unknowns = VariablePointers::addList(var_lst, var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                    discretes = VariablePointers::addList(var_lst, var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone())?,
                    initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::START { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::PARAMETER { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    parameters = VariablePointers::addList(var_lst, var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone())?,
                    knowns = VariablePointers::addList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::ITERATOR { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    knowns = VariablePointers::addList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?,
                    artificials = VariablePointers::addList(var_lst, var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone())?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::CLOCK) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM; clocks = VariablePointers::addList(var_lst, var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone())?);
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::RECORD { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::addList(var_lst, var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                    records = VariablePointers::addList(var_lst, var_field!((*varData).records, VarData::VAR_DATA_SIM).clone())?,
                    knowns = VariablePointers::addList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?
                );
                assign_variant_field!(varData => VarData::VAR_DATA_SIM; records = VariablePointers::mapPtr(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), &({ let __pe_b1 = var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(); move |__pe_a0| BackendDAE::lowerUnkownRecordChildren(__pe_a0, &__pe_b1) }))?);
                varData
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.addTypedList")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(varData)
    }

    pub(crate) fn removeTypedCheck(
        mut varData: metamodelica::Ref<VarData>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
        mut varType: VarType,
    ) -> Result<metamodelica::Ref<VarData>> {
        let mut varData: metamodelica::Ref<VarData> = varData;
        varData = (::match_deref::match_deref! { match &((varData.clone(), varType)) {
            (Deref @ VAR_DATA_SIM { .. }, VarType::STATE { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func)?,
                    states = VariablePointers::removeCheck(var_field!((*varData).states, VarData::VAR_DATA_SIM).clone(), func)?,
                    initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::STATE_DER) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func)?,
                    derivatives = VariablePointers::removeCheck(var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone(), func)?,
                    initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::ALGEBRAIC) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func)?,
                    algebraics = VariablePointers::removeCheck(var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone(), func)?,
                    initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::DISCRETE) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func)?,
                    discretes = VariablePointers::removeCheck(var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone(), func)?,
                    initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::START { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::PARAMETER { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    parameters = VariablePointers::removeCheck(var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone(), func)?,
                    knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::ITERATOR { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func)?,
                    artificials = VariablePointers::removeCheck(var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::CLOCK) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM; clocks = VariablePointers::removeCheck(var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone(), func)?);
                varData
            },
            (Deref @ VAR_DATA_SIM { .. }, VarType::RECORD { .. }) => {
                assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                    variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func)?,
                    records = VariablePointers::removeCheck(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), func)?,
                    knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func)?
                );
                varData
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.removeTypedCheck")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(varData)
    }
}

// ==========================================================================
//                      Protected utility functions
// ==========================================================================
fn getVarNameTraverse(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut acc: Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> () {
    Pointer::update(acc.clone(), metamodelica::cons(getVarName(var), Pointer::access(acc)));
    ()
}
