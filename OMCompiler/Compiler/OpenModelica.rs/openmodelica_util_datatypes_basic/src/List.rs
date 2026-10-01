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

use crate::Array;
use crate::DoubleEnded;
use crate::GCExt;

// these styles can be used with List.toString() to get predefined behaviour. Use List.toStringCustom for full control.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Style {
    NONE = 1,
    FLAT = 2,
    FLAT_BRACKETS = 3,
    FLAT_CURLY = 4,
    FLAT_CURLY_SHORT = 5,
    NEWLINE = 6,
    NEWLINE_INDENT = 7,
    NEWLINE_TAB = 8,
}
impl PartialOrd for Style {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Style {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Style {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub fn create<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inElement: T) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T> = list![inElement.clone()];
    outList
}

pub fn fill<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElement: T,
    mut inCount: i32,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut i: i32 = 0;
    while i < inCount {
        outList = metamodelica::cons(inElement.clone(), outList);
        i = i + 1;
    }
    outList
}

pub fn repeat<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElement: metamodelica::List<T>,
    mut inCount: i32,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut i: i32 = 0;
    while i < inCount {
        outList = listAppend(inElement.clone(), outList);
        i = i + 1;
    }
    outList
}

pub fn intRange(mut inStop: i32) -> metamodelica::List<i32> {
    let mut outRange: metamodelica::List<i32> = metamodelica::nil();
    let mut i: i32 = inStop;
    while i > 0 {
        outRange = metamodelica::cons(i, outRange);
        i = i - 1;
    }
    outRange
}

pub fn intRange2(mut inStart: i32, mut inStop: i32) -> metamodelica::List<i32> {
    let mut outRange: metamodelica::List<i32> = metamodelica::nil();
    let mut i: i32 = inStop;
    if inStart < inStop {
        while i >= inStart {
            outRange = metamodelica::cons(i, outRange);
            i = i - 1;
        }
    } else {
        while i <= inStart {
            outRange = metamodelica::cons(i, outRange);
            i = i + 1;
        }
    }
    outRange
}

pub fn intRange3(mut inStart: i32, mut inStep: i32, mut inStop: i32) -> Result<metamodelica::List<i32>> {
    let mut outRange: metamodelica::List<i32>;
    if inStep == 0 {
        return Err("fail");
    }
    outRange = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut i in ({
            let __s = inStart;
            let __e = inStop;
            let __step = inStep;
            (0i32..)
                .map(move |__k| __s + __k * __step)
                .take_while(move |&__v| __step != 0 && (if __step > 0 { __v <= __e } else { __v >= __e }))
        })
        .into_iter()
        {
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outRange)
}

pub fn fromOption<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inElement: Option<T>) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T>;
    outList = (match inElement {
        Some(mut e) => {
            list![e]
        }
        _ => metamodelica::nil(),
    });
    outList
}

pub fn isEqual<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inList1: metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
    mut inEqualLength: bool,
) -> Result<bool> {
    let mut outIsEqual: bool;
    let mut rest1: metamodelica::List<T> = inList1;
    let mut rest2: metamodelica::List<T> = inList2;
    let mut e1: T;
    let mut e2: T;
    while !((rest1).is_empty() || (rest2).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        rest1 = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa2);
        rest2 = metamodelica::Own::own(__pa3);
        if !(e1 == e2) {
            outIsEqual = false;
            return Ok(outIsEqual);
        }
    }
    outIsEqual = if ((rest1).is_empty() && (rest2).is_empty()) {
        true
    } else {
        !(inEqualLength)
    };
    Ok(outIsEqual)
}

pub fn isEqualOnTrue<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inCompFunc: &dyn ::std::ops::Fn(T1, T2) -> Result<bool>,
) -> Result<bool> {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outIsEqual: bool;
    let mut rest1: metamodelica::List<T1> = inList1;
    let mut rest2: metamodelica::List<T2> = inList2;
    let mut e1: T1;
    let mut e2: T2;
    while !((rest1).is_empty() || (rest2).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        rest1 = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa2);
        rest2 = metamodelica::Own::own(__pa3);
        if !(inCompFunc(e1, e2)?) {
            outIsEqual = false;
            return Ok(outIsEqual);
        }
    }
    outIsEqual = (rest1).is_empty() && (rest2).is_empty();
    Ok(outIsEqual)
}

pub fn allEqual<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<bool> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outAllEqual: bool = true;
    let mut e1: T;
    let mut rest: metamodelica::List<T>;
    if !((inList).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inList)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        for mut e in &*rest {
            if !(inCompFunc(e1.clone(), e.clone())?) {
                outAllEqual = false;
                return Ok(outAllEqual);
            }
        }
    }
    Ok(outAllEqual)
}

pub fn compareLength<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut list1: metamodelica::List<T1>,
    mut list2: metamodelica::List<T2>,
) -> Result<i32> {
    let mut res: i32;
    let mut rest1: metamodelica::List<T1> = list1;
    let mut rest2: metamodelica::List<T2> = list2;
    while !((rest1).is_empty() || (rest2).is_empty()) {
        rest1 = (rest1).rest()?;
        rest2 = (rest2).rest()?;
    }
    res = if ((rest1).is_empty()) {
        if ((rest2).is_empty()) { 0 } else { -1 }
    } else {
        1
    };
    Ok(res)
}

pub fn compare<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(
    mut list1: metamodelica::List<T1>,
    mut list2: metamodelica::List<T2>,
    mut compareFn: &dyn ::std::ops::Fn(T1, T2) -> Result<i32>,
) -> Result<i32> {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>;

    let mut res: i32;
    let mut e2: T2;
    let mut rest_e2: metamodelica::List<T2>;
    res = compareLength(list1.clone(), list2.clone())?;
    if res != 0 {
        return Ok(res);
    }
    rest_e2 = list2;
    for mut e1 in &*list1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_e2 = metamodelica::Own::own(__pa1);
        res = compareFn(e1.clone(), e2)?;
        if res != 0 {
            return Ok(res);
        }
    }
    Ok(res)
}

pub(crate) fn isPrefixOnTrue<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inCompFunc: &dyn ::std::ops::Fn(T1, T2) -> Result<bool>,
) -> Result<bool> {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outIsPrefix: bool;
    let mut rest1: metamodelica::List<T1> = inList1;
    let mut rest2: metamodelica::List<T2> = inList2;
    let mut e1: T1;
    let mut e2: T2;
    while !((rest1).is_empty()) {
        if (rest2).is_empty() {
            outIsPrefix = false;
            return Ok(outIsPrefix);
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        rest1 = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa2);
        rest2 = metamodelica::Own::own(__pa3);
        if !(inCompFunc(e1, e2)?) {
            outIsPrefix = false;
            return Ok(outIsPrefix);
        }
    }
    outIsPrefix = true;
    Ok(outIsPrefix)
}

pub fn consr<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inElement: T,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T>;
    outList = metamodelica::cons(inElement, inList);
    outList
}

pub fn consOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCondition: bool,
    mut inElement: T,
    mut inList: metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T>;
    outList = if (inCondition) {
        metamodelica::cons(inElement, inList)
    } else {
        inList
    };
    outList
}

pub fn consOption<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElement: Option<T>,
    mut inList: metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T>;
    outList = (match inElement {
        Some(mut e) => metamodelica::cons(e, inList),
        _ => inList,
    });
    outList
}

pub fn consN<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut size: i32,
    mut inElement: T,
    mut inList: metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut inList: metamodelica::List<T> = inList;
    for mut i in 1..=size {
        inList = metamodelica::cons(inElement.clone(), inList);
    }
    inList
}

pub fn append_reverse<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: &metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T> = inList2;
    for mut e in &**inList1 {
        outList = metamodelica::cons(e.clone(), outList);
    }
    outList
}

pub fn appendElt<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElement: T,
    mut inList: metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T>;
    outList = listAppend(inList, list![inElement]);
    outList
}

pub fn appendLastList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inListList: &metamodelica::List<metamodelica::List<T>>,
    mut inList: metamodelica::List<T>,
) -> Result<metamodelica::List<metamodelica::List<T>>> {
    let mut outListList: metamodelica::List<metamodelica::List<T>>;
    outListList = ({
        let mut ol: metamodelica::List<metamodelica::List<T>> = metamodelica::nil();
        (::match_deref::match_deref! { match inListList {
            Deref @ metamodelica::ListNode::Nil => {
                list![inList]
            },
            Deref @ metamodelica::ListNode::Cons { head: l, tail: Deref @ metamodelica::ListNode::Nil } => {
                list![listAppend(l.clone(), inList)]
            },
            Deref @ metamodelica::ListNode::Cons { head: l, tail: ll } => {
                let mut l = (*l).clone();
                let mut ll = (*ll).clone();
                while !((ll).is_empty()) {
                    ol = metamodelica::cons(l.clone(), ol);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ll.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    l = metamodelica::Own::own(__pa0);
                    ll = metamodelica::Own::own(__pa1);
                }
                ol = metamodelica::cons(listAppend(l.clone(), inList), ol);
                ol = metamodelica::Dangerous::listReverseInPlace(ol);
                ol
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(outListList)
}

pub fn insert<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inN: i32,
    mut inElement: T,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    let mut lst1: metamodelica::List<T>;
    let mut lst2: metamodelica::List<T>;
    let true = (inN > 0) else {
        return Err("pattern mismatch");
    };
    (lst1, lst2) = splitr(inList, inN - 1)?;
    outList = append_reverse(&lst1, metamodelica::cons(inElement, lst2));
    Ok(outList)
}

pub fn insertListSorted<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = metamodelica::Dangerous::listReverseInPlace(insertListSorted1(
        inList,
        inList2,
        inCompFunc,
        &(metamodelica::nil()),
    )?);
    Ok(outList)
}

fn insertListSorted1<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
    mut inResultList: &metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outResultList: metamodelica::List<T>;
    let mut listRest: metamodelica::List<T>;
    let mut listRest2: metamodelica::List<T>;
    let mut tmpResultList: metamodelica::List<T>;
    let mut listHead: T;
    let mut listHead2: T;
    outResultList = (::match_deref::match_deref! { match (inList, inList2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => inResultList.clone(),
        (Deref @ metamodelica::ListNode::Nil, _) => append_reverse(inList2, inResultList.clone()),
        (_, Deref @ metamodelica::ListNode::Nil) => append_reverse(inList, inResultList.clone()),
        (Deref @ metamodelica::ListNode::Cons { head: __esc_listHead, tail: __esc_listRest }, Deref @ metamodelica::ListNode::Cons { head: __esc_listHead2, tail: __esc_listRest2 }) => {
            listHead = (*__esc_listHead).clone();
            listRest = (*__esc_listRest).clone();
            listHead2 = (*__esc_listHead2).clone();
            listRest2 = (*__esc_listRest2).clone();
            if inCompFunc(listHead.clone(), listHead2.clone())? {
                tmpResultList = metamodelica::cons(listHead.clone(), inResultList.clone());
                tmpResultList = insertListSorted1(metamodelica::AsArg::as_arg(&listRest), inList2, inCompFunc, &tmpResultList)?;
            } else {
                tmpResultList = metamodelica::cons(listHead2.clone(), inResultList.clone());
                tmpResultList = insertListSorted1(inList, metamodelica::AsArg::as_arg(&listRest2), inCompFunc, &tmpResultList)?;
            }
            tmpResultList
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outResultList)
}

pub fn set<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inN: i32,
    mut inElement: T,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    let mut lst1: metamodelica::List<T>;
    let mut lst2: metamodelica::List<T>;
    let true = (inN > 0) else {
        return Err("pattern mismatch");
    };
    (lst1, lst2) = splitr(inList, inN - 1)?;
    lst2 = restOrEmpty(lst2)?;
    outList = append_reverse(&lst1, metamodelica::cons(inElement, lst2));
    Ok(outList)
}

pub fn firstOrEmpty<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T>;
    outList = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Cons { head: e, tail: _ } => {
            list![e.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

pub fn second<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inList: &metamodelica::List<T>) -> Result<T> {
    let mut outSecond: T;
    let __pa0 = ::match_deref::match_deref! { match &((*inList)) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outSecond = metamodelica::Own::own(__pa0);
    Ok(outSecond)
}

pub fn last<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inList: &metamodelica::List<T>) -> Result<T> {
    let mut outLast: T;
    let mut rest: metamodelica::List<T>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inList)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outLast = metamodelica::Own::own(__pa0);
    rest = metamodelica::Own::own(__pa1);
    for mut e in &*rest {
        outLast = e.clone();
    }
    Ok(outLast)
}

pub fn lastListOrEmpty<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inListList: &metamodelica::List<metamodelica::List<T>>,
) -> metamodelica::List<T> {
    let mut outLastList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inListList {
        outLastList = e.clone();
    }
    outLastList
}

pub fn lastN<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inN: i32,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    let mut len: i32;
    let true = (inN >= 0) else {
        return Err("pattern mismatch");
    };
    len = ((inList).len() as i32);
    outList = stripN(inList, len - inN)?;
    Ok(outList)
}

pub fn trimToLength<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: metamodelica::List<T>,
    mut n: i32,
) -> Result<metamodelica::List<T>> {
    let mut lst: metamodelica::List<T> = lst;
    let mut len: i32;
    len = ((lst).len() as i32);
    for mut i in 1..=len - n {
        lst = (lst).rest()?;
    }
    Ok(lst)
}

pub fn restOrEmpty<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    outList = if ((inList).is_empty()) {
        inList
    } else {
        (inList).rest()?
    };
    Ok(outList)
}

pub fn getIndexFirst<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut index: i32,
    mut inList: &metamodelica::List<T>,
) -> Result<T> {
    let mut element: T;
    element = (inList).get(index)?;
    Ok(element)
}

