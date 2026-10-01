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
use crate::SBPWLinearMap;
use crate::SBSet;
use crate::System;
use crate::UnorderedSet;
use crate::Util;
use crate::Vector;
use openmodelica_util_datatypes_basic::Array;

pub(crate) fn minAtomPW(
    mut dom: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
    mut lm1: &metamodelica::Ref<SBLinearMap::SBLinearMap>,
    mut lm2: &metamodelica::Ref<SBLinearMap::SBLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    fn make_result(
        mut aset: metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
        mut map: metamodelica::Ref<SBLinearMap::SBLinearMap>,
    ) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
        let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
        let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
        let mut lm: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
        dom = arrayCreate(1, SBSet::addAtomicSet(aset, SBSet::newEmpty())?);
        lm = arrayCreate(1, map);
        outMap = SBPWLinearMap::new(dom.clone(), lm.clone())?;
        Ok(outMap)
    }

    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut g1: metamodelica::Array<metamodelica::Real>;
    let mut g2: metamodelica::Array<metamodelica::Real>;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut o1: metamodelica::Array<metamodelica::Real>;
    let mut o2: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut as_aux: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut lm_aux: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut dom_res: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut lm_res: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut d1: metamodelica::Ref<SBSet::SBSet>;
    let mut d2: metamodelica::Ref<SBSet::SBSet>;
    let mut g1i: metamodelica::Real;
    let mut g2i: metamodelica::Real;
    let mut o1i: metamodelica::Real;
    let mut o2i: metamodelica::Real;
    let mut xinter: metamodelica::Real;
    let mut inti: metamodelica::Ref<SBInterval::SBInterval>;
    let mut i1: metamodelica::Ref<SBInterval::SBInterval>;
    let mut i2: metamodelica::Ref<SBInterval::SBInterval>;
    g1 = SBLinearMap::gain(lm1);
    o1 = SBLinearMap::offset(lm1);
    g2 = SBLinearMap::gain(lm2);
    o2 = SBLinearMap::offset(lm2);
    ints = SBMultiInterval::intervals(&(SBAtomicSet::aset(dom)));
    as_aux = SBAtomicSet::copy(dom);
    lm_aux = SBLinearMap::copy(lm1);
    resg = metamodelica::arrayFromVec(g1.clone().borrow().clone());
    reso = metamodelica::arrayFromVec(o1.clone().borrow().clone());
    for mut i in 1..=metamodelica::arrayLength(g1.clone()) {
        g1i = ({
            let __elt = (*metamodelica::index_checked(&g1.borrow(), i)?).clone();
            __elt
        });
        g2i = ({
            let __elt = (*metamodelica::index_checked(&g2.borrow(), i)?).clone();
            __elt
        });
        o1i = ({
            let __elt = (*metamodelica::index_checked(&o1.borrow(), i)?).clone();
            __elt
        });
        o2i = ({
            let __elt = (*metamodelica::index_checked(&o2.borrow(), i)?).clone();
            __elt
        });
        inti = ({
            let __elt = (*metamodelica::index_checked(&ints.borrow(), i)?).clone();
            __elt
        });
        if g1i != g2i {
            xinter = metamodelica::real_div_checked((o2i - o1i), (g1i - g2i))?;
            if xinter <= metamodelica::OrderedFloat((SBInterval::lowerBound(&inti)) as f64) {
                if g2i < g1i {
                    lm_aux = SBLinearMap::copy(lm2);
                }
                outMap = make_result(as_aux, lm_aux)?;
            } else if xinter >= metamodelica::OrderedFloat((SBInterval::upperBound(&inti)) as f64) {
                if g2i > g1i {
                    lm_aux = SBLinearMap::copy(lm2);
                }
                outMap = make_result(as_aux, lm_aux)?;
            } else {
                i1 = SBInterval::new(
                    SBInterval::lowerBound(&inti),
                    SBInterval::stepValue(&inti),
                    (((xinter).floor()).0.floor() as i32),
                );
                i2 = SBInterval::new(
                    SBInterval::upperBound(&i1) + SBInterval::stepValue(&i1),
                    SBInterval::stepValue(&inti),
                    SBInterval::upperBound(&inti),
                );
                d1 = SBSet::addAtomicSet(SBAtomicSet::replace(i1, i, &as_aux)?, SBSet::newEmpty())?;
                d2 = SBSet::addAtomicSet(SBAtomicSet::replace(i2, i, &as_aux)?, SBSet::newEmpty())?;
                dom_res = metamodelica::arrayFromVec(list![d1, d2].into_iter().cloned().collect());
                if g1i > g2i {
                    lm_res = metamodelica::arrayFromVec(
                        list![SBLinearMap::copy(lm1), SBLinearMap::copy(lm2)]
                            .into_iter()
                            .cloned()
                            .collect(),
                    );
                } else {
                    lm_res = metamodelica::arrayFromVec(
                        list![SBLinearMap::copy(lm2), SBLinearMap::copy(lm1)]
                            .into_iter()
                            .cloned()
                            .collect(),
                    );
                }
                outMap = SBPWLinearMap::new(dom_res.clone(), lm_res.clone())?;
            }
            return Ok(outMap);
        } else if o1i != o2i {
            if o2i < o1i {
                lm_aux = SBLinearMap::copy(lm2);
            }
            outMap = make_result(as_aux, lm_aux)?;
            return Ok(outMap);
        }
    }
    outMap = make_result(as_aux, lm_aux)?;
    Ok(outMap)
}

