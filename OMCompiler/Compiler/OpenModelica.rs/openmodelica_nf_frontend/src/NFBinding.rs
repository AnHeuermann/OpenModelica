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
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFBinding {
    UNBOUND,
    RAW_BINDING {
        bindingExp: metamodelica::Ref<Absyn::Exp>,
        /// Weakly: the scope owns the class this binding
        ///      sits in.
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        eachType: EachType,
        source: Source,
        confidence: i32,
        info: SourceInfo,
    },
    UNTYPED_BINDING {
        bindingExp: metamodelica::Ref<Expression::NFExpression>,
        isProcessing: bool,
        /// See RAW_BINDING.scope.
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        eachType: EachType,
        source: Source,
        confidence: i32,
        info: SourceInfo,
    },
    TYPED_BINDING {
        bindingExp: metamodelica::Ref<Expression::NFExpression>,
        bindingType: metamodelica::Ref<Type::NFType>,
        variability: Variability,
        purity: Purity,
        eachType: EachType,
        evalState: Mutable::Mutable<EvalState>,
        isFlattened: bool,
        source: Source,
        confidence: i32,
        info: SourceInfo,
    },
    FLAT_BINDING {
        bindingExp: metamodelica::Ref<Expression::NFExpression>,
        variability: Variability,
        source: Source,
        confidence: i32,
    },
    /// Used by the constant evaluation for generated bindings (e.g. record
    ///     bindings constructed from the record fields) that should be discarded
    ///     during flattening.
    CEVAL_BINDING {
        bindingExp: metamodelica::Ref<Expression::NFExpression>,
    },
    INVALID_BINDING {
        binding: metamodelica::Ref<NFBinding>,
        errors: metamodelica::List<ErrorTypes::TotalMessage>,
    },
    WILD,
}
impl metamodelica::gc::MMTrace for NFBinding {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFBinding::UNBOUND => Ok(()),
            NFBinding::RAW_BINDING {
                bindingExp,
                scope,
                subs,
                eachType,
                source,
                confidence,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(confidence, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFBinding::UNTYPED_BINDING {
                bindingExp,
                isProcessing,
                scope,
                eachType,
                source,
                confidence,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProcessing, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(confidence, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFBinding::TYPED_BINDING {
                bindingExp,
                bindingType,
                variability,
                purity,
                eachType,
                evalState,
                isFlattened,
                source,
                confidence,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(bindingType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(purity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(evalState, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFlattened, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(confidence, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFBinding::FLAT_BINDING {
                bindingExp,
                variability,
                source,
                confidence,
            } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(confidence, __mmv)?;
                Ok(())
            }
            NFBinding::CEVAL_BINDING { bindingExp } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                Ok(())
            }
            NFBinding::INVALID_BINDING { binding, errors } => {
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(errors, __mmv)?;
                Ok(())
            }
            NFBinding::WILD => Ok(()),
        }
    }
}
impl NFBinding {
    pub fn interned_UNBOUND() -> metamodelica::Ref<NFBinding> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFBinding> = metamodelica::Ref::new(NFBinding::UNBOUND);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_WILD() -> metamodelica::Ref<NFBinding> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFBinding> = metamodelica::Ref::new(NFBinding::WILD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_UNBOUND() -> metamodelica::Ref<NFBinding> {
    NFBinding::interned_UNBOUND()
}
pub fn interned_WILD() -> metamodelica::Ref<NFBinding> {
    NFBinding::interned_WILD()
}
impl Default for NFBinding {
    fn default() -> Self {
        Self::UNBOUND
    }
}
pub use self::NFBinding::{
    CEVAL_BINDING, FLAT_BINDING, INVALID_BINDING, RAW_BINDING, TYPED_BINDING, UNBOUND, UNTYPED_BINDING, WILD,
};
thread_local! { static __EMPTY_BINDING_TLS: metamodelica::Ref<NFBinding> = crate::NFBinding::interned_UNBOUND(); }
pub fn EMPTY_BINDING() -> metamodelica::Ref<NFBinding> {
    __EMPTY_BINDING_TLS.with(|__t| __t.clone())
}

pub const NO_CONFIDENCE: i32 = 99999;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum EachType {
    NOT_EACH = 1,
    EACH = 2,
}
impl PartialOrd for EachType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EachType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for EachType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum EvalState {
    NOT_EVALUATED = 1,
    EVALUATING = 2,
    EVALUATED = 3,
}
impl PartialOrd for EvalState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EvalState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for EvalState {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Source {
    /// The binding comes from a binding equation.
    BINDING = 1,
    /// The binding is an attribute modifier of a type, see Inst.markTypeModifier.
    TYPE = 2,
    /// The binding comes from a modifier.
    MODIFIER = 3,
    /// The binding was generated by the frontend.
    GENERATED = 4,
}
impl PartialOrd for Source {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Source {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Source {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Source {
    fn default() -> Self {
        Self::BINDING
    }
}

pub(crate) fn fromAbsyn(
    mut bindingExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut eachPrefix: bool,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut instanceLevel: i32,
    mut info: SourceInfo,
) -> metamodelica::Ref<NFBinding> {
    let mut binding: metamodelica::Ref<NFBinding>;
    binding = (::match_deref::match_deref! { match &(bindingExp) {
        Some(exp) => {
            let mut each_ty: EachType;
            each_ty = if (eachPrefix) {EachType::EACH.clone()} else {EachType::NOT_EACH.clone()};
            metamodelica::Ref::new(NFBinding::RAW_BINDING { bindingExp: exp.clone(), scope: NFInstNode::InstNode::scopeRef(scope), subs: metamodelica::nil(), eachType: each_ty, source: Source::BINDING.clone(), confidence: instanceLevel, info: info })
        },
        _ => {
            EMPTY_BINDING().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

pub fn isBound(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isBound: bool;
    isBound = (match &**binding {
        UNBOUND { .. } => false,
        INVALID_BINDING { .. } => false,
        _ => true,
    });
    isBound
}

pub fn isExplicitlyBound(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isBound: bool;
    isBound = (match &**binding {
        UNBOUND { .. } => false,
        CEVAL_BINDING { .. } => false,
        INVALID_BINDING { .. } => false,
        _ => true,
    });
    isBound
}

pub(crate) fn isUnbound(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isUnbound: bool;
    isUnbound = (match &**binding {
        UNBOUND { .. } => true,
        _ => false,
    });
    isUnbound
}

pub(crate) fn isInvalid(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isInvalid: bool;
    isInvalid = (match &**binding {
        INVALID_BINDING { .. } => true,
        _ => false,
    });
    isInvalid
}

pub fn typedExp(mut binding: &metamodelica::Ref<NFBinding>) -> Option<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    exp = (match &**binding {
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Some(__binding_bindingExp.clone()),
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Some(__binding_bindingExp.clone()),
        _ => None,
    });
    exp
}

pub(crate) fn getUntypedExp(
    mut binding: &metamodelica::Ref<NFBinding>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let __pa0 = ::match_deref::match_deref! { match &((*binding)) {
        Deref @ UNTYPED_BINDING { bindingExp: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    Ok(exp)
}

pub fn getTypedExp(mut binding: &metamodelica::Ref<NFBinding>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &**binding {
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => __binding_bindingExp.clone(),
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => __binding_bindingExp.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(exp)
}

pub(crate) fn setTypedExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut binding: metamodelica::Ref<NFBinding>,
) -> Result<metamodelica::Ref<NFBinding>> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let () = (match &*binding {
        TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = exp);
            ()
        }
        FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = exp);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(binding)
}

pub(crate) fn hasExp(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut hasExp: bool;
    hasExp = (match &**binding {
        UNTYPED_BINDING { .. } => true,
        TYPED_BINDING { .. } => true,
        FLAT_BINDING { .. } => true,
        _ => false,
    });
    hasExp
}

pub fn getExp(mut binding: &metamodelica::Ref<NFBinding>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &**binding {
        UNTYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => __binding_bindingExp.clone(),
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => __binding_bindingExp.clone(),
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => __binding_bindingExp.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(exp)
}

pub fn getExpOpt(mut binding: &metamodelica::Ref<NFBinding>) -> Option<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    exp = (match &**binding {
        UNTYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Some(__binding_bindingExp.clone()),
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Some(__binding_bindingExp.clone()),
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Some(__binding_bindingExp.clone()),
        _ => None,
    });
    exp
}

pub fn setExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut binding: metamodelica::Ref<NFBinding>,
) -> Result<metamodelica::Ref<NFBinding>> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let () = (match &*binding {
        UNTYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = exp);
            ()
        }
        TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = exp);
            ()
        }
        FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = exp);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(binding)
}

pub(crate) fn isRecordExp(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isRecordExp: bool;
    isRecordExp = (match &**binding {
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::isRecord(metamodelica::AsArg::as_arg(&__binding_bindingExp)),
        _ => false,
    });
    isRecordExp
}

pub(crate) fn isCrefExp(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isCref: bool;
    isCref = (match &**binding {
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::isCref(metamodelica::AsArg::as_arg(&__binding_bindingExp)),
        _ => false,
    });
    isCref
}

pub(crate) fn recordFieldBinding(
    mut fieldNode: &metamodelica::Ref<InstNode::InstNode>,
    mut recordBinding: metamodelica::Ref<NFBinding>,
) -> Result<metamodelica::Ref<NFBinding>> {
    let mut fieldBinding: metamodelica::Ref<NFBinding> = recordBinding;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut field_name: ArcStr = NFInstNode::InstNode::name(fieldNode)?;
    fieldBinding = (match &*fieldBinding {
        UNTYPED_BINDING {
            bindingExp: __fieldBinding_bindingExp,
            ..
        } => {
            assign_variant_field!(fieldBinding => NFBinding::UNTYPED_BINDING; bindingExp = Expression::recordElement(&field_name, metamodelica::AsArg::as_arg(&__fieldBinding_bindingExp))?);
            fieldBinding
        }
        TYPED_BINDING {
            bindingExp: __fieldBinding_bindingExp,
            confidence: __fieldBinding_confidence,
            eachType: __fieldBinding_eachType,
            evalState: __fieldBinding_evalState,
            info: __fieldBinding_info,
            isFlattened: __fieldBinding_isFlattened,
            source: __fieldBinding_source,
            ..
        } => {
            exp = Expression::recordElement(&field_name, metamodelica::AsArg::as_arg(&__fieldBinding_bindingExp))?;
            ty = Expression::typeOf(exp.clone());
            purity = Expression::purity(exp.clone())?;
            var = Expression::variability(exp.clone())?;
            metamodelica::Ref::new(NFBinding::TYPED_BINDING {
                bindingExp: exp,
                bindingType: ty,
                variability: var,
                purity: purity,
                eachType: __fieldBinding_eachType.clone(),
                evalState: __fieldBinding_evalState.clone(),
                isFlattened: __fieldBinding_isFlattened.clone(),
                source: __fieldBinding_source.clone(),
                confidence: __fieldBinding_confidence.clone(),
                info: __fieldBinding_info.clone(),
            })
        }
        FLAT_BINDING {
            bindingExp: __fieldBinding_bindingExp,
            confidence: __fieldBinding_confidence,
            source: __fieldBinding_source,
            ..
        } => {
            exp = Expression::recordElement(&field_name, metamodelica::AsArg::as_arg(&__fieldBinding_bindingExp))?;
            var = Expression::variability(exp.clone())?;
            metamodelica::Ref::new(NFBinding::FLAT_BINDING {
                bindingExp: exp,
                variability: var,
                source: __fieldBinding_source.clone(),
                confidence: __fieldBinding_confidence.clone(),
            })
        }
        CEVAL_BINDING {
            bindingExp: __fieldBinding_bindingExp,
        } => {
            assign_variant_field!(fieldBinding => NFBinding::CEVAL_BINDING; bindingExp = Expression::recordElement(&field_name, metamodelica::AsArg::as_arg(&__fieldBinding_bindingExp))?);
            fieldBinding
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(fieldBinding)
}

pub fn variability(mut binding: &metamodelica::Ref<NFBinding>) -> Result<Variability> {
    let mut var: Variability;
    var = (match &**binding {
        TYPED_BINDING {
            variability: __binding_variability,
            ..
        } => __binding_variability.clone(),
        FLAT_BINDING {
            variability: __binding_variability,
            ..
        } => __binding_variability.clone(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFBinding.variability"));
                    __mm_s.push_str(&*literal!(" got unknown binding"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFBinding.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(var)
}

pub(crate) fn setVariability(
    mut var: Variability,
    mut binding: metamodelica::Ref<NFBinding>,
) -> metamodelica::Ref<NFBinding> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let () = (match &*binding {
        TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; variability = var);
            ()
        }
        FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; variability = var);
            ()
        }
        _ => (),
    });
    binding
}

pub fn purity(mut binding: &metamodelica::Ref<NFBinding>) -> Purity {
    let mut purity: Purity;
    purity = (match &**binding {
        TYPED_BINDING {
            purity: __binding_purity,
            ..
        } => __binding_purity.clone(),
        _ => Purity::PURE.clone(),
    });
    purity
}

pub(crate) fn getInfo(mut binding: &metamodelica::Ref<NFBinding>) -> SourceInfo {
    let mut info: SourceInfo;
    info = (match &**binding {
        RAW_BINDING {
            info: __binding_info, ..
        } => __binding_info.clone(),
        UNTYPED_BINDING {
            info: __binding_info, ..
        } => __binding_info.clone(),
        TYPED_BINDING {
            info: __binding_info, ..
        } => __binding_info.clone(),
        _ => Absyn::dummyInfo.clone(),
    });
    info
}

pub(crate) fn getType<'__b>(
    mut binding: &'__b metamodelica::Ref<NFBinding>,
) -> Result<metamodelica::Ref<Type::NFType>> {
    '__tco: loop {
        match &**binding {
            UNBOUND { .. } => return Ok(crate::NFType::interned_UNKNOWN()),
            RAW_BINDING { .. } => return Ok(crate::NFType::interned_UNKNOWN()),
            UNTYPED_BINDING { .. } => return Ok(crate::NFType::interned_UNKNOWN()),
            TYPED_BINDING { .. } => return Ok(var_field!((**binding).bindingType, NFBinding::TYPED_BINDING).clone()),
            FLAT_BINDING { .. } => {
                return Ok(Expression::typeOf(
                    var_field!((**binding).bindingExp, NFBinding::FLAT_BINDING).clone(),
                ));
            }
            CEVAL_BINDING { .. } => {
                return Ok(Expression::typeOf(
                    var_field!((**binding).bindingExp, NFBinding::CEVAL_BINDING).clone(),
                ));
            }
            INVALID_BINDING { .. } => {
                binding = var_field!((**binding).binding, NFBinding::INVALID_BINDING);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub(crate) fn isEach(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isEach: bool;
    isEach = (match &**binding {
        RAW_BINDING {
            eachType: __binding_eachType,
            ..
        } => __binding_eachType.clone() == EachType::EACH.clone(),
        UNTYPED_BINDING {
            eachType: __binding_eachType,
            ..
        } => __binding_eachType.clone() == EachType::EACH.clone(),
        TYPED_BINDING {
            eachType: __binding_eachType,
            ..
        } => __binding_eachType.clone() == EachType::EACH.clone(),
        _ => false,
    });
    isEach
}

pub(crate) fn isTyped(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut isTyped: bool;
    isTyped = (match &**binding {
        TYPED_BINDING { .. } => true,
        FLAT_BINDING { .. } => true,
        _ => false,
    });
    isTyped
}

pub fn toString<'__b>(mut binding: &'__b metamodelica::Ref<NFBinding>, mut prefix: &'__b ArcStr) -> Result<ArcStr> {
    '__tco: loop {
        match &**binding {
            UNBOUND { .. } => return Ok(literal!("")),
            RAW_BINDING { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*prefix);
                    __mm_s.push_str(&*Dump::printExpStr(
                        var_field!((**binding).bindingExp, NFBinding::RAW_BINDING).clone(),
                    )?);
                    ArcStr::from(__mm_s)
                });
            }
            UNTYPED_BINDING { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*prefix);
                    __mm_s.push_str(&*Expression::toString(
                        var_field!((**binding).bindingExp, NFBinding::UNTYPED_BINDING).clone(),
                    )?);
                    ArcStr::from(__mm_s)
                });
            }
            TYPED_BINDING { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*prefix);
                    __mm_s.push_str(&*Expression::toString(
                        var_field!((**binding).bindingExp, NFBinding::TYPED_BINDING).clone(),
                    )?);
                    ArcStr::from(__mm_s)
                });
            }
            FLAT_BINDING { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*prefix);
                    __mm_s.push_str(&*Expression::toString(
                        var_field!((**binding).bindingExp, NFBinding::FLAT_BINDING).clone(),
                    )?);
                    ArcStr::from(__mm_s)
                });
            }
            CEVAL_BINDING { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*prefix);
                    __mm_s.push_str(&*Expression::toString(
                        var_field!((**binding).bindingExp, NFBinding::CEVAL_BINDING).clone(),
                    )?);
                    ArcStr::from(__mm_s)
                });
            }
            INVALID_BINDING { .. } => {
                (binding, prefix) = (var_field!((**binding).binding, NFBinding::INVALID_BINDING), prefix);
                continue '__tco;
            }
            _ => return Ok(literal!("")),
        }
    }
}

