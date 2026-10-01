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

use crate::List;

pub fn mapNoCopy<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<T>,
) -> Result<metamodelica::Array<T>> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outArray: metamodelica::Array<T> = inArray.clone();
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
            inArray.clone(),
            i,
            inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i))?,
        );
    }
    Ok(outArray)
}

pub fn mapNoCopy_1<
    T: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inArray: metamodelica::Array<T>,
    mut inFunc: &dyn ::std::ops::Fn((T, ArgT)) -> Result<(T, ArgT)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Array<T>, ArgT)> {
    pub type FuncType<T: Clone + 'static, ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn((T, ArgT)) -> Result<(T, ArgT)> + 'static>;

    let mut outArray: metamodelica::Array<T> = inArray.clone();
    let mut outArg: ArgT = inArg;
    let mut e: T;
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        (e, outArg) = inFunc((
            metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i),
            outArg,
        ))?;
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(inArray.clone(), i, e);
    }
    Ok((outArray, outArg))
}

fn downheap(mut inArray: metamodelica::Array<i32>, mut n: i32, mut vIn: i32) -> Result<metamodelica::Array<i32>> {
    let mut inArray: metamodelica::Array<i32> = inArray;
    let mut v: i32 = vIn;
    let mut w: i32 = 2 * v + 1;
    let mut tmp: i32;
    while w < n {
        if w + 1 < n {
            if ({
                let __elt = (*metamodelica::index_checked(&inArray.borrow(), w + 2)?).clone();
                __elt
            }) > ({
                let __elt = (*metamodelica::index_checked(&inArray.borrow(), w + 1)?).clone();
                __elt
            }) {
                w = w + 1;
            }
        }
        if ({
            let __elt = (*metamodelica::index_checked(&inArray.borrow(), v + 1)?).clone();
            __elt
        }) >= ({
            let __elt = (*metamodelica::index_checked(&inArray.borrow(), w + 1)?).clone();
            __elt
        }) {
            return Ok(inArray);
        }
        tmp = ({
            let __elt = (*metamodelica::index_checked(&inArray.borrow(), v + 1)?).clone();
            __elt
        });
        {
            let __cell0 = ({
                let __elt = (*metamodelica::index_checked(&inArray.borrow(), w + 1)?).clone();
                __elt
            });
            let __idx0 = v + 1;
            *metamodelica::index_mut_checked(&mut inArray.clone().borrow_mut(), __idx0)? = __cell0;
        }
        {
            let __cell1 = tmp;
            let __idx1 = w + 1;
            *metamodelica::index_mut_checked(&mut inArray.clone().borrow_mut(), __idx1)? = __cell1;
        }
        v = w;
        w = 2 * v + 1;
    }
    Ok(inArray)
}

pub fn heapSort(mut inArray: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut inArray: metamodelica::Array<i32> = inArray;
    let mut n: i32 = metamodelica::arrayLength(inArray.clone());
    let mut tmp: i32;
    for mut v in ({
        let __s = intDiv(n, 2) - 1;
        let __e = 0;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        inArray = downheap(inArray.clone(), n, v)?;
    }
    for mut v in ({
        let __s = n;
        let __e = 2;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        tmp = ({
            let __elt = (*metamodelica::index_checked(&inArray.borrow(), 1)?).clone();
            __elt
        });
        {
            let __cell0 = ({
                let __elt = (*metamodelica::index_checked(&inArray.borrow(), v)?).clone();
                __elt
            });
            let __idx0 = 1;
            *metamodelica::index_mut_checked(&mut inArray.clone().borrow_mut(), __idx0)? = __cell0;
        }
        {
            let __cell1 = tmp;
            let __idx1 = v;
            *metamodelica::index_mut_checked(&mut inArray.clone().borrow_mut(), __idx1)? = __cell1;
        }
        inArray = downheap(inArray.clone(), v - 1, 0)?;
    }
    Ok(inArray)
}

pub(crate) fn findFirstOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
    mut inPredicate: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<Option<T>> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: Option<T>;
    outElement = None;
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inPredicate(e.clone())? {
            outElement = Some(e);
            break;
        }
    }
    Ok(outElement)
}

