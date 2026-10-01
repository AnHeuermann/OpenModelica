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

use crate::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/// An implementation of a generic unordered set, a.k.a. hash set.
///
///   This implementation uses separate chaining and automatically rehashes the set
///   when the load factor becomes too large to keep the performance up.
#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct UnorderedSet<T: Clone> {
    pub buckets: Mutable::Mutable<metamodelica::Array<metamodelica::List<T>>>,
    pub size: Mutable::Mutable<i32>,
    pub hashFn: Hash<T>,
    pub eqFn: KeyEq<T>,
}

impl<T: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace for UnorderedSet<T> {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.buckets, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.size, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hashFn, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqFn, __mmv)?;
        Ok(())
    }
}
impl<T: Clone + 'static + PartialEq> PartialEq for UnorderedSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.buckets == other.buckets
            && self.size == other.size
            && std::sync::Arc::ptr_eq((&self.hashFn), (&other.hashFn))
            && std::sync::Arc::ptr_eq((&self.eqFn), (&other.eqFn))
    }
}
impl<T: Clone + 'static + PartialEq + Eq> Eq for UnorderedSet<T> {}
impl<T: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> PartialOrd for UnorderedSet<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> Ord for UnorderedSet<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.buckets
            .cmp(&other.buckets)
            .then_with(|| self.size.cmp(&other.size))
            .then_with(|| {
                (std::sync::Arc::as_ptr((&self.hashFn)) as *const ())
                    .cmp(&(std::sync::Arc::as_ptr((&other.hashFn)) as *const ()))
            })
            .then_with(|| {
                (std::sync::Arc::as_ptr((&self.eqFn)) as *const ())
                    .cmp(&(std::sync::Arc::as_ptr((&other.eqFn)) as *const ()))
            })
    }
}
impl<T: Clone + 'static + std::fmt::Debug> std::fmt::Debug for UnorderedSet<T> {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("UnorderedSet");
        __ds.field("buckets", &self.buckets);
        __ds.field("size", &self.size);
        __ds.field(
            "hashFn",
            &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.hashFn))),
        );
        __ds.field("eqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.eqFn))));
        __ds.finish()
    }
}

impl<T: Clone + 'static + metamodelica::gc::MMTrace> Default for UnorderedSet<T> {
    fn default() -> Self {
        Self {
            buckets: Default::default(),
            size: Default::default(),
            hashFn: {
                let __placeholder: Hash<T> =
                    std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                __placeholder
            },
            eqFn: {
                let __placeholder: KeyEq<T> =
                    std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                __placeholder
            },
        }
    }
}

pub type UNORDERED_SET<T> = UnorderedSet<T>;

pub type Hash<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>;

pub type KeyEq<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

pub fn new<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut hash: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>,
    mut keyEq: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
    mut bucketCount: i32,
) -> metamodelica::Ref<UnorderedSet<T>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    let mut buckets: Mutable::Mutable<metamodelica::Array<metamodelica::List<T>>>;
    buckets = Mutable::create(arrayCreate(bucketCount, metamodelica::nil()));
    set = metamodelica::Ref::new(UnorderedSet {
        buckets: buckets,
        size: Mutable::create(0),
        hashFn: hash.clone(),
        eqFn: keyEq.clone(),
    });
    set
}

pub fn fromList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut elements: &metamodelica::List<T>,
    mut hash: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>,
    mut keyEq: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    set = new(hash.clone(), keyEq.clone(), Util::nextPrime(((elements).len() as i32)));
    for mut e in &**elements {
        add(e.clone(), set.clone())?;
    }
    Ok(set)
}

pub fn copy<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> metamodelica::Ref<UnorderedSet<T>> {
    let mut outSet: metamodelica::Ref<UnorderedSet<T>>;
    outSet = metamodelica::Ref::new(UnorderedSet {
        buckets: Mutable::create(metamodelica::arrayFromVec(
            Mutable::access(set.buckets.clone()).borrow().clone(),
        )),
        size: Mutable::create(Mutable::access(set.size.clone())),
        hashFn: set.hashFn.clone(),
        eqFn: set.eqFn.clone(),
    });
    outSet
}