pub(crate) fn toFlatString(
    mut binding: metamodelica::Ref<NFBinding>,
    mut format: BaseModelica::OutputFormat,
    mut prefix: &ArcStr,
) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (match &*binding {
        UNBOUND { .. } => literal!(""),
        RAW_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefix);
            __mm_s.push_str(&*Dump::printExpStr(__binding_bindingExp.clone())?);
            ArcStr::from(__mm_s)
        }
        UNTYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefix);
            __mm_s.push_str(&*Expression::toFlatString(__binding_bindingExp.clone(), format)?);
            ArcStr::from(__mm_s)
        }
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefix);
            __mm_s.push_str(&*Expression::toFlatString(__binding_bindingExp.clone(), format)?);
            ArcStr::from(__mm_s)
        }
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefix);
            __mm_s.push_str(&*Expression::toFlatString(__binding_bindingExp.clone(), format)?);
            ArcStr::from(__mm_s)
        }
        CEVAL_BINDING {
            bindingExp: __binding_bindingExp,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefix);
            __mm_s.push_str(&*Expression::toFlatString(__binding_bindingExp.clone(), format)?);
            ArcStr::from(__mm_s)
        }
        INVALID_BINDING {
            binding: __binding_binding,
            ..
        } => toFlatString(__binding_binding.clone(), format, prefix)?,
        _ => literal!(""),
    });
    if format.showConfidence.clone() {
        string = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*string);
            __mm_s.push_str(&*literal!(" /* confidence = "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", actualConfidence(binding)?)));
            __mm_s.push_str(&*literal!("*/"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(string)
}

pub fn toDebugString(mut binding: &metamodelica::Ref<NFBinding>) -> ArcStr {
    let mut string: ArcStr;
    string = (match &**binding {
        WILD { .. } => literal!("WILD"),
        UNBOUND { .. } => literal!("UNBOUND"),
        RAW_BINDING { .. } => literal!("RAW_BINDING"),
        UNTYPED_BINDING { .. } => literal!("UNTYPED_BINDING"),
        TYPED_BINDING { .. } => literal!("TYPED_BINDING"),
        FLAT_BINDING { .. } => literal!("FLAT_BINDING"),
        CEVAL_BINDING { .. } => literal!("CEVAL_BINDING"),
        INVALID_BINDING { .. } => literal!("INVALID_BINDING"),
        _ => literal!("UNKNOWN"),
    });
    string
}

pub(crate) fn isEqual(
    mut binding1: &metamodelica::Ref<NFBinding>,
    mut binding2: &metamodelica::Ref<NFBinding>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (binding1, binding2) {
        (Deref @ UNBOUND { .. }, Deref @ UNBOUND { .. }) => true,
        (Deref @ RAW_BINDING { .. }, Deref @ RAW_BINDING { .. }) => AbsynUtil::expEqual(var_field!((**binding1).bindingExp, NFBinding::RAW_BINDING).clone(), var_field!((**binding2).bindingExp, NFBinding::RAW_BINDING).clone())?,
        (Deref @ UNTYPED_BINDING { .. }, Deref @ UNTYPED_BINDING { .. }) => Expression::isEqual(var_field!((**binding1).bindingExp, NFBinding::UNTYPED_BINDING).clone(), var_field!((**binding2).bindingExp, NFBinding::UNTYPED_BINDING).clone())?,
        (Deref @ TYPED_BINDING { .. }, Deref @ TYPED_BINDING { .. }) => Expression::isEqual(var_field!((**binding1).bindingExp, NFBinding::TYPED_BINDING).clone(), var_field!((**binding2).bindingExp, NFBinding::TYPED_BINDING).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub(crate) fn toDAE(mut binding: &metamodelica::Ref<NFBinding>) -> Result<metamodelica::Ref<DAE::Binding>> {
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    outBinding = (match &**binding {
        WILD { .. } => openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        UNBOUND { .. } => openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            variability: __binding_variability,
            ..
        } => makeDAEBinding(__binding_bindingExp.clone(), __binding_variability.clone())?,
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            variability: __binding_variability,
            ..
        } => makeDAEBinding(__binding_bindingExp.clone(), __binding_variability.clone())?,
        CEVAL_BINDING { .. } => openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        INVALID_BINDING {
            errors: __binding_errors,
            ..
        } => {
            Error::addTotalMessages(metamodelica::AsArg::as_arg(&__binding_errors))?;
            return Err("fail");
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFBinding.toDAE"));
                    __mm_s.push_str(&*literal!(" got untyped binding"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFBinding.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(outBinding)
}

pub(crate) fn makeDAEBinding(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut var: Variability,
) -> Result<metamodelica::Ref<DAE::Binding>> {
    let mut binding: metamodelica::Ref<DAE::Binding>;
    binding = metamodelica::Ref::new(DAE::Binding::EQBOUND {
        exp: Expression::toDAE(exp, false)?,
        evaluatedExp: None,
        constant_: NFPrefixes::variabilityToDAEConst(var),
        source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE,
    });
    Ok(binding)
}

pub(crate) fn toDAEExp(mut binding: &metamodelica::Ref<NFBinding>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut bindingExp: Option<metamodelica::Ref<DAE::Exp>>;
    bindingExp = (match &**binding {
        UNBOUND { .. } => None,
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Some(Expression::toDAE(__binding_bindingExp.clone(), false)?),
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Some(Expression::toDAE(__binding_bindingExp.clone(), false)?),
        CEVAL_BINDING { .. } => None,
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFBinding.toDAEExp"));
                    __mm_s.push_str(&*literal!(" got untyped binding"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFBinding.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(bindingExp)
}

pub(crate) fn applyExp(
    mut binding: &metamodelica::Ref<NFBinding>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFn =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**binding {
        UNTYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            Expression::apply(__binding_bindingExp.clone(), r#fn)?;
            ()
        }
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            Expression::apply(__binding_bindingExp.clone(), r#fn)?;
            ()
        }
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            Expression::apply(__binding_bindingExp.clone(), r#fn)?;
            ()
        }
        CEVAL_BINDING {
            bindingExp: __binding_bindingExp,
        } => {
            Expression::apply(__binding_bindingExp.clone(), r#fn)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn applyExpShallow(
    mut binding: &metamodelica::Ref<NFBinding>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFn =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**binding {
        UNTYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            r#fn(__binding_bindingExp.clone())?;
            ()
        }
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            r#fn(__binding_bindingExp.clone())?;
            ()
        }
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            r#fn(__binding_bindingExp.clone())?;
            ()
        }
        CEVAL_BINDING {
            bindingExp: __binding_bindingExp,
        } => {
            r#fn(__binding_bindingExp.clone())?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn mapExp(
    mut binding: metamodelica::Ref<NFBinding>,
    mut mapFn: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFBinding>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let () = (match &*binding.clone() {
        UNTYPED_BINDING {
            bindingExp: __esc_e1, ..
        } => {
            e1 = (*__esc_e1).clone();
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = e2);
            }
            ()
        }
        TYPED_BINDING {
            bindingExp: __esc_e1, ..
        } => {
            e1 = (*__esc_e1).clone();
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = e2);
            }
            ()
        }
        FLAT_BINDING {
            bindingExp: __esc_e1, ..
        } => {
            e1 = (*__esc_e1).clone();
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = e2);
            }
            ()
        }
        CEVAL_BINDING { bindingExp: __esc_e1 } => {
            e1 = (*__esc_e1).clone();
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::CEVAL_BINDING; bindingExp = e2);
            }
            ()
        }
        _ => (),
    });
    Ok(binding)
}

pub(crate) fn mapExpShallow(
    mut binding: metamodelica::Ref<NFBinding>,
    mut mapFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFBinding>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let () = (match &*binding.clone() {
        UNTYPED_BINDING {
            bindingExp: __esc_e1, ..
        } => {
            e1 = (*__esc_e1).clone();
            e2 = mapFn(e1.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = e2);
            }
            ()
        }
        TYPED_BINDING {
            bindingExp: __esc_e1, ..
        } => {
            e1 = (*__esc_e1).clone();
            e2 = mapFn(e1.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = e2);
            }
            ()
        }
        FLAT_BINDING {
            bindingExp: __esc_e1, ..
        } => {
            e1 = (*__esc_e1).clone();
            e2 = mapFn(e1.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = e2);
            }
            ()
        }
        CEVAL_BINDING { bindingExp: __esc_e1 } => {
            e1 = (*__esc_e1).clone();
            e2 = mapFn(e1.clone())?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(binding => NFBinding::CEVAL_BINDING; bindingExp = e2);
            }
            ()
        }
        _ => (),
    });
    Ok(binding)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut binding: &metamodelica::Ref<NFBinding>,
    mut foldFn: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    arg = (match &**binding {
        UNTYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::fold(__binding_bindingExp.clone(), foldFn.clone(), arg)?,
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::fold(__binding_bindingExp.clone(), foldFn.clone(), arg)?,
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::fold(__binding_bindingExp.clone(), foldFn.clone(), arg)?,
        CEVAL_BINDING {
            bindingExp: __binding_bindingExp,
        } => Expression::fold(__binding_bindingExp.clone(), foldFn.clone(), arg)?,
        _ => arg,
    });
    Ok(arg)
}

pub(crate) fn containsExp(
    mut binding: &metamodelica::Ref<NFBinding>,
    mut predFn: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**binding {
        UNTYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::contains(__binding_bindingExp.clone(), predFn)?,
        TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::contains(__binding_bindingExp.clone(), predFn)?,
        FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => Expression::contains(__binding_bindingExp.clone(), predFn)?,
        CEVAL_BINDING {
            bindingExp: __binding_bindingExp,
        } => Expression::contains(__binding_bindingExp.clone(), predFn)?,
        _ => false,
    });
    Ok(res)
}

pub fn update(
    mut binding: metamodelica::Ref<NFBinding>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<NFBinding>> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    binding = (match &*binding {
        WILD { .. } => metamodelica::Ref::new(NFBinding::TYPED_BINDING {
            bindingExp: exp.clone(),
            bindingType: Expression::typeOf(exp.clone()),
            variability: Expression::variability(exp.clone())?,
            purity: Expression::purity(exp.clone())?,
            eachType: EachType::NOT_EACH.clone(),
            evalState: if (Expression::isConstNumber(exp)) {
                Mutable::create(EvalState::EVALUATED.clone())
            } else {
                Mutable::create(EvalState::NOT_EVALUATED.clone())
            },
            isFlattened: true,
            source: Source::BINDING.clone(),
            confidence: NO_CONFIDENCE.clone(),
            info: metamodelica::sourceInfo!("NFFrontEnd/NFBinding.mo"),
        }),
        UNBOUND { .. } => metamodelica::Ref::new(NFBinding::TYPED_BINDING {
            bindingExp: exp.clone(),
            bindingType: Expression::typeOf(exp.clone()),
            variability: Expression::variability(exp.clone())?,
            purity: Expression::purity(exp.clone())?,
            eachType: EachType::NOT_EACH.clone(),
            evalState: if (Expression::isConstNumber(exp)) {
                Mutable::create(EvalState::EVALUATED.clone())
            } else {
                Mutable::create(EvalState::NOT_EVALUATED.clone())
            },
            isFlattened: true,
            source: Source::BINDING.clone(),
            confidence: NO_CONFIDENCE.clone(),
            info: metamodelica::sourceInfo!("NFFrontEnd/NFBinding.mo"),
        }),
        UNTYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = exp.clone());
            binding
        }
        TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = exp.clone());
            binding
        }
        FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = exp.clone());
            binding
        }
        CEVAL_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::CEVAL_BINDING; bindingExp = exp.clone());
            binding
        }
        INVALID_BINDING {
            binding: __binding_binding,
            ..
        } => {
            assign_variant_field!(binding => NFBinding::INVALID_BINDING; binding = update(__binding_binding.clone(), exp)?);
            binding
        }
        RAW_BINDING { .. } => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFBinding.update"));
                    __mm_s.push_str(&*literal!(" failed because a raw binding cannot be updated."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFBinding.update"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(binding)
}

pub(crate) fn setAttr(
    mut ty_attr: metamodelica::List<(ArcStr, metamodelica::Ref<NFBinding>)>,
    mut attr_name: &ArcStr,
    mut attr_value: &metamodelica::Ref<NFBinding>,
) -> metamodelica::List<(ArcStr, metamodelica::Ref<NFBinding>)> {
    let mut ty_attr: metamodelica::List<(ArcStr, metamodelica::Ref<NFBinding>)> = ty_attr;
    ty_attr = (::match_deref::match_deref! { match &(ty_attr) {
        Deref @ metamodelica::ListNode::Cons { head: (name, _), tail: rest } if (metamodelica::stringEq(&name, &attr_name)) => {
            metamodelica::cons((attr_name.clone(), attr_value.clone()), rest.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: at, tail: rest } => {
            metamodelica::cons(at.clone(), setAttr(rest.clone(), attr_name, attr_value))
        },
        Deref @ metamodelica::ListNode::Nil => {
            list![(attr_name.clone(), attr_value.clone())]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty_attr
}

pub(crate) fn propagate(
    mut binding: metamodelica::Ref<NFBinding>,
    mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> metamodelica::Ref<NFBinding> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let () = (match &*binding {
        RAW_BINDING {
            subs: __binding_subs, ..
        } => {
            assign_variant_field!(binding => NFBinding::RAW_BINDING; subs = listAppend(__binding_subs.clone(), subs));
            if var_field!((*binding).source, NFBinding::RAW_BINDING).clone() != Source::TYPE.clone() {
                assign_variant_field!(binding => NFBinding::RAW_BINDING; source = Source::MODIFIER.clone());
            }
            ()
        }
        _ => (),
    });
    binding
}

pub(crate) fn unpropagate(
    mut binding: metamodelica::Ref<NFBinding>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFBinding>> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let () = (match &*binding {
        RAW_BINDING {
            subs: __binding_subs, ..
        } => {
            assign_variant_field!(binding => NFBinding::RAW_BINDING; subs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut s in (__binding_subs.clone()).into_iter().cloned() {
                    if !(!(Subscript::isSplitFromOrigin(&(s.clone()), node)?)) { continue; }
                    let __x = s.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(binding)
}

pub fn source(mut binding: &metamodelica::Ref<NFBinding>) -> Source {
    let mut source: Source;
    source = (match &**binding {
        RAW_BINDING {
            source: __binding_source,
            ..
        } => __binding_source.clone(),
        UNTYPED_BINDING {
            source: __binding_source,
            ..
        } => __binding_source.clone(),
        TYPED_BINDING {
            source: __binding_source,
            ..
        } => __binding_source.clone(),
        FLAT_BINDING {
            source: __binding_source,
            ..
        } => __binding_source.clone(),
        _ => Source::BINDING.clone(),
    });
    source
}

pub(crate) fn setSource(mut source: Source, mut binding: metamodelica::Ref<NFBinding>) -> metamodelica::Ref<NFBinding> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let () = (match &*binding {
        RAW_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::RAW_BINDING; source = source);
            ()
        }
        UNTYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; source = source);
            ()
        }
        TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; source = source);
            ()
        }
        FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; source = source);
            ()
        }
        _ => (),
    });
    binding
}

pub(crate) fn setConfidence(
    mut confidence: i32,
    mut binding: metamodelica::Ref<NFBinding>,
) -> metamodelica::Ref<NFBinding> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let () = (match &*binding {
        RAW_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::RAW_BINDING; confidence = confidence);
            ()
        }
        UNTYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; confidence = confidence);
            ()
        }
        TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; confidence = confidence);
            ()
        }
        FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; confidence = confidence);
            ()
        }
        _ => (),
    });
    binding
}

