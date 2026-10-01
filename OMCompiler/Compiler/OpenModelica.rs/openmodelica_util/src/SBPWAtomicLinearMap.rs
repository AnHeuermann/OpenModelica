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
use crate::SBLinearMap;
use crate::SBMultiInterval;
use crate::System;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBPWAtomicLinearMap {
    pub dom: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
    pub lmap: metamodelica::Ref<SBLinearMap::SBLinearMap>,
}

impl metamodelica::gc::MMTrace for SBPWAtomicLinearMap {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.dom, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lmap, __mmv)?;
        Ok(())
    }
}
impl Default for SBPWAtomicLinearMap {
    fn default() -> Self {
        Self {
            dom: Default::default(),
            lmap: Default::default(),
        }
    }
}

pub type PW_ATOMIC_LINEAR_MAP = SBPWAtomicLinearMap;

pub(crate) fn new(
    mut dom: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
    mut lmap: &metamodelica::Ref<SBLinearMap::SBLinearMap>,
) -> metamodelica::Ref<SBPWAtomicLinearMap> {
    let mut map: metamodelica::Ref<SBPWAtomicLinearMap>;
    let mut compatible: bool = true;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut g: metamodelica::Array<metamodelica::Real>;
    let mut o: metamodelica::Array<metamodelica::Real>;
    let mut i: metamodelica::Ref<SBInterval::SBInterval>;
    let mut gain: metamodelica::Real;
    let mut offset: metamodelica::Real;
    let mut lo: metamodelica::Real;
    let mut step: metamodelica::Real;
    let mut hi: metamodelica::Real;
    if SBAtomicSet::ndim(dom) != SBLinearMap::ndim(lmap) {
        map = newEmpty();
        return map;
    }
    ints = SBMultiInterval::intervals(&(SBAtomicSet::aset(dom)));
    g = SBLinearMap::gain(lmap);
    o = SBLinearMap::offset(lmap);
    for mut j in 1..=metamodelica::arrayLength(ints.clone()) {
        i = metamodelica::Dangerous::arrayGetNoBoundsChecking(ints.clone(), j);
        gain = metamodelica::Dangerous::arrayGetNoBoundsChecking(g.clone(), j);
        offset = metamodelica::Dangerous::arrayGetNoBoundsChecking(g.clone(), j);
        if gain < intReal(System::intMaxLit()) {
            lo = metamodelica::OrderedFloat((SBInterval::lowerBound(&i)) as f64) * gain + offset;
            step = metamodelica::OrderedFloat((SBInterval::stepValue(&i)) as f64) * gain;
            hi = metamodelica::OrderedFloat((SBInterval::upperBound(&i)) as f64) * gain + offset;
            if lo != metamodelica::OrderedFloat(((lo).0.floor() as i32) as f64) && SBInterval::lowerBound(&i) > 0 {
                compatible = false;
                break;
            }
            if step != metamodelica::OrderedFloat(((step).0.floor() as i32) as f64) && SBInterval::stepValue(&i) > 0 {
                compatible = false;
                break;
            }
            if hi != metamodelica::OrderedFloat(((hi).0.floor() as i32) as f64) && SBInterval::upperBound(&i) > 0 {
                compatible = false;
                break;
            }
        }
    }
    if compatible {
        map = metamodelica::Ref::new(SBPWAtomicLinearMap {
            dom: SBAtomicSet::copy(dom),
            lmap: SBLinearMap::copy(lmap),
        });
    } else {
        map = newEmpty();
    }
    map
}

pub(crate) fn newEmpty() -> metamodelica::Ref<SBPWAtomicLinearMap> {
    let mut map: metamodelica::Ref<SBPWAtomicLinearMap>;
    map = metamodelica::Ref::new(SBPWAtomicLinearMap {
        dom: SBAtomicSet::newEmpty(),
        lmap: SBLinearMap::newEmpty(),
    });
    map
}

pub(crate) fn dom(mut map: &metamodelica::Ref<SBPWAtomicLinearMap>) -> metamodelica::Ref<SBAtomicSet::SBAtomicSet> {
    let mut dom: metamodelica::Ref<SBAtomicSet::SBAtomicSet> = map.dom.clone();
    dom
}

pub(crate) fn lmap(mut map: &metamodelica::Ref<SBPWAtomicLinearMap>) -> metamodelica::Ref<SBLinearMap::SBLinearMap> {
    let mut lmap: metamodelica::Ref<SBLinearMap::SBLinearMap> = map.lmap.clone();
    lmap
}

pub(crate) fn isEmpty(mut map: &metamodelica::Ref<SBPWAtomicLinearMap>) -> bool {
    let mut empty: bool;
    empty = SBAtomicSet::isEmpty(&map.dom) && SBLinearMap::isEmpty(&map.lmap);
    empty
}

