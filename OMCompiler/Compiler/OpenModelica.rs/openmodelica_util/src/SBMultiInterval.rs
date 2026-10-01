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

use crate::SBInterval;
use crate::UnorderedSet;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBMultiInterval {
    pub intervals: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>,
    pub ndim: i32,
}

impl metamodelica::gc::MMTrace for SBMultiInterval {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.intervals, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ndim, __mmv)?;
        Ok(())
    }
}
impl Default for SBMultiInterval {
    fn default() -> Self {
        Self {
            intervals: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type MULTI_INTERVAL = SBMultiInterval;

pub fn newEmpty() -> metamodelica::Ref<SBMultiInterval> {
    let mut mi: metamodelica::Ref<SBMultiInterval>;
    mi = metamodelica::Ref::new(SBMultiInterval {
        intervals: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        ndim: 0,
    });
    mi
}

pub(crate) fn copy(mut mi: &metamodelica::Ref<SBMultiInterval>) -> metamodelica::Ref<SBMultiInterval> {
    let mut outMI: metamodelica::Ref<SBMultiInterval>;
    outMI = metamodelica::Ref::new(SBMultiInterval {
        intervals: metamodelica::arrayFromVec(mi.intervals.clone().borrow().clone()),
        ndim: mi.ndim.clone(),
    });
    outMI
}

pub(crate) fn fromList(
    mut ints: metamodelica::List<metamodelica::Ref<SBInterval::SBInterval>>,
) -> Result<metamodelica::Ref<SBMultiInterval>> {
    let mut outMI: metamodelica::Ref<SBMultiInterval>;
    if List::any(
        &ints,
        &move |__a0: metamodelica::Ref<SBInterval::SBInterval>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(SBInterval::isEmpty(&__a0))
        },
    )? {
        outMI = newEmpty();
    } else {
        outMI = metamodelica::Ref::new(SBMultiInterval {
            intervals: metamodelica::arrayFromVec(ints.clone().into_iter().cloned().collect()),
            ndim: ((ints).len() as i32),
        });
    }
    Ok(outMI)
}

pub fn fromArray(
    mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>,
) -> Result<metamodelica::Ref<SBMultiInterval>> {
    let mut outMI: metamodelica::Ref<SBMultiInterval>;
    if Array::any(
        ints.clone(),
        &move |__a0: metamodelica::Ref<SBInterval::SBInterval>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(SBInterval::isEmpty(&__a0))
        },
    )? {
        outMI = newEmpty();
    } else {
        outMI = metamodelica::Ref::new(SBMultiInterval {
            intervals: metamodelica::arrayFromVec(ints.clone().borrow().clone()),
            ndim: metamodelica::arrayLength(ints.clone()),
        });
    }
    Ok(outMI)
}

pub fn isEmpty(mut mi: &metamodelica::Ref<SBMultiInterval>) -> bool {
    let mut empty: bool;
    empty = mi.intervals.clone().borrow().is_empty();
    empty
}

pub(crate) fn contains(
    mut vals: metamodelica::Array<i32>,
    mut mi: &metamodelica::Ref<SBMultiInterval>,
) -> Result<bool> {
    let mut res: bool;
    if metamodelica::arrayLength(vals.clone()) != mi.ndim.clone() {
        res = false;
    } else {
        res = Array::isEqualOnTrue(vals.clone(), mi.intervals.clone(), &move |__a0: i32,
                                                                              __a1: metamodelica::Ref<
            SBInterval::SBInterval,
        >|
              -> metamodelica::Result<
            _,
        > {
            ::std::result::Result::Ok(SBInterval::contains(__a0, &__a1))
        })?;
    }
    Ok(res)
}

pub fn intersection(
    mut mi1: &metamodelica::Ref<SBMultiInterval>,
    mut mi2: &metamodelica::Ref<SBMultiInterval>,
) -> Result<metamodelica::Ref<SBMultiInterval>> {
    let mut outMI: metamodelica::Ref<SBMultiInterval>;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut ires: metamodelica::Ref<SBInterval::SBInterval>;
    if mi1.ndim.clone() != mi2.ndim.clone() || isEmpty(mi1) {
        outMI = newEmpty();
        return Ok(outMI);
    }
    ints = metamodelica::arrayCreate(mi1.ndim.clone(), metamodelica::arrayGet(mi1.intervals.clone(), 1)?);
    for mut i in 1..=metamodelica::arrayLength(ints.clone()) {
        ires = SBInterval::intersection(
            &(metamodelica::arrayGet(mi1.intervals.clone(), i)?),
            &(metamodelica::arrayGet(mi2.intervals.clone(), i)?),
        );
        if SBInterval::isEmpty(&ires) {
            outMI = newEmpty();
            return Ok(outMI);
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(ints.clone(), i, ires) };
    }
    outMI = fromArray(ints.clone())?;
    Ok(outMI)
}

pub(crate) fn complement(
    mut mi1: metamodelica::Ref<SBMultiInterval>,
    mut mi2: &metamodelica::Ref<SBMultiInterval>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBMultiInterval>>>> {
    fn add_interval(
        mut i: metamodelica::Ref<SBInterval::SBInterval>,
        mut count: i32,
        mut size: i32,
        mut ints1: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>,
        mut ints2: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>,
        mut res: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBMultiInterval>>>,
    ) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBMultiInterval>>>> {
        let mut res: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBMultiInterval>>> = res;
        let mut dummyi: metamodelica::Ref<SBInterval::SBInterval>;
        let mut resi: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
        if !(SBInterval::isEmpty(&i)) {
            resi = metamodelica::arrayCreateDefault(size);
            Array::copyN(ints1.clone(), resi.clone(), count, 0, 0)?;
            {
                let __cell0 = i;
                let __idx0 = count + 1;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(resi.clone().clone(), __idx0, __cell0) }?;
            }
            Array::copyN(
                ints2.clone(),
                resi.clone(),
                metamodelica::arrayLength(ints2.clone()) - count - 1,
                count + 1,
                count + 1,
            )?;
            UnorderedSet::add(fromArray(resi.clone())?, res.clone())?;
        }
        Ok(res)
    }

    let mut res: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBMultiInterval>>>;
    let mut tmp_mi: metamodelica::Ref<SBMultiInterval>;
    let mut dummys: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBInterval::SBInterval>>>;
    let mut diffs: metamodelica::Array<
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBInterval::SBInterval>>>,
    >;
    let mut count: i32;
    let mut mi1_size: i32;
    let mut resi: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    res = UnorderedSet::new(
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SBMultiInterval>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(hash(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SBMultiInterval>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SBMultiInterval>, __a1: metamodelica::Ref<SBMultiInterval>| {
                isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SBMultiInterval>,
                        metamodelica::Ref<SBMultiInterval>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    if isEmpty(&mi1) || mi1.ndim.clone() != mi2.ndim.clone() {
        return Ok(res);
    }
    tmp_mi = intersection(&mi1, mi2)?;
    if isEmpty(&tmp_mi) {
        UnorderedSet::add(mi1, res.clone())?;
        return Ok(res);
    }
    if isEqual(&mi1, &tmp_mi)? {
        return Ok(res);
    }
    mi1_size = metamodelica::arrayLength(mi1.intervals.clone());
    diffs = metamodelica::arrayCreateDefault(mi1_size);
    for mut i in 1..=mi1_size {
        {
            let __cell0 = SBInterval::complement(
                metamodelica::Dangerous::arrayGetNoBoundsChecking(mi1.intervals.clone(), i),
                &(metamodelica::arrayGet(tmp_mi.intervals.clone(), i)?),
            )?;
            let __idx0 = i;
            let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(diffs.clone().clone(), __idx0, __cell0) }?;
        }
    }
    count = 0;
    let __range1 = diffs.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut vdiff in __range1 {
        UnorderedSet::fold(
            vdiff,
            &({
                let __pe_b1 = count;
                let __pe_b2 = mi1_size;
                let __pe_b3 = tmp_mi.intervals.clone();
                let __pe_b4 = mi1.intervals.clone();
                move |__pe_a0, __pe_a5| {
                    add_interval(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                        __pe_a5,
                    )
                }
            }),
            res.clone(),
        )?;
        count = count + 1;
    }
    Ok(res)
}

pub(crate) fn crossProd(
    mut mi1: &metamodelica::Ref<SBMultiInterval>,
    mut mi2: &metamodelica::Ref<SBMultiInterval>,
) -> Result<metamodelica::Ref<SBMultiInterval>> {
    let mut res: metamodelica::Ref<SBMultiInterval>;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    ints = Array::join(mi1.intervals.clone(), mi2.intervals.clone())?;
    res = metamodelica::Ref::new(SBMultiInterval {
        intervals: ints.clone(),
        ndim: metamodelica::arrayLength(ints.clone()),
    });
    Ok(res)
}

pub(crate) fn cardinality(mut mi: &metamodelica::Ref<SBMultiInterval>) -> Result<i32> {
    let mut card: i32 = 0;
    for mut i in 1..=mi.ndim.clone() {
        card = card
            + SBInterval::cardinality(
                &({
                    let __elt = (*metamodelica::index_checked(&mi.intervals.borrow(), i)?).clone();
                    __elt
                }),
            )?;
    }
    Ok(card)
}

pub fn intervals(
    mut mi: &metamodelica::Ref<SBMultiInterval>,
) -> metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>> {
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>> = mi.intervals.clone();
    ints
}

pub fn ndim(mut mi: &metamodelica::Ref<SBMultiInterval>) -> i32 {
    let mut ndim: i32 = metamodelica::arrayLength(mi.intervals.clone());
    ndim
}

pub fn minElem(mut mi: &metamodelica::Ref<SBMultiInterval>) -> Result<metamodelica::Array<i32>> {
    let mut res: metamodelica::Array<i32>;
    let __range0 = mi.intervals.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        if SBInterval::isEmpty(&i) {
            res = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            return Ok(res);
        }
    }
    res = Array::map(mi.intervals.clone(), &move |__a0: metamodelica::Ref<
        SBInterval::SBInterval,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(SBInterval::lowerBound(&__a0))
    })?;
    Ok(res)
}

pub(crate) fn replace(
    mut i: metamodelica::Ref<SBInterval::SBInterval>,
    mut dim: i32,
    mut mi: &metamodelica::Ref<SBMultiInterval>,
) -> Result<metamodelica::Ref<SBMultiInterval>> {
    let mut res: metamodelica::Ref<SBMultiInterval>;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    ints = metamodelica::arrayFromVec(mi.intervals.clone().borrow().clone());
    {
        let __cell0 = i;
        let __idx0 = dim;
        *metamodelica::index_mut_checked(&mut ints.clone().borrow_mut(), __idx0)? = __cell0;
    }
    res = fromArray(ints.clone())?;
    Ok(res)
}

pub(crate) fn isEqual(
    mut mi1: &metamodelica::Ref<SBMultiInterval>,
    mut mi2: &metamodelica::Ref<SBMultiInterval>,
) -> Result<bool> {
    let mut equal: bool;
    equal = Array::isEqualOnTrue(
        mi1.intervals.clone(),
        mi2.intervals.clone(),
        &move |__a0: metamodelica::Ref<SBInterval::SBInterval>,
               __a1: metamodelica::Ref<SBInterval::SBInterval>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(SBInterval::isEqual(&__a0, &__a1)) },
    )?;
    Ok(equal)
}

pub(crate) fn hash(mut mi: &metamodelica::Ref<SBMultiInterval>) -> i32 {
    let mut res: i32;
    res = metamodelica::arrayLength(mi.intervals.clone());
    res
}

pub fn size(mut mi: &metamodelica::Ref<SBMultiInterval>) -> i32 {
    let mut sz: i32 = 1;
    let __range0 = mi.intervals.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        sz = sz * SBInterval::size(&i);
    }
    sz
}

pub(crate) fn toString(mut mi: &metamodelica::Ref<SBMultiInterval>) -> ArcStr {
    let mut r#str: ArcStr;
    if isEmpty(mi) {
        r#str = literal!("emptyInterval");
    } else {
        r#str = stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut i in (mi.intervals.clone()).borrow().iter() {
                    let __x = SBInterval::toString(&(i.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!("x"),
        );
    }
    r#str
}