pub(crate) fn makeUntyped(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut eachType: EachType,
    mut source: Source,
    mut info: SourceInfo,
    mut confidence: i32,
) -> metamodelica::Ref<NFBinding> {
    let mut binding: metamodelica::Ref<NFBinding>;
    binding = metamodelica::Ref::new(NFBinding::UNTYPED_BINDING {
        bindingExp: exp,
        isProcessing: false,
        scope: NFInstNode::InstNode::scopeRef(scope),
        eachType: eachType,
        source: source,
        confidence: confidence,
        info: info,
    });
    binding
}

pub(crate) fn makeTyped(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut eachType: EachType,
    mut source: Source,
    mut info: SourceInfo,
    mut state: EvalState,
    mut confidence: i32,
) -> Result<metamodelica::Ref<NFBinding>> {
    let mut binding: metamodelica::Ref<NFBinding>;
    binding = metamodelica::Ref::new(NFBinding::TYPED_BINDING {
        bindingExp: exp.clone(),
        bindingType: Expression::typeOf(exp.clone()),
        variability: Expression::variability(exp.clone())?,
        purity: Expression::purity(exp)?,
        eachType: eachType,
        evalState: Mutable::create(state),
        isFlattened: false,
        source: source,
        confidence: confidence,
        info: info,
    });
    Ok(binding)
}