pub(crate) fn minPW(
    mut dom: &metamodelica::Ref<SBSet::SBSet>,
    mut lm1: &metamodelica::Ref<SBLinearMap::SBLinearMap>,
    mut lm2: &metamodelica::Ref<SBLinearMap::SBLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut aux_dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut aux_lm: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut sres1: metamodelica::Ref<SBSet::SBSet>;
    let mut sres2: metamodelica::Ref<SBSet::SBSet>;
    let mut d: metamodelica::Ref<SBSet::SBSet>;
    let mut lres1: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut lres2: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut l: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut asets: metamodelica::Array<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>;
    let mut aux: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut as_aux: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut sres: metamodelica::List<metamodelica::Ref<SBSet::SBSet>> = metamodelica::nil();
    let mut lres: metamodelica::List<metamodelica::Ref<SBLinearMap::SBLinearMap>> = metamodelica::nil();
    sres1 = SBSet::newEmpty();
    lres1 = SBLinearMap::newEmpty();
    sres2 = SBSet::newEmpty();
    lres2 = SBLinearMap::newEmpty();
    if !(SBSet::isEmpty(dom)) {
        asets = UnorderedSet::toArray(SBSet::asets(dom));
        as_aux = ({
            let __elt = (*metamodelica::index_checked(&asets.borrow(), 1)?).clone();
            __elt
        });
        aux = minAtomPW(&as_aux, lm1, lm2)?;
        if !(SBPWLinearMap::isEmpty(&aux)) {
            sres1 = metamodelica::arrayGet(SBPWLinearMap::dom(&aux), 1)?;
            lres1 = metamodelica::arrayGet(SBPWLinearMap::lmap(&aux), 1)?;
            for mut i in 2..=metamodelica::arrayLength(asets.clone()) {
                aux = minAtomPW(
                    &({
                        let __elt = (*metamodelica::index_checked(&asets.borrow(), i)?).clone();
                        __elt
                    }),
                    lm1,
                    lm2,
                )?;
                aux_dom = SBPWLinearMap::dom(&aux);
                aux_lm = SBPWLinearMap::lmap(&aux);
                for mut i in 1..=metamodelica::arrayLength(aux_dom.clone()) {
                    d = ({
                        let __elt = (*metamodelica::index_checked(&aux_dom.borrow(), i)?).clone();
                        __elt
                    });
                    l = ({
                        let __elt = (*metamodelica::index_checked(&aux_lm.borrow(), i)?).clone();
                        __elt
                    });
                    if SBLinearMap::isEqual(&l, &lres1)? {
                        sres1 = SBSet::union(&sres1, &d)?;
                    } else {
                        if SBSet::isEmpty(&sres2) {
                            sres2 = SBSet::copy(d);
                            lres2 = SBLinearMap::copy(&l);
                        } else {
                            sres2 = SBSet::union(&sres2, &d)?;
                        }
                    }
                }
            }
        }
    }
    if !(SBSet::isEmpty(&sres2)) && !(SBLinearMap::isEmpty(&lres2)) {
        sres = metamodelica::cons(sres2, sres);
        lres = metamodelica::cons(lres2, lres);
    }
    if !(SBSet::isEmpty(&sres1)) && !(SBLinearMap::isEmpty(&lres1)) {
        sres = metamodelica::cons(sres1, sres);
        lres = metamodelica::cons(lres1, lres);
    }
    outMap = SBPWLinearMap::new(
        metamodelica::arrayFromVec(sres.into_iter().cloned().collect()),
        metamodelica::arrayFromVec(lres.into_iter().cloned().collect()),
    )?;
    Ok(outMap)
}

pub(crate) fn minMap(
    mut pw1: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    mut pw2: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap> = SBPWLinearMap::newEmpty();
    let mut d1: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut d2: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut lm1: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut lm2: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut d1i: metamodelica::Ref<SBSet::SBSet>;
    let mut dom: metamodelica::Ref<SBSet::SBSet>;
    let mut lm1i: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut aux: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    if SBPWLinearMap::isEmpty(pw1) || SBPWLinearMap::isEmpty(pw2) {
        return Ok(outMap);
    }
    d1 = SBPWLinearMap::dom(pw1);
    lm1 = SBPWLinearMap::lmap(pw1);
    d2 = SBPWLinearMap::dom(pw2);
    lm2 = SBPWLinearMap::lmap(pw2);
    for mut i in 1..=metamodelica::arrayLength(d1.clone()) {
        d1i = ({
            let __elt = (*metamodelica::index_checked(&d1.borrow(), i)?).clone();
            __elt
        });
        lm1i = ({
            let __elt = (*metamodelica::index_checked(&lm1.borrow(), i)?).clone();
            __elt
        });
        for mut j in 1..=metamodelica::arrayLength(d2.clone()) {
            dom = SBSet::intersection(
                &d1i,
                &({
                    let __elt = (*metamodelica::index_checked(&d2.borrow(), j)?).clone();
                    __elt
                }),
            )?;
            if !(SBSet::isEmpty(&dom)) {
                aux = minPW(
                    &dom,
                    &lm1i,
                    &({
                        let __elt = (*metamodelica::index_checked(&lm2.borrow(), j)?).clone();
                        __elt
                    }),
                )?;
                outMap = if (SBPWLinearMap::isEmpty(&outMap)) {
                    aux
                } else {
                    SBPWLinearMap::combine(aux, outMap)?
                };
            }
        }
    }
    Ok(outMap)
}

