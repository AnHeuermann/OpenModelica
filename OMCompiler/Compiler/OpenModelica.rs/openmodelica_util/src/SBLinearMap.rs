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

use crate::SBAtomicSet;
use crate::SBInterval;
use crate::SBMultiInterval;
use crate::SBSet;
use crate::System;
use crate::UnorderedSet;
use crate::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBLinearMap {
    pub gain: metamodelica::Array<metamodelica::Real>,
    pub offset: metamodelica::Array<metamodelica::Real>,
}

impl metamodelica::gc::MMTrace for SBLinearMap {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.gain, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.offset, __mmv)?;
        Ok(())
    }
}
impl Default for SBLinearMap {
    fn default() -> Self {
        Self {
            gain: Default::default(),
            offset: Default::default(),
        }
    }
}

pub type LINEAR_MAP = SBLinearMap;

pub fn new(
    mut gain: metamodelica::Array<metamodelica::Real>,
    mut offset: metamodelica::Array<metamodelica::Real>,
) -> Result<metamodelica::Ref<SBLinearMap>> {
    let mut map: metamodelica::Ref<SBLinearMap>;
    if Array::any(gain.clone(), &fnptr!(Util::realNegative, metamodelica::Real))? {
        map = newEmpty();
    } else if metamodelica::arrayLength(gain.clone()) == metamodelica::arrayLength(offset.clone()) {
        map = metamodelica::Ref::new(SBLinearMap {
            gain: metamodelica::arrayFromVec(gain.clone().borrow().clone()),
            offset: metamodelica::arrayFromVec(offset.clone().borrow().clone()),
        });
    } else {
        map = newEmpty();
    }
    Ok(map)
}

pub(crate) fn newEmpty() -> metamodelica::Ref<SBLinearMap> {
    let mut map: metamodelica::Ref<SBLinearMap> = metamodelica::Ref::new(SBLinearMap {
        gain: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        offset: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
    });
    map
}

pub(crate) fn newIdentity(mut dim: i32) -> metamodelica::Ref<SBLinearMap> {
    let mut map: metamodelica::Ref<SBLinearMap>;
    map = metamodelica::Ref::new(SBLinearMap {
        gain: arrayCreate(dim, metamodelica::OrderedFloat(1.0_f64)),
        offset: arrayCreate(dim, metamodelica::OrderedFloat(0.0_f64)),
    });
    map
}

pub(crate) fn copy(mut map: &metamodelica::Ref<SBLinearMap>) -> metamodelica::Ref<SBLinearMap> {
    let mut outMap: metamodelica::Ref<SBLinearMap>;
    outMap = metamodelica::Ref::new(SBLinearMap {
        gain: metamodelica::arrayFromVec(map.gain.clone().borrow().clone()),
        offset: metamodelica::arrayFromVec(map.offset.clone().borrow().clone()),
    });
    outMap
}

pub(crate) fn ndim(mut map: &metamodelica::Ref<SBLinearMap>) -> i32 {
    let mut ndim: i32 = metamodelica::arrayLength(map.gain.clone());
    ndim
}

pub(crate) fn isDim(mut map: &metamodelica::Ref<SBLinearMap>, mut dim: i32) -> bool {
    let mut res: bool = metamodelica::arrayLength(map.gain.clone()) == dim;
    res
}

pub(crate) fn gain(mut map: &metamodelica::Ref<SBLinearMap>) -> metamodelica::Array<metamodelica::Real> {
    let mut gain: metamodelica::Array<metamodelica::Real> = map.gain.clone();
    gain
}

pub(crate) fn offset(mut map: &metamodelica::Ref<SBLinearMap>) -> metamodelica::Array<metamodelica::Real> {
    let mut offset: metamodelica::Array<metamodelica::Real> = map.offset.clone();
    offset
}

pub(crate) fn isEmpty(mut map: &metamodelica::Ref<SBLinearMap>) -> bool {
    let mut empty: bool = map.gain.clone().borrow().is_empty();
    empty
}