pub fn getAtIndexLst<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: metamodelica::List<T>,
    mut positions: metamodelica::List<i32>,
    mut zeroBased: bool,
) -> Result<metamodelica::List<T>> {
    let mut olst: metamodelica::List<T>;
    let mut arr: metamodelica::Array<T> = metamodelica::arrayFromVec(lst.clone().into_iter().cloned().collect());
    let mut shift: i32 = if (zeroBased) { 1 } else { 0 };
    olst = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut pos in (positions).into_iter().cloned() {
            let __x = ({
                let __elt = (*metamodelica::index_checked(&arr.borrow(), pos.clone() + shift)?).clone();
                __elt
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(olst)
}

pub fn firstN<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut N: i32,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    outList = firstN_reverse(inList, N)?;
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub(crate) fn firstN_reverse<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut N: i32,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest: metamodelica::List<T>;
    let true = (N >= 0) else { return Err("pattern mismatch") };
    rest = inList;
    for mut i in 1..=N {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        outList = metamodelica::cons(e, outList);
    }
    Ok(outList)
}

pub fn stripLast<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    if (inList).is_empty() {
        outList = metamodelica::nil();
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(inList.reverse()) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        outList = metamodelica::Own::own(__pa0);
        outList = metamodelica::Dangerous::listReverseInPlace(outList);
    }
    Ok(outList)
}

pub fn stripN<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inN: i32,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = inList;
    let true = (inN >= 0) else {
        return Err("pattern mismatch");
    };
    for mut i in 1..=inN {
        let __pa0 = ::match_deref::match_deref! { match &(outList) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        outList = metamodelica::Own::own(__pa0);
    }
    Ok(outList)
}

pub fn heapSortIntList(mut lst: metamodelica::List<i32>) -> Result<metamodelica::List<i32>> {
    let mut lst: metamodelica::List<i32> = lst;
    lst = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::ListNode::Nil => lst,
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => lst,
        _ => Array::heapSort(metamodelica::arrayFromVec(lst.into_iter().cloned().collect()))?.borrow().iter().cloned().collect::<metamodelica::List<_>>(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lst)
}

pub fn sort<T: Clone + 'static + metamodelica::gc::MMTrace>(
    inList: metamodelica::List<T>,
    inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<T>> {
    fn sort_slice<T: Clone>(v: &[T], ix: &[usize], comp: &dyn Fn(T, T) -> Result<bool>) -> Result<Vec<usize>> {
        let n = ix.len();
        if n < 2 {
            return Ok(ix.to_vec());
        }
        if n == 2 {
            return Ok(if comp(v[ix[1]].clone(), v[ix[0]].clone())? {
                ix.to_vec()
            } else {
                vec![ix[1], ix[0]]
            });
        }
        let (l, r) = ix.split_at(n / 2);
        let left = sort_slice(v, l, comp)?;
        let right = sort_slice(v, r, comp)?;
        let mut res = Vec::with_capacity(n);
        let (mut i, mut j) = (0, 0);
        while i < left.len() && j < right.len() {
            if comp(v[right[j]].clone(), v[left[i]].clone())? {
                res.push(left[i]);
                i += 1;
            } else {
                res.push(right[j]);
                j += 1;
            }
        }
        res.extend_from_slice(&left[i..]);
        res.extend_from_slice(&right[j..]);
        Ok(res)
    }
    let v: Vec<T> = (&*inList).into_iter().cloned().collect();
    if v.len() < 2 || (v.len() == 2 && inCompFunc(v[1].clone(), v[0].clone())?) {
        return Ok(inList);
    }
    let ix: Vec<usize> = (0..v.len()).collect();
    let sorted = sort_slice(&v, &ix, &*inCompFunc)?;
    let mut slots: Vec<Option<T>> = v.into_iter().map(Some).collect();
    let mut out = metamodelica::nil();
    for &k in sorted.iter().rev() {
        out = metamodelica::cons(slots[k].take().unwrap(), out);
    }
    Ok(out)
}
pub fn sortedDuplicates<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outDuplicates: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if !((rest).is_empty()) && inCompFunc(e.clone(), (rest).head().cloned()?)? {
            outDuplicates = metamodelica::cons(e, outDuplicates);
        }
    }
    outDuplicates = metamodelica::Dangerous::listReverseInPlace(outDuplicates);
    Ok(outDuplicates)
}

pub fn sortedListAllUnique<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: metamodelica::List<T>,
    mut compareFn: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<bool> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut allUnique: bool = false;
    let mut rest: metamodelica::List<T> = lst;
    while !((rest).is_empty()) {
        rest = (::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
                metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: e1, tail: __esc_rest @ Deref @ metamodelica::ListNode::Cons { head: e2, tail: _ } } => {
                rest = (*__esc_rest).clone();
                if compareFn(e1.clone(), e2.clone())? {
                    return Ok(allUnique);
                }
                rest.clone()
            },
            _ => return Err("match: no arm matched"),
        } });
    }
    allUnique = true;
    Ok(allUnique)
}

pub fn sortedUnique<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUniqueElements: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if (rest).is_empty() || !(inCompFunc(e.clone(), (rest).head().cloned()?)?) {
            outUniqueElements = metamodelica::cons(e, outUniqueElements);
        }
    }
    outUniqueElements = metamodelica::Dangerous::listReverseInPlace(outUniqueElements);
    Ok(outUniqueElements)
}

pub(crate) fn sortedUniqueAndDuplicates<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUniqueElements: metamodelica::List<T> = metamodelica::nil();
    let mut outDuplicateElements: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if !((rest).is_empty()) && inCompFunc(e.clone(), (rest).head().cloned()?)? {
            outDuplicateElements = metamodelica::cons(e, outDuplicateElements);
        } else {
            outUniqueElements = metamodelica::cons(e, outUniqueElements);
        }
    }
    outUniqueElements = metamodelica::Dangerous::listReverseInPlace(outUniqueElements);
    outDuplicateElements = metamodelica::Dangerous::listReverseInPlace(outDuplicateElements);
    Ok((outUniqueElements, outDuplicateElements))
}

pub fn sortedUniqueOnlyDuplicates<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outDuplicateElements: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if !((rest).is_empty()) && inCompFunc(e.clone(), (rest).head().cloned()?)? {
            outDuplicateElements = metamodelica::cons(e, outDuplicateElements);
        }
    }
    outDuplicateElements = metamodelica::Dangerous::listReverseInPlace(outDuplicateElements);
    Ok(outDuplicateElements)
}

fn merge<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inLeft: metamodelica::List<T>,
    mut inRight: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
    mut acc: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    let mut left: metamodelica::List<T> = inLeft;
    let mut right: metamodelica::List<T> = inRight;
    let mut res: metamodelica::List<T> = acc;
    let mut el: T;
    while !((left).is_empty() || (right).is_empty()) {
        if inCompFunc((right).head().cloned()?, (left).head().cloned()?)? {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(left) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            el = metamodelica::Own::own(__pa0);
            left = metamodelica::Own::own(__pa1);
        } else {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(right) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            el = metamodelica::Own::own(__pa2);
            right = metamodelica::Own::own(__pa3);
        }
        res = metamodelica::cons(el, res);
    }
    outList = if ((left).is_empty()) {
        if ((right).is_empty()) {
            metamodelica::Dangerous::listReverseInPlace(res)
        } else {
            append_reverse(&res, right)
        }
    } else {
        append_reverse(&res, left)
    };
    Ok(outList)
}

pub fn mergeSorted<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut l1: metamodelica::List<T>;
    let mut l2: metamodelica::List<T>;
    let mut e1: T;
    let mut e2: T;
    l1 = inList1;
    l2 = inList2;
    while !((l1).is_empty()) && !((l2).is_empty()) {
        let __pa0 = ::match_deref::match_deref! { match &(l1.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        let __pa1 = ::match_deref::match_deref! { match &(l2.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa1);
        if inCompFunc(e1.clone(), e2.clone())? {
            outList = metamodelica::cons(e1, outList);
            let __pa2 = ::match_deref::match_deref! { match &(l1) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa2 } => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            l1 = metamodelica::Own::own(__pa2);
        } else {
            outList = metamodelica::cons(e2, outList);
            let __pa3 = ::match_deref::match_deref! { match &(l2) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa3 } => __pa3.clone(),
                _ => return Err("pattern mismatch"),
            } };
            l2 = metamodelica::Own::own(__pa3);
        }
    }
    l1 = if ((l1).is_empty()) { l2 } else { l1 };
    outList = append_reverse(&outList, l1);
    Ok(outList)
}

pub(crate) fn countingSort(mut inList: metamodelica::List<i32>, mut N: i32) -> Result<metamodelica::List<i32>> {
    let mut outSorted: metamodelica::List<i32> = metamodelica::nil();
    let mut a1: metamodelica::Array<i32>;
    if !(hasSeveralElements(&inList)) {
        outSorted = inList;
        return Ok(outSorted);
    }
    a1 = arrayCreate(N, 0);
    for mut v in &*inList {
        {
            let __cell0 = intAdd(
                ({
                    let __elt = (*metamodelica::index_checked(&a1.borrow(), v.clone())?).clone();
                    __elt
                }),
                1,
            );
            let __idx0 = v.clone();
            *metamodelica::index_mut_checked(&mut a1.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    for mut v in ({
        let __s = N;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        let __range1 = 1..=({
            let __elt = (*metamodelica::index_checked(&a1.borrow(), v)?).clone();
            __elt
        });
        for mut c in __range1 {
            outSorted = metamodelica::cons(v, outSorted);
        }
    }
    GCExt::free(a1.clone());
    Ok(outSorted)
}

pub fn unique<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inList: &metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if !(listMember(e.clone(), outList.clone())) {
            outList = metamodelica::cons(e.clone(), outList);
        }
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    outList
}

pub fn uniqueIntN(mut inList: &metamodelica::List<i32>, mut inN: i32) -> Result<metamodelica::List<i32>> {
    let mut outList: metamodelica::List<i32> = metamodelica::nil();
    let mut arr: metamodelica::Array<bool>;
    arr = arrayCreate(inN, true);
    for mut i in &**inList {
        if metamodelica::arrayGet(arr.clone(), i.clone())? {
            outList = metamodelica::cons(i.clone(), outList);
        }
        metamodelica::arrayUpdate(arr.clone(), i.clone(), false)?;
    }
    GCExt::free(arr.clone());
    Ok(outList)
}

pub fn uniqueOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if !(isMemberOnTrue(e.clone(), &outList, inCompFunc)?) {
            outList = metamodelica::cons(e.clone(), outList);
        }
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub fn split<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPosition: i32,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    let mut outList1: metamodelica::List<T>;
    let mut outList2: metamodelica::List<T>;
    let mut pos: i32;
    let mut l1: metamodelica::List<T> = metamodelica::nil();
    let mut l2: metamodelica::List<T> = inList;
    let mut e: T;
    let true = (inPosition >= 0) else {
        return Err("pattern mismatch");
    };
    pos = inPosition;
    for mut i in 1..=pos {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(l2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        l2 = metamodelica::Own::own(__pa1);
        l1 = metamodelica::cons(e, l1);
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(l1);
    outList2 = l2;
    Ok((outList1, outList2))
}

pub fn splitr<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPosition: i32,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    let mut outList1: metamodelica::List<T>;
    let mut outList2: metamodelica::List<T>;
    let mut pos: i32;
    let mut l1: metamodelica::List<T> = metamodelica::nil();
    let mut l2: metamodelica::List<T> = inList;
    let mut e: T;
    let true = (inPosition >= 0) else {
        return Err("pattern mismatch");
    };
    pos = inPosition;
    for mut i in 1..=pos {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(l2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        l2 = metamodelica::Own::own(__pa1);
        l1 = metamodelica::cons(e, l1);
    }
    outList1 = l1;
    outList2 = l2;
    Ok((outList1, outList2))
}

pub fn splitOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type PredicateFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outTrueList: metamodelica::List<T> = metamodelica::nil();
    let mut outFalseList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if inFunc(e.clone())? {
            outTrueList = metamodelica::cons(e.clone(), outTrueList);
        } else {
            outFalseList = metamodelica::cons(e.clone(), outFalseList);
        }
    }
    outTrueList = metamodelica::Dangerous::listReverseInPlace(outTrueList);
    outFalseList = metamodelica::Dangerous::listReverseInPlace(outFalseList);
    Ok((outTrueList, outFalseList))
}

pub fn split1OnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<bool>,
    mut inArg1: ArgT1,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type PredicateFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outTrueList: metamodelica::List<T> = metamodelica::nil();
    let mut outFalseList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if inFunc(e.clone(), inArg1.clone())? {
            outTrueList = metamodelica::cons(e.clone(), outTrueList);
        } else {
            outFalseList = metamodelica::cons(e.clone(), outFalseList);
        }
    }
    outTrueList = metamodelica::Dangerous::listReverseInPlace(outTrueList);
    outFalseList = metamodelica::Dangerous::listReverseInPlace(outFalseList);
    Ok((outTrueList, outFalseList))
}

pub fn split2OnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type PredicateFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool> + 'static>;

    let mut outTrueList: metamodelica::List<T> = metamodelica::nil();
    let mut outFalseList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if inFunc(e.clone(), inArg1.clone(), inArg2.clone())? {
            outTrueList = metamodelica::cons(e.clone(), outTrueList);
        } else {
            outFalseList = metamodelica::cons(e.clone(), outFalseList);
        }
    }
    outTrueList = metamodelica::Dangerous::listReverseInPlace(outTrueList);
    outFalseList = metamodelica::Dangerous::listReverseInPlace(outFalseList);
    Ok((outTrueList, outFalseList))
}

pub fn splitOnFirstMatch<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList1: metamodelica::List<T> = metamodelica::nil();
    let mut outList2: metamodelica::List<T> = inList;
    let mut e: T;
    while !((outList2).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outList2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        outList2 = metamodelica::Own::own(__pa1);
        if inFunc(e.clone())? {
            outList2 = metamodelica::cons(e, outList2);
            break;
        }
        outList1 = metamodelica::cons(e, outList1);
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    Ok((outList1, outList2))
}

pub fn splitLast<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
) -> Result<(T, metamodelica::List<T>)> {
    let mut outLast: T;
    let mut outRest: metamodelica::List<T>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inList.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outLast = metamodelica::Own::own(__pa0);
    outRest = metamodelica::Own::own(__pa1);
    outRest = metamodelica::Dangerous::listReverseInPlace(outRest);
    Ok((outLast, outRest))
}

pub fn splitEqualParts<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inParts: i32,
) -> Result<metamodelica::List<metamodelica::List<T>>> {
    let mut outParts: metamodelica::List<metamodelica::List<T>>;
    let mut length: i32;
    if inParts == 0 {
        outParts = metamodelica::nil();
    } else {
        length = ((inList).len() as i32);
        let 0 = (intMod(length, inParts)) else {
            return Err("pattern mismatch");
        };
        outParts = partition(inList, intDiv(length, inParts))?;
    }
    Ok(outParts)
}

pub fn splitOnBoolList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inBools: metamodelica::List<bool>,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    let mut outTrueList: metamodelica::List<T> = metamodelica::nil();
    let mut outFalseList: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest_e: metamodelica::List<T> = inList;
    let mut b: bool;
    let mut rest_b: metamodelica::List<bool> = inBools;
    while !((rest_e).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest_e = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_b) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        b = metamodelica::Own::own(__pa2);
        rest_b = metamodelica::Own::own(__pa3);
        if b {
            outTrueList = metamodelica::cons(e, outTrueList);
        } else if true
        /* isPresent not implemented in Rust */
        {
            outFalseList = metamodelica::cons(e, outFalseList);
        }
    }
    outTrueList = metamodelica::Dangerous::listReverseInPlace(outTrueList);
    outFalseList = metamodelica::Dangerous::listReverseInPlace(outFalseList);
    Ok((outTrueList, outFalseList))
}