pub(crate) fn reduceMapN(
    mut pw: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    mut dim: i32,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut new_s: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut sres: metamodelica::Ref<Vector::Vector<metamodelica::Ref<SBSet::SBSet>>>;
    let mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut new_l: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut lres: metamodelica::Ref<Vector::Vector<metamodelica::Ref<SBLinearMap::SBLinearMap>>>;
    let mut pw_copy: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut new_map: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut di: metamodelica::Ref<SBSet::SBSet>;
    let mut new_domi: metamodelica::Ref<SBSet::SBSet>;
    let mut li: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut gdim: metamodelica::Real;
    let mut odim: metamodelica::Real;
    let mut off: i32;
    let mut mi: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut idim: metamodelica::Ref<SBInterval::SBInterval>;
    let mut new_inter: metamodelica::Ref<SBInterval::SBInterval>;
    let mut loint: i32;
    let mut hiint: i32;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut aux_as: metamodelica::Ref<SBAtomicSet::SBAtomicSet>;
    let mut aux_newd: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>;
    let mut asets: metamodelica::Array<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>;
    dom = SBPWLinearMap::dom(&pw);
    lmap = SBPWLinearMap::lmap(&pw);
    pw_copy = SBPWLinearMap::copy(pw)?;
    sres = Vector::fromArray(SBPWLinearMap::dom(&pw_copy));
    lres = Vector::fromArray(SBPWLinearMap::lmap(&pw_copy));
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        di = ({
            let __elt = (*metamodelica::index_checked(&dom.borrow(), i)?).clone();
            __elt
        });
        li = ({
            let __elt = (*metamodelica::index_checked(&lmap.borrow(), i)?).clone();
            __elt
        });
        gdim = metamodelica::arrayGet(SBLinearMap::gain(&li), dim)?;
        odim = metamodelica::arrayGet(SBLinearMap::offset(&li), dim)?;
        if gdim == metamodelica::OrderedFloat((1) as f64) && odim < metamodelica::OrderedFloat((0) as f64) {
            off = ((-(odim)).0.floor() as i32);
            asets = UnorderedSet::toArray(SBSet::asets(&di));
            let __range0 = asets.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut adom in __range0 {
                mi = SBAtomicSet::aset(&adom);
                idim = metamodelica::arrayGet(SBMultiInterval::intervals(&mi), dim)?;
                loint = SBInterval::lowerBound(&idim);
                hiint = SBInterval::upperBound(&idim);
                if hiint - loint > off * off {
                    new_s = metamodelica::arrayCreate(off, di.clone());
                    new_l = metamodelica::arrayCreate(off, li.clone());
                    for mut k in 1..=off {
                        resg = metamodelica::arrayFromVec(SBLinearMap::gain(&li).borrow().clone());
                        reso = metamodelica::arrayFromVec(SBLinearMap::offset(&li).borrow().clone());
                        {
                            let __cell1 = metamodelica::OrderedFloat((0) as f64);
                            let __idx1 = dim;
                            *metamodelica::index_mut_checked(&mut resg.clone().borrow_mut(), __idx1)? = __cell1;
                        }
                        {
                            let __cell2 = metamodelica::OrderedFloat((loint + k - off - 1) as f64);
                            let __idx2 = dim;
                            *metamodelica::index_mut_checked(&mut reso.clone().borrow_mut(), __idx2)? = __cell2;
                        }
                        {
                            let __cell3 = SBLinearMap::new(resg.clone(), reso.clone())?;
                            let __idx3 = k;
                            let _ = unsafe {
                                metamodelica::Dangerous::arrayInitSlotChecked(new_l.clone().clone(), __idx3, __cell3)
                            }?;
                        }
                        new_inter = SBInterval::new(loint + k - 1, off, hiint);
                        aux_as = SBAtomicSet::replace(new_inter, dim, &adom)?;
                        {
                            let __cell4 = SBSet::addAtomicSet(aux_as, SBSet::newEmpty())?;
                            let __idx4 = k;
                            let _ = unsafe {
                                metamodelica::Dangerous::arrayInitSlotChecked(new_s.clone().clone(), __idx4, __cell4)
                            }?;
                        }
                    }
                    new_map = SBPWLinearMap::new(new_s.clone(), new_l.clone())?;
                    aux_newd = UnorderedSet::new(
                        (std::sync::Arc::new(
                            move |__a0: metamodelica::Ref<SBAtomicSet::SBAtomicSet>| -> metamodelica::Result<_> {
                                ::std::result::Result::Ok(SBAtomicSet::hash(&__a0))
                            },
                        )
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<SBAtomicSet::SBAtomicSet>) -> Result<i32>
                                    + 'static,
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
                    let __range5 = asets.clone().borrow().iter().cloned().collect::<Vec<_>>();
                    for mut aux_asi in __range5 {
                        if !(SBAtomicSet::isEqual(&aux_asi, &adom)?) {
                            UnorderedSet::add(aux_asi, aux_newd.clone())?;
                        }
                    }
                    new_domi = SBSet::new(aux_newd)?;
                    if SBSet::isEmpty(&new_domi) {
                        if i < Vector::size(sres.clone()) {
                            Vector::remove(sres.clone(), i)?;
                            Vector::remove(lres.clone(), i)?;
                        } else {
                            Vector::shrink(sres.clone(), i + 1);
                            Vector::shrink(lres.clone(), i + 1);
                        }
                    } else {
                        Vector::update(sres.clone(), i, new_domi)?;
                    }
                    Vector::appendArray(sres.clone(), SBPWLinearMap::dom(&new_map));
                    Vector::appendArray(lres.clone(), SBPWLinearMap::lmap(&new_map));
                }
            }
        }
    }
    outMap = SBPWLinearMap::new(Vector::toArray(sres), Vector::toArray(lres))?;
    Ok(outMap)
}

