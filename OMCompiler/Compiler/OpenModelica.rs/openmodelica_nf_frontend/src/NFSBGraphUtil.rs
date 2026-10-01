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

use crate::NFCeval as Ceval;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstContext;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use openmodelica_ast::Absyn;
use openmodelica_util::Error;
use openmodelica_util::SBAtomicSet;
use openmodelica_util::SBGraph::IncidenceList;
use openmodelica_util::SBGraph::VertexDescriptor;
use openmodelica_util::SBInterval;
use openmodelica_util::SBLinearMap;
use openmodelica_util::SBMultiInterval;
use openmodelica_util::SBPWLinearMap;
use openmodelica_util::SBSet;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;

pub(crate) fn multiIntervalFromDimensions(
    mut dims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
) -> Result<metamodelica::Ref<SBMultiInterval::SBMultiInterval>> {
    let mut multiInt: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut new_vCount: metamodelica::Ref<Vector::Vector<i32>>;
    let mut vc: i32;
    let mut dim_size: i32;
    let mut index: i32;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut int: metamodelica::Ref<SBInterval::SBInterval>;
    if (dims).is_empty() {
        vc = Vector::get(vCount.clone(), 1)?;
        Vector::update(vCount.clone(), 1, vc + 1)?;
        multiInt = SBMultiInterval::fromArray(arrayCreate(Vector::size(vCount), SBInterval::new(vc, 1, vc)))?;
    } else {
        ints = arrayCreate(Vector::size(vCount.clone()), SBInterval::newEmpty());
        new_vCount = Vector::copy(vCount.clone());
        index = 1;
        for mut dim in &**dims {
            if !(Dimension::isKnown(metamodelica::AsArg::as_arg(&dim), false)) {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFSBGraphUtil.multiIntervalFromDimensions"));
                        __mm_s.push_str(&*literal!(": unknown dimension "));
                        __mm_s.push_str(&*Dimension::toString(metamodelica::AsArg::as_arg(&dim))?);
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFSBGraphUtil.mo")),
                )?;
            }
            dim_size = Dimension::size(metamodelica::AsArg::as_arg(&dim), false)?;
            vc = Vector::get(vCount.clone(), index)?;
            int = SBInterval::new(vc, 1, vc + dim_size - 1);
            if SBInterval::isEmpty(&int) {
                ints = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                break;
            } else {
                {
                    let __cell0 = int;
                    let __idx0 = index;
                    *metamodelica::index_mut_checked(&mut ints.clone().borrow_mut(), __idx0)? = __cell0;
                }
                Vector::update(new_vCount.clone(), index, vc + dim_size)?;
            }
            index = index + 1;
        }
        for mut i in ((dims).len() as i32) + 1..=Vector::size(vCount.clone()) {
            vc = Vector::get(vCount.clone(), 1)?;
            {
                let __cell1 = SBInterval::new(vc, 1, vc);
                let __idx1 = i;
                *metamodelica::index_mut_checked(&mut ints.clone().borrow_mut(), __idx1)? = __cell1;
            }
        }
        multiInt = SBMultiInterval::fromArray(ints.clone())?;
        if !(SBMultiInterval::isEmpty(&multiInt)) {
            Vector::swap(new_vCount, vCount);
        }
    }
    Ok(multiInt)
}

pub(crate) fn multiIntervalFromSubscripts(
    mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut multiInt: metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
) -> Result<metamodelica::Ref<SBMultiInterval::SBMultiInterval>> {
    let mut multiInt: metamodelica::Ref<SBMultiInterval::SBMultiInterval> = multiInt;
    let mut mi: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut miv: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut int: metamodelica::Ref<SBInterval::SBInterval>;
    let mut index: i32;
    let mut aux_lo: i32;
    let mut sub_exp: metamodelica::Ref<Expression::NFExpression>;
    miv = SBMultiInterval::intervals(&multiInt);
    if (subs).is_empty() {
        mi = Array::map(
            miv.clone(),
            &move |__a0: metamodelica::Ref<SBInterval::SBInterval>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(make_lo_interval(&__a0))
            },
        )?;
    } else {
        index = 1;
        mi = metamodelica::arrayFromVec(miv.clone().borrow().clone());
        for mut s in &**subs {
            sub_exp = evalCrefs(Subscript::toExp(metamodelica::AsArg::as_arg(&s))?)?;
            int = intervalFromExp(sub_exp)?;
            aux_lo = SBInterval::lowerBound(
                &({
                    let __elt = (*metamodelica::index_checked(&miv.borrow(), index)?).clone();
                    __elt
                }),
            ) - 1;
            int = SBInterval::new(
                aux_lo + SBInterval::lowerBound(&int),
                SBInterval::stepValue(&int),
                aux_lo + SBInterval::upperBound(&int),
            );
            if !(SBInterval::isEmpty(&int)) {
                {
                    let __cell0 = int;
                    let __idx0 = index;
                    *metamodelica::index_mut_checked(&mut mi.clone().borrow_mut(), __idx0)? = __cell0;
                }
            } else {
                mi = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                break;
            }
            index = index + 1;
        }
        for mut i in ((subs).len() as i32) + 1..=metamodelica::arrayLength(mi.clone()) {
            aux_lo = SBInterval::lowerBound(
                &({
                    let __elt = (*metamodelica::index_checked(&miv.borrow(), i)?).clone();
                    __elt
                }),
            );
            {
                let __cell1 = SBInterval::new(aux_lo, 1, aux_lo);
                let __idx1 = index;
                *metamodelica::index_mut_checked(&mut mi.clone().borrow_mut(), __idx1)? = __cell1;
            }
        }
    }
    multiInt = SBMultiInterval::fromArray(mi.clone())?;
    Ok(multiInt)
}