pub fn partition<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPartitionLength: i32,
) -> Result<metamodelica::List<metamodelica::List<T>>> {
    let mut outPartitions: metamodelica::List<metamodelica::List<T>> = metamodelica::nil();
    let mut lst: metamodelica::List<T> = inList.clone();
    let mut part: metamodelica::List<T>;
    let mut length: i32;
    let true = (inPartitionLength > 0) else {
        return Err("pattern mismatch");
    };
    if (inList).is_empty() {
        return Ok(outPartitions);
    }
    length = ((inList).len() as i32);
    if inPartitionLength >= length {
        outPartitions = list![inList];
        return Ok(outPartitions);
    }
    for mut i in 1..=intDiv(length, inPartitionLength) {
        (part, lst) = split(lst, inPartitionLength)?;
        outPartitions = metamodelica::cons(part, outPartitions);
    }
    if !((lst).is_empty()) {
        outPartitions = metamodelica::cons(lst, outPartitions);
    }
    outPartitions = metamodelica::Dangerous::listReverseInPlace(outPartitions);
    Ok(outPartitions)
}

pub fn balancedPartition<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: metamodelica::List<T>,
    mut maxLength: i32,
) -> Result<metamodelica::List<metamodelica::List<T>>> {
    let mut outPartitions: metamodelica::List<metamodelica::List<T>>;
    let mut length: i32;
    let mut n: i32;
    let true = (maxLength > 0) else {
        return Err("pattern mismatch");
    };
    if (lst).is_empty() {
        outPartitions = metamodelica::nil();
        return Ok(outPartitions);
    }
    length = ((lst).len() as i32);
    n = intDiv(length - 1, maxLength) + 1;
    outPartitions = partition(lst, intDiv(length - 1, n) + 1)?;
    Ok(outPartitions)
}

pub fn sublist<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inOffset: i32,
    mut inLength: i32,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    let true = (inOffset > 0) else {
        return Err("pattern mismatch");
    };
    let true = (inLength >= 0) else {
        return Err("pattern mismatch");
    };
    for mut i in 2..=inOffset {
        let __pa0 = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        rest = metamodelica::Own::own(__pa0);
    }
    for mut i in 1..=inLength {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa1);
        rest = metamodelica::Own::own(__pa2);
        outList = metamodelica::cons(e, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub fn transposeList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<metamodelica::List<T>>,
) -> Result<metamodelica::List<metamodelica::List<T>>> {
    let mut outList: metamodelica::List<metamodelica::List<T>> = metamodelica::nil();
    let mut arr: metamodelica::Array<metamodelica::Array<T>>;
    let mut new_row: metamodelica::List<T>;
    let mut c_len: i32;
    let mut r_len: i32;
    if (inList).is_empty() {
        return Ok(outList);
    }
    arr = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut lst in (inList).into_iter().cloned() {
                let __x = metamodelica::arrayFromVec(lst.clone().into_iter().cloned().collect());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    c_len = metamodelica::arrayLength(arr.clone());
    r_len = metamodelica::arrayLength(metamodelica::arrayGet(arr.clone(), 1)?);
    for mut i in ({
        let __s = r_len;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        new_row = metamodelica::nil();
        for mut j in ({
            let __s = c_len;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            new_row = metamodelica::cons(
                metamodelica::Dangerous::arrayGetNoBoundsChecking(metamodelica::arrayGet(arr.clone(), j)?, i),
                new_row,
            );
        }
        outList = metamodelica::cons(new_row, outList);
    }
    Ok(outList)
}

pub fn listArrayReverse<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inLst: metamodelica::List<T>,
) -> Result<metamodelica::Array<T>> {
    let mut outArr: metamodelica::Array<T>;
    let mut len: i32;
    let mut defaultValue: T;
    if (inLst).is_empty() {
        outArr = metamodelica::arrayFromVec(inLst.into_iter().cloned().collect());
        return Ok(outArr);
    }
    len = ((inLst).len() as i32);
    let __pa0 = ::match_deref::match_deref! { match &(inLst.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    defaultValue = metamodelica::Own::own(__pa0);
    outArr = metamodelica::arrayCreate(len, defaultValue);
    for mut e in &*inLst {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArr.clone(), len, e.clone()) };
        len = len - 1;
    }
    Ok(outArr)
}

pub fn setEqualOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: &metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<bool> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outIsEqual: bool;
    let mut lst: metamodelica::List<T>;
    let mut lst_size: i32;
    lst = intersectionOnTrue(inList1, inList2, inCompFunc)?;
    lst_size = ((lst).len() as i32);
    outIsEqual = intEq(lst_size, ((inList1).len() as i32)) && intEq(lst_size, ((inList2).len() as i32));
    Ok(outIsEqual)
}

fn addPos(
    mut inList: &metamodelica::List<i32>,
    mut inArray: metamodelica::Array<i32>,
    mut inIndex: i32,
) -> Result<metamodelica::Array<i32>> {
    let mut outArray: metamodelica::Array<i32>;
    for mut i in &**inList {
        metamodelica::arrayUpdate(
            inArray.clone(),
            i.clone(),
            intAdd(metamodelica::arrayGet(inArray.clone(), i.clone())?, inIndex),
        )?;
    }
    outArray = inArray.clone();
    Ok(outArray)
}

pub fn intersectionOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: &metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outIntersection: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList1 {
        if isMemberOnTrue(e.clone(), inList2, inCompFunc)? {
            outIntersection = metamodelica::cons(e.clone(), outIntersection);
        }
    }
    outIntersection = metamodelica::Dangerous::listReverseInPlace(outIntersection);
    Ok(outIntersection)
}

pub fn intersection1OnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>, metamodelica::List<T>)> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outIntersection: metamodelica::List<T> = metamodelica::nil();
    let mut outList1Rest: metamodelica::List<T> = metamodelica::nil();
    let mut outList2Rest: metamodelica::List<T> = inList2.clone();
    let mut lst1: metamodelica::List<T> = inList1.clone();
    let mut lst2: metamodelica::List<T> = inList2.clone();
    if (inList1).is_empty() {
        return Ok((outIntersection, outList1Rest, outList2Rest));
    }
    if (inList2).is_empty() {
        outList1Rest = inList1;
        return Ok((outIntersection, outList1Rest, outList2Rest));
    }
    while !((lst1).is_empty() || (lst2).is_empty()) {
        if !(inCompFunc((lst1).head().cloned()?, (lst2).head().cloned()?)?) {
            break;
        }
        outIntersection = metamodelica::cons((lst1).head().cloned()?, outIntersection);
        lst1 = (lst1).rest()?;
        lst2 = (lst2).rest()?;
    }
    for mut e in &*lst1 {
        if isMemberOnTrue(e.clone(), &inList2, inCompFunc)? {
            outIntersection = metamodelica::cons(e.clone(), outIntersection);
        } else if true
        /* isPresent not implemented in Rust */
        {
            outList1Rest = metamodelica::cons(e.clone(), outList1Rest);
        }
    }
    outIntersection = metamodelica::Dangerous::listReverseInPlace(outIntersection);
    outList1Rest = if (true/* isPresent not implemented in Rust */) {
        metamodelica::Dangerous::listReverseInPlace(outList1Rest)
    } else {
        metamodelica::nil()
    };
    outList2Rest = if (true/* isPresent not implemented in Rust */) {
        setDifferenceOnTrue(inList2, &outIntersection, inCompFunc)?
    } else {
        metamodelica::nil()
    };
    Ok((outIntersection, outList1Rest, outList2Rest))
}

pub fn setDifferenceIntN(
    mut inList1: &metamodelica::List<i32>,
    mut inList2: &metamodelica::List<i32>,
    mut inN: i32,
) -> Result<metamodelica::List<i32>> {
    let mut outDifference: metamodelica::List<i32> = metamodelica::nil();
    let mut a: metamodelica::Array<i32>;
    if inN > 0 {
        a = arrayCreate(inN, 0);
        a = addPos(inList1, a.clone(), 1)?;
        a = addPos(inList2, a.clone(), 1)?;
        for mut i in ({
            let __s = inN;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            if metamodelica::arrayGet(a.clone(), i)? == 1 {
                outDifference = metamodelica::cons(i, outDifference);
            }
        }
        GCExt::free(a.clone());
    }
    Ok(outDifference)
}

pub fn setDifferenceOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outDifference: metamodelica::List<T> = inList1.clone();
    if (inList1).is_empty() {
        return Ok(outDifference);
    }
    for mut e in &**inList2 {
        (outDifference, _) = deleteMemberOnTrue(e.clone(), outDifference, inCompFunc)?;
    }
    Ok(outDifference)
}

pub fn setDifference<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inList1: metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    let mut outDifference: metamodelica::List<T> = inList1.clone();
    if (inList1).is_empty() {
        return Ok(outDifference);
    }
    for mut e in &**inList2 {
        (outDifference, _) = deleteMemberOnTrue(e.clone(), outDifference, &fnptr!(valueEq, _, _))?;
    }
    Ok(outDifference)
}

pub(crate) fn unionIntN(
    mut inList1: &metamodelica::List<i32>,
    mut inList2: &metamodelica::List<i32>,
    mut inN: i32,
) -> Result<metamodelica::List<i32>> {
    let mut outUnion: metamodelica::List<i32> = metamodelica::nil();
    let mut a: metamodelica::Array<i32>;
    if inN > 0 {
        a = arrayCreate(inN, 0);
        a = addPos(inList1, a.clone(), 1)?;
        a = addPos(inList2, a.clone(), 1)?;
        for mut i in ({
            let __s = inN;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            if metamodelica::arrayGet(a.clone(), i)? > 0 {
                outUnion = metamodelica::cons(i, outUnion);
            }
        }
        GCExt::free(a.clone());
    }
    Ok(outUnion)
}

pub fn unionElt<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inElement: T,
    mut inList: metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outList: metamodelica::List<T>;
    outList = consOnTrue(!(listMember(inElement.clone(), inList.clone())), inElement, inList);
    outList
}

pub fn unionEltOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElement: T,
    mut inList: metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = consOnTrue(
        !(isMemberOnTrue(inElement.clone(), &inList, inCompFunc)?),
        inElement,
        inList,
    );
    Ok(outList)
}

pub fn union<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inList1: &metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
) -> metamodelica::List<T> {
    let mut outUnion: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList1 {
        outUnion = unionElt(e.clone(), outUnion);
    }
    for mut e in &**inList2 {
        outUnion = unionElt(e.clone(), outUnion);
    }
    outUnion = metamodelica::Dangerous::listReverseInPlace(outUnion);
    outUnion
}

pub fn unionOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: &metamodelica::List<T>,
    mut inList2: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUnion: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList1 {
        outUnion = unionEltOnTrue(e.clone(), outUnion, inCompFunc)?;
    }
    for mut e in &**inList2 {
        outUnion = unionEltOnTrue(e.clone(), outUnion, inCompFunc)?;
    }
    outUnion = metamodelica::Dangerous::listReverseInPlace(outUnion);
    Ok(outUnion)
}

pub fn unionAppendListOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inUnion: metamodelica::List<T>,
    mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUnion: metamodelica::List<T>;
    outUnion = fold(
        inList,
        &({
            let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = inCompFunc.clone();
            move |__pe_a0, __pe_a1| unionEltOnTrue(__pe_a0, __pe_a1, &*__pe_b2)
        }),
        inUnion,
    )?;
    Ok(outUnion)
}

pub fn unionList<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inList: &metamodelica::List<metamodelica::List<T>>,
) -> Result<metamodelica::List<T>> {
    let mut outUnion: metamodelica::List<T>;
    outUnion = if ((inList).is_empty()) {
        metamodelica::nil()
    } else {
        reduce(inList, &move |__a0: _, __a1: _| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(union(&__a0, &__a1))
        })?
    };
    Ok(outUnion)
}

pub fn unionOnTrueList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<metamodelica::List<T>>,
    mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUnion: metamodelica::List<T>;
    outUnion = if ((inList).is_empty()) {
        metamodelica::nil()
    } else {
        reduce(
            inList,
            &({
                let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = inCompFunc.clone();
                move |__pe_a0, __pe_a1| unionOnTrue(&__pe_a0, &__pe_a1, &*__pe_b2)
            }),
        )?
    };
    Ok(outUnion)
}

pub fn map<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn mapArray<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inArray.clone()).borrow().iter() {
            let __x = inFunc(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn mapCheckReferenceEq<TI: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TI>,
) -> Result<metamodelica::List<TI>> {
    pub type MapFunc<TI: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TI> + 'static>;

    let mut outList: metamodelica::List<TI>;
    let mut delst: DoubleEnded::MutableList<TI>;
    let mut n: i32 = 0;
    let mut e1: TI;
    let mut savedElt: TI;
    for mut e in &*inList {
        e1 = inFunc(e.clone())?;
        if !(metamodelica::ReferenceEq::reference_eq(&(e.clone()), &(e1.clone()))) {
            savedElt = e1.clone();
            delst = DoubleEnded::empty(e1);
            for mut elt in &*inList {
                if n < 0 {
                    e1 = inFunc(elt.clone())?;
                } else {
                    e1 = if (n == 0) { savedElt.clone() } else { elt.clone() };
                }
                DoubleEnded::push_back(delst.clone(), e1)?;
                n = n - 1;
            }
            outList = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
            return Ok(outList);
        }
        n = n + 1;
    }
    outList = inList;
    Ok(outList)
}

pub fn mapReverse<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc
    });
    Ok(outList)
}

pub fn map_2<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO1: Clone + 'static + metamodelica::gc::MMTrace,
    TO2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2)>,
) -> Result<(metamodelica::List<TO1>, metamodelica::List<TO2>)> {
    pub type MapFunc<TI: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: metamodelica::List<TO1> = metamodelica::nil();
    let mut outList2: metamodelica::List<TO2> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    for mut e in &**inList {
        (e1, e2) = inFunc(e.clone())?;
        outList1 = metamodelica::cons(e1, outList1);
        if true
        /* isPresent not implemented in Rust */
        {
            outList2 = metamodelica::cons(e2, outList2);
        }
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    if true
    /* isPresent not implemented in Rust */
    {
        outList2 = metamodelica::Dangerous::listReverseInPlace(outList2);
    }
    Ok((outList1, outList2))
}

pub fn map_3<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO1: Clone + 'static + metamodelica::gc::MMTrace,
    TO2: Clone + 'static + metamodelica::gc::MMTrace,
    TO3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2, TO3)>,
) -> Result<(
    metamodelica::List<TO1>,
    metamodelica::List<TO2>,
    metamodelica::List<TO3>,
)> {
    pub type MapFunc<TI: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static, TO3: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2, TO3)> + 'static>;

    let mut outList1: metamodelica::List<TO1> = metamodelica::nil();
    let mut outList2: metamodelica::List<TO2> = metamodelica::nil();
    let mut outList3: metamodelica::List<TO3> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    let mut e3: TO3;
    for mut e in &**inList {
        (e1, e2, e3) = inFunc(e.clone())?;
        outList1 = metamodelica::cons(e1, outList1);
        if true
        /* isPresent not implemented in Rust */
        {
            outList2 = metamodelica::cons(e2, outList2);
        }
        if true
        /* isPresent not implemented in Rust */
        {
            outList3 = metamodelica::cons(e3, outList3);
        }
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    if true
    /* isPresent not implemented in Rust */
    {
        outList2 = metamodelica::Dangerous::listReverseInPlace(outList2);
    }
    if true
    /* isPresent not implemented in Rust */
    {
        outList3 = metamodelica::Dangerous::listReverseInPlace(outList3);
    }
    Ok((outList1, outList2, outList3))
}