pub fn makeFlat(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut var: Variability,
    mut source: Source,
    mut confidence: i32,
) -> metamodelica::Ref<NFBinding> {
    let mut binding: metamodelica::Ref<NFBinding>;
    binding = metamodelica::Ref::new(NFBinding::FLAT_BINDING {
        bindingExp: exp,
        variability: var,
        source: source,
        confidence: confidence,
    });
    binding
}

pub fn isEvaluated(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut evaluated: bool;
    evaluated = (match &**binding {
        TYPED_BINDING {
            evalState: __binding_evalState,
            ..
        } => Mutable::access(__binding_evalState.clone()) == EvalState::EVALUATED.clone(),
        CEVAL_BINDING { .. } => true,
        _ => false,
    });
    evaluated
}

pub(crate) fn hasTypeOrigin(mut binding: &metamodelica::Ref<NFBinding>) -> Result<bool> {
    let mut res: bool;
    res = (match &**binding {
        RAW_BINDING {
            subs: __binding_subs, ..
        } if (!((__binding_subs).is_empty())) => Subscript::isSplitClassProxy(&((__binding_subs).head().cloned()?))?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn expandEach(
    mut binding: metamodelica::Ref<NFBinding>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFBinding>> {
    let mut binding: metamodelica::Ref<NFBinding> = binding;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut node_exp: metamodelica::Ref<Absyn::Exp>;
    let size_name: metamodelica::Ref<Absyn::ComponentRef> = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
        name: literal!("size"),
        subscripts: metamodelica::nil(),
    });
    let fill_name: metamodelica::Ref<Absyn::ComponentRef> = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
        name: literal!("fill"),
        subscripts: metamodelica::nil(),
    });
    let () = (match &*binding {
        RAW_BINDING {
            eachType: EachType::EACH,
            bindingExp: __binding_bindingExp,
            ..
        } => {
            node_exp = metamodelica::Ref::new(Absyn::Exp::CREF {
                componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: NFInstNode::InstNode::name(node)?,
                    subscripts: metamodelica::nil(),
                }),
            });
            args = metamodelica::nil();
            for mut i in ({
                let __s = NFInstNode::InstNode::dimensionCount(node);
                let __e = 1;
                (0i32..)
                    .map(move |__k| __s + __k * (-1))
                    .take_while(move |&__v| __v >= __e)
            }) {
                args = metamodelica::cons(
                    AbsynUtil::makeCall(
                        size_name.clone(),
                        list![
                            node_exp.clone(),
                            metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i })
                        ],
                        metamodelica::nil(),
                    ),
                    args,
                );
            }
            args = metamodelica::cons(__binding_bindingExp.clone(), args);
            assign_variant_field!(binding => NFBinding::RAW_BINDING; bindingExp = AbsynUtil::makeCall(fill_name, args, metamodelica::nil()));
            ()
        }
        _ => (),
    });
    Ok(binding)
}