pub fn findFirstOnTrueWithIdx<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
    mut inPredicate: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<(Option<T>, i32)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: Option<T>;
    let mut idxOut: i32 = -1;
    let mut idx: i32 = 1;
    outElement = None;
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inPredicate(e.clone())? {
            idxOut = idx;
            outElement = Some(e);
            break;
        }
        idx = idx + 1;
    }
    Ok((outElement, idxOut))
}

pub fn select<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
    mut inIndices: &metamodelica::List<i32>,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut i: i32 = 1;
    outArray = metamodelica::arrayCreate(
        ((inIndices).len() as i32),
        ({
            let __elt = (*metamodelica::index_checked(&inArray.borrow(), 1)?).clone();
            __elt
        }),
    );
    for mut e in &**inIndices {
        unsafe {
            metamodelica::Dangerous::arrayInitSlotChecked(
                outArray.clone(),
                i,
                metamodelica::arrayGet(inArray.clone(), e.clone())?,
            )
        }?;
        i = i + 1;
    }
    Ok(outArray)
}

pub fn map<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    let mut res: TO;
    if len == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), 1))?;
        outArray = metamodelica::arrayCreate(len, res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res) };
        for mut i in 2..=len {
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    outArray.clone(),
                    i,
                    inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i))?,
                )
            };
        }
    }
    Ok(outArray)
}

pub fn map1<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inArray: metamodelica::Array<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, ArgT) -> Result<TO>,
    mut inArg: ArgT,
) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    let mut res: TO;
    if len == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(
            metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), 1),
            inArg.clone(),
        )?;
        outArray = metamodelica::arrayCreate(len, res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlotChecked(outArray.clone(), 1, res) }?;
        for mut i in 2..=len {
            unsafe {
                metamodelica::Dangerous::arrayInitSlotChecked(
                    outArray.clone(),
                    i,
                    inFunc(
                        metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i),
                        inArg.clone(),
                    )?,
                )
            }?;
        }
    }
    Ok(outArray)
}

pub fn map1Ind<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inArray: metamodelica::Array<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI, i32, ArgT) -> Result<TO>,
    mut inArg: ArgT,
) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, i32, ArgT) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    let mut res: TO;
    if len == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(
            metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), 1),
            1,
            inArg.clone(),
        )?;
        outArray = metamodelica::arrayCreate(len, res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlotChecked(outArray.clone(), 1, res) }?;
        for mut i in 2..=len {
            unsafe {
                metamodelica::Dangerous::arrayInitSlotChecked(
                    outArray.clone(),
                    i,
                    inFunc(
                        metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i),
                        i,
                        inArg.clone(),
                    )?,
                )
            }?;
        }
    }
    Ok(outArray)
}

pub fn mapList<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<TI>,
    mut inFunc: &dyn ::std::ops::Fn(TI) -> Result<TO>,
) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut i: i32 = 2;
    let mut len: i32 = ((inList).len() as i32);
    let mut res: TO;
    if len == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc((inList).head().cloned()?)?;
        outArray = metamodelica::arrayCreate(len, res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlotChecked(outArray.clone(), 1, res) }?;
        for mut e in &*(inList).rest()? {
            unsafe { metamodelica::Dangerous::arrayInitSlotChecked(outArray.clone(), i, inFunc(e.clone())?) }?;
            i = i + 1;
        }
    }
    Ok(outArray)
}

pub fn fold<T: Clone + 'static + metamodelica::gc::MMTrace, FoldT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, FoldT) -> Result<FoldT>,
    mut inStartValue: FoldT,
) -> Result<FoldT> {
    pub type FoldFunc<T: Clone + 'static, FoldT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, FoldT) -> Result<FoldT> + 'static>;

    let mut outResult: FoldT = inStartValue;
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        outResult = inFoldFunc(e, outResult)?;
    }
    Ok(outResult)
}