pub fn mapOption<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<Option<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut ei: TI;
    let mut eo: TO;
    for mut oe in &**inList {
        if (oe).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(oe.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ei = metamodelica::Own::own(__pa0);
            eo = inFunc(ei)?;
            outList = metamodelica::cons(eo, outList);
        }
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub fn map1Option<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<Option<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT) -> Result<TO>,
    mut inArg1: ArgT,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut ei: TI;
    let mut eo: TO;
    for mut oe in &**inList {
        if (oe).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(oe.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ei = metamodelica::Own::own(__pa0);
            eo = inFunc(ei, inArg1.clone())?;
            outList = metamodelica::cons(eo, outList);
        }
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub fn map2Option<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<Option<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut ei: TI;
    let mut eo: TO;
    for mut oe in &**inList {
        if (oe).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(oe.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ei = metamodelica::Own::own(__pa0);
            eo = inFunc(ei, inArg1.clone(), inArg2.clone())?;
            outList = metamodelica::cons(eo, outList);
        }
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub fn map_0<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<()>,
) -> Result<()> {
    pub type MapFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

    for mut e in &**inList {
        inFunc(e.clone())?;
    }
    Ok(())
}

pub fn map1<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inMapFunc: &dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO>,
    mut inArg1: ArgT1,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inMapFunc(e.clone(), inArg1.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn map1r<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(ArgT1, TI) -> Result<TO>,
    mut inArg1: ArgT1,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<ArgT1: Clone + 'static, TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(ArgT1, TI) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(inArg1.clone(), e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn map1_0<TI: Clone + 'static + metamodelica::gc::MMTrace, ArgT1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1) -> Result<()>,
    mut inArg1: ArgT1,
) -> Result<()> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<()> + 'static>;

    for mut e in &**inList {
        inFunc(e.clone(), inArg1.clone())?;
    }
    Ok(())
}

pub fn map1_2<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO1: Clone + 'static + metamodelica::gc::MMTrace,
    TO2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO1, TO2)>,
    mut inArg1: ArgT1,
) -> Result<(metamodelica::List<TO1>, metamodelica::List<TO2>)> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: metamodelica::List<TO1> = metamodelica::nil();
    let mut outList2: metamodelica::List<TO2> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    for mut e in &**inList {
        (e1, e2) = inFunc(e.clone(), inArg1.clone())?;
        outList1 = metamodelica::cons(e1, outList1);
        outList2 = metamodelica::cons(e2, outList2);
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    outList2 = metamodelica::Dangerous::listReverseInPlace(outList2);
    Ok((outList1, outList2))
}

pub fn map2<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn map2Reverse<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc
    });
    Ok(outList)
}

pub fn map2_0<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<()>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<()> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<()> + 'static>;

    for mut e in &**inList {
        inFunc(e.clone(), inArg1.clone(), inArg2.clone())?;
    }
    Ok(())
}

pub fn map2_2<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    TO1: Clone + 'static + metamodelica::gc::MMTrace,
    TO2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<(TO1, TO2)>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<(metamodelica::List<TO1>, metamodelica::List<TO2>)> {
    pub type MapFunc<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        TO1: Clone + 'static,
        TO2: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: metamodelica::List<TO1> = metamodelica::nil();
    let mut outList2: metamodelica::List<TO2> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    for mut e in &**inList {
        (e1, e2) = inFunc(e.clone(), inArg1.clone(), inArg2.clone())?;
        outList1 = metamodelica::cons(e1, outList1);
        outList2 = metamodelica::cons(e2, outList2);
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    outList2 = metamodelica::Dangerous::listReverseInPlace(outList2);
    Ok((outList1, outList2))
}

pub fn map3<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
    mut inArg3: ArgT3,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn map4<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT4: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
    mut inArg3: ArgT3,
    mut inArg4: ArgT4,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        ArgT4: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(
                e.clone(),
                inArg1.clone(),
                inArg2.clone(),
                inArg3.clone(),
                inArg4.clone(),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn map4_0<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT4: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<()>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
    mut inArg3: ArgT3,
    mut inArg4: ArgT4,
) -> Result<()> {
    pub type MapFunc<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        ArgT4: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<()> + 'static>;

    for mut e in &**inList {
        inFunc(
            e.clone(),
            inArg1.clone(),
            inArg2.clone(),
            inArg3.clone(),
            inArg4.clone(),
        )?;
    }
    Ok(())
}

pub fn map5<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT4: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT5: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
    mut inArg3: ArgT3,
    mut inArg4: ArgT4,
    mut inArg5: ArgT5,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        ArgT4: Clone + 'static,
        ArgT5: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(
                e.clone(),
                inArg1.clone(),
                inArg2.clone(),
                inArg3.clone(),
                inArg4.clone(),
                inArg5.clone(),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn map6<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT4: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT5: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT6: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5, ArgT6) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
    mut inArg3: ArgT3,
    mut inArg4: ArgT4,
    mut inArg5: ArgT5,
    mut inArg6: ArgT6,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        ArgT4: Clone + 'static,
        ArgT5: Clone + 'static,
        ArgT6: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5, ArgT6) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inFunc(
                e.clone(),
                inArg1.clone(),
                inArg2.clone(),
                inArg3.clone(),
                inArg4.clone(),
                inArg5.clone(),
                inArg6.clone(),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn mapFlat<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<TI>,
    mut inMapFunc: &dyn ::std::ops::Fn(TI) -> Result<metamodelica::List<TO>>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<metamodelica::List<TO>> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = mapFlatReverse(inList, inMapFunc)?.reverse();
    Ok(outList)
}

pub fn mapFlatReverse<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inMapFunc: &dyn ::std::ops::Fn(TI) -> Result<metamodelica::List<TO>>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<metamodelica::List<TO>> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    for mut e in &**inList {
        outList = listAppend(inMapFunc(e.clone())?, outList);
    }
    Ok(outList)
}

pub fn mapMap<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO1: Clone + 'static + metamodelica::gc::MMTrace,
    TO2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TI>,
    mut inMapFunc1: &dyn ::std::ops::Fn(TI) -> Result<TO1>,
    mut inMapFunc2: &dyn ::std::ops::Fn(TO1) -> Result<TO2>,
) -> Result<metamodelica::List<TO2>> {
    pub type MapFunc1<TI: Clone + 'static, TO1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO1> + 'static>;

    pub type MapFunc2<TO1: Clone + 'static, TO2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TO1) -> Result<TO2> + 'static>;

    let mut outList: metamodelica::List<TO2>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = inMapFunc2(inMapFunc1(e.clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn foldAllValue<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inList: &metamodelica::List<TI>,
    mut inMapFunc: &dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO, ArgT1)>,
    mut inValue: TO,
    mut inArg1: ArgT1,
) -> Result<()> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO, ArgT1)> + 'static>;

    let mut arg: ArgT1 = inArg1;
    let mut eo: TO;
    for mut e in &**inList {
        (eo, arg) = inMapFunc(e.clone(), arg)?;
        let true = (eo == inValue.clone()) else {
            return Err("pattern mismatch");
        };
    }
    Ok(())
}

pub fn applyAndFold<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFoldFunc: &dyn ::std::ops::Fn(TO, FT) -> Result<FT>,
    mut inApplyFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
    mut inFoldArg: FT,
) -> Result<FT> {
    pub type ApplyFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    pub type FoldFunc<TO: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TO, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inFoldArg;
    for mut e in &**inList {
        outResult = inFoldFunc(inApplyFunc(e.clone())?, outResult)?;
    }
    Ok(outResult)
}

pub fn applyAndFold1<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFoldFunc: &dyn ::std::ops::Fn(TO, FT) -> Result<FT>,
    mut inApplyFunc: &dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO>,
    mut inExtraArg: ArgT1,
    mut inFoldArg: FT,
) -> Result<FT> {
    pub type ApplyFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    pub type FoldFunc<TO: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TO, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inFoldArg;
    for mut e in &**inList {
        outResult = inFoldFunc(inApplyFunc(e.clone(), inExtraArg.clone())?, outResult)?;
    }
    Ok(outResult)
}

pub fn mapMapBoolAnd<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TI2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TI2>,
    mut inBFunc: &dyn ::std::ops::Fn(TI2) -> Result<bool>,
) -> Result<bool> {
    pub type MapBFunc<TI2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI2) -> Result<bool> + 'static>;

    pub type MapFunc<TI: Clone + 'static, TI2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TI2> + 'static>;

    let mut res: bool = false;
    for mut e in &**inList {
        if !(inBFunc(inFunc(e.clone())?)?) {
            return Ok(res);
        }
    }
    res = true;
    Ok(res)
}

pub fn mapList<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inListList: metamodelica::List<metamodelica::List<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::List<metamodelica::List<TO>>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outListList: metamodelica::List<metamodelica::List<TO>>;
    outListList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut lst in (inListList).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<_> = metamodelica::nil();
                for mut e in (lst.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outListList)
}

pub fn mapListReverse<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inListList: metamodelica::List<metamodelica::List<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::List<metamodelica::List<TO>>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outListList: metamodelica::List<metamodelica::List<TO>>;
    outListList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut lst in (inListList).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<_> = metamodelica::nil();
                for mut e in (lst.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outListList)
}

pub fn map1List<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inListList: metamodelica::List<metamodelica::List<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO>,
    mut inArg1: ArgT1,
) -> Result<metamodelica::List<metamodelica::List<TO>>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    let mut outListList: metamodelica::List<metamodelica::List<TO>>;
    outListList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut lst in (inListList).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<_> = metamodelica::nil();
                for mut e in (lst.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone(), inArg1.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outListList)
}

pub fn map2List<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inListList: metamodelica::List<metamodelica::List<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<metamodelica::List<metamodelica::List<TO>>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outListList: metamodelica::List<metamodelica::List<TO>>;
    outListList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut lst in (inListList).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<_> = metamodelica::nil();
                for mut e in (lst.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outListList)
}

pub fn fold<T: Clone + 'static + metamodelica::gc::MMTrace, FT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, FT) -> Result<FT>,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<T: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(e.clone(), outResult)?;
    }
    Ok(outResult)
}

pub fn foldr<T: Clone + 'static + metamodelica::gc::MMTrace, FT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(FT, T) -> Result<FT>,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<FT: Clone + 'static, T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(FT, T) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(outResult, e.clone())?;
    }
    Ok(outResult)
}

pub fn fold1<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, ArgT1, FT) -> Result<FT>,
    mut inExtraArg: ArgT1,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(e.clone(), inExtraArg.clone(), outResult)?;
    }
    Ok(outResult)
}

pub fn fold1r<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(FT, T, ArgT1) -> Result<FT>,
    mut inExtraArg: ArgT1,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<FT: Clone + 'static, T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(FT, T, ArgT1) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(outResult, e.clone(), inExtraArg.clone())?;
    }
    Ok(outResult)
}

pub fn fold2<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT) -> Result<FT>,
    mut inExtraArg1: ArgT1,
    mut inExtraArg2: ArgT2,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(e.clone(), inExtraArg1.clone(), inExtraArg2.clone(), outResult)?;
    }
    Ok(outResult)
}

pub fn fold22<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT1, FT2) -> Result<(FT1, FT2)>,
    mut inExtraArg1: ArgT1,
    mut inExtraArg2: ArgT2,
    mut inStartValue1: FT1,
    mut inStartValue2: FT2,
) -> Result<(FT1, FT2)> {
    pub type FoldFunc<
        T: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        FT1: Clone + 'static,
        FT2: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

    let mut outResult1: FT1 = inStartValue1;
    let mut outResult2: FT2 = inStartValue2;
    for mut e in &**inList {
        (outResult1, outResult2) = inFoldFunc(
            e.clone(),
            inExtraArg1.clone(),
            inExtraArg2.clone(),
            outResult1,
            outResult2,
        )?;
    }
    Ok((outResult1, outResult2))
}

pub fn foldList<T: Clone + 'static + metamodelica::gc::MMTrace, FT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<metamodelica::List<T>>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, FT) -> Result<FT>,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<T: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut lst in &**inList {
        for mut e in &*lst.clone() {
            outResult = inFoldFunc(e.clone(), outResult)?;
        }
    }
    Ok(outResult)
}

pub fn fold2r<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(FT, T, ArgT1, ArgT2) -> Result<FT>,
    mut inExtraArg1: ArgT1,
    mut inExtraArg2: ArgT2,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<FT: Clone + 'static, T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(FT, T, ArgT1, ArgT2) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(outResult, e.clone(), inExtraArg1.clone(), inExtraArg2.clone())?;
    }
    Ok(outResult)
}

pub fn fold3<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, FT) -> Result<FT>,
    mut inExtraArg1: ArgT1,
    mut inExtraArg2: ArgT2,
    mut inExtraArg3: ArgT3,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<
        T: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        FT: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(
            e.clone(),
            inExtraArg1.clone(),
            inExtraArg2.clone(),
            inExtraArg3.clone(),
            outResult,
        )?;
    }
    Ok(outResult)
}

pub fn fold4<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT4: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, ArgT4, FT) -> Result<FT>,
    mut inExtraArg1: ArgT1,
    mut inExtraArg2: ArgT2,
    mut inExtraArg3: ArgT3,
    mut inExtraArg4: ArgT4,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<
        T: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        ArgT4: Clone + 'static,
        FT: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, ArgT4, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue;
    for mut e in &**inList {
        outResult = inFoldFunc(
            e.clone(),
            inExtraArg1.clone(),
            inExtraArg2.clone(),
            inExtraArg3.clone(),
            inExtraArg4.clone(),
            outResult,
        )?;
    }
    Ok(outResult)
}

pub fn fold20<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    FT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, FT1, FT2) -> Result<(FT1, FT2)>,
    mut inStartValue1: FT1,
    mut inStartValue2: FT2,
) -> Result<(FT1, FT2)> {
    pub type FoldFunc<T: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

    let mut outResult1: FT1 = inStartValue1;
    let mut outResult2: FT2 = inStartValue2;
    for mut e in &**inList {
        (outResult1, outResult2) = inFoldFunc(e.clone(), outResult1, outResult2)?;
    }
    Ok((outResult1, outResult2))
}

