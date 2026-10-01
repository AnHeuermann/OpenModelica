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
use crate::SBLinearMap;
use crate::SBPWAtomicLinearMap;
use crate::SBSet;
use crate::System;
use crate::UnorderedSet;
use crate::Vector;
use openmodelica_util_datatypes_basic::Array;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBPWLinearMap {
    pub dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>,
    pub lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>,
    pub ndim: i32,
}

impl metamodelica::gc::MMTrace for SBPWLinearMap {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.dom, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lmap, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ndim, __mmv)?;
        Ok(())
    }
}
impl Default for SBPWLinearMap {
    fn default() -> Self {
        Self {
            dom: Default::default(),
            lmap: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type PW_LINEAR_MAP = SBPWLinearMap;

pub(crate) fn new(
    mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>,
    mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>,
) -> Result<metamodelica::Ref<SBPWLinearMap>> {
    let mut map: metamodelica::Ref<SBPWLinearMap>;
    let mut dim: i32 = 0;
    let mut same_dims: bool = false;
    if metamodelica::arrayLength(dom.clone()) != metamodelica::arrayLength(lmap.clone()) {
        map = newEmpty();
        return Ok(map);
    }
    if !(dom.clone().borrow().is_empty()) {
        dim = SBSet::ndim(
            &({
                let __elt = (*metamodelica::index_checked(&dom.borrow(), 1)?).clone();
                __elt
            }),
        );
        same_dims = Array::all(
            dom.clone(),
            &({
                let __pe_b1 = dim;
                move |__pe_a0| Ok(SBSet::isDim(&__pe_a0, __pe_b1.clone()))
            }),
        )? && Array::all(
            lmap.clone(),
            &({
                let __pe_b1 = dim;
                move |__pe_a0| Ok(SBLinearMap::isDim(&__pe_a0, __pe_b1.clone()))
            }),
        )?;
    }
    if !(same_dims) {
        map = newEmpty();
    } else {
        map = metamodelica::Ref::new(SBPWLinearMap {
            dom: metamodelica::arrayFromVec(dom.clone().borrow().clone()),
            lmap: metamodelica::arrayFromVec(lmap.clone().borrow().clone()),
            ndim: dim,
        });
    }
    Ok(map)
}

pub fn newScalar(
    mut dom: metamodelica::Ref<SBSet::SBSet>,
    mut lmap: metamodelica::Ref<SBLinearMap::SBLinearMap>,
) -> metamodelica::Ref<SBPWLinearMap> {
    let mut map: metamodelica::Ref<SBPWLinearMap>;
    if SBSet::ndim(&dom) == SBLinearMap::ndim(&lmap) {
        map = metamodelica::Ref::new(SBPWLinearMap {
            dom: arrayCreate(1, dom),
            lmap: arrayCreate(1, lmap),
            ndim: 1,
        });
    } else {
        map = newEmpty();
    }
    map
}

pub fn newEmpty() -> metamodelica::Ref<SBPWLinearMap> {
    let mut map: metamodelica::Ref<SBPWLinearMap>;
    map = metamodelica::Ref::new(SBPWLinearMap {
        dom: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        lmap: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        ndim: 0,
    });
    map
}

pub(crate) fn newIdentity(mut set: metamodelica::Ref<SBSet::SBSet>) -> metamodelica::Ref<SBPWLinearMap> {
    let mut map: metamodelica::Ref<SBPWLinearMap>;
    let mut lmap: metamodelica::Ref<SBLinearMap::SBLinearMap> = SBLinearMap::newIdentity(SBSet::ndim(&set));
    map = metamodelica::Ref::new(SBPWLinearMap {
        dom: arrayCreate(1, set),
        lmap: arrayCreate(1, lmap),
        ndim: 1,
    });
    map
}

pub(crate) fn copy(mut map: metamodelica::Ref<SBPWLinearMap>) -> Result<metamodelica::Ref<SBPWLinearMap>> {
    let mut map: metamodelica::Ref<SBPWLinearMap> = map;
    assign_field!(
        map.dom = Array::map(map.dom.clone(), &fnptr!(SBSet::copy, metamodelica::Ref<SBSet::SBSet>))?,
        map.lmap = Array::map(map.lmap.clone(), &move |__a0: metamodelica::Ref<
            SBLinearMap::SBLinearMap,
        >|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(SBLinearMap::copy(&__a0))
        })?
    );
    Ok(map)
}

pub(crate) fn dom(mut map: &metamodelica::Ref<SBPWLinearMap>) -> metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> {
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> = map.dom.clone();
    dom
}

pub(crate) fn lmap(
    mut map: &metamodelica::Ref<SBPWLinearMap>,
) -> metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> {
    let mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    lmap
}

pub(crate) fn ndim(mut map: &metamodelica::Ref<SBPWLinearMap>) -> i32 {
    let mut ndim: i32 = map.ndim.clone();
    ndim
}

pub(crate) fn isEmpty(mut map: &metamodelica::Ref<SBPWLinearMap>) -> bool {
    let mut empty: bool = map.dom.clone().borrow().is_empty();
    empty
}

pub fn image(
    mut map: &metamodelica::Ref<SBPWLinearMap>,
    mut set: &metamodelica::Ref<SBSet::SBSet>,
) -> Result<metamodelica::Ref<SBSet::SBSet>> {
    fn add_set(
        mut aset: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
        mut map: &metamodelica::Ref<SBLinearMap::SBLinearMap>,
        mut set: metamodelica::Ref<SBSet::SBSet>,
    ) -> Result<metamodelica::Ref<SBSet::SBSet>> {
        let mut set: metamodelica::Ref<SBSet::SBSet> = set;
        let mut aux_map: metamodelica::Ref<SBPWAtomicLinearMap::SBPWAtomicLinearMap>;
        aux_map = SBPWAtomicLinearMap::new(aset, map);
        set = SBSet::addAtomicSet(SBPWAtomicLinearMap::image(&aux_map, aset)?, set.clone())?;
        Ok(set)
    }

    let mut outSet: metamodelica::Ref<SBSet::SBSet> = SBSet::newEmpty();
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut ss: metamodelica::Ref<SBSet::SBSet>;
    let mut partial_res: metamodelica::Ref<SBSet::SBSet>;
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        ss = ({
            let __elt = (*metamodelica::index_checked(&dom.borrow(), i)?).clone();
            __elt
        });
        ss = SBSet::intersection(&ss, set)?;
        partial_res = UnorderedSet::fold(
            SBSet::asets(&ss),
            &({
                let __pe_b1 = ({
                    let __elt = (*metamodelica::index_checked(&lmap.borrow(), i)?).clone();
                    __elt
                });
                move |__pe_a0, __pe_a2| add_set(&__pe_a0, &__pe_b1, __pe_a2)
            }),
            SBSet::newEmpty(),
        )?;
        outSet = SBSet::union(&outSet, &partial_res)?;
    }
    Ok(outSet)
}

pub fn preImage(
    mut map: &metamodelica::Ref<SBPWLinearMap>,
    mut set: &metamodelica::Ref<SBSet::SBSet>,
) -> Result<metamodelica::Ref<SBSet::SBSet>> {
    fn add_set(
        mut aset: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
        mut map: &metamodelica::Ref<SBLinearMap::SBLinearMap>,
        mut sets: metamodelica::Array<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>,
        mut set: metamodelica::Ref<SBSet::SBSet>,
    ) -> Result<metamodelica::Ref<SBSet::SBSet>> {
        let mut set: metamodelica::Ref<SBSet::SBSet> = set;
        let mut aux_map: metamodelica::Ref<SBPWAtomicLinearMap::SBPWAtomicLinearMap>;
        aux_map = SBPWAtomicLinearMap::new(aset, map);
        let __range0 = sets.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut as2 in __range0 {
            set = SBSet::addAtomicSet(SBPWAtomicLinearMap::preImage(&aux_map, &as2)?, set.clone())?;
        }
        Ok(set)
    }

    let mut outSet: metamodelica::Ref<SBSet::SBSet> = SBSet::newEmpty();
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut ss: metamodelica::Ref<SBSet::SBSet>;
    let mut partial_res: metamodelica::Ref<SBSet::SBSet>;
    let mut sets: metamodelica::Array<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>;
    sets = UnorderedSet::toArray(SBSet::asets(set));
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        ss = ({
            let __elt = (*metamodelica::index_checked(&dom.borrow(), i)?).clone();
            __elt
        });
        partial_res = SBSet::newEmpty();
        partial_res = UnorderedSet::fold(
            SBSet::asets(&ss),
            &({
                let __pe_b1 = ({
                    let __elt = (*metamodelica::index_checked(&lmap.borrow(), i)?).clone();
                    __elt
                });
                let __pe_b2 = sets.clone();
                move |__pe_a0, __pe_a3| add_set(&__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_a3)
            }),
            SBSet::newEmpty(),
        )?;
        outSet = SBSet::union(&outSet, &partial_res)?;
    }
    Ok(outSet)
}