pub fn add<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<bool> {
    let mut added: bool;
    let mut hash: i32;
    let mut okey: Option<T>;
    (okey, hash) = find(key.clone(), set.clone())?;
    added = (okey).is_none();
    if added {
        addKey(key, hash, set)?;
    }
    Ok(added)
}

pub fn addNew<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<()> {
    let mut hashfn: Hash<T> = set.hashFn.clone();
    let mut hash: i32;
    hash = intMod(
        hashfn(key.clone())?,
        metamodelica::arrayLength(Mutable::access(set.buckets.clone())),
    );
    addKey(key, hash, set)?;
    Ok(())
}

pub fn addUnique<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<()> {
    let mut hash: i32;
    let __pa0 = ::match_deref::match_deref! { match &(find(key.clone(), set.clone())?) {
        (None, __pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    hash = metamodelica::Own::own(__pa0);
    addKey(key, hash, set)?;
    Ok(())
}

pub fn remove<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<bool> {
    let mut removed: bool;
    let mut buckets: metamodelica::Array<metamodelica::List<T>> = Mutable::access(set.buckets.clone());
    let mut hashfn: Hash<T> = set.hashFn.clone();
    let mut eqfn: KeyEq<T> = set.eqFn.clone();
    let mut hash: i32;
    let mut bucket: metamodelica::List<T>;
    let mut okey: Option<T>;
    hash = intMod(hashfn(key.clone())?, metamodelica::arrayLength(buckets.clone()));
    bucket = metamodelica::arrayGet(buckets.clone(), hash + 1)?;
    (bucket, okey) = List::deleteMemberOnTrue(key, bucket, &*(eqfn.clone()))?;
    removed = (okey).is_some();
    if removed {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(buckets.clone(), hash + 1, bucket);
        Mutable::update(set.size.clone(), Mutable::access(set.size.clone()) - 1);
    }
    Ok(removed)
}

pub fn get<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<Option<T>> {
    let mut outKey: Option<T>;
    (outKey, _) = find(key, set)?;
    Ok(outKey)
}

pub(crate) fn getOrFail<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<T> {
    let mut outKey: T;
    let mut okey: Option<T>;
    (okey, _) = find(key, set)?;
    let __pa0 = ::match_deref::match_deref! { match &(okey) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outKey = metamodelica::Own::own(__pa0);
    Ok(outKey)
}

pub fn contains<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<bool> {
    let mut res: bool;
    res = ((find(key, set)?).0).is_some();
    Ok(res)
}

pub fn first<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: metamodelica::Ref<UnorderedSet<T>>) -> Result<T> {
    let mut val: T;
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        if !((b).is_empty()) {
            val = (b).head().cloned()?;
            return Ok(val);
        }
    }
    return Err("fail");
    Ok(val)
}

pub fn isEqual<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set1: metamodelica::Ref<UnorderedSet<T>>,
    mut set2: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<bool> {
    let mut equal: bool = true;
    if Mutable::access(set1.size.clone()) != Mutable::access(set2.size.clone()) {
        equal = false;
        return Ok(equal);
    }
    let __range0 = Mutable::access(set1.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            if !(contains(k.clone(), set2.clone())?) {
                equal = false;
                return Ok(equal);
            }
        }
    }
    Ok(equal)
}

pub fn toList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            outList = metamodelica::cons(k.clone(), outList);
        }
    }
    outList
}

pub fn toArray<T: Clone + 'static + metamodelica::gc::MMTrace + Default>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> metamodelica::Array<T> {
    let mut outArray: metamodelica::Array<T>;
    let mut dummy: T;
    let mut i: i32 = 1;
    outArray = metamodelica::arrayCreateDefault(Mutable::access(set.size.clone()));
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i, k.clone()) };
            i = i + 1;
        }
    }
    outArray
}

