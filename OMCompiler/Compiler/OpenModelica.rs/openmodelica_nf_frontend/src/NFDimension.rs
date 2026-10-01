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
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInst as Inst;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_ast::Absyn::Exp;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn::Subscript;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFDimension {
    RAW_DIM {
        dim: metamodelica::Ref<Subscript>,
        /// Weakly: the class tree owns the scope this
        ///      dimension was written in.
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    },
    UNTYPED {
        dimension: metamodelica::Ref<Expression::NFExpression>,
        isProcessing: bool,
    },
    INTEGER {
        size: i32,
        var: Variability,
    },
    BOOLEAN,
    ENUM {
        enumType: metamodelica::Ref<Type::NFType>,
    },
    EXP {
        exp: metamodelica::Ref<Expression::NFExpression>,
        var: Variability,
    },
    /// for all symbolic purposes this is INTEGER() for codegeneration it is EXP()
    ///    invoked by using annotation(__OpenModelica_resizable=true) on a parameter
    RESIZABLE {
        /// the actual size defined by the user
        size: i32,
        /// the optimal size determined by the backend
        opt_size: Option<i32>,
        /// the full expression (parameter)
        exp: metamodelica::Ref<Expression::NFExpression>,
        var: Variability,
    },
    UNKNOWN,
}
impl metamodelica::gc::MMTrace for NFDimension {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFDimension::RAW_DIM { dim, scope } => {
                metamodelica::gc::MMTrace::mm_accept(dim, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                Ok(())
            }
            NFDimension::UNTYPED {
                dimension,
                isProcessing,
            } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProcessing, __mmv)?;
                Ok(())
            }
            NFDimension::INTEGER { size, var } => {
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            NFDimension::BOOLEAN => Ok(()),
            NFDimension::ENUM { enumType } => {
                metamodelica::gc::MMTrace::mm_accept(enumType, __mmv)?;
                Ok(())
            }
            NFDimension::EXP { exp, var } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            NFDimension::RESIZABLE {
                size,
                opt_size,
                exp,
                var,
            } => {
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(opt_size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            NFDimension::UNKNOWN => Ok(()),
        }
    }
}
impl NFDimension {
    pub fn interned_BOOLEAN() -> metamodelica::Ref<NFDimension> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFDimension> = metamodelica::Ref::new(NFDimension::BOOLEAN);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_UNKNOWN() -> metamodelica::Ref<NFDimension> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFDimension> = metamodelica::Ref::new(NFDimension::UNKNOWN);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_BOOLEAN() -> metamodelica::Ref<NFDimension> {
    NFDimension::interned_BOOLEAN()
}
pub fn interned_UNKNOWN() -> metamodelica::Ref<NFDimension> {
    NFDimension::interned_UNKNOWN()
}
impl Default for NFDimension {
    fn default() -> Self {
        Self::BOOLEAN
    }
}
pub use self::NFDimension::{BOOLEAN, ENUM, EXP, INTEGER, RAW_DIM, RESIZABLE, UNKNOWN, UNTYPED};
pub fn fromExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut var: Variability,
) -> Result<metamodelica::Ref<NFDimension>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(&*exp) {
            Deref @ Expression::INTEGER { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(NFDimension::INTEGER { size: __exp_value.clone(), var: var }))
            },
            Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: ty, .. } } => {
                match &*ty.clone() {
            Type::BOOLEAN => return Ok(crate::NFDimension::interned_BOOLEAN()),
            Type::ENUMERATION { .. } => return Ok(metamodelica::Ref::new(NFDimension::ENUM { enumType: ty.clone() })),
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.fromExp")); __mm_s.push_str(&*literal!(" got invalid typename")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFDimension.mo")))?;
                return Ok(return Err("fail"))
            },
        }
            },
            Deref @ Expression::ARRAY { .. } if (Expression::arrayAllEqual(exp.clone())) => {
                { (exp, var) = (Expression::arrayFirstScalar(exp.clone())?, var); continue '__tco; }
            },
            Deref @ Expression::SUBSCRIPTED_EXP { split: true, exp: __exp_exp, .. } if (Expression::isArray(metamodelica::AsArg::as_arg(&__exp_exp)) && Expression::arrayAllEqual(__exp_exp.clone())) => {
                { (exp, var) = (Expression::arrayFirstScalar(__exp_exp.clone())?, var); continue '__tco; }
            },
            _ => {
                let mut exp_simple: metamodelica::Ref<Expression::NFExpression>;
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e2: metamodelica::Ref<Expression::NFExpression>;
                let mut value: i32;
                let mut value_original: i32;
                exp_simple = SimplifyExp::simplify(exp.clone(), false)?;
                match &*exp_simple {
            Expression::INTEGER { value: __esc_value } => {
                value = (*__esc_value).clone();
                return Ok(metamodelica::Ref::new(NFDimension::INTEGER { size: value.clone(), var: var }))
            },
            _ => {
                e1 = Expression::map(exp_simple.clone(), (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                e1 = SimplifyExp::simplify(e1, false)?;
                match &*e1 {
            Expression::INTEGER { value: __esc_value } => {
                value = (*__esc_value).clone();
                e2 = Expression::map(exp_simple, (std::sync::Arc::new(Expression::replaceResizableParameterWithOriginal) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                e2 = SimplifyExp::simplify(e2, false)?;
                match &*e2 {
            Expression::INTEGER { value: value_original } if (value.clone() != value_original.clone()) => return Ok(metamodelica::Ref::new(NFDimension::RESIZABLE { size: value_original.clone(), opt_size: Some(value.clone()), exp: exp.clone(), var: var })),
            _ => return Ok(metamodelica::Ref::new(NFDimension::RESIZABLE { size: value.clone(), opt_size: None, exp: exp.clone(), var: var })),
        }
            },
            _ => return Ok(metamodelica::Ref::new(NFDimension::EXP { exp: exp.clone(), var: var })),
        }
            },
        }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn fromRange(
    mut range: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<NFDimension>> {
    let mut dim: metamodelica::Ref<NFDimension>;
    let mut start: i32;
    let mut step: i32;
    let mut stop: i32;
    (start, step, stop) = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: __esc_start }, step: None, stop: Deref @ Expression::INTEGER { value: __esc_stop }, .. } => {
            start = (*__esc_start).clone();
            stop = (*__esc_stop).clone();
            (start.clone(), 1, stop.clone())
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: __esc_start }, step: Some(Deref @ Expression::INTEGER { value: __esc_step }), stop: Deref @ Expression::INTEGER { value: __esc_stop }, .. } => {
            start = (*__esc_start).clone();
            step = (*__esc_step).clone();
            stop = (*__esc_stop).clone();
            (start.clone(), step.clone(), stop.clone())
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.fromRange")); __mm_s.push_str(&*literal!(" got non-range expression: ")); __mm_s.push_str(&*Expression::toString(range)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFDimension.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dim = metamodelica::Ref::new(NFDimension::INTEGER {
        size: intDiv(stop - start, step) + 1,
        var: Prefixes::Variability::CONSTANT.clone(),
    });
    Ok(dim)
}

pub fn fromInteger(mut n: i32, mut var: Variability) -> metamodelica::Ref<NFDimension> {
    let mut dim: metamodelica::Ref<NFDimension> = metamodelica::Ref::new(NFDimension::INTEGER { size: n, var: var });
    dim
}

pub(crate) fn fromExpArray(
    mut expl: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
) -> metamodelica::Ref<NFDimension> {
    let mut dim: metamodelica::Ref<NFDimension> = metamodelica::Ref::new(NFDimension::INTEGER {
        size: metamodelica::arrayLength(expl.clone()),
        var: Variability::CONSTANT.clone(),
    });
    dim
}

pub(crate) fn fromExpList(
    mut expl: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> metamodelica::Ref<NFDimension> {
    let mut dim: metamodelica::Ref<NFDimension> = metamodelica::Ref::new(NFDimension::INTEGER {
        size: ((expl).len() as i32),
        var: Variability::CONSTANT.clone(),
    });
    dim
}

pub(crate) fn toRange(mut dim: &metamodelica::Ref<NFDimension>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    range = metamodelica::Ref::new(Expression::NFExpression::RANGE {
        ty: Type::liftArrayLeft(typeOf(dim), dim),
        start: lowerBoundExp(dim)?,
        step: None,
        stop: upperBoundExp(dim)?,
    });
    Ok(range)
}

pub(crate) fn toDAE(mut dim: &metamodelica::Ref<NFDimension>) -> Result<metamodelica::Ref<DAE::Dimension>> {
    let mut daeDim: metamodelica::Ref<DAE::Dimension>;
    daeDim = (::match_deref::match_deref! { match dim {
        Deref @ INTEGER { size: __dim_size, .. } => {
            metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: __dim_size.clone() })
        },
        Deref @ BOOLEAN { .. } => {
            openmodelica_frontend_types::DAE::Dimension::interned_DIM_BOOLEAN()
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            metamodelica::Ref::new(DAE::Dimension::DIM_ENUM { enumTypeName: var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), literals: var_field!((**ty).literals, Type::NFType::ENUMERATION).clone(), size: ((var_field!((**ty).literals, Type::NFType::ENUMERATION)).len() as i32) })
        },
        Deref @ EXP { exp: __dim_exp, .. } => {
            metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: Expression::toDAE(__dim_exp.clone(), false)? })
        },
        Deref @ RESIZABLE { exp: __dim_exp, .. } => {
            metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: Expression::toDAE(__dim_exp.clone(), false)? })
        },
        Deref @ UNKNOWN { .. } => {
            openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(daeDim)
}

pub(crate) fn add(
    mut a: &metamodelica::Ref<NFDimension>,
    mut b: &metamodelica::Ref<NFDimension>,
) -> metamodelica::Ref<NFDimension> {
    fn addExp(
        mut e1: metamodelica::Ref<Expression::NFExpression>,
        mut e2: metamodelica::Ref<Expression::NFExpression>,
    ) -> metamodelica::Ref<Expression::NFExpression> {
        let mut res: metamodelica::Ref<Expression::NFExpression> =
            metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: e1.clone(),
                operator: metamodelica::Ref::new(Operator::NFOperator {
                    ty: crate::NFType::interned_INTEGER(),
                    op: Operator::Op::ADD.clone(),
                }),
                exp2: e2.clone(),
            });
        res
    }

    fn addOpt(mut s1: Option<i32>, mut s2: Option<i32>) -> Option<i32> {
        let mut res: Option<i32>;
        res = (match (s1, s2) {
            (Some(mut i1), Some(mut i2)) => Some(i1 + i2),
            _ => None,
        });
        res
    }

    let mut c: metamodelica::Ref<NFDimension>;
    c = (::match_deref::match_deref! { match (a, b) {
        (Deref @ UNKNOWN { .. }, _) => crate::NFDimension::interned_UNKNOWN(),
        (_, Deref @ UNKNOWN { .. }) => crate::NFDimension::interned_UNKNOWN(),
        (Deref @ INTEGER { .. }, Deref @ INTEGER { .. }) => metamodelica::Ref::new(NFDimension::INTEGER { size: var_field!((**a).size, NFDimension::INTEGER).clone() + var_field!((**b).size, NFDimension::INTEGER).clone(), var: Prefixes::variabilityMax(var_field!((**a).var, NFDimension::INTEGER).clone(), var_field!((**b).var, NFDimension::INTEGER).clone()) }),
        (Deref @ INTEGER { .. }, Deref @ EXP { .. }) => metamodelica::Ref::new(NFDimension::EXP { exp: addExp(var_field!((**b).exp, NFDimension::EXP).clone(), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((**a).size, NFDimension::INTEGER).clone() })), var: var_field!((**b).var, NFDimension::EXP).clone() }),
        (Deref @ EXP { .. }, Deref @ INTEGER { .. }) => metamodelica::Ref::new(NFDimension::EXP { exp: addExp(var_field!((**a).exp, NFDimension::EXP).clone(), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((**b).size, NFDimension::INTEGER).clone() })), var: var_field!((**a).var, NFDimension::EXP).clone() }),
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => metamodelica::Ref::new(NFDimension::EXP { exp: addExp(var_field!((**a).exp, NFDimension::EXP).clone(), var_field!((**b).exp, NFDimension::EXP).clone()), var: Prefixes::variabilityMax(var_field!((**a).var, NFDimension::EXP).clone(), var_field!((**b).var, NFDimension::EXP).clone()) }),
        (Deref @ INTEGER { .. }, Deref @ RESIZABLE { .. }) => metamodelica::Ref::new(NFDimension::RESIZABLE { size: var_field!((**a).size, NFDimension::INTEGER).clone() + var_field!((**b).size, NFDimension::RESIZABLE).clone(), opt_size: addOpt(Some(var_field!((**a).size, NFDimension::INTEGER).clone()), var_field!((**b).opt_size, NFDimension::RESIZABLE).clone()), exp: addExp(var_field!((**b).exp, NFDimension::RESIZABLE).clone(), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((**a).size, NFDimension::INTEGER).clone() })), var: var_field!((**b).var, NFDimension::RESIZABLE).clone() }),
        (Deref @ RESIZABLE { .. }, Deref @ INTEGER { .. }) => metamodelica::Ref::new(NFDimension::RESIZABLE { size: var_field!((**a).size, NFDimension::RESIZABLE).clone() + var_field!((**b).size, NFDimension::INTEGER).clone(), opt_size: addOpt(var_field!((**a).opt_size, NFDimension::RESIZABLE).clone(), Some(var_field!((**b).size, NFDimension::INTEGER).clone())), exp: addExp(var_field!((**a).exp, NFDimension::RESIZABLE).clone(), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((**b).size, NFDimension::INTEGER).clone() })), var: var_field!((**a).var, NFDimension::RESIZABLE).clone() }),
        (Deref @ EXP { .. }, Deref @ RESIZABLE { .. }) => metamodelica::Ref::new(NFDimension::EXP { exp: addExp(var_field!((**a).exp, NFDimension::EXP).clone(), var_field!((**b).exp, NFDimension::RESIZABLE).clone()), var: Prefixes::variabilityMax(var_field!((**a).var, NFDimension::EXP).clone(), var_field!((**b).var, NFDimension::RESIZABLE).clone()) }),
        (Deref @ RESIZABLE { .. }, Deref @ EXP { .. }) => metamodelica::Ref::new(NFDimension::EXP { exp: addExp(var_field!((**a).exp, NFDimension::RESIZABLE).clone(), var_field!((**b).exp, NFDimension::EXP).clone()), var: Prefixes::variabilityMax(var_field!((**a).var, NFDimension::RESIZABLE).clone(), var_field!((**b).var, NFDimension::EXP).clone()) }),
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => metamodelica::Ref::new(NFDimension::RESIZABLE { size: var_field!((**a).size, NFDimension::RESIZABLE).clone() + var_field!((**b).size, NFDimension::RESIZABLE).clone(), opt_size: addOpt(var_field!((**a).opt_size, NFDimension::RESIZABLE).clone(), var_field!((**b).opt_size, NFDimension::RESIZABLE).clone()), exp: addExp(var_field!((**a).exp, NFDimension::RESIZABLE).clone(), var_field!((**b).exp, NFDimension::RESIZABLE).clone()), var: Prefixes::variabilityMax(var_field!((**a).var, NFDimension::RESIZABLE).clone(), var_field!((**b).var, NFDimension::RESIZABLE).clone()) }),
        _ => crate::NFDimension::interned_UNKNOWN(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    c
}