pub(crate) fn compPW(
    mut map1: &metamodelica::Ref<SBPWLinearMap>,
    mut map2: &metamodelica::Ref<SBPWLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap>;
    let mut dom1: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> = map1.dom.clone();
    let mut dom2: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> = map2.dom.clone();
    let mut ress: metamodelica::Ref<Vector::Vector<metamodelica::Ref<SBSet::SBSet>>>;
    let mut lmap1: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> = map1.lmap.clone();
    let mut lmap2: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> = map2.lmap.clone();
    let mut reslm: metamodelica::Ref<Vector::Vector<metamodelica::Ref<SBLinearMap::SBLinearMap>>>;
    let mut aux_dom: metamodelica::Ref<SBSet::SBSet>;
    let mut new_dom: metamodelica::Ref<SBSet::SBSet>;
    let mut d1: metamodelica::Ref<SBSet::SBSet>;
    let mut d2: metamodelica::Ref<SBSet::SBSet>;
    let mut l1: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut l2: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut new_lm: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    if isEmpty(map1) || isEmpty(map2) {
        outMap = newEmpty();
        return Ok(outMap);
    }
    ress = Vector::new(0);
    reslm = Vector::new(0);
    for mut i in 1..=metamodelica::arrayLength(dom1.clone()) {
        d1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(dom1.clone(), i);
        for mut j in 1..=metamodelica::arrayLength(dom2.clone()) {
            d2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(dom2.clone(), j);
            aux_dom = image(map2, &d2)?;
            aux_dom = SBSet::intersection(&aux_dom, &d1)?;
            aux_dom = preImage(map2, &aux_dom)?;
            new_dom = SBSet::intersection(&aux_dom, &d2)?;
            if !(SBSet::isEmpty(&new_dom)) {
                l1 = ({
                    let __elt = (*metamodelica::index_checked(&lmap1.borrow(), i)?).clone();
                    __elt
                });
                l2 = ({
                    let __elt = (*metamodelica::index_checked(&lmap2.borrow(), j)?).clone();
                    __elt
                });
                new_lm = SBLinearMap::compose(&l1, &l2);
                Vector::push(ress.clone(), new_dom);
                Vector::push(reslm.clone(), new_lm);
            }
        }
    }
    outMap = new(Vector::toArray(ress), Vector::toArray(reslm))?;
    Ok(outMap)
}