pub fn fold<T: Clone + 'static + metamodelica::gc::MMTrace, FT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T, FT) -> Result<FT>,
    mut startValue: FT,
) -> Result<FT> {
    pub type FoldFn<T: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>;

    let mut result: FT = startValue;
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            result = r#fn(k.clone(), result)?;
        }
    }
    Ok(result)
}

pub fn selfMap<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<T>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    pub type MapFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outSet: metamodelica::Ref<UnorderedSet<T>> = new(set.hashFn.clone(), set.eqFn.clone(), 13);
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            add(r#fn(k.clone())?, outSet.clone())?;
        }
    }
    Ok(outSet)
}

pub(crate) fn apply<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<()>,
) -> Result<()> {
    pub type ApplyFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            r#fn(k.clone())?;
        }
    }
    Ok(())
}

pub fn all<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut res: bool;
    if isEmpty(set.clone()) {
        res = true;
        return Ok(res);
    }
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            if !(r#fn(k.clone())?) {
                res = false;
                return Ok(res);
            }
        }
    }
    res = true;
    Ok(res)
}

pub(crate) fn any<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut res: bool;
    if isEmpty(set.clone()) {
        res = false;
        return Ok(res);
    }
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            if r#fn(k.clone())? {
                res = true;
                return Ok(res);
            }
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn none<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut res: bool;
    if isEmpty(set.clone()) {
        res = true;
        return Ok(res);
    }
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            if r#fn(k.clone())? {
                res = false;
                return Ok(res);
            }
        }
    }
    res = true;
    Ok(res)
}

pub fn filterOnFalse<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut falseSet: metamodelica::Ref<UnorderedSet<T>> = new(set.hashFn.clone(), set.eqFn.clone(), 13);
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            if !(r#fn(k.clone())?) {
                add(k.clone(), falseSet.clone())?;
            }
        }
    }
    Ok(falseSet)
}

pub fn splitOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(metamodelica::Ref<UnorderedSet<T>>, metamodelica::Ref<UnorderedSet<T>>)> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut trueSet: metamodelica::Ref<UnorderedSet<T>> = new(set.hashFn.clone(), set.eqFn.clone(), 13);
    let mut falseSet: metamodelica::Ref<UnorderedSet<T>> = new(set.hashFn.clone(), set.eqFn.clone(), 13);
    let __range0 = Mutable::access(set.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            add(
                k.clone(),
                if (r#fn(k.clone())?) {
                    trueSet.clone()
                } else {
                    falseSet.clone()
                },
            )?;
        }
    }
    Ok((trueSet, falseSet))
}

pub fn size<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: metamodelica::Ref<UnorderedSet<T>>) -> i32 {
    let mut s: i32 = Mutable::access(set.size.clone());
    s
}

pub fn isEmpty<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: metamodelica::Ref<UnorderedSet<T>>) -> bool {
    let mut empty: bool = Mutable::access(set.size.clone()) == 0;
    empty
}

pub(crate) fn bucketCount<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> i32 {
    let mut count: i32 = metamodelica::arrayLength(Mutable::access(set.buckets.clone()));
    count
}

pub(crate) fn loadFactor<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<metamodelica::Real> {
    let mut load: metamodelica::Real = metamodelica::real_div_checked(
        intReal(Mutable::access(set.size.clone())),
        metamodelica::OrderedFloat((bucketCount(set.clone())) as f64),
    )?;
    Ok(load)
}

pub(crate) fn rehash<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<()> {
    let mut old_buckets: metamodelica::Array<metamodelica::List<T>> = Mutable::access(set.buckets.clone());
    let mut new_buckets: metamodelica::Array<metamodelica::List<T>>;
    let mut bucket_count: i32;
    let mut hash: i32;
    let mut hashfn: Hash<T> = set.hashFn.clone();
    bucket_count = Util::nextPrime(Mutable::access(set.size.clone()) * 2);
    new_buckets = arrayCreate(bucket_count, metamodelica::nil());
    let __range0 = old_buckets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            hash = intMod(hashfn(k.clone())?, bucket_count);
            metamodelica::arrayUpdate(
                new_buckets.clone(),
                hash + 1,
                metamodelica::cons(k.clone(), metamodelica::arrayGet(new_buckets.clone(), hash + 1)?),
            )?;
        }
    }
    Mutable::update(set.buckets.clone(), new_buckets.clone());
    Ok(())
}

