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

use crate::BaseModelica;
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableAttributes;
use crate::NFBackendExtension::VariableKind;
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::AccessLevel;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFVariable {
    pub name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    pub ty: metamodelica::Ref<Type::NFType>,
    pub binding: metamodelica::Ref<Binding::NFBinding>,
    pub visibility: Visibility,
    pub attributes: metamodelica::Ref<Attributes::NFAttributes>,
    pub typeAttributes: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    pub children: metamodelica::List<metamodelica::Ref<NFVariable>>,
    pub comment: metamodelica::Ref<SCode::Comment>,
    pub info: SourceInfo,
    /// NFBackendExtension.DUMMY_BACKEND_INFO for all of frontend. Only used in Backend.
    pub backendinfo: metamodelica::Ref<BackendInfo::BackendInfo>,
}

impl metamodelica::gc::MMTrace for NFVariable {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.binding, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visibility, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.attributes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.typeAttributes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.children, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.comment, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.backendinfo, __mmv)?;
        Ok(())
    }
}
impl Default for NFVariable {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Default::default(),
            binding: Default::default(),
            visibility: Default::default(),
            attributes: Default::default(),
            typeAttributes: Default::default(),
            children: Default::default(),
            comment: Default::default(),
            info: Default::default(),
            backendinfo: Default::default(),
        }
    }
}

pub type VARIABLE = NFVariable;