pub(crate) fn minInvCompact(mut map: &metamodelica::Ref<SBPWLinearMap>) -> Result<metamodelica::Ref<SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap>;
    let mut aux_dom: metamodelica::Ref<SBSet::SBSet>;
    let mut dom_inv: metamodelica::Ref<SBSet::SBSet>;
    let mut aux_map: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut map_inv: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut min: metamodelica::Array<i32>;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut g: metamodelica::Array<metamodelica::Real>;
    let mut o: metamodelica::Array<metamodelica::Real>;
    if metamodelica::arrayLength(map.dom.clone()) != 1 {
        outMap = newEmpty();
        return Ok(outMap);
    }
    aux_dom = metamodelica::arrayGet(map.dom.clone(), 1)?;
    dom_inv = image(map, &aux_dom)?;
    aux_map = metamodelica::arrayGet(map.lmap.clone(), 1)?;
    map_inv = SBLinearMap::inverse(&aux_map)?;
    min = SBSet::minElem(&aux_dom)?;
    g = SBLinearMap::gain(&map_inv);
    o = SBLinearMap::offset(&map_inv);
    resg = metamodelica::arrayCreate(
        metamodelica::arrayLength(g.clone()),
        metamodelica::OrderedFloat(0.0_f64),
    );
    reso = metamodelica::arrayCreate(
        metamodelica::arrayLength(o.clone()),
        metamodelica::OrderedFloat(0.0_f64),
    );
    for mut i in 1..=metamodelica::arrayLength(g.clone()) {
        if ({
            let __elt = (*metamodelica::index_checked(&g.borrow(), i)?).clone();
            __elt
        }) == intReal(System::intMaxLit())
        {
            {
                let __cell0 = metamodelica::OrderedFloat((0) as f64);
                let __idx0 = i;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(resg.clone().clone(), __idx0, __cell0) }?;
            }
            {
                let __cell1 = intReal(
                    ({
                        let __elt = (*metamodelica::index_checked(&min.borrow(), i)?).clone();
                        __elt
                    }),
                );
                let __idx1 = i;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(reso.clone().clone(), __idx1, __cell1) }?;
            }
        } else {
            {
                let __cell2 = ({
                    let __elt = (*metamodelica::index_checked(&g.borrow(), i)?).clone();
                    __elt
                });
                let __idx2 = i;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(resg.clone().clone(), __idx2, __cell2) }?;
            }
            {
                let __cell3 = ({
                    let __elt = (*metamodelica::index_checked(&o.borrow(), i)?).clone();
                    __elt
                });
                let __idx3 = i;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(reso.clone().clone(), __idx3, __cell3) }?;
            }
        }
    }
    outMap = new(
        arrayCreate(1, dom_inv),
        arrayCreate(1, SBLinearMap::new(resg.clone(), reso.clone())?),
    )?;
    Ok(outMap)
}