pub(crate) fn make_lo_interval(
    mut i: &metamodelica::Ref<SBInterval::SBInterval>,
) -> metamodelica::Ref<SBInterval::SBInterval> {
    let mut res: metamodelica::Ref<SBInterval::SBInterval>;
    let mut lo: i32 = SBInterval::lowerBound(i);
    res = SBInterval::new(lo, 1, lo);
    res
}

pub(crate) fn evalCrefs(
    mut e: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    fn evalCref(
        mut e: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut outExp: metamodelica::Ref<Expression::NFExpression>;
        if Expression::isCref(&e) {
            outExp = Ceval::evalExp(
                e,
                &(Ceval::EvalTarget::new(Absyn::dummyInfo.clone(), NFInstContext::ITERATION_RANGE.clone(), None)),
            )?;
        } else {
            outExp = e;
        }
        Ok(outExp)
    }

    let mut e: metamodelica::Ref<Expression::NFExpression> = e;
    e = Expression::map(
        e,
        (std::sync::Arc::new(evalCref)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(e)
}

pub(crate) fn intervalFromExp(
    mut e: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<SBInterval::SBInterval>> {
    let mut i: metamodelica::Ref<SBInterval::SBInterval>;
    i = (match &*e {
        Expression::INTEGER { value: __e_value } => SBInterval::new(__e_value.clone(), 1, __e_value.clone()),
        Expression::BOOLEAN { value: __e_value } => {
            SBInterval::new(Util::boolInt(__e_value.clone()), 1, Util::boolInt(__e_value.clone()))
        }
        Expression::REAL { value: __e_value } => SBInterval::new(
            ((__e_value.clone()).0.floor() as i32),
            1,
            ((__e_value.clone()).0.floor() as i32),
        ),
        Expression::BINARY {
            exp1: __e_exp1,
            exp2: __e_exp2,
            operator: __e_operator,
        } => intervalFromBinaryExp(__e_exp1.clone(), __e_operator.clone(), __e_exp2.clone())?,
        Expression::UNARY { exp: __e_exp, .. } => intervalFromUnaryExp(__e_exp.clone())?,
        Expression::RANGE { .. } => intervalFromRange(e)?,
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSBGraphUtil.intervalFromExp"));
                    __mm_s.push_str(&*literal!(" got unknown expression "));
                    __mm_s.push_str(&*Expression::toString(e)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSBGraphUtil.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(i)
}

pub(crate) fn intervalFromBinaryExp(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<SBInterval::SBInterval>> {
    let mut i: metamodelica::Ref<SBInterval::SBInterval> =
        <metamodelica::Ref<SBInterval::SBInterval> as ::std::default::Default>::default();
    let mut lhs_i: metamodelica::Ref<SBInterval::SBInterval>;
    let mut rhs_i: metamodelica::Ref<SBInterval::SBInterval>;
    let mut lhs_sz: i32;
    let mut rhs_sz: i32;
    let mut res: i32;
    let mut llo: i32;
    let mut rlo: i32;
    let mut lhi: i32;
    let mut rhi: i32;
    let mut step: i32;
    lhs_i = intervalFromExp(lhs.clone())?;
    rhs_i = intervalFromExp(rhs.clone())?;
    lhs_sz = SBInterval::size(&lhs_i);
    rhs_sz = SBInterval::size(&rhs_i);
    llo = SBInterval::lowerBound(&lhs_i);
    rlo = SBInterval::lowerBound(&rhs_i);
    if lhs_sz == 1 && rhs_sz == 1 {
        let __pa0 = ::match_deref::match_deref! { match &(Ceval::evalBinaryOp_dispatch(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: llo }), op, metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: rlo }), &(Ceval::noTarget().clone()))?) {
            Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        res = metamodelica::Own::own(__pa0);
        i = SBInterval::new(res, 1, res);
    } else if lhs_sz == 1 || rhs_sz == 1 {
        lhi = SBInterval::upperBound(&lhs_i);
        rhi = SBInterval::upperBound(&rhs_i);
        step = SBInterval::stepValue(&(if (lhs_sz == 1) { rhs_i } else { lhs_i }));
        i = (match op.op.clone() {
            Operator::Op::ADD => SBInterval::new(llo + rlo, step, lhi + rhi),
            Operator::Op::SUB => SBInterval::new(llo - rlo, step, lhi - rhi),
            Operator::Op::MUL => SBInterval::new(llo * rlo, llo * step, lhi * rhi),
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFSBGraphUtil.intervalFromBinaryExp"));
                        __mm_s.push_str(&*literal!(" got unknown operator "));
                        __mm_s.push_str(&*Operator::symbol(&op, &(literal!(" ")))?);
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFSBGraphUtil.mo")),
                )?;
                return Err("fail");
            }
        });
    } else {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFSBGraphUtil.intervalFromBinaryExp"));
                __mm_s.push_str(&*literal!(" got unknown expression "));
                __mm_s.push_str(&*Expression::toString(metamodelica::Ref::new(
                    Expression::NFExpression::BINARY {
                        exp1: lhs,
                        operator: op,
                        exp2: rhs,
                    },
                ))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFSBGraphUtil.mo")),
        )?;
    }
    Ok(i)
}

pub(crate) fn intervalFromUnaryExp(
    mut e: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<SBInterval::SBInterval>> {
    let mut i: metamodelica::Ref<SBInterval::SBInterval>;
    i = intervalFromExp(e)?;
    i = SBInterval::new(-(SBInterval::lowerBound(&i)), 1, -(SBInterval::upperBound(&i)));
    Ok(i)
}

pub(crate) fn intervalFromRange(
    mut e: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<SBInterval::SBInterval>> {
    let mut i: metamodelica::Ref<SBInterval::SBInterval>;
    let mut start: metamodelica::Ref<Expression::NFExpression>;
    let mut stop: metamodelica::Ref<Expression::NFExpression>;
    let mut ostep: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut lo: i32;
    let mut step: i32;
    let mut hi: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(SimplifyExp::simplify(e, false)?) {
        Deref @ Expression::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    start = metamodelica::Own::own(__pa0);
    ostep = metamodelica::Own::own(__pa1);
    stop = metamodelica::Own::own(__pa2);
    lo = Expression::toInteger(&start)?;
    hi = Expression::toInteger(&stop)?;
    if (ostep).is_some() {
        step = Expression::toInteger(&(ostep.ok_or("pattern mismatch")?))?;
    } else {
        step = 1;
    }
    i = SBInterval::new(lo, step, hi);
    Ok(i)
}

pub(crate) fn linearMapFromIntervals(
    mut d1: i32,
    mut d2: i32,
    mut mi1: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut mi2: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut eCount: metamodelica::Ref<Vector::Vector<i32>>,
) -> Result<(
    ArcStr,
    metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
)> {
    let mut name: ArcStr;
    let mut pw1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut pw2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut ints1: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut ints2: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut mi: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut mi1_sz: i32;
    let mut mi2_sz: i32;
    let mut sz: i32;
    let mut sz1: i32;
    let mut sz2: i32;
    let mut count: i32;
    let mut aux_ec: i32;
    let mut g1: metamodelica::Array<metamodelica::Real>;
    let mut g2: metamodelica::Array<metamodelica::Real>;
    let mut o1: metamodelica::Array<metamodelica::Real>;
    let mut o2: metamodelica::Array<metamodelica::Real>;
    let mut g1i: metamodelica::Real;
    let mut g2i: metamodelica::Real;
    let mut o1i: metamodelica::Real;
    let mut o2i: metamodelica::Real;
    let mut i1: metamodelica::Ref<SBInterval::SBInterval>;
    let mut i2: metamodelica::Ref<SBInterval::SBInterval>;
    let mut new_ec: metamodelica::Ref<Vector::Vector<i32>>;
    let mut s: metamodelica::Ref<SBSet::SBSet>;
    let mut lm1: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut lm2: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    ints1 = SBMultiInterval::intervals(mi1);
    mi1_sz = SBMultiInterval::size(mi1);
    ints2 = SBMultiInterval::intervals(mi2);
    mi2_sz = SBMultiInterval::size(mi2);
    if SBMultiInterval::ndim(mi1) != SBMultiInterval::ndim(mi2) && mi1_sz != 1 && mi2_sz != 1 {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFSBGraphUtil.linearMapFromIntervals"));
                __mm_s.push_str(&*literal!(" got incompatible connect"));
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFSBGraphUtil.mo")),
        )?;
    }
    sz = metamodelica::arrayLength(ints1.clone());
    g1 = metamodelica::arrayCreate(sz, metamodelica::OrderedFloat(0.0_f64));
    g2 = metamodelica::arrayCreate(sz, metamodelica::OrderedFloat(0.0_f64));
    o1 = metamodelica::arrayCreate(sz, metamodelica::OrderedFloat(0.0_f64));
    o2 = metamodelica::arrayCreate(sz, metamodelica::OrderedFloat(0.0_f64));
    mi = metamodelica::arrayCreate(
        sz,
        ({
            let __elt = (*metamodelica::index_checked(&ints1.borrow(), 1)?).clone();
            __elt
        }),
    );
    new_ec = Vector::new(0);
    for mut i in 1..=sz {
        sz1 = SBInterval::size(
            &({
                let __elt = (*metamodelica::index_checked(&ints1.borrow(), i)?).clone();
                __elt
            }),
        );
        sz2 = SBInterval::size(
            &({
                let __elt = (*metamodelica::index_checked(&ints2.borrow(), i)?).clone();
                __elt
            }),
        );
        if sz1 != sz2 && sz1 != 1 && sz2 != 1 {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSBGraphUtil.linearMapFromIntervals"));
                    __mm_s.push_str(&*literal!(" got incompatible connect"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSBGraphUtil.mo")),
            )?;
        }
        count = std::cmp::max(sz1, sz2);
        aux_ec = Vector::get(eCount.clone(), i)?;
        {
            let __cell0 = SBInterval::new(aux_ec, 1, aux_ec + count - 1);
            let __idx0 = i;
            let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(mi.clone().clone(), __idx0, __cell0) }?;
        }
        i1 = ({
            let __elt = (*metamodelica::index_checked(&ints1.borrow(), i)?).clone();
            __elt
        });
        i2 = ({
            let __elt = (*metamodelica::index_checked(&ints2.borrow(), i)?).clone();
            __elt
        });
        if sz1 == 1 {
            {
                let __cell1 = metamodelica::OrderedFloat(0.0_f64);
                let __idx1 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(g1.clone().clone(), __idx1, __cell1) }?;
            }
            {
                let __cell2 = metamodelica::OrderedFloat((SBInterval::lowerBound(&i1)) as f64);
                let __idx2 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(o1.clone().clone(), __idx2, __cell2) }?;
            }
        } else {
            g1i = metamodelica::OrderedFloat((SBInterval::stepValue(&i1)) as f64);
            o1i = -(g1i * metamodelica::OrderedFloat((aux_ec) as f64))
                + metamodelica::OrderedFloat((SBInterval::lowerBound(&i1)) as f64);
            {
                let __cell3 = g1i;
                let __idx3 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(g1.clone().clone(), __idx3, __cell3) }?;
            }
            {
                let __cell4 = o1i;
                let __idx4 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(o1.clone().clone(), __idx4, __cell4) }?;
            }
        }
        if sz2 == 1 {
            {
                let __cell5 = metamodelica::OrderedFloat(0.0_f64);
                let __idx5 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(g2.clone().clone(), __idx5, __cell5) }?;
            }
            {
                let __cell6 = metamodelica::OrderedFloat((SBInterval::lowerBound(&i2)) as f64);
                let __idx6 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(o2.clone().clone(), __idx6, __cell6) }?;
            }
        } else {
            g2i = metamodelica::OrderedFloat((SBInterval::stepValue(&i2)) as f64);
            o2i = -(g2i * metamodelica::OrderedFloat((aux_ec) as f64))
                + metamodelica::OrderedFloat((SBInterval::lowerBound(&i2)) as f64);
            {
                let __cell7 = g2i;
                let __idx7 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(g2.clone().clone(), __idx7, __cell7) }?;
            }
            {
                let __cell8 = o2i;
                let __idx8 = i;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(o2.clone().clone(), __idx8, __cell8) }?;
            }
        }
        Vector::push(new_ec.clone(), aux_ec + count);
    }
    Vector::swap(eCount, new_ec);
    s = SBSet::newEmpty();
    s = SBSet::addAtomicSet(SBAtomicSet::new(&(SBMultiInterval::fromArray(mi.clone())?)), s)?;
    lm1 = SBLinearMap::new(g1.clone(), o1.clone())?;
    lm2 = SBLinearMap::new(g2.clone(), o2.clone())?;
    pw1 = SBPWLinearMap::newScalar(s.clone(), lm1);
    pw2 = SBPWLinearMap::newScalar(s, lm2);
    name = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("E"));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", System::tmpTick())));
        ArcStr::from(__mm_s)
    };
    Ok((name, pw1, pw2))
}