pub fn isClockOrSampleFunction(mut binding: &metamodelica::Ref<NFBinding>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(getExpOpt(binding)) {
        Some(exp) => {
            Expression::isClockOrSampleFunction(metamodelica::AsArg::as_arg(&exp))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn confidence(mut binding: &metamodelica::Ref<NFBinding>) -> i32 {
    let mut confidence: i32;
    confidence = (match &**binding {
        RAW_BINDING {
            confidence: __binding_confidence,
            ..
        } => __binding_confidence.clone(),
        UNTYPED_BINDING {
            confidence: __binding_confidence,
            ..
        } => __binding_confidence.clone(),
        TYPED_BINDING {
            confidence: __binding_confidence,
            ..
        } => __binding_confidence.clone(),
        FLAT_BINDING {
            confidence: __binding_confidence,
            ..
        } => __binding_confidence.clone(),
        _ => NO_CONFIDENCE.clone(),
    });
    confidence
}

pub(crate) fn isFromType(mut binding: &metamodelica::Ref<NFBinding>) -> bool {
    let mut fromType: bool = source(binding) == Source::TYPE.clone();
    fromType
}

pub fn compareStartConfidence(
    mut b1: metamodelica::Ref<NFBinding>,
    mut b2: metamodelica::Ref<NFBinding>,
) -> Result<i32> {
    let mut cmp: i32;
    if isFromType(&b1) != isFromType(&b2) {
        cmp = if (isFromType(&b1)) { 1 } else { -1 };
    } else {
        cmp = actualConfidence(b1.clone())? - actualConfidence(b2.clone())?;
        if cmp == 0 {
            cmp = confidence(&b1) - confidence(&b2);
        }
    }
    Ok(cmp)
}

pub(crate) fn actualConfidence(mut binding: metamodelica::Ref<NFBinding>) -> Result<i32> {
    let mut conf: i32 = NO_CONFIDENCE.clone();
    let mut b: metamodelica::Ref<NFBinding> = binding.clone();
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    if isFromType(&binding) {
        conf = confidence(&binding);
        return Ok(conf);
    }
    while hasExp(&b) {
        conf = std::cmp::min(conf, confidence(&b));
        exp = getExp(&b)?;
        b = EMPTY_BINDING().clone();
        let () = (match &*exp {
            Expression::CREF { cref: __exp_cref, .. }
                if (ComponentRef::isCref(metamodelica::AsArg::as_arg(&__exp_cref))) =>
            {
                node =
                    NFInstNode::InstNode::resolveInner(ComponentRef::node(metamodelica::AsArg::as_arg(&__exp_cref))?);
                let () = (match &*node {
                    NFInstNode::InstNode::VAR_NODE {
                        varPointer: __node_varPointer,
                        ..
                    } => {
                        var = Pointer::access(PointerWeak::upgrade(__node_varPointer.clone())?);
                        if Variable::variability(&var) < Variability::DISCRETE.clone() {
                            b = var.binding.clone();
                        }
                        ()
                    }
                    _ => {
                        if NFInstNode::InstNode::isComponent(&node)? {
                            comp = NFInstNode::InstNode::component(&node)?;
                            if Component::variability(&comp)? < Variability::DISCRETE.clone() {
                                b = Component::getBinding(&comp);
                            }
                        }
                        ()
                    }
                });
                ()
            }
            _ => (),
        });
    }
    Ok(conf)
}