pub fn wholeDom(mut map: &metamodelica::Ref<SBPWLinearMap>) -> Result<metamodelica::Ref<SBSet::SBSet>> {
    let mut set: metamodelica::Ref<SBSet::SBSet>;
    set = SBSet::newEmpty();
    let __range0 = map.dom.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut s in __range0 {
        set = SBSet::union(&set, &s)?;
    }
    Ok(set)
}

pub fn combine(
    mut map1: metamodelica::Ref<SBPWLinearMap>,
    mut map2: metamodelica::Ref<SBPWLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap>;
    let mut sres: metamodelica::Ref<Vector::Vector<metamodelica::Ref<SBSet::SBSet>>>;
    let mut lres: metamodelica::Ref<Vector::Vector<metamodelica::Ref<SBLinearMap::SBLinearMap>>>;
    let mut dom2: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut lm2: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut aux1: metamodelica::Ref<SBSet::SBSet>;
    let mut s2: metamodelica::Ref<SBSet::SBSet>;
    let mut new_dom: metamodelica::Ref<SBSet::SBSet>;
    if isEmpty(&map1) {
        outMap = copy(map2)?;
        return Ok(outMap);
    }
    if isEmpty(&map2) {
        outMap = copy(map1)?;
        return Ok(outMap);
    }
    sres = Vector::fromArray(map1.dom.clone());
    lres = Vector::fromArray(map1.lmap.clone());
    dom2 = map2.dom.clone();
    lm2 = map2.lmap.clone();
    aux1 = wholeDom(&map1)?;
    for mut i in 1..=metamodelica::arrayLength(dom2.clone()) {
        s2 = ({
            let __elt = (*metamodelica::index_checked(&dom2.borrow(), i)?).clone();
            __elt
        });
        new_dom = SBSet::complement(&s2, &aux1)?;
        if !(SBSet::isEmpty(&new_dom)) {
            Vector::push(sres.clone(), new_dom);
            Vector::push(
                lres.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&lm2.borrow(), i)?).clone();
                    __elt
                }),
            );
        }
    }
    outMap = new(Vector::toArray(sres), Vector::toArray(lres))?;
    Ok(outMap)
}

