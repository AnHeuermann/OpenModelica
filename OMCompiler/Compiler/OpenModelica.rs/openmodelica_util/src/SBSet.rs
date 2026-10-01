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
use crate::UnorderedSet;
use crate::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBSet {
    pub asets: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>,
    pub ndim: i32,
}

impl metamodelica::gc::MMTrace for SBSet {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.asets, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ndim, __mmv)?;
        Ok(())
    }
}
impl Default for SBSet {
    fn default() -> Self {
        Self {
            asets: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type SET = SBSet;

pub(crate) fn new(
    mut ss: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>,
) -> Result<metamodelica::Ref<SBSet>> {
    fn is_equal_dim(mut set1: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>, mut dim: i32) -> bool {
        let mut equal: bool = SBAtomicSet::ndim(set1) == dim;
        equal
    }

    let mut set: metamodelica::Ref<SBSet>;
    let mut dim: i32;
    if !(UnorderedSet::isEmpty(ss.clone())) {
        dim = SBAtomicSet::ndim(&(UnorderedSet::first(ss.clone())?));
        if dim != 0
            && UnorderedSet::all(
                ss.clone(),
                &({
                    let __pe_b1 = dim;
                    move |__pe_a0| Ok(is_equal_dim(&__pe_a0, __pe_b1.clone()))
                }),
            )?
        {
            set = metamodelica::Ref::new(SBSet {
                asets: UnorderedSet::copy(ss),
                ndim: dim,
            });
        } else {
            set = newEmpty();
        }
    } else {
        set = metamodelica::Ref::new(SBSet {
            asets: UnorderedSet::copy(ss),
            ndim: 0,
        });
    }
    Ok(set)
}

pub fn newEmpty() -> metamodelica::Ref<SBSet> {
    let mut set: metamodelica::Ref<SBSet>;
    set = metamodelica::Ref::new(SBSet {
        asets: UnorderedSet::new(
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(SBAtomicSet::hash(&__a0))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                      __a1: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| {
                    SBAtomicSet::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                            metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        ),
        ndim: 0,
    });
    set
}

pub(crate) fn copy(mut set: metamodelica::Ref<SBSet>) -> metamodelica::Ref<SBSet> {
    let mut set: metamodelica::Ref<SBSet> = set;
    assign_field!(set.asets = UnorderedSet::copy(set.asets.clone()));
    set
}

pub(crate) fn ndim(mut set: &metamodelica::Ref<SBSet>) -> i32 {
    let mut ndim: i32 = set.ndim.clone();
    ndim
}

pub fn isEmpty(mut set: &metamodelica::Ref<SBSet>) -> bool {
    let mut empty: bool = UnorderedSet::isEmpty(set.asets.clone());
    empty
}

pub(crate) fn isDim(mut set: &metamodelica::Ref<SBSet>, mut dim: i32) -> bool {
    let mut res: bool = set.ndim.clone() == dim;
    res
}

pub fn asets(
    mut set: &metamodelica::Ref<SBSet>,
) -> metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>> {
    let mut asets: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>> =
        set.asets.clone();
    asets
}

pub(crate) fn contains(mut vals: metamodelica::Array<i32>, mut set: &metamodelica::Ref<SBSet>) -> Result<bool> {
    let mut res: bool;
    res = UnorderedSet::all(
        set.asets.clone(),
        &({
            let __pe_b0 = vals.clone();
            move |__pe_a1| SBAtomicSet::contains(__pe_b0.clone(), &__pe_a1)
        }),
    )?;
    Ok(res)
}

pub fn addAtomicSet(
    mut aset: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
    mut set: metamodelica::Ref<SBSet>,
) -> Result<metamodelica::Ref<SBSet>> {
    let mut set: metamodelica::Ref<SBSet> = set;
    if SBAtomicSet::isEmpty(&aset) {
        return Ok(set);
    }
    if UnorderedSet::isEmpty(set.asets.clone()) {
        UnorderedSet::add(aset.clone(), set.asets.clone())?;
        assign_field!(set.ndim = SBAtomicSet::ndim(&aset));
    } else if SBAtomicSet::ndim(&aset) == set.ndim.clone() {
        UnorderedSet::add(aset, set.asets.clone())?;
    }
    Ok(set)
}

pub(crate) fn addAtomicSets(
    mut asets: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>,
    mut set: metamodelica::Ref<SBSet>,
) -> Result<metamodelica::Ref<SBSet>> {
    let mut set: metamodelica::Ref<SBSet> = set;
    set = UnorderedSet::fold(asets, &addAtomicSet, set)?;
    Ok(set)
}

pub fn intersection(
    mut set1: &metamodelica::Ref<SBSet>,
    mut set2: &metamodelica::Ref<SBSet>,
) -> Result<metamodelica::Ref<SBSet>> {
    let mut outSet: metamodelica::Ref<SBSet>;
    let mut int_set: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut res: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>;
    if UnorderedSet::isEmpty(set1.asets.clone()) || UnorderedSet::isEmpty(set2.asets.clone()) {
        outSet = newEmpty();
        return Ok(outSet);
    }
    res = UnorderedSet::new(
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(SBAtomicSet::hash(&__a0))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                  __a1: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| SBAtomicSet::isEqual(&__a0, &__a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                        metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    let __range0 = UnorderedSet::toArray(set1.asets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut as1 in __range0 {
        let __range1 = UnorderedSet::toArray(set2.asets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut as2 in __range1 {
            int_set = SBAtomicSet::intersection(&as1, &as2)?;
            if !(SBAtomicSet::isEmpty(&int_set)) {
                UnorderedSet::add(int_set, res.clone())?;
            }
        }
    }
    outSet = new(res)?;
    Ok(outSet)
}

pub fn complement(
    mut set1: &metamodelica::Ref<SBSet>,
    mut set2: &metamodelica::Ref<SBSet>,
) -> Result<metamodelica::Ref<SBSet>> {
    let mut outSet: metamodelica::Ref<SBSet>;
    let mut int_res: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>;
    let mut aux: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>;
    let mut comp_res: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>;
    let mut new_sets: metamodelica::Ref<SBSet>;
    outSet = newEmpty();
    let __arc1 = intersection(set1, set2)?;
    let SET { asets: __pa0, .. } = &*__arc1;
    int_res = metamodelica::Own::own(__pa0);
    if !(UnorderedSet::isEmpty(int_res.clone())) {
        let __range2 = UnorderedSet::toArray(set1.asets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut as1 in __range2 {
            aux = UnorderedSet::new(
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(SBAtomicSet::hash(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                          __a1: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| {
                        SBAtomicSet::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                                metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                13,
            );
            UnorderedSet::add(as1, aux.clone())?;
            let __range3 = UnorderedSet::toArray(int_res.clone())
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut as2 in __range3 {
                new_sets = newEmpty();
                let __range4 = UnorderedSet::toArray(aux).borrow().iter().cloned().collect::<Vec<_>>();
                for mut as3 in __range4 {
                    comp_res = SBAtomicSet::complement(&as3, &as2)?;
                    new_sets = addAtomicSets(comp_res, new_sets)?;
                }
                aux = new_sets.asets.clone();
            }
            outSet = addAtomicSets(aux, outSet)?;
        }
    } else {
        outSet = addAtomicSets(set1.asets.clone(), outSet)?;
    }
    Ok(outSet)
}

pub fn union(
    mut set1: &metamodelica::Ref<SBSet>,
    mut set2: &metamodelica::Ref<SBSet>,
) -> Result<metamodelica::Ref<SBSet>> {
    let mut outSet: metamodelica::Ref<SBSet>;
    let mut aux: metamodelica::Ref<SBSet>;
    outSet = metamodelica::Ref::new(SBSet {
        asets: UnorderedSet::copy(set1.asets.clone()),
        ndim: set1.ndim.clone(),
    });
    aux = complement(set2, &outSet)?;
    if !(isEmpty(&aux)) {
        outSet = addAtomicSets(aux.asets.clone(), outSet)?;
    }
    Ok(outSet)
}

pub(crate) fn card(mut set: &metamodelica::Ref<SBSet>) -> Result<i32> {
    let mut cardinality: i32 = UnorderedSet::fold(
        set.asets.clone(),
        &move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>, __a1: i32| SBAtomicSet::cardinality(&__a0, __a1),
        0,
    )?;
    Ok(cardinality)
}

pub(crate) fn maxCardinality(
    mut sets: metamodelica::Ref<Vector::Vector<metamodelica::Ref<SBSet>>>,
) -> Result<(metamodelica::Ref<SBSet>, i32)> {
    pub(crate) fn maxCardinality_traverse(mut set: &metamodelica::Ref<SBSet>, mut maxCard: i32) -> Result<(bool, i32)> {
        let mut res: bool = false;
        let mut maxCard: i32 = maxCard;
        let mut cardinality: i32 = card(set)?;
        if cardinality > maxCard {
            res = true;
            maxCard = cardinality;
        }
        Ok((res, maxCard))
    }

    let mut maxSet: metamodelica::Ref<SBSet>;
    let mut index: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Vector::findFold(sets.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SBSet>, __a1: i32| maxCardinality_traverse(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SBSet>, i32) -> Result<(bool, i32)> + 'static>), 0)) {
        Ok((Some(__pa0), __pa1, _)) => (__pa0.clone(), __pa1.clone()),
        _ => {
        return Err("fail");
        },
    } };
    maxSet = metamodelica::Own::own(__pa0);
    index = metamodelica::Own::own(__pa1);
    Ok((maxSet, index))
}

pub(crate) fn minElem(mut set: &metamodelica::Ref<SBSet>) -> Result<metamodelica::Array<i32>> {
    fn lessFn(mut set1: metamodelica::Array<i32>, mut set2: metamodelica::Array<i32>) -> Result<bool> {
        let mut res: bool;
        res = Array::isLess(set1.clone(), set2.clone(), &fnptr!(intLt, i32, i32))?;
        Ok(res)
    }

    let mut res: metamodelica::Array<i32>;
    let mut min_elems: metamodelica::List<metamodelica::Array<i32>>;
    if isEmpty(set) {
        res = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        min_elems = ({
            let mut __acc: metamodelica::List<metamodelica::Array<i32>> = metamodelica::nil();
            for mut e in (UnorderedSet::toArray(set.asets.clone())).borrow().iter() {
                let __x = SBAtomicSet::minElem(&(e.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        res = List::minElement(&min_elems, &lessFn)?;
    }
    Ok(res)
}

pub(crate) fn isEqual(mut set1: &metamodelica::Ref<SBSet>, mut set2: &metamodelica::Ref<SBSet>) -> Result<bool> {
    let mut equal: bool = UnorderedSet::isEqual(set1.asets.clone(), set2.asets.clone())?;
    Ok(equal)
}

pub(crate) fn hash(mut set: &metamodelica::Ref<SBSet>) -> i32 {
    let mut hash: i32 = UnorderedSet::size(set.asets.clone());
    hash
}

pub fn toString(mut set: &metamodelica::Ref<SBSet>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*UnorderedSet::toString(
            set.asets.clone(),
            &move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(SBAtomicSet::toString(&__a0))
            },
            literal!("U"),
        )?);
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}
