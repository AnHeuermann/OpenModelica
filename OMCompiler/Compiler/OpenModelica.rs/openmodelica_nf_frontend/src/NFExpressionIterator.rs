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

use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFSimplifyExp as SimplifyExp;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFExpressionIterator {
    ARRAY_ITERATOR {
        arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
        index: i32,
        arrays: metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>,
    },
    SCALAR_ITERATOR {
        exp: metamodelica::Ref<Expression::NFExpression>,
    },
    EACH_ITERATOR {
        exp: metamodelica::Ref<Expression::NFExpression>,
    },
    NONE_ITERATOR,
}
impl metamodelica::gc::MMTrace for NFExpressionIterator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFExpressionIterator::ARRAY_ITERATOR { arr, index, arrays } => {
                metamodelica::gc::MMTrace::mm_accept(arr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(arrays, __mmv)?;
                Ok(())
            }
            NFExpressionIterator::SCALAR_ITERATOR { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpressionIterator::EACH_ITERATOR { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpressionIterator::NONE_ITERATOR => Ok(()),
        }
    }
}
impl NFExpressionIterator {
    pub fn interned_NONE_ITERATOR() -> metamodelica::Ref<NFExpressionIterator> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFExpressionIterator> = metamodelica::Ref::new(NFExpressionIterator::NONE_ITERATOR);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NONE_ITERATOR() -> metamodelica::Ref<NFExpressionIterator> {
    NFExpressionIterator::interned_NONE_ITERATOR()
}
impl Default for NFExpressionIterator {
    fn default() -> Self {
        Self::NONE_ITERATOR
    }
}
pub use self::NFExpressionIterator::{ARRAY_ITERATOR, EACH_ITERATOR, NONE_ITERATOR, SCALAR_ITERATOR};
pub(crate) fn toString(mut iter: &metamodelica::Ref<NFExpressionIterator>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**iter {
        ARRAY_ITERATOR {
            arrays: __iter_arrays, ..
        } => List::toStringCustom(
            __iter_arrays.clone(),
            &({
                let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> =
                    (std::sync::Arc::new(Expression::toString)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<ArcStr> + 'static,
                        >);
                let __pe_b2 = literal!("");
                let __pe_b3 = literal!("{");
                let __pe_b4 = literal!(", ");
                let __pe_b5 = literal!("}");
                let __pe_b6 = false;
                let __pe_b7 = 0;
                move |__pe_a0| {
                    Array::toString(
                        __pe_a0,
                        &*__pe_b1,
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                        __pe_b5.clone(),
                        __pe_b6.clone(),
                        __pe_b7.clone(),
                    )
                }
            }),
            literal!("[ARRY] array iterator:\n"),
            literal!(""),
            literal!("\n"),
            literal!(""),
            true,
            0,
        )?,
        SCALAR_ITERATOR { exp: __iter_exp } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[SCAL] scalar iterator: "));
            __mm_s.push_str(&*Expression::toString(__iter_exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        }
        EACH_ITERATOR { exp: __iter_exp } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[EACH] each iterator: "));
            __mm_s.push_str(&*Expression::toString(__iter_exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        }
        NONE_ITERATOR { .. } => literal!("[NONE] no iterator.\n"),
    });
    Ok(r#str)
}

pub fn fromExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut backend: bool,
    mut resize: bool,
) -> Result<metamodelica::Ref<NFExpressionIterator>> {
    let mut iterator: metamodelica::Ref<NFExpressionIterator>;
    iterator = (match &*exp {
        Expression::ARRAY { .. } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut expanded: bool;
            (e, expanded) = ExpandExp::expand(exp.clone(), backend, resize)?;
            if !(expanded) {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFExpressionIterator.fromExp"));
                        __mm_s.push_str(&*literal!(" got unexpandable expression `"));
                        __mm_s.push_str(&*Expression::toString(exp)?);
                        __mm_s.push_str(&*literal!("`"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFExpressionIterator.mo")),
                )?;
            }
            makeArrayIterator(&e)?
        }
        Expression::CREF { .. } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            (e, _) = ExpandExp::expandCref(exp, backend, false)?;
            iterator = (match &*e {
                Expression::ARRAY { .. } => fromExp(e, backend, resize)?,
                _ => metamodelica::Ref::new(NFExpressionIterator::SCALAR_ITERATOR { exp: e }),
            });
            iterator
        }
        _ => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut expanded: bool;
            (e, expanded) = ExpandExp::expand(exp.clone(), backend, resize)?;
            if (expanded) {
                if (Expression::isEqual(e.clone(), exp.clone())?) {
                    metamodelica::Ref::new(NFExpressionIterator::SCALAR_ITERATOR { exp: exp })
                } else {
                    fromExp(e, backend, resize)?
                }
            } else {
                crate::NFExpressionIterator::interned_NONE_ITERATOR()
            }
        }
    });
    Ok(iterator)
}