pub fn fold21<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, ArgT1, FT1, FT2) -> Result<(FT1, FT2)>,
    mut inExtraArg1: ArgT1,
    mut inStartValue1: FT1,
    mut inStartValue2: FT2,
) -> Result<(FT1, FT2)> {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

    let mut outResult1: FT1 = inStartValue1;
    let mut outResult2: FT2 = inStartValue2;
    for mut e in &**inList {
        (outResult1, outResult2) = inFoldFunc(e.clone(), inExtraArg1.clone(), outResult1, outResult2)?;
    }
    Ok((outResult1, outResult2))
}

pub fn fold31<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, ArgT1, FT1, FT2, FT3) -> Result<(FT1, FT2, FT3)>,
    mut inExtraArg1: ArgT1,
    mut inStartValue1: FT1,
    mut inStartValue2: FT2,
    mut inStartValue3: FT3,
) -> Result<(FT1, FT2, FT3)> {
    pub type FoldFunc<
        T: Clone + 'static,
        ArgT1: Clone + 'static,
        FT1: Clone + 'static,
        FT2: Clone + 'static,
        FT3: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, FT1, FT2, FT3) -> Result<(FT1, FT2, FT3)> + 'static>;

    let mut outResult1: FT1 = inStartValue1;
    let mut outResult2: FT2 = inStartValue2;
    let mut outResult3: FT3 = inStartValue3;
    for mut e in &**inList {
        (outResult1, outResult2, outResult3) =
            inFoldFunc(e.clone(), inExtraArg1.clone(), outResult1, outResult2, outResult3)?;
    }
    Ok((outResult1, outResult2, outResult3))
}

pub fn mapFold<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)>,
    mut inArg: FT,
) -> Result<(metamodelica::List<TO>, FT)> {
    pub type FuncType<TI: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut outArg: FT = inArg;
    let mut res: TO;
    for mut e in &**inList {
        (res, outArg) = inFunc(e.clone(), outArg)?;
        outList = metamodelica::cons(res, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, outArg))
}

pub fn mapFold2<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    FT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, FT1, FT2) -> Result<(TO, FT1, FT2)>,
    mut inArg1: FT1,
    mut inArg2: FT2,
) -> Result<(metamodelica::List<TO>, FT1, FT2)> {
    pub type FuncType<TI: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, FT1, FT2) -> Result<(TO, FT1, FT2)> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut outArg1: FT1 = inArg1;
    let mut outArg2: FT2 = inArg2;
    let mut res: TO;
    for mut e in &**inList {
        (res, outArg1, outArg2) = inFunc(e.clone(), outArg1, outArg2)?;
        outList = metamodelica::cons(res, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, outArg1, outArg2))
}

pub fn mapFold3<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    FT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT3: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, FT1, FT2, FT3) -> Result<(TO, FT1, FT2, FT3)>,
    mut inArg1: FT1,
    mut inArg2: FT2,
    mut inArg3: FT3,
) -> Result<(metamodelica::List<TO>, FT1, FT2, FT3)> {
    pub type FuncType<
        TI: Clone + 'static,
        FT1: Clone + 'static,
        FT2: Clone + 'static,
        FT3: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, FT1, FT2, FT3) -> Result<(TO, FT1, FT2, FT3)> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut inArg1: FT1 = inArg1;
    let mut inArg2: FT2 = inArg2;
    let mut inArg3: FT3 = inArg3;
    let mut res: TO;
    for mut e in &**inList {
        (res, inArg1, inArg2, inArg3) = inFunc(e.clone(), inArg1, inArg2, inArg3)?;
        outList = metamodelica::cons(res, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, inArg1, inArg2, inArg3))
}

pub fn mapFold5<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    FT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT3: Clone + 'static + metamodelica::gc::MMTrace,
    FT4: Clone + 'static + metamodelica::gc::MMTrace,
    FT5: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, FT1, FT2, FT3, FT4, FT5) -> Result<(TO, FT1, FT2, FT3, FT4, FT5)>,
    mut inArg1: FT1,
    mut inArg2: FT2,
    mut inArg3: FT3,
    mut inArg4: FT4,
    mut inArg5: FT5,
) -> Result<(metamodelica::List<TO>, FT1, FT2, FT3, FT4, FT5)> {
    pub type FuncType<
        TI: Clone + 'static,
        FT1: Clone + 'static,
        FT2: Clone + 'static,
        FT3: Clone + 'static,
        FT4: Clone + 'static,
        FT5: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<
        dyn ::std::ops::Fn(TI, FT1, FT2, FT3, FT4, FT5) -> Result<(TO, FT1, FT2, FT3, FT4, FT5)> + 'static,
    >;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut inArg1: FT1 = inArg1;
    let mut inArg2: FT2 = inArg2;
    let mut inArg3: FT3 = inArg3;
    let mut inArg4: FT4 = inArg4;
    let mut inArg5: FT5 = inArg5;
    let mut res: TO;
    for mut e in &**inList {
        (res, inArg1, inArg2, inArg3, inArg4, inArg5) = inFunc(e.clone(), inArg1, inArg2, inArg3, inArg4, inArg5)?;
        outList = metamodelica::cons(res, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, inArg1, inArg2, inArg3, inArg4, inArg5))
}

pub fn map1Fold<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, FT) -> Result<(TO, FT)>,
    mut inConstArg: ArgT1,
    mut inArg: FT,
) -> Result<(metamodelica::List<TO>, FT)> {
    pub type FuncType<TI: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut outArg: FT = inArg;
    let mut res: TO;
    for mut e in &**inList {
        (res, outArg) = inFunc(e.clone(), inConstArg.clone(), outArg)?;
        outList = metamodelica::cons(res, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, outArg))
}

pub fn map2Fold<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2, FT) -> Result<(TO, FT)>,
    mut inConstArg: ArgT1,
    mut inConstArg2: ArgT2,
    mut inArg: FT,
    mut inAccum: metamodelica::List<TO>,
) -> Result<(metamodelica::List<TO>, FT)> {
    pub type FuncType<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        FT: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: metamodelica::List<TO> = inAccum;
    let mut outArg: FT = inArg;
    let mut res: TO;
    for mut e in &**inList {
        (res, outArg) = inFunc(e.clone(), inConstArg.clone(), inConstArg2.clone(), outArg)?;
        outList = metamodelica::cons(res, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, outArg))
}

pub fn map2FoldCheckReferenceEq<
    TIO: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<TIO>,
    mut inFunc: &dyn ::std::ops::Fn(TIO, ArgT1, ArgT2, FT) -> Result<(TIO, FT)>,
    mut inConstArg: ArgT1,
    mut inConstArg2: ArgT2,
    mut inArg: FT,
) -> Result<(metamodelica::List<TIO>, FT)> {
    pub type FuncType<TIO: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TIO, ArgT1, ArgT2, FT) -> Result<(TIO, FT)> + 'static>;

    let mut outList: metamodelica::List<TIO>;
    let mut outArg: FT = inArg;
    let mut res: TIO;
    let mut savedElt: TIO;
    let mut delst: DoubleEnded::MutableList<TIO>;
    let mut n: i32 = 0;
    for mut e in &*inList {
        (res, outArg) = inFunc(e.clone(), inConstArg.clone(), inConstArg2.clone(), outArg)?;
        if !(metamodelica::ReferenceEq::reference_eq(&(e.clone()), &(res.clone()))) {
            savedElt = res.clone();
            delst = DoubleEnded::empty(res);
            for mut elt in &*inList {
                if n < 0 {
                    (res, outArg) = inFunc(elt.clone(), inConstArg.clone(), inConstArg2.clone(), outArg)?;
                } else {
                    res = if (n == 0) { savedElt.clone() } else { elt.clone() };
                }
                DoubleEnded::push_back(delst.clone(), res)?;
                n = n - 1;
            }
            outList = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
            return Ok((outList, outArg));
        }
        n = n + 1;
    }
    outList = inList;
    Ok((outList, outArg))
}

pub fn map3Fold<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, FT) -> Result<(TO, FT)>,
    mut inConstArg: ArgT1,
    mut inConstArg2: ArgT2,
    mut inConstArg3: ArgT3,
    mut inArg: FT,
) -> Result<(metamodelica::List<TO>, FT)> {
    pub type FuncType<
        TI: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        FT: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut outArg: FT = inArg;
    let mut res: TO;
    for mut e in &**inList {
        (res, outArg) = inFunc(
            e.clone(),
            inConstArg.clone(),
            inConstArg2.clone(),
            inConstArg3.clone(),
            outArg,
        )?;
        outList = metamodelica::cons(res, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, outArg))
}

pub fn mapFoldList<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inListList: &metamodelica::List<metamodelica::List<TI>>,
    mut inFunc: &dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)>,
    mut inArg: FT,
) -> Result<(metamodelica::List<metamodelica::List<TO>>, FT)> {
    pub type FuncType<TI: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)> + 'static>;

    let mut outListList: metamodelica::List<metamodelica::List<TO>> = metamodelica::nil();
    let mut outArg: FT = inArg;
    let mut res: metamodelica::List<TO>;
    for mut lst in &**inListList {
        (res, outArg) = mapFold(metamodelica::AsArg::as_arg(&lst), inFunc, outArg)?;
        outListList = metamodelica::cons(res, outListList);
    }
    outListList = metamodelica::Dangerous::listReverseInPlace(outListList);
    Ok((outListList, outArg))
}

pub fn reduce<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inReduceFunc: &dyn ::std::ops::Fn(T, T) -> Result<T>,
) -> Result<T> {
    pub type ReduceFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>;

    let mut outResult: T;
    let mut rest: metamodelica::List<T>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inList)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outResult = metamodelica::Own::own(__pa0);
    rest = metamodelica::Own::own(__pa1);
    for mut e in &*rest {
        outResult = inReduceFunc(outResult, e.clone())?;
    }
    Ok(outResult)
}

pub fn flatten<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<metamodelica::List<T>>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = if ((inList).is_empty()) {
        metamodelica::nil()
    } else if (hasOneElement(&inList)) {
        (inList).head().cloned()?
    } else {
        ({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut lst in (inList.clone().reverse()).into_iter().cloned() {
                let __x = lst.clone();
                __acc = __x.append(&__acc);
            }
            __acc
        })
    };
    Ok(outList)
}

pub fn flattenReverse<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<metamodelica::List<T>>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = if ((inList).is_empty()) {
        metamodelica::nil()
    } else if (hasOneElement(&inList)) {
        (inList).head().cloned()?
    } else {
        ({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut lst in (inList.clone()).into_iter().cloned() {
                let __x = lst.clone();
                __acc = __x.append(&__acc);
            }
            __acc
        })
    };
    Ok(outList)
}

pub fn thread<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: &metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
    mut inAccum: &metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut e2: T;
    let mut rest_e2: metamodelica::List<T> = inList2;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_e2 = metamodelica::Own::own(__pa1);
        outList = metamodelica::cons(e1.clone(), metamodelica::cons(e2, outList));
    }
    let true = ((rest_e2).is_empty()) else {
        return Err("pattern mismatch");
    };
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub fn zip<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
) -> metamodelica::List<(T1, T2)> {
    let mut outTuples: metamodelica::List<(T1, T2)>;
    outTuples = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = inList1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inList2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = (e1.clone(), e2.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => panic!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    outTuples
}

pub fn zip3<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    T3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut l1: metamodelica::List<T1>,
    mut l2: metamodelica::List<T2>,
    mut l3: metamodelica::List<T3>,
) -> metamodelica::List<(T1, T2, T3)> {
    let mut res: metamodelica::List<(T1, T2, T3)>;
    res = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = l1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = l2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        let __thr_src2 = l3;
        let mut __thr_it2 = (&__thr_src2).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next(), __thr_it2.next()) {
                (Some(e1), Some(e2), Some(e3)) => {
                    let __x = (e1.clone(), e2.clone(), e3.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None, None) => break,
                _ => panic!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    res
}

pub fn unzip<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTuples: &metamodelica::List<(T1, T2)>,
) -> (metamodelica::List<T1>, metamodelica::List<T2>) {
    let mut outList1: metamodelica::List<T1> = metamodelica::nil();
    let mut outList2: metamodelica::List<T2> = metamodelica::nil();
    let mut e1: T1;
    let mut e2: T2;
    for mut tpl in &**inTuples {
        (e1, e2) = tpl.clone();
        outList1 = metamodelica::cons(e1, outList1);
        if true
        /* isPresent not implemented in Rust */
        {
            outList2 = metamodelica::cons(e2, outList2);
        }
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    outList2 = metamodelica::Dangerous::listReverseInPlace(outList2);
    (outList1, outList2)
}

pub fn unzip3<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    T3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut tuples: metamodelica::List<(T1, T2, T3)>,
) -> (metamodelica::List<T1>, metamodelica::List<T2>, metamodelica::List<T3>) {
    let mut l1: metamodelica::List<T1> = metamodelica::nil();
    let mut l2: metamodelica::List<T2> = metamodelica::nil();
    let mut l3: metamodelica::List<T3> = metamodelica::nil();
    let mut e1: T1;
    let mut e2: T2;
    let mut e3: T3;
    for mut t in &*tuples.reverse() {
        (e1, e2, e3) = t.clone();
        l1 = metamodelica::cons(e1, l1);
        l2 = metamodelica::cons(e2, l2);
        l3 = metamodelica::cons(e3, l3);
    }
    (l1, l2, l3)
}

pub fn unzipSecond<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTuples: &metamodelica::List<(T1, T2)>,
) -> metamodelica::List<T2> {
    let mut outList: metamodelica::List<T2> = metamodelica::nil();
    let mut e: T2;
    for mut tpl in &**inTuples {
        (_, e) = tpl.clone();
        outList = metamodelica::cons(e, outList);
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    outList
}

pub fn threadMap<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inMapFunc: &dyn ::std::ops::Fn(T1, T2) -> Result<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = inList1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inList2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = inMapFunc(e1.clone(), e2.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub(crate) fn threadMap_2<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    TO1: Clone + 'static + metamodelica::gc::MMTrace,
    TO2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inMapFunc: &dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)>,
) -> Result<(metamodelica::List<TO1>, metamodelica::List<TO2>)> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: metamodelica::List<TO1> = metamodelica::nil();
    let mut outList2: metamodelica::List<TO2> = metamodelica::nil();
    let mut e2: T2;
    let mut rest_e2: metamodelica::List<T2> = inList2;
    let mut ret1: TO1;
    let mut ret2: TO2;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_e2 = metamodelica::Own::own(__pa1);
        (ret1, ret2) = inMapFunc(e1.clone(), e2)?;
        outList1 = metamodelica::cons(ret1, outList1);
        outList2 = metamodelica::cons(ret2, outList2);
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    outList2 = metamodelica::Dangerous::listReverseInPlace(outList2);
    Ok((outList1, outList2))
}