pub fn toString<T: Clone + 'static + metamodelica::gc::MMTrace + Default>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut stringFn: &dyn ::std::ops::Fn(T) -> Result<ArcStr>,
    mut delimiter: ArcStr,
) -> Result<ArcStr> {
    pub type StringFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut k in (toArray(set)).borrow().iter() {
                let __x = stringFn(k.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        delimiter,
    );
    Ok(r#str)
}

pub(crate) fn dump<T: Clone + 'static + metamodelica::gc::MMTrace + Default>(
    mut set: metamodelica::Ref<UnorderedSet<T>>,
    mut stringFn: &dyn ::std::ops::Fn(T) -> Result<ArcStr>,
) -> Result<()> {
    pub type StringFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    metamodelica::print(toString(set, stringFn, literal!("\n"))?);
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub fn unique_list<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>,
    mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = if (List::hasSeveralElements(&inList)) {
        toList(fromList(&inList, hashFunc.clone(), keyEqFunc.clone())?)
    } else {
        inList.clone()
    };
    Ok(outList)
}

pub fn union<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set1: metamodelica::Ref<UnorderedSet<T>>,
    mut set2: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    let mut sz1: i32;
    let mut sz2: i32;
    let mut small_sz: i32;
    let mut small_set: metamodelica::Ref<UnorderedSet<T>>;
    sz1 = Mutable::access(set1.size.clone());
    sz2 = Mutable::access(set2.size.clone());
    if sz1 > sz2 {
        set = set1;
        small_set = set2;
        small_sz = sz2;
    } else {
        set = set2;
        small_set = set1;
        small_sz = sz1;
    }
    if small_sz > 0 {
        let __range0 = Mutable::access(small_set.buckets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut b in __range0 {
            for mut k in &*b {
                add(k.clone(), set.clone())?;
            }
        }
    }
    Ok(set)
}

pub fn union_list<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set_lst: &metamodelica::List<metamodelica::Ref<UnorderedSet<T>>>,
    mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>,
    mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    let mut rest: metamodelica::List<metamodelica::Ref<UnorderedSet<T>>>;
    if (set_lst).is_empty() {
        set = new(hashFunc.clone(), keyEqFunc.clone(), 13);
    } else {
        (set, rest) = extractFromLst(set_lst, &fnptr!(intGt, i32, i32))?;
        for mut tmp in &*rest {
            set = union(set, tmp.clone())?;
        }
    }
    Ok(set)
}

pub fn merge<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set1: metamodelica::Ref<UnorderedSet<T>>,
    mut set2: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set1: metamodelica::Ref<UnorderedSet<T>> = set1;
    if isEmpty(set2.clone()) {
        return Ok(set1);
    }
    let __range0 = Mutable::access(set2.buckets.clone())
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b {
            add(k.clone(), set1.clone())?;
        }
    }
    Ok(set1)
}

pub(crate) fn intersection<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set1: metamodelica::Ref<UnorderedSet<T>>,
    mut set2: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    let mut set_small: metamodelica::Ref<UnorderedSet<T>>;
    let mut set_big: metamodelica::Ref<UnorderedSet<T>>;
    let mut acc: metamodelica::List<T> = metamodelica::nil();
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set_small = set2;
        set_big = set1.clone();
    } else {
        set_small = set1.clone();
        set_big = set2;
    }
    if !(isEmpty(set_small.clone())) {
        let __range0 = Mutable::access(set_small.buckets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut b in __range0 {
            for mut k in &*b {
                if contains(k.clone(), set_big.clone())? {
                    acc = metamodelica::cons(k.clone(), acc);
                }
            }
        }
    }
    set = fromList(&acc, set1.hashFn.clone(), set1.eqFn.clone())?;
    Ok(set)
}