pub(crate) fn mapInf(
    mut pw: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    fn max_inter(
        mut aset: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
        mut offset: metamodelica::Real,
        mut dim: i32,
        mut its: metamodelica::Real,
    ) -> Result<metamodelica::Real> {
        let mut its: metamodelica::Real = its;
        let mut is: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
        let mut i: metamodelica::Ref<SBInterval::SBInterval>;
        let mut hi: metamodelica::Real;
        let mut lo: metamodelica::Real;
        is = SBMultiInterval::intervals(&(SBAtomicSet::aset(aset)));
        i = ({
            let __elt = (*metamodelica::index_checked(&is.borrow(), dim)?).clone();
            __elt
        });
        hi = metamodelica::OrderedFloat((SBInterval::upperBound(&i)) as f64);
        lo = metamodelica::OrderedFloat((SBInterval::lowerBound(&i)) as f64);
        its = std::cmp::max(its, (metamodelica::real_div_checked((hi - lo), (offset).abs())?).ceil());
        Ok(its)
    }

    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut max_it: i32;
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut d: metamodelica::Ref<SBSet::SBSet>;
    let mut lm: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut off: metamodelica::Array<metamodelica::Real>;
    let mut a: metamodelica::Real;
    let mut b: metamodelica::Real;
    let mut its: metamodelica::Real;
    if SBPWLinearMap::isEmpty(&pw) {
        outMap = SBPWLinearMap::newEmpty();
        return Ok(outMap);
    }
    outMap = reduceMapN(pw.clone(), 1)?;
    for mut i in 2..=SBPWLinearMap::ndim(&outMap) {
        outMap = reduceMapN(pw.clone(), i)?;
    }
    max_it = 0;
    dom = SBPWLinearMap::dom(&outMap);
    lmap = SBPWLinearMap::lmap(&outMap);
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        d = ({
            let __elt = (*metamodelica::index_checked(&dom.borrow(), i)?).clone();
            __elt
        });
        lm = ({
            let __elt = (*metamodelica::index_checked(&lmap.borrow(), i)?).clone();
            __elt
        });
        gain = SBLinearMap::gain(&lm);
        off = SBLinearMap::offset(&lm);
        a = metamodelica::OrderedFloat((0) as f64);
        b = ({
            let __elt = (*metamodelica::index_checked(&gain.borrow(), 1)?).clone();
            __elt
        });
        for mut j in 1..=metamodelica::arrayLength(gain.clone()) {
            a = realMax(
                a,
                ({
                    let __elt = (*metamodelica::index_checked(&gain.borrow(), j)?).clone();
                    __elt
                }) * ({
                    let __elt = (*metamodelica::index_checked(&off.borrow(), j)?).clone();
                    __elt
                })
                .abs(),
            );
            b = realMin(
                b,
                ({
                    let __elt = (*metamodelica::index_checked(&gain.borrow(), j)?).clone();
                    __elt
                }),
            );
        }
        if a > metamodelica::OrderedFloat((0) as f64) {
            its = metamodelica::OrderedFloat((0) as f64);
            for mut dim in 1..=SBPWLinearMap::ndim(&outMap) {
                if ({
                    let __elt = (*metamodelica::index_checked(&gain.borrow(), dim)?).clone();
                    __elt
                }) == metamodelica::OrderedFloat((1) as f64)
                    && ({
                        let __elt = (*metamodelica::index_checked(&off.borrow(), dim)?).clone();
                        __elt
                    }) < metamodelica::OrderedFloat((0) as f64)
                {
                    its = UnorderedSet::fold(
                        SBSet::asets(&d),
                        &({
                            let __pe_b1 = ({
                                let __elt = (*metamodelica::index_checked(&off.borrow(), dim)?).clone();
                                __elt
                            });
                            let __pe_b2 = dim;
                            move |__pe_a0, __pe_a3| max_inter(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
                        }),
                        its,
                    )?;
                }
            }
            max_it = max_it + ((its).0.floor() as i32);
        } else if b == metamodelica::OrderedFloat((0) as f64) {
            max_it = max_it + 1;
        }
    }
    for mut i in 1..=Util::msb(max_it) {
        outMap = SBPWLinearMap::compPW(&outMap, &outMap)?;
    }
    Ok(outMap)
}