pub fn foldIndex<T: Clone + 'static + metamodelica::gc::MMTrace, FoldT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
    mut inFoldFunc: &dyn ::std::ops::Fn(T, i32, FoldT) -> Result<FoldT>,
    mut inStartValue: FoldT,
) -> Result<FoldT> {
    pub type FoldFunc<T: Clone + 'static, FoldT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, i32, FoldT) -> Result<FoldT> + 'static>;

    let mut outResult: FoldT = inStartValue;
    let mut e: T;
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        e = metamodelica::arrayGet(inArray.clone(), i)?;
        outResult = inFoldFunc(e, i, outResult)?;
    }
    Ok(outResult)
}

pub fn reduce<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
    mut inReduceFunc: &dyn ::std::ops::Fn(T, T) -> Result<T>,
) -> Result<T> {
    pub type ReduceFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>;

    let mut outResult: T;
    outResult = metamodelica::arrayGet(inArray.clone(), 1)?;
    for mut i in 2..=metamodelica::arrayLength(inArray.clone()) {
        outResult = inReduceFunc(outResult, metamodelica::arrayGet(inArray.clone(), i)?)?;
    }
    Ok(outResult)
}

pub fn updateIndexFirst<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIndex: i32,
    mut inValue: T,
    mut inArray: metamodelica::Array<T>,
) -> Result<()> {
    metamodelica::arrayUpdate(inArray.clone(), inIndex, inValue)?;
    Ok(())
}

pub fn getIndexFirst<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIndex: i32,
    mut inArray: metamodelica::Array<T>,
) -> Result<T> {
    let mut outElement: T = metamodelica::arrayGet(inArray.clone(), inIndex)?;
    Ok(outElement)
}

pub fn replaceAtWithFill<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inPos: i32,
    mut inTypeReplace: T,
    mut inTypeFill: T,
    mut inArray: metamodelica::Array<T>,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    outArray = expandToSize(inPos, inArray.clone(), inTypeFill)?;
    metamodelica::arrayUpdate(outArray.clone(), inPos, inTypeReplace)?;
    Ok(outArray)
}

pub fn expandToSize<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNewSize: i32,
    mut inArray: metamodelica::Array<T>,
    mut inFill: T,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    if inNewSize <= metamodelica::arrayLength(inArray.clone()) {
        outArray = inArray.clone();
    } else {
        outArray = arrayCreate(inNewSize, inFill);
        copy(inArray.clone(), outArray.clone())?;
    }
    Ok(outArray)
}

pub fn expand<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inN: i32,
    mut inArray: metamodelica::Array<T>,
    mut inFill: T,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut len: i32;
    if inN < 1 {
        outArray = inArray.clone();
    } else {
        len = metamodelica::arrayLength(inArray.clone());
        outArray = metamodelica::arrayCreate(len + inN, inFill.clone());
        copy(inArray.clone(), outArray.clone())?;
        setRange(len + 1, len + inN, outArray.clone(), inFill)?;
    }
    Ok(outArray)
}

pub fn expandOnDemand<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNewSize: i32,
    mut inArray: metamodelica::Array<T>,
    mut inExpansionFactor: metamodelica::Real,
    mut inFillValue: T,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut new_size: i32;
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    if inNewSize <= len {
        outArray = inArray.clone();
    } else {
        new_size = ((intReal(len) * inExpansionFactor).0.floor() as i32);
        outArray = metamodelica::arrayCreate(new_size, inFillValue.clone());
        copy(inArray.clone(), outArray.clone())?;
        setRange(len + 1, new_size, outArray.clone(), inFillValue)?;
    }
    Ok(outArray)
}

pub fn consToElement<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIndex: i32,
    mut inElement: T,
    mut inArray: metamodelica::Array<metamodelica::List<T>>,
) -> Result<metamodelica::Array<metamodelica::List<T>>> {
    let mut outArray: metamodelica::Array<metamodelica::List<T>>;
    outArray = metamodelica::arrayUpdate(
        inArray.clone(),
        inIndex,
        metamodelica::cons(
            inElement,
            ({
                let __elt = (*metamodelica::index_checked(&inArray.borrow(), inIndex)?).clone();
                __elt
            }),
        ),
    )?;
    Ok(outArray)
}

