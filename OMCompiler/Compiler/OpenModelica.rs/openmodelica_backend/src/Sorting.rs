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

use crate::BackendDump;
use crate::Matching;
use openmodelica_backend_types::BackendDAE;
use openmodelica_util_datatypes_basic::GCExt;

pub fn Tarjan(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut N: i32,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut outComponents: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut stack: metamodelica::List<i32> = metamodelica::nil();
    let mut number: metamodelica::Array<i32>;
    let mut lowlink: metamodelica::Array<i32>;
    let mut onStack: metamodelica::Array<bool>;
    let mut eqn: i32;
    number = arrayCreate(N, -1);
    lowlink = arrayCreate(N, -1);
    onStack = arrayCreate(N, false);
    for mut var in 1..=metamodelica::arrayLength(ass1.clone()) {
        eqn = ({
            let __elt = (*metamodelica::index_checked(&ass1.borrow(), var)?).clone();
            __elt
        });
        if eqn > 0
            && ({
                let __elt = (*metamodelica::index_checked(&number.borrow(), eqn)?).clone();
                __elt
            }) == -1
        {
            (stack, index, outComponents) = StrongConnect(
                m.clone(),
                ass1.clone(),
                eqn,
                stack,
                index,
                number.clone(),
                lowlink.clone(),
                onStack.clone(),
                outComponents,
            )?;
        }
    }
    GCExt::free(number.clone());
    GCExt::free(lowlink.clone());
    GCExt::free(onStack.clone());
    outComponents = outComponents.reverse();
    Ok(outComponents)
}

fn StrongConnect(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut eqn: i32,
    mut stack: metamodelica::List<i32>,
    mut index: i32,
    mut number: metamodelica::Array<i32>,
    mut lowlink: metamodelica::Array<i32>,
    mut onStack: metamodelica::Array<bool>,
    mut inComponents: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<i32>,
    i32,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut outStack: metamodelica::List<i32> = stack;
    let mut outIndex: i32 = index;
    let mut outComponents: metamodelica::List<metamodelica::List<i32>> = inComponents;
    let mut callStack: metamodelica::List<(i32, metamodelica::List<i32>)> = metamodelica::nil();
    let mut SCC: metamodelica::List<i32>;
    let mut successors: metamodelica::List<i32> = metamodelica::nil();
    let mut current: i32 = eqn;
    let mut eqn2: i32;
    let mut parent: i32;
    let mut entering: bool = true;
    let mut descended: bool;
    loop {
        if entering {
            entering = false;
            metamodelica::arrayUpdate(number.clone(), current, outIndex)?;
            metamodelica::arrayUpdate(lowlink.clone(), current, outIndex)?;
            metamodelica::arrayUpdate(onStack.clone(), current, true)?;
            outIndex = outIndex + 1;
            outStack = metamodelica::cons(current, outStack);
            successors = Matching::incomingEquations(current, m.clone(), ass1.clone())?;
        }
        descended = false;
        while !((successors).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(successors) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eqn2 = metamodelica::Own::own(__pa0);
            successors = metamodelica::Own::own(__pa1);
            if ({
                let __elt = (*metamodelica::index_checked(&number.borrow(), eqn2)?).clone();
                __elt
            }) == -1
            {
                callStack = metamodelica::cons((current, successors.clone()), callStack);
                current = eqn2;
                entering = true;
                descended = true;
                break;
            } else if ({
                let __elt = (*metamodelica::index_checked(&onStack.borrow(), eqn2)?).clone();
                __elt
            }) {
                metamodelica::arrayUpdate(
                    lowlink.clone(),
                    current,
                    intMin(
                        ({
                            let __elt = (*metamodelica::index_checked(&lowlink.borrow(), current)?).clone();
                            __elt
                        }),
                        ({
                            let __elt = (*metamodelica::index_checked(&number.borrow(), eqn2)?).clone();
                            __elt
                        }),
                    ),
                )?;
            }
        }
        if !(descended) {
            if ({
                let __elt = (*metamodelica::index_checked(&lowlink.borrow(), current)?).clone();
                __elt
            }) == ({
                let __elt = (*metamodelica::index_checked(&number.borrow(), current)?).clone();
                __elt
            }) {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(outStack) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                eqn2 = metamodelica::Own::own(__pa2);
                outStack = metamodelica::Own::own(__pa3);
                metamodelica::arrayUpdate(onStack.clone(), eqn2, false)?;
                SCC = list![eqn2];
                while current != eqn2 {
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(outStack) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn2 = metamodelica::Own::own(__pa4);
                    outStack = metamodelica::Own::own(__pa5);
                    metamodelica::arrayUpdate(onStack.clone(), eqn2, false)?;
                    SCC = metamodelica::cons(eqn2, SCC);
                }
                outComponents = metamodelica::cons(metamodelica::Dangerous::listReverseInPlace(SCC), outComponents);
            }
            if (callStack).is_empty() {
                break;
            }
            let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(callStack) {
                Deref @ metamodelica::ListNode::Cons { head: (__pa6, __pa7), tail: __pa8 } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
                _ => return Err("pattern mismatch"),
            } };
            parent = metamodelica::Own::own(__pa6);
            successors = metamodelica::Own::own(__pa7);
            callStack = metamodelica::Own::own(__pa8);
            metamodelica::arrayUpdate(
                lowlink.clone(),
                parent,
                intMin(
                    ({
                        let __elt = (*metamodelica::index_checked(&lowlink.borrow(), parent)?).clone();
                        __elt
                    }),
                    ({
                        let __elt = (*metamodelica::index_checked(&lowlink.borrow(), current)?).clone();
                        __elt
                    }),
                ),
            )?;
            current = parent;
        }
    }
    Ok((outStack, outIndex, outComponents))
}