pub(crate) fn image(
    mut map: &metamodelica::Ref<SBPWAtomicLinearMap>,
    mut set: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
) -> Result<metamodelica::Ref<SBAtomicSet::SBAtomicSet>> {
    fn crop_inf(mut v: metamodelica::Real) -> i32 {
        let mut i: i32;
        i = if (v >= intReal(System::intMaxLit())) {
            System::intMaxLit()
        } else {
            ((v).0.floor() as i32)
        };
        i
    }

    let mut outSet: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut inters: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut res: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut gains: metamodelica::Array<metamodelica::Real>;
    let mut offsets: metamodelica::Array<metamodelica::Real>;
    let mut set_int: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut int: metamodelica::Ref<SBInterval::SBInterval>;
    let mut gain: metamodelica::Real;
    let mut offset: metamodelica::Real;
    let mut new_lo: i32;
    let mut new_step: i32;
    let mut new_hi: i32;
    let mut tmp_lo: metamodelica::Real;
    let mut tmp_step: metamodelica::Real;
    let mut tmp_hi: metamodelica::Real;
    if SBAtomicSet::isEmpty(&map.dom) {
        outSet = SBAtomicSet::newEmpty();
        return Ok(outSet);
    }
    set_int = SBAtomicSet::intersection(set, &map.dom)?;
    inters = SBMultiInterval::intervals(&(SBAtomicSet::aset(&set_int)));
    if inters.clone().borrow().is_empty() {
        outSet = SBAtomicSet::newEmpty();
        return Ok(outSet);
    }
    gains = SBLinearMap::gain(&map.lmap);
    offsets = SBLinearMap::offset(&map.lmap);
    res = metamodelica::arrayCreate(
        metamodelica::arrayLength(inters.clone()),
        ({
            let __elt = (*metamodelica::index_checked(&inters.borrow(), 1)?).clone();
            __elt
        }),
    );
    for mut i in 1..=metamodelica::arrayLength(inters.clone()) {
        int = metamodelica::Dangerous::arrayGetNoBoundsChecking(inters.clone(), i);
        gain = ({
            let __elt = (*metamodelica::index_checked(&gains.borrow(), i)?).clone();
            __elt
        });
        offset = ({
            let __elt = (*metamodelica::index_checked(&offsets.borrow(), i)?).clone();
            __elt
        });
        tmp_lo = metamodelica::OrderedFloat((SBInterval::lowerBound(&int)) as f64) * gain + offset;
        tmp_step = metamodelica::OrderedFloat((SBInterval::stepValue(&int)) as f64) * gain;
        tmp_hi = metamodelica::OrderedFloat((SBInterval::upperBound(&int)) as f64) * gain + offset;
        if gain < intReal(System::intMaxLit()) {
            new_lo = crop_inf(tmp_lo);
            new_step = crop_inf(tmp_step);
            new_hi = crop_inf(tmp_hi);
        } else {
            new_lo = 1;
            new_step = 1;
            new_hi = System::intMaxLit();
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(res.clone(), i, SBInterval::new(new_lo, new_step, new_hi)) };
    }
    outSet = SBAtomicSet::new(&(SBMultiInterval::fromArray(res.clone())?));
    Ok(outSet)
}

pub(crate) fn preImage(
    mut map: &metamodelica::Ref<SBPWAtomicLinearMap>,
    mut set: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
) -> Result<metamodelica::Ref<SBAtomicSet::SBAtomicSet>> {
    let mut outSet: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut full_im: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut actual_im: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut aux: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut inv: metamodelica::Ref<SBPWAtomicLinearMap>;
    full_im = image(map, &map.dom)?;
    actual_im = SBAtomicSet::intersection(&full_im, set)?;
    inv = new(&actual_im, &(SBLinearMap::inverse(&map.lmap)?));
    aux = image(&inv, &actual_im)?;
    outSet = SBAtomicSet::intersection(&map.dom, &aux)?;
    Ok(outSet)
}

pub(crate) fn isEqual(
    mut map1: &metamodelica::Ref<SBPWAtomicLinearMap>,
    mut map2: &metamodelica::Ref<SBPWAtomicLinearMap>,
) -> Result<bool> {
    let mut equal: bool;
    equal = SBAtomicSet::isEqual(&map1.dom, &map2.dom)? && SBLinearMap::isEqual(&map1.lmap, &map2.lmap)?;
    Ok(equal)
}

pub(crate) fn toString(mut map: &metamodelica::Ref<SBPWAtomicLinearMap>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut g: metamodelica::Array<metamodelica::Real>;
    let mut o: metamodelica::Array<metamodelica::Real>;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    g = SBLinearMap::gain(&map.lmap);
    o = SBLinearMap::offset(&map.lmap);
    ints = SBMultiInterval::intervals(&(SBAtomicSet::aset(&map.dom)));
    for mut i in ({
        let __s = metamodelica::arrayLength(ints.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*SBInterval::toString(
                &({
                    let __elt = (*metamodelica::index_checked(&ints.borrow(), i)?).clone();
                    __elt
                }),
            ));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*ArcStr::from(::std::format!(
                "{}",
                ({
                    let __elt = (*metamodelica::index_checked(&g.borrow(), i)?).clone();
                    __elt
                })
            )));
            __mm_s.push_str(&*literal!(" * x + "));
            __mm_s.push_str(&*ArcStr::from(::std::format!(
                "{}",
                ({
                    let __elt = (*metamodelica::index_checked(&o.borrow(), i)?).clone();
                    __elt
                })
            )));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        strl = metamodelica::cons(r#str, strl);
    }
    r#str = stringDelimitList(strl, literal!("x"));
    Ok(r#str)
}