pub(crate) fn minAdjCompMap(
    mut pw2: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    mut pw1: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut dom: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut lmap: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut d: metamodelica::Ref<SBSet::SBSet>;
    let mut dom_inv: metamodelica::Ref<SBSet::SBSet>;
    let mut aux: metamodelica::Ref<SBSet::SBSet>;
    let mut lm_inv: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut aux_lm1: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut aux_lm2: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut lm_res: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    let mut inv_pw: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut aux_inv: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut aux_res: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut inf: metamodelica::Real;
    let mut g: metamodelica::Real;
    let mut min_aux: metamodelica::Array<i32>;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut off: metamodelica::Array<metamodelica::Real>;
    let mut gres: metamodelica::Array<metamodelica::Real>;
    let mut oi: metamodelica::Array<metamodelica::Real>;
    let mut ginv: metamodelica::Array<metamodelica::Real>;
    dom = SBPWLinearMap::dom(pw2);
    lmap = SBPWLinearMap::lmap(pw2);
    if metamodelica::arrayLength(dom.clone()) != 1 {
        outMap = SBPWLinearMap::newEmpty();
        return Ok(outMap);
    }
    d = ({
        let __elt = (*metamodelica::index_checked(&dom.borrow(), 1)?).clone();
        __elt
    });
    dom_inv = SBPWLinearMap::image(pw2, &d)?;
    lm_inv = SBLinearMap::inverse(
        &({
            let __elt = (*metamodelica::index_checked(&lmap.borrow(), 1)?).clone();
            __elt
        }),
    )?;
    inv_pw = SBPWLinearMap::newScalar(dom_inv.clone(), lm_inv.clone());
    inf = intReal(System::intMaxLit());
    if Array::maxElement(
        SBLinearMap::gain(&lm_inv),
        &fnptr!(realLt, metamodelica::Real, metamodelica::Real),
    )? < inf
    {
        outMap = SBPWLinearMap::compPW(pw1, &inv_pw)?;
    } else if Array::minElement(
        SBLinearMap::gain(&lm_inv),
        &fnptr!(realLt, metamodelica::Real, metamodelica::Real),
    )? == inf
    {
        if !(SBPWLinearMap::isEmpty(pw2)) {
            aux = SBPWLinearMap::image(pw1, &d)?;
            min_aux = SBSet::minElem(&aux)?;
            resg = arrayCreate(
                metamodelica::arrayLength(min_aux.clone()),
                metamodelica::OrderedFloat(0.0_f64),
            );
            reso = Array::map(min_aux.clone(), &fnptr!(intReal, i32))?;
            lm_res = SBLinearMap::new(resg.clone(), reso.clone())?;
            outMap = SBPWLinearMap::newScalar(dom_inv, lm_res);
        } else {
            outMap = SBPWLinearMap::newEmpty();
        }
    } else {
        min_aux = SBSet::minElem(&d)?;
        gain = SBLinearMap::gain(&lm_inv);
        off = SBLinearMap::offset(&lm_inv);
        resg = metamodelica::arrayCreate(
            metamodelica::arrayLength(gain.clone()),
            metamodelica::OrderedFloat(0.0_f64),
        );
        reso = metamodelica::arrayCreate(
            metamodelica::arrayLength(gain.clone()),
            metamodelica::OrderedFloat(0.0_f64),
        );
        for mut i in 1..=metamodelica::arrayLength(gain.clone()) {
            g = metamodelica::Dangerous::arrayGetNoBoundsChecking(gain.clone(), i);
            if g == inf {
                {
                    let __cell0 = metamodelica::OrderedFloat(0.0_f64);
                    let __idx0 = i;
                    let _ = unsafe {
                        metamodelica::Dangerous::arrayInitSlotChecked(resg.clone().clone(), __idx0, __cell0)
                    }?;
                }
                {
                    let __cell1 = intReal(
                        ({
                            let __elt = (*metamodelica::index_checked(&min_aux.borrow(), i)?).clone();
                            __elt
                        }),
                    );
                    let __idx1 = i;
                    let _ = unsafe {
                        metamodelica::Dangerous::arrayInitSlotChecked(reso.clone().clone(), __idx1, __cell1)
                    }?;
                }
            } else {
                {
                    let __cell2 = g;
                    let __idx2 = i;
                    let _ = unsafe {
                        metamodelica::Dangerous::arrayInitSlotChecked(resg.clone().clone(), __idx2, __cell2)
                    }?;
                }
                {
                    let __cell3 = ({
                        let __elt = (*metamodelica::index_checked(&off.borrow(), i)?).clone();
                        __elt
                    });
                    let __idx3 = i;
                    let _ = unsafe {
                        metamodelica::Dangerous::arrayInitSlotChecked(reso.clone().clone(), __idx3, __cell3)
                    }?;
                }
            }
        }
        aux_lm1 = SBLinearMap::new(resg.clone(), reso.clone())?;
        aux_inv = SBPWLinearMap::newScalar(dom_inv, aux_lm1);
        aux_res = SBPWLinearMap::compPW(pw1, &aux_inv)?;
        if SBPWLinearMap::isEmpty(&aux_res) {
            outMap = SBPWLinearMap::newEmpty();
        } else {
            aux = SBPWLinearMap::image(pw1, &d)?;
            min_aux = SBSet::minElem(&aux)?;
            lm_res = metamodelica::arrayGet(SBPWLinearMap::lmap(&aux_res), 1)?;
            gres = SBLinearMap::gain(&lm_res);
            oi = SBLinearMap::offset(&lm_res);
            ginv = SBLinearMap::gain(&lm_inv);
            for mut i in 1..=metamodelica::arrayLength(gain.clone()) {
                g = metamodelica::Dangerous::arrayGetNoBoundsChecking(gain.clone(), i);
                if g == inf {
                    {
                        let __cell4 = metamodelica::OrderedFloat(0.0_f64);
                        let __idx4 = i;
                        let _ = unsafe {
                            metamodelica::Dangerous::arrayInitSlotChecked(resg.clone().clone(), __idx4, __cell4)
                        }?;
                    }
                    {
                        let __cell5 = intReal(
                            ({
                                let __elt = (*metamodelica::index_checked(&min_aux.borrow(), i)?).clone();
                                __elt
                            }),
                        );
                        let __idx5 = i;
                        let _ = unsafe {
                            metamodelica::Dangerous::arrayInitSlotChecked(reso.clone().clone(), __idx5, __cell5)
                        }?;
                    }
                } else {
                    {
                        let __cell6 = ({
                            let __elt = (*metamodelica::index_checked(&gres.borrow(), i)?).clone();
                            __elt
                        });
                        let __idx6 = i;
                        let _ = unsafe {
                            metamodelica::Dangerous::arrayInitSlotChecked(resg.clone().clone(), __idx6, __cell6)
                        }?;
                    }
                    {
                        let __cell7 = ({
                            let __elt = (*metamodelica::index_checked(&oi.borrow(), i)?).clone();
                            __elt
                        });
                        let __idx7 = i;
                        let _ = unsafe {
                            metamodelica::Dangerous::arrayInitSlotChecked(reso.clone().clone(), __idx7, __cell7)
                        }?;
                    }
                }
            }
            aux_lm2 = SBLinearMap::new(resg.clone(), reso.clone())?;
            outMap = SBPWLinearMap::newScalar(metamodelica::arrayGet(SBPWLinearMap::dom(&aux_res), 1)?, aux_lm2);
        }
    }
    Ok(outMap)
}