pub fn appendToElement<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIndex: i32,
    mut inElements: metamodelica::List<T>,
    mut inArray: metamodelica::Array<metamodelica::List<T>>,
) -> Result<metamodelica::Array<metamodelica::List<T>>> {
    let mut outArray: metamodelica::Array<metamodelica::List<T>>;
    outArray = metamodelica::arrayUpdate(
        inArray.clone(),
        inIndex,
        listAppend(
            ({
                let __elt = (*metamodelica::index_checked(&inArray.borrow(), inIndex)?).clone();
                __elt
            }),
            inElements,
        ),
    )?;
    Ok(outArray)
}

pub fn appendList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut lst: metamodelica::List<T>,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut arr_len: i32 = metamodelica::arrayLength(arr.clone());
    let mut lst_len: i32;
    let mut e: T;
    let mut rest: metamodelica::List<T>;
    if (lst).is_empty() {
        outArray = arr.clone();
    } else if arr_len == 0 {
        outArray = metamodelica::arrayFromVec(lst.into_iter().cloned().collect());
    } else {
        lst_len = ((lst).len() as i32);
        outArray = metamodelica::arrayCreate(
            arr_len + lst_len,
            ({
                let __elt = (*metamodelica::index_checked(&arr.borrow(), 1)?).clone();
                __elt
            }),
        );
        copy(arr.clone(), outArray.clone())?;
        rest = lst;
        for mut i in arr_len + 1..=arr_len + lst_len {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            rest = metamodelica::Own::own(__pa1);
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i, e) };
        }
    }
    Ok(outArray)
}

pub fn join<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr1: metamodelica::Array<T>,
    mut arr2: metamodelica::Array<T>,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut len1: i32 = metamodelica::arrayLength(arr1.clone());
    let mut len2: i32 = metamodelica::arrayLength(arr2.clone());
    if len1 == 0 {
        outArray = metamodelica::arrayFromVec(arr2.clone().borrow().clone());
    } else if len2 == 0 {
        outArray = metamodelica::arrayFromVec(arr1.clone().borrow().clone());
    } else {
        outArray = metamodelica::arrayCreate(
            len1 + len2,
            ({
                let __elt = (*metamodelica::index_checked(&arr1.borrow(), 1)?).clone();
                __elt
            }),
        );
        copyRange(arr1.clone(), outArray.clone(), 1, len1, 1)?;
        copyRange(arr2.clone(), outArray.clone(), 1, len2, len1 + 1)?;
    }
    Ok(outArray)
}

pub fn copy<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArraySrc: metamodelica::Array<T>,
    mut inArrayDest: metamodelica::Array<T>,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArrayDest.clone();
    if metamodelica::arrayLength(inArraySrc.clone()) > metamodelica::arrayLength(inArrayDest.clone()) {
        return Err("fail");
    }
    for mut i in 1..=metamodelica::arrayLength(inArraySrc.clone()) {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
            outArray.clone(),
            i,
            metamodelica::Dangerous::arrayGetNoBoundsChecking(inArraySrc.clone(), i),
        );
    }
    Ok(outArray)
}

pub fn copyN<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArraySrc: metamodelica::Array<T>,
    mut inArrayDest: metamodelica::Array<T>,
    mut inN: i32,
    mut srcOffset: i32,
    mut dstOffset: i32,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArrayDest.clone();
    if inN + dstOffset > metamodelica::arrayLength(inArrayDest.clone())
        || inN + srcOffset > metamodelica::arrayLength(inArraySrc.clone())
    {
        return Err("fail");
    }
    for mut i in 1..=inN {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
            outArray.clone(),
            i + dstOffset,
            metamodelica::Dangerous::arrayGetNoBoundsChecking(inArraySrc.clone(), i + srcOffset),
        );
    }
    Ok(outArray)
}