pub(crate) fn fromExpOpt(
    mut optExp: Option<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFExpressionIterator>> {
    let mut iterator: metamodelica::Ref<NFExpressionIterator>;
    iterator = (::match_deref::match_deref! { match &(optExp) {
        Some(exp) => {
            fromExp(exp.clone(), false, false)?
        },
        _ => {
            crate::NFExpressionIterator::interned_NONE_ITERATOR()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub(crate) fn fromBinding(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
) -> Result<metamodelica::Ref<NFExpressionIterator>> {
    let mut iterator: metamodelica::Ref<NFExpressionIterator>;
    iterator = (match &**binding {
        Binding::TYPED_BINDING {
            eachType: Binding::EachType::EACH,
            bindingExp: __binding_bindingExp,
            ..
        } => metamodelica::Ref::new(NFExpressionIterator::EACH_ITERATOR {
            exp: __binding_bindingExp.clone(),
        }),
        Binding::TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => fromExp(__binding_bindingExp.clone(), false, false)?,
        Binding::FLAT_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => metamodelica::Ref::new(NFExpressionIterator::EACH_ITERATOR {
            exp: __binding_bindingExp.clone(),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(iterator)
}

pub(crate) fn isUniform(mut iterator: &metamodelica::Ref<NFExpressionIterator>) -> Result<bool> {
    let mut uniform: bool;
    uniform = (match &**iterator {
        ARRAY_ITERATOR {
            arrays: __iterator_arrays,
            ..
        } => isUniformArrays(
            &(metamodelica::cons(
                var_field!((**iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone(),
                __iterator_arrays.clone(),
            )),
        )?,
        EACH_ITERATOR { .. } => true,
        NONE_ITERATOR { .. } => true,
        _ => false,
    });
    Ok(uniform)
}

pub(crate) fn isUniformArrays(
    mut arrays: &metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>,
) -> Result<bool> {
    let mut uniform: bool = true;
    let mut first: Option<metamodelica::Ref<Expression::NFExpression>> = None;
    for mut arr in &**arrays {
        let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut e in __range0 {
            if (first).is_none() {
                first = Some(e);
            } else if !(referenceEq(&*(&*e), &*(Util::getOption(first.clone())?))) {
                uniform = false;
                return Ok(uniform);
            }
        }
    }
    Ok(uniform)
}

pub fn hasNext(mut iterator: &metamodelica::Ref<NFExpressionIterator>) -> bool {
    let mut hasNext: bool;
    hasNext = (match &**iterator {
        ARRAY_ITERATOR {
            index: __iterator_index,
            ..
        } => {
            __iterator_index.clone()
                <= metamodelica::arrayLength(var_field!((**iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone())
        }
        SCALAR_ITERATOR { .. } => true,
        EACH_ITERATOR { .. } => true,
        NONE_ITERATOR { .. } => false,
    });
    hasNext
}

pub fn next(
    mut iterator: metamodelica::Ref<NFExpressionIterator>,
) -> Result<(
    metamodelica::Ref<NFExpressionIterator>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut iterator: metamodelica::Ref<NFExpressionIterator> = iterator;
    let mut nextExp: metamodelica::Ref<Expression::NFExpression>;
    (iterator, nextExp) = (match &*iterator.clone() {
        ARRAY_ITERATOR {
            index: __iterator_index,
            ..
        } => {
            let mut next: metamodelica::Ref<Expression::NFExpression>;
            let mut arrs: metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>;
            next = metamodelica::arrayGet(
                var_field!((*iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone(),
                __iterator_index.clone(),
            )?;
            if var_field!((*iterator).index, NFExpressionIterator::ARRAY_ITERATOR).clone()
                >= metamodelica::arrayLength(var_field!((*iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone())
            {
                arrs = var_field!((*iterator).arrays, NFExpressionIterator::ARRAY_ITERATOR).clone();
                while !((arrs).is_empty()) && (arrs).head().cloned()?.borrow().is_empty() {
                    arrs = (arrs).rest()?;
                }
                if (arrs).is_empty() {
                    iterator = metamodelica::Ref::new(NFExpressionIterator::ARRAY_ITERATOR {
                        arr: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                        index: 1,
                        arrays: metamodelica::nil(),
                    });
                } else {
                    iterator = metamodelica::Ref::new(NFExpressionIterator::ARRAY_ITERATOR {
                        arr: (arrs).head().cloned()?,
                        index: 1,
                        arrays: (arrs).rest()?,
                    });
                }
            } else {
                assign_variant_field!(iterator => NFExpressionIterator::ARRAY_ITERATOR; index = var_field!((*iterator).index, NFExpressionIterator::ARRAY_ITERATOR).clone() + 1);
            }
            (iterator, next)
        }
        SCALAR_ITERATOR { exp: __iterator_exp } => (
            crate::NFExpressionIterator::interned_NONE_ITERATOR(),
            __iterator_exp.clone(),
        ),
        EACH_ITERATOR { exp: __iterator_exp } => (iterator, __iterator_exp.clone()),
        _ => return Err("match: no arm matched"),
    });
    Ok((iterator, nextExp))
}

pub(crate) fn nextOpt(
    mut iterator: metamodelica::Ref<NFExpressionIterator>,
) -> Result<(
    metamodelica::Ref<NFExpressionIterator>,
    Option<metamodelica::Ref<Expression::NFExpression>>,
)> {
    let mut iterator: metamodelica::Ref<NFExpressionIterator> = iterator;
    let mut nextExp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    if hasNext(&iterator) {
        (iterator, exp) = next(iterator)?;
        nextExp = Some(exp);
    } else {
        nextExp = None;
    }
    Ok((iterator, nextExp))
}

pub(crate) fn toList(
    mut iterator: metamodelica::Ref<NFExpressionIterator>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut iter: metamodelica::Ref<NFExpressionIterator>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    iter = iterator;
    while hasNext(&iter) {
        (iter, exp) = next(iter)?;
        expl = metamodelica::cons(exp, expl);
    }
    expl = expl.reverse();
    Ok(expl)
}

pub(crate) fn isSubscriptedArrayCall(
    mut iterator: &metamodelica::Ref<NFExpressionIterator>,
    mut trySimplify: bool,
) -> Result<bool> {
    fn is_sub_call(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut trySimplify: bool) -> Result<bool> {
        let mut res: bool;
        res = (::match_deref::match_deref! { match exp {
            Deref @ Expression::SUBSCRIPTED_EXP { exp: Deref @ Expression::CALL { .. }, .. } => !(trySimplify) || Expression::isCall(&(SimplifyExp::simplify(var_field!((**exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), false)?)),
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(res)
    }

    let mut b: bool;
    b = (match &**iterator {
        ARRAY_ITERATOR { .. } => is_sub_call(
            &(metamodelica::arrayGet(
                var_field!((**iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone(),
                1,
            )?),
            trySimplify,
        )?,
        _ => false,
    });
    Ok(b)
}

fn makeArrayIterator(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<NFExpressionIterator>> {
    let mut iterator: metamodelica::Ref<NFExpressionIterator>;
    let mut arrays: metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>;
    arrays = flattenArray(exp, metamodelica::nil())?;
    if (arrays).is_empty() {
        iterator = metamodelica::Ref::new(NFExpressionIterator::ARRAY_ITERATOR {
            arr: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            index: 1,
            arrays: arrays,
        });
    } else {
        iterator = metamodelica::Ref::new(NFExpressionIterator::ARRAY_ITERATOR {
            arr: (arrays).head().cloned()?,
            index: 1,
            arrays: (arrays).rest()?,
        });
    }
    Ok(iterator)
}

fn flattenArray(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut arrays: metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>,
) -> Result<metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>> {
    let mut arrays: metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>> = arrays;
    arrays = flattenArray_impl(exp, metamodelica::nil())?;
    arrays = metamodelica::Dangerous::listReverseInPlace(arrays);
    while !((arrays).is_empty()) && (arrays).head().cloned()?.borrow().is_empty() {
        arrays = (arrays).rest()?;
    }
    Ok(arrays)
}

fn flattenArray_impl(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut arrays: metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>,
) -> Result<metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>> {
    let mut arrays: metamodelica::List<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>> = arrays;
    if Expression::isVector(exp)? {
        arrays = metamodelica::cons(Expression::arrayElements(exp)?, arrays);
    } else {
        let __range0 = Expression::arrayElements(exp)?
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut e in __range0 {
            arrays = flattenArray_impl(&e, arrays)?;
        }
    }
    Ok(arrays)
}