pub(crate) fn threadMapList<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: metamodelica::List<metamodelica::List<T1>>,
    mut inList2: metamodelica::List<metamodelica::List<T2>>,
    mut inMapFunc: &dyn ::std::ops::Fn(T1, T2) -> Result<TO>,
) -> Result<metamodelica::List<metamodelica::List<TO>>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<metamodelica::List<TO>>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = inList1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inList2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(lst1), Some(lst2)) => {
                    let __x = threadMap(lst1.clone(), lst2.clone(), inMapFunc)?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn threadMapList_2<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    TO1: Clone + 'static + metamodelica::gc::MMTrace,
    TO2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<metamodelica::List<T1>>,
    mut inList2: metamodelica::List<metamodelica::List<T2>>,
    mut inMapFunc: &dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)>,
) -> Result<(
    metamodelica::List<metamodelica::List<TO1>>,
    metamodelica::List<metamodelica::List<TO2>>,
)> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: metamodelica::List<metamodelica::List<TO1>> = metamodelica::nil();
    let mut outList2: metamodelica::List<metamodelica::List<TO2>> = metamodelica::nil();
    let mut l2: metamodelica::List<T2>;
    let mut rest_l2: metamodelica::List<metamodelica::List<T2>> = inList2;
    let mut ret1: metamodelica::List<TO1>;
    let mut ret2: metamodelica::List<TO2>;
    for mut l1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_l2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        l2 = metamodelica::Own::own(__pa0);
        rest_l2 = metamodelica::Own::own(__pa1);
        (ret1, ret2) = threadMap_2(metamodelica::AsArg::as_arg(&l1), l2, inMapFunc)?;
        outList1 = metamodelica::cons(ret1, outList1);
        outList2 = metamodelica::cons(ret2, outList2);
    }
    outList1 = metamodelica::Dangerous::listReverseInPlace(outList1);
    outList2 = metamodelica::Dangerous::listReverseInPlace(outList2);
    Ok((outList1, outList2))
}

pub fn threadMap1<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inMapFunc: &dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<TO>,
    mut inArg1: ArgT1,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = inList1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inList2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = inMapFunc(e1.clone(), e2.clone(), inArg1.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn threadMap1_0<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inMapFunc: &dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<()>,
    mut inArg1: ArgT1,
) -> Result<()> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<()> + 'static>;

    let mut rest2: metamodelica::List<T2> = inList2;
    let mut e2: T2;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest2 = metamodelica::Own::own(__pa1);
        inMapFunc(e1.clone(), e2, inArg1.clone())?;
    }
    let true = ((rest2).is_empty()) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

pub fn threadMap2<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inMapFunc: &dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2) -> Result<TO>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<
        T1: Clone + 'static,
        T2: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = inList1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inList2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = inMapFunc(e1.clone(), e2.clone(), inArg1.clone(), inArg2.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn thread3Map<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    T3: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inList3: metamodelica::List<T3>,
    mut inFunc: &dyn ::std::ops::Fn(T1, T2, T3) -> Result<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2, T3) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = inList1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inList2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        let __thr_src2 = inList3;
        let mut __thr_it2 = (&__thr_src2).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next(), __thr_it2.next()) {
                (Some(e1), Some(e2), Some(e3)) => {
                    let __x = inFunc(e1.clone(), e2.clone(), e3.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn thread3MapFold<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    T3: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inList3: metamodelica::List<T3>,
    mut inFunc: &dyn ::std::ops::Fn(T1, T2, T3, ArgT1) -> Result<(TO, ArgT1)>,
    mut inArg: ArgT1,
) -> Result<(metamodelica::List<TO>, ArgT1)> {
    pub type MapFunc<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        ArgT1: Clone + 'static,
        TO: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, T3, ArgT1) -> Result<(TO, ArgT1)> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut outArg: ArgT1 = inArg;
    let mut e2: T2;
    let mut rest_e2: metamodelica::List<T2> = inList2;
    let mut e3: T3;
    let mut rest_e3: metamodelica::List<T3> = inList3;
    let mut res: TO;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_e2 = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_e3) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e3 = metamodelica::Own::own(__pa2);
        rest_e3 = metamodelica::Own::own(__pa3);
        (res, outArg) = inFunc(e1.clone(), e2, e3, outArg)?;
        outList = metamodelica::cons(res, outList);
    }
    let true = ((rest_e2).is_empty()) else {
        return Err("pattern mismatch");
    };
    let true = ((rest_e3).is_empty()) else {
        return Err("pattern mismatch");
    };
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, outArg))
}

pub fn threadFold1<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T1, T2, ArgT1, FT) -> Result<FT>,
    mut inArg1: ArgT1,
    mut inFoldArg: FT,
) -> Result<FT> {
    pub type FoldFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT = inFoldArg;
    let mut rest2: metamodelica::List<T2> = inList2;
    let mut e2: T2;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest2 = metamodelica::Own::own(__pa1);
        outFoldArg = inFoldFunc(e1.clone(), e2, inArg1.clone(), outFoldArg)?;
    }
    let true = ((rest2).is_empty()) else {
        return Err("pattern mismatch");
    };
    Ok(outFoldArg)
}

pub fn threadFold2<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, FT) -> Result<FT>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
    mut inFoldArg: FT,
) -> Result<FT> {
    pub type FoldFunc<
        T1: Clone + 'static,
        T2: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        FT: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT = inFoldArg;
    let mut rest2: metamodelica::List<T2> = inList2;
    let mut e2: T2;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest2 = metamodelica::Own::own(__pa1);
        outFoldArg = inFoldFunc(e1.clone(), e2, inArg1.clone(), inArg2.clone(), outFoldArg)?;
    }
    let true = ((rest2).is_empty()) else {
        return Err("pattern mismatch");
    };
    Ok(outFoldArg)
}

pub fn threadFold3<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT3: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, ArgT3, FT) -> Result<FT>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
    mut inArg3: ArgT3,
    mut inFoldArg: FT,
) -> Result<FT> {
    pub type FoldFunc<
        T1: Clone + 'static,
        T2: Clone + 'static,
        ArgT1: Clone + 'static,
        ArgT2: Clone + 'static,
        ArgT3: Clone + 'static,
        FT: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, ArgT3, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT = inFoldArg;
    let mut rest2: metamodelica::List<T2> = inList2;
    let mut e2: T2;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest2 = metamodelica::Own::own(__pa1);
        outFoldArg = inFoldFunc(
            e1.clone(),
            e2,
            inArg1.clone(),
            inArg2.clone(),
            inArg3.clone(),
            outFoldArg,
        )?;
    }
    let true = ((rest2).is_empty()) else {
        return Err("pattern mismatch");
    };
    Ok(outFoldArg)
}

pub fn threadFold<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T1, T2, FT) -> Result<FT>,
    mut inFoldArg: FT,
) -> Result<FT> {
    pub type FoldFunc<T1: Clone + 'static, T2: Clone + 'static, FT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT = inFoldArg;
    let mut rest2: metamodelica::List<T2> = inList2;
    let mut e2: T2;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest2 = metamodelica::Own::own(__pa1);
        outFoldArg = inFoldFunc(e1.clone(), e2, outFoldArg)?;
    }
    let true = ((rest2).is_empty()) else {
        return Err("pattern mismatch");
    };
    Ok(outFoldArg)
}

pub fn threadMapFold<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    FT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList1: &metamodelica::List<T1>,
    mut inList2: metamodelica::List<T2>,
    mut inFunc: &dyn ::std::ops::Fn(T1, T2, FT) -> Result<(TO, FT)>,
    mut inArg: FT,
) -> Result<(metamodelica::List<TO>, FT)> {
    pub type FuncType<T1: Clone + 'static, T2: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut outArg: FT = inArg;
    let mut e2: T2;
    let mut rest_e2: metamodelica::List<T2> = inList2;
    let mut res: TO;
    for mut e1 in &**inList1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_e2 = metamodelica::Own::own(__pa1);
        (res, outArg) = inFunc(e1.clone(), e2, outArg)?;
        outList = metamodelica::cons(res, outList);
    }
    let true = ((rest_e2).is_empty()) else {
        return Err("pattern mismatch");
    };
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok((outList, outArg))
}

pub fn position<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inElement: T,
    mut inList: &metamodelica::List<T>,
) -> Result<i32> {
    let mut outPosition: i32 = 1;
    for mut e in &**inList {
        if e.clone() == inElement.clone() {
            return Ok(outPosition);
        }
        outPosition = outPosition + 1;
    }
    return Err("fail");
    Ok(outPosition)
}

pub fn positionOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inPredFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<i32> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outPosition: i32 = 1;
    for mut e in &**inList {
        if inPredFunc(e.clone())? {
            return Ok(outPosition);
        }
        outPosition = outPosition + 1;
    }
    outPosition = -1;
    Ok(outPosition)
}

pub fn position1OnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inPredFunc: &dyn ::std::ops::Fn(T, ArgT) -> Result<bool>,
    mut inArg: ArgT,
) -> Result<i32> {
    pub type PredFunc<T: Clone + 'static, ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT) -> Result<bool> + 'static>;

    let mut outPosition: i32 = 1;
    for mut e in &**inList {
        if inPredFunc(e.clone(), inArg.clone())? {
            return Ok(outPosition);
        }
        outPosition = outPosition + 1;
    }
    outPosition = -1;
    Ok(outPosition)
}

pub fn getMember<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inElement: T,
    mut inList: &metamodelica::List<T>,
) -> Result<T> {
    let mut outElement: T;
    let mut e: T;
    for mut e in &**inList {
        let mut e = e.clone();
        if inElement.clone() == e.clone() {
            outElement = e;
            return Ok(outElement);
        }
    }
    return Err("fail");
    Ok(outElement)
}

pub fn getMemberOnTrue<
    VT: Clone + 'static + metamodelica::gc::MMTrace,
    T: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inValue: VT,
    mut inList: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(VT, T) -> Result<bool>,
) -> Result<T> {
    pub type CompFunc<VT: Clone + 'static, T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outElement: T;
    for mut e in &**inList {
        if inCompFunc(inValue.clone(), e.clone())? {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    return Err("fail");
    Ok(outElement)
}

pub fn notMember<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inElement: T,
    mut inList: metamodelica::List<T>,
) -> bool {
    let mut outIsNotMember: bool;
    outIsNotMember = !(listMember(inElement, inList));
    outIsNotMember
}

pub fn isMemberOnTrue<
    VT: Clone + 'static + metamodelica::gc::MMTrace,
    T: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inValue: VT,
    mut inList: &metamodelica::List<T>,
    mut inCompFunc: &dyn ::std::ops::Fn(VT, T) -> Result<bool>,
) -> Result<bool> {
    pub type CompFunc<VT: Clone + 'static, T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outIsMember: bool;
    for mut e in &**inList {
        if inCompFunc(inValue.clone(), e.clone())? {
            outIsMember = true;
            return Ok(outIsMember);
        }
    }
    outIsMember = false;
    Ok(outIsMember)
}

pub fn exist1<T: Clone + 'static + metamodelica::gc::MMTrace, ArgT1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFindFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<bool>,
    mut inExtraArg: ArgT1,
) -> Result<bool> {
    pub type FindFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outExists: bool;
    for mut e in &**inList {
        if inFindFunc(e.clone(), inExtraArg.clone())? {
            outExists = true;
            return Ok(outExists);
        }
    }
    outExists = false;
    Ok(outExists)
}

pub fn extractOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outExtractedList: metamodelica::List<T> = metamodelica::nil();
    let mut outRemainingList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if inFilterFunc(e.clone())? {
            outExtractedList = metamodelica::cons(e.clone(), outExtractedList);
        } else {
            outRemainingList = metamodelica::cons(e.clone(), outRemainingList);
        }
    }
    outExtractedList = metamodelica::Dangerous::listReverseInPlace(outExtractedList);
    outRemainingList = metamodelica::Dangerous::listReverseInPlace(outRemainingList);
    Ok((outExtractedList, outRemainingList))
}

pub fn extract1OnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<bool>,
    mut inArg: ArgT1,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outExtractedList: metamodelica::List<T> = metamodelica::nil();
    let mut outRemainingList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if inFilterFunc(e.clone(), inArg.clone())? {
            outExtractedList = metamodelica::cons(e.clone(), outExtractedList);
        } else {
            outRemainingList = metamodelica::cons(e.clone(), outRemainingList);
        }
    }
    outExtractedList = metamodelica::Dangerous::listReverseInPlace(outExtractedList);
    outRemainingList = metamodelica::Dangerous::listReverseInPlace(outRemainingList);
    Ok((outExtractedList, outRemainingList))
}

pub(crate) fn filter<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T) -> Result<()>,
) -> metamodelica::List<T> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if '__try0: {
            unwrap_break_err!(inFilterFunc(e.clone()), '__try0);
            outList = metamodelica::cons(e.clone(), outList.clone());
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    outList
}

pub fn filterMap<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<TI>,
    mut inFilterMapFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> metamodelica::List<TO> {
    pub type FilterMapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut oe: TO;
    for mut e in &**inList {
        if '__try0: {
            oe = unwrap_break_err!(inFilterMapFunc(e.clone()), '__try0);
            outList = metamodelica::cons(oe.clone(), outList.clone());
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    outList
}

pub fn filterMap1<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inFilterMapFunc: &dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO>,
    mut inExtraArg: ArgT1,
) -> metamodelica::List<TO> {
    pub type FilterMapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    let mut oe: TO;
    for mut e in &**inList {
        if '__try0: {
            oe = unwrap_break_err!(inFilterMapFunc(e.clone(), inExtraArg.clone()), '__try0);
            outList = metamodelica::cons(oe.clone(), outList.clone());
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    outList
}

pub fn filterOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inFilterFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            if !(inFilterFunc(e.clone())?) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn filterOnFalse<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            if !(boolNot(inFilterFunc(e.clone())?)) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn filter1OnTrueSync<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T1>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T1, ArgT1) -> Result<bool>,
    mut inArg1: ArgT1,
    mut inSyncList: metamodelica::List<T2>,
) -> Result<(metamodelica::List<T1>, metamodelica::List<T2>)> {
    pub type FilterFunc<T1: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, ArgT1) -> Result<bool> + 'static>;

    let mut outList_a: metamodelica::List<T1> = metamodelica::nil();
    let mut outList_b: metamodelica::List<T2> = metamodelica::nil();
    let mut e2: T2;
    let mut rest2: metamodelica::List<T2> = inSyncList;
    for mut e1 in &**inList {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest2 = metamodelica::Own::own(__pa1);
        if inFilterFunc(e1.clone(), inArg1.clone())? {
            outList_a = metamodelica::cons(e1.clone(), outList_a);
            outList_b = metamodelica::cons(e2, outList_b);
        }
    }
    outList_a = metamodelica::Dangerous::listReverseInPlace(outList_a);
    outList_b = metamodelica::Dangerous::listReverseInPlace(outList_b);
    Ok((outList_a, outList_b))
}