pub fn copyRange<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut srcArray: metamodelica::Array<T>,
    mut dstArray: metamodelica::Array<T>,
    mut srcFirst: i32,
    mut srcLast: i32,
    mut dstPos: i32,
) -> Result<()> {
    let mut offset: i32 = dstPos - srcFirst;
    if srcFirst > srcLast
        || srcLast > metamodelica::arrayLength(srcArray.clone())
        || offset + srcLast > metamodelica::arrayLength(dstArray.clone())
    {
        return Err("fail");
    }
    for mut i in srcFirst..=srcLast {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
            dstArray.clone(),
            offset + i,
            metamodelica::Dangerous::arrayGetNoBoundsChecking(srcArray.clone(), i),
        );
    }
    Ok(())
}

pub fn createIntRange(mut inLen: i32) -> metamodelica::Array<i32> {
    let mut outArray: metamodelica::Array<i32>;
    outArray = metamodelica::arrayCreate(inLen, 0);
    for mut i in 1..=inLen {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i, i) };
    }
    outArray
}

pub fn setRange<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStart: i32,
    mut inEnd: i32,
    mut inArray: metamodelica::Array<T>,
    mut inValue: T,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArray.clone();
    if inStart > metamodelica::arrayLength(inArray.clone()) {
        return Err("fail");
    }
    for mut i in inStart..=inEnd {
        metamodelica::arrayUpdate(inArray.clone(), i, inValue.clone())?;
    }
    Ok(outArray)
}

pub fn getRange<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStart: i32,
    mut inEnd: i32,
    mut inArray: metamodelica::Array<T>,
) -> Result<metamodelica::List<T>> {
    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut value: T;
    if inStart > metamodelica::arrayLength(inArray.clone()) {
        return Err("fail");
    }
    for mut i in inStart..=inEnd {
        value = metamodelica::arrayGet(inArray.clone(), i)?;
        outList = metamodelica::cons(value, outList);
    }
    Ok(outList)
}

pub fn position<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inArray: metamodelica::Array<T>,
    mut inElement: T,
    mut inFilledSize: i32,
) -> Result<i32> {
    let __ab_inArray = inArray.borrow();
    let mut outIndex: i32;
    for mut i in 1..=inFilledSize {
        if inElement.clone() == (*metamodelica::index_checked(&__ab_inArray, i)?).clone() {
            outIndex = i;
            return Ok(outIndex);
        }
    }
    outIndex = 0;
    Ok(outIndex)
}

pub fn getMemberOnTrue<
    VT: Clone + 'static + metamodelica::gc::MMTrace,
    ET: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inValue: VT,
    mut inArray: metamodelica::Array<ET>,
    mut inCompFunc: &dyn ::std::ops::Fn(VT, ET) -> Result<bool>,
) -> Result<(ET, i32)> {
    pub type CompFunc<VT: Clone + 'static, ET: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(VT, ET) -> Result<bool> + 'static>;

    let mut outElement: ET;
    let mut outIndex: i32;
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        if inCompFunc(
            inValue.clone(),
            metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i),
        )? {
            outElement = metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i);
            outIndex = i;
            return Ok((outElement, outIndex));
        }
    }
    return Err("fail");
    Ok((outElement, outIndex))
}

pub fn reverse<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut size: i32;
    let mut i: i32 = 0;
    let mut elem1: T;
    let mut elem2: T;
    outArray = inArray.clone();
    size = metamodelica::arrayLength(inArray.clone());
    for mut i in 1..=((metamodelica::real_div_checked(
        metamodelica::OrderedFloat((size) as f64),
        metamodelica::OrderedFloat((2) as f64),
    )?)
    .0 as i32)
    {
        elem1 = metamodelica::arrayGet(inArray.clone(), i)?;
        elem2 = metamodelica::arrayGet(inArray.clone(), size - i + 1)?;
        outArray = metamodelica::arrayUpdate(outArray.clone(), i, elem2)?;
        outArray = metamodelica::arrayUpdate(outArray.clone(), size - i + 1, elem1)?;
    }
    Ok(outArray)
}