pub(crate) fn intersection_list<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set_lst: &metamodelica::List<metamodelica::Ref<UnorderedSet<T>>>,
    mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>,
    mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    let mut set_small: metamodelica::Ref<UnorderedSet<T>>;
    let mut rest: metamodelica::List<metamodelica::Ref<UnorderedSet<T>>>;
    let mut acc: metamodelica::List<T> = metamodelica::nil();
    if !((set_lst).is_empty()) {
        (set_small, rest) = extractFromLst(set_lst, &fnptr!(intLt, i32, i32))?;
        let __range0 = Mutable::access(set_small.buckets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut b in __range0 {
            for mut k in &*b {
                if List::all(
                    &rest,
                    &({
                        let __pe_b0 = k.clone();
                        move |__pe_a1| contains(__pe_b0.clone(), __pe_a1)
                    }),
                )? {
                    acc = metamodelica::cons(k.clone(), acc);
                }
            }
        }
    }
    set = fromList(&acc, hashFunc.clone(), keyEqFunc.clone())?;
    Ok(set)
}

pub fn difference_list<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
    mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>,
    mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<T>> {
    let mut acc: metamodelica::List<T> = metamodelica::nil();
    let mut set2: metamodelica::Ref<UnorderedSet<T>>;
    let mut lst1: metamodelica::List<T> = inList1;
    let mut lst2: metamodelica::List<T> = inList2;
    while !((lst1).is_empty() || (lst2).is_empty()) && keyEqFunc((lst1).head().cloned()?, (lst2).head().cloned()?)? {
        lst1 = (lst1).rest()?;
        lst2 = (lst2).rest()?;
    }
    if (lst1).is_empty() || (lst2).is_empty() {
        acc = lst1;
        return Ok(acc);
    }
    set2 = fromList(&lst2, hashFunc.clone(), keyEqFunc.clone())?;
    for mut k in &*lst1 {
        if !(contains(k.clone(), set2.clone())?) {
            acc = metamodelica::cons(k.clone(), acc);
        }
    }
    Ok(acc)
}

pub fn equal_list<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: &metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
    mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>,
    mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<bool> {
    let mut b: bool = false;
    let mut set1: metamodelica::Ref<UnorderedSet<T>> = fromList(inList1, hashFunc.clone(), keyEqFunc.clone())?;
    let mut set2: metamodelica::Ref<UnorderedSet<T>> = fromList(inList2, hashFunc.clone(), keyEqFunc.clone())?;
    if Mutable::access(set1.size.clone()) != Mutable::access(set2.size.clone()) {
        return Ok(b);
    }
    for mut k in &**inList1 {
        if !(contains(k.clone(), set2.clone())?) {
            return Ok(b);
        }
    }
    b = true;
    Ok(b)
}

pub fn difference<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set1: metamodelica::Ref<UnorderedSet<T>>,
    mut set2: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    let mut acc: metamodelica::List<T> = metamodelica::nil();
    if !(isEmpty(set1.clone())) {
        let __range0 = Mutable::access(set1.buckets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut b in __range0 {
            for mut k in &*b {
                if !(contains(k.clone(), set2.clone())?) {
                    acc = metamodelica::cons(k.clone(), acc);
                }
            }
        }
    }
    set = fromList(&acc, set1.hashFn.clone(), set1.eqFn.clone())?;
    Ok(set)
}