pub fn filterOnTrueSync<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T1>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T1) -> Result<bool>,
    mut inSyncList: metamodelica::List<T2>,
) -> Result<(metamodelica::List<T1>, metamodelica::List<T2>)> {
    pub type FilterFunc<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<bool> + 'static>;

    let mut outList_a: metamodelica::List<T1> = metamodelica::nil();
    let mut outList_b: metamodelica::List<T2> = metamodelica::nil();
    let mut e2: T2;
    let mut rest2: metamodelica::List<T2> = inSyncList.clone();
    let true = (((inList).len() as i32) == ((inSyncList).len() as i32)) else {
        return Err("pattern mismatch");
    };
    for mut e1 in &**inList {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest2 = metamodelica::Own::own(__pa1);
        if inFilterFunc(e1.clone())? {
            outList_a = metamodelica::cons(e1.clone(), outList_a);
            outList_b = metamodelica::cons(e2, outList_b);
        }
    }
    outList_a = metamodelica::Dangerous::listReverseInPlace(outList_a);
    outList_b = metamodelica::Dangerous::listReverseInPlace(outList_b);
    Ok((outList_a, outList_b))
}

pub fn filter1<T: Clone + 'static + metamodelica::gc::MMTrace, ArgT1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<()>,
    mut inArg1: ArgT1,
) -> metamodelica::List<T> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<()> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if '__try0: {
            unwrap_break_err!(inFilterFunc(e.clone(), inArg1.clone()), '__try0);
            outList = metamodelica::cons(e.clone(), outList.clone());
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    outList
}

pub fn filter1OnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<T>,
    mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>,
    mut inArg1: ArgT1,
) -> Result<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            if !(inFilterFunc(e.clone(), inArg1.clone())?) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn filter1OnTrueAndUpdate<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<bool>,
    mut inUpdateFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<T>,
    mut inArg1: ArgT1,
) -> Result<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    pub type UpdateFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<T> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            if !(inFilterFunc(e.clone(), inArg1.clone())?) {
                continue;
            }
            let __x = inUpdateFunc(e.clone(), inArg1.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn filter1rOnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<T>,
    mut inFilterFunc: Arc<dyn ::std::ops::Fn(ArgT1, T) -> Result<bool> + 'static>,
    mut inArg1: ArgT1,
) -> Result<metamodelica::List<T>> {
    pub type FilterFunc<ArgT1: Clone + 'static, T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(ArgT1, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            if !(inFilterFunc(inArg1.clone(), e.clone())?) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn filter2OnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<T>,
    mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool> + 'static>,
    mut inArg1: ArgT1,
    mut inArg2: ArgT2,
) -> Result<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            if !(inFilterFunc(e.clone(), inArg1.clone(), inArg2.clone())?) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn removeOnTrue<VT: Clone + 'static + metamodelica::gc::MMTrace, T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inValue: VT,
    mut inCompFunc: &dyn ::std::ops::Fn(VT, T) -> Result<bool>,
    mut inList: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    pub type CompFunc<VT: Clone + 'static, T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T>;
    outList = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            if !(!(inCompFunc(inValue.clone(), e.clone())?)) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outList)
}

pub fn filterCons<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
    mut accumList: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut accumList: metamodelica::List<T> = accumList;
    for mut e in &**inList {
        if r#fn(e.clone())? {
            accumList = metamodelica::cons(e.clone(), accumList);
        }
    }
    Ok(accumList)
}

pub use filterOnTrue as select;

pub use filter1OnTrue as select1;

pub use filter1rOnTrue as select1r;

pub use filter2OnTrue as select2;

pub fn find<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<T> {
    pub type SelectFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: T;
    for mut e in &**inList {
        if inFunc(e.clone())? {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    return Err("fail");
    Ok(outElement)
}

pub fn findOption<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: &metamodelica::List<T>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<Option<T>> {
    pub type Predicate<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut result: Option<T>;
    for mut e in &**lst {
        if r#fn(e.clone())? {
            result = Some(e.clone());
            return Ok(result);
        }
    }
    result = None;
    Ok(result)
}

pub fn find1<T: Clone + 'static + metamodelica::gc::MMTrace, ArgT1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<bool>,
    mut arg1: ArgT1,
) -> Result<T> {
    pub type SelectFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outElement: T;
    for mut e in &**inList {
        if inFunc(e.clone(), arg1.clone())? {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    return Err("fail");
    Ok(outElement)
}

pub fn findAndRemove<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(T, metamodelica::List<T>)> {
    pub type SelectFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: T;
    let mut rest: metamodelica::List<T>;
    let mut i: i32 = 0;
    let mut delst: DoubleEnded::MutableList<T>;
    let mut t: T;
    for mut e in &*inList {
        if inFunc(e.clone())? {
            outElement = e.clone();
            delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
            rest = inList;
            for mut i in 1..=i {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                t = metamodelica::Own::own(__pa0);
                rest = metamodelica::Own::own(__pa1);
                DoubleEnded::push_back(delst.clone(), t)?;
            }
            let __pa2 = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa2 } => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            rest = metamodelica::Own::own(__pa2);
            rest = DoubleEnded::toListAndClear(delst, rest)?;
            return Ok((outElement, rest));
        }
        i = i + 1;
    }
    return Err("fail");
    Ok((outElement, rest))
}

pub fn findAndRemove1<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<bool>,
    mut arg1: ArgT1,
) -> Result<(T, metamodelica::List<T>)> {
    pub type SelectFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outElement: T;
    let mut rest: metamodelica::List<T>;
    let mut i: i32 = 0;
    let mut delst: DoubleEnded::MutableList<T>;
    let mut t: T;
    for mut e in &*inList {
        if inFunc(e.clone(), arg1.clone())? {
            outElement = e.clone();
            delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
            rest = inList;
            for mut i in 1..=i {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                t = metamodelica::Own::own(__pa0);
                rest = metamodelica::Own::own(__pa1);
                DoubleEnded::push_back(delst.clone(), t)?;
            }
            let __pa2 = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa2 } => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            rest = metamodelica::Own::own(__pa2);
            rest = DoubleEnded::toListAndClear(delst, rest)?;
            return Ok((outElement, rest));
        }
        i = i + 1;
    }
    return Err("fail");
    Ok((outElement, rest))
}

pub fn findBoolList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inBooleans: &metamodelica::List<bool>,
    mut inList: metamodelica::List<T>,
    mut inFalseValue: T,
) -> Result<T> {
    let mut outElement: T;
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    for mut b in &**inBooleans {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if b.clone() {
            outElement = e;
            return Ok(outElement);
        }
    }
    outElement = inFalseValue;
    Ok(outElement)
}

pub fn deleteMemberOnTrue<
    VT: Clone + 'static + metamodelica::gc::MMTrace,
    T: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inValue: VT,
    mut inList: metamodelica::List<T>,
    mut inCompareFunc: &dyn ::std::ops::Fn(VT, T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, Option<T>)> {
    pub type CompareFunc<VT: Clone + 'static, T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T> = inList.clone();
    let mut outDeletedElement: Option<T> = None;
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList.clone();
    let mut acc: metamodelica::List<T> = metamodelica::nil();
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if inCompareFunc(inValue.clone(), e.clone())? {
            outList = listAppend(metamodelica::Dangerous::listReverseInPlace(acc), rest);
            if true
            /* isPresent not implemented in Rust */
            {
                outDeletedElement = Some(e);
            }
            return Ok((outList, outDeletedElement));
        }
        acc = metamodelica::cons(e, acc);
    }
    Ok((outList, outDeletedElement))
}

pub fn deletePositions<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPositions: metamodelica::List<i32>,
    mut zeroBased: bool,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    let mut sorted_pos: metamodelica::List<i32>;
    sorted_pos = sortedUnique(
        sort(
            inPositions,
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?,
        &fnptr!(intEq, i32, i32),
    )?;
    outList = deletePositionsSorted(inList, &sorted_pos, zeroBased)?;
    Ok(outList)
}

pub(crate) fn deletePositionsSorted<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPositions: &metamodelica::List<i32>,
    mut zeroBased: bool,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut i: i32 = if (zeroBased) { 0 } else { 1 };
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    for mut pos in &**inPositions {
        while i != pos.clone() {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            rest = metamodelica::Own::own(__pa1);
            outList = metamodelica::cons(e, outList);
            i = i + 1;
        }
        let __pa2 = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa2 } => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        rest = metamodelica::Own::own(__pa2);
        i = i + 1;
    }
    outList = append_reverse(&outList, rest);
    Ok(outList)
}

pub fn keepPositions<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPositions: metamodelica::List<i32>,
    mut zeroBased: bool,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    let mut sorted_pos: metamodelica::List<i32>;
    sorted_pos = sortedUnique(
        sort(
            inPositions,
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?,
        &fnptr!(intEq, i32, i32),
    )?;
    outList = keepPositionsSorted(inList, &sorted_pos, zeroBased)?;
    Ok(outList)
}

pub(crate) fn keepPositionsSorted<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPositions: &metamodelica::List<i32>,
    mut zeroBased: bool,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut i: i32 = if (zeroBased) { 0 } else { 1 };
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    for mut pos in &**inPositions {
        while i != pos.clone() {
            let __pa0 = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            rest = metamodelica::Own::own(__pa0);
            i = i + 1;
        }
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa1);
        rest = metamodelica::Own::own(__pa2);
        outList = metamodelica::cons(e, outList);
        i = i + 1;
    }
    outList = outList.reverse();
    Ok(outList)
}

pub fn replaceAt<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElement: T,
    mut inPosition: i32,
    mut inList: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    let mut delst: DoubleEnded::MutableList<T>;
    let true = (inPosition >= 1) else {
        return Err("pattern mismatch");
    };
    delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
    for mut i in 1..=inPosition - 1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        DoubleEnded::push_back(delst.clone(), e)?;
    }
    let __pa2 = ::match_deref::match_deref! { match &(rest) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa2 } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    rest = metamodelica::Own::own(__pa2);
    outList = DoubleEnded::toListAndClear(delst, metamodelica::cons(inElement, rest))?;
    Ok(outList)
}

pub fn replaceOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inReplacement: T,
    mut inList: metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, bool)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut outReplaced: bool = false;
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList.clone();
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if inFunc(e.clone())? {
            outReplaced = true;
            outList = append_reverse(&outList, metamodelica::cons(inReplacement, rest));
            return Ok((outList, outReplaced));
        }
        outList = metamodelica::cons(e, outList);
    }
    outList = inList;
    Ok((outList, outReplaced))
}

pub fn replaceAtIndexFirst<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inPosition: i32,
    mut inElement: T,
    mut inList: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T>;
    outList = replaceAt(inElement, inPosition, inList)?;
    Ok(outList)
}

pub fn replaceAtWithList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inReplacementList: metamodelica::List<T>,
    mut inPosition: i32,
    mut inList: metamodelica::List<T>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    let true = (inPosition > 0) else {
        return Err("pattern mismatch");
    };
    for mut i in 1..=inPosition - 1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        outList = metamodelica::cons(e, outList);
    }
    let __pa2 = ::match_deref::match_deref! { match &(rest) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa2 } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    rest = metamodelica::Own::own(__pa2);
    rest = listAppend(inReplacementList, rest);
    outList = append_reverse(&outList, rest);
    Ok(outList)
}

pub fn toString<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPrintFunc: &dyn ::std::ops::Fn(T) -> Result<ArcStr>,
    mut style: Style,
) -> Result<ArcStr> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut s: ArcStr;
    s = (match style {
        Style::NONE => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!(""),
            literal!(""),
            literal!(""),
            true,
            0,
        )?,
        Style::FLAT => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!(""),
            literal!(", "),
            literal!(""),
            true,
            0,
        )?,
        Style::FLAT_BRACKETS => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!("("),
            literal!(", "),
            literal!(")"),
            true,
            0,
        )?,
        Style::FLAT_CURLY => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!("{"),
            literal!(", "),
            literal!("}"),
            true,
            0,
        )?,
        Style::FLAT_CURLY_SHORT => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!("{"),
            literal!(", "),
            literal!("}"),
            true,
            10,
        )?,
        Style::NEWLINE => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!(""),
            literal!("\n"),
            literal!(""),
            true,
            0,
        )?,
        Style::NEWLINE_INDENT => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!("  "),
            literal!("\n  "),
            literal!(""),
            true,
            0,
        )?,
        Style::NEWLINE_TAB => toStringCustom(
            inList,
            inPrintFunc,
            literal!(""),
            literal!("\t"),
            literal!("\n\t"),
            literal!(""),
            true,
            0,
        )?,
        _ => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("List.toString"));
                __mm_s.push_str(&*literal!(" failed because of unknown list style.\n"));
                ArcStr::from(__mm_s)
            });
            return Err("fail");
        }
    });
    Ok(s)
}

pub fn toStringCustom<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inPrintFunc: &dyn ::std::ops::Fn(T) -> Result<ArcStr>,
    mut inNameStr: ArcStr,
    mut inBeginStr: ArcStr,
    mut inDelimitStr: ArcStr,
    mut inEndStr: ArcStr,
    mut inPrintEmpty: bool,
    mut maxLength: i32,
) -> Result<ArcStr> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    let mut lst: metamodelica::List<T> = inList;
    let mut endStr: ArcStr = inEndStr;
    if maxLength > 0 && ((lst).len() as i32) > maxLength {
        lst = firstN(lst, maxLength)?;
        endStr = stringAppendList(list![inDelimitStr.clone(), literal!("..."), endStr]);
    }
    outString = (::match_deref::match_deref! { match &((lst.clone(), inPrintEmpty)) {
        (Deref @ metamodelica::ListNode::Nil, true) => {
            stringAppendList(list![inNameStr, inBeginStr, endStr])
        },
        (Deref @ metamodelica::ListNode::Nil, false) => {
            inNameStr
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = stringDelimitList(map(lst, inPrintFunc)?, inDelimitStr);
            r#str = stringAppendList(list![inNameStr, inBeginStr, r#str, endStr]);
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn hasOneElement<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inList: &metamodelica::List<T>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hasSeveralElements<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inList: &metamodelica::List<T>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => false,
        Deref @ metamodelica::ListNode::Nil => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn lengthListElements<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inListList: metamodelica::List<metamodelica::List<T>>,
) -> i32 {
    let mut outLength: i32;
    outLength = ({
        let mut __acc: i32 = 0;
        for mut lst in (inListList).into_iter().cloned() {
            let __x = ((lst).len() as i32);
            __acc += __x;
        }
        __acc
    });
    outLength
}

pub fn accumulateMapAccum<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<TI>,
    mut inMapFunc: &dyn ::std::ops::Fn(TI, metamodelica::List<TO>) -> Result<metamodelica::List<TO>>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, metamodelica::List<TO>) -> Result<metamodelica::List<TO>> + 'static>;

    let mut outList: metamodelica::List<TO> = metamodelica::nil();
    for mut e in &**inList {
        outList = inMapFunc(e.clone(), outList)?;
    }
    outList = outList.reverse();
    Ok(outList)
}

pub fn findMap<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<(T, bool)>,
) -> Result<(metamodelica::List<T>, bool)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<(T, bool)> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut outFound: bool = false;
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    while !((rest).is_empty()) && !(outFound) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        (e, outFound) = inFunc(e)?;
        outList = metamodelica::cons(e, outList);
    }
    outList = append_reverse(&outList, rest);
    Ok((outList, outFound))
}