pub fn TarjanTransposed(
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut outComponents: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut stack: metamodelica::List<i32> = metamodelica::nil();
    let mut number: metamodelica::Array<i32>;
    let mut lowlink: metamodelica::Array<i32>;
    let mut onStack: metamodelica::Array<bool>;
    let mut N: i32 = metamodelica::arrayLength(ass2.clone());
    number = arrayCreate(N, -1);
    lowlink = arrayCreate(N, -1);
    onStack = arrayCreate(N, false);
    for mut eqn in 1..=N {
        if ({
            let __elt = (*metamodelica::index_checked(&number.borrow(), eqn)?).clone();
            __elt
        }) == -1
            && ({
                let __elt = (*metamodelica::index_checked(&ass2.borrow(), eqn)?).clone();
                __elt
            }) > 0
        {
            (stack, index, outComponents) = StrongConnectTransposed(
                mT.clone(),
                ass2.clone(),
                eqn,
                stack,
                index,
                number.clone(),
                lowlink.clone(),
                onStack.clone(),
                outComponents,
            )?;
        }
    }
    Ok(outComponents)
}

fn StrongConnectTransposed(
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass2: metamodelica::Array<i32>,
    mut eqn: i32,
    mut stack: metamodelica::List<i32>,
    mut index: i32,
    mut number: metamodelica::Array<i32>,
    mut lowlink: metamodelica::Array<i32>,
    mut onStack: metamodelica::Array<bool>,
    mut inComponents: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<i32>,
    i32,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let __ab_ass2 = ass2.borrow();
    let __ab_mT = mT.borrow();
    let mut outStack: metamodelica::List<i32> = stack;
    let mut outIndex: i32 = index;
    let mut outComponents: metamodelica::List<metamodelica::List<i32>> = inComponents;
    let mut callStack: metamodelica::List<(i32, metamodelica::List<i32>)> = metamodelica::nil();
    let mut SCC: metamodelica::List<i32>;
    let mut successors: metamodelica::List<i32> = metamodelica::nil();
    let mut current: i32 = eqn;
    let mut var: i32;
    let mut eqn2: i32;
    let mut parent: i32;
    let mut entering: bool = true;
    let mut descended: bool;
    loop {
        if entering {
            entering = false;
            metamodelica::arrayUpdate(number.clone(), current, outIndex)?;
            metamodelica::arrayUpdate(lowlink.clone(), current, outIndex)?;
            metamodelica::arrayUpdate(onStack.clone(), current, true)?;
            outIndex = outIndex + 1;
            outStack = metamodelica::cons(current, outStack);
            var = (*metamodelica::index_checked(&__ab_ass2, current)?).clone();
            successors = if (var > 0) {
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut e in ((*metamodelica::index_checked(&__ab_mT, var)?).clone())
                        .into_iter()
                        .cloned()
                    {
                        if !(e.clone() > 0 && e.clone() != current) {
                            continue;
                        }
                        let __x = e.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            } else {
                metamodelica::nil()
            };
        }
        descended = false;
        while !((successors).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(successors) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eqn2 = metamodelica::Own::own(__pa0);
            successors = metamodelica::Own::own(__pa1);
            if ({
                let __elt = (*metamodelica::index_checked(&number.borrow(), eqn2)?).clone();
                __elt
            }) == -1
            {
                callStack = metamodelica::cons((current, successors.clone()), callStack);
                current = eqn2;
                entering = true;
                descended = true;
                break;
            } else if ({
                let __elt = (*metamodelica::index_checked(&onStack.borrow(), eqn2)?).clone();
                __elt
            }) {
                metamodelica::arrayUpdate(
                    lowlink.clone(),
                    current,
                    intMin(
                        ({
                            let __elt = (*metamodelica::index_checked(&lowlink.borrow(), current)?).clone();
                            __elt
                        }),
                        ({
                            let __elt = (*metamodelica::index_checked(&number.borrow(), eqn2)?).clone();
                            __elt
                        }),
                    ),
                )?;
            }
        }
        if !(descended) {
            if ({
                let __elt = (*metamodelica::index_checked(&lowlink.borrow(), current)?).clone();
                __elt
            }) == ({
                let __elt = (*metamodelica::index_checked(&number.borrow(), current)?).clone();
                __elt
            }) {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(outStack) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                eqn2 = metamodelica::Own::own(__pa2);
                outStack = metamodelica::Own::own(__pa3);
                metamodelica::arrayUpdate(onStack.clone(), eqn2, false)?;
                SCC = list![eqn2];
                while current != eqn2 {
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(outStack) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn2 = metamodelica::Own::own(__pa4);
                    outStack = metamodelica::Own::own(__pa5);
                    metamodelica::arrayUpdate(onStack.clone(), eqn2, false)?;
                    SCC = metamodelica::cons(eqn2, SCC);
                }
                outComponents = metamodelica::cons(metamodelica::Dangerous::listReverseInPlace(SCC), outComponents);
            }
            if (callStack).is_empty() {
                break;
            }
            let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(callStack) {
                Deref @ metamodelica::ListNode::Cons { head: (__pa6, __pa7), tail: __pa8 } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
                _ => return Err("pattern mismatch"),
            } };
            parent = metamodelica::Own::own(__pa6);
            successors = metamodelica::Own::own(__pa7);
            callStack = metamodelica::Own::own(__pa8);
            metamodelica::arrayUpdate(
                lowlink.clone(),
                parent,
                intMin(
                    ({
                        let __elt = (*metamodelica::index_checked(&lowlink.borrow(), parent)?).clone();
                        __elt
                    }),
                    ({
                        let __elt = (*metamodelica::index_checked(&lowlink.borrow(), current)?).clone();
                        __elt
                    }),
                ),
            )?;
            current = parent;
        }
    }
    Ok((outStack, outIndex, outComponents))
}