pub(crate) fn isIdentity(mut map: &metamodelica::Ref<SBLinearMap>) -> Result<bool> {
    let mut isIdentity: bool;
    isIdentity = Array::all(
        map.gain.clone(),
        &({
            let __pe_b0 = metamodelica::OrderedFloat(1.0_f64);
            move |__pe_a1| Ok(realEq(__pe_b0.clone(), __pe_a1))
        }),
    )? && Array::all(
        map.offset.clone(),
        &({
            let __pe_b0 = metamodelica::OrderedFloat(0.0_f64);
            move |__pe_a1| Ok(realEq(__pe_b0.clone(), __pe_a1))
        }),
    )?;
    Ok(isIdentity)
}

pub(crate) fn isEqual(
    mut map1: &metamodelica::Ref<SBLinearMap>,
    mut map2: &metamodelica::Ref<SBLinearMap>,
) -> Result<bool> {
    let mut equal: bool;
    equal = Array::isEqualOnTrue(
        map1.gain.clone(),
        map2.gain.clone(),
        &fnptr!(realEq, metamodelica::Real, metamodelica::Real),
    )? && Array::isEqualOnTrue(
        map1.offset.clone(),
        map2.offset.clone(),
        &fnptr!(realEq, metamodelica::Real, metamodelica::Real),
    )?;
    Ok(equal)
}

pub(crate) fn compose(
    mut map1: &metamodelica::Ref<SBLinearMap>,
    mut map2: &metamodelica::Ref<SBLinearMap>,
) -> metamodelica::Ref<SBLinearMap> {
    let mut map: metamodelica::Ref<SBLinearMap>;
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut offset: metamodelica::Array<metamodelica::Real>;
    let mut len1: i32 = ndim(map1);
    let mut len2: i32 = ndim(map2);
    let mut g1: metamodelica::Real;
    let mut g2: metamodelica::Real;
    let mut o1: metamodelica::Real;
    let mut o2: metamodelica::Real;
    if len1 == len2 {
        gain = metamodelica::arrayCreate(len1, metamodelica::OrderedFloat(0.0_f64));
        offset = metamodelica::arrayCreate(len1, metamodelica::OrderedFloat(0.0_f64));
        for mut i in 1..=len1 {
            g1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(map1.gain.clone(), i);
            g2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(map2.gain.clone(), i);
            o1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(map1.offset.clone(), i);
            o2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(map2.offset.clone(), i);
            unsafe { metamodelica::Dangerous::arrayInitSlot(gain.clone(), i, g1 * g2) };
            unsafe { metamodelica::Dangerous::arrayInitSlot(offset.clone(), i, o2 * g1 + o1) };
        }
        map = metamodelica::Ref::new(SBLinearMap {
            gain: gain.clone(),
            offset: offset.clone(),
        });
    } else {
        map = newEmpty();
    }
    map
}

pub(crate) fn inverse(mut map: &metamodelica::Ref<SBLinearMap>) -> Result<metamodelica::Ref<SBLinearMap>> {
    let mut inv: metamodelica::Ref<SBLinearMap>;
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut offset: metamodelica::Array<metamodelica::Real>;
    let mut len: i32 = ndim(map);
    let mut g: metamodelica::Real;
    let mut o: metamodelica::Real;
    gain = metamodelica::arrayCreate(len, metamodelica::OrderedFloat(0.0_f64));
    offset = metamodelica::arrayCreate(len, metamodelica::OrderedFloat(0.0_f64));
    for mut i in 1..=len {
        g = metamodelica::Dangerous::arrayGetNoBoundsChecking(map.gain.clone(), i);
        o = metamodelica::Dangerous::arrayGetNoBoundsChecking(map.offset.clone(), i);
        if g != metamodelica::OrderedFloat((0) as f64) {
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    gain.clone(),
                    i,
                    metamodelica::real_div_checked(metamodelica::OrderedFloat(1.0_f64), g)?,
                )
            };
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(offset.clone(), i, -(metamodelica::real_div_checked(o, g)?))
            };
        } else {
            unsafe { metamodelica::Dangerous::arrayInitSlot(gain.clone(), i, intReal(System::intMaxLit())) };
            unsafe { metamodelica::Dangerous::arrayInitSlot(offset.clone(), i, intReal(System::intMaxLit())) };
        }
    }
    inv = metamodelica::Ref::new(SBLinearMap {
        gain: gain.clone(),
        offset: offset.clone(),
    });
    Ok(inv)
}