pub fn findAndMap<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut pred: &dyn ::std::ops::Fn(T) -> Result<bool>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<T>,
) -> Result<(metamodelica::List<T>, bool)> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type Func<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut found: bool = false;
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList.clone();
    while !((rest).is_empty()) && !(found) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if pred(e.clone())? {
            e = func(e)?;
            found = true;
        }
        outList = metamodelica::cons(e, outList);
    }
    if found {
        outList = append_reverse(&outList, rest);
    } else {
        outList = inList;
    }
    Ok((outList, found))
}

pub fn findSome<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T1>,
    mut inFunc: &dyn ::std::ops::Fn(T1) -> Result<Option<T2>>,
) -> Result<Option<T2>> {
    pub type FuncType<T1: Clone + 'static, T2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<Option<T2>> + 'static>;

    let mut outVal: Option<T2> = None;
    for mut e in &**inList {
        outVal = inFunc(e.clone())?;
        if (outVal).is_some() {
            return Ok(outVal);
        }
    }
    Ok(outVal)
}

pub fn splitEqualPrefix<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFullList: metamodelica::List<T1>,
    mut inPrefixList: metamodelica::List<T2>,
    mut inEqFunc: &dyn ::std::ops::Fn(T1, T2) -> Result<bool>,
    mut inAccum: &metamodelica::List<T1>,
) -> Result<(metamodelica::List<T1>, metamodelica::List<T1>)> {
    pub type EqFunc<T1: Clone + 'static, T2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outPrefix: metamodelica::List<T1> = metamodelica::nil();
    let mut outRest: metamodelica::List<T1>;
    let mut e1: T1;
    let mut e2: T2;
    let mut rest_e1: metamodelica::List<T1> = inFullList;
    let mut rest_e2: metamodelica::List<T2> = inPrefixList;
    loop {
        if (rest_e1).is_empty() || (rest_e2).is_empty() {
            break;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        rest_e1 = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_e2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa2);
        rest_e2 = metamodelica::Own::own(__pa3);
        if !(inEqFunc(e1.clone(), e2)?) {
            break;
        }
        outPrefix = metamodelica::cons(e1, outPrefix);
    }
    outPrefix = metamodelica::Dangerous::listReverseInPlace(outPrefix);
    outRest = rest_e1;
    Ok((outPrefix, outRest))
}

pub fn combination<TI: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElements: &metamodelica::List<metamodelica::List<TI>>,
) -> metamodelica::List<metamodelica::List<TI>> {
    let mut outElements: metamodelica::List<metamodelica::List<TI>>;
    let mut elems: metamodelica::List<metamodelica::List<TI>>;
    if (inElements).is_empty() {
        outElements = metamodelica::nil();
    } else {
        elems = combination_tail(inElements, &(metamodelica::nil()), &(metamodelica::nil()));
        outElements = elems.reverse();
    }
    outElements
}

fn combination_tail<TI: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElements: &metamodelica::List<metamodelica::List<TI>>,
    mut inCombination: &metamodelica::List<TI>,
    mut inAccumElems: &metamodelica::List<metamodelica::List<TI>>,
) -> metamodelica::List<metamodelica::List<TI>> {
    let mut outElements: metamodelica::List<metamodelica::List<TI>>;
    outElements = (::match_deref::match_deref! { match inElements {
        Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
            let mut acc: metamodelica::List<metamodelica::List<TI>>;
            acc = inAccumElems.clone();
            for mut e in &*head.clone() {
                acc = combination_tail(rest, &(metamodelica::cons(e.clone(), inCombination.clone())), &acc);
            }
            acc
        },
        _ => {
            metamodelica::cons(inCombination.clone().reverse(), inAccumElems.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

pub fn combinationMap<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inElements: &metamodelica::List<metamodelica::List<TI>>,
    mut inMapFunc: &dyn ::std::ops::Fn(metamodelica::List<TI>) -> Result<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<TI>) -> Result<TO> + 'static>;

    let mut outElements: metamodelica::List<TO>;
    let mut elems: metamodelica::List<TO>;
    elems = combinationMap_tail(inElements, inMapFunc, &(metamodelica::nil()), &(metamodelica::nil()))?;
    outElements = elems.reverse();
    Ok(outElements)
}

fn combinationMap_tail<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inElements: &metamodelica::List<metamodelica::List<TI>>,
    mut inMapFunc: &dyn ::std::ops::Fn(metamodelica::List<TI>) -> Result<TO>,
    mut inCombination: &metamodelica::List<TI>,
    mut inAccumElems: &metamodelica::List<TO>,
) -> Result<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<TI>) -> Result<TO> + 'static>;

    let mut outElements: metamodelica::List<TO>;
    outElements = (::match_deref::match_deref! { match inElements {
        Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
            let mut acc: metamodelica::List<TO>;
            acc = inAccumElems.clone();
            for mut e in &*head.clone() {
                acc = combinationMap_tail(rest, inMapFunc, &(metamodelica::cons(e.clone(), inCombination.clone())), &acc)?;
            }
            acc
        },
        _ => {
            metamodelica::cons(inMapFunc(inCombination.clone().reverse())?, inAccumElems.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElements)
}

pub fn allReferenceEq<T: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inList1: metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
) -> Result<bool> {
    let mut outEqual: bool;
    let mut rest1: metamodelica::List<T> = inList1;
    let mut rest2: metamodelica::List<T> = inList2;
    let mut e1: T;
    let mut e2: T;
    while !((rest1).is_empty() || (rest2).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        rest1 = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa2);
        rest2 = metamodelica::Own::own(__pa3);
        if !(metamodelica::ReferenceEq::reference_eq(&(e1), &(e2))) {
            outEqual = false;
            return Ok(outEqual);
        }
    }
    outEqual = (rest1).is_empty() && (rest2).is_empty();
    Ok(outEqual)
}

pub fn listIsLonger<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList1: metamodelica::List<T>,
    mut inList2: metamodelica::List<T>,
) -> Result<bool> {
    let mut isLonger: bool = compareLength(inList1.clone(), inList2.clone())? > 0;
    Ok(isLonger)
}

pub fn toListWithPositions<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
) -> metamodelica::List<(T, i32)> {
    let mut outList: metamodelica::List<(T, i32)> = metamodelica::nil();
    let mut pos: i32 = 1;
    for mut e in &**inList {
        outList = metamodelica::cons((e.clone(), pos), outList);
        pos = pos + 1;
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    outList
}

pub fn mkOption<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
) -> Option<metamodelica::List<T>> {
    let mut outOption: Option<metamodelica::List<T>>;
    outOption = if ((inList).is_empty()) { None } else { Some(inList) };
    outOption
}

pub fn all<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool;
    for mut e in &**inList {
        if !(inFunc(e.clone())?) {
            outResult = false;
            return Ok(outResult);
        }
    }
    outResult = true;
    Ok(outResult)
}

pub fn none<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool;
    for mut e in &**inList {
        if inFunc(e.clone())? {
            outResult = false;
            return Ok(outResult);
        }
    }
    outResult = true;
    Ok(outResult)
}

pub fn any<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool;
    for mut e in &**inList {
        if inFunc(e.clone())? {
            outResult = true;
            return Ok(outResult);
        }
    }
    outResult = false;
    Ok(outResult)
}

pub fn count<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<i32> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: i32 = 0;
    for mut e in &**inList {
        if inFunc(e.clone())? {
            outResult = outResult + 1;
        }
    }
    Ok(outResult)
}

pub fn separateOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outListTrue: metamodelica::List<T> = metamodelica::nil();
    let mut outListFalse: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if inFilterFunc(e.clone())? {
            outListTrue = metamodelica::cons(e.clone(), outListTrue);
        } else {
            outListFalse = metamodelica::cons(e.clone(), outListFalse);
        }
    }
    Ok((outListTrue, outListFalse))
}

pub fn separate1OnTrue<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<T>,
    mut inFilterFunc: &dyn ::std::ops::Fn(T, ArgT1) -> Result<bool>,
    mut inArg1: ArgT1,
) -> Result<(metamodelica::List<T>, metamodelica::List<T>)> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outListTrue: metamodelica::List<T> = metamodelica::nil();
    let mut outListFalse: metamodelica::List<T> = metamodelica::nil();
    for mut e in &**inList {
        if inFilterFunc(e.clone(), inArg1.clone())? {
            outListTrue = metamodelica::cons(e.clone(), outListTrue);
        } else {
            outListFalse = metamodelica::cons(e.clone(), outListFalse);
        }
    }
    Ok((outListTrue, outListFalse))
}

pub fn mapIndices<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut indices: &metamodelica::List<i32>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<T>,
) -> Result<metamodelica::List<T>> {
    pub type MapFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outList: metamodelica::List<T>;
    let mut i: i32 = 1;
    let mut idx: i32;
    let mut rest_idx: metamodelica::List<i32>;
    let mut e: T;
    let mut rest_lst: metamodelica::List<T>;
    if (indices).is_empty() {
        outList = inList;
        return Ok(outList);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*indices)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    idx = metamodelica::Own::own(__pa0);
    rest_idx = metamodelica::Own::own(__pa1);
    rest_lst = inList;
    outList = metamodelica::nil();
    while !((rest_lst).is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa2);
        rest_lst = metamodelica::Own::own(__pa3);
        if i == idx {
            outList = metamodelica::cons(func(e)?, outList);
            if (rest_idx).is_empty() {
                outList = append_reverse(&rest_lst, outList);
                break;
            } else {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_idx) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                idx = metamodelica::Own::own(__pa4);
                rest_idx = metamodelica::Own::own(__pa5);
            }
        } else {
            outList = metamodelica::cons(e, outList);
        }
        i = i + 1;
    }
    outList = metamodelica::Dangerous::listReverseInPlace(outList);
    Ok(outList)
}

pub fn allCombinations<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: &metamodelica::List<metamodelica::List<T>>,
    mut maxTotalSize: Option<i32>,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::List<T>>> {
    let mut out: metamodelica::List<metamodelica::List<T>>;
    out = (match maxTotalSize {
        Some(mut maxSz) => {
            let mut sz: i32;
            sz = intMul(
                ((lst).len() as i32),
                applyAndFold(lst, &fnptr!(intMul, i32, i32), &fnptr!(listLength, _), 1)?,
            );
            let true = (sz <= maxSz) else {
                return Err("pattern mismatch");
            };
            allCombinations2(lst)
        }
        None => allCombinations2(lst),
    });
    Ok(out)
}

fn allCombinations2<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ilst: &metamodelica::List<metamodelica::List<T>>,
) -> metamodelica::List<metamodelica::List<T>> {
    let mut out: metamodelica::List<metamodelica::List<T>>;
    out = (::match_deref::match_deref! { match ilst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: lst } => {
            let mut lst = (*lst).clone();
            lst = allCombinations2(metamodelica::AsArg::as_arg(&lst));
            allCombinations3(metamodelica::AsArg::as_arg(&x), metamodelica::AsArg::as_arg(&lst), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn allCombinations3<'__b, T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ilst1: &'__b metamodelica::List<T>,
    mut ilst2: &'__b metamodelica::List<metamodelica::List<T>>,
    mut iacc: metamodelica::List<metamodelica::List<T>>,
) -> metamodelica::List<metamodelica::List<T>> {
    '__tco: loop {
        ::match_deref::match_deref! { match ilst1 {
            Deref @ metamodelica::ListNode::Nil => {
                return iacc.reverse()
            },
            Deref @ metamodelica::ListNode::Cons { head: x, tail: lst1 } => {
                let mut acc: metamodelica::List<metamodelica::List<T>>;
                acc = allCombinations4(x.clone(), ilst2, iacc);
                { (ilst1, ilst2, iacc) = (lst1, ilst2, acc); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn allCombinations4<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut x: T,
    mut ilst: &metamodelica::List<metamodelica::List<T>>,
    mut iacc: metamodelica::List<metamodelica::List<T>>,
) -> metamodelica::List<metamodelica::List<T>> {
    let mut out: metamodelica::List<metamodelica::List<T>>;
    let mut acc: metamodelica::List<metamodelica::List<T>> = iacc;
    if (ilst).is_empty() {
        out = metamodelica::cons(list![x], acc);
        return out;
    }
    for mut l in &**ilst {
        acc = metamodelica::cons(metamodelica::cons(x.clone(), l.clone()), acc);
    }
    out = acc;
    out
}

pub fn contains<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: &metamodelica::List<T>,
    mut elem: T,
    mut eqFunc: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<bool> {
    pub type equalityFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: bool = false;
    for mut i in &**lst {
        if eqFunc(i.clone(), elem.clone())? {
            res = true;
            return Ok(res);
        }
    }
    Ok(res)
}

pub fn minElement<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut lessFn: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    res = (inList).head().cloned()?;
    for mut e in &*(inList).rest()? {
        if lessFn(e.clone(), res.clone())? {
            res = e.clone();
        }
    }
    Ok(res)
}

pub fn maxElement<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<T>,
    mut lessFn: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    res = (inList).head().cloned()?;
    for mut e in &*(inList).rest()? {
        if lessFn(res.clone(), e.clone())? {
            res = e.clone();
        }
    }
    Ok(res)
}

pub fn trim<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut l: metamodelica::List<T>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<metamodelica::List<T>> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut l: metamodelica::List<T> = l;
    while !((l).is_empty()) && r#fn((l).head().cloned()?)? {
        l = (l).rest()?;
    }
    Ok(l)
}

pub fn apply<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: &metamodelica::List<T>,
    mut r#fn: &dyn ::std::ops::Fn(T) -> Result<()>,
) -> Result<()> {
    pub type Fn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

    for mut e in &**lst {
        r#fn(e.clone())?;
    }
    Ok(())
}