pub fn sym_difference<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set1: metamodelica::Ref<UnorderedSet<T>>,
    mut set2: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<metamodelica::Ref<UnorderedSet<T>>> {
    let mut set: metamodelica::Ref<UnorderedSet<T>>;
    let mut acc: metamodelica::List<T> = metamodelica::nil();
    if !(isEmpty(set1.clone())) {
        let __range0 = Mutable::access(set1.buckets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut b in __range0 {
            for mut k in &*b {
                if !(contains(k.clone(), set2.clone())?) {
                    acc = metamodelica::cons(k.clone(), acc);
                }
            }
        }
    }
    if !(isEmpty(set2.clone())) {
        let __range1 = Mutable::access(set2.buckets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut b in __range1 {
            for mut k in &*b {
                if !(contains(k.clone(), set1.clone())?) {
                    acc = metamodelica::cons(k.clone(), acc);
                }
            }
        }
    }
    set = fromList(&acc, set1.hashFn.clone(), set1.eqFn.clone())?;
    Ok(set)
}

pub fn isDisjoint<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut set1: metamodelica::Ref<UnorderedSet<T>>,
    mut set2: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<bool> {
    let mut b: bool = true;
    let mut set_small: metamodelica::Ref<UnorderedSet<T>>;
    let mut set_big: metamodelica::Ref<UnorderedSet<T>>;
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set_small = set2;
        set_big = set1;
    } else {
        set_small = set1;
        set_big = set2;
    }
    if !(isEmpty(set_small.clone())) {
        let __range0 = Mutable::access(set_small.buckets.clone())
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut buckets in __range0 {
            for mut k in &*buckets {
                if contains(k.clone(), set_big.clone())? {
                    b = false;
                    return Ok(b);
                }
            }
        }
    }
    Ok(b)
}

fn find<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<(Option<T>, i32)> {
    let mut outKey: Option<T> = None;
    let mut hash: i32;
    let mut hashfn: Hash<T> = set.hashFn.clone();
    let mut eqfn: KeyEq<T> = set.eqFn.clone();
    let mut buckets: metamodelica::Array<metamodelica::List<T>> = Mutable::access(set.buckets.clone());
    let mut bucket: metamodelica::List<T>;
    hash = intMod(hashfn(key.clone())?, metamodelica::arrayLength(buckets.clone()));
    bucket = metamodelica::arrayGet(buckets.clone(), hash + 1)?;
    for mut k in &*bucket {
        if eqfn(k.clone(), key.clone())? {
            outKey = Some(k.clone());
            break;
        }
    }
    Ok((outKey, hash))
}

fn addKey<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: T,
    mut hash: i32,
    mut set: metamodelica::Ref<UnorderedSet<T>>,
) -> Result<()> {
    let mut buckets: metamodelica::Array<metamodelica::List<T>>;
    let mut h: i32;
    let mut hashfn: Hash<T>;
    if loadFactor(set.clone())? > metamodelica::OrderedFloat((1) as f64) {
        rehash(set.clone())?;
        hashfn = set.hashFn.clone();
        buckets = Mutable::access(set.buckets.clone());
        h = intMod(hashfn(key.clone())?, metamodelica::arrayLength(buckets.clone()));
    } else {
        buckets = Mutable::access(set.buckets.clone());
        h = hash;
    }
    metamodelica::arrayUpdate(
        buckets.clone(),
        h + 1,
        metamodelica::cons(key, metamodelica::arrayGet(buckets.clone(), h + 1)?),
    )?;
    Mutable::update(set.size.clone(), Mutable::access(set.size.clone()) + 1);
    Ok(())
}

fn extractFromLst<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: &metamodelica::List<metamodelica::Ref<UnorderedSet<T>>>,
    mut func: &dyn ::std::ops::Fn(i32, i32) -> Result<bool>,
) -> Result<(
    metamodelica::Ref<UnorderedSet<T>>,
    metamodelica::List<metamodelica::Ref<UnorderedSet<T>>>,
)> {
    type size_compare = std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>;

    let mut single: metamodelica::Ref<UnorderedSet<T>>;
    let mut rest: metamodelica::List<metamodelica::Ref<UnorderedSet<T>>> = metamodelica::nil();
    let mut size: i32;
    let mut tmp_lst: metamodelica::List<metamodelica::Ref<UnorderedSet<T>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*lst)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    single = metamodelica::Own::own(__pa0);
    tmp_lst = metamodelica::Own::own(__pa1);
    size = Mutable::access(single.size.clone());
    for mut tmp in &*tmp_lst {
        if func(Mutable::access(tmp.size.clone()), size)? {
            size = Mutable::access(tmp.size.clone());
            rest = metamodelica::cons(single, rest);
            single = tmp.clone();
        } else {
            rest = metamodelica::cons(tmp.clone(), rest);
        }
    }
    Ok((single, rest))
}
