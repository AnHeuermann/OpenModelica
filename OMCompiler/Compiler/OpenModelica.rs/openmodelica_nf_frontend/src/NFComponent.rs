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
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRestriction as Restriction;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::SCode::Element;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFComponent {
    COMPONENT_DEF {
        definition: metamodelica::Ref<Element>,
        modifier: metamodelica::Ref<Modifier::Modifier>,
    },
    COMPONENT {
        classInst: metamodelica::Ref<InstNode::InstNode>,
        ty: metamodelica::Ref<Type::NFType>,
        binding: metamodelica::Ref<Binding::NFBinding>,
        condition: metamodelica::Ref<Binding::NFBinding>,
        attributes: metamodelica::Ref<Attributes::NFAttributes>,
        comment: metamodelica::Ref<SCode::Comment>,
        state: ComponentState,
        info: SourceInfo,
    },
    ITERATOR {
        ty: metamodelica::Ref<Type::NFType>,
        variability: Prefixes::Variability,
        info: SourceInfo,
    },
    ENUM_LITERAL {
        literal: metamodelica::Ref<Expression::NFExpression>,
        comment: metamodelica::Ref<SCode::Comment>,
    },
    TYPE_ATTRIBUTE {
        ty: metamodelica::Ref<Type::NFType>,
        modifier: metamodelica::Ref<Modifier::Modifier>,
    },
    INVALID_COMPONENT {
        component: metamodelica::Ref<NFComponent>,
        errors: ArcStr,
    },
    /// needed for new crefs in the backend
    WILD,
}
impl metamodelica::gc::MMTrace for NFComponent {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFComponent::COMPONENT_DEF { definition, modifier } => {
                metamodelica::gc::MMTrace::mm_accept(definition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                Ok(())
            }
            NFComponent::COMPONENT {
                classInst,
                ty,
                binding,
                condition,
                attributes,
                comment,
                state,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(classInst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comment, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(state, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFComponent::ITERATOR { ty, variability, info } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFComponent::ENUM_LITERAL { literal, comment } => {
                metamodelica::gc::MMTrace::mm_accept(literal, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comment, __mmv)?;
                Ok(())
            }
            NFComponent::TYPE_ATTRIBUTE { ty, modifier } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                Ok(())
            }
            NFComponent::INVALID_COMPONENT { component, errors } => {
                metamodelica::gc::MMTrace::mm_accept(component, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(errors, __mmv)?;
                Ok(())
            }
            NFComponent::WILD => Ok(()),
        }
    }
}
impl NFComponent {
    pub fn interned_WILD() -> metamodelica::Ref<NFComponent> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFComponent> = metamodelica::Ref::new(NFComponent::WILD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_WILD() -> metamodelica::Ref<NFComponent> {
    NFComponent::interned_WILD()
}
impl Default for NFComponent {
    fn default() -> Self {
        Self::WILD
    }
}
pub use self::NFComponent::{
    COMPONENT, COMPONENT_DEF, ENUM_LITERAL, INVALID_COMPONENT, ITERATOR, TYPE_ATTRIBUTE, WILD,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum ComponentState {
    /// Component instance has been created
    PartiallyInstantiated = 1,
    /// All component expressions have been instantiated
    FullyInstantiated = 2,
    /// The component's type has been determined
    Typed = 3,
    /// The component's binding has been typed and type checked
    TypeChecked = 4,
}
impl PartialOrd for ComponentState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ComponentState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ComponentState {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn new(mut definition: metamodelica::Ref<Element>) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent>;
    component = metamodelica::Ref::new(NFComponent::COMPONENT_DEF {
        definition: definition,
        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
    });
    component
}

pub(crate) fn newEnum(
    mut enumType: metamodelica::Ref<Type::NFType>,
    mut literalName: ArcStr,
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut literalIndex: i32,
) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent>;
    component = metamodelica::Ref::new(NFComponent::ENUM_LITERAL {
        literal: metamodelica::Ref::new(Expression::NFExpression::ENUM_LITERAL {
            ty: enumType,
            name: literalName,
            index: literalIndex,
        }),
        comment: comment,
    });
    component
}

pub(crate) fn newIterator(
    mut iterType: metamodelica::Ref<Type::NFType>,
    mut info: SourceInfo,
) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent>;
    component = metamodelica::Ref::new(NFComponent::ITERATOR {
        ty: iterType,
        variability: Variability::IMPLICITLY_DISCRETE.clone(),
        info: info,
    });
    component
}

pub(crate) fn definition(mut component: &metamodelica::Ref<NFComponent>) -> Result<metamodelica::Ref<Element>> {
    let mut definition: metamodelica::Ref<Element>;
    let __pa0 = ::match_deref::match_deref! { match &((*component)) {
        Deref @ COMPONENT_DEF { definition: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    definition = metamodelica::Own::own(__pa0);
    Ok(definition)
}

pub(crate) fn isDefinition(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isDefinition: bool;
    isDefinition = (match &**component {
        COMPONENT_DEF { .. } => true,
        _ => false,
    });
    isDefinition
}

pub(crate) fn info(mut component: &metamodelica::Ref<NFComponent>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (match &**component {
        COMPONENT_DEF {
            definition: __component_definition,
            ..
        } => SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&__component_definition)),
        COMPONENT {
            info: __component_info, ..
        } => __component_info.clone(),
        ITERATOR {
            info: __component_info, ..
        } => __component_info.clone(),
        TYPE_ATTRIBUTE {
            modifier: __component_modifier,
            ..
        } => Modifier::info(metamodelica::AsArg::as_arg(&__component_modifier)),
        _ => return Err("match: no arm matched"),
    });
    Ok(info)
}

pub fn classInstance(mut component: &metamodelica::Ref<NFComponent>) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut classInst: metamodelica::Ref<InstNode::InstNode>;
    classInst = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { classInst: __component_classInst, .. } => __component_classInst.clone(),
        Deref @ ITERATOR { ty: Deref @ Type::COMPLEX { .. }, .. } => Type::complexNode(var_field!((**component).ty, NFComponent::ITERATOR))?,
        Deref @ ITERATOR { ty: __component_ty, .. } => metamodelica::Ref::new(InstNode::InstNode::ITERATOR_NODE { exp: metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: __component_ty.clone() }) }),
        _ => crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(classInst)
}

pub(crate) fn setClassInstance(
    mut classInst: metamodelica::Ref<InstNode::InstNode>,
    mut component: metamodelica::Ref<NFComponent>,
) -> Result<metamodelica::Ref<NFComponent>> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (match &*component {
        COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; classInst = classInst);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(component)
}

pub(crate) fn getModifier(mut component: &metamodelica::Ref<NFComponent>) -> metamodelica::Ref<Modifier::Modifier> {
    let mut modifier: metamodelica::Ref<Modifier::Modifier>;
    modifier = (match &**component {
        COMPONENT_DEF {
            modifier: __component_modifier,
            ..
        } => __component_modifier.clone(),
        TYPE_ATTRIBUTE {
            modifier: __component_modifier,
            ..
        } => __component_modifier.clone(),
        _ => crate::NFModifier::Modifier::interned_NOMOD(),
    });
    modifier
}

pub(crate) fn setModifier(
    mut modifier: metamodelica::Ref<Modifier::Modifier>,
    mut component: metamodelica::Ref<NFComponent>,
) -> Result<metamodelica::Ref<NFComponent>> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (match &*component {
        COMPONENT_DEF { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT_DEF; modifier = modifier);
            ()
        }
        TYPE_ATTRIBUTE { .. } => {
            assign_variant_field!(component => NFComponent::TYPE_ATTRIBUTE; modifier = modifier);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(component)
}

pub(crate) fn mergeModifier(
    mut modifier: metamodelica::Ref<Modifier::Modifier>,
    mut component: metamodelica::Ref<NFComponent>,
) -> Result<metamodelica::Ref<NFComponent>> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    component = (match &*component {
        COMPONENT_DEF {
            modifier: __component_modifier,
            ..
        } => {
            assign_variant_field!(component => NFComponent::COMPONENT_DEF; modifier = Modifier::merge(modifier, __component_modifier.clone(), &(literal!("")))?);
            component
        }
        TYPE_ATTRIBUTE {
            modifier: __component_modifier,
            ty: __component_ty,
        } => metamodelica::Ref::new(NFComponent::TYPE_ATTRIBUTE {
            ty: __component_ty.clone(),
            modifier: Modifier::merge(modifier, __component_modifier.clone(), &(literal!("")))?,
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(component)
}

pub fn getType<'__b>(mut component: &'__b metamodelica::Ref<NFComponent>) -> Result<metamodelica::Ref<Type::NFType>> {
    '__tco: loop {
        let mut t: metamodelica::Ref<Type::NFType>;
        ::match_deref::match_deref! { match component {
            Deref @ COMPONENT { ty: __esc_t @ Deref @ Type::UNTYPED { .. }, .. } => {
                t = (*__esc_t).clone();
                return Ok(Type::liftArrayLeftList(InstNode::getType(var_field!((**component).classInst, NFComponent::COMPONENT).clone())?, &(var_field!((*t).dimensions, Type::NFType::UNTYPED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())))
            },
            Deref @ COMPONENT { .. } => return Ok(var_field!((**component).ty, NFComponent::COMPONENT).clone()),
            Deref @ ITERATOR { .. } => return Ok(var_field!((**component).ty, NFComponent::ITERATOR).clone()),
            Deref @ TYPE_ATTRIBUTE { .. } => return Ok(var_field!((**component).ty, NFComponent::TYPE_ATTRIBUTE).clone()),
            Deref @ INVALID_COMPONENT { .. } => { component = var_field!((**component).component, NFComponent::INVALID_COMPONENT); continue '__tco; },
            _ => return Ok(crate::NFType::interned_UNKNOWN()),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn setType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut component: metamodelica::Ref<NFComponent>,
) -> Result<metamodelica::Ref<NFComponent>> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    component = (match &*component {
        COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; ty = ty);
            component
        }
        ITERATOR { .. } => {
            assign_variant_field!(component => NFComponent::ITERATOR; ty = ty);
            component
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(component)
}

pub(crate) fn isTyped(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isTyped: bool;
    isTyped = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { state: __component_state, .. } => __component_state.clone() >= ComponentState::Typed.clone(),
        Deref @ ITERATOR { ty: Deref @ Type::UNKNOWN, .. } => false,
        Deref @ ITERATOR { .. } => true,
        Deref @ TYPE_ATTRIBUTE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTyped
}

pub(crate) fn unliftType(mut component: metamodelica::Ref<NFComponent>) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { ty: Deref @ Type::ARRAY { elementType: ty, .. }, .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; ty = ty.clone());
            ()
        },
        Deref @ ITERATOR { ty: Deref @ Type::ARRAY { elementType: ty, .. }, .. } => {
            assign_variant_field!(component => NFComponent::ITERATOR; ty = ty.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    component
}

pub fn getAttributes(mut component: &metamodelica::Ref<NFComponent>) -> metamodelica::Ref<Attributes::NFAttributes> {
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    attr = (match &**component {
        COMPONENT {
            attributes: __component_attributes,
            ..
        } => __component_attributes.clone(),
        _ => Attributes::DEFAULT_ATTR().clone(),
    });
    attr
}

pub(crate) fn setAttributes(
    mut attr: metamodelica::Ref<Attributes::NFAttributes>,
    mut component: metamodelica::Ref<NFComponent>,
) -> Result<metamodelica::Ref<NFComponent>> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (match &*component {
        COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(component)
}

pub(crate) fn setComment(
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut component: metamodelica::Ref<NFComponent>,
) -> Result<metamodelica::Ref<NFComponent>> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (match &*component {
        COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; comment = comment);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(component)
}

pub fn getBinding(mut component: &metamodelica::Ref<NFComponent>) -> metamodelica::Ref<Binding::NFBinding> {
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    b = (match &**component {
        COMPONENT {
            binding: __component_binding,
            ..
        } => __component_binding.clone(),
        TYPE_ATTRIBUTE {
            modifier: __component_modifier,
            ..
        } => Modifier::binding(metamodelica::AsArg::as_arg(&__component_modifier)),
        WILD { .. } => crate::NFBinding::interned_WILD(),
        _ => Binding::EMPTY_BINDING().clone(),
    });
    b
}

pub(crate) fn getImplicitBinding(
    mut component: &metamodelica::Ref<NFComponent>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut record_exp: metamodelica::Ref<Expression::NFExpression>;
    binding = getBinding(component);
    if Binding::isUnbound(&binding) {
        cls_node = classInstance(component)?;
        if InstNode::isRecord(cls_node.clone())? {
            if '__try0: {
                if isTyped(component) {
                    record_exp = unwrap_break_err!(Class::makeRecordExp(cls_node.clone(), scope.clone(), true), '__try0);
                    binding = unwrap_break_err!(Binding::makeTyped(record_exp.clone(), Binding::EachType::NOT_EACH.clone(), Binding::Source::GENERATED.clone(), unwrap_break_err!(info(component), '__try0), Binding::EvalState::NOT_EVALUATED.clone(), Binding::NO_CONFIDENCE.clone()), '__try0);
                } else {
                    record_exp = unwrap_break_err!(Class::makeRecordExp(cls_node.clone(), scope.clone(), false), '__try0);
                    binding = Binding::makeUntyped(record_exp.clone(), scope.clone(), Binding::EachType::NOT_EACH.clone(), Binding::Source::GENERATED.clone(), unwrap_break_err!(info(component), '__try0), Binding::NO_CONFIDENCE.clone());
                }
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
    }
    Ok(binding)
}

pub(crate) fn getTypeAttributeBinding(
    mut component: &metamodelica::Ref<NFComponent>,
    mut attrName: ArcStr,
) -> metamodelica::Ref<Binding::NFBinding> {
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut start_node: metamodelica::Ref<InstNode::InstNode>;
    let mut start_comp: metamodelica::Ref<NFComponent>;
    match '__try0: {
        (start_node, _) = unwrap_break_err!(Class::lookupElement(attrName.clone(), unwrap_break_err!(InstNode::getClass(unwrap_break_err!(classInstance(component), '__try0)), '__try0)), '__try0);
        start_comp = unwrap_break_err!(InstNode::component(&start_node), '__try0);
        let true = (isTypeAttribute(&start_comp)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        binding = getBinding(&start_comp);
        Ok::<_, &'static str>((binding.clone(),))
    } {
        Ok((__try0_o0,)) => {
            binding = __try0_o0;
        }
        Err(_) => {
            binding = Binding::EMPTY_BINDING().clone();
        }
    }
    binding
}

pub(crate) fn setBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut component: metamodelica::Ref<NFComponent>,
) -> Result<metamodelica::Ref<NFComponent>> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (match &*component {
        COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; binding = binding);
            ()
        }
        TYPE_ATTRIBUTE {
            modifier: __component_modifier,
            ..
        } => {
            assign_variant_field!(component => NFComponent::TYPE_ATTRIBUTE; modifier = Modifier::setBinding(binding, __component_modifier.clone())?);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(component)
}

pub(crate) fn hasBinding(
    mut component: &metamodelica::Ref<NFComponent>,
    mut parent: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    fn has_missing_binding(mut component: &metamodelica::Ref<InstNode::InstNode>) -> Result<bool> {
        let mut noBinding: bool;
        noBinding = InstNode::isComponent(component)?
            && !(hasBinding(
                &(InstNode::component(component)?),
                &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()),
            )?);
        Ok(noBinding)
    }

    let mut b: bool;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    if Binding::isBound(&(getBinding(component))) {
        b = true;
        return Ok(b);
    }
    cls = InstNode::getClass(classInstance(component)?)?;
    if !(Restriction::isRecord(&(Class::restriction(&cls)))) {
        b = false;
        return Ok(b);
    }
    if (ClassTree::findComponent(&(Class::classTree(cls)?), &move |__a0: metamodelica::Ref<
        InstNode::InstNode,
    >| has_missing_binding(&__a0))?)
    .is_some()
    {
        b = false;
    }
    b = true;
    Ok(b)
}

pub(crate) fn getCondition(mut component: &metamodelica::Ref<NFComponent>) -> metamodelica::Ref<Binding::NFBinding> {
    let mut cond: metamodelica::Ref<Binding::NFBinding>;
    cond = (match &**component {
        COMPONENT {
            condition: __component_condition,
            ..
        } => __component_condition.clone(),
        _ => Binding::EMPTY_BINDING().clone(),
    });
    cond
}

pub(crate) fn hasCondition(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut b: bool;
    b = Binding::isBound(&(getCondition(component)));
    b
}

pub(crate) fn direction(mut component: &metamodelica::Ref<NFComponent>) -> Prefixes::Direction {
    let mut direction: Prefixes::Direction;
    direction = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { direction: __esc_direction, .. }, .. } => {
            direction = (*__esc_direction).clone();
            direction.clone()
        },
        _ => Direction::NONE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    direction
}

pub(crate) fn isInput(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isInput: bool = direction(component) == Direction::INPUT.clone();
    isInput
}

pub(crate) fn setDirection(
    mut direction: Prefixes::Direction,
    mut component: metamodelica::Ref<NFComponent>,
) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let () = (match &*component {
        COMPONENT {
            attributes: __esc_attr, ..
        } => {
            attr = (*__esc_attr).clone();
            assign_field!(attr.direction = direction);
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        }
        _ => (),
    });
    component
}

pub(crate) fn isOutput(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isOutput: bool = direction(component) == Direction::OUTPUT.clone();
    isOutput
}

pub(crate) fn parallelism(mut component: &metamodelica::Ref<NFComponent>) -> Prefixes::Parallelism {
    let mut parallelism: Prefixes::Parallelism;
    parallelism = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { parallelism: __esc_parallelism, .. }, .. } => {
            parallelism = (*__esc_parallelism).clone();
            parallelism.clone()
        },
        _ => Parallelism::NON_PARALLEL.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    parallelism
}

pub(crate) fn variability<'__b>(mut component: &'__b metamodelica::Ref<NFComponent>) -> Result<Prefixes::Variability> {
    let mut variability: Prefixes::Variability;
    variability = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { variability: __esc_variability, .. }, .. } => {
            variability = (*__esc_variability).clone();
            variability.clone()
        },
        Deref @ ITERATOR { .. } => var_field!((**component).variability, NFComponent::ITERATOR).clone(),
        Deref @ ENUM_LITERAL { .. } => Variability::CONSTANT.clone(),
        Deref @ INVALID_COMPONENT { .. } => self::variability(var_field!((**component).component, NFComponent::INVALID_COMPONENT))?,
        _ => Variability::CONTINUOUS.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(variability)
}

pub(crate) fn setVariability(
    mut variability: Prefixes::Variability,
    mut component: metamodelica::Ref<NFComponent>,
) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (match &*component {
        COMPONENT { attributes: attr, .. } => {
            let mut attr = (*attr).clone();
            assign_field!(attr.variability = variability);
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        }
        _ => (),
    });
    component
}

pub(crate) fn isConst(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isConst: bool = variability(component)? == Variability::CONSTANT.clone();
    Ok(isConst)
}

pub(crate) fn isParameter(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut b: bool = variability(component)? == Variability::PARAMETER.clone();
    Ok(b)
}

pub(crate) fn isStructuralParameter(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut b: bool = variability(component)? == Variability::STRUCTURAL_PARAMETER.clone();
    Ok(b)
}

pub(crate) fn isVar(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isVar: bool = variability(component)? == Variability::CONTINUOUS.clone();
    Ok(isVar)
}

pub(crate) fn isRedeclare(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isRedeclare: bool;
    isRedeclare = (match &**component {
        COMPONENT_DEF {
            definition: __component_definition,
            ..
        } => SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&__component_definition))?,
        _ => false,
    });
    Ok(isRedeclare)
}

pub(crate) fn isFinal(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isFinal: bool;
    isFinal = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT_DEF { definition: __component_definition, .. } => SCodeUtil::finalBool(SCodeUtil::prefixesFinal(&(SCodeUtil::elementPrefixes(metamodelica::AsArg::as_arg(&__component_definition))?))),
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { isFinal: __esc_isFinal, .. }, .. } => {
            isFinal = (*__esc_isFinal).clone();
            isFinal.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isFinal)
}

pub(crate) fn setFinal(
    mut component: metamodelica::Ref<NFComponent>,
    mut isFinal: bool,
) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let () = (match &*component {
        COMPONENT {
            attributes: __esc_attr, ..
        } => {
            attr = (*__esc_attr).clone();
            assign_field!(attr.isFinal = isFinal);
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        }
        _ => (),
    });
    component
}

pub(crate) fn isResizable(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { isResizable: __esc_b, .. }, .. } => {
            b = (*__esc_b).clone();
            b.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn innerOuter(mut component: &metamodelica::Ref<NFComponent>) -> Result<Prefixes::InnerOuter> {
    let mut io: Prefixes::InnerOuter;
    io = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { innerOuter: __esc_io, .. }, .. } => {
            io = (*__esc_io).clone();
            io.clone()
        },
        Deref @ COMPONENT_DEF { definition: __component_definition, .. } => Prefixes::innerOuterFromSCode(SCodeUtil::prefixesInnerOuter(&(SCodeUtil::elementPrefixes(metamodelica::AsArg::as_arg(&__component_definition))?))),
        _ => InnerOuter::NOT_INNER_OUTER.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(io)
}

pub(crate) fn isInnerOuter(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isInnerOuter: bool;
    isInnerOuter = innerOuter(component)? != InnerOuter::NOT_INNER_OUTER.clone();
    Ok(isInnerOuter)
}

pub(crate) fn isInner(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isInner: bool;
    let mut io: Prefixes::InnerOuter = innerOuter(component)?;
    isInner = io == InnerOuter::INNER.clone() || io == InnerOuter::INNER_OUTER.clone();
    Ok(isInner)
}

pub(crate) fn isOuter(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isOuter: bool;
    let mut io: Prefixes::InnerOuter = innerOuter(component)?;
    isOuter = io == InnerOuter::OUTER.clone() || io == InnerOuter::INNER_OUTER.clone();
    Ok(isOuter)
}

pub(crate) fn isOnlyOuter(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isOuter: bool = innerOuter(component)? == InnerOuter::OUTER.clone();
    Ok(isOuter)
}

pub(crate) fn connectorType(mut component: &metamodelica::Ref<NFComponent>) -> i32 {
    let mut cty: i32;
    cty = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { connectorType: __esc_cty, .. }, .. } => {
            cty = (*__esc_cty).clone();
            cty.clone()
        },
        _ => ConnectorType::NON_CONNECTOR.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cty
}

pub(crate) fn setConnectorType(
    mut cty: i32,
    mut component: metamodelica::Ref<NFComponent>,
) -> metamodelica::Ref<NFComponent> {
    let mut component: metamodelica::Ref<NFComponent> = component;
    let () = (match &*component {
        COMPONENT { attributes: attr, .. } => {
            let mut attr = (*attr).clone();
            assign_field!(attr.connectorType = cty);
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        }
        _ => (),
    });
    component
}

pub(crate) fn isFlow(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isFlow: bool = Prefixes::ConnectorType::isFlow(connectorType(component));
    isFlow
}

pub(crate) fn isConnector(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isConnector: bool = Prefixes::ConnectorType::isConnectorType(connectorType(component));
    isConnector
}

pub(crate) fn isExpandableConnector(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isConnector: bool = Prefixes::ConnectorType::isExpandable(connectorType(component));
    isConnector
}

pub(crate) fn isExternalObject(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isEO: bool;
    isEO = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT { ty: Deref @ Type::UNTYPED { .. }, classInst: __component_classInst, .. } => Class::isExternalObject(&(InstNode::getClass(__component_classInst.clone())?)),
        Deref @ COMPONENT { ty: __component_ty, .. } => Type::isExternalObject(metamodelica::AsArg::as_arg(&__component_ty)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEO)
}

pub(crate) fn isIdentical(
    mut comp1: &metamodelica::Ref<NFComponent>,
    mut comp2: &metamodelica::Ref<NFComponent>,
) -> Result<bool> {
    let mut identical: bool = false;
    if referenceEq(&*(&**comp1), &*(&**comp2)) {
        identical = true;
    } else {
        identical = (::match_deref::match_deref! { match (comp1, comp2) {
            (Deref @ COMPONENT { .. }, Deref @ COMPONENT { .. }) => {
                if !(Class::isIdentical(&(InstNode::getClass(var_field!((**comp1).classInst, NFComponent::COMPONENT).clone())?), &(InstNode::getClass(var_field!((**comp2).classInst, NFComponent::COMPONENT).clone())?))?) {
                    return Ok(identical);
                }
                if !(Binding::isEqual(var_field!((**comp1).binding, NFComponent::COMPONENT), var_field!((**comp2).binding, NFComponent::COMPONENT))?) {
                    return Ok(identical);
                }
                true
            },
            _ => true,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(identical)
}

pub(crate) fn toString(mut name: &ArcStr, mut component: &metamodelica::Ref<NFComponent>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match component {
        Deref @ COMPONENT_DEF { definition: def @ Deref @ SCode::Element::COMPONENT { .. }, .. } => {
            SCodeDump::unparseElementStr(def.clone(), SCodeDump::defaultOptions.clone())?
        },
        Deref @ COMPONENT { attributes: __component_attributes, binding: __component_binding, ty: __component_ty, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Attributes::toString(metamodelica::AsArg::as_arg(&__component_attributes), __component_ty.clone())?); __mm_s.push_str(&*Type::toString(metamodelica::AsArg::as_arg(&__component_ty))?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*name); __mm_s.push_str(&*Binding::toString(metamodelica::AsArg::as_arg(&__component_binding), &(literal!(" = ")))?); ArcStr::from(__mm_s) }
        },
        Deref @ TYPE_ATTRIBUTE { modifier: __component_modifier, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*Modifier::toString(metamodelica::AsArg::as_arg(&__component_modifier), false)?); ArcStr::from(__mm_s) }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

pub(crate) fn toFlatStream(
    mut name: ArcStr,
    mut component: &metamodelica::Ref<NFComponent>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut ty_attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let () = (match &**component {
        COMPONENT {
            attributes: __component_attributes,
            binding: __component_binding,
            classInst: __component_classInst,
            ty: __component_ty,
            ..
        } => {
            s = IOStream::append(s, indent)?;
            s = Attributes::toFlatStream(
                metamodelica::AsArg::as_arg(&__component_attributes),
                __component_ty.clone(),
                s,
                true,
            )?;
            s = IOStream::append(
                s,
                Type::toFlatString(
                    &(Type::arrayElementType(metamodelica::AsArg::as_arg(&__component_ty))),
                    format,
                )?,
            )?;
            s = IOStream::append(s, literal!(" "))?;
            s = IOStream::append(s, Util::makeQuotedIdentifier(name)?)?;
            dims = Type::arrayDims(__component_ty.clone());
            if !((dims).is_empty()) {
                s = IOStream::append(s, Dimension::toFlatStringList(dims, format, literal!(""))?)?;
            }
            ty_attrs = ({
                let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> =
                    metamodelica::nil();
                for mut a in (Class::getTypeAttributes(InstNode::getClass(__component_classInst.clone())?))
                    .into_iter()
                    .cloned()
                {
                    let __x = (Modifier::name(&(a.clone()))?, Modifier::binding(&(a.clone())));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            s = typeAttrsToFlatStream(ty_attrs, __component_ty.clone(), format, s)?;
            s = IOStream::append(
                s,
                Binding::toFlatString(__component_binding.clone(), format, &(literal!(" = ")))?,
            )?;
            ()
        }
        TYPE_ATTRIBUTE {
            modifier: __component_modifier,
            ..
        } => {
            s = IOStream::append(s, name)?;
            s = IOStream::append(
                s,
                Modifier::toFlatString(metamodelica::AsArg::as_arg(&__component_modifier), format, false)?,
            )?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(s)
}

pub(crate) fn typeAttrsToFlatStream(
    mut typeAttrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    mut componentType: metamodelica::Ref<Type::NFType>,
    mut format: BaseModelica::OutputFormat,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut var_dims: i32;
    let mut binding_dims: i32;
    let mut ty_attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = typeAttrs;
    let mut name: ArcStr;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut bind_exp: metamodelica::Ref<Expression::NFExpression>;
    if (ty_attrs).is_empty() {
        return Ok(s);
    }
    s = IOStream::append(s, literal!("("))?;
    var_dims = Type::dimensionCount(componentType);
    loop {
        (name, binding) = (ty_attrs).head().cloned()?;
        bind_exp = Expression::expandSplitIndices(Binding::getExp(&binding)?)?;
        binding_dims = Type::dimensionCount(Expression::typeOf(bind_exp.clone()));
        if var_dims > binding_dims {
            s = IOStream::append(s, literal!("each "))?;
        }
        s = IOStream::append(s, name)?;
        s = IOStream::append(s, literal!(" = "))?;
        s = IOStream::append(s, Expression::toFlatString(bind_exp, format)?)?;
        if format.showConfidence.clone() {
            s = IOStream::append(s, literal!(" /* confidence = "))?;
            s = IOStream::append(
                s,
                ArcStr::from(::std::format!("{}", Binding::actualConfidence(binding)?)),
            )?;
            s = IOStream::append(s, literal!("*/"))?;
        }
        ty_attrs = (ty_attrs).rest()?;
        if (ty_attrs).is_empty() {
            break;
        } else {
            s = IOStream::append(s, literal!(", "))?;
        }
    }
    s = IOStream::append(s, literal!(")"))?;
    Ok(s)
}

pub(crate) fn toFlatString(
    mut name: ArcStr,
    mut component: &metamodelica::Ref<NFComponent>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(name.clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toFlatStream(name, component, format, indent, s)?;
    r#str = IOStream::string(&s)?;
    IOStream::delete(&s)?;
    Ok(r#str)
}

pub(crate) fn dimensionCount(mut component: &metamodelica::Ref<NFComponent>) -> i32 {
    let mut count: i32;
    count = (match &**component {
        COMPONENT { ty: __component_ty, .. } => Type::dimensionCount(__component_ty.clone()),
        _ => 0,
    });
    count
}

pub fn comment(mut component: &metamodelica::Ref<NFComponent>) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut comment: metamodelica::Ref<SCode::Comment>;
    comment = (match &**component {
        COMPONENT_DEF {
            definition: __component_definition,
            ..
        } => SCodeUtil::getElementComment(metamodelica::AsArg::as_arg(&__component_definition))
            .ok_or("pattern mismatch")?,
        COMPONENT {
            comment: __component_comment,
            ..
        } => __component_comment.clone(),
        ENUM_LITERAL {
            comment: __component_comment,
            ..
        } => __component_comment.clone(),
        _ => SCode::noComment.clone(),
    });
    Ok(comment)
}

pub(crate) fn getEvaluateAnnotation(mut component: &metamodelica::Ref<NFComponent>) -> Result<Option<bool>> {
    let mut evaluate: Option<bool>;
    evaluate = SCodeUtil::getEvaluateAnnotation(&(comment(component)?));
    Ok(evaluate)
}

pub(crate) fn isFixed(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut fixed: bool;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    fixed = isParameter(component)? || isStructuralParameter(component)?;
    binding = Class::lookupAttributeBinding(literal!("fixed"), InstNode::getClass(classInstance(component)?)?);
    if Binding::isUnbound(&binding) {
        return Ok(fixed);
    }
    if Binding::hasExp(&binding) {
        fixed = fixed && Expression::isTrue(&(Binding::getExp(&binding)?));
    } else {
        fixed = (::match_deref::match_deref! { match &(binding) {
            Deref @ Binding::RAW_BINDING { bindingExp: Deref @ Absyn::Exp::BOOL { value: true }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(fixed)
}

pub(crate) fn getUnitAttribute(
    mut component: &metamodelica::Ref<NFComponent>,
    mut defaultUnit: ArcStr,
) -> Result<ArcStr> {
    let mut unitString: ArcStr;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut unit: metamodelica::Ref<Expression::NFExpression>;
    binding = Class::lookupAttributeBinding(literal!("unit"), InstNode::getClass(classInstance(component)?)?);
    if Binding::isUnbound(&binding) {
        unitString = defaultUnit;
        return Ok(unitString);
    }
    unit = Binding::getExp(&binding)?;
    unitString = (match &*unit {
        Expression::STRING { value: __unit_value } => __unit_value.clone(),
        _ => defaultUnit,
    });
    Ok(unitString)
}

pub fn isDeleted(mut component: &metamodelica::Ref<NFComponent>) -> Result<bool> {
    let mut isDeleted: bool;
    isDeleted = (match &**component {
        COMPONENT { condition, .. } => {
            Binding::isTyped(condition) && Expression::isFalse(&(Binding::getTypedExp(condition)?))
        }
        _ => false,
    });
    Ok(isDeleted)
}

pub(crate) fn isInvalid(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut invalid: bool;
    invalid = (match &**component {
        INVALID_COMPONENT { .. } => true,
        _ => false,
    });
    invalid
}

pub(crate) fn isIterator(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut iterator: bool;
    iterator = (match &**component {
        ITERATOR { .. } => true,
        _ => false,
    });
    iterator
}

pub(crate) fn isTypeAttribute(mut component: &metamodelica::Ref<NFComponent>) -> bool {
    let mut isAttribute: bool;
    isAttribute = (match &**component {
        TYPE_ATTRIBUTE { .. } => true,
        _ => false,
    });
    isAttribute
}

pub(crate) fn countConnectorVars(
    mut component: &metamodelica::Ref<NFComponent>,
    mut isRoot: bool,
) -> Result<(i32, i32, i32, bool)> {
    let mut potentials: i32 = 0;
    let mut flows: i32 = 0;
    let mut streams: i32 = 0;
    let mut knownSize: bool = true;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cty: i32;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut eq_node_opt: Option<metamodelica::Ref<InstNode::InstNode>>;
    let mut eq_node: metamodelica::Ref<InstNode::InstNode>;
    let mut comp_size: i32 = 0;
    let mut p: i32;
    let mut f: i32;
    let mut s: i32;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut known_size: bool;
    cls = InstNode::getClass(classInstance(component)?)?;
    (eq_node_opt, _) = Class::tryLookupElement(literal!("equalityConstraint"), cls.clone());
    if (eq_node_opt).is_some()
        && SCodeUtil::isFunction(&(InstNode::definition(eq_node_opt.clone().ok_or("pattern mismatch")?)?))
    {
        let __pa0 = ::match_deref::match_deref! { match &(eq_node_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        eq_node = metamodelica::Own::own(__pa0);
        Function::instFunctionNode(eq_node.clone(), NFInstContext::NO_CONTEXT.clone(), info(component)?)?;
        r#fn = (Function::typeNodeCache(eq_node, NFInstContext::FUNCTION.clone())?)
            .head()
            .cloned()?;
        ty = Function::returnType(&r#fn);
        if Type::hasKnownSize(ty.clone())? {
            comp_size = Type::sizeOf(&ty, false)?;
        } else {
            comp_size = 0;
            knownSize = false;
        }
    } else {
        ty = getType(component)?;
        if isRoot {
            comp_size = 1;
        } else if Type::hasKnownSize(ty.clone())? {
            comp_size = Dimension::sizesProduct(Type::arrayDims(ty.clone()), false)?;
        } else {
            comp_size = 0;
            knownSize = false;
        }
        ty = Type::arrayElementType(&ty);
        if Type::isComplex(&ty) {
            if Type::isRecord(&ty) || isRoot {
                let __range1 = ClassTree::getComponents(&(Class::classTree(cls)?))?
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range1 {
                    (p, f, s, known_size) = countConnectorVars(&(InstNode::component(&c)?), false)?;
                    potentials = potentials + p * comp_size;
                    flows = flows + f * comp_size;
                    streams = streams + s * comp_size;
                    knownSize = known_size && knownSize;
                }
            }
            comp_size = 0;
        }
    }
    if comp_size > 0 {
        cty = connectorType(component);
        if Prefixes::ConnectorType::isFlow(cty) {
            flows = flows + comp_size;
        } else if Prefixes::ConnectorType::isStream(cty) {
            streams = streams + comp_size;
        } else if variability(component)? >= Variability::DISCRETE.clone()
            && direction(component) == Direction::NONE.clone()
        {
            potentials = potentials + comp_size;
        }
    }
    Ok((potentials, flows, streams, knownSize))
}