pub(crate) fn apply(
    mut domain: metamodelica::Ref<SBSet::SBSet>,
    mut map: &metamodelica::Ref<SBLinearMap>,
) -> Result<metamodelica::Ref<SBSet::SBSet>> {
    let mut target: metamodelica::Ref<SBSet::SBSet> = SBSet::copy(domain.clone());
    if !(isIdentity(map)?) {
        assign_field!(
            target.asets = UnorderedSet::selfMap(
                target.asets.clone(),
                &({
                    let __pe_b1 = map.clone();
                    move |__pe_a0| applyAtomicSet(__pe_a0, &__pe_b1)
                })
            )?
        );
    }
    Ok(target)
}

pub(crate) fn applyAtomicSet(
    mut atomic: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
    mut map: &metamodelica::Ref<SBLinearMap>,
) -> Result<metamodelica::Ref<SBAtomicSet::SBAtomicSet>> {
    let mut atomic: metamodelica::Ref<SBAtomicSet::SBAtomicSet> = atomic;
    assign_field!(atomic.aset = applyMultiInterval(atomic.aset.clone(), map)?);
    Ok(atomic)
}

pub(crate) fn applyMultiInterval(
    mut multiInt: metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut map: &metamodelica::Ref<SBLinearMap>,
) -> Result<metamodelica::Ref<SBMultiInterval::SBMultiInterval>> {
    let mut multiInt: metamodelica::Ref<SBMultiInterval::SBMultiInterval> = multiInt;
    for mut i in 1..=multiInt.ndim.clone() {
        {
            let __cell0 = applyInterval(
                ({
                    let __elt = (*metamodelica::index_checked(&multiInt.intervals.borrow(), i)?).clone();
                    __elt
                }),
                ({
                    let __elt = (*metamodelica::index_checked(&map.gain.borrow(), i)?).clone();
                    __elt
                }),
                ({
                    let __elt = (*metamodelica::index_checked(&map.offset.borrow(), i)?).clone();
                    __elt
                }),
            );
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut multiInt.intervals.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    Ok(multiInt)
}

pub(crate) fn applyInterval(
    mut interval: metamodelica::Ref<SBInterval::SBInterval>,
    mut gain: metamodelica::Real,
    mut offset: metamodelica::Real,
) -> metamodelica::Ref<SBInterval::SBInterval> {
    let mut interval: metamodelica::Ref<SBInterval::SBInterval> = interval;
    assign_field!(
        interval.lo = ((intReal(interval.lo.clone()) * gain + offset).0.floor() as i32),
        interval.step = ((intReal(interval.step.clone()) * gain).0.floor() as i32),
        interval.hi = ((intReal(interval.hi.clone()) * gain + offset).0.floor() as i32)
    );
    interval
}

pub(crate) fn toString(mut map: &metamodelica::Ref<SBLinearMap>) -> ArcStr {
    let mut r#str: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    for mut i in ({
        let __s = metamodelica::arrayLength(map.gain.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        strl = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ArcStr::from(::std::format!(
                    "{}",
                    metamodelica::Dangerous::arrayGetNoBoundsChecking(map.gain.clone(), i)
                )));
                __mm_s.push_str(&*literal!(" * x + "));
                __mm_s.push_str(&*ArcStr::from(::std::format!(
                    "{}",
                    metamodelica::Dangerous::arrayGetNoBoundsChecking(map.offset.clone(), i)
                )));
                ArcStr::from(__mm_s)
            },
            strl,
        );
    }
    r#str = stringDelimitList(strl, literal!("\n"));
    r#str
}
