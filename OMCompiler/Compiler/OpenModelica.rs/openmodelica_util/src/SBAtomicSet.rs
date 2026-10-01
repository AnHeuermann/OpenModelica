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
use crate::SBMultiInterval;
use crate::UnorderedSet;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBAtomicSet {
    pub aset: metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    pub ndim: i32,
}

impl metamodelica::gc::MMTrace for SBAtomicSet {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.aset, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ndim, __mmv)?;
        Ok(())
    }
}
impl Default for SBAtomicSet {
    fn default() -> Self {
        Self {
            aset: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type ATOMIC_SET = SBAtomicSet;

pub fn new(mut mi: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>) -> metamodelica::Ref<SBAtomicSet> {
    let mut set: metamodelica::Ref<SBAtomicSet>;
    set = metamodelica::Ref::new(SBAtomicSet {
        aset: SBMultiInterval::copy(mi),
        ndim: mi.ndim.clone(),
    });
    set
}

pub(crate) fn newEmpty() -> metamodelica::Ref<SBAtomicSet> {
    let mut set: metamodelica::Ref<SBAtomicSet>;
    set = metamodelica::Ref::new(SBAtomicSet {
        aset: SBMultiInterval::newEmpty(),
        ndim: 0,
    });
    set
}

pub(crate) fn copy(mut set: &metamodelica::Ref<SBAtomicSet>) -> metamodelica::Ref<SBAtomicSet> {
    let mut outSet: metamodelica::Ref<SBAtomicSet>;
    outSet = metamodelica::Ref::new(SBAtomicSet {
        aset: SBMultiInterval::copy(&set.aset),
        ndim: set.ndim.clone(),
    });
    outSet
}

pub(crate) fn ndim(mut set: &metamodelica::Ref<SBAtomicSet>) -> i32 {
    let mut ndim: i32 = set.ndim.clone();
    ndim
}

pub(crate) fn isEmpty(mut set: &metamodelica::Ref<SBAtomicSet>) -> bool {
    let mut empty: bool = SBMultiInterval::isEmpty(&set.aset);
    empty
}

pub(crate) fn contains(mut vals: metamodelica::Array<i32>, mut set: &metamodelica::Ref<SBAtomicSet>) -> Result<bool> {
    let mut res: bool = SBMultiInterval::contains(vals.clone(), &set.aset)?;
    Ok(res)
}

pub(crate) fn intersection(
    mut set1: &metamodelica::Ref<SBAtomicSet>,
    mut set2: &metamodelica::Ref<SBAtomicSet>,
) -> Result<metamodelica::Ref<SBAtomicSet>> {
    let mut res: metamodelica::Ref<SBAtomicSet>;
    res = new(&(SBMultiInterval::intersection(&set1.aset, &set2.aset)?));
    Ok(res)
}

pub(crate) fn complement(
    mut set1: &metamodelica::Ref<SBAtomicSet>,
    mut set2: &metamodelica::Ref<SBAtomicSet>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet>>>> {
    let mut res: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet>>>;
    let mut diff: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBMultiInterval::SBMultiInterval>>>;
    diff = SBMultiInterval::complement(set1.aset.clone(), &set2.aset)?;
    res = UnorderedSet::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<SBAtomicSet>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(hash(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SBAtomicSet>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SBAtomicSet>, __a1: metamodelica::Ref<SBAtomicSet>| isEqual(&__a0, &__a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<SBAtomicSet>, metamodelica::Ref<SBAtomicSet>) -> Result<bool>
                    + 'static,
            >),
        UnorderedSet::bucketCount(diff.clone()),
    );
    if !(UnorderedSet::isEmpty(diff.clone())) {
        let __range0 = UnorderedSet::toArray(diff).borrow().iter().cloned().collect::<Vec<_>>();
        for mut s in __range0 {
            UnorderedSet::add(new(&s), res.clone())?;
        }
    }
    Ok(res)
}

pub(crate) fn crossProd(
    mut set1: &metamodelica::Ref<SBAtomicSet>,
    mut set2: &metamodelica::Ref<SBAtomicSet>,
) -> Result<metamodelica::Ref<SBAtomicSet>> {
    let mut res: metamodelica::Ref<SBAtomicSet>;
    res = new(&(SBMultiInterval::crossProd(&set1.aset, &set2.aset)?));
    Ok(res)
}

pub(crate) fn cardinality(mut set: &metamodelica::Ref<SBAtomicSet>, mut card: i32) -> Result<i32> {
    let mut card: i32 = card;
    card = card + SBMultiInterval::cardinality(&set.aset)?;
    Ok(card)
}

pub fn aset(mut set: &metamodelica::Ref<SBAtomicSet>) -> metamodelica::Ref<SBMultiInterval::SBMultiInterval> {
    let mut res: metamodelica::Ref<SBMultiInterval::SBMultiInterval> = set.aset.clone();
    res
}

pub(crate) fn minElem(mut set: &metamodelica::Ref<SBAtomicSet>) -> Result<metamodelica::Array<i32>> {
    let mut res: metamodelica::Array<i32> = SBMultiInterval::minElem(&set.aset)?;
    Ok(res)
}

pub(crate) fn replace(
    mut i: metamodelica::Ref<SBInterval::SBInterval>,
    mut dim: i32,
    mut set: &metamodelica::Ref<SBAtomicSet>,
) -> Result<metamodelica::Ref<SBAtomicSet>> {
    let mut res: metamodelica::Ref<SBAtomicSet>;
    res = new(&(SBMultiInterval::replace(i, dim, &set.aset)?));
    Ok(res)
}

pub(crate) fn isEqual(
    mut set1: &metamodelica::Ref<SBAtomicSet>,
    mut set2: &metamodelica::Ref<SBAtomicSet>,
) -> Result<bool> {
    let mut equal: bool = SBMultiInterval::isEqual(&set1.aset, &set2.aset)?;
    Ok(equal)
}

pub(crate) fn hash(mut set1: &metamodelica::Ref<SBAtomicSet>) -> i32 {
    let mut hash: i32 = SBMultiInterval::hash(&set1.aset);
    hash
}

pub(crate) fn toString(mut set: &metamodelica::Ref<SBAtomicSet>) -> ArcStr {
    let mut r#str: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*SBMultiInterval::toString(&set.aset));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    r#str
}