pub(crate) fn minAdjMap(
    mut pw2: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    mut pw1: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut dom2: metamodelica::Array<metamodelica::Ref<SBSet::SBSet>>;
    let mut lm2: metamodelica::Array<metamodelica::Ref<SBLinearMap::SBLinearMap>>;
    let mut map1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut mapi: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut min_adj: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut min_m: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    if SBPWLinearMap::isEmpty(pw2) {
        outMap = SBPWLinearMap::newEmpty();
        return Ok(outMap);
    }
    dom2 = SBPWLinearMap::dom(pw2);
    lm2 = SBPWLinearMap::lmap(pw2);
    map1 = SBPWLinearMap::newScalar(
        ({
            let __elt = (*metamodelica::index_checked(&dom2.borrow(), 1)?).clone();
            __elt
        }),
        ({
            let __elt = (*metamodelica::index_checked(&lm2.borrow(), 1)?).clone();
            __elt
        }),
    );
    outMap = minAdjCompMap(&map1, pw1)?;
    for mut i in 1..=metamodelica::arrayLength(dom2.clone()) {
        mapi = SBPWLinearMap::newScalar(
            ({
                let __elt = (*metamodelica::index_checked(&dom2.borrow(), i)?).clone();
                __elt
            }),
            ({
                let __elt = (*metamodelica::index_checked(&lm2.borrow(), i)?).clone();
                __elt
            }),
        );
        min_adj = minAdjCompMap(&mapi, pw1)?;
        min_m = minMap(&outMap, &min_adj)?;
        outMap = SBPWLinearMap::combine(min_adj, outMap)?;
        if !(SBPWLinearMap::isEmpty(&min_m)) {
            outMap = SBPWLinearMap::combine(min_m, outMap)?;
        }
    }
    Ok(outMap)
}

pub fn connectedComponents(
    mut vss: metamodelica::Ref<SBSet::SBSet>,
    mut emap1: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    mut emap2: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut ermap1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut ermap2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut rmap1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut rmap2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut new_res: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut last_im: metamodelica::Ref<SBSet::SBSet>;
    let mut new_im: metamodelica::Ref<SBSet::SBSet>;
    let mut diff_im: metamodelica::Ref<SBSet::SBSet>;
    outMap = SBPWLinearMap::newIdentity(vss.clone());
    new_im = vss.clone();
    diff_im = vss.clone();
    while !(SBSet::isEmpty(&diff_im)) {
        ermap1 = SBPWLinearMap::compPW(&outMap, emap1)?;
        ermap2 = SBPWLinearMap::compPW(&outMap, emap2)?;
        rmap1 = minAdjMap(&ermap1, &ermap2)?;
        rmap2 = minAdjMap(&ermap2, &ermap1)?;
        rmap1 = SBPWLinearMap::combine(rmap1, outMap.clone())?;
        rmap2 = SBPWLinearMap::combine(rmap2, outMap.clone())?;
        new_res = minMap(&rmap1, &rmap2)?;
        last_im = new_im;
        new_im = SBPWLinearMap::image(&new_res, &vss)?;
        diff_im = SBSet::complement(&last_im, &new_im)?;
        if !(SBSet::isEmpty(&diff_im)) {
            outMap = mapInf(new_res)?;
            new_im = SBPWLinearMap::image(&outMap, &vss)?;
        }
    }
    Ok(outMap)
}

pub(crate) fn test() -> Result<()> {
    test1()?;
    test2()?;
    test3()?;
    Ok(())
}