pub fn toString<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArray: metamodelica::Array<T>,
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
    let mut lst: metamodelica::List<T>;
    let mut endStr: ArcStr = inEndStr.clone();
    if maxLength > 0 && metamodelica::arrayLength(inArray.clone()) > maxLength {
        lst = List::firstN(
            inArray
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
            maxLength,
        )?;
        endStr = stringAppendList(list![inDelimitStr.clone(), literal!("..."), inEndStr.clone()]);
    } else {
        lst = inArray
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>();
    }
    outString = (::match_deref::match_deref! { match &((lst.clone(), inPrintEmpty)) {
        (Deref @ metamodelica::ListNode::Nil, true) => {
            stringAppendList(list![inNameStr, inBeginStr, inEndStr])
        },
        (Deref @ metamodelica::ListNode::Nil, false) => {
            inNameStr
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = stringDelimitList(List::map(lst, inPrintFunc)?, inDelimitStr);
            r#str = stringAppendList(list![inNameStr, inBeginStr, r#str, endStr]);
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn hashIntArray(mut arr: metamodelica::Array<i32>) -> i32 {
    let mut hash: i32 = 5381;
    for mut i in 1..=metamodelica::arrayLength(arr.clone()) {
        hash = intMod(
            hash * 31 + metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i),
            536870911,
        );
    }
    hash
}

pub fn isEqual<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inArr1: metamodelica::Array<T>,
    mut inArr2: metamodelica::Array<T>,
) -> Result<bool> {
    let mut outIsEqual: bool = true;
    let mut arrLength: i32;
    arrLength = metamodelica::arrayLength(inArr1.clone());
    if !(intEq(arrLength, metamodelica::arrayLength(inArr2.clone()))) {
        return Err("fail");
    }
    for mut i in 1..=arrLength {
        if !(({
            let __elt = (*metamodelica::index_checked(&inArr1.borrow(), i)?).clone();
            __elt
        }) == ({
            let __elt = (*metamodelica::index_checked(&inArr2.borrow(), i)?).clone();
            __elt
        })) {
            outIsEqual = false;
            break;
        }
    }
    Ok(outIsEqual)
}

pub fn isEqualOnTrue<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut arr1: metamodelica::Array<T1>,
    mut arr2: metamodelica::Array<T2>,
    mut pred: &dyn ::std::ops::Fn(T1, T2) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc<T1: Clone + 'static, T2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut equal: bool;
    equal = metamodelica::arrayLength(arr1.clone()) == metamodelica::arrayLength(arr2.clone());
    if !(equal) {
        return Ok(equal);
    }
    for mut i in 1..=metamodelica::arrayLength(arr1.clone()) {
        if !(pred(
            metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i),
            metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i),
        )?) {
            equal = false;
            return Ok(equal);
        }
    }
    Ok(equal)
}

pub fn allEqual<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut pred: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut equal: bool = true;
    if arr.clone().borrow().is_empty() {
        return Ok(equal);
    }
    for mut i in 2..=metamodelica::arrayLength(arr.clone()) {
        if !(pred(
            metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), 1),
            metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i),
        )?) {
            equal = false;
            return Ok(equal);
        }
    }
    Ok(equal)
}

pub fn isLess<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr1: metamodelica::Array<T>,
    mut arr2: metamodelica::Array<T>,
    mut lessFn: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<bool> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: bool;
    let mut len1: i32;
    let mut len2: i32;
    let mut e1: T;
    let mut e2: T;
    len1 = metamodelica::arrayLength(arr1.clone());
    len2 = metamodelica::arrayLength(arr2.clone());
    for mut i in 1..=std::cmp::min(len1, len2) {
        e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i);
        e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i);
        if lessFn(e1.clone(), e2.clone())? {
            res = true;
            return Ok(res);
        } else if lessFn(e2, e1)? {
            res = false;
            return Ok(res);
        }
    }
    res = len1 < len2;
    Ok(res)
}