pub fn fromCref(mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<metamodelica::Ref<NFVariable>> {
    let mut variable: metamodelica::Ref<NFVariable>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut class_node: metamodelica::Ref<InstNode::InstNode>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut vis: Visibility;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut info: SourceInfo;
    let mut binfo: metamodelica::Ref<BackendInfo::BackendInfo> = NFBackendExtension::DUMMY_BACKEND_INFO().clone();
    let mut child_nodes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut children: metamodelica::List<metamodelica::Ref<NFVariable>> = metamodelica::nil();
    node = ComponentRef::node(&cref)?;
    comp = InstNode::component(&node)?;
    ty = ComponentRef::getSubscriptedType(&cref, false)?;
    vis = InstNode::visibility(&node);
    attr = Component::getAttributes(&comp);
    cmt = Component::comment(&comp)?;
    info = InstNode::info(&node);
    if ComponentRef::isIterator(&cref) {
        binding = Binding::EMPTY_BINDING().clone();
        assign_field!(binfo.varKind = crate::NFBackendExtension::VariableKind::interned_ITERATOR());
    } else {
        binding = Component::getImplicitBinding(&comp, InstNode::instanceParent(node)?)?;
    }
    if !(Type::isExternalObject(&ty)) {
        children = (::match_deref::match_deref! { match &(Type::arrayElementType(&ty)) {
            __esc_elem_ty @ Deref @ Type::COMPLEX { .. } => {
                elem_ty = (*__esc_elem_ty).clone();
                class_node = Type::complexNode(metamodelica::AsArg::as_arg(&elem_ty))?;
                child_nodes = Class::getComponents(InstNode::getClass(class_node)?)?;
                children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFVariable>> = metamodelica::nil();
            for mut c in (child_nodes.clone()).borrow().iter() {
                let __x = fromCref(ComponentRef::prefixCref(c.clone(), InstNode::getType(c.clone())?, metamodelica::nil(), cref.clone())?)?;
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
    variable = metamodelica::Ref::new(NFVariable {
        name: cref,
        ty: ty,
        binding: binding,
        visibility: vis,
        attributes: attr,
        typeAttributes: metamodelica::nil(),
        children: children,
        comment: cmt,
        info: info,
        backendinfo: binfo,
    });
    Ok(variable)
}

pub fn name(mut var: &metamodelica::Ref<NFVariable>) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = var.name.clone();
    name
}

pub fn size(mut var: &metamodelica::Ref<NFVariable>, mut resize: bool) -> Result<i32> {
    let mut s: i32 = Type::sizeOf(&var.ty, resize)?;
    Ok(s)
}

pub fn hash(mut var: &metamodelica::Ref<NFVariable>) -> Result<i32> {
    let mut i: i32 = ComponentRef::hash(&var.name)?;
    Ok(i)
}

pub fn equalName(mut var1: &metamodelica::Ref<NFVariable>, mut var2: &metamodelica::Ref<NFVariable>) -> Result<bool> {
    let mut b: bool = ComponentRef::isEqual(&var1.name, &var2.name)?;
    Ok(b)
}

pub(crate) fn expand(
    mut var: metamodelica::Ref<NFVariable>,
    mut backend: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFVariable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<NFVariable>>;
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut v: metamodelica::Ref<NFVariable>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut bind_var: Variability;
    let mut bind_src: Binding::Source;
    let mut bind_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut crefs_len: i32;
    let mut expl_len: i32;
    if Type::isArray(&var.ty) {
        exp = Expression::fromCref(var.name.clone(), false)?;
        (exp, _) = ExpandExp::expandCref(exp, backend, false)?;
        expl = Expression::arrayScalarElements(&exp);
        crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut e in (expl).into_iter().cloned() {
                let __x = Expression::toCref(&(e.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        v = var.clone();
        assign_field!(v.ty = Type::arrayElementType(&v.ty));
        vars = metamodelica::nil();
        binding = var.binding.clone();
        if Binding::isBound(&binding) {
            bind_exp = Binding::getTypedExp(&binding)?;
            expl = Expression::arrayScalarElements(&((ExpandExp::expand(bind_exp, false, false)?).0));
            crefs_len = ((crefs).len() as i32);
            expl_len = ((expl).len() as i32);
            if expl_len < crefs_len {
                if intMod(crefs_len, expl_len) != 0 {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFVariable.expand"));
                            __mm_s.push_str(&*literal!(" failed to expand "));
                            __mm_s.push_str(&*ComponentRef::toString(&var.name)?);
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFVariable.mo")),
                    )?;
                }
                expl = List::flatten(List::fill(expl, intDiv(crefs_len, expl_len)))?;
            }
            bind_var = Binding::variability(&binding)?;
            bind_src = Binding::source(&binding);
            for mut cr in &*crefs {
                assign_field!(v.name = cr.clone());
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expl) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                expl = metamodelica::Own::own(__pa1);
                assign_field!(v.binding = Binding::makeFlat(exp, bind_var, bind_src, Binding::NO_CONFIDENCE.clone()));
                vars = metamodelica::cons(v.clone(), vars);
            }
        } else {
            for mut cr in &*crefs {
                assign_field!(v.name = cr.clone());
                vars = metamodelica::cons(v.clone(), vars);
            }
        }
        vars = metamodelica::Dangerous::listReverseInPlace(vars);
    } else {
        vars = list![var];
    }
    Ok(vars)
}

pub fn expandChildren(
    mut var: metamodelica::Ref<NFVariable>,
    mut arrayDims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut addDimensions: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFVariable>>> {
    let mut children: metamodelica::List<metamodelica::Ref<NFVariable>>;
    let mut newArrayDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    if addDimensions && !((arrayDims).is_empty()) {
        assign_field!(var.ty = Type::liftArrayLeftList(var.ty.clone(), arrayDims));
    }
    newArrayDims = Type::arrayDims(var.ty.clone());
    children = metamodelica::cons(
        var.clone(),
        List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<NFVariable>>> =
                    metamodelica::nil();
                for mut v in (var.children.clone()).into_iter().cloned() {
                    let __x = expandChildren(v.clone(), &newArrayDims, addDimensions)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?,
    );
    Ok(children)
}

pub fn typeOf(mut var: &metamodelica::Ref<NFVariable>) -> metamodelica::Ref<Type::NFType> {
    let mut ty: metamodelica::Ref<Type::NFType> = var.ty.clone();
    ty
}

pub fn attributes(mut variable: &metamodelica::Ref<NFVariable>) -> metamodelica::Ref<Attributes::NFAttributes> {
    let mut attributes: metamodelica::Ref<Attributes::NFAttributes> = variable.attributes.clone();
    attributes
}

pub fn variability(mut variable: &metamodelica::Ref<NFVariable>) -> Variability {
    let mut variability: Variability = variable.attributes.variability.clone();
    variability
}

pub fn setVariability(
    mut variable: metamodelica::Ref<NFVariable>,
    mut variability: Variability,
) -> metamodelica::Ref<NFVariable> {
    let mut variable: metamodelica::Ref<NFVariable> = variable;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    attr = variable.attributes.clone();
    assign_field!(attr.variability = variability);
    assign_field!(variable.attributes = attr);
    variable
}

pub(crate) fn visibility(mut variable: &metamodelica::Ref<NFVariable>) -> Visibility {
    let mut visibility: Visibility = variable.visibility.clone();
    visibility
}

pub(crate) fn isComplex(mut var: &metamodelica::Ref<NFVariable>) -> bool {
    let mut b: bool = Type::isComplex(&var.ty);
    b
}

pub(crate) fn isComplexArray(mut var: &metamodelica::Ref<NFVariable>) -> Result<bool> {
    let mut b: bool = Type::isComplexArray(&var.ty)?;
    Ok(b)
}

pub(crate) fn isStructural(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut structural: bool = variable.attributes.variability.clone() <= Variability::STRUCTURAL_PARAMETER.clone();
    structural
}

pub fn isEmptyArray(mut variable: &metamodelica::Ref<NFVariable>) -> Result<bool> {
    let mut isEmpty: bool = Type::isEmptyArray(&variable.ty)?;
    Ok(isEmpty)
}

pub(crate) fn isDeleted(mut variable: &metamodelica::Ref<NFVariable>) -> Result<bool> {
    let mut deleted: bool;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    node = ComponentRef::node(&variable.name)?;
    deleted = InstNode::isComponent(&node)? && Component::isDeleted(&(InstNode::component(&node)?))?;
    Ok(deleted)
}

pub(crate) fn isPresent(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut present: bool = !(ConnectorType::isPotentiallyPresent(variable.attributes.connectorType.clone()));
    present
}

pub(crate) fn isPotential(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut potential: bool = ConnectorType::isPotential(variable.attributes.connectorType.clone());
    potential
}

pub fn isFlow(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut potential: bool = ConnectorType::isFlow(variable.attributes.connectorType.clone());
    potential
}

pub(crate) fn isStream(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut potential: bool = ConnectorType::isStream(variable.attributes.connectorType.clone());
    potential
}

pub fn isInput(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut b: bool = variable.attributes.direction.clone() == Direction::INPUT.clone();
    b
}

pub fn isOutput(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut b: bool = variable.attributes.direction.clone() == Direction::OUTPUT.clone();
    b
}

pub fn isTopLevelInput(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut topInput: bool =
        ComponentRef::isTopLevel(&variable.name) && variable.attributes.direction.clone() == Direction::INPUT.clone();
    topInput
}

pub(crate) fn isPublic(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut isPublic: bool = variable.visibility.clone() == Visibility::PUBLIC.clone();
    isPublic
}

pub(crate) fn isProtected(mut variable: &metamodelica::Ref<NFVariable>) -> bool {
    let mut isProtected: bool = variable.visibility.clone() == Visibility::PROTECTED.clone();
    isProtected
}

pub fn isEncrypted(mut variable: &metamodelica::Ref<NFVariable>) -> Result<bool> {
    let mut isEncrypted: bool = isEncryptedName(variable.name.clone())?;
    Ok(isEncrypted)
}

pub(crate) fn isEncryptedName(mut name: metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut isEncrypted: bool = false;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef> = name;
    while ComponentRef::isCref(&cr) {
        if isEncryptedNode(&(ComponentRef::node(&cr)?)) {
            isEncrypted = true;
            return Ok(isEncrypted);
        }
        cr = ComponentRef::rest(&cr)?;
    }
    Ok(isEncrypted)
}

pub(crate) fn isEncryptedNode(mut node: &metamodelica::Ref<InstNode::InstNode>) -> bool {
    let mut isEncrypted: bool;
    let mut info: SourceInfo = InstNode::info(node);
    isEncrypted = StringUtil::endsWith(info.fileName.clone(), literal!(".moc"));
    isEncrypted
}

pub(crate) fn isAccessible(mut variable: &metamodelica::Ref<NFVariable>) -> Result<bool> {
    let mut isAccessible: bool;
    let mut oaccess: Option<AccessLevel>;
    let mut access: AccessLevel;
    oaccess = InstNode::getAccessLevel(ComponentRef::node(&variable.name)?)?;
    if (oaccess).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(oaccess) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        access = metamodelica::Own::own(__pa0);
    } else {
        access = if (isEncrypted(variable)?) {
            AccessLevel::DOCUMENTATION.clone()
        } else {
            AccessLevel::PACKAGE_DUPLICATE.clone()
        };
    }
    if access < AccessLevel::ICON.clone() {
        isAccessible = false;
    } else if access < AccessLevel::NON_PACKAGE_TEXT.clone() {
        isAccessible = !(isProtected(variable));
    } else {
        isAccessible = true;
    }
    Ok(isAccessible)
}

pub(crate) fn isFixed(mut var: &metamodelica::Ref<NFVariable>) -> Result<bool> {
    let mut fixed: bool;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    fixed = if (var.attributes.variability.clone() < Variability::DISCRETE.clone()) {
        true
    } else {
        false
    };
    binding = lookupTypeAttribute(&(literal!("fixed")), var);
    if Binding::hasExp(&binding) {
        fixed = Expression::isTrue(&(Binding::getExp(&binding)?));
    }
    Ok(fixed)
}

pub(crate) fn lookupTypeAttribute(
    mut name: &ArcStr,
    mut var: &metamodelica::Ref<NFVariable>,
) -> metamodelica::Ref<Binding::NFBinding> {
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    for mut attr in &*var.typeAttributes.clone() {
        if metamodelica::stringEq(&(Util::tuple21(attr.clone())), &name) {
            binding = Util::tuple22(attr.clone());
            return binding;
        }
    }
    binding = Binding::EMPTY_BINDING().clone();
    binding
}

pub fn applyToType(
    mut var: metamodelica::Ref<NFVariable>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>,
) -> Result<metamodelica::Ref<NFVariable>> {
    pub type typeFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
    >;

    let mut var: metamodelica::Ref<NFVariable> = var;
    assign_field!(
        var.ty = func(var.ty.clone())?,
        var.name = ComponentRef::applyToType(var.name.clone(), func)?
    );
    Ok(var)
}

pub(crate) fn propagateAnnotation(
    mut name: ArcStr,
    mut overwrite: bool,
    mut evaluate: bool,
    mut var: metamodelica::Ref<NFVariable>,
) -> Result<metamodelica::Ref<NFVariable>> {
    let mut var: metamodelica::Ref<NFVariable> = var;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut aexp: metamodelica::Ref<Absyn::Exp>;
    let mut exp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut anno: metamodelica::Ref<SCode::Annotation>;
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    if ComponentRef::isCref(&var.name) {
        node = ComponentRef::node(&var.name)?;
        if overwrite && InstNode::isComponent(&node)? {
            node = InstNode::parent(&node)?;
        }
        (r#mod, scope) = InstNode::getAnnotation(&name, node)?;
        if !(SCodeUtil::isEmptyMod(&r#mod)) {
            if evaluate {
                let () = 'mc: {
                    let __mc_input = r#mod.clone();
                    if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                        ::match_deref::match_deref! { match &__mc_input {
                            Deref @ SCode::Mod::MOD { binding: Some(aexp), .. } => {
                                let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                                let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod.clone();
                                exp = Inst::instExp(aexp.clone(), &scope, NFInstContext::ANNOTATION.clone(), var_field!((*r#mod).info, SCode::Mod::MOD))?;
                                (exp, _, _, _) = Typing::typeExp(exp.clone(), NFInstContext::ANNOTATION.clone(), var_field!((*r#mod).info, SCode::Mod::MOD), false)?;
                                exp = Ceval::evalExp(exp.clone(), &(Ceval::noTarget().clone()))?;
                                assign_variant_field!(r#mod => SCode::Mod::MOD; binding = Some(Expression::toAbsyn(exp.clone())?));
                                Ok(((), exp.clone(), r#mod.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
                    })() {
                        exp = __wb0;
                        r#mod = __wb1;
                        break 'mc __v;
                    }
                    if let Ok(__v) = (|| -> Result<_> {
                        ::match_deref::match_deref! { match &__mc_input {
                            _ => {
                                Ok(())
                            }
                            _ => return Err("nomatch"),
                        }}
                    })() {
                        break 'mc __v;
                    }
                    return Err("matchcontinue: no arm matched");
                };
            }
            anno = metamodelica::Ref::new(SCode::Annotation {
                modification: metamodelica::Ref::new(SCode::Mod::MOD {
                    finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                    eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                    subModLst: list![metamodelica::Ref::new(SCode::SubMod {
                        ident: name,
                        r#mod: r#mod
                    })],
                    binding: None,
                    comment: None,
                    info: metamodelica::sourceInfo!("NFFrontEnd/NFVariable.mo"),
                }),
            });
            assign_field!(var.comment = SCodeUtil::appendAnnotationToComment(anno, &var.comment, true)?);
        }
    }
    Ok(var)
}

pub(crate) fn removeNonTopLevelDirection(
    mut var: metamodelica::Ref<NFVariable>,
) -> Result<metamodelica::Ref<NFVariable>> {
    let mut var: metamodelica::Ref<NFVariable> = var;
    let mut rest_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    if var.attributes.direction.clone() == Direction::NONE.clone() {
        return Ok(var);
    }
    rest_name = ComponentRef::rest(&var.name)?;
    while !(ComponentRef::isEmpty(&rest_name)) {
        node = ComponentRef::node(&rest_name)?;
        if !(InstNode::isConnector(&node)? || InstNode::isInput(&node)) {
            attr = var.attributes.clone();
            assign_field!(attr.direction = Direction::NONE.clone());
            assign_field!(var.attributes = attr);
            return Ok(var);
        }
        rest_name = ComponentRef::rest(&rest_name)?;
    }
    Ok(var)
}

pub type ApplyFn =
    std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

pub(crate) fn applyExp(
    mut var: &metamodelica::Ref<NFVariable>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    Binding::applyExp(&var.binding, r#fn)?;
    for mut ty_attr in &*var.typeAttributes.clone() {
        Binding::applyExp(&(Util::tuple22(ty_attr.clone())), r#fn)?;
    }
    for mut c in &*var.children.clone() {
        applyExp(metamodelica::AsArg::as_arg(&c), r#fn)?;
    }
    Ok(())
}

pub(crate) fn applyExpShallow(
    mut var: &metamodelica::Ref<NFVariable>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    Binding::applyExpShallow(&var.binding, r#fn)?;
    for mut ty_attr in &*var.typeAttributes.clone() {
        Binding::applyExpShallow(&(Util::tuple22(ty_attr.clone())), r#fn)?;
    }
    for mut c in &*var.children.clone() {
        applyExpShallow(metamodelica::AsArg::as_arg(&c), r#fn)?;
    }
    Ok(())
}

pub type MapFn = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
        + 'static,
>;

pub fn mapExp(
    mut var: metamodelica::Ref<NFVariable>,
    mut r#fn: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFVariable>> {
    let mut var: metamodelica::Ref<NFVariable> = var;
    assign_field!(
        var.binding = Binding::mapExp(var.binding.clone(), r#fn.clone())?,
        var.typeAttributes = ({
            let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
            for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
                let __x = (
                    Util::tuple21(a.clone()),
                    Binding::mapExp(Util::tuple22(a.clone()), r#fn.clone())?,
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        var.children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFVariable>> = metamodelica::nil();
            for mut v in (var.children.clone()).into_iter().cloned() {
                let __x = mapExp(v.clone(), r#fn.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        var.backendinfo = BackendInfo::map(var.backendinfo.clone(), r#fn.clone())?,
        var.ty = Type::applyToDims(
            var.ty.clone(),
            &({
                let __pe_b1: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                > = r#fn.clone();
                move |__pe_a0| Dimension::mapExp(__pe_a0, __pe_b1.clone())
            })
        )?,
        var.name = ComponentRef::mapTypes(
            &var.name,
            &({
                let __pe_b1: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Dimension::NFDimension>,
                        ) -> Result<metamodelica::Ref<Dimension::NFDimension>>
                        + 'static,
                > = (std::sync::Arc::new({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = r#fn.clone();
                    move |__pe_a0| Dimension::mapExp(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Dimension::NFDimension>,
                            )
                                -> Result<metamodelica::Ref<Dimension::NFDimension>>
                            + 'static,
                    >);
                move |__pe_a0| Type::applyToDims(__pe_a0, &*__pe_b1)
            })
        )?
    );
    Ok(var)
}

pub(crate) fn mapExpShallow(
    mut var: metamodelica::Ref<NFVariable>,
    mut r#fn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFVariable>> {
    let mut var: metamodelica::Ref<NFVariable> = var;
    assign_field!(
        var.binding = Binding::mapExpShallow(var.binding.clone(), r#fn)?,
        var.typeAttributes = ({
            let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
            for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
                let __x = (
                    Util::tuple21(a.clone()),
                    Binding::mapExpShallow(Util::tuple22(a.clone()), r#fn)?,
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        var.children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFVariable>> = metamodelica::nil();
            for mut v in (var.children.clone()).into_iter().cloned() {
                let __x = mapExpShallow(v.clone(), r#fn)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(var)
}

pub fn toString(
    mut var: &metamodelica::Ref<NFVariable>,
    mut indent: ArcStr,
    mut printBindingType: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(
        literal!("NFVariable.toString"),
        openmodelica_util::IOStream::IOStreamType::LIST,
    )?;
    s = toStream(var, indent, printBindingType, s)?;
    r#str = IOStream::string(&s)?;
    IOStream::delete(&s)?;
    Ok(r#str)
}

pub(crate) fn toStream(
    mut var: &metamodelica::Ref<NFVariable>,
    mut indent: ArcStr,
    mut printBindingType: bool,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut first: bool;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    s = IOStream::append(s, indent)?;
    if var.visibility.clone() == Visibility::PROTECTED.clone() {
        s = IOStream::append(s, literal!("protected "))?;
    }
    s = IOStream::append(s, Attributes::toString(&var.attributes, var.ty.clone())?)?;
    s = IOStream::append(s, Type::toString(&var.ty)?)?;
    s = IOStream::append(s, literal!(" "))?;
    s = IOStream::append(s, ComponentRef::toString(&var.name)?)?;
    if !((var.typeAttributes).is_empty()) {
        s = IOStream::append(s, literal!("("))?;
        first = true;
        for mut a in &*var.typeAttributes.clone() {
            if first {
                first = false;
            } else {
                s = IOStream::append(s, literal!(", "))?;
            }
            b = Util::tuple22(a.clone());
            if Binding::isEach(&b) {
                s = IOStream::append(s, literal!("each "))?;
            }
            s = IOStream::append(s, Util::tuple21(a.clone()))?;
            s = IOStream::append(s, literal!(" = "))?;
            s = IOStream::append(s, Binding::toString(&b, &(literal!("")))?)?;
        }
        s = IOStream::append(s, literal!(")"))?;
    }
    if Binding::isBound(&var.binding) {
        s = IOStream::append(s, literal!(" = "))?;
        if printBindingType {
            s = IOStream::append(s, literal!("("))?;
            s = IOStream::append(s, Type::toString(&(Binding::getType(&var.binding)?))?)?;
            s = IOStream::append(s, literal!(") "))?;
        }
        s = IOStream::append(s, Binding::toString(&var.binding, &(literal!("")))?)?;
    }
    Ok(s)
}

pub(crate) fn toFlatStream(
    mut var: &metamodelica::Ref<NFVariable>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut printBindingType: bool,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    s = IOStream::append(s, indent)?;
    s = Attributes::toFlatStream(&var.attributes, var.ty.clone(), s, ComponentRef::isSimple(&var.name))?;
    s = IOStream::append(s, Type::toFlatString(&(Type::arrayElementType(&var.ty)), format)?)?;
    s = IOStream::append(s, literal!(" "))?;
    s = IOStream::append(s, ComponentRef::toFlatString(&var.name, format)?)?;
    dims = Type::arrayDims(var.ty.clone());
    if !((dims).is_empty()) {
        s = IOStream::append(s, Dimension::toFlatStringList(dims, format, literal!(""))?)?;
    }
    if !((var.typeAttributes).is_empty()) {
        s = Component::typeAttrsToFlatStream(var.typeAttributes.clone(), var.ty.clone(), format, s)?;
    } else if !((var.children).is_empty()) {
        s = toFlatStreamModifier(
            &var.children,
            format.moveBindings.clone() || Binding::isBound(&var.binding),
            printBindingType,
            format,
            s,
        )?;
    }
    s = toFlatStreamBinding(var.binding.clone(), printBindingType, format, s)?;
    s = FlatModelicaUtil::appendComment(&var.comment, FlatModelicaUtil::ElementType::COMPONENT.clone(), s)?;
    Ok(s)
}

pub(crate) fn toFlatStreamBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut printBindingType: bool,
    mut format: BaseModelica::OutputFormat,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    if Binding::isBound(&binding) {
        s = IOStream::append(s, literal!(" = "))?;
        if printBindingType {
            s = IOStream::append(s, literal!("("))?;
            s = IOStream::append(s, Type::toFlatString(&(Binding::getType(&binding)?), format)?)?;
            s = IOStream::append(s, literal!(") "))?;
        }
        s = IOStream::append(s, Binding::toFlatString(binding, format, &(literal!("")))?)?;
    }
    Ok(s)
}

pub(crate) fn toFlatStreamModifier(
    mut children: &metamodelica::List<metamodelica::Ref<NFVariable>>,
    mut overwrittenBinding: bool,
    mut printBindingType: bool,
    mut format: BaseModelica::OutputFormat,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut empty: bool = true;
    let mut overwritten_binding: bool;
    let mut ss: IOStream::IOStream;
    let mut src: Binding::Source;
    for mut child in &**children {
        ss = IOStream::create(
            literal!("NFVariable.toFlatStreamModifier"),
            openmodelica_util::IOStream::IOStreamType::LIST,
        )?;
        if !((child.typeAttributes).is_empty()) {
            ss = Component::typeAttrsToFlatStream(child.typeAttributes.clone(), child.ty.clone(), format, ss)?;
        } else if !((child.children).is_empty()) {
            overwritten_binding = overwrittenBinding || Binding::isBound(&child.binding);
            ss = toFlatStreamModifier(&child.children, overwritten_binding, printBindingType, format, ss)?;
        }
        if !(overwrittenBinding) {
            src = Binding::source(&child.binding);
            if src == Binding::Source::MODIFIER.clone() || src == Binding::Source::GENERATED.clone() {
                ss = toFlatStreamBinding(child.binding.clone(), printBindingType, format, ss)?;
            }
        }
        if !(IOStream::empty(&ss)?) {
            if empty {
                s = IOStream::append(s, literal!("("))?;
                empty = false;
            } else {
                s = IOStream::append(s, literal!(", "))?;
            }
            s = IOStream::append(
                s,
                Util::makeQuotedIdentifier(ComponentRef::firstName(&child.name, false)?)?,
            )?;
            s = IOStream::appendListStream(ss, s)?;
        }
    }
    if !(empty) {
        s = IOStream::append(s, literal!(")"))?;
    }
    Ok(s)
}

pub(crate) fn moveBinding(
    mut var: metamodelica::Ref<NFVariable>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<(
    metamodelica::Ref<NFVariable>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
)> {
    let mut var: metamodelica::Ref<NFVariable> = var;
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    if variability(&var) >= Variability::DISCRETE.clone() && Binding::isBound(&var.binding) {
        equations = metamodelica::cons(
            Equation::makeEquality(
                Expression::fromCref(var.name.clone(), false)?,
                Binding::getExp(&var.binding)?,
                var.ty.clone(),
                ElementSource::createElementSource(
                    var.info.clone(),
                    None,
                    &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                    (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
                ),
                crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                Equation::ScalarizeMode::NO_PREFERENCE.clone(),
            ),
            equations,
        );
        assign_field!(var.binding = Binding::EMPTY_BINDING().clone());
    }
    Ok((var, equations))
}

pub fn getVariableAttributes(
    mut var: &metamodelica::Ref<NFVariable>,
) -> metamodelica::Ref<VariableAttributes::VariableAttributes> {
    let mut variableAttributes: metamodelica::Ref<VariableAttributes::VariableAttributes> =
        var.backendinfo.attributes.clone();
    variableAttributes
}

pub(crate) fn getNominal(
    mut var: &metamodelica::Ref<NFVariable>,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut nominal: Option<metamodelica::Ref<Expression::NFExpression>> =
        VariableAttributes::getNominal(&(getVariableAttributes(var)))?;
    Ok(nominal)
}

pub(crate) fn asBinding(
    mut var: &metamodelica::Ref<NFVariable>,
    mut source: Binding::Source,
) -> metamodelica::Ref<Binding::NFBinding> {
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    binding = Binding::makeFlat(
        Expression::fromTypedCref(var.name.clone(), var.ty.clone()),
        variability(var),
        source,
        Binding::NO_CONFIDENCE.clone(),
    );
    binding
}