pub fn size(mut dim: &metamodelica::Ref<NFDimension>, mut resize: bool) -> Result<i32> {
    let mut size: i32;
    size = (::match_deref::match_deref! { match dim {
        Deref @ INTEGER { size: __dim_size, .. } => {
            __dim_size.clone()
        },
        Deref @ RESIZABLE { opt_size: __dim_opt_size, size: __dim_size, .. } => {
            if (resize) {__dim_opt_size.clone().unwrap_or(__dim_size.clone())} else {__dim_size.clone()}
        },
        Deref @ BOOLEAN { .. } => {
            2
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            ((var_field!((**ty).literals, Type::NFType::ENUMERATION)).len() as i32)
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.size")); __mm_s.push_str(&*literal!(" could not get size of: ")); __mm_s.push_str(&*toString(dim)?); ArcStr::from(__mm_s) })?;
            }
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

pub fn sizes(
    mut dims: metamodelica::List<metamodelica::Ref<NFDimension>>,
    mut resize: bool,
) -> Result<metamodelica::List<i32>> {
    let mut outSizes: metamodelica::List<i32> = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = size(&(d.clone()), resize)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outSizes)
}

pub fn sizesProduct(mut dims: metamodelica::List<metamodelica::Ref<NFDimension>>, mut resize: bool) -> Result<i32> {
    let mut outSize: i32 = ({
        let mut __acc: i32 = 1;
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = size(&(d.clone()), resize)?;
            __acc *= __x;
        }
        __acc
    });
    Ok(outSize)
}

pub fn isEqual(mut dim1: &metamodelica::Ref<NFDimension>, mut dim2: &metamodelica::Ref<NFDimension>) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match (dim1, dim2) {
        (Deref @ UNKNOWN { .. }, _) => true,
        (_, Deref @ UNKNOWN { .. }) => true,
        (Deref @ EXP { .. }, _) => true,
        (_, Deref @ EXP { .. }) => true,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((**dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((**dim2).exp, NFDimension::RESIZABLE).clone())?,
        _ => size(dim1, false)? == size(dim2, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub(crate) fn isEqualKnown(
    mut dim1: &metamodelica::Ref<NFDimension>,
    mut dim2: &metamodelica::Ref<NFDimension>,
) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match (dim1, dim2) {
        (Deref @ UNKNOWN { .. }, _) => false,
        (_, Deref @ UNKNOWN { .. }) => false,
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Expression::isEqual(var_field!((**dim1).exp, NFDimension::EXP).clone(), var_field!((**dim2).exp, NFDimension::EXP).clone())?,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((**dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((**dim2).exp, NFDimension::RESIZABLE).clone())?,
        (Deref @ EXP { .. }, _) => false,
        (_, Deref @ EXP { .. }) => false,
        _ => size(dim1, false)? == size(dim2, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub(crate) fn isEqualKnownSize(
    mut dim1: &metamodelica::Ref<NFDimension>,
    mut node1: &metamodelica::Ref<InstNode::InstNode>,
    mut index1: i32,
    mut dim2: &metamodelica::Ref<NFDimension>,
    mut node2: &metamodelica::Ref<InstNode::InstNode>,
    mut index2: i32,
) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match (dim1, dim2) {
        (Deref @ EXP { .. }, _) if (isSizeOf(dim1, node2, index2)?) => true,
        (_, Deref @ EXP { .. }) if (isSizeOf(dim2, node1, index1)?) => true,
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Expression::isEqual(var_field!((**dim1).exp, NFDimension::EXP).clone(), var_field!((**dim2).exp, NFDimension::EXP).clone())?,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((**dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((**dim2).exp, NFDimension::RESIZABLE).clone())?,
        (Deref @ UNKNOWN { .. }, _) => false,
        (_, Deref @ UNKNOWN { .. }) => false,
        _ => size(dim1, false)? == size(dim2, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub(crate) fn isSame(
    mut dim1: &metamodelica::Ref<NFDimension>,
    mut dim2: &metamodelica::Ref<NFDimension>,
) -> Result<bool> {
    let mut same: bool;
    same = (::match_deref::match_deref! { match (dim1, dim2) {
        (Deref @ RAW_DIM { .. }, Deref @ RAW_DIM { .. }) => NFInstNode::InstNode::isSame(NFInstNode::InstNode::borrow(var_field!((**dim1).scope, NFDimension::RAW_DIM).clone())?, NFInstNode::InstNode::borrow(var_field!((**dim2).scope, NFDimension::RAW_DIM).clone())?) && AbsynUtil::subscriptEqual(var_field!((**dim1).dim, NFDimension::RAW_DIM), var_field!((**dim2).dim, NFDimension::RAW_DIM))?,
        (Deref @ UNTYPED { .. }, Deref @ UNTYPED { .. }) => Expression::isEqual(var_field!((**dim1).dimension, NFDimension::UNTYPED).clone(), var_field!((**dim2).dimension, NFDimension::UNTYPED).clone())?,
        (Deref @ INTEGER { .. }, Deref @ INTEGER { .. }) => var_field!((**dim1).size, NFDimension::INTEGER).clone() == var_field!((**dim2).size, NFDimension::INTEGER).clone(),
        (Deref @ BOOLEAN { .. }, Deref @ BOOLEAN { .. }) => true,
        (Deref @ ENUM { .. }, Deref @ ENUM { .. }) => Type::isEqual(var_field!((**dim1).enumType, NFDimension::ENUM), var_field!((**dim2).enumType, NFDimension::ENUM))?,
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Expression::isEqual(var_field!((**dim1).exp, NFDimension::EXP).clone(), var_field!((**dim2).exp, NFDimension::EXP).clone())?,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((**dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((**dim2).exp, NFDimension::RESIZABLE).clone())?,
        (Deref @ UNKNOWN { .. }, Deref @ UNKNOWN { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(same)
}

pub(crate) fn isSizeOf(
    mut dim: &metamodelica::Ref<NFDimension>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut index: i32,
) -> Result<bool> {
    let mut res: bool;
    let mut cref_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut index_exp: metamodelica::Ref<Expression::NFExpression>;
    res = (::match_deref::match_deref! { match dim {
        Deref @ EXP { exp: Deref @ Expression::SIZE { exp: __esc_cref_exp @ Deref @ Expression::CREF { .. }, dimIndex: Some(__esc_index_exp) }, .. } => {
            cref_exp = (*__esc_cref_exp).clone();
            index_exp = (*__esc_index_exp).clone();
            NFInstNode::InstNode::refEqual(&(ComponentRef::node(var_field!((*cref_exp).cref, Expression::NFExpression::CREF))?), node)? && Expression::isEqual(index_exp.clone(), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: index }))?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isResizable(mut dim: &metamodelica::Ref<NFDimension>) -> bool {
    let mut b: bool;
    b = (match &**dim {
        RESIZABLE { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn allEqualKnown(
    mut dims1: metamodelica::List<metamodelica::Ref<NFDimension>>,
    mut dims2: metamodelica::List<metamodelica::Ref<NFDimension>>,
) -> Result<bool> {
    let mut allEqual: bool = List::isEqualOnTrue(dims1.clone(), dims2.clone(), &move |__a0: metamodelica::Ref<
        NFDimension,
    >,
                                                                                      __a1: metamodelica::Ref<
        NFDimension,
    >| isEqualKnown(&__a0, &__a1))?;
    Ok(allEqual)
}

pub fn isKnown(mut dim: &metamodelica::Ref<NFDimension>, mut allowExp: bool) -> bool {
    let mut known: bool;
    known = (match &**dim {
        INTEGER { .. } => true,
        BOOLEAN { .. } => true,
        ENUM { .. } => true,
        RESIZABLE { .. } => true,
        EXP { .. } => allowExp,
        _ => false,
    });
    known
}

pub(crate) fn isUnknown(mut dim: &metamodelica::Ref<NFDimension>) -> bool {
    let mut isUnknown: bool;
    isUnknown = (match &**dim {
        UNKNOWN { .. } => true,
        _ => false,
    });
    isUnknown
}

pub(crate) fn isZero(mut dim: &metamodelica::Ref<NFDimension>) -> Result<bool> {
    let mut isZero: bool;
    isZero = (match &**dim {
        INTEGER { size: __dim_size, .. } => __dim_size.clone() == 0,
        ENUM {
            enumType: __dim_enumType,
        } => Type::enumSize(metamodelica::AsArg::as_arg(&__dim_enumType))? == 0,
        _ => false,
    });
    Ok(isZero)
}

pub fn isOne(mut dim: &metamodelica::Ref<NFDimension>) -> Result<bool> {
    let mut isOne: bool;
    isOne = (match &**dim {
        INTEGER { size: __dim_size, .. } => __dim_size.clone() == 1,
        ENUM {
            enumType: __dim_enumType,
        } => Type::enumSize(metamodelica::AsArg::as_arg(&__dim_enumType))? == 1,
        _ => false,
    });
    Ok(isOne)
}

pub(crate) fn subscriptType(mut dim: &metamodelica::Ref<NFDimension>) -> metamodelica::Ref<Type::NFType> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = (match &**dim {
        INTEGER { .. } => crate::NFType::interned_INTEGER(),
        BOOLEAN { .. } => crate::NFType::interned_BOOLEAN(),
        ENUM {
            enumType: __dim_enumType,
        } => __dim_enumType.clone(),
        EXP { exp: __dim_exp, .. } => Expression::typeOf(__dim_exp.clone()),
        RESIZABLE { exp: __dim_exp, .. } => Expression::typeOf(__dim_exp.clone()),
        _ => crate::NFType::interned_UNKNOWN(),
    });
    ty
}

pub fn toString(mut dim: &metamodelica::Ref<NFDimension>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match dim {
        Deref @ RAW_DIM { dim: __dim_dim, .. } => {
            Dump::printSubscriptStr(metamodelica::AsArg::as_arg(&__dim_dim))?
        },
        Deref @ INTEGER { size: __dim_size, .. } => {
            ArcStr::from(::std::format!("{}", __dim_size.clone()))
        },
        Deref @ BOOLEAN { .. } => {
            literal!("Boolean")
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            AbsynUtil::pathString(var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), literal!("."), true, false)?
        },
        Deref @ EXP { exp: __dim_exp, .. } => {
            Expression::toString(__dim_exp.clone())?
        },
        Deref @ RESIZABLE { exp: __dim_exp, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(__dim_exp.clone())?); __mm_s.push_str(&*literal!("(R)")); ArcStr::from(__mm_s) }
        },
        Deref @ UNKNOWN { .. } => {
            literal!(":")
        },
        Deref @ UNTYPED { dimension: __dim_dimension, .. } => {
            Expression::toString(__dim_dimension.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

pub fn hashList(mut dims: &metamodelica::List<metamodelica::Ref<NFDimension>>) -> Result<i32> {
    let mut hash: i32 = Util::HASH_SEED.clone();
    for mut dim in &**dims {
        hash = stringHashDjb2Continue(&(toString(metamodelica::AsArg::as_arg(&dim))?), hash);
    }
    Ok(hash)
}

pub(crate) fn toStringList(
    mut dims: metamodelica::List<metamodelica::Ref<NFDimension>>,
    mut brackets: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut d in (dims).into_iter().cloned() {
                let __x = toString(&(d.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        literal!(", "),
    );
    if brackets {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("["));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn toFlatString(
    mut dim: &metamodelica::Ref<NFDimension>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**dim {
        INTEGER { size: __dim_size, .. } => ArcStr::from(::std::format!("{}", __dim_size.clone())),
        BOOLEAN { .. } => literal!("Boolean"),
        ENUM {
            enumType: __dim_enumType,
        } => Type::toFlatString(metamodelica::AsArg::as_arg(&__dim_enumType), format)?,
        EXP { exp: __dim_exp, .. } => Expression::toFlatString(__dim_exp.clone(), format)?,
        RESIZABLE { exp: __dim_exp, .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toFlatString(__dim_exp.clone(), format)?);
            __mm_s.push_str(&*literal!("(R)"));
            ArcStr::from(__mm_s)
        }
        UNKNOWN { .. } => literal!(":"),
        UNTYPED {
            dimension: __dim_dimension,
            ..
        } => Expression::toFlatString(__dim_dimension.clone(), format)?,
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

pub(crate) fn toFlatStringList(
    mut dims: metamodelica::List<metamodelica::Ref<NFDimension>>,
    mut format: BaseModelica::OutputFormat,
    mut name: ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = List::toStringCustom(
        dims,
        &({
            let __pe_b1 = format;
            move |__pe_a0| toFlatString(&__pe_a0, __pe_b1.clone())
        }),
        name,
        literal!("["),
        literal!(", "),
        literal!("]"),
        false,
        0,
    )?;
    Ok(r#str)
}

pub(crate) fn endExp(
    mut dim: &metamodelica::Ref<NFDimension>,
    mut subscriptedExp: &metamodelica::Ref<Expression::NFExpression>,
    mut index: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut sizeExp: metamodelica::Ref<Expression::NFExpression>;
    sizeExp = (::match_deref::match_deref! { match dim {
        Deref @ INTEGER { size: __dim_size, .. } => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: __dim_size.clone() })
        },
        Deref @ BOOLEAN { .. } => {
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Expression::makeEnumLiteral(ty.clone(), ((var_field!((**ty).literals, Type::NFType::ENUMERATION)).len() as i32))?
        },
        Deref @ EXP { exp: __dim_exp, .. } => {
            __dim_exp.clone()
        },
        Deref @ RESIZABLE { exp: __dim_exp, .. } => {
            __dim_exp.clone()
        },
        Deref @ UNKNOWN { .. } => {
            (match &**subscriptedExp {
        Expression::CREF { cref: __subscriptedExp_cref, .. } => metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: Expression::fromCref((ComponentRef::stripSubscripts(__subscriptedExp_cref.clone())).0, false)?, dimIndex: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: index })) }),
        Expression::SUBSCRIPTED_EXP { exp: __subscriptedExp_exp, .. } => metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: __subscriptedExp_exp.clone(), dimIndex: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: index })) }),
        _ => return Err("match: no arm matched"),
    })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub fn sizeExp(mut dim: &metamodelica::Ref<NFDimension>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut sizeExp: metamodelica::Ref<Expression::NFExpression>;
    sizeExp = (::match_deref::match_deref! { match dim {
        Deref @ INTEGER { size: __dim_size, .. } => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: __dim_size.clone() })
        },
        Deref @ BOOLEAN { .. } => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 2 })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: ((var_field!((**ty).literals, Type::NFType::ENUMERATION)).len() as i32) })
        },
        Deref @ EXP { exp: __dim_exp, .. } => {
            __dim_exp.clone()
        },
        Deref @ RESIZABLE { exp: __dim_exp, .. } => {
            __dim_exp.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub(crate) fn lowerBoundExp(
    mut dim: &metamodelica::Ref<NFDimension>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &**dim {
        BOOLEAN { .. } => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        ENUM {
            enumType: __dim_enumType,
        } => Expression::makeEnumLiteral(__dim_enumType.clone(), 1)?,
        _ => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
    });
    Ok(exp)
}

pub(crate) fn expIsLowerBound(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> bool {
    let mut isStart: bool;
    isStart = (match &**exp {
        Expression::INTEGER { value: __exp_value } => __exp_value.clone() == 1,
        Expression::BOOLEAN { value: __exp_value } => __exp_value.clone() == false,
        Expression::ENUM_LITERAL { index: __exp_index, .. } => __exp_index.clone() == 1,
        _ => false,
    });
    isStart
}

pub(crate) fn upperBoundExp(
    mut dim: &metamodelica::Ref<NFDimension>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match dim {
        Deref @ INTEGER { size: __dim_size, .. } => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: __dim_size.clone() })
        },
        Deref @ BOOLEAN { .. } => {
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Expression::makeEnumLiteral(ty.clone(), ((var_field!((**ty).literals, Type::NFType::ENUMERATION)).len() as i32))?
        },
        Deref @ EXP { exp: __dim_exp, .. } => {
            __dim_exp.clone()
        },
        Deref @ RESIZABLE { exp: __dim_exp, .. } => {
            __dim_exp.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(exp)
}

pub(crate) fn expIsUpperBound(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut dim: &metamodelica::Ref<NFDimension>,
) -> bool {
    let mut isEnd: bool;
    isEnd = (::match_deref::match_deref! { match (exp, dim) {
        (Deref @ Expression::INTEGER { .. }, Deref @ INTEGER { .. }) => {
            var_field!((**exp).value, Expression::NFExpression::INTEGER).clone() == var_field!((**dim).size, NFDimension::INTEGER).clone()
        },
        (Deref @ Expression::BOOLEAN { .. }, _) => {
            var_field!((**exp).value, Expression::NFExpression::BOOLEAN).clone() == true
        },
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } }) => {
            var_field!((**exp).index, Expression::NFExpression::ENUM_LITERAL).clone() == ((var_field!((**ty).literals, Type::NFType::ENUMERATION)).len() as i32)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEnd
}

pub(crate) fn variability(mut dim: &metamodelica::Ref<NFDimension>) -> Result<Variability> {
    let mut var: Variability;
    var = (match &**dim {
        INTEGER { var: __dim_var, .. } => __dim_var.clone(),
        BOOLEAN { .. } => Variability::CONSTANT.clone(),
        ENUM { .. } => Variability::CONSTANT.clone(),
        EXP { var: __dim_var, .. } => __dim_var.clone(),
        RESIZABLE { var: __dim_var, .. } => __dim_var.clone(),
        UNKNOWN { .. } => Variability::CONTINUOUS.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(var)
}

pub(crate) fn mapExp(
    mut dim: metamodelica::Ref<NFDimension>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFDimension>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outDim: metamodelica::Ref<NFDimension>;
    outDim = (match &*dim {
        UNTYPED {
            dimension: e1,
            isProcessing: __dim_isProcessing,
        } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                dim
            } else {
                metamodelica::Ref::new(NFDimension::UNTYPED {
                    dimension: e2,
                    isProcessing: __dim_isProcessing.clone(),
                })
            }
        }
        EXP {
            exp: e1,
            var: __dim_var,
        } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                dim
            } else {
                fromExp(e2, __dim_var.clone())?
            }
        }
        RESIZABLE {
            exp: e1,
            var: __dim_var,
            ..
        } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                dim
            } else {
                fromExp(e2, __dim_var.clone())?
            }
        }
        _ => dim,
    });
    Ok(outDim)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut dim: &metamodelica::Ref<NFDimension>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT;
    outArg = (match &**dim {
        UNTYPED {
            dimension: __dim_dimension,
            ..
        } => Expression::fold(__dim_dimension.clone(), func.clone(), arg)?,
        EXP { exp: __dim_exp, .. } => Expression::fold(__dim_exp.clone(), func.clone(), arg)?,
        RESIZABLE { exp: __dim_exp, .. } => Expression::fold(__dim_exp.clone(), func.clone(), arg)?,
        _ => arg,
    });
    Ok(outArg)
}

pub(crate) fn foldExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut dims: &metamodelica::List<metamodelica::Ref<NFDimension>>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut dim in &**dims {
        arg = foldExp(metamodelica::AsArg::as_arg(&dim), func.clone(), arg)?;
    }
    Ok(arg)
}

pub(crate) fn eval(
    mut dim: metamodelica::Ref<NFDimension>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<NFDimension>> {
    let mut outDim: metamodelica::Ref<NFDimension>;
    outDim = (match &*dim {
        EXP {
            exp: __dim_exp,
            var: __dim_var,
        } => fromExp(Ceval::evalExp(__dim_exp.clone(), target)?, __dim_var.clone())?,
        RESIZABLE { exp: __dim_exp, .. } => {
            assign_variant_field!(dim => NFDimension::RESIZABLE; exp = Ceval::evalExp(__dim_exp.clone(), target)?);
            dim
        }
        _ => dim,
    });
    Ok(outDim)
}

pub(crate) fn simplify(mut dim: metamodelica::Ref<NFDimension>) -> Result<metamodelica::Ref<NFDimension>> {
    let mut dim: metamodelica::Ref<NFDimension> = dim;
    dim = (match &*dim {
        EXP { exp: __dim_exp, .. } => {
            let mut simple: metamodelica::Ref<Expression::NFExpression>;
            simple = SimplifyExp::simplify(__dim_exp.clone(), false)?;
            fromExp(simple.clone(), Expression::variability(simple)?)?
        }
        RESIZABLE { exp: __dim_exp, .. } => {
            assign_variant_field!(dim => NFDimension::RESIZABLE; exp = SimplifyExp::simplify(__dim_exp.clone(), false)?);
            dim
        }
        _ => dim,
    });
    Ok(dim)
}

pub(crate) fn typeOf(mut dim: &metamodelica::Ref<NFDimension>) -> metamodelica::Ref<Type::NFType> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = (match &**dim {
        INTEGER { .. } => crate::NFType::interned_INTEGER(),
        BOOLEAN { .. } => crate::NFType::interned_BOOLEAN(),
        ENUM {
            enumType: __dim_enumType,
        } => __dim_enumType.clone(),
        EXP { exp: __dim_exp, .. } => Expression::typeOf(__dim_exp.clone()),
        RESIZABLE { exp: __dim_exp, .. } => Expression::typeOf(__dim_exp.clone()),
        _ => crate::NFType::interned_UNKNOWN(),
    });
    ty
}
