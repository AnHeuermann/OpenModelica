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

use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFType as Type;
use openmodelica_util::Error;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum NFRangeIterator {
    INT_RANGE {
        current: i32,
        last: i32,
    },
    INT_STEP_RANGE {
        current: i32,
        stepsize: i32,
        last: i32,
    },
    REAL_RANGE {
        start: metamodelica::Real,
        stepsize: metamodelica::Real,
        current: i32,
        steps: i32,
    },
    ARRAY_RANGE {
        values: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
        index: i32,
    },
    INVALID_RANGE {
        exp: metamodelica::Ref<Expression::NFExpression>,
    },
}
impl metamodelica::gc::MMTrace for NFRangeIterator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFRangeIterator::INT_RANGE { current, last } => {
                metamodelica::gc::MMTrace::mm_accept(current, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(last, __mmv)?;
                Ok(())
            }
            NFRangeIterator::INT_STEP_RANGE {
                current,
                stepsize,
                last,
            } => {
                metamodelica::gc::MMTrace::mm_accept(current, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stepsize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(last, __mmv)?;
                Ok(())
            }
            NFRangeIterator::REAL_RANGE {
                start,
                stepsize,
                current,
                steps,
            } => {
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stepsize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(current, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(steps, __mmv)?;
                Ok(())
            }
            NFRangeIterator::ARRAY_RANGE { values, index } => {
                metamodelica::gc::MMTrace::mm_accept(values, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            NFRangeIterator::INVALID_RANGE { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NFRangeIterator {
    fn default() -> Self {
        Self::INVALID_RANGE {
            exp: Default::default(),
        }
    }
}
pub(crate) use self::NFRangeIterator::{ARRAY_RANGE, INT_RANGE, INT_STEP_RANGE, INVALID_RANGE, REAL_RANGE};
pub(crate) fn isValid(mut iterator: &metamodelica::Ref<NFRangeIterator>) -> bool {
    let mut isValid: bool;
    isValid = (match &**iterator {
        INVALID_RANGE { .. } => false,
        _ => true,
    });
    isValid
}

pub(crate) fn fromExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<NFRangeIterator>> {
    let mut iterator: metamodelica::Ref<NFRangeIterator>;
    iterator = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { .. } => {
            metamodelica::Ref::new(NFRangeIterator::ARRAY_RANGE { values: var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), index: 1 })
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: istart }, step: Some(Deref @ Expression::INTEGER { value: istep }), stop: Deref @ Expression::INTEGER { value: istop }, .. } => {
            metamodelica::Ref::new(NFRangeIterator::INT_STEP_RANGE { current: istart.clone(), stepsize: istep.clone(), last: istop.clone() })
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: istart }, step: None, stop: Deref @ Expression::INTEGER { value: istop }, .. } => {
            metamodelica::Ref::new(NFRangeIterator::INT_RANGE { current: istart.clone(), last: istop.clone() })
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::REAL { value: rstart }, step: Some(Deref @ Expression::REAL { value: rstep }), stop: Deref @ Expression::REAL { value: rstop }, .. } => {
            metamodelica::Ref::new(NFRangeIterator::REAL_RANGE { start: rstart.clone(), stepsize: rstep.clone(), current: 0, steps: Util::realRangeSize(rstart.clone(), rstep.clone(), rstop.clone())? })
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::REAL { value: rstart }, step: None, stop: Deref @ Expression::REAL { value: rstop }, .. } => {
            metamodelica::Ref::new(NFRangeIterator::REAL_RANGE { start: rstart.clone(), stepsize: metamodelica::OrderedFloat(1.0_f64), current: 0, steps: Util::realRangeSize(rstart.clone(), metamodelica::OrderedFloat(1.0_f64), rstop.clone())? })
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::BOOLEAN { value: bstart }, stop: Deref @ Expression::BOOLEAN { value: bstop }, .. } => {
            metamodelica::Ref::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut b in (({let __bs = bstart.clone(); let __be = bstop.clone(); if !__bs && !__be { vec![false] } else if !__bs && __be { vec![false, true] } else if __bs && __be { vec![true] } else { Vec::<bool>::new() }})).into_iter() {
            let __x = metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: b.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::ENUM_LITERAL { ty, index: istart, .. }, step: None, stop: Deref @ Expression::ENUM_LITERAL { index: istop, .. }, .. } => {
            let mut literals: metamodelica::List<ArcStr>;
            let mut values: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
                Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            literals = metamodelica::Own::own(__pa0);
            values = metamodelica::nil();
            if istart.clone() <= istop.clone() {
                for mut i in 2..=istart.clone() {
                    literals = (literals).rest()?;
                }
                for mut i in istart.clone()..=istop.clone() {
                    values = metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (literals).head().cloned()?, index: i }), values);
                    literals = (literals).rest()?;
                }
                values = values.reverse();
            }
            metamodelica::Ref::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(values.into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: ty @ Deref @ Type::ENUMERATION { literals, .. }, .. } } => {
            let mut istep: i32;
            let mut values: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            values = metamodelica::nil();
            istep = 0;
            for mut l in &*literals.clone() {
                istep = istep + 1;
                values = metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::ENUM_LITERAL { ty: ty.clone(), name: l.clone(), index: istep }), values);
            }
            metamodelica::Ref::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(values.into_iter().cloned().collect()), index: 1 })
        },
        _ => {
            metamodelica::Ref::new(NFRangeIterator::INVALID_RANGE { exp: exp })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub(crate) fn fromDim(
    mut dim: &metamodelica::Ref<Dimension::NFDimension>,
    mut resizable: bool,
) -> Result<metamodelica::Ref<NFRangeIterator>> {
    let mut iterator: metamodelica::Ref<NFRangeIterator>;
    iterator = (::match_deref::match_deref! { match dim {
        Deref @ Dimension::INTEGER { size: __dim_size, .. } => {
            metamodelica::Ref::new(NFRangeIterator::INT_RANGE { current: 1, last: __dim_size.clone() })
        },
        Deref @ Dimension::BOOLEAN => {
            metamodelica::Ref::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(list![metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }), metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })].into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Dimension::ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            metamodelica::Ref::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(Expression::makeEnumLiterals(ty.clone())?.into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Dimension::EXP { exp: __dim_exp, .. } => {
            fromExp(__dim_exp.clone())?
        },
        Deref @ Dimension::RESIZABLE { opt_size: __dim_opt_size, size: __dim_size, .. } => {
            metamodelica::Ref::new(NFRangeIterator::INT_RANGE { current: 1, last: if (resizable) {Util::getOptionOrDefault(__dim_opt_size.clone(), __dim_size.clone())} else {__dim_size.clone()} })
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFRangeIterator.fromDim")); __mm_s.push_str(&*literal!(" got unknown dim")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFRangeIterator.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub(crate) fn next(
    mut iterator: metamodelica::Ref<NFRangeIterator>,
) -> Result<(
    metamodelica::Ref<NFRangeIterator>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut iterator: metamodelica::Ref<NFRangeIterator> = iterator;
    let mut nextExp: metamodelica::Ref<Expression::NFExpression>;
    nextExp = (match &*iterator {
        INT_RANGE {
            current: __iterator_current,
            ..
        } => {
            nextExp = metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                value: __iterator_current.clone(),
            });
            assign_variant_field!(iterator => NFRangeIterator::INT_RANGE; current = __iterator_current.clone() + 1);
            nextExp
        }
        INT_STEP_RANGE {
            current: __iterator_current,
            stepsize: __iterator_stepsize,
            ..
        } => {
            nextExp = metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                value: __iterator_current.clone(),
            });
            assign_variant_field!(iterator => NFRangeIterator::INT_STEP_RANGE; current = __iterator_current.clone() + __iterator_stepsize.clone());
            nextExp
        }
        REAL_RANGE {
            current: __iterator_current,
            start: __iterator_start,
            stepsize: __iterator_stepsize,
            ..
        } => {
            nextExp = metamodelica::Ref::new(Expression::NFExpression::REAL {
                value: __iterator_start.clone()
                    + __iterator_stepsize.clone() * metamodelica::OrderedFloat((__iterator_current.clone()) as f64),
            });
            assign_variant_field!(iterator => NFRangeIterator::REAL_RANGE; current = __iterator_current.clone() + 1);
            nextExp
        }
        ARRAY_RANGE {
            index: __iterator_index,
            ..
        } => {
            nextExp = metamodelica::arrayGet(
                var_field!((*iterator).values, NFRangeIterator::ARRAY_RANGE).clone(),
                __iterator_index.clone(),
            )?;
            assign_variant_field!(iterator => NFRangeIterator::ARRAY_RANGE; index = __iterator_index.clone() + 1);
            nextExp
        }
        INVALID_RANGE { exp: __iterator_exp } => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFRangeIterator.next"));
                    __mm_s.push_str(&*literal!(" got invalid range "));
                    __mm_s.push_str(&*Expression::toString(__iterator_exp.clone())?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFRangeIterator.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((iterator, nextExp))
}

pub(crate) fn hasNext(mut iterator: &metamodelica::Ref<NFRangeIterator>) -> Result<bool> {
    let mut hasNext: bool;
    hasNext = (match &**iterator {
        INT_RANGE {
            current: __iterator_current,
            last: __iterator_last,
        } => __iterator_current.clone() <= __iterator_last.clone(),
        INT_STEP_RANGE {
            current: __iterator_current,
            last: __iterator_last,
            stepsize: __iterator_stepsize,
        } => {
            if (__iterator_stepsize.clone() > 0) {
                __iterator_current.clone() <= __iterator_last.clone()
            } else {
                __iterator_current.clone() >= __iterator_last.clone()
            }
        }
        REAL_RANGE {
            current: __iterator_current,
            steps: __iterator_steps,
            ..
        } => __iterator_current.clone() < __iterator_steps.clone(),
        ARRAY_RANGE {
            index: __iterator_index,
            ..
        } => {
            __iterator_index.clone()
                <= metamodelica::arrayLength(var_field!((**iterator).values, NFRangeIterator::ARRAY_RANGE).clone())
        }
        INVALID_RANGE { exp: __iterator_exp } => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFRangeIterator.hasNext"));
                    __mm_s.push_str(&*literal!(" got invalid range "));
                    __mm_s.push_str(&*Expression::toString(__iterator_exp.clone())?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFRangeIterator.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(hasNext)
}

pub(crate) fn toList(
    mut iterator: metamodelica::Ref<NFRangeIterator>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
        toListReverse(iterator.clone())?.reverse();
    Ok(expl)
}

pub(crate) fn toListReverse(
    mut iterator: metamodelica::Ref<NFRangeIterator>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut iter: metamodelica::Ref<NFRangeIterator> = iterator;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    while hasNext(&iter)? {
        (iter, exp) = next(iter)?;
        expl = metamodelica::cons(exp, expl);
    }
    Ok(expl)
}

pub(crate) fn map<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iterator: metamodelica::Ref<NFRangeIterator>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<T>,
) -> Result<metamodelica::List<T>> {
    pub type FuncT<T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<T> + 'static>;

    let mut lst: metamodelica::List<T> = metamodelica::nil();
    let mut iter: metamodelica::Ref<NFRangeIterator> = iterator;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    while hasNext(&iter)? {
        (iter, exp) = next(iter)?;
        lst = metamodelica::cons(func(exp)?, lst);
    }
    lst = lst.reverse();
    Ok(lst)
}

pub(crate) fn fold<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iterator: metamodelica::Ref<NFRangeIterator>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FuncT<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    let mut iter: metamodelica::Ref<NFRangeIterator> = iterator;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    while hasNext(&iter)? {
        (iter, exp) = next(iter)?;
        arg = func(exp, arg)?;
    }
    Ok(arg)
}