pub(crate) fn insertList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut lst: &metamodelica::List<T>,
    mut startPos: i32,
) -> Result<metamodelica::Array<T>> {
    let mut arr: metamodelica::Array<T> = arr;
    let mut i: i32 = startPos;
    for mut e in &**lst {
        {
            let __cell0 = e.clone();
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut arr.clone().borrow_mut(), __idx0)? = __cell0;
        }
        i = i + 1;
    }
    Ok(arr)
}

pub(crate) fn remove<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut index: i32,
) -> Result<metamodelica::Array<T>> {
    let mut outArr: metamodelica::Array<T>;
    let mut len: i32 = metamodelica::arrayLength(arr.clone());
    let true = (index <= len && index >= 1) else {
        return Err("pattern mismatch");
    };
    if len <= 1 {
        outArr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        outArr = metamodelica::arrayCreate(
            len - 1,
            ({
                let __elt = (*metamodelica::index_checked(&arr.borrow(), 1)?).clone();
                __elt
            }),
        );
        for mut i in 1..=index - 1 {
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    outArr.clone(),
                    i,
                    metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i),
                )
            };
        }
        for mut i in index + 1..=len {
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    outArr.clone(),
                    i - 1,
                    metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i),
                )
            };
        }
    }
    Ok(outArr)
}

pub fn all<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool;
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if !(inFunc(e)?) {
            outResult = false;
            return Ok(outResult);
        }
    }
    outResult = true;
    Ok(outResult)
}

pub fn any<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool;
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inFunc(e)? {
            outResult = true;
            return Ok(outResult);
        }
    }
    outResult = false;
    Ok(outResult)
}

pub fn minElement<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut lessFn: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    let mut e: T;
    res = ({
        let __elt = (*metamodelica::index_checked(&arr.borrow(), 1)?).clone();
        __elt
    });
    for mut i in 2..=metamodelica::arrayLength(arr.clone()) {
        e = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i);
        if lessFn(e.clone(), res.clone())? {
            res = e;
        }
    }
    Ok(res)
}

pub fn maxElement<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut lessFn: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    let mut e: T;
    res = ({
        let __elt = (*metamodelica::index_checked(&arr.borrow(), 1)?).clone();
        __elt
    });
    for mut i in 2..=metamodelica::arrayLength(arr.clone()) {
        e = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i);
        if lessFn(res.clone(), e.clone())? {
            res = e;
        }
    }
    Ok(res)
}

pub fn compare<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr1: metamodelica::Array<T1>,
    mut arr2: metamodelica::Array<T2>,
    mut compFn: &dyn ::std::ops::Fn(T1, T2) -> Result<i32>,
) -> Result<i32> {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>;

    let mut res: i32;
    let mut l1: i32;
    let mut l2: i32;
    l1 = metamodelica::arrayLength(arr1.clone());
    l2 = metamodelica::arrayLength(arr2.clone());
    res = if (l1 == l2) {
        0
    } else if (l1 > l2) {
        1
    } else {
        -1
    };
    if res != 0 {
        return Ok(res);
    }
    for mut i in 1..=l1 {
        res = compFn(
            metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i),
            metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i),
        )?;
        if res != 0 {
            return Ok(res);
        }
    }
    Ok(res)
}

pub fn mapFold<
    TI: Clone + 'static + metamodelica::gc::MMTrace,
    ArgT: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut arr: metamodelica::Array<TI>,
    mut func: &dyn ::std::ops::Fn(TI, ArgT) -> Result<(TO, ArgT)>,
    mut arg: ArgT,
) -> Result<(metamodelica::Array<TO>, ArgT)> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<(TO, ArgT)> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut outArg: ArgT = arg;
    let mut len: i32 = metamodelica::arrayLength(arr.clone());
    let mut res: TO;
    if len == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        (res, outArg) = func(
            metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), 1),
            outArg,
        )?;
        outArray = metamodelica::arrayCreate(len, res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res) };
        for mut i in 2..=len {
            (res, outArg) = func(
                metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i),
                outArg,
            )?;
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i, res) };
        }
    }
    Ok((outArray, outArg))
}