pub(crate) fn make_set(
    mut i: metamodelica::List<metamodelica::Ref<SBInterval::SBInterval>>,
) -> Result<metamodelica::Ref<SBSet::SBSet>> {
    let mut s: metamodelica::Ref<SBSet::SBSet>;
    let mut ss: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<SBAtomicSet::SBAtomicSet>>>;
    ss = UnorderedSet::new(
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
    UnorderedSet::add(SBAtomicSet::new(&(SBMultiInterval::fromList(i)?)), ss.clone())?;
    s = SBSet::new(ss)?;
    Ok(s)
}

pub(crate) fn make_pw(
    mut i: metamodelica::List<metamodelica::Ref<SBInterval::SBInterval>>,
    mut gain: metamodelica::List<metamodelica::Real>,
    mut offset: metamodelica::List<metamodelica::Real>,
) -> Result<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>> {
    let mut pw: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut dom: metamodelica::Ref<SBSet::SBSet>;
    let mut lmap: metamodelica::Ref<SBLinearMap::SBLinearMap>;
    dom = make_set(i)?;
    lmap = SBLinearMap::new(
        metamodelica::arrayFromVec(gain.into_iter().cloned().collect()),
        metamodelica::arrayFromVec(offset.into_iter().cloned().collect()),
    )?;
    pw = SBPWLinearMap::newScalar(dom, lmap);
    Ok(pw)
}

pub(crate) fn test1() -> Result<()> {
    let mut vss: metamodelica::Ref<SBSet::SBSet>;
    let mut emap1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut emap2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut sets: metamodelica::List<metamodelica::Ref<SBSet::SBSet>>;
    let mut pws1: metamodelica::List<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>>;
    let mut pws2: metamodelica::List<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>>;
    let mut res: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    sets = list![
        make_set(list![SBInterval::new(1, 1, 1)])?,
        make_set(list![SBInterval::new(2, 1, 1001)])?,
        make_set(list![SBInterval::new(1002, 1, 1002)])?,
        make_set(list![SBInterval::new(1003, 1, 1003)])?,
        make_set(list![SBInterval::new(1004, 1, 2003)])?,
        make_set(list![SBInterval::new(2004, 1, 3003)])?,
        make_set(list![SBInterval::new(3004, 1, 4003)])?
    ];
    vss = SBSet::newEmpty();
    for mut s in &*sets {
        vss = SBSet::union(&vss, metamodelica::AsArg::as_arg(&s))?;
    }
    pws1 = list![
        make_pw(
            list![SBInterval::new(1, 1, 1)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2, 1, 2)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1002.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(3, 1, 1001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(1001.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(1002, 1, 2001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(1002.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2002, 1, 3001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(1002.0_f64)]
        )?
    ];
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pws1) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    emap1 = metamodelica::Own::own(__pa0);
    pws1 = metamodelica::Own::own(__pa1);
    for mut pw in &*pws1 {
        emap1 = SBPWLinearMap::combine(pw.clone(), emap1)?;
    }
    pws2 = list![
        make_pw(
            list![SBInterval::new(1, 1, 1)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(2.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2, 1, 2)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1003.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(3, 1, 1001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(0.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(1002, 1, 2001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(2.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2002, 1, 3001)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1003.0_f64)]
        )?
    ];
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(pws2) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    emap2 = metamodelica::Own::own(__pa2);
    pws2 = metamodelica::Own::own(__pa3);
    for mut pw in &*pws2 {
        emap2 = SBPWLinearMap::combine(pw.clone(), emap2)?;
    }
    res = connectedComponents(vss, &emap1, &emap2)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*SBPWLinearMap::toString(&res)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn test2() -> Result<()> {
    let mut vss: metamodelica::Ref<SBSet::SBSet>;
    let mut emap1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut emap2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut sets: metamodelica::List<metamodelica::Ref<SBSet::SBSet>>;
    let mut pws1: metamodelica::List<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>>;
    let mut pws2: metamodelica::List<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>>;
    let mut res: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    sets = list![
        make_set(list![SBInterval::new(1, 1, 1)])?,
        make_set(list![SBInterval::new(2, 1, 1001)])?,
        make_set(list![SBInterval::new(1002, 1, 1002)])?,
        make_set(list![SBInterval::new(1003, 1, 1003)])?,
        make_set(list![SBInterval::new(1004, 1, 2003)])?,
        make_set(list![SBInterval::new(2004, 1, 3003)])?,
        make_set(list![SBInterval::new(3004, 1, 4003)])?
    ];
    vss = SBSet::newEmpty();
    for mut s in &*sets {
        vss = SBSet::union(&vss, metamodelica::AsArg::as_arg(&s))?;
    }
    pws1 = list![
        make_pw(
            list![SBInterval::new(1, 1, 1)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2, 1, 2)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1002.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(3, 1, 3)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1004.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(4, 1, 1002)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(2000.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(1003, 1, 2001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(2.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2002, 1, 3001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(1002.0_f64)]
        )?
    ];
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pws1) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    emap1 = metamodelica::Own::own(__pa0);
    pws1 = metamodelica::Own::own(__pa1);
    for mut pw in &*pws1 {
        emap1 = SBPWLinearMap::combine(pw.clone(), emap1)?;
    }
    pws2 = list![
        make_pw(
            list![SBInterval::new(1, 1, 1)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(2.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2, 1, 2)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1003.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(3, 1, 3)],
            list![metamodelica::OrderedFloat(0.0_f64)],
            list![metamodelica::OrderedFloat(1003.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(4, 1, 1002)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(-1.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(1003, 1, 2001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(1.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(2002, 1, 3001)],
            list![metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(2.0_f64)]
        )?
    ];
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(pws2) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    emap2 = metamodelica::Own::own(__pa2);
    pws2 = metamodelica::Own::own(__pa3);
    for mut pw in &*pws2 {
        emap2 = SBPWLinearMap::combine(pw.clone(), emap2)?;
    }
    res = connectedComponents(vss, &emap1, &emap2)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*SBPWLinearMap::toString(&res)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn test3() -> Result<()> {
    let mut vss: metamodelica::Ref<SBSet::SBSet>;
    let mut emap1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut emap2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut res: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut sets: metamodelica::List<metamodelica::Ref<SBSet::SBSet>>;
    let mut pws1: metamodelica::List<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>>;
    let mut pws2: metamodelica::List<metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>>;
    sets = list![
        make_set(list![SBInterval::new(1, 1, 1000), SBInterval::new(1, 1, 100)])?,
        make_set(list![SBInterval::new(1001, 1, 2000), SBInterval::new(101, 1, 200)])?,
        make_set(list![SBInterval::new(2001, 1, 3000), SBInterval::new(201, 1, 300)])?,
        make_set(list![SBInterval::new(3001, 1, 4000), SBInterval::new(301, 1, 400)])?,
        make_set(list![SBInterval::new(4001, 1, 4001)])?,
        make_set(list![SBInterval::new(4002, 1, 4002)])?
    ];
    vss = SBSet::newEmpty();
    for mut s in &*sets {
        vss = SBSet::union(&vss, metamodelica::AsArg::as_arg(&s))?;
    }
    pws1 = list![
        make_pw(
            list![SBInterval::new(1, 1, 999), SBInterval::new(1, 1, 99)],
            list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)],
            list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(0.0_f64)]
        )?,
        make_pw(
            list![SBInterval::new(1000, 1, 1998), SBInterval::new(100, 1, 198)],
            list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)],
            list![
                metamodelica::OrderedFloat(1001.0_f64),
                metamodelica::OrderedFloat(101.0_f64)
            ]
        )?,
        make_pw(
            list![SBInterval::new(1999, 1, 2998), SBInterval::new(199, 1, 199)],
            list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(0.0_f64)],
            list![
                metamodelica::OrderedFloat(-1998.0_f64),
                metamodelica::OrderedFloat(100.0_f64)
            ]
        )?,
        make_pw(
            list![SBInterval::new(2999, 1, 2999), SBInterval::new(200, 1, 299)],
            list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(1.0_f64)],
            list![
                metamodelica::OrderedFloat(3001.0_f64),
                metamodelica::OrderedFloat(101.0_f64)
            ]
        )?,
        make_pw(
            list![SBInterval::new(3000, 1, 3000), SBInterval::new(300, 1, 399)],
            list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(1.0_f64)],
            list![
                metamodelica::OrderedFloat(3000.0_f64),
                metamodelica::OrderedFloat(-99.0_f64)
            ]
        )?
    ];
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pws1) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    emap1 = metamodelica::Own::own(__pa0);
    pws1 = metamodelica::Own::own(__pa1);
    for mut pw in &*pws1 {
        emap1 = SBPWLinearMap::combine(pw.clone(), emap1)?;
    }
    pws2 = list![
        make_pw(
            list![SBInterval::new(1, 1, 999), SBInterval::new(1, 1, 99)],
            list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)],
            list![
                metamodelica::OrderedFloat(1000.0_f64),
                metamodelica::OrderedFloat(101.0_f64)
            ]
        )?,
        make_pw(
            list![SBInterval::new(1000, 1, 1998), SBInterval::new(100, 1, 198)],
            list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)],
            list![
                metamodelica::OrderedFloat(2002.0_f64),
                metamodelica::OrderedFloat(201.0_f64)
            ]
        )?,
        make_pw(
            list![SBInterval::new(1999, 1, 2998), SBInterval::new(199, 1, 199)],
            list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(0.0_f64)],
            list![
                metamodelica::OrderedFloat(-998.0_f64),
                metamodelica::OrderedFloat(101.0_f64)
            ]
        )?,
        make_pw(
            list![SBInterval::new(2999, 1, 2999), SBInterval::new(200, 1, 299)],
            list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(0.0_f64)],
            list![
                metamodelica::OrderedFloat(4001.0_f64),
                metamodelica::OrderedFloat(4001.0_f64)
            ]
        )?,
        make_pw(
            list![SBInterval::new(3000, 1, 3000), SBInterval::new(300, 1, 399)],
            list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(0.0_f64)],
            list![
                metamodelica::OrderedFloat(4002.0_f64),
                metamodelica::OrderedFloat(4002.0_f64)
            ]
        )?
    ];
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(pws2) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    emap2 = metamodelica::Own::own(__pa2);
    pws2 = metamodelica::Own::own(__pa3);
    for mut pw in &*pws2 {
        emap2 = SBPWLinearMap::combine(pw.clone(), emap2)?;
    }
    res = connectedComponents(vss, &emap1, &emap2)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*SBPWLinearMap::toString(&res)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}