pub(crate) fn atomize(mut map: &metamodelica::Ref<SBPWLinearMap>) -> Result<metamodelica::Ref<SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap>;
    let mut dres: metamodelica::List<metamodelica::Ref<SBSet::SBSet>> = metamodelica::nil();
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> = map.dom.clone();
    let mut lres: metamodelica::List<metamodelica::Ref<SBLinearMap::SBLinearMap>> = metamodelica::nil();
    let mut lm: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut d: metamodelica::Ref<SBSet::SBSet>;
    let mut aux: metamodelica::Ref<SBSet::SBSet>;
    let mut l: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut asets: metamodelica::Array<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>;
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        d = ({
            let __elt = (*metamodelica::index_checked(&dom.borrow(), i)?).clone();
            __elt
        });
        l = ({
            let __elt = (*metamodelica::index_checked(&lm.borrow(), i)?).clone();
            __elt
        });
        asets = UnorderedSet::toArray(SBSet::asets(&d));
        let __range0 = asets.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut s in __range0 {
            aux = SBSet::newEmpty();
            aux = SBSet::addAtomicSet(s, aux)?;
            dres = metamodelica::cons(aux, dres);
            lres = metamodelica::cons(l.clone(), lres);
        }
    }
    outMap = new(
        metamodelica::arrayFromVec(
            metamodelica::Dangerous::listReverseInPlace(dres)
                .into_iter()
                .cloned()
                .collect(),
        ),
        metamodelica::arrayFromVec(
            metamodelica::Dangerous::listReverseInPlace(lres)
                .into_iter()
                .cloned()
                .collect(),
        ),
    )?;
    Ok(outMap)
}

pub(crate) fn isEqual(
    mut map1: &metamodelica::Ref<SBPWLinearMap>,
    mut map2: &metamodelica::Ref<SBPWLinearMap>,
) -> Result<bool> {
    let mut equal: bool;
    equal = Array::isEqualOnTrue(map1.dom.clone(), map2.dom.clone(), &move |__a0: metamodelica::Ref<
        SBSet::SBSet,
    >,
                                                                            __a1: metamodelica::Ref<
        SBSet::SBSet,
    >| SBSet::isEqual(&__a0, &__a1))?
        && Array::isEqualOnTrue(map1.lmap.clone(), map2.lmap.clone(), &move |__a0: metamodelica::Ref<
            SBLinearMap::SBLinearMap,
        >,
                                                                             __a1: metamodelica::Ref<
            SBLinearMap::SBLinearMap,
        >| {
            SBLinearMap::isEqual(&__a0, &__a1)
        })?;
    Ok(equal)
}

pub fn toString(mut map: &metamodelica::Ref<SBPWLinearMap>) -> Result<ArcStr> {
    fn helper(
        mut set: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
        mut lm: metamodelica::Ref<SBLinearMap::SBLinearMap>,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("{"));
            __mm_s.push_str(&*SBPWAtomicLinearMap::toString(
                &(metamodelica::Ref::new(SBPWAtomicLinearMap::SBPWAtomicLinearMap { dom: set, lmap: lm })),
            )?);
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    let mut r#str: ArcStr;
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    for mut i in ({
        let __s = metamodelica::arrayLength(dom.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        strl = metamodelica::cons(
            UnorderedSet::toString(
                SBSet::asets(
                    &({
                        let __elt = (*metamodelica::index_checked(&dom.borrow(), i)?).clone();
                        __elt
                    }),
                ),
                &({
                    let __pe_b1 = ({
                        let __elt = (*metamodelica::index_checked(&lmap.borrow(), i)?).clone();
                        __elt
                    });
                    move |__pe_a0| helper(__pe_a0, __pe_b1.clone())
                }),
                literal!("U"),
            )?,
            strl,
        );
    }
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("["));
        __mm_s.push_str(&*stringDelimitList(strl, literal!(",")));
        __mm_s.push_str(&*literal!("]"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}