pub fn transpose<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<metamodelica::Array<T>>,
) -> metamodelica::Array<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<metamodelica::Array<T>>;
    let mut c_len: i32;
    let mut r_len: i32;
    let mut val: T;
    let mut row: metamodelica::Array<T>;
    if arr.clone().borrow().is_empty() {
        outArray = arr.clone();
        return outArray;
    }
    row = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), 1);
    if row.clone().borrow().is_empty() {
        outArray = arr.clone();
        return outArray;
    }
    val = metamodelica::Dangerous::arrayGetNoBoundsChecking(row.clone(), 1);
    c_len = metamodelica::arrayLength(arr.clone());
    r_len = metamodelica::arrayLength(row.clone());
    outArray = metamodelica::arrayCreate(r_len, row.clone());
    for mut i in 1..=r_len {
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i, metamodelica::arrayCreate(c_len, val.clone()))
        };
    }
    for mut r in 1..=r_len {
        for mut c in 1..=c_len {
            val = metamodelica::Dangerous::arrayGetNoBoundsChecking(
                metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), c),
                r,
            );
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
                metamodelica::Dangerous::arrayGetNoBoundsChecking(outArray.clone(), r),
                c,
                val,
            );
        }
    }
    outArray
}

pub fn threadMap<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
    TO: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut arr1: metamodelica::Array<T1>,
    mut arr2: metamodelica::Array<T2>,
    mut func: &dyn ::std::ops::Fn(T1, T2) -> Result<TO>,
) -> Result<metamodelica::Array<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut res: TO;
    let mut len1: i32;
    let mut len2: i32;
    if arr1.clone().borrow().is_empty() {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
        return Ok(outArray);
    }
    len1 = metamodelica::arrayLength(arr1.clone());
    len2 = metamodelica::arrayLength(arr2.clone());
    if len1 != len2 {
        return Err("fail");
    }
    res = func(
        metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), 1),
        metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), 1),
    )?;
    outArray = metamodelica::arrayCreate(len1, res.clone());
    unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res) };
    for mut i in 2..=len1 {
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(
                outArray.clone(),
                i,
                func(
                    metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i),
                    metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i),
                )?,
            )
        };
    }
    Ok(outArray)
}

pub fn generate<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut n: i32,
    mut generator: &dyn ::std::ops::Fn() -> Result<T>,
) -> Result<metamodelica::Array<T>> {
    pub type Generator<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn() -> Result<T> + 'static>;

    let mut arr: metamodelica::Array<T>;
    let mut e: T;
    if n <= 0 {
        arr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        e = generator()?;
        arr = metamodelica::arrayCreate(n, e.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), 1, e) };
        for mut i in 2..=n {
            unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i, generator()?) };
        }
    }
    Ok(arr)
}

pub fn filter<T: Clone + 'static + metamodelica::gc::MMTrace + Default>(
    mut arr: metamodelica::Array<T>,
    mut fun: &dyn ::std::ops::Fn(T) -> Result<bool>,
) -> Result<metamodelica::Array<T>> {
    pub type filterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut new_arr: metamodelica::Array<T>;
    let mut new_size: i32;
    let mut dummy: T;
    let mut index: i32 = 1;
    new_size = metamodelica::arrayLength(arr.clone())
        - ({
            let mut __acc: i32 = 0;
            for mut e in (arr.clone()).borrow().iter() {
                if !(fun(e.clone())?) {
                    continue;
                }
                let __x = 1;
                __acc += __x;
            }
            __acc
        });
    new_arr = metamodelica::arrayCreateDefault(new_size);
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if !(fun(e.clone())?) {
            unsafe { metamodelica::Dangerous::arrayInitSlot(new_arr.clone(), index, e) };
            index = index + 1;
        }
    }
    Ok(new_arr)
}
